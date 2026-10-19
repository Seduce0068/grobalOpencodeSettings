# Test Incident Report

## Target: {{FAILED_TEST_OR_DEFECT}}

> ISO/IEC/IEEE 29119-2:2021 の 8.5 Test incident reporting に対応する。
> 失敗したテストは「不具合」未満であり，まずインシデントとして記録する。
> 再現性・影響範囲・原因の未確定の段階を明示したうえで，
> 缺陷への昇格判断は別工程で行う。

## インシデント情報

| 項目 | 内容 |
| --- | --- |
| incident_id | {{INC-0001}} |
| 検出時点 | {{TIMESTAMP}} |
| 検出者 | {{ACTOR}} |
| 関連テスト | {{TEST_ID_OR_PATH}} |
| 関連仕様 | {{SPEC_PATH}} |
| ビルド | {{BUILD_ID}} |
| 環境 | {{ENVIRONMENT}} |
| ステータス | {{OPEN\|INVESTIGATING\|RESOLVED\|CLOSED}} |
| 深刻度 | {{SEVERITY}} |
| 緊急度 | {{URGENCY}} |
| 影響範囲 | {{IMPACT_SCOPE}} |

## 再現性

| 項目 | 内容 |
| --- | --- |
| 再現率 | {{N}}/{{M}} (直近 N 回中 M 回) |
| 再現手順 | numbered steps |
| 最小再現 | {{MINIMAL_REPRO}} |
| フレーキーか | {{YES\|NO}} |
| 初回検出 | {{COMMIT_OR_RELEASE}} |
| suspect commit | {{COMMIT_SHA}} |

### 再現手順

1. {{STEP_1}}
2. {{STEP_2}}
3. {{STEP_3}}

## 期待結果と実際結果

| 種別 | 内容 |
| --- | --- |
| 期待 (test oracle) | {{EXPECTED}} |
| 実際 | {{ACTUAL}} |
| 差分 | {{DIFF}} |
| ログ・スタック | {{LOG_EXCERPT}} |

## 影響分析

| 対象 | 影響 |
| --- | --- |
| 影響を受ける要件 | {{REQ_IDS}} |
| 影響を受ける統制 | {{SP800-53r5.2/XX-X}} |
| 影響を受ける ASVS 要件 | {{v5.0.0-V0.0.0}} |
| リリース判定への影響 | {{BLOCKER\|NON_BLOCKER}} |

## 原因 (未確定でもよい)

| 段階 | 内容 |
| --- | --- |
| 推定原因 | {{HYPOTHESIS}} |
| 確度 | {{CONFIRMED\|PROBABLE\|POSSIBLE}} |
| 根本原因 | {{ROOT_CAUSE}} (未確定なら `未定`) |
| 修正 | {{FIX_REFERENCE}} |
| 修正のテスト | {{REGRESSION_TEST_ID}} |

## 处置

| 項目 | 内容 |
| --- | --- |
| 処置 | {{FIX\|DEFER\|DUPLICATE\|NOT_A_DEFECT}} |
| Responsible | {{OWNER}} |
| 期限 | {{DUE_DATE}} |
| 欠陥登録先 | {{TRACKER_URL}} |

## 再発防止 (回归試験)

- [ ] 再発防止のテストを追加した
- [ ] 追加テストは仕様書 (Traceability) に紐付けた
- [ ] 修正と追加テストが同一コミットに含まれる
- [ ] エビデンスを再生成した (spec_hash の更新)

## 関連記録

| 種別 | 参照 |
| --- | --- |
| 実行ログ | `evidence/test-run-*.json` |
| 仕様書 | `specs/test-*.md` |
| レビュー | {{REVIEW_URL}} |
