# Unit Test Specification (BDD/Gherkin)

## Feature: {{FEATURE_NAME}}

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
|---------|-------------|-----------------|--------------|
| TC-001  | {{VALUE_1}} | {{RESULT_1}}    | {{EFFECT_1}} |
| TC-002  | {{VALUE_2}} | {{RESULT_2}}    | {{EFFECT_2}} |

### Scenario Outline: {{BOUNDARY_CASES}}
```gherkin
When {{ACTION}} with {{BOUNDARY_INPUT}}
Then {{BOUNDARY_RESULT}}
```

| case_id | BOUNDARY_INPUT | BOUNDARY_RESULT |
|---------|----------------|-----------------|
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
|---------|---------------|------------|---------------|
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
| Scenario | Requirement ID | Test Function | Status |
|----------|----------------|---------------|--------|
| TC-001   | REQ-{{ID}}     | `test_{{name}}` | ⬜ TODO |
| TC-B01   | REQ-{{ID}}     | `test_{{name}}_boundary` | ⬜ TODO |
| TC-E01   | REQ-{{ID}}     | `test_{{name}}_error` | ⬜ TODO |

---

## Implementation Notes
- **Framework**: {{Vitest|Jest|pytest|cargo test}}
- **Mock Strategy**: {{MSW|vi.mock|mockall|unittest.mock}}
- **Data Setup**: {{Factory|Fixture|Builder Pattern}}
- **Assertions**: {{expect|assert|prop_assert}}