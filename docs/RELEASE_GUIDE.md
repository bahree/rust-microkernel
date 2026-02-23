# rust-microkernel Release Guide

This guide explains how to create pre-built binaries for users who want to try rust-microkernel without building from scratch (similar to downloading a pre-trained model).

## Building Release Binaries

### x86_64 QEMU Image

```bash
# Build the bootable disk image
./scripts/build-x86.sh

# The binary is at: dist/x86/os-x86.img
# This is a raw disk image that can run in QEMU

# Test it:
./scripts/run-x86.sh

# Package for release:
cd dist/x86
tar -czf rustos-x86_64-v1.0.tar.gz os-x86.img
```

**Size**: ~2-5 MB
**Format**: Raw disk image
**Works on**: Any platform with QEMU x86_64

### AArch64 QEMU virt ELF

```bash
# Build with default demo (memory management)
./scripts/build-aarch64-virt.sh

# Or build specific demo:
./scripts/build-aarch64-virt.sh demo-ipc

# The binary is at: dist/virt/os-aarch64-virt.elf

# Test it:
./scripts/run-aarch64-virt.sh

# Package for release:
cd dist/virt
tar -czf rustos-aarch64-virt-v1.0.tar.gz os-aarch64-virt.elf
```

**Size**: ~500 KB - 1 MB
**Format**: ELF binary
**Works on**: Any platform with QEMU aarch64

### Raspberry Pi Image

```bash
# Build the kernel
./scripts/build-rpi.sh

# The binary is at: dist/rpi/kernel8.img

# Package with firmware for complete SD card image:
cd dist/rpi
wget https://github.com/raspberrypi/firmware/raw/master/boot/bootcode.bin
wget https://github.com/raspberrypi/firmware/raw/master/boot/start.elf
wget https://github.com/raspberrypi/firmware/raw/master/boot/fixup.dat

# Create config.txt
cat > config.txt << EOF
arm_64bit=1
uart_2ndstage=1
enable_uart=1
kernel=kernel8.img
EOF

# Package complete SD card contents:
tar -czf rustos-rpi-v1.0.tar.gz kernel8.img bootcode.bin start.elf fixup.dat config.txt
```

**Size**: ~5-10 MB (with firmware)
**Format**: Raw binary + Pi firmware
**Works on**: Raspberry Pi Zero 2 W (physical hardware)

---

## GitHub Release Process

### 1. Create Release on GitHub

```bash
# Tag the release
git tag -a v1.0 -m "rust-microkernel v1.0 - Core features complete"
git push origin v1.0

# Create release on GitHub UI or via gh CLI:
gh release create v1.0 \
    dist/x86/rustos-x86_64-v1.0.tar.gz \
    dist/virt/rustos-aarch64-virt-v1.0.tar.gz \
    dist/rpi/rustos-rpi-v1.0.tar.gz \
    --title "rust-microkernel v1.0" \
    --notes "Core microkernel features: boot, IPC, scheduling, interrupts, MMU"
```

### 2. Release Notes Template

```markdown
## rust-microkernel v1.0 - Core Features Complete

### What's Included

Three pre-built binaries for immediate testing:

- **x86_64 QEMU**: Full IPC + scheduler demo
- **AArch64 QEMU virt**: Memory management demo (MMU enabled)
- **Raspberry Pi Zero 2 W**: Real hardware boot

### Quick Start

#### x86_64 (fastest to try)
```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-x86_64-v1.0.tar.gz
tar -xzf rustos-x86_64-v1.0.tar.gz
qemu-system-x86_64 -drive format=raw,file=os-x86.img -serial stdio
```

Expected: Boot messages showing ping/pong IPC demo

#### AArch64 virt
```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-aarch64-virt-v1.0.tar.gz
tar -xzf rustos-aarch64-virt-v1.0.tar.gz
qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 256M -nographic -serial mon:stdio -kernel os-aarch64-virt.elf
```

Expected: MMU enablement demo with virtual address translation

#### Raspberry Pi Zero 2 W
```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-rpi-v1.0.tar.gz
tar -xzf rustos-rpi-v1.0.tar.gz
# Copy all files to FAT32-formatted SD card root
# Insert SD card into Pi, connect USB-to-serial adapter (GPIO 14/15)
# Power on and watch UART output at 115200 baud
```

### Features Demonstrated

- ✅ Bare-metal boot on 3 platforms
- ✅ Message-passing IPC
- ✅ Cooperative scheduler
- ✅ Timer interrupts (ARM only)
- ✅ Preemptive multitasking (ARM only)
- ✅ Virtual memory with MMU (ARM only)

### Blog Series

Full tutorial: https://yourblog.com/rustos-series/

### Building From Source

See [Part 1 of the blog series](link) for complete build instructions.
```

---

## Docker Image Publishing

### Build and Push Docker Image

```bash
# Build the Docker image
docker build -t rust-microkernel:latest .

# Tag for GitHub Container Registry
docker tag rust-microkernel:latest amitbahree/rust-microkernel:latest
docker tag rust-microkernel:latest amitbahree/rust-microkernel:v1.0

# Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u bahree --password-stdin

# Push
docker push amitbahree/rust-microkernel:latest
docker push amitbahree/rust-microkernel:v1.0
```

### Making the Image Public

In GitHub:
1. Go to Packages → rust-microkernel
2. Package settings → Change visibility → Public

### User Instructions (for blog/README)

```bash
# Pull the development environment
docker pull amitbahree/rust-microkernel:latest

# Run container
docker run -it amitbahree/rust-microkernel:latest

# Inside container, everything is pre-installed:
cd /workspace
./scripts/build-x86.sh && ./scripts/run-x86.sh
```

---

## GitHub Codespaces Setup

### 1. Add devcontainer.json

Create `.devcontainer/devcontainer.json`:

```json
{
  "name": "rust-microkernel Development",
  "image": "amitbahree/rust-microkernel:latest",
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer",
        "vadimcn.vscode-lldb"
      ]
    }
  },
  "postCreateCommand": "rustup default nightly",
  "forwardPorts": []
}
```

### 2. Enable Codespaces

In GitHub repo settings:
1. Settings → Codespaces
2. Enable Codespaces for your repository

### 3. Add Badge to README

```markdown
[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/bahree/rust-microkernel)
```

Users can click this to launch a browser-based development environment with everything pre-configured.

---

## Continuous Integration (Optional)

Create `.github/workflows/release.yml` to automate builds:

```yaml
name: Build Release Binaries

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust nightly
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          override: true

      - name: Install targets
        run: |
          rustup target add x86_64-unknown-none
          rustup target add aarch64-unknown-none

      - name: Install QEMU
        run: sudo apt-get install -y qemu-system-x86 qemu-system-aarch64

      - name: Build x86_64
        run: ./scripts/build-x86.sh

      - name: Build AArch64 virt
        run: ./scripts/build-aarch64-virt.sh

      - name: Build Raspberry Pi
        run: ./scripts/build-rpi.sh

      - name: Package binaries
        run: |
          cd dist/x86 && tar -czf ../../rustos-x86_64-${{ github.ref_name }}.tar.gz os-x86.img
          cd dist/virt && tar -czf ../../rustos-aarch64-virt-${{ github.ref_name }}.tar.gz os-aarch64-virt.elf
          cd dist/rpi && tar -czf ../../rustos-rpi-${{ github.ref_name }}.tar.gz kernel8.img

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            rustos-x86_64-${{ github.ref_name }}.tar.gz
            rustos-aarch64-virt-${{ github.ref_name }}.tar.gz
            rustos-rpi-${{ github.ref_name }}.tar.gz
```

This automatically builds and publishes binaries when you push a version tag.

---

## Checklist Before Release

- [ ] All platforms build successfully
- [ ] Tested each binary in QEMU/hardware
- [ ] Updated version numbers in Cargo.toml
- [ ] Created git tag
- [ ] Built and packaged all binaries
- [ ] Created GitHub release with notes
- [ ] Updated blog post links (if applicable)
- [ ] Docker image pushed to GHCR
- [ ] Codespaces devcontainer tested
- [ ] Announced release (Twitter, Reddit, etc.)

---

## Download Sizes (Approximate)

| Binary | Compressed | Uncompressed | Notes |
|--------|-----------|--------------|-------|
| x86_64 | ~500 KB | ~2 MB | Bootable disk image |
| AArch64 virt | ~200 KB | ~800 KB | ELF binary only |
| Raspberry Pi | ~2 MB | ~5 MB | Includes firmware files |
| Docker image | ~400 MB | ~1.2 GB | Full dev environment |

---

## User Experience Goal

Similar to your LLM series:
- **Quick start**: Download and run in < 2 minutes
- **Pre-built option**: No compilation required to try it
- **Full source**: Available for those who want to learn/modify
- **Multiple paths**: Pre-built → Docker → Build from scratch

This lowers the barrier to entry and lets people "kick the tires" before committing to the full build process.
