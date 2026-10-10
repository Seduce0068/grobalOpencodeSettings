---
name: Test Spec
description: テスト仕様書・テストコード・エビデンスの標準化・生成・検証。TDD/BDD/Property-based 対応。レビューフローと連携。
---

## 目的
- テスト仕様の「書きっぱなし・古い・実装と乖離」を防ぐ
- テストコード・仕様書・エビデンスの三位一体管理
- レビュー・CI で自動検証

## 三層構造

| 層 | 成果物 | 検証方法 | 更新トリガー |
|---|---|---|---|
| **仕様** | `specs/test-*.md` (Gherkin/Markdown) | レビューで承認 | 要件変更時 |
| **実装** | `tests/**/*.test.ts` 等 | CI で実行・カバレッジ | 仕様変更・リファクタ時 |
| **エビデンス** | `evidence/test-run-*.json` | ハッシュ比較・署名 | テスト実行毎 |

---

## ワークフロー

### 1. 仕様作成 (`/test-spec create`)
```
入力: ユーザーストーリー / 要件 / バグ報告
処理:
  1. テンプレート選択 (unit/integration/e2e/property/contract)
  2. Gherkin シナリオ生成
  3. 受入基準・境界値・異常系を網羅
  4. `specs/test-<feature>.md` 出力
  5. `/review-flow start` で仕様レビュー依頼
出力: 承認済みテスト仕様
```

### 2. 実装生成 (`/test-spec implement`)
```
入力: 承認済み仕様 + 実装コード (またはスケルトン)
処理:
  1. 仕様からテストコード雛形生成 (言語別テンプレート)
  2. 実装コードとの整合性検証 (型・シグネチャ)
  3. `tests/` 配下に配置
  4. `npm test / cargo test / pytest` で初回実行
出力: パスするテストコード
```

### 3. エビデンス収集 (`/test-spec evidence`)
```
入力: テスト実行結果
処理:
  1. 実行ログ・カバレッジ・ミューテーション結果を JSON 化
  2. 仕様ハッシュと照合 (仕様変更後の再実行検知)
  3. `evidence/test-run-<timestamp>-<spec-hash>.json` 保存
  4. 署名付きコミットで改竄検知可能化
出力: 検証可能なエビデンス
```

### 4. 整合性検証 (`/test-spec verify`)
```
実行: CI / pre-push / 手動
検証項目:
  ☐ 全テストパス
  ☐ カバレッジ ≥ 閾値 (unit 80%, integration 60%)
  ☐ 仕様と実装の 1:1 対応 (未実装シナリオ 0)
  ☐ フレーキーテスト 0 (直近 10 回成功)
  ☐ ミューテーションスコア ≥ 70%
  ☐ エビデンスハッシュ一致
判定: 全項目 PASS でのみマージ許可
```

---

## テンプレート一覧 (`templates/`)

| テンプレート | 用途 | 形式 |
|---|---|---|
| `unit-bdd.md` | 単体テスト (Given/When/Then) | Gherkin + TypeScript |
| `integration-api.md` | API 結合テスト | OpenAPI + SuperTest |
| `e2e-playwright.md` | E2E (Playwright MCP 連携) | TypeScript + Page Object |
| `property-based.md` | プロパティベース (fast-check) | TypeScript + Properties |
| `contract.md` | コントラクトテスト (Pact) | Consumer/Provider |
| `mutation.md` | ミューテーションテスト設定 | Stryker/gremlins.js |

---

## エビデンススキーマ (`evidence/schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["spec_hash", "timestamp", "environment", "results", "signature"],
  "properties": {
    "spec_hash": { "type": "string", "pattern": "^sha256:[a-f0-9]{64}$" },
    "timestamp": { "type": "string", "format": "date-time" },
    "environment": {
      "type": "object",
      "required": ["os", "runtime", "versions"],
      "properties": {
        "os": { "type": "string" },
        "runtime": { "type": "string" },
        "versions": { "type": "object" }
      }
    },
    "results": {
      "type": "object",
      "required": ["passed", "failed", "skipped", "duration_ms", "coverage", "mutation_score"],
      "properties": {
        "passed": { "type": "integer" },
        "failed": { "type": "integer" },
        "skipped": { "type": "integer" },
        "duration_ms": { "type": "integer" },
        "coverage": {
          "type": "object",
          "properties": {
            "lines": { "type": "number" },
            "branches": { "type": "number" },
            "functions": { "type": "number" },
            "statements": { "type": "number" }
          }
        },
        "mutation_score": { "type": "number" }
      }
    },
    "signature": { "type": "string" }
  }
}
```

---

## 権限
- `read`, `glob`, `grep`: 全許可
- `write`: `specs/`, `tests/`, `evidence/` のみ
- `shell`: テスト実行コマンドのみ (`npm test`, `cargo test`, `pytest`, `npx playwright test`)
- `subagent`: `/review-flow` 連携のため許可

## 連携
- `/cross-review` でテスト仕様もレビュー対象に含める
- `/review-flow gate-check` に Stage 0 として仕様存在チェック追加
- `/doc-create` でテスト仕様から ADR/Runbook 自動生成