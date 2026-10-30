#!/data/data/com.termux/files/usr/glibc/bin/bash
# NSPR 4.40 を glibc 環境にビルドする。
# 事前に gcc-glibc と make-glibc を導入し、ソースを
# ~/build/nss/nspr-4.40 に展開しておく。
set -euo pipefail

SRC=/data/data/com.termux/files/home/build/nss/nspr-4.40/nspr
PREFIX=/data/data/com.termux/files/usr/glibc

cd "$SRC"
./configure --prefix="$PREFIX" --target=aarch64-linux-gnu
make -j8
make install
