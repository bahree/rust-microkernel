.section .text.boot
.global _start
_start:
  // Set up a simple stack and jump into Rust.
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
  bl rust_main
3:
  wfe
  b 3b


