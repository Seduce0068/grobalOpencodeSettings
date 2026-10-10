---
name: Cross Review
description: 複数視点での相互コードレビュー手順。セキュリティ・アーキテクチャ・運用の3観点を別エージェントで並列実行し、統合レポートを出力する。
---

## 目的
単一レビューの盲点を、役割分担した複数エージェントで補完する。

## ワークフロー

### 1. 対象特定
- 変更ファイル一覧を取得: `git diff --name-only HEAD~1` または指定コミット範囲
- 影響範囲を `references/architecture.md` 等で確認

### 2. 三視点並列レビュー（サブエージェント起動）

#### A. セキュリティ視点（`security-reviewer`）
```
Launch subagent security-reviewer with:
- Target: 変更ファイル
- Focus: 入力検証、認証認可、秘密情報、依存関係脆弱性、SSRF/XSS/インジェクション
- Output: 発見事項を {file, line, severity, description, remediation} 形式で
```

#### B. アーキテクチャ視点（`arch-reviewer`）
```
Launch subagent arch-reviewer with:
- Target: 変更ファイル
- Focus: 責務分離、循環依存、インターフェース設計、スケーラビリティ、技術的負債
- Output: 同上形式
```

#### C. 運用・保守視点（`ops-reviewer`）
```
Launch subagent ops-reviewer with:
- Target: 変更ファイル
- Focus: ログ・メトリクス・トレーシング、エラー処理、設定外部化、マイグレーション安全性、ロールバック容易性
- Output: 同上形式
```

### 3. 統合・判定
- 3エージェントの出力をマージ
- 同一箇所の重複指摘を統合
- Critical/High は **マージブロック**、Medium 以下は **条件付き承認**
- 判定結果を `review-report.md` に出力

### 4. フィードバックループ
- 指摘事項を元の変更作者に伝達
- 修正後の再レビューは差分のみ対象

## 権限設定
- 全サブエージェント: `read`, `glob`, `grep` のみ許可、`edit`, `shell` は拒否
- 親エージェント: `subagent` 許可、統合レポートの `write` 許可

## 実行コマンド
`/cross-review [commit-range]` で実行。省略時は直近コミット。