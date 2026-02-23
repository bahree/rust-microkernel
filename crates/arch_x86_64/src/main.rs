#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use hal::log::Logger;
use spin::Mutex;
use uart_16550::SerialPort;

struct SerialLogger;

static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

impl SerialLogger {
    fn init() {
        SERIAL1.lock().init();
    }
}

impl hal::log::Logger for SerialLogger {
    fn log(&self, s: &str) {
        use core::fmt::Write;
        let mut port = SERIAL1.lock();
        for &b in s.as_bytes() {
            match b {
                b'\n' => {
                    let _ = port.write_str("\r\n");
                }
                b'\r' => {}
                _ => {
                    let _ = port.write_char(b as char);
                }
            }
        }
    }
}

entry_point!(kernel_entry);

fn kernel_entry(_boot_info: &'static mut BootInfo) -> ! {
    SerialLogger::init();
    let logger = SerialLogger;
    logger.log("rustOS: x86_64 boot OK\n");
    kernel::kmain(&logger)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let logger = SerialLogger;
    logger.log("rustOS: PANIC\n");
    use core::fmt::Write;
    let mut port = SERIAL1.lock();
    let _ = write!(port, "details: {}\r\n", info);
    loop {
        hal::arch::halt();
    }
}


