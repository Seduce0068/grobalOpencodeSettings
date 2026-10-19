---
description: テスト仕様作成ウィザード
agent: build
subagent: true
---

テスト仕様書を作成します。

種類: $ARGUMENTS (unit-bdd / integration-api / e2e-playwright /
property-based / decision-table / state-transition / contract / mutation)
省略時は対話で選択。

プロファイル: $PROFILE (`references/test/profiles/*.yaml` のいずれか。
既定は `web-api-l2-moderate`)。未指定なら対話で選択する。

手順:

1. 種類・対象モジュール・要件・プロファイルをヒアリング
2. `skills/test-spec/templates/<type>.md` で雛形生成
3. 技法の選択と 29119-4 の節番号を記録
   (仕様ベースは 5.2.1-5.2.12，構造ベースは 5.3.1-5.3.7，
   経験ベースは 5.4.1。`references/test/index/iso-iec-ieee-29119.md` を参照)

4. Gherkin シナリオ・境界値・異常系を網羅
5. リスク項目と tier (T1-T3) を割当。完了基準はプロファイルの
   `risk_tiers` から決める

6. テスト基準 ID を割当
   - ASVS: `references/test/catalog/owasp-asvs-5.0.0.csv` から候補を提示
   - SP 800-53: `references/test/catalog/nist-sp800-53-r5.2.csv` から候補を提示
   - CC: `references/test/catalog/iso-iec-15408-3-components.csv` から該当成分
   いずれも **版付き ID** で記載する。該当なしは `-`

7. 必須 4 セクション (Test basis / Risk & tier / Design technique /
   Document map) を含む `specs/test-<feature>.md` を出力

8. Traceability 表を拡張列 (Scenario | Req | ASVS | 800-53 | CC |
   Technique | Tier | Status) で埋める

9. `/review-flow start` で仕様レビュー依頼
10. 承認後、`/test-spec implement` で実装生成へ連携

禁止:

- 「準拠」「認証済み」の表現を仕様書に入れない
- ASVS / SP 800-53 の ID を版なし (\`V1.2.5\` や \`AC-3\`) で書かない
- 空欄を残さず、該当なしは `-` で埋める
