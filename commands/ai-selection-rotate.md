---
description: モデルローテーション・新版評価・切替実行
agent: build
subagent: true
---

定期的なモデル見直し・新版評価・切替を実行します。

トリガー: $ARGUMENTS (scheduled / new-release / price-change / vulnerability / manual)
- `scheduled`: 月次/四半期定期実行
- `new-release`: HuggingFace/プロバイダ新版検知時
- `price-change`: 価格改定検知時
- `vulnerability`: CVE/ライセンス変更検知時
- `manual`: 手動指定

処理フロー:

### 1. 変更検知・影響範囲特定
- 対象: 採用中モデル/プロバイダ/エンジンの新版・価格・脆弱性
- ソース: HuggingFace RSS・プロバイダChangelog・GitHub Releases・CVE DB・価格API
- 影響: 該当 decision_id・依存サービス・ルーティングルール

### 2. 自動ベンチマーク実行
- 現行版 vs 新版 (同一ベンチマーク仕様)
- 量子化別比較 (該当時)
- コスト再試算 (新価格・新性能)
- 所要時間: ~30-60 分

### 3. 判定マトリクス

| 条件 | 判定 | アクション |
|---|---|---|
| 品質向上 + コスト同等/低減 | **Auto-Promote** | カナリア 10% → 50% → 100% |
| 品質同等 + コスト大幅低減 | **Promote** | 承認後段階的切替 |
| 品質低下許容範囲 + コスト大幅低減 | **Conditional** | ユースケース限定・監視強化 |
| 品質大幅低下 / コスト増大 | **Reject** | 現行維持・理由記録 |
| 脆弱性 (CVSS ≥ 7) | **Emergency Patch** | 即時切替・フォールバック実行 |

### 4. 切替実行 (Promote 時)
1. ADR 更新 (Superseded → 新版 Accepted)
2. インフラ更新: コンテナイメージ・モデルファイル・設定
3. カナリアデプロイ: 5% → 25% → 100% (各段階 30 分監視)
4. ロールバック準備: 即時切戻しスクリプト・データ互換性確認
5. 完了通知・ドキュメント更新

### 5. 記録
- `ai-selection/rotations/<decision_id>-<date>.md`
  - トリガー・比較結果・判定根拠・切替手順・ロールバック計画
- メトリクス: 切替前後 24h 比較 (品質・コスト・エラー率)

コマンド:
- `ai-selection rotate --check` : 変更検知のみ・レポート
- `ai-selection rotate --auto` : 検知→ベンチ→判定→承認待ち
- `ai-selection rotate --execute <decision_id> <new_version>` : 手動切替実行
- `ai-selection rotate --rollback <decision_id>` : 緊急ロールバック