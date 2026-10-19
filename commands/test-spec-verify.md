---
description: テスト三層 (仕様・実装・エビデンス) 整合性検証
agent: build
subagent: true
---

テスト仕様・コード・エビデンスの整合性を検証します。CIゲート・pre-pushで実行。

実行: `$ARGUMENTS` (オプション: --strict --json --profile {{profile-name}})

プロファイル: `--profile` が無い場合は `specs/` 内の仕様が参照する
プロファイルを使う。どちらも無い場合は `references/test/profiles/` の
既定プロファイル (web-api-l2-moderate) を使う。

検証項目:

- [ ] **仕様存在**: 対象機能の `specs/test-*.md` が存在し承認済み
- [ ] **必須セクション**: Test basis / Risk & tier / Design technique /
      Document map の 4 セクションが揃っている

- [ ] **実装存在**: `tests/` 配下に対応テストコードが存在
- [ ] **1:1 対応**: 仕様シナリオとテスト関数が 1:1 対応 (未実装 0)
- [ ] **全パス**: 全テストパス (フレーキー 0: 直近10回全成功)
- [ ] **カバレッジ**: プロファイルのリスク階層閾値を満たす
      (既定 T1 unit 90% / T2 unit 80% / T3 unit 60%)

- [ ] **ミューテーション**: スコア ≥ 70% (プロファイル指定時のみ)
- [ ] **エビデンス**: 最新実行のエビデンス存在・ハッシュ一致・署名検証
- [ ] **仕様ハッシュ**: エビデンスの spec_hash が現行仕様と一致 (変更後未実行検知)
- [ ] **ID 整合性**: Traceability の ASVS ID が
      `references/test/catalog/owasp-asvs-5.0.0.csv` に実在する

- [ ] **ID 整合性**: Traceability の SP 800-53 統制 ID が
      `references/test/catalog/nist-sp800-53-r5.2.csv` に実在する

- [ ] **ID 整合性**: Traceability の CC 構成要素 ID が
      `references/test/catalog/iso-iec-15408-3-components.csv` に実在する

- [ ] **版付き ID**: 版を含まない ID を使っていない
- [ ] **統制カバレッジ**: 対象スコープの統制について「対象外」と
      「未実施」が区別されて記録されている

判定: 全項目 PASS で `exit 0`、いずれか FAIL で `exit 1` + 詳細レポート

`--json` の出力形式:

```json
{
  "profile": "web-api-l2-moderate",
  "checks": [
    { "id": "coverage", "status": "pass", "detail": "unit 87.3% >= 80%" }
  ],
  "control_coverage": {
    "release": "5.2.0",
    "baseline": "moderate",
    "covered": { "AC-3": ["TC-001"] },
    "not_applicable": ["IA-2"],
    "not_tested": []
  },
  "assurance": { "level": "A2", "cc_components": ["ATE_FUN.1", "ATE_COV.2"] }
}
```

CI 連携:

```yaml
- name: Test Spec Verify
  run: opencode run "/test-spec-verify --strict --json"
```
