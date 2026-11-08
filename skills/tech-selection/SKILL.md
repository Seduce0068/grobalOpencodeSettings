---
name: tech-selection
description: >-
  ソフト・ハード・アプリ・サービス・サブスク・技術・フレームワーク・ライブラリの調査・選定・レビューを標準化。決定記録（ADR）と連携。
---

## 目的
- 「なんとなく選ぶ」「後で後悔する」「選定理由が残らない」を防ぐ
- 調査・評価・決定・見直しのライフサイクル管理
- チーム合意形成の証跡化

## 対象カテゴリ

| カテゴリ | 例 | 評価観点 |
|---|---|---|
| **SaaS/サービス** | GitHub, Vercel, Datadog, Notion, Linear | 料金・API・SLA・ベンダーロックイン・移行コスト |
| **サブスク** | Copilot, Cursor, JetBrains, Adobe | 必要性・代替・利用頻度・ROI |
| **フレームワーク** | Next.js, React, Rust/Tauri, Go/Fiber | エコシステム・学習コスト・性能・長期サポート |
| **ライブラリ** | zod, tanstack/query, axios, serde | バンドルサイズ・メンテ状況・TypeScript対応・依存数 |
| **インフラ/ツール** | Docker, Kubernetes, Terraform, GitHub Actions | 運用コスト・チームスキル・ロックイン |
| **ハードウェア** | PC, Phone, Server, GPU, Keyboard | スペック・価格・拡張性・保証・互換性 |
| **開発ツール** | IDE, Terminal, Profiler, Debugger | 生産性・統合・カスタマイズ性 |

## ライフサイクル

```
[要件定義] → [候補出し] → [評価マトリクス] → [PoC/試用] → [決定(ADR)] → [導入] → [定期見直し]
     ↓            ↓            ↓              ↓            ↓           ↓           ↓
  テンプレート  検索スキル   スコアリング    自動化/手動   記録        ドキュメント  スケジュール
```

---

## ワークフロー

### 1. 要件定義 (`/tech-selection requirements`)
```
入力: 解決したい課題・制約・予算・期限・必須/歓迎要件
出力: `tech-selection/requirements/<id>.md`
  - 機能要件 / 非機能要件
  - 予算上限 (月額/年額/一括)
  - 制約 (オンプレ必須/特定クラウド/ライセンス/コンプラ)
  - タイムライン
  - ステークホルダー
```

### 2. 候補出し (`/tech-selection discover`)
```
入力: 要件 + カテゴリ
処理:
  1. 公式サイト・GitHub・npm・crates.io・Docker Hub 検索
  2. 代替案・競合・フォークも網羅
  3. `tech-selection/candidates/<id>.md` に一覧化
出力: 候補リスト (最低 3 つ、理想 5-7 つ)
```

### 3. 評価マトリクス (`/tech-selection evaluate`)
```
入力: 要件 + 候補リスト
処理:
  1. 重み付けスコアカード作成 (必須=ブロッカー、歓迎=加点)
  2. 各観点を 1-5 点評価
  3. 定性メモ・リスク・懸念も記録
  4. 総合スコア・ランキング算出
出力: `tech-selection/evaluations/<id>.md` (マトリクス表 + 根拠)
```

### 4. PoC/試用 (`/tech-selection poc`)
```
入力: 上位 2-3 候補
処理:
  1. 最小実装・設定・動作確認スクリプト実行
  2. 実測値取得 (速度・メモリ・コスト・DX)
  3. チームメンバーによる試用フィードバック収集
  4. 失敗パターン・ハマり所記録
出力: `tech-selection/poc/<id>.md` (実測・定性・推奨)
```

### 5. 決定記録 (`/tech-selection decide` → ADR)
```
入力: 評価結果 + PoC 結果
処理:
  1. `/doc-create adr "tech-selection: <category> <name>"` で ADR 生成
  2. Status: Accepted / Superseded / Deprecated
  3. 代替案・却下理由・移行計画も記録
出力: `docs/adr/YYYYMMDD-tech-selection-<name>.md`
```

### 6. 導入・ドキュメント化
```
- 設定手順・ハマり所・ベストプラクティスを `docs/tech/<name>.md` に
- チーム共有・オンボーディング資料化
- 依存関係図・アーキテクチャ図更新
```

### 7. 定期見直し (四半期/年次)
```
トリガー: カレンダー / 契約更新 / メジャーバージョン / インシデント
実行: `/tech-selection review`
  - 現状スコア再評価
  - 代替案再調査
  - 継続/移行/廃止決定
  - ADR 更新 (Superseded)
```

---

## スコアリング基準 (共通)

### 必須項目 (いずれか 1 でも 0 なら即除外)
| 項目 | 判定基準 |
|---|---|
| **ライセンス適合** | MIT/Apache-2.0/BSD 等許容、GPL/AGPL/独自は法務確認必須 |
| **セキュリティ** | 既知脆弱性(CVSS≥7)未対応なし、供給チェーン署名あり |
| **メンテナンス** | 直近 6 ヶ月コミット/リリースあり、Issue 対応活発 |
| **互換性** | 対象ランタイム・OS・依存バージョンで動作確認済み |

### 加点項目 (重み付け合計 100)
| 観点 | 重み | 評価基準 (5点満点) |
|---|---:|---|
| **機能適合** | 25% | 要件網羅度・拡張性・カスタマイズ性 |
| **開発体験 (DX)** | 20% | TypeScript対応・ドキュメント・エラー分かりやすさ・デバッグ容易さ |
| **性能・リソース** | 15% | ベンチマーク・バンドルサイズ・メモリ・CPU・起動速度 |
| **運用・保守** | 15% | ログ・メトリクス・アップグレード容易さ・移行パス |
| **コスト** | 10% | 直接費・間接費(学習・運用)・スケール時単価 |
| **エコシステム** | 10% | コミュニティ・プラグイン・採用実績・人材確保容易さ |
| **ベンダー信頼性** | 5% | 企業規模・財務・サポートSLA・ロードマップ公開度 |

---

## テンプレート (`templates/`)

| ファイル | 用途 |
|---|---|
| `requirements.md` | 要件定義書 |
| `candidate-list.md` | 候補一覧・比較表 |
| `scorecard.md` | 重み付け評価シート |
| `poc-plan.md` | PoC 計画・実施記録 |
| `poc-result.md` | PoC 結果・実測値 |
| `decision-log.md` | 決定会議ログ |
| `review-checklist.md` | 定期見直しチェックリスト |

---

## 自動化連携

| ツール | 用途 |
|---|---|
| **GitHub Dependabot/Renovate** | ライブラリ更新自動PR・スコアリング連携 |
| **npm audit / cargo audit / pip-audit** | 脆弱性自動検知 → 再評価トリガー |
| **Bundlephobia / pkg-size** | バンドルサイズ自動取得 |
| **Libraries.io / deps.dev** | メンテ状況・依存グラフ取得 |
| **GitHub API / GitLab API** | スター・フォーク・Issue活発度取得 |
| **価格API / 請求書パース** | SaaSコスト実測・予算アラート |

---

## 権限
- `read`, `glob`, `grep`: 全許可
- `write`: `tech-selection/`, `docs/adr/`, `docs/tech/` のみ
- `shell`: 検索・ベンチマーク・価格取得スクリプト実行
- `webfetch`, `websearch`: 公式サイト・レビュー・比較記事取得
- `subagent`: `/doc-create`, `/review-flow` 連携