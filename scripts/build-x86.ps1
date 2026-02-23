Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path "dist" | Out-Null

Write-Host "Building x86_64 boot image (requires cargo bootimage)..." -ForegroundColor Cyan
pushd "crates/arch_x86_64"
cargo bootimage --target x86_64-unknown-none --release | Out-Host
popd

$src = Get-ChildItem -Recurse -Filter "bootimage-arch_x86_64.bin" "target\x86_64-unknown-none\release" | Select-Object -First 1
if (-not $src) { throw "bootimage output not found" }

Copy-Item $src.FullName "dist\os-x86_64-bootimage.bin" -Force
Write-Host "Wrote dist\os-x86_64-bootimage.bin" -ForegroundColor Green


