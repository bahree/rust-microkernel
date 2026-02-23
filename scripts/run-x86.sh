#!/usr/bin/env bash
set -euo pipefail

if [[ ! -f dist/os-x86_64-bios.img ]]; then
  echo "Missing dist/os-x86_64-bios.img. Run ./scripts/build-x86.sh first." >&2
  exit 1
fi

tmp_img="$(mktemp /tmp/rustos-x86-XXXXXX.img)"
trap 'rm -f "$tmp_img"' EXIT
cp -f dist/os-x86_64-bios.img "$tmp_img"

echo "[x86] running QEMU..."
qemu-system-x86_64 \
  -drive format=raw,file="$tmp_img" \
  -serial stdio \
  -display none \
  -m 256M
