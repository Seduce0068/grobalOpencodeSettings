# Unit Test Specification (BDD/Gherkin)

## Feature: {{FEATURE_NAME}}

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{MODULE_OR_FUNCTION}} |
| 根拠 (test basis) | {{REQUIREMENT_DOC}} / {{ISSUE_ID}} |
| 判定基準 (test oracle) | {{EXPECTED_VALUE_SOURCE}} |
| プロファイル | {{PROFILE_NAME}} |

## Risk & tier

| risk_id | リスク | tier | 完了基準 (カバレッジ) |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | unit {{COV}} / integration {{COV}} |

tier は `references/test/profiles/<profile>.yaml` の `risk_tiers` を参照する。
T1 は安全・認証・金貨に関わる機能を指す。

## Design technique

| 技法 | 29119-4 節 | 適用箇所 |
| --- | --- | --- |
| {{EQUIVALENCE_PARTITIONING}} | 5.2.1 | 正常系・異常系 |
| {{BOUNDARY_VALUE_ANALYSIS}} | 5.2.3 | TC-B* |
| {{ERROR_GUESSING}} | 5.4.1 | チェックリスト |

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test case specification | 本ファイル全体 |
| Test design specification | Design technique |
| Test procedure specification | Implementation Notes |
| Test execution log / result | `evidence/test-run-*.json` |

### Background

```gherkin
Given {{SYSTEM_UNDER_TEST}} is initialized
  And {{PRECONDITIONS}}
```

### Scenario Outline: {{NORMAL_CASE}}

```gherkin
When {{ACTION}} with {{VALID_INPUT}}
Then {{EXPECTED_OUTPUT}}
And {{SIDE_EFFECTS}}
```

| case_id | VALID_INPUT | EXPECTED_OUTPUT | SIDE_EFFECTS |
| --- | --- | --- | --- |
| TC-001  | {{VALUE_1}} | {{RESULT_1}}    | {{EFFECT_1}} |
| TC-002  | {{VALUE_2}} | {{RESULT_2}}    | {{EFFECT_2}} |

### Scenario Outline: {{BOUNDARY_CASES}}

```gherkin
When {{ACTION}} with {{BOUNDARY_INPUT}}
Then {{BOUNDARY_RESULT}}
```

| case_id | BOUNDARY_INPUT | BOUNDARY_RESULT |
| --- | --- | --- |
| TC-B01  | {{MIN_VALUE}}  | {{MIN_RESULT}}  |
| TC-B02  | {{MAX_VALUE}}  | {{MAX_RESULT}}  |
| TC-B03  | {{OFF_BY_ONE}} | {{ERROR}}       |

### Scenario Outline: {{ERROR_CASES}}

```gherkin
When {{ACTION}} with {{INVALID_INPUT}}
Then {{ERROR_TYPE}} is thrown
And {{ERROR_MESSAGE}}
And {{NO_SIDE_EFFECTS}}
```

| case_id | INVALID_INPUT | ERROR_TYPE | ERROR_MESSAGE |
| --- | --- | --- | --- |
| TC-E01  | {{NULL}}      | {{TYPE}}   | {{MSG}}       |
| TC-E02  | {{MALFORMED}} | {{TYPE}}   | {{MSG}}       |
| TC-E03  | {{OUT_OF_RANGE}} | {{TYPE}} | {{MSG}}       |

### Scenario: {{CONCURRENCY}} (if applicable)

```gherkin
Given {{SHARED_STATE}}
When {{PARALLEL_ACTIONS}}
Then {{CONSISTENCY_GUARANTEE}}
And {{NO_DATA_RACE}}
```

---

## Acceptance Criteria Checklist

- [ ] 全正常系ケース網羅
- [ ] 境界値 (min/max/off-by-one) 網羅
- [ ] 異常系 (null/不正形式/範囲外/権限不足) 網羅
- [ ] 並行アクセス・冪等性 (必要な場合)
- [ ] パフォーマンス基準 (p99 < X ms) (必要な場合)
- [ ] セキュリティ (入力検証・認可・情報漏洩なし)

---

## Traceability

| Scenario | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TC-001   | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.1 | T2 | ⬜ TODO |
| TC-B01   | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.3 | T2 | ⬜ TODO |
| TC-E01   | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.4.1 | T2 | ⬜ TODO |

- `ASVS` は版付き ID (`v5.0.0-V1.2.5`)。該当なしは `-`
- `800-53` は Release 付き (`SP800-53r5.2/AC-3`)。該当なしは `-`
- `CC` は構成要素 ID (`ATE_FUN.1`)。該当なしは `-`
- 空欄は禁止。`-` を入れる

---

## Implementation Notes

- **Framework**: {{Vitest|Jest|pytest|cargo test}}
- **Mock Strategy**: {{MSW|vi.mock|mockall|unittest.mock}}
- **Data Setup**: {{Factory|Fixture|Builder Pattern}}
- **Assertions**: {{expect|assert|prop_assert}}
