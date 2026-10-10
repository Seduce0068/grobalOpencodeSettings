---
description: ベンチマーク結果から採用決定・ADR記録
agent: build
subagent: true
---

ベンチマーク結果を評価し、採用決定を記録します。

結果ID: $ARGUMENTS (ai-selection/results/<id>.json のパス)
省略時は最新結果を使用。

処理:
1. 結果レポート・スコアカード読み込み
2. 要件の Must/Should/Could 照合
   - Must 未達 → 即却下 (理由記録)
   - Should 未達 → 条件付き採用 (改善計画必須)
   - Could 未達 → 参考記録
3. コスト試算 (月次・年次・スケール時) 再確認
4. リスク評価: ベンダーロックイン・ライセンス変更・サポート終了・性能ドリフト
5. `/doc-create adr "ai-selection: <category> <model/provider>"` 実行
   ADR 内容:
   - Decision: 採用モデル/プロバイダ/エンジン/量子化/バージョン
   - Context: 要件・制約・候補比較サマリ
   - Consequences: 期待メリット・既知トレードオフ・運用負荷
   - Alternatives: 却下候補・却下理由
   - Routing Rules (ハイブリッド時): 機密→ローカル, 長文脈→ローカル, 低レイテンシ→ローカル
   - Fallback: フォールバック先・フェイルオーバー条件
   - Monitoring: 監視指標・アラート閾値・ドリフト検知ルール
   - Review Trigger: 月次ベンチマーク・新版リリース・価格改定・脆弱性
6. `ai-selection/decisions/<id>.md` 記録
7. 導入タスク生成: インフラ・シークレット・監視・ドキュメント