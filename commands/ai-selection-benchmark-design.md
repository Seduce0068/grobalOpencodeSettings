---
description: ベンチマーク設計・仕様書生成
agent: build
subagent: true
---

要件・候補に基づきベンチマーク仕様を生成します。

入力: $ARGUMENTS (requirements_id candidates_id またはパス)
省略時は最新のものを使用。

処理:
1. 要件の品質基準・コスト制約・スケールを読み込み
2. 候補のモデル・量子化・エンジン構成を読み込み
3. `skills/ai-selection/templates/benchmark.yaml` ベースで生成
   - シナリオ選択 (要件の優先タスクに合わせて)
   - 閾値設定 (要件の合格ラインを反映)
   - 量子化比較有効化 (ローカル候補ありの場合)
   - コストトラッキング設定 (クラウド/ローカル/ハイブリッド)
   - ハイブリッドルーティングルール (機密/長文脈/低レイテンシ)
4. `ai-selection/benchmarks/<id>.yaml` 出力
5. `/ai-selection benchmark-run` 実行へ連携