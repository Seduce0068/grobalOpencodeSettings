---
name: AI Selection
description: クラウドAI・ローカルLLM環境・モデル・推論エンジン・ベクトルDB・RAG・エージェントFWの調査・選定・ベンチマーク・レビューを標準化。技術選定スキルのAI特化版。
---

## 目的
- 「流行りで選ぶ」「スペックだけで選ぶ」「コスト・運用を見落とす」を防ぐ
- クラウド/ローカル/ハイブリッドの統一評価軸
- 継続的ベンチマーク・コスト監視・モデルローテーション管理

## 対象カテゴリ

| カテゴリ | 例 | 評価観点 |
|---|---|---|
| **クラウドAIプロバイダ** | OpenAI, Anthropic, Google, Azure, AWS, Groq, Together | レイテンシ・スループット・価格・SLA・データ居住地・モデル多様性 |
| **ローカル推論エンジン** | Ollama, llama.cpp, LM Studio, vLLM, TGI, MLX, MLC-LLM | 量子化対応・メモリ効率・並列処理・API互換・モデル配布 |
| **モデル (OSS/独自)** | Llama 3.x, Qwen 2.5, Gemma 2, Phi-3, Nemotron, Yi, DeepSeek | パラメータ数・量子化・ライセンス・ベンチマーク・ファインチューニング可否 |
| **ベクトルDB/検索** | Qdrant, Weaviate, Milvus, Pinecone, Chroma, pgvector | スケール・フィルタ・ハイブリッド検索・マルチテナント・運用コスト |
| **RAG/エージェントFW** | LangChain, LlamaIndex, Haystack, LangGraph, CrewAI, AutoGen | 構成可能性・デバッグ・評価・本番運用・ベンダーロックイン |
| **埋め込みモデル** | text-embedding-3, bge, e5, gte, jina, nomic | 次元数・多言語・検索品質・推論速度・ライセンス |
| **ガードレール/評価** | Guardrails AI, NeMo Guardrails, Ragas, DeepEval, TruLens | ポリシー表現・遅延・誤検知・統合容易さ |

## 選定マトリクス: クラウド vs ローカル vs ハイブリッド

| 観点 | クラウドAPI | ローカル (セルフホスト) | ハイブリッド |
|---|---:|---:|---:|
| **初期コスト** | 低 (従量) | 高 (GPU購入/レンタル) | 中 |
| **ランニングコスト** | トークン課金 | 電力・人件・インフラ | 用途別最適化 |
| **レイテンシ (p50)** | 200-800ms | 50-200ms (同一DC) | ルーティング次第 |
| **スループット** | 無制限 (クォータ) | GPU依存・スケールアウト要 | 柔軟 |
| **データプライバシー** | 契約依存 (多くは学習不使用) | 完全制御 | 機密のみローカル |
| **モデル選択肢** | プロバイダ提供のみ | 任意 (HuggingFace等) | 両方活用 |
| **カスタマイズ** | ファインチューニング限定 | 完全 (LoRA/全量/蒸留) | 使い分け |
| **可用性/SLA** | 99.9%+ (プロバイダ) | 自己責任 (冗長化必須) | 高可用設計可能 |
| **ベンダーロックイン** | 高 | 低 (OSS標準) | 中 |

**推奨デフォルト**: ハイブリッド (機密・高頻度・低レイテンシ要求 → ローカル、それ以外 → クラウド)

---

## ライフサイクル

```
[要件・制約定義] → [候補出し] → [ベンチマーク設計] → [実測・評価] → [決定(ADR)] → [導入・監視] → [定期ローテーション]
      ↓              ↓              ↓                ↓             ↓            ↓               ↓
   テンプレ      自動収集        シナリオ定義      自動実行       記録         ダッシュボード    スケジュール
```

---

## ワークフロー

### 1. 要件・制約定義 (`/ai-selection requirements`)
```
入力: ユースケース・データ機密度・予算・レイテンシ要求・スループット・多言語・コンプラ
出力: `ai-selection/requirements/<id>.md`
  - Functional: チャット/埋め込み/分類/抽出/コード生成/推論/ツール使用
  - Non-functional: p99 < 500ms, 100 req/s, データ外部送信不可, 月額 $500 以内
  - Constraints: GPU 保有/クラウドのみ/オンプレ必須/特定リージョン/ライセンス制約
  - Scale: 同時接続・日次トークン量・ピーク係数
  - Compliance: GDPR/データ居住地/業界規制/監査要件
```

### 2. 候補自動収集 (`/ai-selection discover`)
```
入力: 要件 + カテゴリ
自動収集ソース:
  - クラウド: プロバイダ公式・価格ページ・モデルカード
  - ローカル: HuggingFace Trending・llama.cpp対応表・vLLMサポートモデル
  - ベンチマーク: LMSYS Chatbot Arena・Open LLM Leaderboard・HELM・MTEB
  - コスト: GPU クラウド価格 (RunPod, Lambda, vast.ai, Hyperbolic)
出力: `ai-selection/candidates/<id>.md` (構造化比較表)
```

### 3. ベンチマーク設計 (`/ai-selection benchmark-design`)
```
入力: 要件 + 上位候補
設計項目:
  - **精度**: タスク固有データセット (MMLU/GPQA/HumanEval/独自)
  - **速度**: TTFT (Time to First Token), TPOT (Token Per Second), 同時接続
  - **品質**: 指示追従・推論・コード・多言語・長文脈 (Needle in Haystack)
  - **コスト**: $/1M tokens (入力/出力/埋め込み), GPU時間単価
  - **安定性**: エラー率・タイムアウト・メモリリーク・長時間運転
  - **量子化影響**: FP16/INT8/INT4/GPTQ/AWQ/GGUF 比較
出力: `ai-selection/benchmarks/<id>.yaml` (実行可能仕様)
```

### 4. 実測・評価 (`/ai-selection benchmark-run`)
```
実行環境:
  - クラウド: 公式API (並列・リトライ・レート制限考慮)
  - ローカル: 同一ハードウェア (GPU固定・ドライバ固定・OS固定)
  - ハイブリッド: ルーティングロジック込み

指標収集 (自動):
  - latency: p50/p90/p99 TTFT, TPOT, E2E
  - throughput: tokens/s, req/s (同時接続別)
  - quality: Pass@k, Exact Match, F1, BLEU, ルーブリック評価
  - cost: $/1M tokens, $/hour (GPU), 総所有コスト試算
  - stability: error rate, OOM率, 24h連続稼働

出力: `ai-selection/results/<id>.json` + `ai-selection/reports/<id>.md`
```

### 5. 決定記録 (`/ai-selection decide` → ADR)
```
/doc-create adr "ai-selection: <category> <model/provider>"
内容:
  - 採用モデル/プロバイダ/エンジン/バージョン
  - 量子化・パラメータ・コンテキスト長
  - ルーティングルール (ハイブリッド時)
  - フォールバック・フェイルオーバー設計
  - コスト上限・アラート閾値
  - 見直しトリガー (新モデル/価格変更/ベンチマーク更新)
```

### 6. 導入・監視・ローテーション
```
導入:
  - インフラコード化 (Terraform/Helm/Docker Compose)
  - シークレット管理 (Vault/SealedSecrets/EnvKey)
  - 観測性: Langfuse/LangSmith/Weights&Biases/Prometheus+Grafana

監視ダッシュボード:
  - リアルタイム: レイテンシ・スループット・エラー率・コスト/時間
  - 日次: トークン消費・コスト推移・品質トレンド
  - 週次: モデル比較・量子化影響・新版評価

ローテーション (月次/四半期):
  - 新モデルリリース → 自動ベンチマーク → 既存比較 → 切替判定
  - 価格改定 → コスト再試算 → プロバイダ見直し
  - 脆弱性/ライセンス変更 → 緊急評価 → 移行計画
```

---

## ベンチマーク標準シナリオ (`benchmarks/`)

| シナリオ | データセット | 指標 | 合格ライン例 |
|---|---|---|---|
| **一般会話** | MT-Bench, AlpacaEval | Win rate vs GPT-4 | ≥ 85% |
| **コード生成** | HumanEval, MBPP, BigCodeBench | Pass@1, Pass@10 | ≥ 75% |
| **推論** | GPQA, MMLU-Pro, ARC | Accuracy | ≥ 70% |
| **長文脈** | Needle in Haystack, RULER | Recall @ 32k/128k | 100% @ 32k |
| **多言語** | XCOPA, MGSM, Flores | ChrF++ / Accuracy | ≥ 80% (ja) |
| **埋め込み検索** | MTEB (Retrieval) | nDCG@10 | ≥ 0.65 |
| **RAG 端到端** | HotpotQA, 2WikiMultiHop | Answer F1 | ≥ 0.75 |
| **構造化出力** | JSON Schema 準拠率 | Valid JSON % | 100% |
| **ツール使用** | BFCL, API-Bank | Tool Call Accuracy | ≥ 90% |
| **安全性** | ToxiGen, RealToxicityPrompts | Refusal Rate / False Positive | ≥ 95% / ≤ 5% |

---

## コストモデル (試算テンプレート)

### クラウド API
```
月額コスト = (入力トークン数 × 入力単価 + 出力トークン数 × 出力単価) / 1M
          + 埋め込みトークン数 × 埋め込み単価 / 1M
          + リクエスト数 × リクエスト単価 (該当時)
```

### ローカル GPU (購入)
```
月額相当 = (GPU購入価格 / 減価償却月数) + 電力代 + 保守人件費 + ラック/ネットワーク
電力代 = GPU_TDP_W × 24h × 30日 × 電力単価 / 1000
例: H100 80GB (700W, $30k) → $30k/36 + 700×720×$0.15/1000 ≈ $833 + $76 = $909/月
```

### ローカル GPU (レンタル - RunPod/lambda/vast.ai)
```
月額 = GPU時間単価 × 稼働時間
例: H100 80GB @ $2.5/hr × 720h = $1,800/月 (フル稼働)
スポット/中断可: $0.8/hr → $576/月
```

### ハイブリッド最適化
```
総コスト = Σ (ルーティング先別コスト)
制約: p99レイテンシ < 閾値, 機密データはローカル, 品質スコア ≥ 基準
→ 線形計画法 / ルールベースで最適ルーティング決定
```

---

## 量子化判定ガイド

| 量子化 | VRAM削減 | 品質低下 | 推奨用途 | 非推奨 |
|---|---:|---:|---|---|
| **FP16/BF16** | 基準 (1x) | なし | 品質最優先・少数展開 | コスト敏感 |
| **INT8 (GPTQ/AWQ)** | ~50% | < 1% | バランス・汎用 | 高精度推論必須 |
| **INT4 (GPTQ/AWQ/GGUF)** | ~75% | 1-3% | コスト優先・エッジ | 推論・コード・多言語 |
| **INT3/INT2** | ~85%+ | 5-15% | 極限圧縮・実験 | 本番 |

**判定フロー**: FP16 でベンチマーク → 合格なら INT8 → 合格なら INT4 → 失敗したら一つ上

---

## モデルライフサイクル管理

| フェーズ | アクション | 頻度 | 自動化 |
|---|---|---|---|
| **発見** | HuggingFace Trending / リーダーボード監視 | 日次 | ✅ RSS/API |
| **スクリーニング** | ライセンス・量子化対応・アーキテクチャ確認 | 発見時 | ✅ ルールベース |
| **ベンチマーク** | 標準シナリオ実行 | 週次/新モデル時 | ✅ CI/CD |
| **採用/却下** | ADR 記録 | ベンチマーク後 | 手動承認 |
| **本番監視** | 品質・コスト・レイテンシ・ドリフト検知 | リアルタイム | ✅ アラート |
| **ローテーション** | 新版比較・切替実行 | 月次/四半期 | 半自動 (承認要) |
| **アーカイブ** | 旧モデル・成果物保存 | 廃止時 | ✅ |

---

## 権限
- `read`, `glob`, `grep`: 全許可
- `write`: `ai-selection/`, `docs/adr/`, `docs/ai/` のみ
- `shell`: ベンチマーク・価格取得・GPU監視スクリプト
- `webfetch`, `websearch`: 公式・リーダーボード・レビュー取得
- `subagent`: `/doc-create`, `/review-flow`, `/tech-selection` 連携