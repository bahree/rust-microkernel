#!/usr/bin/env bash
set -euo pipefail

mkdir -p dist/rpi

echo "[rpi] building aarch64 kernel..."
(cd crates/arch_aarch64_rpi && cargo build --target aarch64-unknown-none --release)

echo "[rpi] objcopy -> dist/rpi/kernel8.img"
(cd crates/arch_aarch64_rpi && cargo objcopy --target aarch64-unknown-none --release -- -O binary ../../dist/rpi/kernel8.img)

cat > dist/rpi/config.txt <<'EOF'
# rustOS (RPi) boot files
#
# Copy the Raspberry Pi firmware boot files to a FAT32 boot partition,
# then place this kernel at the root as kernel8.img.
#
# Raspberry Pi Zero 2 W notes:
# - We use the mini UART (UART1) for early logging
# - Disable Bluetooth so it doesn't steal the UART
# - Pin core_freq so mini UART baud is stable
#
# config.txt:
arm_64bit=1
kernel=kernel8.img
enable_uart=1
dtoverlay=disable-bt
core_freq=250
EOF

echo "[rpi] wrote dist/rpi/kernel8.img and dist/rpi/config.txt"


