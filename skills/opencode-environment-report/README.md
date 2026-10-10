---
type: Reference
title: OpenCode環境レポートの使い方
---

## 要件

- Rustツールチェーンと `rust-script`。
- 追加crate、Python、Node、AI APIは不要。
- Rust 1.82以降を使用する。
- 初回はローカルでコンパイルする。rust-script自身のキャッシュ書き込みは発生する。
- 収集処理は読み取り専用。ネットワークやOpenCodeサービスを起動しない。

## AIなしで実行

調査したいプロジェクトのディレクトリで実行する。

```bash
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_NET_OFFLINE=true
script="$HOME/.config/opencode/skills/opencode-environment-report/scripts/report.rs"
rust-script "$script" --project "$PWD"
```

ファイル保存はシェルのリダイレクトを使う。
既存ファイルの上書きを防ぎ、本人だけが読める権限で生成する例:

```bash
(
  umask 077
  set -C
  rust-script "$script" --project "$PWD" > opencode-environment-report.md
)
```

別のホームディレクトリを対象にする例:

```bash
rust-script "$script" --home /path/to/fixture-home --project /path/to/project
```

`--home` 指定時はホストの `XDG_CONFIG_HOME` を使わず、
指定したホーム配下の `.config/opencode` を対象にする。
プロジェクトの祖先設定も調査するため、分離テストでは対象パスを確認する。

## スキルとして使用

OpenCodeに次のように依頼する。

```text
@opencode-environment-report 環境と設定をレポートし、
モデル・サブエージェント・自動化の安全性をレビューしてください。
設定は変更しないでください。
```

新しいセッションでスキルの検出を確認する。
設定ファイルや既存エージェントの変更は不要。

## 検証

```bash
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_NET_OFFLINE=true
script="$HOME/.config/opencode/skills/opencode-environment-report/scripts/report.rs"
rust-script --test "$script"
rustfmt --check "$script"
```

## 制約

- 静的ファイルの棚卸しであり、OpenCodeの完全な設定リゾルバーではない。
- 実際の使用モデル、接続状態、利用料金、稼働中のジョブは確認しない。
- OS共通のcronやsystemd設定、認証情報、サービスDBは対象外。
  ユーザー領域のOpenCode関連タイマーはファイル名だけを棚卸しする。
  タイマー本文・登録状態・稼働状態・停止手段は確認しない。
- シンボリックリンク、巨大ファイル、走査上限はスキップや制限として扱う。
- Markdownは最大128行・16 KiBのフロントマターだけを解析する。
  複雑なYAML、ブロック形式、アンカーは評価しない。
- 明示追加のスキルソースやリモート設定は完全には走査しない。
- 自動化の定期登録は行わない。必要なら実行周期と保存先を別途決める。
- 名前やモデルIDも機密になり得る。外部共有前に必ず確認する。
- 終了コード0はレポート生成成功を表し、監査合格を意味しない。
  調査の欠落・制限は本文を確認する。不正な引数や対象パスは終了コード2。

## 参照

- [OpenCode V2設定](https://opencode.ai/v2/docs/config)
- [OpenCode V2スキル](https://opencode.ai/v2/docs/skills)
- [OpenCode V2エージェント](https://opencode.ai/v2/docs/agents)
- [rust-script](https://rust-script.org/)
