---
type: Reference
title: "ISO/IEC/IEEE 29119 索引"
description: "ソフトウェア試験 ([]29119) シリーズの版・節構成と節番号の対応"
tags:
  - reference
  - iso-iec-ieee-29119
  - test-spec
status: active
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## シリーズ構成 (2026-10-19 時点)

| パート | 版 | 題名 | 状態 |
| --- | --- | --- | --- |
| Part 1 | 2022 | General concepts | 発行済 |
| Part 2 | 2021 | Test processes | 発行済 |
| Part 3 | 2021 | Test documentation | 発行済 |
| Part 4 | 2021 | Test techniques | 発行済 |
| Part 5 | 2024 | Keyword-driven testing | 発行済 |
| Part 6 (TR) | 2021 | 全パートのアジャイル適用ガイド | 発行済 |
| Part 8 | 2026 | Model-based testing | FDIS (未発行) |
| Part 11 (TR) | 2020 | AI ベースシステム試験 | 発行済 |
| Part 13 | 2022 | バイオメトリクスシステムへの適用 | 発行済 |

静的試験 (レビュー) は本シリーズではなく ISO/IEC 20246 の領域である。

## Part 2 のプロセス構成

| 節 | プロセス | 情報項目 | 活動 |
| --- | --- | --- | --- |
| 6.2 | Organizational test process | test policy / practices | — |
| 7.2 | Test strategy and planning | test plan | TP1-TP9 |
| 7.3 | Test monitoring and control | test status report | TMC1-TMC5 |
| 7.4 | Test completion | test completion report | TC1-TC4 |
| 8.2 | Test design and implementation | test design / case / procedure spec | TD1-TD3 |
| 8.3 | Test environment and data management | environment / data requirements, readiness | — |
| 8.4 | Test execution | test result / actual result / execution log | — |
| 8.5 | Test incident reporting | incident report | — |

- TP4 はリスク処理手段 (試験レベル・試験型・設計技法・完了基準) の識別と記録
- TP5 は test basis・リスク・制約を根拠に test strategy を設計する
- 8.2 の TD1 は test model の作成、TD2 は test coverage item の識別、
  TD3 は test case の導出

- 2021 年版は test conditions の概念を test model に置換 (Annex E)

## Part 3 の文書構成

文書は Part 2 のプロセス節に対応して配置される。
Annex A に情報項目と Part 2 の節との shall / should / may 対応がある。
Clause 4 で nomenclature と contents の tailor 化和が公认されている。

| 分類 | 情報項目 |
| --- | --- |
| 組織 | test policy, organizational test practices |
| 管理 | test plan, test status report, test completion report |
| 動的 | test design / case / procedure specification |
| 動的 | test environment / data requirements, readiness report |
| 動的 | test result, actual result, test execution log |
| 動的 | incident report |

## Part 4 の技法

| 分類 | 技法 (節) |
| --- | --- |
| 仕様ベース | equivalence partitioning (5.2.1), classification tree method (5.2.2), boundary value analysis (5.2.3), syntax testing (5.2.4), combinatorial test design (5.2.5), decision table testing (5.2.6), cause-effect graphing (5.2.7), state transition testing (5.2.8), scenario testing (5.2.9), random testing (5.2.10), metamorphic testing (5.2.11), requirements-based testing (5.2.12) |
| 構造ベース | statement (5.3.1), branch (5.3.2), decision (5.3.3), branch condition (5.3.4), branch condition combination (5.3.5), MCDC (5.3.6), data flow (5.3.7) |
| 経験ベース | error guessing (5.4.1) |

カバレッジ測定は Clause 6。各技法の TD1 (test model) / TD2
(coverage item) / TD3 (test case) の 3 手順が共通構造である。

## 適合の形態

| 節 | 形態 |
| --- | --- |
| 4.1.1 | intended usage |
| 4.1.2 | full conformance |
| 4.1.3 | tailored conformance |

`test-spec` スキルが主張するのは **tailored** のみである。

## 出典

- [29119 シリーズ (ISO/IEC JTC 1/SC 7)][series]
- [29119-2:2021 Test processes][p2]
- [29119-3:2021 Test documentation][p3]
- [29119-4:2021 Test techniques][p4]
- [29119-1:2022 General concepts][p1]
- [ISO/IEC TR 29119-6:2021][p6]

[series]: https://committee.iso.org/sites/jtc1sc7/home/projects/flagship-standards/isoiecieee-29119-series.html
[p2]: https://www.iso.org/standard/79428.html
[p3]: https://standards.ieee.org/ieee/29119-3/7499/
[p4]: https://www.iso.org/standard/79430.html
[p1]: https://www.iso.org/standard/79427.html
[p6]: https://www.iso.org/standard/81293.html
