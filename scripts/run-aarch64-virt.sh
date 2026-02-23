#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f dist/virt/os-aarch64-virt.elf ]]; then
  echo "Missing dist/virt/os-aarch64-virt.elf. Run ./scripts/build-aarch64-virt.sh first." >&2
  exit 1
fi

echo "[virt] running QEMU (aarch64, virt)..."
qemu-system-aarch64 \
  -machine virt,gic-version=2 \
  -cpu cortex-a53 \
  -m 256M \
  -nographic \
  -serial mon:stdio \
  -kernel dist/virt/os-aarch64-virt.elf


