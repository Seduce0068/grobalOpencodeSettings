---
description: レビューフローを管理する
agent: build
subagent: true
---

レビューフローを操作します。

サブコマンド: $ARGUMENTS (start / status / escalate / gate-check)
- `start [pr-number]`: パイプライン開始 (自動チェック → クロスレビュー → 承認)
- `status [pr-number]`: 進捗・ゲート状況表示
- `escalate [pr-number]`: 手動エスカレーション
- `gate-check`: Stage 1 自動チェックのみ実行 (CI連携用)

設定: `skills/review-flow` のパイプライン定義に準拠。