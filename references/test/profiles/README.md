---
type: Reference
title: "検証プロファイル"
description: "テストで何を確認するか、どの基準で合格とするかを宣言するベースライン定義"
tags:
  - reference
  - test-spec
  - profile
status: active
verified:
  by: opencode
  at: "2026-10-19T22:19:11+09:00"
---

## 役割

検証プロファイルは「そのプロジェクトが何を主張するか」の宣言である。
4 層の選択を 1 ファイルに集約し，`/test-spec` の各コマンドの
第一引数として使う。

**外部への適合宣言ではない。** ファイル冒頭へこの文を固定で記載する。

## フィールド

| フィールド | 内容 |
| --- | --- |
| `profile` | プロファイル名 |
| `basis.iso29119` | 29119 の適合形態 (本スキルは tailored のみ) |
| `basis.asvs` | ASVS の版とレベル |
| `basis.nist800_53` | SP 800-53 の Release とベースライン |
| `basis.cc_assurance` | 根拠 CC 構成要素 |
| `techniques` | 採用する試験設計技法と 29119-4 の節番号 |
| `gates` | 完了基準 (カバレッジ・フレーキー・保証尺度) |

## 既存プロファイル

| ファイル | 対象 |
| --- | --- |
| `web-api-l2-moderate.yaml` | web / API。ASVS L2 + SP 800-53 moderate |
| `rust-lib-moderate.yaml` | Rust ライブラリ。統制は外部依存のみ |

## 作成規則

1. `basis` に指定のない層は「適用しない」を意味する
2. `techniques` は 29119-4 の節番号を必ず併記する
3. `gates.assurance_target` は A0 から A5 のいずれか
4. 既存プロファイルは **編集せず複製して派生** する

## 出典

- [ISO/IEC/IEEE 29119 索引](index/iso-iec-ieee-29119.md)
- [OWASP 索引](index/owasp.md)
- [NIST SP 800 索引](index/nist-sp-800.md)
- [ISO/IEC 15408 索引](index/iso-iec-15408.md)
