#!/data/data/com.termux/files/usr/glibc/bin/bash
set -euo pipefail

SRC=/data/data/com.termux/files/home/build/xdamage/libXdamage-1.1.6
PREFIX=/data/data/com.termux/files/usr/glibc

cd "$SRC"
export CC="$PREFIX/bin/gcc"
export PKG_CONFIG="$PREFIX/bin/pkgconf"
export PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig:$PREFIX/share/pkgconfig"
export PATH="$PREFIX/bin:$PATH"

./configure --prefix="$PREFIX" --disable-static
make -j8
make install
