---
type: Reference
title: "OWASP 索引"
description: "ASVS / WSTG / Top 10 / MASVS / SAMM の版、ID 規則、役割分担"
tags:
  - reference
  - owasp
  - appsec
status: active
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## 役割分担

| 成果物 | 版 | 問い | カタログ |
| --- | --- | --- | --- |
| ASVS | 5.0.0 (2025-05) | 何を検証すべきか | `owasp-asvs-5.0.0.csv` |
| WSTG | v4.2 (2020-12) | どう試験するか | `owasp-wstg-v4.2.csv` |
| Top 10 | 2025 | どのリスクが重大か | `owasp-top10-2025.csv` |
| MASVS | 2.1.0 | モバイル何をすべきか | `owasp-masvs-2.1.0.csv` |
| SAMM | 2.2.0 | プロセスはどこまで成熟か | カタログ化しない |

ASVS を主軸とする。ASVS は「検証できること」を目的に設計され，
要件 ID が版を含んで定義され，CSV / JSON が配布される。

## ID 規則

| 成果物 | 参照形式 | 例 |
| --- | --- | --- |
| ASVS | `v<version>-<req_id>` | `v5.0.0-V1.2.5` |
| WSTG | `v<version>-<test_id>` | `v4.2-WSTG-INFO-02` |
| Top 10 | `<year>-<risk_id>` | `2025-A01` |
| MASVS | `v<version>-<control_id>` | `v2.1.0-MASVS-STORAGE-1` |

版を含まない ID の単独使用は禁止する。ID は版をまたいでも
意味が変わらないよう、版を必ず添える。

### 上流の ID 重複

`OWASP/wstg` タグ `v4.2` の `checklist/checklist.json` は
`WSTG-INPV-13` を 2 つのシナリオに割り当てている
(Buffer Overflow と Format String Injection)。
v4.2 リリースノートでは Buffer Overflow 側が削除され
Format String Injection が `WSTG-INPV-13` を継承しているため，
カタログでは Format String Injection 側を正とする。
残ったlegacy エントリは `note` 列に重複的事实と
`ref_id` への slug 付与で区別している。

## レベルの扱い

ASVS は 3 レベル (L1 / L2 / L3) を持つ。章・節単位での絞り込みが可能で，
対象スコープに含めない要件は除外理由を明記する。

報告の形式は「対象スコープに含めた要件の一覧 + 例外 + 除外理由」であり，
「合格した要件の一覧」ではない。

## 生成物の禁止表現

OWASP は Top 10 の full coverage 主張を明確に否定しており，
自動テストの実行と要件の検証は別と明言している。よって次を出さない。

- 「Top 10 対応完了」
- 「ASVS L2 準拠」

出力するのは **要求単位の到達可否** までとする。

## 出典

- [OWASP ASVS][asvs] / [ASVS 5.0.0 リリース][asvs50]
- [OWASP WSTG v4.2][wstg]
- [OWASP Top 10][top10]
- [OWASP MASVS][masvs]
- [OWASP SAMM][samm]

[asvs]: https://owasp.org/projects/asvs
[asvs50]: https://github.com/OWASP/ASVS/releases/tag/v5.0.0_release
[wstg]: https://wstg.owasp.org/v4.2/
[top10]: https://owasp.org/projects/top-ten
[masvs]: https://mas.owasp.org/MASVS/
[samm]: https://owasp.org/www-project-samm/
