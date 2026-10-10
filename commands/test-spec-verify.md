---
description: テスト三層 (仕様・実装・エビデンス) 整合性検証
agent: build
subagent: true
---

テスト仕様・コード・エビデンスの整合性を検証します。CIゲート・pre-pushで実行。

実行: `$ARGUMENTS` (オプション: --strict --json)

検証項目:
- [ ] **仕様存在**: 対象機能の `specs/test-*.md` が存在し承認済み
- [ ] **実装存在**: `tests/` 配下に対応テストコードが存在
- [ ] **1:1 対応**: 仕様シナリオとテスト関数が 1:1 対応 (未実装 0)
- [ ] **全パス**: 全テストパス (フレーキー 0: 直近10回全成功)
- [ ] **カバレッジ**: unit ≥ 80%, integration ≥ 60%, e2e ≥ 主要フロー 100%
- [ ] **ミューテーション**: スコア ≥ 70% (Stryker/gremlins.js)
- [ ] **エビデンス**: 最新実行のエビデンス存在・ハッシュ一致・署名検証
- [ ] **仕様ハッシュ**: エビデンスの spec_hash が現行仕様と一致 (変更後未実行検知)

判定: 全項目 PASS で `exit 0`、いずれか FAIL で `exit 1` + 詳細レポート

CI 連携:
```yaml
- name: Test Spec Verify
  run: opencode run "/test-spec-verify --strict --json"
```