---
name: review-flow
description: >-
  レビュープロセス全体を管理する。PR作成→自動チェック→クロスレビュー→承認→マージのパイプラインを定義し、ゲート条件とエスカレーションを制御する。敵対的レビュー設定を含む。
---

## 目的
レビューの「抜け・遅れ・属人化」を防ぎ、品質ゲートを自動化する。敵対的視点での自動拒否ルールを組み込み、人的バイアスを排除する。

## パイプライン定義

### Stage 1: 自動検証（ゲート: 必須通過 / 敵対的判定）
| チェック | ツール | 失敗時アクション | 敵対的閾値 |
| --- | --- | --- | --- |
| Lint | `npm run lint` / `cargo clippy` / `ruff` | **Auto-Reject** | warning 0 件必須 |
| 型検査 | `tsc --noEmit` / `mypy` / `cargo check` | **Auto-Reject** | error 0 件必須 |
| テスト | `npm test` / `pytest` / `cargo test` | **Auto-Reject** | リスク階層閾値 (`references/test/profiles/*.yaml` の `risk_tiers`)、flaky 0 |
| ビルド | `npm run build` / `cargo build --release` | **Auto-Reject** | artifact 生成必須 |
| 依存脆弱性 | `npm audit` / `cargo audit` / `pip-audit` | **Auto-Reject(High+)** | CVE High/Critical 0 |
| 秘密検出 | `gitleaks` / `trufflehog` | **Auto-Reject** | 真陽性 0 件 |
| 循環依存 | `madge --circular` / `cargo depgraph` | **Auto-Reject** | 循環 0 件 |
| 秘密情報ハードコード | `grep -rE "(api[_-]?key|secret|token|password)\s*="` | **Auto-Reject** | マッチ 0 件 |

### Stage 2: クロスレビュー（`cross-review` スキル / 敵対的統合）
- 3視点並列実行（security / arch / ops）
- **Critical/High あり → Auto-Reject（差し戻し）**
- **Medium 以上 → 条件付き承認**（修正コミット必須、再ゲート通過までブロック）
- **Low のみ → 承認**
- 期限: 非同期エージェントでは適用しない（自動キャンセル時のみ）。
- **敵対的ルール**: 同一指摘が 2 回以上繰り返される → Auto-Reject（学習不足とみなす）

### Stage 3: 承認・マージ（リスク階層）
- 個人リポジトリ（単独運用）: Stage 1 + Stage 2 通過で**自己承認**しマージ可。
  結果を `review-report.md` と最終回答に記録する。
- High/Critical（セキュリティ・認証・決済・データ破壊・外部公開）: **ユーザーの明示承認を必須**。
- チーム/共有リポジトリ: コードオーナー全員 +（Critical時）セキュリティオーナーの承認
  （最低 2 名の LGTM）を維持する。
- 署名コミットは設定済みの場合のみ。未設定時は通常コミットとし、その旨を報告する。
- マージ戦略: Squash + 線形履歴
- マージ前最終ゲート: Stage 1 再実行（リグレッション防止）
- マージ後: タグ付け、CHANGELOG 更新、デプロイメントトリガー

## CODEOWNERS 相当の自動アサインルール

### 所有者マッピング（`.opencode/owners.yml` で管理）
```yaml
# パターン優先順位: 詳細 > 一般
# すべてのパスはリポジトリルートからの相対パス

"*":                          # デフォルト（フォールバック）
  - "team-lead"
  - "platform-team"

"**/*.rs":                    # Rust
  - "rust-team"
  - "platform-team"

"**/*.ts/**/*.tsx":           # TypeScript/React
  - "frontend-team"
  - "api-team"

"**/*.py":                    # Python
  - "ml-team"
  - "backend-team"

"**/infra/**":                # インフラ/IaC
  - "platform-team"
  - "sre-team"

"**/security/**":             # セキュリティ関連
  - "security-team"
  - "platform-team"

"**/migrations/**":           # DBマイグレーション
  - "backend-team"
  - "dba-team"

"**/*.md":                    # ドキュメント
  - "docs-team"
  - "team-lead"

".github/**":                 # CI/CD
  - "platform-team"

"docker/**":                  # コンテナ
  - "platform-team"
  - "sre-team"
```

### アサインアルゴリズム
1. 変更ファイル一覧取得: `git diff --name-only BASE..HEAD`
2. 各ファイルをパターンマッチング（最長一致優先）
3. マッチしたオーナー集合を和集合
3. **最低 2 名**確保（不足時は `team-lead` + `platform-team` を補完）
4. 自己アサイン除外（作者 ≠ レビュアー）
5. 不在者除外（`owners.yml` に `unavailable: true` フラグ）
6. 通知: `/review-flow assign <pr-number>` で実行・通知

## CI/CD 連携（`gate-check` 実行インターフェース）

### GitHub Actions
```yaml
# .github/workflows/review-gate.yml
name: Review Gate Check
on:
  pull_request:
    types: [opened, synchronize, reopened, ready_for_review]
  workflow_dispatch:
    inputs:
      pr_number:
        required: true
        type: number

jobs:
  gate-check:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Setup tools
        run: |
          # Node/Rust/Python 等のセットアップ
          
      - name: Run gate-check
        id: gate
        run: |
          opencode run "/review-flow gate-check ${{ github.event.pull_request.number }}"
        env:
          OPENCODE_CLI_CONFIG_CONTENT: '{"session":{"permissions":"autoaccept"}}'
          
      - name: Post result
        if: always()
        run: |
          if [ "${{ steps.gate.outcome }}" = "success" ]; then
            gh pr check "Review Gate" --conclusion success --output "All gates passed"
          else
            gh pr check "Review Gate" --conclusion failure --output "Gate failed: see logs"
          fi
```

### GitLab CI
```yaml
# .gitlab-ci.yml
review_gate_check:
  stage: review
  image: node:22-alpine
  variables:
    OPENCODE_CLI_CONFIG_CONTENT: '{"session":{"permissions":"autoaccept"}}'
  script:
    - apk add --no-cache curl git
    - curl -fsSL https://opencode.ai/v2/install | bash
    - export PATH="$HOME/.local/bin:$PATH"
    - opencode run "/review-flow gate-check $CI_MERGE_REQUEST_IID"
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
  allow_failure: false
  timeout: 30m
```

### ローカル/汎用実行
```bash
# PR番号指定
opencode run "/review-flow gate-check 123"

# 現在のブランチ vs main
opencode run "/review-flow gate-check --base main --head HEAD"

# JSON出力（CI連携用）
opencode run "/review-flow gate-check 123 --json" > gate-result.json
```

**出力形式（JSON）:**
```json
{
  "passed": true,
  "checks": [
    {"name": "lint", "passed": true, "duration_ms": 1234, "details": "..."},
    {"name": "typecheck", "passed": true, "duration_ms": 567, "details": "..."},
    {"name": "test", "passed": true, "duration_ms": 15000, "details": "coverage: 87%"},
    {"name": "build", "passed": true, "duration_ms": 8000, "details": "..."},
    {"name": "vuln-scan", "passed": true, "duration_ms": 3000, "details": "0 high/critical"},
    {"name": "secret-scan", "passed": true, "duration_ms": 2000, "details": "0 findings"},
    {"name": "circular-deps", "passed": true, "duration_ms": 500, "details": "0 cycles"}
  ],
  "summary": {"total": 7, "passed": 7, "failed": 0, "duration_ms": 30401}
}
```

## 敵対的レビュー設定

### 自動拒否トリガー
| 条件 | アクション | 理由 |
| --- | --- | --- |
| Stage 1 で 1 件でも失敗 | **即座に Reject** | 品質基準未達 |
| Stage 2 で Critical/High 指摘 | **即座に Reject** | 設計/セキュリティ欠陥 |
| Stage 2 期限超過（24h） | **Auto-Reject** | レビュー遅延=品質リスク |
| 同一指摘 2 回目 | **Auto-Reject** | 改善ループ不全 |
| 単独承認（チームリポジトリのみ） | **Reject** | 相互確認不足 |
| Stage 3 再ゲート失敗 | **Reject** | リグレッション混入 |

### バイアス排除ルール
- 作者の過去実績による甘い判断 **禁止**
- 「動くから OK」判断 **禁止**（テスト・型・Lint で担保）
- 「後で直す」TODO コメント **禁止**（Issue 化必須）
- レビュアーの主観「好み」 **禁止**（規約・ADR 根拠必須）

### 敵対的テスト生成（オプション）
- 変更コードに対する **境界値・異常系・プロパティベーステスト** を自動生成提案
- 既存テストの **ミューテーションテスト** 実行（生存ミュータント = テスト不足 = Reject）
- 採用する技法は ISO/IEC/IEEE 29119-4 の節番号を明記する
  (仕様ベース 5.2.1-5.2.12、構造ベース 5.3.1-5.3.7、経験ベース 5.4.1)

## 静的試験と動的試験の分離

| 種別 | 担当 | 根拠 |
| --- | --- | --- |
| 静的試験 (レビュー・検査・静的解析) | `review-flow` / `cross-review` | ISO/IEC 20246 |
| 動的試験 (実行して確かめる) | `test-spec` スキル | ISO/IEC/IEEE 29119-2/-3/-4 |

ISO/IEC/IEEE 29119 シリーズに静的試験は含まれない。
ISO/IEC 29119-2:2021 の TP5 は test strategy が静的試験も含み得ると
注記するが，シリーズ自体は動的試験を対象とする。
`review-flow` が生成するレビュー指摘は 20246 の枠組みで表現する。

## 開発側の安全実践 (NIST SP 800-218 SSDF)

`review-flow` は動的試験とは別に，SSDF の実践が反映されているかを
Stage 1 の観点として確認できる。判定は shall/should の引用で行い，
適合の宣言はしない。

| 実践 | 観点 |
| --- | --- |
| PO (準備) | 保護要件が仕様に反映されているか |
| PS (設計) | 脅威モデルがテスト基準に落ちているか |
| PW (実装) | 静的解析とテストがパイプラインに統合されているか |
| RV (リリース) | 脆弱性情報が修正と追跡に結びついているか |

## メトリクス蓄積（SQLite / `~/.local/share/opencode/review-metrics.db`）

### スキーマ
```sql
CREATE TABLE reviews (
  id INTEGER PRIMARY KEY,
  pr_number INTEGER NOT NULL,
  repo TEXT NOT NULL,
  author TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  completed_at TIMESTAMP,
  status TEXT NOT NULL, -- 'passed' | 'rejected' | 'abandoned'
  stage1_duration_ms INTEGER,
  stage2_duration_ms INTEGER,
  stage3_duration_ms INTEGER,
  total_duration_ms INTEGER,
  critical_count INTEGER DEFAULT 0,
  high_count INTEGER DEFAULT 0,
  medium_count INTEGER DEFAULT 0,
  low_count INTEGER DEFAULT 0,
  reviewer_count INTEGER,
  files_changed INTEGER,
  lines_added INTEGER,
  lines_removed INTEGER
);

CREATE TABLE findings (
  id INTEGER PRIMARY KEY,
  review_id INTEGER NOT NULL REFERENCES reviews(id),
  agent TEXT NOT NULL, -- 'security' | 'arch' | 'ops'
  severity TEXT NOT NULL, -- 'critical' | 'high' | 'medium' | 'low'
  file TEXT NOT NULL,
  line INTEGER,
  rule_id TEXT, -- CWE/OWASP/自社ルールID
  description TEXT NOT NULL,
  remediated BOOLEAN DEFAULT FALSE,
  remediated_at TIMESTAMP
);

CREATE INDEX idx_reviews_pr ON reviews(pr_number);
CREATE INDEX idx_findings_review ON findings(review_id);
CREATE INDEX idx_findings_severity ON findings(severity);
```

### 記録タイミング
- `gate-check` 実行時 → Stage 1 結果記録
- `cross-review` 完了時 → Stage 2 指摘・判定記録
- マージ/クローズ時 → Stage 3 結果・総括記録

### クエリ例
```sql
-- 平均レビュー時間（直近30日）
SELECT AVG(total_duration_ms)/60000.0 AS avg_minutes FROM reviews 
WHERE created_at > datetime('now', '-30 days') AND status='passed';

-- 指摘密度（件/KLOC）
SELECT SUM(critical_count+high_count+medium_count+low_count)*1000.0/SUM(lines_added+lines_removed) 
AS findings_per_kloc FROM reviews WHERE status='passed';

-- 再発指摘率
SELECT COUNT(*) FILTER (WHERE remediated=FALSE)*1.0/COUNT(*) AS recurrence_rate FROM findings;

-- チーム別傾向
SELECT author, AVG(total_duration_ms)/60000.0 AS avg_min, 
       AVG(critical_count+high_count) AS avg_severe
FROM reviews GROUP BY author;
```

## 実行コマンド

| コマンド | 説明 |
| --- | --- |
| `/review-flow start [pr-number]` | パイプライン開始（Stage 1→2→3） |
| `/review-flow gate-check [pr-number] [--json] [--base main]` | Stage 1 のみ実行（CI用） |
| `/review-flow status [pr-number]` | 進捗・ゲート状況・メトリクス表示 |
| `/review-flow escalate [pr-number]` | 手動エスカレーション |
| `/review-flow assign [pr-number]` | CODEOWNERSベース自動アサイン |
| `/review-flow metrics [--days 30] [--json]` | メトリクス集計表示 |

## 権限
- `read`, `glob`, `grep` 全許可
- `shell`: CIツール実行パターンのみ許可（`npm run *`, `cargo *`, `pytest`, `gitleaks`, `trufflehog`, `madge`, `gh`, `glab` 等）
- `write`: レポート・メトリクスDB出力先のみ
- `subagent`: `cross-review` 実行のため許可
- `webfetch`: CI/CD API 呼出しのため許可（GitHub/GitLab API）