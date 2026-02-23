# rust-microkernel v1.0 - Core Operating System Features

## What's New

This is the first stable release of rust-microkernel, featuring a complete microkernel implementation with:

- ✅ Bare-metal boot on 3 platforms (x86_64, ARM Raspberry Pi, ARM QEMU)
- ✅ Message-passing IPC with router and mailboxes
- ✅ Cooperative task scheduling
- ✅ Timer interrupts (ARM only)
- ✅ Preemptive multitasking (ARM only)
- ✅ Virtual memory management with MMU (ARM only)

## Pre-Built Binaries

Try rust-microkernel without building from source:

### Quick Start (x86_64)

```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-x86_64-v1.0.tar.gz
tar -xzf rustos-x86_64-v1.0.tar.gz
qemu-system-x86_64 -drive format=raw,file=os-x86_64-bios.img -serial stdio -display none
```

**Expected output**: IPC ping/pong demo showing message-passing between tasks

### Docker (All Tools Included)

```bash
docker pull amitbahree/rust-microkernel:latest
docker run -it amitbahree/rust-microkernel:latest
# Inside container:
cd /workspace
./scripts/build-x86.sh && ./scripts/run-x86.sh
```

### AArch64 QEMU virt

```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-aarch64-virt-v1.0.tar.gz
tar -xzf rustos-aarch64-virt-v1.0.tar.gz
qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 256M -nographic \
  -serial mon:stdio -kernel virt/os-aarch64-virt.elf
```

**Expected output**: Memory management demo with MMU enablement and VA→PA translation

### Raspberry Pi Zero 2 W

```bash
wget https://github.com/bahree/rust-microkernel/releases/download/v1.0/rustos-rpi-v1.0.tar.gz
tar -xzf rustos-rpi-v1.0.tar.gz
# Copy all files to FAT32-formatted SD card root
# Insert SD card into Pi, connect USB-to-serial adapter (GPIO 14/15)
# Power on and watch UART output at 115200 baud
```

## Learning Resources

**5-Part Blog Series** (Part 0 through Part 4): Complete tutorials focusing on AArch64 QEMU virt

- Part 0: Why build an OS from scratch? (overview and motivation)
- Part 1: Foundations (boot and platform abstraction)
- Part 2: Communication (IPC and cooperative scheduling)
- Part 3: Concurrency (interrupts, timers and preemption)
- Part 4: Memory management and beyond

The blog series focuses on the AArch64 QEMU virt platform so anyone with a laptop can follow along. x86_64 and Raspberry Pi platforms are also supported in the codebase but not covered in the blog series. All blog posts are available at [blog.desigeek.com](https://blog.desigeek.com/).

## Building From Source

See [Part 1 of the blog series](https://blog.desigeek.com/post/2026/02/building-microkernel-part1-foundations-boot/) for detailed build instructions.

**Prerequisites**:
- Rust nightly: `rustup default nightly`
- QEMU: Platform-specific installation
- (Optional) Raspberry Pi Zero 2 W for real hardware

**Quick build**:
```bash
# x86_64
./scripts/build-x86.sh && ./scripts/run-x86.sh

# AArch64 virt
./scripts/build-aarch64-virt.sh && ./scripts/run-aarch64-virt.sh

# Raspberry Pi
./scripts/build-rpi.sh
# Flash dist/kernel8.img to SD card
```

## Known Limitations

- Single-core only (no SMP)
- No heap allocator (stack-only)
- No user/kernel separation yet
- No filesystem or block devices
- ARM-only for advanced features (preemption, MMU)

See Part 4 of the blog series for extension ideas.

## Changelog

### Added
- Initial release with core microkernel features
- Cross-platform boot demonstration (x86_64, Raspberry Pi, AArch64 QEMU virt)
- IPC with message-passing router and mailboxes
- Cooperative task scheduling
- Timer interrupts on ARM platforms (Generic Timer + GICv2)
- Preemptive multitasking with context switching (ARM only)
- Memory management: frame allocator, page tables, MMU (ARM only)
- Comprehensive 5-part tutorial blog series (AArch64 virt focused)
- Docker development environment (1.27 GB)
- Pre-built binaries for all platforms

### Features by Platform

**All Platforms**:
- Bare-metal boot sequences
- Serial/UART logging
- Message-passing IPC
- Cooperative scheduler

**AArch64 virt only**:
- Timer interrupts
- Preemptive multitasking
- Virtual memory (4-level page tables)
- MMU enablement with VA→PA translation

## Binary Sizes

| Platform | Compressed | Uncompressed | Format |
|----------|-----------|--------------|--------|
| x86_64 | 29 KB | ~2 MB | Bootable disk image |
| AArch64 virt | 3.8 KB | ~500 KB | ELF binary |
| Raspberry Pi | 1.7 KB | ~50 KB | Raw kernel image |

## Acknowledgments

- **Philipp Oppermann** - ["Writing an OS in Rust"](https://os.phil-opp.com/) series
- **MIT xv6** - Teaching OS design clarity
- **OSDev community** - Invaluable documentation and support
- **Rust embedded working group** - Tooling and best practices

---

**Repository**: https://github.com/bahree/rust-microkernel
**Docker Hub**: https://hub.docker.com/r/amitbahree/rust-microkernel
**License**: MIT

**Educational Use**: This project is designed for learning OS fundamentals and Rust systems programming. It is not intended for production use.
