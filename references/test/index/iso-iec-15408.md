---
type: Reference
title: "ISO/IEC 15408 (Common Criteria) 索引"
description: "CC:2022 のパート構成、保証構成要素の節番号、内部保証尺度の対応"
tags:
  - reference
  - iso-iec-15408
  - common-criteria
  - assurance
status: active
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## 適用範囲

Common Criteria は **TOE (Target of Evaluation) の評価** であり，
開発プロセスの評価ではない。`test-spec` にそのまま被せると範囲違いになる。

本スキルの適用対象は **根拠の信頼性** に限定する。
既存の evidence 機構 (spec_hash と署名) が担う「完全性と改ざん検知」の
評価軸に，CC の assurance component の考え方を割り当てる。

## シリーズ構成

CC:2022 は ISO/IEC 15408-1 から -5:2022 と，
CEM の ISO/IEC 18045:2022 で構成される。

| パート | 題名 |
| --- | --- |
| Part 1 | Introduction and general model |
| Part 2 | Security functional components |
| Part 3 | Security assurance components |
| Part 4 | Framework for the specification of evaluation methods and activities |
| Part 5 | Pre-defined packages of security requirements |

Part 1 の Clause 4 に適合の 3 形態 (intended usage / full / tailored)，
Part 1 の Clause 4.3 に tailor 化，GOM に exact conformance の
導入が含まれる。Part 5 に EAL と CAP の定義がある。

## 保証構成要素 (Part 3)

節番号と表題の完全な索引は `../catalog/iso-iec-15408-3-components.csv`
にある (69 件)。ファミリー別の主なクラスを次に示す。

| ファミリー | 内容 | 節 |
| --- | --- | --- |
| ADV | Development | 10 |
| AGD | Guidance documents | 11 |
| ALC | Life-cycle support | 12 |
| ASE | ST evaluation | 9 |
| ATE | Tests | 13 |
| AVA | Vulnerability assessment | 14 |

## 内部保証尺度

`test-spec` の内部尺度 A0 から A5 は，根拠となる CC 構成要素を
併記する。**EAL の名称は使わない**。「EAL 相当」を自称しないため。

| 尺度 | 根拠 CC 構成要素 | 条件 |
| --- | --- | --- |
| A0 | なし | 手動・記録なし |
| A1 | ATE_FUN.1 | 実行結果の記録 |
| A2 | + ATE_COV.2 | カバレッジ基準の充足 |
| A3 | + ALC_CMS.3 | 根拠のハッシュと署名 |
| A4 | + ATE_IND.2 | 独立主体による実行 |
| A5 | + AVA_VAN.2 | 脆弱性解析の取込 |

A5 は自動実行では到達しない。人間による独立レビューを要する。
この到達不能性の明示が A4 と A5 の境界である。

## 出典

- [Common Criteria (CCRA)][cc]
- [ISO/IEC 15408-1:2022][cc1]
- [CEM:2022 (ISO/IEC 18045:2022)][cem]

[cc]: https://commoncriteriaportal.org/cc/index.cfm
[cc1]: https://www.iso.org/standard/72891.html
[cem]: https://commoncriteriaportal.org/files/ccfiles/CEM2022R1.pdf
