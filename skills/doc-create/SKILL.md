---
name: doc-create
description: >-
  仕様書・設計書・運用ドキュメントをテンプレートから生成し、レビューサイクルに乗せる。ADR/RFC/Runbook/Architecture Decision 等の標準フォーマットを提供。
---

## 目的
ドキュメント作成のばらつきを防ぎ、レビュー・承認・更新のトレーサビリティを確保する。

## テンプレート一覧

| テンプレート | 用途 | 必須セクション |
|---|---|---|
| `adr.md` | Architecture Decision Record | Context, Decision, Consequences, Alternatives, Links |
| `rfc.md` | Request for Comments | Problem, Proposal, Trade-offs, Implementation Plan, Rollback |
| `runbook.md` | 運用手順書 | Overview, Prerequisites, Procedures, Troubleshooting, Contacts |
| `api-spec.md` | API仕様 | Endpoint, Auth, Request/Response, Errors, Examples, Versioning |
| `data-dict.md` | データ辞書 | Entity, Fields, Types, Constraints, Indexes, Relationships |
| `threat-model.md` | 脅威モデル | Assets, Trust Boundaries, Threats, Mitigations, Residual Risk |

## ワークフロー

### 1. 要件ヒアリング（質問ツール使用）
```
- 文書種類: adr / rfc / runbook / api-spec / data-dict / threat-model
- 対象システム/コンポーネント
- ステークホルダー（レビュアー、承認者）
- 期限・優先度
- 既存関連ドキュメント（references 確認）
```

### 2. 下書き生成
- 選択テンプレートに沿って骨子作成
- 既知の事実（コード・設定・インフラ）を `read`/`grep` で収集し埋め込み
- 不明点は `TODO: [要確認]` マーカーで残す

### 3. レビュー依頼（`cross-review` スキル連携）
- 技術的正確性 → `arch-reviewer` / `security-reviewer`
- 運用実現性 → `ops-reviewer`
- ステークホルダー確認 → 手動または `question` ツール

### 4. 承認・配置
- 承認済みマーク追加: `Status: Approved`, `Approved-by`, `Date`
- 配置先: `docs/adr/`, `docs/rfc/`, `runbooks/`, `docs/architecture/` 等
- インデックス更新: `docs/README.md` または `mkdocs.yml` 等

### 5. 更新サイクル
- 四半期ごとまたは重大変更時に `review-flow` で再レビュー
- 古い文書は `Status: Deprecated` + 後継リンク

## ファイル命名規則
```
adr-YYYYMMDD-short-title.md
rfc-NNN-short-title.md
runbook-component-operation.md
api-spec-service-version.md
```

## 権限
- `read`, `glob`, `grep`, `write`（下書き・確定版出力先のみ）
- `shell`: `git add/commit` 許可パターンのみ

## 実行コマンド
`/doc-create <type> [title]` で対話開始。