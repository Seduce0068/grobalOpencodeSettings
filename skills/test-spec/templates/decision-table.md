# Decision Table Test Specification

## Target: {{RULE_OR_POLICY}}

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{RULE_OR_POLICY}} (認可・_validation・価格計算など) |
| 根拠 (test basis) | {{SPEC_DOC}} |
| 判定基準 (test oracle) | 決定表の各行の期待結果 |
| プロファイル | {{PROFILE_NAME}} |

## Risk & tier

| risk_id | リスク | tier | 完了基準 |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | 決定表の行 100% / カバレッジ {{COV}} |

## Design technique

| 技法 | 29119-4 節 | 適用箇所 |
| --- | --- | --- |
| Decision table testing | 5.2.6 | 本ファイル全体 |
| Equivalence partitioning | 5.2.1 | 条件列の値クラス |
| Requirements-based testing | 5.2.12 | 仕様との 1:1 |

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test design specification | 決定表 (test model) |
| Test case specification | テスト関数マッピング |
| Test procedure specification | 実行順序 |
| Test execution log / result | `evidence/test-run-*.json` |

---

## 1. 条件列 (Conditions)

技法 5.2.6 の TD1 (test model) として，条件を列として定義する。

| 列 | 条件 | 型 | 値クラス (5.2.1) |
| --- | --- | --- | --- |
| C1 | {{CONDITION_1}} | boolean | true / false |
| C2 | {{CONDITION_2}} | enum | {{V1}} / {{V2}} / {{V3}} |
| C3 | {{CONDITION_3}} | range | {{MIN}}..{{MAX}} |

## 2. 動作列 (Actions)

| 列 | 動作 | 結果 |
| --- | --- | --- |
| A1 | {{ACTION_1}} | {{RESULT_1}} |
| A2 | {{ACTION_2}} | {{RESULT_2}} |

## 3. 決定表 (Decision Table)

省略値 `-` は「don't care」を表す。
規則の独立性はTD2 (test coverage item) として確認する。

| Rule | C1 | C2 | C3 | A1 | A2 | 説明 |
| --- | --- | --- | --- | --- | --- | --- |
| R-1 | T | {{V1}} | - | X | - | {{RULE_1}} |
| R-2 | F | {{V1}} | - | - | X | {{RULE_2}} |
| R-3 | T | {{V2}} | {{MAX}} | X | - | {{RULE_3}} |

### 2-valued / 3-valued の判定

- 2-valued (T/F のみ) の場合，規則数は `2^n` まで膨張しうる
- 3-valued (T/F/don't care) にするのは和集合が安全と判断した場合のみ
- 適用外 (N/A) を使う場合は documented rationale を残す

## 4. テストケースへの写像 (TD3)

| case_id | Rule | 入力 | 期待動作 | 期待結果 | 備考 |
| --- | --- | --- | --- | --- | --- |
| TC-D01 | R-1 | {{INPUT_1}} | {{A1}} | {{RESULT_1}} | |
| TC-D02 | R-2 | {{INPUT_2}} | {{A2}} | {{RESULT_2}} | |
| TC-D03 | R-3 | {{INPUT_3}} | {{A1}} | {{RESULT_3}} | 境界値含む |

各規則に 1 つ以上のケースを割り当てる。
未割り当ての規則があれば `verify` が FAIL とする。

## 5. テスト関数

```typescript
describe('{{RULE_OR_POLICY}}', () => {
  test.each([
    // [rule, input, expectedAction, expectedResult]
    ['R-1', { c1: true, c2: '{{V1}}' }, 'A1', {{RESULT_1}}],
    ['R-2', { c1: false, c2: '{{V1}}' }, 'A2', {{RESULT_2}}],
  ])('rule %s', (_rule, input, expectedAction, expectedResult) => {
    expect({{FUNCTION_NAME}}(input)).toEqual(expectedResult);
  });
});
```

## 6. 網羅性の確認

| 確認項目 | 方法 | 合格条件 |
| --- | --- | --- |
| 規則の網羅 | 各 Rule にケースがあること | 0 欠落 |
| 条件の冗長 | 2 規則で同じ入力集合になっていないか | 重複 0 |
| 矛盾 | 同じ入力に複数の規則がapplicableでないか | 矛盾 0 |

## 7. Traceability

| case_id | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TC-D01 | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.6 | T1 | ⬜ TODO |

ASVS / 800-53 / CC の各列は該当なしを `-` とする。空欄は禁止。
