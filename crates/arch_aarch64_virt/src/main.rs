#![no_std]
#![no_main]

use core::panic::PanicInfo;
use hal::log::Logger;

core::arch::global_asm!(include_str!("boot.S"));

// QEMU `virt` PL011 UART base.
const UART0_BASE: usize = 0x0900_0000;

struct UartLogger;

impl UartLogger {
    #[inline(always)]
    fn mmio_write(offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((UART0_BASE + offset) as *mut u32, val) }
    }

    #[inline(always)]
    fn mmio_read(offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((UART0_BASE + offset) as *const u32) }
    }

    fn putc(c: u8) {
        // FR (0x18) bit5 = TXFF (transmit FIFO full)
        while (Self::mmio_read(0x18) & (1 << 5)) != 0 {}
        Self::mmio_write(0x00, c as u32);
    }

    pub(crate) fn puts(s: &str) {
        for &b in s.as_bytes() {
            if b == b'\n' {
                Self::putc(b'\r');
            }
            Self::putc(b);
        }
    }
}

impl hal::log::Logger for UartLogger {
    fn log(&self, s: &str) {
        UartLogger::puts(s);
    }
}

mod timer;
mod preempt;
mod mem;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let logger = UartLogger;
    logger.log("rustOS: aarch64 QEMU virt boot OK\n");

    #[cfg(feature = "demo-ipc")]
    {
        logger.log("rustOS: IPC + cooperative scheduling demo\n");
        kernel::kmain(&logger)
    }

    #[cfg(feature = "demo-timer")]
    {
        logger.log("rustOS: timer interrupts demo\n");
        timer::init();
        logger.log("rustOS: timer started, entering idle loop\n");
        loop {
            hal::arch::halt();
        }
    }

    #[cfg(feature = "demo-preempt")]
    {
        logger.log("rustOS: preemptive multitasking demo\n");
        preempt::init();
        extern "C" {
            fn start_first(ctx: *const preempt::Context) -> !;
        }
        unsafe { start_first(preempt::first_context()) }
    }

    #[cfg(feature = "demo-memory")]
    {
        logger.log("rustOS: memory management demo (frames + page tables)\n");
        mem::demo();
        loop {
            hal::arch::halt();
        }
    }

    #[cfg(not(any(feature = "demo-ipc", feature = "demo-timer", feature = "demo-preempt", feature = "demo-memory")))]
    {
        logger.log("rustOS: no demo selected, halting\n");
        loop {
            hal::arch::halt();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    UartLogger::puts("rustOS: PANIC\n");
    loop {
        hal::arch::halt();
    }
}


