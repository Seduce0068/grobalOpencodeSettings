---
type: Reference
title: "dsh-gitbash-win クロスレビュー統合レポート"
description: "dsh-gitbash-win v0.1.0 (37 ファイル) の 5 視点クロスレビュー結果の統合と判定。"
tags:
  - dsh-gitbash-win
  - cross-review
  - code-review
  - windows
generated:
  by: z-ai/glm-5.3-flash
  at: "2026-11-11T14:05:35+09:00"
verified:
  by: z-ai/glm-5.3-flash
  at: "2026-11-11T14:05:35+09:00"
status: conditional-approval
---

## 判定

**条件付き承認** (マージブロックなし)。

- Critical 0 件・High 0 件 → マージブロック要件に抵触しない。
- Medium 8 件 (統合後) → 修正コミット必須。再ゲート通過までブロック。
- Low 46 件 (統合後) → 承認。改善提案として次回以降に反映。

## 実施体制

| 視点 | 形式 | 守備範囲 |
|---|---|---|
| security-reviewer | subagent | 入力検証・任意実行・XSS・秘密情報・CI |
| arch-reviewer | subagent | 責務分離・isolate 設計・DSH 互換戦略 |
| ops-reviewer | subagent | ログ・エラー処理・ロールバック・文書整合 |
| quality-reviewer | subagent | 保守性・効率性・共通部品・モジュール性・美しさ (コード) |
| beauty-reviewer | teammate | 美しさ (設計・全体)・総合評価 |

- 全員読み取り専用 (read / glob / grep のみ)。
- リポジトリ 37 ファイルを実読。行番号はレビュアーが実際に読んだ行。
- arch-reviewer は稼働中 DSH 0.2.2-rc.1 実機インストール (mise 配下) の
  公式インターフェースとの突き合わせ検証も実施した。
- 品質 5 観点の内訳 (統合前): 保守性 15 / 効率性 2 / 共通部品 9 /
  モジュール性 1 / 美しさ (コード) 8。

## Medium 統合発見事項 (8 件)

### M1. host-modules 解決の別コピー混入リスク

- file: `lib/host-modules.js` / line: 46-109, 119-141, 161-166
- sources: arch-reviewer
- description: anchor 候補のうち最初に成功したものを静かに採用する。
  docblock は「no silent fallback to a different copy of DSH」と宣言するが、
  symlink した peer やユーザー側 node_modules に @deepseek-ai/* がある環境では
  稼働中 DSH と版の異なるモジュールが解決され得る。
- remediation: 解決後に解決先パッケージの version を読み、
  稼働中ランタイムの DSH 版と照合して不一致なら警告または失敗させる。

### M2. resolve / spawnSpec の親クラス複製

- file: `lib/executor.js` / line: 262-281, 294-315
- sources: arch-reviewer
- description: 公式 LocalBashExecutor の同名メソッドをフィールド単位で
  丸ごと再実装しており、super.resolve を使わない。DSH 0.2.x 更新で
  契約が変わると複製側だけ静かに乖離し、検知が遅れる。
- remediation: super.resolve を呼んで差分だけ上書きする構成に寄せる。
  難しければ実 BaseExecutor との parity テスト (deep 比較) を追加する。

### M3. sandboxFallback の volatile 変更経路

- file: `lib/schema.js` / line: 61
- sources: arch-reviewer + security-reviewer (同一箇所を統合)
- description: sandboxFallback が .volatile() のままだと、settings 経路で
  再起動なしに値が変わり、公式ツールの mount 時広告と実行経路が乖離し得る。
  カードは FIELDS から除外しているが、それは UI レベルの防御にすぎない。
- remediation: sandboxFallback だけは mount 時スナップショットに固定するか、
  live 値が mount 値と異なった時点で警告ログを出す。
  settings API からの変更拒否を E2E で固定すること。

### M4. カード Save と手編集 cordis.patch.yml の共存未文書化

- file: `README.md` / line: 167-170, 197-198
- sources: ops-reviewer
- description: 実テストは Save 後に cordis.patch.yml が書き換わることを
  検証しているが、手編集した行との共存挙動が文書化されていない。
  手編集設定が Save で失われるリスクを運用者が評価できない。
- remediation: 「Save は volatile settings を cordis.patch.yml に永続化する」
  ことと手編集内容の共存注意を README と docs/install-windows.md に明記する。

### M5. DSH バージョン更新の追従手順不在

- file: `lib/host-modules.js` / line: 170-175 ほか 8 ファイル以上
- sources: ops-reviewer
- description: 0.2.2-rc.1 固定の追随箇所が散在し、
  変更対象ファイルの一覧がどこにも文書化されていない。
  手動配置経路では engines 強制が効かず、実行時エラーまで検知できない。
- remediation: 「DSH バージョン更新の追従チェックリスト」を docs に追加し、
  手動配置経路での最初の検証手段 (status route 等) を明記する。

### M6. configValues() の二重 projection と分散定義

- file: `lib/executor.js` / line: 209-236 (関係: schema.js 43-62,
  executor.js 87-97, index.js 74-84)
- sources: quality-reviewer + beauty-reviewer (同一エリアを統合)
- description: live reader 経路と mount スナップショット経路で
  同じ 9 フィールドの projection を手書きで 2 回繰り返す。
  フィールド定義も 5 箇所以上に分散し、新フィールド追加時に同期漏れが起きる。
- remediation: 両経路を 1 つの normalize 関数に統合する。
  executor.js 側は Object.keys(FIELD_DEFAULTS) で畳む。

### M7. ログ・エラー接頭辞と言語方針の未定義

- file: `lib/executor.js` / line: 48-62 ほか (index.js 327, 345 等)
- sources: beauty-reviewer + quality-reviewer + ops-reviewer (統合)
- description: 接頭辞が `dsh-gitbash-win:` と `gitbash-win:` の 2 系統で
  基準が明文化されていない。エラーメッセージも日本語と英語が混在し、
  障害解析時の照合性が落ちる。
- remediation: 「モデル・ユーザーに届く文面 = パッケージ名接頭辞、
  内部ログ = 行 ID 接頭辞」の規則を DESIGN.md に明記し、言語方針を統一する。

### M8. NUMBER_PLACEHOLDERS の三重管理

- file: `lib/client.js` / line: 156
- sources: beauty-reviewer + quality-reviewer + ops-reviewer +
  arch-reviewer (同一箇所を統合)
- description: 数値デフォルトが schema.js (唯一の正)、cordis.patch.yml、
  client.js の placeholder の 3 箇所に存在する。
  placeholder 側は schema との突き合わせ検証がなく、変更時に沈黙してズレる。
- remediation: client.test.mjs に
  `NUMBER_PLACEHOLDERS.timeoutMs === String(DEFAULT_TIMEOUT_MS)` の assert を
  1 本追加する。

## Low 統合発見事項 (46 件、グループ別)

### G1 ドキュメント整合 (8 件)

| file | line | 内容 |
|---|---|---|
| test/windows-sandbox-probe.mjs | 26-27 | docstring が旧仕様 (non-blocking) のまま。実際は blocking |
| .github/workflows/windows-smoke.yml | 13-14 | probe 結果の列挙が 2 種のみ。acl-incompatible を追加 |
| README.md | 293 | 再検出手順を「15 秒待機 → bashPath 明示 → 再起動は最後」に改順 |
| README.md | 306-311 | 開発・テスト節に dsh CLI / DSH 本体の前提を明記 |
| docs/install-windows.md | 64-95 | 1-B に cordis.patch.yml 編集手順を追加し相互参照 |
| SECURITY.md | 5-11 | サポート範囲を engines 範囲と揃える (統合: ops-reviewer) |
| README.md | 308 | テスト数ハードコードを撤去し verification.md に寄せる (統合: quality-reviewer) |
| docs/security.md | 77 | 誤記「工具」を「ツール」へ修正 |

### G2 重複コード・共通部品 (8 件)

| file | line | 内容 |
|---|---|---|
| test/e2e.sh + test/e2e-windows.sh | 全体 | 約 8 割が重複。共通ライブラリ抽出 (統合: arch / quality / beauty) |
| lib/detect.js | 494-563 | auto-detection と registry ループの同一ロジックを evaluateCandidate() に切り出し |
| lib/detect.js | 343, 356 | failureResult() の同形検索式を 1 回に共有 |
| lib/detect.js + lib/host-modules.js | 63-65 / 35-37 | pathKey() 重複。共有ユーティリティ化 |
| .github/workflows | 各所 | checkout/setup-node/cache/install の composite 化 |
| lib/executor.js | 399-417 | direct/unconfined 分岐の重複を 1 経路に統合 |
| test/windows-sandbox-probe.mjs | 86 | KNOWN_RESTRICTED_TOKEN_CRASHES を executor.js から import |
| test/windows-live.test.mjs | 78-81 | toPosixPath を共通ヘルパー化 |

### G3 テスト強化 (5 件)

| file | line | 内容 |
|---|---|---|
| test/detect.test.mjs | 172-341 | 共通オプション群 250 行超をビルダーヘルパーに抽出 |
| test/host.test.mjs | 330-335 | reason を自分自身と比較する恒真断言を修正 |
| test/e2e-windows.sh 関連 | 107-130 | カード生成値そのものの BASH_ENV 動作確認を追加 (arch-reviewer) |
| README.md (開発節) | - | npm test に DSH 0.2.2-rc.1 導入が必要な旨を明記 |
| test/windows-live.test.mjs | 30-35 | 動的 import の理由をコメント明記 |

### G4 堅牢性・境界 (5 件)

| file | line | 内容 |
|---|---|---|
| lib/detect.js | 471-477, 102-118 | MSYS 系でない system を明示パスでも fail-closed に (security-reviewer) |
| lib/executor.js | 481, 72-74 | 0xC0000005 誤分類の可能性を docs に追記 |
| DESIGN.md §3.1 + lib/executor.js | 102 / 271 | 空 cwd の規定 (セッション cwd) と実装 (process.cwd()) のずれを解消 |
| lib/executor.js | 169-177, 268 | standing policy 解決不能時の fail-open 境界に警告ログ |
| lib/schema.js | 46-55 | 数値フィールドに .max(MAX_TIMER_DELAY_MS) 等の上限 (統合: security-reviewer) |

### G5 可観測性・運用 (6 件)

| file | line | 内容 |
|---|---|---|
| lib/index.js | 199-230 | status route の再検出コスト上限 (統合: quality-reviewer) |
| lib/index.js | 217-221 | 検出成功時の info ログ (1 回のみ) と候補への version 表示 |
| lib/host-modules.js | 83-89, 170-175 | tried 全 anchor の join を要約表示に |
| lib/client.js | 613-631 | saveFailed 文言に再読込の案内を追加 |
| README.md | - | run_code (PTC) からも bash が見える点を既知の制約に追記 |
| lib/index.js | 228 | status の tool.global を実登録結果の反映に |

### G6 命名・整形・デッドコード (9 件)

| file | line | 内容 |
|---|---|---|
| lib/executor.js | 364-366 | routeFor() デッドメソッドを削除 |
| test/client.test.mjs | 80-112 | 未使用 React stand-in hook を削除 |
| test/windows-live.test.mjs | 114 | detection という紛らわしい束縛名を変更 |
| test/e2e.sh + e2e-windows.sh | 192 / 188 | --timeout-ms 131000 の根拠をコメント |
| test/windows-sandbox-probe.mjs | 145-160 | exit 後の到達不能 break を削除 |
| lib/detect.js | 28 | MINGW_SYSTEM_PATTERN を MSYS_FAMILY_SYSTEM_PATTERN へ改名 |
| lib/detect.js | 679-686 | clearDetectionCache() の JSDoc を実態に合わせ修正 |
| lib/detect.js | 213 | candidateBashPaths() をテスト専用 API として JSDoc 明記 |
| lib/client.js | 352-354 | requestStatus の三項演算子ネストを分解 |

### G7 契約・設計文書 (7 件)

| file | line | 内容 |
|---|---|---|
| lib/index.js | 50-53 | STATUS_PATH / ROUTING_SECTION を export し複製を解消 |
| DESIGN.md §7 | 289-292 | slot inject 契約を実装面へ更新 (統合: quality-reviewer) |
| DESIGN.md §5 | 213-217 | 解決候補リストを実装に合わせ更新 |
| DESIGN.md §2 | 58-80 | v0.1 開発時の担当表として位置づけを注記 |
| package.json | 49-54 | dshReleases の宣言のみである旨を明記 (arch-reviewer) |
| package.json + DESIGN | 35, 50, 57 | 版範囲 3 箇所重複の更新手順を 1 行明記 |
| lib/host-modules.js | 172 | エラー文面の DSH 版ハードコードを解消 |

### G8 将来設計 (1 件)

| file | line | 内容 |
|---|---|---|
| DESIGN.md | 19-21, 405-411 | v0.2 (WSL/cmd/Nushell・pwsh 非表示) の加算的実現メカニズムを事前に DESIGN へ追記 |

## 重複マージ記録

| 統合後 | 吸収した個別指摘 | 統合理由 |
|---|---|---|
| M3 | arch#3 + security#1 | 同一箇所 (schema.js:61)・同一本質 |
| M6 | quality#1 + beauty#8 + quality#17 | 同一エリア (configValues・FIELD_DEFAULTS 同期) |
| M7 | beauty#1 + quality#2 + ops#6 | 同一 remediation (DESIGN.md への方針明文化) |
| M8 | beauty#2 + quality#3 + ops#12 + arch#11 | 同一箇所 (client.js:156) |
| G1 (SECURITY.md) | arch#5 + ops#14 | 同一箇所 (SECURITY.md:5-11) |
| G1 (README:308) | beauty#7 + quality#13 | 同一箇所 (README.md:308) |
| G2 (e2e 重複) | arch#8 + quality#22 + beauty#10 + quality#8 | 同一主題 (e2e 2 本の共通化) |
| G5 (status route) | security#3 + quality#16 | 同一本質 (再検出コスト上限) |
| G4 (schema 上限) | security#6 + arch#6 | 同一箇所 (schema.js 数値範囲) |
| G7 (DESIGN §7) | beauty#5 + quality#4 | 同一契約面 (inject 契約) |
| G2 (cache key) | ops#8 + quality#34 | 同一箇所 (CI cache key 記法) |
| G6 辞書重複 | quality#21 + beauty#9 | 同一主題 (locale 辞書の正本) |

統合前合計 71 件 (Medium 9 / Low 62) → 統合後 54 件 (Medium 8 / Low 46)。

## 検証済みの妥当な設計 (指摘に至らなかった確認事項)

- XSS なし: client.js は createElement のテキスト子のみ。locale 埋め込みは
  テストで辞書一致固定。
- コマンドインジェクションなし: argv 1 要素 + shell:false。
  shell:true 不使用をテストがソースレベルで担保。
- initScript (BASH_ENV) は環境変数渡しで注入不可。
- status route はフェンス最優先・破損時 fail-closed・GET のみ・no-store。
- gitleaks はバージョン + tarball SHA-256 固定・redact・全履歴 + ツリー。
- CI は actions / DSH / playwright-core すべてピン留め。
- 責務分離と依存方向は健全。循環依存なし。
- ctx.isolate('shell') 方式は実機 DSH に対して検証済みで妥当。
- routing section の注入位置は実 SystemPrompt サービスで検証済み。
- volatile config → liveConfigReader の流れは live-config.test.mjs で
  実行可能エビデンス化されており堅牢。

## 美しさ総合評価 (beauty-reviewer)

良かった点:

- 中心アイデア (公式 dsh-tool-bash を 1 行も fork せず ctx.isolate('shell')
  内で私有 executor に接続) が優雅で、採らなかった案と実測根拠まで
  DESIGN に書かれている。
- schema → executor → status route → 設定カードの縦の流れが、
  DESIGN の各節と実装・テストまで 1 対 1 で追跡可能。
- 3 種 ID (パッケージ名 / 行 ID / クラス名) の役割分担が明確で、
  effect 名・DOM id・locale 名前空間まで行 ID で統一。
- fail-closed・無断 fallback 禁止の方針が全文書で一語もブレずに貫かれている。
- 実測日付きの既知制約が全文書・エラー文面・テストで同じ記述として共有され、
  文書と実装の乖離がほぼない。

惜しかった点:

- 接頭辞の 2 系統 (M7) と数値デフォルトの 3 箇所管理 (M8) が
  最も目立つ不統一。いずれも規則の明文化と assert 1 本で解決する。
- 契約値のリテラル複製 (G7) と DESIGN の契約面の追従漏れ。
- DESIGN.md が設計契約・開発工程メモ・実測ログ要約の 3 種を
  1 ファイルに収めており密度が高い (G7 の DESIGN §2 と関連)。

評価: 37 ファイル規模のプラグインとして、設計・実装・テスト・文書の四層が
同じ物語 (「公式構成に 1 行だけ足す、触れたら戻す」) を語っており、
指摘はすべて Low-Medium の命名明文化・リテラル集約・文書追従である。
additive 設計の思想が patch → isolate → カード → ドキュメント → CI まで
機械的に検証されている点は特に美しく、総合では高く評価できる。

## 次のアクション (修正優先順位の提案)

1. コード系 Medium: M1 (version 照合) → M3 (sandboxFallback 固定) →
   M2 (parity テスト) → M6 (normalize 統合)。
2. 文書・テスト系 Medium: M8 (assert 1 本) → M7 (方針明文化) →
   M4 / M5 (文書追加)。
3. Low は G4 (堅牢性) から着手し、G1 (文書) と G2 (共通化) は
   次回機能改訂 (v0.2) にバンドルするのが効率的。
4. 修正後の再レビューは差分のみ対象 (cross-review スキルの
   フィードバックループ規定)。
