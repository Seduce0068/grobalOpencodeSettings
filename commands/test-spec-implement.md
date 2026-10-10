---
description: 承認済み仕様からテストコード実装生成
agent: build
subagent: true
---

承認済みテスト仕様からテストコードを生成します。

対象: $ARGUMENTS (specs/test-<feature>.md のパスまたは feature 名)
省略時は最新の承認済み仕様から選択。

手順:
1. 仕様ファイル読み込み (`specs/test-*.md`)
2. 言語・フレームワーク検出 (package.json / Cargo.toml / pyproject.toml)
3. テンプレート適用でテストコード雛形生成
4. 実装コードとのシグネチャ整合性検証
5. `tests/` 配下に配置 (種別ごとにサブディレクトリ)
6. 初回実行 (`npm test` / `cargo test` / `pytest`) でパス確認
7. 失敗時は実装側の修正が必要か、テスト側の調整かを判定して報告