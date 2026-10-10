---
description: 本番監視・ドリフト検知・アラート
agent: build
subagent: true
---

本番環境の AI モデル性能・コスト・品質を継続監視します。

対象: $ARGUMENTS (decision_id または all)
省略時は全採用モデル。

監視項目 (リアルタイム/日次/週次):

| 指標 | 閾値 | アクション |
|---|---|---|
| **p99 TTFT** | > 2x ベンチマーク値 | Warning → 1h 継続で Alert |
| **p99 E2E Latency** | > 2x ベンチマーク値 | Warning |
| **Error Rate** | > 1% | Immediate Alert |
| **OOM Rate** | > 0 | Immediate Alert |
| **Throughput** | < 0.5x ベンチマーク値 | Warning |
| **Cost / 1M tokens** | > 1.5x 試算値 | Weekly Review |
| **Quality Drift** (サンプル評価) | Score < 0.9x ベンチマーク | Monthly Deep Dive |
| **Token Usage** | 予算 80% 到達 | Budget Alert |

実装:
- メトリクス収集: OpenTelemetry + Prometheus + Grafana
- 品質サンプリング: 1% リクエストをルーブリック評価 (LLM-as-a-Judge)
- コスト集計: 日次バッチでプロバイダ請求 / GPU 使用量取得
- ダッシュボード: Grafana / Langfuse / LangSmith / Weights&Biases

出力:
- `ai-selection/monitoring/<decision_id>-<date>.json` (日次サマリ)
- アラート: Slack/PagerDuty/Email (Alertmanager 経由)
- 週次レポート: `ai-selection/reports/monitoring-weekly-<date>.md`

コマンド:
- `ai-selection monitor --daemon` : デーモン起動 (systemd/container)
- `ai-selection monitor --once` : 1 回実行・レポート出力
- `ai-selection monitor --check <decision_id>` : 現状スナップショット