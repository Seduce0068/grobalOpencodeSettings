---
description: テスト実行エビデンス収集・署名・検証
agent: build
subagent: true
---

テスト実行結果からエビデンスを生成・検証します。

サブコマンド: $ARGUMENTS (collect / verify / sign)
- `collect <test-result.json> [--spec-hash <sha256>]`: エビデンス生成・保存
- `verify <evidence.json>`: エビデンス整合性検証 (ハッシュ・署名・スキーマ)
- `sign <evidence.json>`: 秘密鍵で署名 (GPG/SSH/age)

収集項目:
- 実行環境 (OS, runtime, versions)
- テスト結果 (passed/failed/skipped, duration)
- カバレッジ (lines/branches/functions/statements)
- ミューテーションスコア
- 仕様ハッシュとの照合 (仕様変更後の再実行検知)

出力: `evidence/test-run-<timestamp>-<spec-hash>.json` (schema.json 準拠)