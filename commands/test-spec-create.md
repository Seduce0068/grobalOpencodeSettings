---
description: テスト仕様作成ウィザード
agent: build
subagent: true
---

テスト仕様書を作成します。

種類: $ARGUMENTS (unit-bdd / integration-api / e2e-playwright / property-based / contract / mutation)
省略時は対話で選択。

手順:
1. 種類・対象モジュール・要件をヒアリング
2. `skills/test-spec/templates/<type>.md` で雛形生成
3. Gherkin シナリオ・境界値・異常系を網羅
4. `specs/test-<feature>.md` 出力
5. `/review-flow start` で仕様レビュー依頼
6. 承認後、`/test-spec implement` で実装生成へ連携