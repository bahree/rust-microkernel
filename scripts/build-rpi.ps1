Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path "dist\rpi" | Out-Null

Write-Host "Building Raspberry Pi (aarch64) kernel image..." -ForegroundColor Cyan
pushd "crates/arch_aarch64_rpi"
cargo build --target aarch64-unknown-none --release | Out-Host

Write-Host "Objcopy to kernel8.img..." -ForegroundColor Cyan
cargo objcopy --target aarch64-unknown-none --release -- -O binary "..\..\dist\rpi\kernel8.img" | Out-Host
popd

@"
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
"@ | Set-Content -Encoding ascii "dist\rpi\config.txt"

Write-Host "Wrote dist\rpi\kernel8.img and dist\rpi\config.txt" -ForegroundColor Green


