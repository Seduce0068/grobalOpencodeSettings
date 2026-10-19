# Contract Test Specification

## Target: {{CONSUMER}} <-> {{PROVIDER}}

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{CONSUMER}} が期待する {{PROVIDER}} の契約 |
| 根拠 (test basis) | {{API_SPEC}} / {{SCHEMA_FILE}} |
| 判定基準 (test oracle) | 契約 (schema) への適合 |
| プロファイル | {{PROFILE_NAME}} |

## Risk & tier

| risk_id | リスク | tier | 完了基準 |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | 契約違反 0 |

## Design technique

| 技法 | 29119-4 節 | 適用箇所 |
| --- | --- | --- |
| Requirements-based testing | 5.2.12 | 契約定義との 1:1 |
| Equivalence partitioning | 5.2.1 | 境界的な値 |
| Syntax testing | 5.2.4 | メッセージ構造 |

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test design specification | 契約定義 |
| Test case specification | 検証ケース |
| Test procedure specification | Provider 検証手順 |
| Test execution log / result | `evidence/test-run-*.json` |

---

## 1. 契約の定義

Pact 等の契約テストフレームワークでは，**期待する** 相互作用を
consumer 側のテストとして記述し，provider 側で検証する。

| 項目 | 内容 |
| --- | --- |
| consumer | {{CONSUMER_SERVICE}} |
| provider | {{PROVIDER_SERVICE}} |
| 契約ファイル | `pacts/{{CONSUMER}}-{{PROVIDER}}.json` |
| provider 検証 | `pact-provider-verifier` |
| CI 失敗時の扱い | provider 側を blocker にする |

## 2. 相互作用 (Interactions)

| interaction_id | メソッド | パス | リクエスト | 期待レスポンス | 備考 |
| --- | --- | --- | --- | --- | --- |
| I1 | GET | {{/path}} | {{HEADERS}} | 200 + {{SCHEMA}} | |
| I2 | POST | {{/path}} | {{BODY_SCHEMA}} | 201 + {{SCHEMA}} | |

## 3. 検証ケース

| case_id | interaction | 検証対象 | 期待 |
| --- | --- | --- | --- |
| TC-CT01 | I1 | 正常系 | 契約どおり |
| TC-CT02 | I1 | 存在しないリソース | 404 |
| TC-CT03 | I2 | 必須項目欠落 | 400 + エラー形式 |

## 4. テスト関数

```typescript
// consumer 側
const pact = given('{{CONSUMER}}')
  .uponReceiving('{{DESCRIPTION}}')
  .withRequest('GET', '{{/path}}')
  .willRespondWith(200, { body: {{SCHEMA}} });

describe('{{CONSUMER}} -> {{PROVIDER}}', () => {
  it('{{DESCRIPTION}}', async () => {
    await pact.verify({{PROVIDER_CLIENT}});
  });
});
```

## 5. 境界と例外

- 契約にないパス・メソッド_combination は拒否されることを確認する
- provider が evolution (破壊的変更) を含む場合は，
  破壊的変更である旨を契約ファイルに記録する

- 契約の再生成は CI で行い，手元での手編集を禁止する

## 6. Traceability

| case_id | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TC-CT01 | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.12 | T1 | ⬜ TODO |

ASVS / 800-53 / CC の各列は該当なしを `-` とする。空欄は禁止。
