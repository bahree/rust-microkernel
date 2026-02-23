#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

use super::{timer, UartLogger};

#[repr(C)]
pub struct Context {
    pub x: [u64; 31], // x0..x30
    pub sp: u64,
    pub elr: u64,
    pub spsr: u64,
}

const STACK_SIZE: usize = 16 * 1024;

#[repr(align(16))]
struct Stack([u8; STACK_SIZE]);

static STACK0: Stack = Stack([0; STACK_SIZE]);
static STACK1: Stack = Stack([0; STACK_SIZE]);

static mut CTX: [Context; 2] = [
    Context {
        x: [0; 31],
        sp: 0,
        elr: 0,
        spsr: 0x5, // EL1h
    },
    Context {
        x: [0; 31],
        sp: 0,
        elr: 0,
        spsr: 0x5, // EL1h
    },
];

static CURRENT: AtomicUsize = AtomicUsize::new(0);

pub(crate) extern "C" fn thread_a_entry() -> ! {
    // Enable the timer after the first thread context is active.
    timer::init();
    let mut last_tick: u64 = 0;
    loop {
        // Print rarely so QEMU escape sequences are usable.
        // (Ticks are 100ms; print ~once/second.)
        let t = timer::ticks();
        if t != last_tick {
            last_tick = t;
            if (t % 10) == 0 {
                UartLogger::puts("A\n");
            }
        }
        core::hint::spin_loop();
    }
}

pub(crate) extern "C" fn thread_b_entry() -> ! {
    let mut last_tick: u64 = 0;
    loop {
        let t = timer::ticks();
        if t != last_tick {
            last_tick = t;
            if (t % 10) == 5 {
                UartLogger::puts("B\n");
            }
        }
        core::hint::spin_loop();
    }
}

pub fn init() {
    unsafe {
        let top0 = STACK0.0.as_ptr().add(STACK_SIZE) as u64;
        let top1 = STACK1.0.as_ptr().add(STACK_SIZE) as u64;

        CTX[0].sp = top0;
        CTX[0].elr = thread_a_entry as *const () as usize as u64;
        CTX[0].spsr = 0x5;

        CTX[1].sp = top1;
        CTX[1].elr = thread_b_entry as *const () as usize as u64;
        CTX[1].spsr = 0x5;
    }
}

pub fn first_context() -> *const Context {
    unsafe { &CTX[0] as *const Context }
}

pub fn switch_next(_current_ctx: *mut Context) -> *const Context {
    // Round-robin 0 <-> 1
    let cur = CURRENT.load(Ordering::Relaxed);
    let next = cur ^ 1;
    CURRENT.store(next, Ordering::Relaxed);
    unsafe { &CTX[next] as *const Context }
}


