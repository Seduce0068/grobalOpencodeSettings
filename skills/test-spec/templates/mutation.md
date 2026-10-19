# Mutation Test Specification

## Target: {{MODULE_OR_SOURCE}}

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{MODULE_OR_SOURCE}} |
| 根拠 (test basis) | 構造ベース技法の被変異 (29119-4 5.3) |
| 判定基準 (test oracle) | 変異が検出されること (テストの感受性) |
| プロファイル | {{PROFILE_NAME}} |

## Risk & tier

| risk_id | リスク | tier | 完了基準 |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | mutation score {{SCORE}} |

## Design technique

| 技法 | 29119-4 節 | 対応する変異 |
| --- | --- | --- |
| Statement testing | 5.3.1 | 条件の反転・演算子の変更 |
| Branch testing | 5.3.2 | 条件の反転・境界値の変更 |
| Decision testing | 5.3.3 | 複合条件の変更 |
| MCDC | 5.3.6 | 独立条件の変更 |

**位置づけ**: 変異スコアは 29119-4 Clause 6 の正規カバレッジ測定
には含まれない。本テンプレートで使う変異スコアは **構造ベース技法の
proxy 指標** であり，正規の測定値 (statement / branch / decision /
MCDC coverage) と併記する。

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test design specification | 適用する変異_OPERATOR |
| Test case specification | 既存テストとの対応 |
| Test execution log / result | `evidence/test-run-*.json` |

---

## 1. 適用する変異 operator

| operator | 対象 | 変異内容 | 対応する技法 |
| --- | --- | --- | --- |
| {{CONDITIONALS_BOUNDARY}} | 条件 | `>` と `>=` の交換 | 5.3.2 branch |
| {{NEGATE_CONDITIONALS}} | 条件 | 真偽の反転 | 5.3.1 statement |
| {{ARITHMETIC}} | 演算 | 演算子の置換 | 5.3.3 decision |
| {{RETURN_VALUES}} | 戻り値 | 既定値への変更 | 5.3.1 statement |

## 2. ツール設定

| 対象 | 設定 |
| --- | --- |
| ツール | {{cargo mutants / Stryker / gremlins.js}} |
| 対象パス | {{SRC_PATH}} |
| 除外 | {{EXCLUDE_PATTERNS}} (生成コード・テストコード) |
| 実行時間制限 | {{TIMEOUT}} |

## 3. 結果

| 指標 | 値 | 基準 |
| --- | --- | --- |
| mutation score | {{SCORE}} | {{THRESHOLD}} |
| killed | {{N}} | |
| survived | {{N}} | 0 目標 |
| timeout | {{N}} | |
| no coverage | {{N}} | 0 目標 |

## 4. 生存変異体の対応

生存変異体は「テストが敏感でない」ことを示す。1 件ずつ 대응を
記録し，テストを追加するか，対象外として理由を残す。

| survived_id | 位置 | 変異内容 | 対応 |
| --- | --- | --- | --- |
| M-001 | {{FILE:LINE}} | {{MUTATION}} | テスト追加 / 対象外 (理由) |

生存変異体を放置しない。`verify` は survived > 0 で FAIL とする
(プロファイルで対象外としたものは除外)。

## 5. テスト関数

```typescript
describe('{{MODULE_OR_SOURCE}} の変異耐性', () => {
  it('条件の反転を検出できる', () => {
    // 変異の意図を明示する
    expect({{FUNCTION_NAME}}({{INPUT}})).toBe({{EXPECTED}});
  });
});
```

## 6. 他のカバレッジとの併記

| 測定 | 値 | 29119-4 節 |
| --- | --- | --- |
| statement coverage | {{PCT}} | 5.3.1 |
| branch coverage | {{PCT}} | 5.3.2 |
| decision coverage | {{PCT}} | 5.3.3 |
| MCDC coverage | {{PCT}} | 5.3.6 |
| mutation score (proxy) | {{PCT}} | なし (proxy) |

## 7. Traceability

| case_id | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TC-M01 | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.3.2 | T2 | ⬜ TODO |

ASVS / 800-53 / CC の各列は該当なしを `-` とする。空欄は禁止。
