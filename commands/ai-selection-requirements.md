---
description: AI選定要件定義ウィザード
agent: build
subagent: true
---

AI選定の要件・制約を定義します。

カテゴリ: $ARGUMENTS (cloud / local / hybrid / embedding / vector-db / rag-framework / guardrail)
省略時は全カテゴリ対話。

手順:
1. ユースケース・機能要件・非機能要件をヒアリング
2. `skills/ai-selection/templates/requirements.md` で構造化
3. 制約 (予算・プライバシー・ハード・ライセンス・コンプラ) を明文化
4. 品質基準 (合格ライン) を数値化
5. `ai-selection/requirements/<id>.md` 出力
6. `/ai-selection discover` で候補自動収集へ連携