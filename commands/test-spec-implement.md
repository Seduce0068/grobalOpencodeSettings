---
description: 承認済み仕様からテストコード実装生成
agent: build
subagent: true
---

承認済みテスト仕様からテストコードを生成します。

対象: $ARGUMENTS (`specs/test-<feature>.md` のパスまたは feature 名)
省略時は最新の承認済み仕様から選択。

手順:

1. 仕様ファイル読み込み (`specs/test-*.md`)
2. 仕様内の Traceability 表を読み、1:1 の対応を確認する
3. 言語・フレームワーク検出 (package.json / Cargo.toml / pyproject.toml)
4. テンプレート適用でテストコード雛形生成
5. 実装コードとのシグネチャ整合性検証
6. 統制 ID と保証尺度の紐付け
   - 統制 ID は `references/test/catalog/nist-sp800-53-r5.2.csv` に
     実在することを確認する

   - ASVS ID は `references/test/catalog/owasp-asvs-5.0.0.csv` に
     実在することを確認する

   - 存在しない ID があれば FAIL として報告し、仕様側を修正する
7. `tests/` 配下に配置 (種別ごとにサブディレクトリ)
8. 初回実行 (`npm test` / `cargo test` / `pytest`) でパス確認
9. 失敗時は実装側の修正が必要か、テスト側の調整かを判定して報告
10. 実行結果を `/test-spec evidence` に渡してエビデンスを生成

禁止:

- 仕様に無いテストを「便宜のため」追加しない (追加は仕様更新を伴って行う)
- 仕様に無い ID を Traceability に補完しない
