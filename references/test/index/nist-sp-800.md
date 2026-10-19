---
type: Reference
title: "NIST SP 800 シリーズ索引"
description: "SP 800-53 統制カタログ、800-115、800-63-4、800-218、800-140 の版と用途"
tags:
  - reference
  - nist-sp-800
  - control
status: active
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## 採用文書

| 文書 | 版 | 用途 | カタログ |
| --- | --- | --- | --- |
| SP 800-53 | Release 5.2.0 (2025-08-27) | 統制カタログ | `nist-sp800-53-r5.2.csv` |
| SP 800-53A | Rev. 5 | 評価手順 (OSCAL に統合済み) | 別カタログ化しない |
| SP 800-53B | Rev. 5 | ベースライン low / moderate / high | プロファイルで選択 |
| SP 800-115 | 2008-09-30 | 試験・評価手法の分類 | 技法の別軸 |
| SP 800-63 | Rev. 4 (2025-07) | デジタル ID ガイドライン | 認証認可の根拠 |
| SP 800-218 | v1.1 (SSDF) | 開発側の安全な実践 | review-flow 側 |
| SP 800-140 | 2019-03-20 | 派生試験要求 (DTR) の先例 | 書式の参照 |

## 統制カタログ

SP 800-53 rev5 のカタログは **OSCAL** (JSON / XML / YAML) で公式配布される。
自前の CSV 生成は不要で，取得元は `usnistgov/oscal-content` の
`nist.gov/SP800-53/rev5/json/` である。

`nist-sp800-53-r5.2.csv` は OSCAL の catalog から
`control_id, family, title` の 3 列で派生した写しである。
ESCO の要素 (control enhancement) は `AC-2(1)` のような
親 ID 括弧表記で含まれる。

差分更新は OSCAL Content のリリースに従う。
v1.5.0 は SP 800-53 v5.2.0 / SP 800-171 rev3 / SP 800-218 v1 を含む。

## 統制 ID の扱い

- 統制 ID は大文字表記 (例 `AC-3`, `SC-12`) で保持する
- 統制 ID の参照は版と.Release 番号を伴う (例 `SP800-53r5.2/AC-3`)
- SP 800-53A の評価手順 (objective) は統制ごとに 1 つ以上ある

## 主張の境界

SP 800-53 自体は適合を保証しない。組織は **tailoring した
ベースライン** に対して統制の満足を宣言する。
スキルは統制の **対応可否の可視化** までを行い，
適合の宣言は行わない。

## 出典

- [SP 800-53 r5 (CSRC)][sp53]
- [SP 800-53 downloads (CSV / OSCAL)][sp53dl]
- [OSCAL content][oscal]
- [SP 800-115][sp115]
- [SP 800-63-4][sp63]
- [SP 800-218 SSDF][ssdf]
- [SP 800-140 DTR][sp140]

[sp53]: https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
[sp53dl]: https://csrc.nist.gov/projects/risk-management/sp800-53-controls/downloads
[oscal]: https://github.com/usnistgov/oscal-content
[sp115]: https://csrc.nist.gov/pubs/sp/800/115/final
[sp63]: https://csrc.nist.gov/pubs/sp/800/63/4/final
[ssdf]: https://csrc.nist.gov/pubs/sp/800/218/final
[sp140]: https://csrc.nist.gov/pubs/sp/800/140/final
