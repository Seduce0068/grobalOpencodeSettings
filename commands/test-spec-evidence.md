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

追加で収集する項目 (任意。無い場合は省略可):

- `document_type`: test-execution-log / test-result / test-summary-report
- `profile`: 適用した検証プロファイル名
- `test_basis_ref`: 仕様の参照 (パスと spec_hash)
- `techniques`: 適用した技法の ID と 29119-4 節番号
- `coverage_measures`: 29119-4 Clause 6 のカバレッジ種別別値
- `risk_items`: リスク項目と tier
- `control_coverage`: SP 800-53 の統制カバレッジ
  (対象外と未達を区別する)

- `assurance`: 内部保証尺度
- `conformance`: `tailored` 固定

内部保証尺度の算出:

| 尺度 | 根拠 CC 構成要素 | 条件 |
| --- | --- | --- |
| A0 | なし | 手動・記録なし |
| A1 | ATE_FUN.1 | 実行結果の記録 |
| A2 | + ATE_COV.2 | カバレッジ基準の充足 |
| A3 | + ALC_CMS.3 | ハッシュと署名の付与 |
| A4 | + ATE_IND.2 | 独立主体による実行 |
| A5 | + AVA_VAN.2 | 脆弱性解析の取込 |

尺度を満たした条件を満たす構成要素の **最も高い** ものを採る。
A5 は自動実行では到達しない (人間による独立レビューを要する)。

出力: `evidence/test-run-<timestamp>-<spec-hash>.json`
スキーマ: `skills/test-spec/evidence/schema.json` に準拠すること。
既存フィールド (spec_hash / timestamp / environment / results / signature)
の必須条件は変更しない。追加フィールドはすべて任意。

禁止:

- 署名が空のエビデンスを保存しない
- `conformance` に `tailored` 以外を書かない
- 「EAL 準拠」「ASVS 準拠」等の表現をエビデンスに書かない
