.section .text.boot
.global _start
_start:
  // Set stack
  ldr x0, =__stack_top
  mov sp, x0

  // Zero BSS: [__bss_start, __bss_end)
  ldr x1, =__bss_start
  ldr x2, =__bss_end
1:
  cmp x1, x2
  b.ge 2f
  str xzr, [x1], #8
  b 1b
2:
  // If we entered at EL2 (typical for QEMU virt), drop to EL1 so the kernel runs
  // in a simpler environment (EL1 + GICv2 + CNTP timer).
  mrs x1, CurrentEL
  lsr x1, x1, #2
  and x1, x1, #3
  cmp x1, #2
  b.ne el1_start

  // Set up an EL1 stack pointer.
  ldr x0, =__stack_top
  msr sp_el1, x0

  // Configure EL2 to return to EL1h.
  mov x0, #(0b0101)         // EL1h
  msr spsr_el2, x0
  adr x0, el1_start
  msr elr_el2, x0

  // Enable FP/ASIMD access at EL1 and ensure EL2 doesn't trap it.
  mrs x0, cptr_el2
  bic x0, x0, #(1 << 10)    // TFP = 0 (don't trap FP/ASIMD)
  msr cptr_el2, x0
  isb
  eret

el1_start:
  // Install a minimal exception vector table for the *current* exception level.
  // QEMU `virt` typically enters at EL2, so we must set VBAR_EL2 (not just VBAR_EL1).
  adr x0, vectors
  msr vbar_el1, x0
  isb

  // Enable FP/ASIMD for Rust/LLVM.
  // Rust/LLVM may use NEON registers for struct copies/memcpy even in early bring-up.
  // If FP is disabled, this traps with EC=0x07 (FP/ASIMD access trap).
  mrs x0, cpacr_el1
  orr x0, x0, #(3 << 20)   // FPEN = 0b11
  msr cpacr_el1, x0
  isb

  bl rust_main
3:
  wfe
  b 3b

// --------------------------------------------------------------------------
// Exception vectors (AArch64):
// The CPU expects 16 entries, each 0x80 bytes apart (total 0x800 bytes).
// If we don't pad correctly, the first real IRQ will jump into garbage.
// --------------------------------------------------------------------------
.align 11
vectors:
  // 0x000: Current EL, SP0, Sync
  b exc_sync
  .space 0x80 - 4
  // 0x080: Current EL, SP0, IRQ
  b exc_irq
  .space 0x80 - 4
  // 0x100: Current EL, SP0, FIQ
  b exc_fiq
  .space 0x80 - 4
  // 0x180: Current EL, SP0, SError
  b exc_serr
  .space 0x80 - 4

  // 0x200: Current EL, SPx, Sync
  b exc_sync
  .space 0x80 - 4
  // 0x280: Current EL, SPx, IRQ
  b exc_irq
  .space 0x80 - 4
  // 0x300: Current EL, SPx, FIQ
  b exc_fiq
  .space 0x80 - 4
  // 0x380: Current EL, SPx, SError
  b exc_serr
  .space 0x80 - 4

  // 0x400: Lower EL (AArch64), Sync
  b exc_sync
  .space 0x80 - 4
  // 0x480: Lower EL (AArch64), IRQ
  b exc_irq
  .space 0x80 - 4
  // 0x500: Lower EL (AArch64), FIQ
  b exc_fiq
  .space 0x80 - 4
  // 0x580: Lower EL (AArch64), SError
  b exc_serr
  .space 0x80 - 4

  // 0x600: Lower EL (AArch32), Sync
  b exc_sync
  .space 0x80 - 4
  // 0x680: Lower EL (AArch32), IRQ
  b exc_irq
  .space 0x80 - 4
  // 0x700: Lower EL (AArch32), FIQ
  b exc_fiq
  .space 0x80 - 4
  // 0x780: Lower EL (AArch32), SError
  b exc_serr
  .space 0x80 - 4

// PL011 UART0 on QEMU virt: 0x0900_0000
.equ UART0_BASE, 0x09000000
.equ UART0_DR,   0x00
.equ UART0_FR,   0x18
.equ UART0_TXFF, (1 << 5)

uart_putc:
  // x0 = byte
1:
  ldr w1, [x2, #UART0_FR]
  tst w1, #UART0_TXFF
  b.ne 1b
  str w0, [x2, #UART0_DR]
  ret

uart_puts:
  // x0 = ptr, x2 = UART base
1:
  ldrb w3, [x0], #1
  cbz w3, 2f
  // map '\n' -> "\r\n"
  cmp w3, #10
  b.ne 3f
  mov w0, #13
  bl uart_putc
3:
  mov w0, w3
  bl uart_putc
  b 1b
2:
  ret

hex_nibble:
  // w0 = 0..15, returns w0 = ASCII
  cmp w0, #9
  ble 1f
  add w0, w0, #55   // 'A' - 10
  ret
1:
  add w0, w0, #48   // '0'
  ret

uart_puthex32:
  // w0 = value, x2 = UART base
  mov w4, w0
  mov w5, #28
1:
  lsr w0, w4, w5
  and w0, w0, #0xF
  bl hex_nibble
  bl uart_putc
  subs w5, w5, #4
  b.ge 1b
  ret

exc_sync:
  // Minimal exception print (no string reads): print ESR/ELR/FAR low32, then hang.
  ldr x2, =UART0_BASE

  // Determine EL and read ESR/ELR/FAR accordingly
  mrs x5, CurrentEL
  lsr x5, x5, #2
  and x5, x5, #3
  cmp x5, #2
  b.ne 7f
  mrs x4, esr_el2
  mrs x6, elr_el2
  mrs x7, far_el2
  b 8f
7:
  mrs x4, esr_el1
  mrs x6, elr_el1
  mrs x7, far_el1
8:
  // Print "E:"
  mov w0, #69   // 'E'
  bl uart_putc
  mov w0, #58   // ':'
  bl uart_putc
  mov w0, w4
  bl uart_puthex32

  // Print " L:"
  mov w0, #32
  bl uart_putc
  mov w0, #76   // 'L'
  bl uart_putc
  mov w0, #58
  bl uart_putc
  mov w0, w6
  bl uart_puthex32

  // Print " F:"
  mov w0, #32
  bl uart_putc
  mov w0, #70   // 'F'
  bl uart_putc
  mov w0, #58
  bl uart_putc
  mov w0, w7
  bl uart_puthex32

  // Newline
  mov w0, #13
  bl uart_putc
  mov w0, #10
  bl uart_putc
  b .

exc_irq:
  // Preemptive scheduling path:
  // We need a scratch reg to read TPIDR_EL1 (context pointer) without losing the
  // interrupted thread's register values. We use a small on-stack scratch area.
  //
  // Context layout (u64):
  // 0..30  : x0..x30
  // 31     : sp
  // 32     : elr_el1
  // 33     : spsr_el1

  // Scratch: save x9,x10 so we can use them as temporaries.
  sub sp, sp, #0x20
  str x9,  [sp, #0x00]
  str x10, [sp, #0x08]

  // x9 = current Context*
  mrs x9, tpidr_el1

  // Save x0..x8
  stp x0,  x1,  [x9, #0x00]
  stp x2,  x3,  [x9, #0x10]
  stp x4,  x5,  [x9, #0x20]
  stp x6,  x7,  [x9, #0x30]
  str x8,          [x9, #0x40]

  // Save original x9/x10 from scratch
  ldr x0, [sp, #0x00]
  str x0, [x9, #0x48]   // x9 slot
  ldr x0, [sp, #0x08]
  str x0, [x9, #0x50]   // x10 slot

  // Save x11..x30
  stp x11, x12, [x9, #0x58]
  stp x13, x14, [x9, #0x68]
  stp x15, x16, [x9, #0x78]
  stp x17, x18, [x9, #0x88]
  stp x19, x20, [x9, #0x98]
  stp x21, x22, [x9, #0xA8]
  stp x23, x24, [x9, #0xB8]
  stp x25, x26, [x9, #0xC8]
  stp x27, x28, [x9, #0xD8]
  stp x29, x30, [x9, #0xE8]

  // Save original SP (before scratch), ELR, SPSR
  add x0, sp, #0x20
  str x0, [x9, #0xF8]
  mrs x0, elr_el1
  str x0, [x9, #0x100]
  mrs x0, spsr_el1
  str x0, [x9, #0x108]

  // Call Rust IRQ handler: x0 = current Context*, returns x0 = next Context*
  mov x0, x9
  bl rust_irq_handler

  // Switch TPIDR_EL1 to next context pointer
  msr tpidr_el1, x0
  mov x19, x0          // x19 = next Context* (keep as base; restore x19 last)

  // Restore SP/ELR/SPSR for next thread
  ldr x1, [x19, #0xF8]
  mov sp, x1
  ldr x1, [x19, #0x100]
  msr elr_el1, x1
  ldr x1, [x19, #0x108]
  msr spsr_el1, x1

  // Restore x0..x8
  ldp x0,  x1,  [x19, #0x00]
  ldp x2,  x3,  [x19, #0x10]
  ldp x4,  x5,  [x19, #0x20]
  ldp x6,  x7,  [x19, #0x30]
  ldp x8,  x9,  [x19, #0x40]

  // Restore x10..x18
  ldp x10, x11, [x19, #0x50]
  ldp x12, x13, [x19, #0x60]
  ldp x14, x15, [x19, #0x70]
  ldr x16,       [x19, #0x80]
  ldp x17, x18,  [x19, #0x88]

  // Restore x20..x30 (skip x19 until the end)
  ldp x20, x21, [x19, #0xA0]
  ldp x22, x23, [x19, #0xB0]
  ldp x24, x25, [x19, #0xC0]
  ldp x26, x27, [x19, #0xD0]
  ldp x28, x29, [x19, #0xE0]
  ldr x30,       [x19, #0xF0]

  // Restore x19 last (base register)
  ldr x19, [x19, #0x98]
  eret
exc_fiq:
  b exc_sync
exc_serr:
  b exc_sync

// (string constants removed; we now print without reading memory)

// Start the first thread by loading its context and eret-ing into it.
// x0 = Context*
.global start_first
start_first:
  msr tpidr_el1, x0
  mov x19, x0          // x19 = Context* (keep as base; restore x19 last)
  // Restore SP/ELR/SPSR
  ldr x1, [x19, #0xF8]
  mov sp, x1
  ldr x1, [x19, #0x100]
  msr elr_el1, x1
  ldr x1, [x19, #0x108]
  msr spsr_el1, x1
  // Restore GPRs (x0..x29, x30)
  ldp x0,  x1,  [x19, #0x00]
  ldp x2,  x3,  [x19, #0x10]
  ldp x4,  x5,  [x19, #0x20]
  ldp x6,  x7,  [x19, #0x30]
  ldp x8,  x9,  [x19, #0x40]
  ldp x10, x11, [x19, #0x50]
  ldp x12, x13, [x19, #0x60]
  ldp x14, x15, [x19, #0x70]
  ldr x16,       [x19, #0x80]
  ldp x17, x18,  [x19, #0x88]
  ldp x20, x21, [x19, #0xA0]
  ldp x22, x23, [x19, #0xB0]
  ldp x24, x25, [x19, #0xC0]
  ldp x26, x27, [x19, #0xD0]
  ldp x28, x29, [x19, #0xE0]
  ldr x30,       [x19, #0xF0]
  ldr x19, [x19, #0x98]
  eret

// --------------------------------------------------------------------------
// enable_mmu(ttbr0: u64)
// - sets MAIR_EL1/TCR_EL1/TTBR0_EL1
// - invalidates TLB
// - enables SCTLR_EL1.M (caches left OFF for safety)
// --------------------------------------------------------------------------
.global enable_mmu
enable_mmu:
  // MAIR_EL1:
  // AttrIdx0 = 0xFF (Normal WBWA)
  // AttrIdx1 = 0x04 (Device-nGnRE)
  mov x1, #0xFF
  mov x2, #0x04
  lsl x2, x2, #8
  orr x1, x1, x2
  msr mair_el1, x1

  // TCR_EL1:
  // - T0SZ = 16 (48-bit VA)
  // - TG0  = 4k
  // - SH0  = inner shareable
  // - IRGN0/ORGN0 = WBWA
  // - EPD1 = 1 (disable TTBR1)
  // - IPS = 40-bit
  mov x1, #16                  // T0SZ
  mov x2, #(0b11)              // SH0
  lsl x2, x2, #12
  orr x1, x1, x2
  mov x2, #(0b01)              // ORGN0
  lsl x2, x2, #10
  orr x1, x1, x2
  mov x2, #(0b01)              // IRGN0
  lsl x2, x2, #8
  orr x1, x1, x2
  mov x2, #(1)                 // EPD1
  lsl x2, x2, #23
  orr x1, x1, x2
  mov x2, #(0b010)             // IPS=40-bit
  lsl x2, x2, #32
  orr x1, x1, x2
  msr tcr_el1, x1

  // TTBR0_EL1 (x0 = L0 base)
  msr ttbr0_el1, x0

  // Synchronize and invalidate
  dsb sy
  isb
  tlbi vmalle1
  dsb ish
  isb

  // Enable MMU (M=1). Keep caches OFF initially (C=0, I=0).
  mrs x1, sctlr_el1
  orr x1, x1, #1              // M
  bic x1, x1, #(1 << 2)        // C
  bic x1, x1, #(1 << 12)       // I
  msr sctlr_el1, x1
  isb
  ret


// (duplicate enable_mmu removed; keep the earlier implementation)
