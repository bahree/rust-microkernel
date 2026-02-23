#!/usr/bin/env bash
set -euo pipefail

mkdir -p dist/virt

# Usage: ./build-aarch64-virt.sh [feature]
# Examples:
#   ./build-aarch64-virt.sh            # default (demo-memory)
#   ./build-aarch64-virt.sh demo-ipc
#   ./build-aarch64-virt.sh demo-preempt
FEATURE="${1:-}"

if [ -n "$FEATURE" ]; then
    echo "[virt] building aarch64 QEMU virt ELF with feature: $FEATURE"
    (cd crates/arch_aarch64_virt && cargo build --target aarch64-unknown-none --release --no-default-features --features "$FEATURE")
else
    echo "[virt] building aarch64 QEMU virt ELF (default features)..."
    (cd crates/arch_aarch64_virt && cargo build --target aarch64-unknown-none --release)
fi

elf="target/aarch64-unknown-none/release/arch_aarch64_virt"
cp -f "$elf" dist/virt/os-aarch64-virt.elf
echo "[virt] wrote dist/virt/os-aarch64-virt.elf"
