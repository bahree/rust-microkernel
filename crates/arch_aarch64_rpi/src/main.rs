#![no_std]
#![no_main]

use core::panic::PanicInfo;
use hal::log::Logger;

// Pull in our `_start` from `boot.S`.
core::arch::global_asm!(include_str!("boot.S"));

// Raspberry Pi Zero 2 W is "Pi 3 class" silicon with peripheral base 0x3F000000.
// We'll use the Mini UART (AUX/MU) for early logging because it is easy to bring up
// when combined with:
// - enable_uart=1
// - core_freq=250
// - dtoverlay=disable-bt
//
// (so the serial console isn't stolen by Bluetooth and the UART clock is stable)
const PERIPHERAL_BASE: usize = 0x3F00_0000;
const GPIO_BASE: usize = PERIPHERAL_BASE + 0x200_000;
const AUX_BASE: usize = PERIPHERAL_BASE + 0x215_000;

struct UartLogger;

impl UartLogger {
    #[inline(always)]
    fn mmio_write(addr: usize, val: u32) {
        unsafe { core::ptr::write_volatile(addr as *mut u32, val) }
    }

    #[inline(always)]
    fn mmio_read(addr: usize) -> u32 {
        unsafe { core::ptr::read_volatile(addr as *const u32) }
    }

    #[inline(always)]
    fn delay(count: u32) {
        for _ in 0..count {
            unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)) }
        }
    }

    fn init() {
        // Enable mini UART.
        const AUX_ENABLES: usize = AUX_BASE + 0x04;
        const AUX_MU_IER: usize = AUX_BASE + 0x44;
        const AUX_MU_CNTL: usize = AUX_BASE + 0x60;
        const AUX_MU_LCR: usize = AUX_BASE + 0x4C;
        const AUX_MU_MCR: usize = AUX_BASE + 0x50;
        const AUX_MU_IIR: usize = AUX_BASE + 0x48;
        const AUX_MU_BAUD: usize = AUX_BASE + 0x68;

        Self::mmio_write(AUX_ENABLES, 1); // enable mini UART
        Self::mmio_write(AUX_MU_CNTL, 0); // disable TX/RX during config
        Self::mmio_write(AUX_MU_IER, 0); // disable interrupts
        Self::mmio_write(AUX_MU_LCR, 3); // 8-bit mode
        Self::mmio_write(AUX_MU_MCR, 0); // RTS high
        Self::mmio_write(AUX_MU_IIR, 0xC6); // clear FIFOs

        // Baud rate:
        // baud = system_clock / (8 * (AUX_MU_BAUD + 1))
        // With core_freq=250MHz => system_clock=250_000_000, AUX_MU_BAUD=270 gives ~115200.
        Self::mmio_write(AUX_MU_BAUD, 270);

        // GPIO14/15 to ALT5 (TXD1/RXD1) for mini UART.
        const GPFSEL1: usize = GPIO_BASE + 0x04;
        const GPPUD: usize = GPIO_BASE + 0x94;
        const GPPUDCLK0: usize = GPIO_BASE + 0x98;

        let mut r = Self::mmio_read(GPFSEL1);
        // GPIO14 (bits 12..14) = ALT5 (010)
        r &= !(0b111 << 12);
        r |= 0b010 << 12;
        // GPIO15 (bits 15..17) = ALT5 (010)
        r &= !(0b111 << 15);
        r |= 0b010 << 15;
        Self::mmio_write(GPFSEL1, r);

        // Disable GPIO pull-up/down for pins 14 and 15.
        Self::mmio_write(GPPUD, 0);
        Self::delay(150);
        Self::mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
        Self::delay(150);
        Self::mmio_write(GPPUDCLK0, 0);

        // Enable TX/RX.
        Self::mmio_write(AUX_MU_CNTL, 3);
    }

    fn putc(c: u8) {
        const AUX_MU_IO: usize = AUX_BASE + 0x40;
        const AUX_MU_LSR: usize = AUX_BASE + 0x54;

        // LSR bit5 = transmitter empty (can accept at least one byte)
        while (Self::mmio_read(AUX_MU_LSR) & (1 << 5)) == 0 {}
        Self::mmio_write(AUX_MU_IO, c as u32);
    }

    fn puts(s: &str) {
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

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    UartLogger::init();
    let logger = UartLogger;
    logger.log("rustOS: aarch64 RPi (Zero 2 W) boot OK\n");
    kernel::kmain(&logger)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    UartLogger::puts("rustOS: PANIC\n");
    loop {
        hal::arch::halt();
    }
}


