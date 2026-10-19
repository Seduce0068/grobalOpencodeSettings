---
type: Reference
title: "test 一次参照とカタログ"
description: "テスト系スキルが参照する一次情報の索引と、固定版カタログの管理規約"
tags:
  - reference
  - test-spec
  - catalog
status: active
generated:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## 役割

`test-spec` スキルが参照する一次情報を保持する。
条文の丸写しは置かず、**節番号・表題・識別子・出典 URL のみ**を記録する。
検証可能な識別子 (ID) を機械可読なカタログとして固定し、
成果物のトレーサビリティを ID で張れるようにする。

## ディレクトリ構成

| パス | 内容 |
| --- | --- |
| `index/` | 規格ごとの版・節番号・表題の索引 |
| `catalog/` | 固定版の機械可読カタログ (CSV) |
| `profiles/` | 検証プロファイル (ベースライン定義) |

## 索引一覧

| ファイル | 対象 |
| --- | --- |
| `index/iso-iec-ieee-29119.md` | ソフトウェア試験シリーズ |
| `index/iso-iec-15408.md` | Common Criteria |
| `index/owasp.md` | OWASP 各プロジェクト |
| `index/nist-sp-800.md` | NIST SP 800 シリーズ |

## カタログ一覧

| ファイル | 件数 | 版 |
| --- | --- | --- |
| `catalog/owasp-asvs-5.0.0.csv` | 345 要件 | ASVS 5.0.0 |
| `catalog/owasp-wstg-v4.2.csv` | 98 シナリオ | WSTG v4.2 |
| `catalog/owasp-top10-2025.csv` | 10 リスク | Top 10 2025 |
| `catalog/owasp-masvs-2.1.0.csv` | 24 制御 | MASVS 2.1.0 |
| `catalog/nist-sp800-53-r5.2.csv` | 1196 統制 | SP 800-53 r5.2.0 |
| `catalog/iso-iec-15408-3-components.csv` | 69 構成要素 | 15408-3:2022 |

## カタログの書式

| ファイル | キー列 |
| --- | --- |
| `owasp-asvs-5.0.0.csv` | `ref_id, req_id, level, chapter_id, ...` |
| `owasp-wstg-v4.2.csv` | `ref_id, test_id, category, name, reference` |
| `owasp-top10-2025.csv` | `ref_id, risk_id, title` |
| `owasp-masvs-2.1.0.csv` | `ref_id, control_id, group_id, ...` |
| `nist-sp800-53-r5.2.csv` | `control_id, family, title` |
| `iso-iec-15408-3-components.csv` | `clause, component_id, title` |

`ref_id` は **版を含む**。`v5.0.0-V1.2.5` のように表記し、
版なし ID (例 `V1.2.5`) の単独使用は禁止する。

## 更新手順

1. 上流の新しい版を確認し，取得する
2. カタログを **版単位** で再生成する
3. 件数と ID の重複・欠落を検査する
4. 索引の版表記と取得日を更新する
5. 差分を 1 コミットにまとめる

## 出典

| 対象 | 取得元 |
| --- | --- |
| ASVS 5.0.0 | `OWASP/ASVS` タグ `v5.0.0` の `5.0/docs_en/*.csv` |
| WSTG v4.2 | `OWASP/wstg` タグ `v4.2` の `checklist/checklist.json` |
| Top 10 2025 | `OWASP/Top10` の `2025/docs/en/A*.md` |
| MASVS 2.1.0 | `OWASP/MASVS` タグ `v2.1.0` の `controls/*.md` |
| SP 800-53 r5.2.0 | `usnistgov/oscal-content` の `nist.gov/SP800-53/rev5/json/` |
| 15408-3:2022 | `commoncriteriaportal.org` の CC:2022 Part 3 目次 |

## ライセンス

| 対象 | ライセンス |
| --- | --- |
| OWASP ASVS / WSTG / Top 10 / MASVS | CC BY-SA 4.0 / CC BY-SA 3.0 |
| NIST SP 800-53 (OSCAL) | 米国政府著作物 (public domain) |
| ISO/IEC 15408 | 有償標準。節番号と表題のみを記載 |

有償標準については **条文本文を一切転載しない**。引用は節番号と
表題に限り、原文の解釈・要約の丸写しは行わない。
