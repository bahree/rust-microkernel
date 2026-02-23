#!/usr/bin/env bash
set -euo pipefail

mkdir -p dist

echo "[x86] building kernel ELF..."
cargo build -p arch_x86_64 --target x86_64-unknown-none --release

echo "[x86] creating BIOS disk image..."
cargo run -p xtask -- build-x86-image

echo "[x86] wrote dist/os-x86_64-bios.img"
