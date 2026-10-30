#!/data/data/com.termux/files/usr/glibc/bin/bash
# NSS 3.132 を glibc 環境にビルドし、共有ライブラリを $PREFIX/glibc に置く。
# NSPR は build-nspr.sh で先に導入しておく。
#
# USE_64=1 は必須。指定しないと 32bit 前提の PR_STATIC_ASSERT が失敗する。
# WARNING_CFLAGS で -Werror を外すのは、GCC 14 の array-parameter 警告対策。
set -euo pipefail

SRC=/data/data/com.termux/files/home/build/nss/nss-3.132/nss
PREFIX=/data/data/com.termux/files/usr/glibc

cd "$SRC"
make all USE_64=1 WARNING_CFLAGS="-Wno-error" \
  NSPR_INCLUDE_DIR="$PREFIX/include/nspr" \
  NSPR_LIB_DIR="$PREFIX/lib"

DIST=$(ls -d "$SRC/../dist"/Linux*_aarch64_cc_glibc_PTH_64_DBG.OBJ | head -1)
# dist 配下の .so はビルドツリーへのシンボリックリンクなので -L で実体をコピーする。
cp -aL "$DIST/lib/"*.so "$PREFIX/lib/"
