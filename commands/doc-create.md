---
description: ドキュメント作成ウィザードを開始する
agent: build
---

ドキュメント作成を開始します。

種類: $ARGUMENTS (adr / rfc / runbook / api-spec / data-dict / threat-model)
省略時は対話で選択。

手順:
1. 種類・タイトル・対象コンポーネントをヒアリング
2. 該当テンプレート (`skills/doc-create` 参照) で骨子生成
3. 既知事実をコードベースから収集し埋め込み
4. `cross-review` で技術レビュー依頼
5. 承認後、所定ディレクトリに配置・インデックス更新