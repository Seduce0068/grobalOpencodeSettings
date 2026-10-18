/**
 * termux-notify: OpenCode イベントを Termux (スマホ) 通知へ転送するサーバプラグイン
 *
 * V2 の Plugin API でサーバイベントを購読し、既存の notify.rs
 * (rust-script キャッシュ済み・約 0.2 秒) を呼び出して通知する。
 * グローバル設定ディレクトリ配下 (plugins/) に置くため、全プロジェクトで
 * 自動的に有効になる (opencode.jsonc への登録は不要)。
 *
 * イベント対応 (NOTIFICATIONS テーブル):
 *   session.execution.started   -> start    (タスク開始)
 *   session.execution.succeeded -> complete (タスク完了)
 *   session.execution.failed    -> error    (エラー発生)
 *   permission.asked            -> wait     (入力待ち: 許可要求)
 *   form.created                -> wait     (入力待ち: 質問)
 *
 * フィルタ:
 *   execution.* は root セッションのみ通知する (サブエージェントの
 *   開始/完了はノイズになるため除外)。permission.asked / form.created は
 *   全セッションを対象にする (放置するとタスクが停止するため)。
 *   セッション情報の取得に失敗した場合は通知側へ倒す (fail-open)。
 *
 * 重複排除 (claim):
 *   V2 のイベントストリームはサーバ全体へ配信され、プラグインは
 *   ロケーション (プロジェクト) ごとにロードされる。そのため同一イベントが
 *   複数インスタンスへ届き、単純に通知するとロケーション数だけ重複する。
 *   イベント ID ごとに claim ファイルを O_EXCL (open "wx") で作成し、
 *   作成できた 1 インスタンスだけが通知する。
 *   claim は ~/.local/share/opencode/termux-notify-claims/ に置き、
 *   TTL (1 時間) を過ぎたファイルは順次削除する。
 *   ファイルシステム異常時は fail-open (重複を許容し通知を継続) とする。
 *
 * 設定変更:
 *   NOTIFICATIONS に定義を追加/削除する。定義のないイベントは無視される。
 *
 * ログ:
 *   失敗時は ~/.local/share/opencode/termux-notify.log へ追記する。
 *   通知失敗はセッション処理へ影響させない (fire-and-forget)。
 *   OPENCODE_NOTIFY_DEBUG=1 で検証ログ (claim/skip/sent) を同ログへ出力する。
 *
 * 依存:
 *   rust-script (PATH 上), ~/.config/opencode/notify.rs, termux-api
 */

import { Plugin } from "@opencode/plugin"
import { spawn } from "node:child_process"
import {
  appendFileSync,
  closeSync,
  mkdirSync,
  openSync,
  readdirSync,
  statSync,
  unlinkSync,
} from "node:fs"
import { homedir } from "node:os"
import { join } from "node:path"

const NOTIFY_SCRIPT = join(homedir(), ".config/opencode/notify.rs")
const LOG = join(homedir(), ".local/share/opencode/termux-notify.log")
const CLAIM_DIR = join(homedir(), ".local/share/opencode/termux-notify-claims")
const CLAIM_TTL_MS = 60 * 60 * 1000
const DEBUG = process.env.OPENCODE_NOTIFY_DEBUG === "1"

type NotifyType = "start" | "complete" | "error" | "wait"

const NOTIFICATIONS: Record<string, { type: NotifyType; message: string }> = {
  "session.execution.started": { type: "start", message: "タスク開始" },
  "session.execution.succeeded": { type: "complete", message: "タスク完了" },
  "session.execution.failed": { type: "error", message: "エラー発生" },
  "permission.asked": { type: "wait", message: "入力待ち (許可要求)" },
  "form.created": { type: "wait", message: "入力待ち (質問)" },
}

/** execution.* は root セッションのみ通知する対象イベント。 */
const ROOT_ONLY = new Set([
  "session.execution.started",
  "session.execution.succeeded",
  "session.execution.failed",
])

interface OpenCodeEvent {
  id?: string
  created?: number
  type?: string
  data?: { sessionID?: string }
}

interface SessionInfoLike {
  title?: string
  parentID?: string | null
}

interface PluginContextLike {
  event: {
    subscribe(options?: { signal?: AbortSignal }): AsyncIterable<OpenCodeEvent>
  }
  session: {
    get(input: { sessionID: string }): Promise<SessionInfoLike>
  }
}

function log(line: string): void {
  try {
    appendFileSync(LOG, `${new Date().toISOString()} ${line}\n`)
  } catch {
    // 診断ログの失敗は無視する。
  }
}

/**
 * イベント ID ごとに claim ファイルを O_EXCL で作成する。
 * 作成できたインスタンスだけが true を受け取り、グローバルで 1 回の通知になる。
 * claim ディレクトリの障害時は fail-open (重複を許容) で true を返す。
 */
function claim(eventID: string): boolean {
  const name = eventID.replace(/[^A-Za-z0-9_.-]/g, "_")
  try {
    mkdirSync(CLAIM_DIR, { recursive: true })
    try {
      closeSync(openSync(join(CLAIM_DIR, name), "wx"))
    } catch (error) {
      if ((error as { code?: string }).code === "EEXIST") {
        if (DEBUG) {
          log(`skip duplicate id=${name}`)
        }
        return false
      }
      throw error
    }
    if (DEBUG) {
      log(`claim id=${name}`)
    }
    sweepClaims()
    return true
  } catch (error) {
    log(`ERROR claim id=${name}: ${String(error)}`)
    return true
  }
}

/** TTL を過ぎた claim ファイルを削除する (ベストエフォート)。 */
function sweepClaims(): void {
  try {
    const now = Date.now()
    for (const name of readdirSync(CLAIM_DIR)) {
      try {
        const path = join(CLAIM_DIR, name)
        if (now - statSync(path).mtimeMs > CLAIM_TTL_MS) {
          unlinkSync(path)
        }
      } catch {
        // 個別の失敗は無視する。
      }
    }
  } catch {
    // sweep の失敗は無視する。
  }
}

/** notify.rs を fire-and-forget で起動する。失敗しても例外を外へ出さない。 */
function notify(type: NotifyType, message: string): void {
  if (DEBUG) {
    log(`sent type=${type} message=${message}`)
  }

  try {
    const child = spawn("rust-script", [NOTIFY_SCRIPT, type, message], {
      detached: true,
      stdio: "ignore",
    })
    child.on("error", (error) => {
      log(`ERROR type=${type} spawn: ${String(error)}`)
    })
    child.on("exit", (code) => {
      if (code !== 0) {
        log(`ERROR type=${type} exit=${String(code)}`)
      } else if (DEBUG) {
        log(`exit type=${type} code=0`)
      }
    })
    child.unref()
  } catch (error) {
    log(`ERROR type=${type} spawn: ${String(error)}`)
  }
}

async function handle(ctx: PluginContextLike, event: OpenCodeEvent): Promise<void> {
  const type = event.type
  const mapping = type ? NOTIFICATIONS[type] : undefined
  if (!type || !mapping) {
    return
  }

  const sessionID = event.data?.sessionID
  let title: string | undefined

  if (sessionID) {
    try {
      const session = await ctx.session.get({ sessionID })
      if (ROOT_ONLY.has(type) && session.parentID) {
        return
      }
      title = typeof session.title === "string" ? session.title : undefined
    } catch {
      // fail-open: 情報が取れなくても通知は送る。
    }
  }

  const claimKey =
    typeof event.id === "string" && event.id.length > 0
      ? event.id
      : `${type}:${sessionID ?? "unknown"}:${String(event.created ?? "unknown")}`
  if (!claim(claimKey)) {
    return
  }

  notify(mapping.type, title ? `${mapping.message}: ${title}` : mapping.message)
}

export default Plugin.define({
  id: "termux-notify",
  async setup(ctx) {
    const controller = new AbortController()

    void (async () => {
      try {
        for await (const event of ctx.event.subscribe({ signal: controller.signal })) {
          void handle(ctx as PluginContextLike, event as OpenCodeEvent).catch((error) => {
            log(`ERROR handle: ${String(error)}`)
          })
        }
      } catch (error) {
        if (!controller.signal.aborted) {
          log(`ERROR subscribe: ${String(error)}`)
        }
      }
    })()

    return () => controller.abort()
  },
})
