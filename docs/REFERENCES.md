# rust-microkernel References

Comprehensive learning resources organized by topic and difficulty level.

**Difficulty**: Beginner | Intermediate | Advanced

---

## Boot and initialization

### Beginner
- [OSDev Wiki: Boot Sequence](https://wiki.osdev.org/Boot_Sequence) - Overview of x86 boot process
- [OSDev Wiki: Raspberry Pi Bare Bones](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones) - ARM bare-metal intro
- [Writing an OS in Rust: A Minimal Kernel](https://os.phil-opp.com/minimal-rust-kernel/) - Philipp Oppermann's x86_64 boot tutorial

### Intermediate
- [ARM Trusted Firmware Design](https://trustedfirmware-a.readthedocs.io/en/latest/design/firmware-design.html) - ARM boot chain and exception levels
- [Learn the architecture: AArch64 Exception model](https://developer.arm.com/documentation/102412/latest/) - Exception levels and privilege model
- [OSDev Wiki: GDT Tutorial](https://wiki.osdev.org/GDT_Tutorial) - Global Descriptor Table for x86

### Advanced
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/) - Sections D1 (AArch64 System Level) and D13 (Generic Timer)
- [Intel 64 and IA-32 SDM, Volume 3](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) - System programming guide
- [Unified Extensible Firmware Interface (UEFI) Specification](https://uefi.org/specifications) - Modern firmware interface

---

## Interrupts and exceptions

### Beginner
- [OSDev Wiki: Interrupts](https://wiki.osdev.org/Interrupts) - Interrupt concepts and x86 specifics
- [OSDev Wiki: Exceptions](https://wiki.osdev.org/Exceptions) - x86 CPU exceptions reference
- *Operating Systems: Three Easy Pieces* - [Chapter 33: Event-Based Concurrency](https://pages.cs.wisc.edu/~remzi/OSTEP/threads-events.pdf)

### Intermediate
- [OSDev Wiki: APIC](https://wiki.osdev.org/APIC) - Advanced Programmable Interrupt Controller
- [Learn the architecture: AArch64 Exception Handling](https://developer.arm.com/documentation/102412/latest/) - ARM exception vectors and handling
- [ARM Generic Interrupt Controller Architecture Specification](https://developer.arm.com/documentation/ihi0048/latest/) - GICv2/v3 reference
- [OSDev Wiki: PIT](https://wiki.osdev.org/Programmable_Interval_Timer) - x86 timer hardware

### Advanced
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/) - Section D1.10 (Exception handling)
- [ARM Generic Timer](https://developer.arm.com/documentation/102379/latest/) - Timer architecture guide

---

## IPC and microkernel design

### Beginner
- *Operating Systems: Three Easy Pieces* - [Chapter 5: Interlude: Process API](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-api.pdf)
- [OSDev Wiki: Message Passing](https://wiki.osdev.org/Message_Passing) - IPC concepts

### Intermediate
- [Improving IPC by Kernel Design](https://dl.acm.org/doi/10.1145/168619.168633) (Liedtke, 1993) - Fast IPC in L4 microkernel
- [On Micro-Kernel Construction](https://dl.acm.org/doi/10.1145/224056.224075) (Liedtke, 1995) - L4 microkernel principles
- *Operating Systems: Design and Implementation* (Tanenbaum & Woodhull) - Minix microkernel design

### Advanced
- [seL4: Formal Verification of an OS Kernel](https://sel4.systems/About/seL4-whitepaper.pdf) (Klein et al., 2009) - Formally verified microkernel
- [From L3 to seL4: What Have We Learnt in 20 Years of L4 Microkernels?](https://sel4.systems/About/seL4-whitepaper.pdf) - Microkernel evolution
- [Exokernel: An Operating System Architecture for Application-Level Resource Management](https://pdos.csail.mit.edu/6.828/2008/readings/engler95exokernel.pdf) (Engler et al., 1995)

---

## Scheduling and context switching

### Beginner
- *Operating Systems: Three Easy Pieces* - [Chapter 7: Scheduling](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched.pdf)
- *Operating Systems: Three Easy Pieces* - [Chapter 8: Multi-Level Feedback Queue](https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-mlfq.pdf)
- [OSDev Wiki: Context Switching](https://wiki.osdev.org/Context_Switching) - Implementation reference

### Intermediate
- [Lottery Scheduling: Flexible Proportional-Share Resource Management](https://www.usenix.org/legacy/publications/library/proceedings/osdi/full_papers/waldspurger.pdf) (Waldspurger & Weihl, 1994)
- [ARM Procedure Call Standard (AAPCS64)](https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst) - Calling conventions for context save/restore
- [OSDev Wiki: Scheduling Algorithms](https://wiki.osdev.org/Scheduling_Algorithms)

### Advanced
- [The Linux Completely Fair Scheduler (CFS)](https://www.kernel.org/doc/html/latest/scheduler/sched-design-CFS.html)
- *The Art of Multiprocessor Programming* (Herlihy & Shavit) - Concurrency theory and lock-free algorithms

---

## Memory management

### Beginner
- *Operating Systems: Three Easy Pieces* - [Chapter 13: Address Spaces](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-intro.pdf)
- *Operating Systems: Three Easy Pieces* - [Chapter 18: Paging Introduction](https://pages.cs.wisc.edu/~remzi/OSTEP/vm-paging.pdf)
- [OSDev Wiki: Paging](https://wiki.osdev.org/Paging) - x86 paging tutorial
- [Writing an OS in Rust: Heap Allocation](https://os.phil-opp.com/heap-allocation/) - Allocator implementations

### Intermediate
- [Virtual Memory](https://people.eecs.berkeley.edu/~brewer/cs262/VM-annotated.pdf) (Denning, 1970) - Classic VM survey
- [Learn the architecture: AArch64 Memory Management](https://developer.arm.com/documentation/101811/latest/) - ARM MMU tutorial
- [The Working Set Model for Program Behaviour](https://denninginstitute.com/pjd/PUBS/WSModel_1968.pdf) (Denning, 1968)
- [linked_list_allocator crate](https://docs.rs/linked_list_allocator/) - Rust heap allocator implementation

### Advanced
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest/) - Section D5 (Virtual Memory System Architecture)
- [TLB Prefetching](https://research.cs.wisc.edu/multifacet/papers/isca02_tlb_prefetch.pdf) - TLB performance optimization
- *Computer Architecture: A Quantitative Approach* (Hennessy & Patterson) - Chapter 2 (Memory Hierarchy)

---

## Rust bare-metal development

### Beginner
- [Writing an OS in Rust](https://os.phil-opp.com/) - Philipp Oppermann's comprehensive blog series
- [The Embedonomicon](https://docs.rust-embedded.org/embedonomicon/) - Bare-metal Rust fundamentals
- [Rust `no_std` book](https://docs.rust-embedded.org/book/) - Embedded Rust development guide

### Intermediate
- [Rust Embedded Discovery Book](https://docs.rust-embedded.org/discovery/) - Hands-on embedded Rust
- [embedded-hal](https://docs.rs/embedded-hal/) - Hardware abstraction layer traits
- [Rust RFC: `no_std`](https://rust-lang.github.io/rfcs/1184-stabilize-no_std.html) - Understanding the `no_std` ecosystem

### Advanced
- [Rust Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/) - Rules for unsafe Rust
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced unsafe Rust patterns

---

## Hardware documentation

### ARM (AArch64)
- [ARM Architecture Reference Manual (ARMv8-A)](https://developer.arm.com/documentation/ddi0487/latest/) - The definitive reference
- [ARM Cortex-A53 Technical Reference Manual](https://developer.arm.com/documentation/ddi0500/latest/) - RPi Zero 2 W CPU
- [BCM2837 ARM Peripherals](https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2711/rpi_DATA_2711_1p0.pdf) - Raspberry Pi peripheral registers
- [Learn the architecture guides](https://developer.arm.com/documentation#cf[navigationhierarchiescontenttype]=Guide) - ARM tutorial series

### x86_64
- [Intel 64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) - Complete x86 reference
- [AMD64 Architecture Programmer's Manual](https://developer.amd.com/resources/developer-guides-manuals/) - AMD perspective
- [OSDev Wiki](https://wiki.osdev.org/) - Community-maintained x86 OS development reference

### Raspberry Pi
- [Raspberry Pi Documentation](https://www.raspberrypi.com/documentation/) - Official docs
- [RPi BCM2835 GPIOs](https://www.raspberrypi.org/documentation/hardware/raspberrypi/gpio/) - GPIO pinout and UART configuration

---

## Classic OS papers

- [The UNIX Time-Sharing System](https://people.eecs.berkeley.edu/~brewer/cs262/unix.pdf) (Ritchie & Thompson, 1974) - Original Unix design
- [The Structure of the "THE" Multiprogramming System](https://www.cs.utexas.edu/users/EWD/transcriptions/EWD01xx/EWD196.html) (Dijkstra, 1968) - Layered OS design
- [The Nucleus of a Multiprogramming System](https://dl.acm.org/doi/10.1145/362258.362278) (Hansen, 1970) - Early microkernel ideas
- [An Experimental Time-Sharing System](https://dl.acm.org/doi/10.1145/1460833.1460871) (Corbato et al., 1962) - CTSS
- [The Design and Implementation of a Log-Structured File System](https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf) (Rosenblum & Ousterhout, 1992)

---

## Books

### Operating systems
- *Operating Systems: Three Easy Pieces* (Arpaci-Dusseau) - **Free online** at [ostep.org](https://pages.cs.wisc.edu/~remzi/OSTEP/). The best introduction.
- *Operating Systems: Design and Implementation* (Tanenbaum & Woodhull) - Minix design rationale
- *Linux Kernel Development* (Robert Love) - Practical Linux internals
- *The Design and Implementation of the FreeBSD Operating System* (McKusick et al.) - BSD perspective
- *Modern Operating Systems* (Tanenbaum & Bos) - Comprehensive textbook

### Computer architecture
- *Computer Systems: A Programmer's Perspective* (Bryant & O'Hallaron) - Essential CS fundamentals
- *Computer Architecture: A Quantitative Approach* (Hennessy & Patterson) - Performance analysis
- *ARM System Developer's Guide* (Sloss et al.) - ARM architecture deep dive

### Concurrency
- *The Art of Multiprocessor Programming* (Herlihy & Shavit) - Lock-free programming and concurrency theory
- *Is Parallel Programming Hard, And If So, What Can You Do About It?* (McKenney) - [Free online](https://mirrors.edge.kernel.org/pub/linux/kernel/people/paulmck/perfbook/perfbook.html)

### Rust
- *Programming Rust* (Blandy, Orendorff, Tindall) - Comprehensive Rust guide
- *Rust for Rustaceans* (Gjengset) - Advanced Rust patterns

---

## Video courses

- [MIT 6.828: Operating System Engineering](https://pdos.csail.mit.edu/6.828/) - Lecture videos and xv6 labs
- [Stanford CS140: Operating Systems](https://web.stanford.edu/~ouster/cs140-spring2014/) - Pintos-based OS course
- [Low-Level Learning (YouTube)](https://www.youtube.com/@LowLevelLearning) - Accessible OS and systems content
- [Ben Eater (YouTube)](https://www.youtube.com/@BenEater) - Hardware fundamentals with breadboard computers

---

## OS projects to study

### Teaching OSes
- **xv6** (MIT): [pdos.csail.mit.edu/6.828/2023/xv6.html](https://pdos.csail.mit.edu/6.828/2023/xv6.html) - RISC-V, C, ~10k lines, excellent [textbook](https://pdos.csail.mit.edu/6.828/2023/xv6/book-riscv-rev3.pdf)
- **Pintos** (Stanford): Educational OS with guided projects
- **JOS** (MIT): x86-based teaching kernel

### Production microkernels
- **seL4**: [sel4.systems](https://sel4.systems/) - Formally verified, used in aerospace and medical
- **Minix 3**: [minix3.org](https://www.minix3.org/) - Tanenbaum's modern microkernel
- **QNX**: Commercial RTOS (free for non-commercial use)
- **Zephyr**: [zephyrproject.org](https://www.zephyrproject.org/) - RTOS for IoT

### Rust OSes
- **Redox**: [redox-os.org](https://www.redox-os.org/) - Unix-like OS written in Rust
- **Tock**: [tockos.org](https://www.tockos.org/) - Embedded OS for microcontrollers, written in Rust
- **Theseus**: [theseus-os.github.io](https://theseus-os.github.io/Theseus/) - Research OS exploring intralingual design
- **Hubris**: [hubris.oxide.computer](https://hubris.oxide.computer/) - Oxide Computer's embedded OS in Rust

### Research OSes
- **Singularity** (Microsoft Research): [microsoft.com/en-us/research/project/singularity](https://www.microsoft.com/en-us/research/project/singularity/) - Type-safe OS
- **Barrelfish**: [barrelfish.org](http://www.barrelfish.org/) - Multikernel design
- **Unikernels**: [unikernel.org](https://unikernel.org/) - Single-purpose library OSes

---

## Community resources

- **OSDev Forum**: [forum.osdev.org](https://forum.osdev.org/) - Active OS development community
- **OSDev Wiki**: [wiki.osdev.org](https://wiki.osdev.org/) - Invaluable reference for x86 and ARM
- **r/osdev**: [reddit.com/r/osdev](https://www.reddit.com/r/osdev/) - Reddit community
- **Rust Embedded WG**: [github.com/rust-embedded/wg](https://github.com/rust-embedded/wg) - Rust embedded working group
- **Rust OSDev**: [rust-osdev.com](https://rust-osdev.com/) - Monthly updates on Rust OS development
