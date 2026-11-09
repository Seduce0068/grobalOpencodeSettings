---
name: test-spec
description: >-
  テスト仕様書・テストコード・エビデンスの標準化・生成・検証。ISO/IEC/IEEE 29119 準拠の構造に OWASP・SP 800・Common Criteria のカタログを対応させる。
---

## 目的

- テスト仕様の「書きっぱなし・古い・実装と乖離」を防ぐ
- テストコード・仕様書・エビデンスの三位一体管理
- 検証する対象と検証基準を ID で明示し、追跡可能性を確保する
- レビュー・CI で自動検証

## 4 層構造

| 層 | 担う問い | 採用 | 一次参照 |
| --- | --- | --- | --- |
| L1 プロセス骨格 | 何をどの順で、どの文書でやるか | ISO/IEC/IEEE 29119-2/-3/-4 | `references/test/index/iso-iec-ieee-29119.md` |
| L2 検証対象 | 何を確認するか | OWASP (ASVS 5.0.0 軸) | `references/test/index/owasp.md` |
| L3 統制と目的 | 組織として何を担保すべきか | NIST SP 800 系 | `references/test/index/nist-sp-800.md` |
| L4 根拠の信頼性 | 根拠をどれだけ信じるか | ISO/IEC 15408-3 / -5 | `references/test/index/iso-iec-15408.md` |

L2 から L4 は L1 に依存する。導入は L1 を先行させる。

## 三層構造 (成果物)

| 層 | 成果物 | 検証方法 | 更新トリガー |
| --- | --- | --- | --- |
| **仕様** | `specs/test-*.md` (Gherkin/Markdown) | レビューで承認 | 要件変更時 |
| **実装** | `tests/**/*.test.ts` 等 | CI で実行・カバレッジ | 仕様変更・リファクタ時 |
| **エビデンス** | `evidence/test-run-*.json` | ハッシュ比較・署名 | テスト実行毎 |

各層の内部に 4 層の対応が埋め込まれる。
仕様書が「どの基準で合格とするか」を宣言し、エビデンスが
「その基準を実際に満たしたか」を記録する。

## 用語 (ISO/IEC/IEEE 29119-1:2022 に基づく)

| 用語 | 意味 | 本スキルでの該当物 |
| --- | --- | --- |
| test item | テスト対象 | 対象モジュール・API |
| test basis | テストの根拠 | 要件・仕様・リスク |
| test oracle | 合否判定の基準 | 期待値・アサーション |
| test model | テストのモデル | 設計技法が作る test model |
| test coverage item | 網羅すべき項目 | 境界値・同値クラス・分岐 |
| test case | 実行単位 | テスト関数 1 個 |
| test procedure | 実行手順 | テスト関数と実行順序 |
| defect / incident | 不具合 | 失敗したテストの記録 |

2021 年版は test conditions の概念を test model に置換している。
新規文書では `test condition` を使わず `test model` / `test coverage item` を使う。

## ワークフロー

### 1. 仕様作成 (`/test-spec create`)

```text
入力: ユーザーストーリー / 要件 / バグ報告 / 検証プロファイル
処理:
  1. プロファイル選択 (references/test/profiles/*.yaml)
  2. テンプレート選択 (unit/integration/e2e/property/decision-table/
     state-transition/contract/mutation)

  3. 技法の選択と 29119-4 節番号の記録 (TD1-TD3 相当)
  4. Gherkin シナリオ・境界値・異常系を網羅
  5. テスト基準 ID (ASVS / SP 800-53) の割当
  6. `specs/test-<feature>.md` 出力
  7. `/review-flow start` で仕様レビュー依頼
出力: テスト仕様（高保証変更は承認必須、通常変更はセルフレビュー可）
```

### 2. 実装生成 (`/test-spec implement`)

```text
入力: 承認済み仕様 + 実装コード (またはスケルトン)
処理:
  1. 仕様からテストコード雛形生成 (言語別テンプレート)
  2. 実装コードとの整合性検証 (型・シグネチャ)
  3. 統制 ID / 保証尺度の紐付け
  4. `tests/` 配下に配置
  5. `npm test / cargo test / pytest` で初回実行
出力: パスするテストコード
```

### 3. エビデンス収集 (`/test-spec evidence`)

```text
入力: テスト実行結果
処理:
  1. 実行ログ・カバレッジ・ミューテーション結果を JSON 化
  2. 仕様ハッシュと照合 (仕様変更後の再実行検知)
  3. 内部保証尺度 (A0-A5) を算出
  4. `evidence/test-run-<timestamp>-<spec-hash>.json` 保存
  5. 署名付きコミットで改竄検知可能化
出力: 検証可能なエビデンス
```

### 4. 整合性検証 (`/test-spec verify`)

```text
実行: CI / pre-push / 手動
検証項目:
  ☐ 全テストパス
  ☐ カバレッジ ≥ プロファイルのリスク階層閾値
  ☐ 仕様と実装の 1:1 対応 (未実装シナリオ 0)
  ☐ フレーキーテスト 0 (直近 10 回成功)
  ☐ ミューテーションスコア ≥ 70% (プロファイル指定時)
  ☐ エビデンスハッシュ一致
  ☐ テスト基準 ID がカタログに実在する
  ☐ 統制カバレッジの未実施統制が明示されている
判定: 高保証変更（セキュリティ/認証/決済/規制）は全項目PASS必須。通常変更は対象テストPASS + 変更範囲のカバレッジで可。
```

## Part 2 プロセスとの対応

| 29119-2 | 節 | 本スキル |
| --- | --- | --- |
| Test strategy and planning | 7.2 | `specs/test-<feature>.md` 冒頭 + プロファイル |
| Test monitoring and control | 7.3 | `/test-spec verify` |
| Test completion | 7.4 | `evidence/test-run-*.json` + 完了サマリ |
| Test design and implementation | 8.2 | `/test-spec create` + `/test-spec implement` |
| Test environment and data | 8.3 | テンプレートの Setup セクション |
| Test execution | 8.4 | `/test-spec evidence` |
| Test incident reporting | 8.5 | `templates/incident-report.md` |

## 内部保証尺度 (L4)

根拠となる CC 構成要素を併記した内部尺度。**EAL の名称は使わない。**

| 尺度 | 根拠 | 条件 |
| --- | --- | --- |
| A0 | なし | 手動・記録なし |
| A1 | ATE_FUN.1 | 実行結果の記録 |
| A2 | + ATE_COV.2 | カバレッジ基準の充足 |
| A3 | + ALC_CMS.3 | エビデンスのハッシュと署名 |
| A4 | + ATE_IND.2 | 独立主体による実行 |
| A5 | + AVA_VAN.2 | 脆弱性解析の取込 |

A5 は自動実行では到達しない。人間による独立レビューを要する。

## テンプレート一覧 (`templates/`)

| テンプレート | 用途 | 形式 |
| --- | --- | --- |
| `unit-bdd.md` | 単体テスト (Given/When/Then) | Gherkin + TypeScript |
| `integration-api.md` | API 結合テスト | OpenAPI + SuperTest |
| `e2e-playwright.md` | E2E (Playwright MCP 連携) | TypeScript + Page Object |
| `property-based.md` | プロパティベース (fast-check) | TypeScript + Properties |
| `decision-table.md` | 同値類・決定表 (29119-4 5.2.6) | 決定表 + テスト関数 |
| `state-transition.md` | 状態遷移 (29119-4 5.2.8) | 状態図 + 遷移表 |
| `contract.md` | コントラクト (29119-4 5.2.12) | Consumer / Provider |
| `mutation.md` | 構造ベース被変異 (29119-4 5.3) | 変異スコア + 生存変異体 |
| `incident-report.md` | 試験インシデント (29119-2 8.5) | インシデント記録 |

## 仕様書に必須の 4 セクション

全テンプレートに共通で持たせる。

| セクション | 内容 |
| --- | --- |
| Test basis | 根拠となる要求・仕様・リスク |
| Risk & tier | リスク項目と T1-T3 の割当 (完了基準の決定根拠) |
| Design technique | 採用技法と 29119-4 節番号 |
| Document map | どの情報項目 (29119-3) を含むかの宣言 |

## Traceability の列

```text
| Scenario | Req | ASVS | 800-53 | CC | Technique | Tier |
```

- `ASVS` は版付き ID (`v5.0.0-V1.2.5`)
- `800-53` は Release 付き (`SP800-53r5.2/AC-3`)
- `CC` は構成要素 ID (`ATE_FUN.1`)
- 未適用の列は `-` を入れる (空欄禁止)

## エビデンススキーマ (`evidence/schema.json`)

既存フィールド (`spec_hash` / `timestamp` / `environment` / `results` /
`signature`) は変更しない。追加フィールドはすべて任意とする。

| 追加フィールド | 内容 |
| --- | --- |
| `document_type` | test-execution-log / test-result / test-summary-report |
| `profile` | 適用した検証プロファイル名 |
| `test_basis_ref` | 仕様の参照 (パスと spec_hash) |
| `techniques` | 適用した技法の ID 配列 |
| `coverage_measures` | 29119-4 Clause 6 のカバレッジ種別別の値 |
| `risk_items` | リスク項目と tier の配列 |
| `assurance` | 内部保証尺度 (A0-A5) と根拠 CC 構成要素 |
| `conformance` | `tailored` 固定 |

## 適合の主張の境界

- **適合を宣言しない**。「準拠」「認証済み」を出力しない
- 成果物の `conformance` は `tailored` 固定
- OWASP 公式の ASVS 認証・トラストマークは存在しない
- Common Criteria の適合宣言は認証機関の評価活動を要する
- SP 800-53 の満足宣言はベースライン tailoring に対して行う
- 「Top 10 対応完了」を出力しない。出力は要求単位の到達可否まで

## 権限

- `read`, `glob`, `grep`: 全許可
- `write`: `specs/`, `tests/`, `evidence/` のみ
- `shell`: テスト実行コマンドのみ (`npm test`, `cargo test`, `pytest`, `npx playwright test`)
- `subagent`: `/review-flow` 連携のため許可

## 連携

- `/cross-review` でテスト仕様もレビュー対象に含える
- `/review-flow gate-check` に Stage 0 として仕様存在チェック追加
- `/doc-create` でテスト仕様から ADR/Runbook 自動生成
- 静的試験 (レビュー) は本スキルではなく ISO/IEC 20246 の領域。
  `review-flow` と `cross-review` が担当する
