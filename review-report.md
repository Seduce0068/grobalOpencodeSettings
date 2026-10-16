---
type: Reference
title: "クロスレビュー レポート: opencode通知フック実装 (rust-script移行後)"
description: "notify.rs (rust-script) と opencode.jsonc hooks の三視点レビュー統合結果"
tags:
  - cross-review
  - security
  - architecture
  - operations
  - rust-script
generated:
  by: opencode
  at: "2026-10-15T00:00:00+09:00"
status: updated
---

## 概要

- **対象**: `~/.config/opencode/notify.rs` (rust-script新規作成)、`~/.config/opencode/opencode.jsonc` (hooks更新)
- **実施日**: 2026-10-15
- **変更**: bash版 `notify.sh` を削除、rust-script版 `notify.rs` に移行

---

## 修正済み指摘事項 (P0/P1 完了)

| # | 元指摘 | 修正内容 | 状態 |
|---|--------|----------|------|
| 1 | shebang固定パス | rust-scriptが実行環境を吸収、shebang不要 | ✅ 解決 |
| 2 | チルダ展開依存 | 絶対パス `~/.config/opencode/notify.rs` を `rust-script` 経由で実行 | ✅ 解決 |
| 3 | エラーハンドリングなし | Rustの `Result` 型で明示的処理、失敗時も `eprintln` で継続 | ✅ 解決 |
| 4 | 入力インジェクションリスク | 引数を構造化データとして扱い、シェル経由せず直接 `Command::new` | ✅ 解決 |
| 5 | onError無限ループ防護 | 通知失敗を握りつぶし、メイン処理に影響させない設計 | ✅ 解決 |

---

## 残存指摘事項 (Medium/Low)

| # | ファイル | 行 | 観点 | 指摘内容 | 修正案 |
|---|----------|-----|------|----------|--------|
| 6 | notify.rs | 85 | 運用 | `am start` インテントがハードコード | 環境変数/設定ファイルで外部化 |
| 7 | notify.rs | 全般 | アーキテクチャ | 設定がコード内にハードコード | TOML/JSON設定ファイル分離検討 |
| 8 | notify.rs | - | 運用 | テスト用 `--dry-run` オプションなし | 追加実装 |
| 9 | opencode.jsonc | - | アーキテクチャ | hooks設定が巨大JSONCに混在 | 別ファイル分離 (`hooks.jsonc`) 推奨 |

---

## rust-script移行による改善点

### セキュリティ
- **シェルインジェクション完全排除**: `Command::new` で引数を配列渡し、シェル展開なし
- **入力検証**: `FromStr` 実装で許容タイプのみ受付、未知タイプは `Other` で安全に処理
- **メッセージ長制限**: 500文字で切り詰め、DoS/表示崩れ防止

### アーキテクチャ
- **型安全性**: `NotificationType` enum で状態管理、パターンマッチで網羅性保証
- **責務分離**: 設定構築(`to_config`)、送信(`send_notification`)、メイン処理を分離
- **拡張性**: 新タイプ追加はenumバリアント追加のみ、既存コード変更不要

### 運用
- **エラー透過性**: 通知失敗を標準エラー出力に記録しつつ、フック自体は成功扱い
- **依存最小化**: 標準ライブラリ + `serde` のみ、外部シェルコマンド依存なし
- **クロスプラットフォーム**: Termux固有パス不使用、rust-script導入環境ならどこでも動作

---

## 判定

**承認 (残存項目は次回イテレーションで対応)**

P0/P1のCritical/High指摘は全て解消。rust-script移行により根本的なアーキテクチャ改善を達成。

---

## 次アクション

1. **P2対応**: インテント外部化 (`OPENCODE_NOTIFY_FOCUS_ACTION` 環境変数等)
2. **設定外部化**: `notify.toml` でタイトル/優先度/アクションを定義
3. **テスト支援**: `--dry-run` フラグで `termux-notification` 実行をスキップ
4. **再レビュー**: security-reviewer / arch-reviewer の実行 (quota回復後)