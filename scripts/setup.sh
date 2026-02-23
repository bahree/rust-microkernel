#!/usr/bin/env bash
set -euo pipefail

echo "[setup] installing rust components/tools for bare metal..."
rustup component add llvm-tools-preview rust-src

echo "[setup] installing cargo tools (cargo-binutils)..."
cargo install cargo-binutils --locked

echo "[setup] adding Rust targets..."
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none

echo "[setup] done."
