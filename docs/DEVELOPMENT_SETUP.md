# Development Environment Setup for rust-microkernel

This guide covers setting up a development environment to build and run rust-microkernel on Ubuntu Linux.

## Table of Contents
- [Quick Start (Docker)](#quick-start-docker)
- [Native Ubuntu Setup](#native-ubuntu-setup)
- [Building rust-microkernel](#building-rust-microkernel)
- [Testing on QEMU](#testing-on-qemu)
- [Troubleshooting](#troubleshooting)

---

## Quick Start (Docker)

The fastest way to get started is using our pre-configured Docker image:

```bash
# Pull the pre-built image
docker pull amitbahree/rust-microkernel:latest

# Run with source mounted
docker run -it -v $(pwd):/workspace amitbahree/rust-microkernel:latest

# Inside container - everything is ready
cd /workspace
./scripts/build-x86.sh
./scripts/run-x86.sh
```

**Pros**: No setup needed, consistent environment
**Cons**: Larger download, requires Docker installed

---

## Native Ubuntu Setup

### System Requirements

- **OS**: Ubuntu 20.04+ (tested on 22.04 and 24.04)
- **RAM**: 4GB minimum, 8GB recommended
- **Disk**: 10GB free space
- **CPU**: x86_64 with virtualization support (VT-x/AMD-V)

### Step 1: Install System Dependencies

```bash
# Update package lists
sudo apt update

# Install build essentials
sudo apt install -y \
    build-essential \
    git \
    curl \
    pkg-config \
    libssl-dev

# Install QEMU (for running kernels)
sudo apt install -y \
    qemu-system-x86 \
    qemu-system-arm \
    qemu-utils

# Verify QEMU installation
qemu-system-x86_64 --version    # Should show version 4.2+
qemu-system-aarch64 --version   # Should show version 4.2+
```

**What each does**:
- `build-essential`: gcc, g++, make (needed for some Rust build dependencies)
- `git`: Clone repository
- `curl`: Download Rust installer
- `qemu-system-x86`: Run x86_64 kernels
- `qemu-system-arm`: Run AArch64 (ARM) kernels

### Step 2: Install Rust Toolchain

```bash
# Install Rust via rustup (official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts:
# - Select option 1 (default installation)
# - Accept defaults

# Reload shell environment
source $HOME/.cargo/env

# Verify installation
rustc --version    # Should show 1.70+
cargo --version

# Switch to nightly (required for bare-metal features)
rustup default nightly
rustup component add rust-src --toolchain nightly

# Add bare-metal targets
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none

# Install cargo-binutils (for binary inspection)
cargo install cargo-binutils
```

**Why nightly?**
- `no_std` Rust requires unstable features
- Bare-metal compilation uses nightly-only APIs
- We explicitly test with nightly

**Why rust-src?**
- Allows Rust to compile core library for bare-metal targets
- Required for `#![no_std]` crates

### Step 3: Install Additional Tools (Optional but Recommended)

```bash
# For ARM cross-compilation debugging
sudo apt install -y \
    gcc-aarch64-linux-gnu \
    gdb-multiarch

# For inspecting binaries
sudo apt install -y \
    binutils \
    objdump

# For serial console (if testing on real Pi)
sudo apt install -y \
    screen \
    minicom
```

### Step 4: Clone Repository

```bash
# Clone the repository
git clone https://github.com/bahree/rust-microkernel.git
cd rust-microkernel

# Verify structure
ls -la
# Should see: Cargo.toml, crates/, scripts/, docs/, etc.
```

### Step 5: Verify Setup

```bash
# Test Rust installation
rustc --version
cargo --version

# Verify targets installed
rustup target list --installed | grep -E "(x86_64-unknown-none|aarch64-unknown-none)"

# Test QEMU
qemu-system-x86_64 -version
qemu-system-aarch64 -version
```

If all commands succeed, you're ready to build!

---

## Building and running (AArch64 QEMU virt)

This is the primary platform for the blog series. All demos run in QEMU with no special hardware.

### Build

```bash
# Build for QEMU virt machine (default: memory management demo)
./scripts/build-aarch64-virt.sh

# Output location
ls -lh dist/virt/os-aarch64-virt.elf
# Should see: ELF binary (~10-30KB)
```

### Run

```bash
./scripts/run-aarch64-virt.sh

# Expected output:
# rustOS: aarch64 QEMU virt boot OK
# rustOS: memory management demo (frames + page tables)
# mm: demo start
# mm: kernel_end=0x000000004009A010
# mm: free_start=0x000000004009B000
# ...
# mm: enabling MMU (caches off)...
# mm: test_va_read=0x00000000DEADBEEF
# mm: demo done (MMU is ON)

# To exit: Ctrl+A, then X
```

**QEMU command** (what the script runs):
```bash
qemu-system-aarch64 \
  -machine virt,gic-version=2 \
  -cpu cortex-a53 \
  -m 256M \
  -nographic \
  -serial mon:stdio \
  -kernel dist/virt/os-aarch64-virt.elf
```

### Other demos

The platform supports multiple demos via feature flags:

```bash
# IPC + cooperative scheduling
./scripts/build-aarch64-virt.sh demo-ipc && ./scripts/run-aarch64-virt.sh
# Output: Ping/Pong messages

# Timer interrupts
./scripts/build-aarch64-virt.sh demo-timer && ./scripts/run-aarch64-virt.sh
# Output: System enters idle loop, wakes on timer ticks

# Preemptive multitasking
./scripts/build-aarch64-virt.sh demo-preempt && ./scripts/run-aarch64-virt.sh
# Output: A and B alternating
```

---

## Other platforms

The codebase also supports x86_64 and Raspberry Pi. These share the same `kernel` and `hal` crates but aren't covered in the blog series.

### x86_64 (QEMU)

```bash
# Requires: rustup target add x86_64-unknown-none
./scripts/build-x86.sh
./scripts/run-x86.sh
# Output: IPC ping/pong demo
# To exit: Ctrl+A, then X
```

### Raspberry Pi Zero 2 W

```bash
./scripts/build-rpi.sh
# Output: dist/rpi/kernel8.img
```

To run on hardware:
1. Format SD card as FAT32
2. Download [Raspberry Pi firmware](https://github.com/raspberrypi/firmware/tree/master/boot) (`bootcode.bin`, `start.elf`, `fixup.dat`)
3. Copy firmware + `kernel8.img` + a `config.txt` (with `arm_64bit=1`, `enable_uart=1`, `kernel=kernel8.img`) to SD card
4. Insert SD, connect USB-to-serial adapter (GPIO 14/15), power on
5. Watch UART output at 115200 baud

---

## Troubleshooting

### Build Errors

**"error: no default toolchain configured"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default nightly
```

**"error: toolchain 'nightly-x86_64-unknown-linux-gnu' does not contain component 'rust-src'"**
```bash
rustup component add rust-src --toolchain nightly
```

**"error: xtask build-x86-image failed"**
```bash
cargo build -p xtask
```

**"can't find crate for `core`"**
```bash
rustup target add x86_64-unknown-none
# or
rustup target add aarch64-unknown-none
```

### QEMU Errors

**"qemu-system-x86_64: command not found"**
```bash
sudo apt install qemu-system-x86
```

**"Could not access KVM kernel module"**
- Not an error! KVM acceleration is optional
- Kernel will run without KVM (just slower)
- To enable KVM: Ensure VT-x/AMD-V enabled in BIOS and `kvm` module loaded

**QEMU hangs at boot**
- Press Ctrl+C to exit
- Check build succeeded: `ls -lh dist/os-x86_64-bios.img`
- Try building with verbose output: `cargo build --verbose`

**"No output in QEMU"**
- Ensure `-serial stdio` flag in run script
- Check binary was built: `file dist/os-x86_64-bios.img`

### Raspberry Pi Issues

**No UART output**
- Check wiring: GPIO 14 (TX) → USB-serial RX, GPIO 15 (RX) → USB-serial TX, GND → GND
- Verify baud rate: 115200 in terminal (`screen /dev/ttyUSB0 115200`)
- Check `config.txt` has `enable_uart=1`

**Pi doesn't boot**
- Verify firmware files on SD card: `bootcode.bin`, `start.elf`, `fixup.dat`
- Check `config.txt` has `kernel=kernel8.img` and `arm_64bit=1`
- Try re-downloading firmware from official repo

---

## VM Setup (Alternative to Native)

If you prefer a clean environment, use a VM:

### VirtualBox Setup

```bash
# 1. Download Ubuntu 22.04 ISO
wget https://releases.ubuntu.com/22.04/ubuntu-22.04.3-desktop-amd64.iso

# 2. Create VM in VirtualBox:
# - Name: rust-microkernel-dev
# - Type: Linux, Ubuntu 64-bit
# - RAM: 4GB (8GB better)
# - Disk: 20GB VDI
# - CPU: 2 cores
# - Enable VT-x/AMD-V (Settings → System → Processor → Enable PAE/NX)

# 3. Install Ubuntu (follow prompts)

# 4. Inside VM, follow "Native Ubuntu Setup" above
```

### VMware Setup

Similar to VirtualBox:
1. Create new VM with Ubuntu 22.04
2. Allocate 4GB RAM, 20GB disk
3. Enable virtualization passthrough
4. Follow native setup steps

---

## Docker Setup (Detailed)

### Building Custom Docker Image

If you want to customize the development environment:

```dockerfile
# Dockerfile
FROM ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    qemu-system-x86 \
    qemu-system-arm \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Configure Rust for bare-metal
RUN rustup default nightly && \
    rustup component add rust-src --toolchain nightly && \
    rustup target add x86_64-unknown-none && \
    rustup target add aarch64-unknown-none && \
    cargo install cargo-binutils

WORKDIR /workspace
CMD ["/bin/bash"]
```

Build and use:
```bash
# Build image
docker build -t rust-microkernel:local .

# Run with source mounted
docker run -it -v $(pwd):/workspace rust-microkernel:local

# Build inside container
cd /workspace
./scripts/build-x86.sh
./scripts/run-x86.sh
```

---

## Testing Checklist

Before submitting code or creating releases, verify:

### x86_64
- [ ] `./scripts/build-x86.sh` succeeds
- [ ] `./scripts/run-x86.sh` boots and shows output
- [ ] Ping/Pong messages appear
- [ ] Can exit QEMU cleanly (Ctrl+A, X)

### AArch64 virt
- [ ] `./scripts/build-aarch64-virt.sh` succeeds
- [ ] Memory demo runs and shows "MMU enabled!"
- [ ] Test VA mapping works (0xDEADBEEF readback)
- [ ] IPC demo works (`demo-ipc`)
- [ ] Timer demo works (`demo-timer`)
- [ ] Preemption demo works (`demo-preempt`)

### Raspberry Pi (if testing on hardware)
- [ ] `./scripts/build-rpi.sh` creates `dist/rpi/kernel8.img`
- [ ] SD card has all firmware files
- [ ] UART shows boot messages at 115200 baud
- [ ] IPC ping/pong visible on serial console

---

## Next Steps

Once setup is complete:
1. Read [Part 0: Why build an OS from scratch?](https://blog.desigeek.com/post/2026/02/building-microkernel-part0-why-build-an-os/) for the big picture
2. Follow [Part 1: Foundations](https://blog.desigeek.com/post/2026/02/building-microkernel-part1-foundations-boot/) to understand boot
3. Continue with Parts 2-4, building incrementally
4. Use git tags to jump to specific checkpoints: `git checkout v0.1-boot`
5. Experiment with code changes and rebuild
6. Join the community and share your experience!

---

## Getting Help

**Build issues**: Check troubleshooting section above
**Concept questions**: Read the blog posts or the README
**Bugs**: Open an issue on GitHub
**Discussion**: Use GitHub Discussions

---

*Last updated: 2026-02-21*
