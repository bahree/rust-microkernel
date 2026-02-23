ENTRY(_start)

SECTIONS
{
  /*
   * QEMU `virt` default RAM starts at 0x4000_0000.
   * A common convention is to place the kernel at 0x4008_0000.
   */
  . = 0x40080000;

  .text.boot : ALIGN(16) {
    KEEP(*(.text.boot))
  }

  .text : ALIGN(16) {
    *(.text .text.*)
  }

  .rodata : ALIGN(16) {
    *(.rodata .rodata.*)
  }

  .data : ALIGN(16) {
    *(.data .data.*)
  }

  .bss : ALIGN(16) {
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
    __bss_end = .;
  }

  . = ALIGN(16);
  .bss.stack (NOLOAD) : ALIGN(16) {
    __stack_bottom = .;
    . = . + 0x10000; /* 64 KiB stack */
    __stack_top = .;
  }
}


