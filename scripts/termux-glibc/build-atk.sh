#!/data/data/com.termux/files/usr/glibc/bin/bash
set -euo pipefail

SRC=/data/data/com.termux/files/home/build/atspi/at-spi2-core-2.62.1
PREFIX=/data/data/com.termux/files/usr/glibc

cd "$SRC"
export CC="$PREFIX/bin/gcc"
export CXX="$PREFIX/bin/g++"
export PKG_CONFIG="$PREFIX/bin/pkgconf"
export PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig:$PREFIX/share/pkgconfig"
export PATH="/data/data/com.termux/files/home/.local/bin:$PREFIX/bin:$PATH"

meson setup build --prefix="$PREFIX" \
  -Datk_only=false \
  -Dintrospection=disabled \
  -Ddocs=false \
  -Dx11=disabled \
  -Dgtk2_atk_adaptor=false

ninja -C build
ninja -C build install
