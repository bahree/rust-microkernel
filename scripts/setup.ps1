Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "Installing Rust components/tools for bare metal..." -ForegroundColor Cyan
rustup component add llvm-tools-preview | Out-Host

Write-Host "Installing cargo tools (bootimage, cargo-binutils)..." -ForegroundColor Cyan
cargo install bootimage --locked | Out-Host
cargo install cargo-binutils --locked | Out-Host

Write-Host "Adding Rust targets..." -ForegroundColor Cyan
rustup target add x86_64-unknown-none | Out-Host
rustup target add aarch64-unknown-none | Out-Host

Write-Host "Done." -ForegroundColor Green


