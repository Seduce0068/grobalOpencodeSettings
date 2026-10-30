#!/data/data/com.termux/files/usr/bin/bash
# OpenCode の shell ツール用ラッパー。
# Termux の termux-exec を LD_PRELOAD し、#!/usr/bin/env 形式の
# shebang を持つ Node CLI を解決可能にする。
_lib="/data/data/com.termux/files/usr/lib/libtermux-exec.so"
if [ -f "$_lib" ]; then
  case ":${LD_PRELOAD:-}:" in
    *":$_lib:"*) ;;
    *) export LD_PRELOAD="$_lib${LD_PRELOAD:+:$LD_PRELOAD}" ;;
  esac
fi
exec /data/data/com.termux/files/usr/bin/bash "$@"
