---
description: クロスレビューを実行する
agent: build
subagent: true
---

クロスレビューを実行してください。

対象: $ARGUMENTS (省略時は直近コミット `HEAD~1..HEAD`)

手順:
1. 変更ファイルを特定 (`git diff --name-only $ARGUMENTS`)
2. 以下の3サブエージェントを並列起動:
   - security-reviewer
   - arch-reviewer
   - ops-reviewer
3. 各エージェントの出力を収集し、統合レポート `review-report.md` を作成
4. 判定: Critical/High があれば BLOCK、Medium 以下なら CONDITIONAL

出力形式は各エージェントの SKILL.md 準拠。