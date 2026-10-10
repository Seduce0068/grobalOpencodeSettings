---
description: AI候補の自動収集・一次スクリーニング
agent: build
subagent: true
---

要件に基づき候補を自動収集・一次評価します。

要件ID: $ARGUMENTS (ai-selection/requirements/<id>.md のパス)
省略時は最新要件を使用。

収集ソース:
- クラウド: 公式価格・モデルカード・SLA・リージョン
- ローカル: HuggingFace Trending・llama.cpp/vLLM/TGI対応表・量子化対応
- ベンチマーク: LMSYS Arena・Open LLM Leaderboard・HELM・MTEB
- コスト: RunPod/Lambda/vast.ai/Hyperbolic GPU価格
- ライセンス: HuggingFaceライセンスタグ・商用利用可否

処理:
1. 要件の Must 制約でフィルタ (ライセンス/プライバシー/ハード互換)
2. 候補を構造化比較表にまとめ (プロバイダ/モデル/量子化/価格/ベンチマーク)
3. `ai-selection/candidates/<id>.md` 出力
4. 上位 5-7 候補を `/ai-selection benchmark-design` へ推薦