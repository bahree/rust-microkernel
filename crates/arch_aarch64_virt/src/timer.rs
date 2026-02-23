#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

use super::preempt::Context;

static TICKS: AtomicU64 = AtomicU64::new(0);
static CNTFRQ: AtomicU64 = AtomicU64::new(0);

// GICv2 memory map on QEMU `virt` (when using gic-version=2)
const GICD_BASE: usize = 0x0800_0000;
const GICC_BASE: usize = 0x0801_0000;

// GICD registers
const GICD_CTLR: usize = 0x000;
const GICD_ISENABLER0: usize = 0x100;
const GICD_IPRIORITYR: usize = 0x400;
const GICD_ITARGETSR: usize = 0x800;

// GICC registers
const GICC_CTLR: usize = 0x0000;
const GICC_PMR: usize = 0x0004;
const GICC_IAR: usize = 0x000C;
const GICC_EOIR: usize = 0x0010;

// ARM generic timer interrupt ID (PPI)
const IRQ_CNTPNS: u32 = 30;

#[inline(always)]
fn mmio_write32(base: usize, off: usize, val: u32) {
    unsafe { core::ptr::write_volatile((base + off) as *mut u32, val) }
}

#[inline(always)]
fn mmio_read32(base: usize, off: usize) -> u32 {
    unsafe { core::ptr::read_volatile((base + off) as *const u32) }
}

fn enable_irq(id: u32) {
    // Enable SGI/PPI IDs 0..31 in ISENABLER0
    if id < 32 {
        let mask = 1u32 << id;
        mmio_write32(GICD_BASE, GICD_ISENABLER0, mask);

        // Priority for interrupt id
        let pri_off = GICD_IPRIORITYR + (id as usize);
        unsafe { core::ptr::write_volatile((GICD_BASE + pri_off) as *mut u8, 0x80) };

        // Target CPU0 (not required for PPIs, but harmless)
        let tgt_off = GICD_ITARGETSR + (id as usize);
        unsafe { core::ptr::write_volatile((GICD_BASE + tgt_off) as *mut u8, 0x01) };
    }
}

fn program_timer(freq: u64) {
    // 100ms tick by default (human friendly)
    let tval = (freq / 10) as u64;
    unsafe {
        core::arch::asm!(
            "msr cntp_tval_el0, {tval}",
            "mov x0, #1",
            "msr cntp_ctl_el0, x0",
            tval = in(reg) tval,
            out("x0") _,
            options(nostack, nomem)
        );
    }
}

pub fn init() {
    // Enable GIC distributor
    mmio_write32(GICD_BASE, GICD_CTLR, 1);

    // Enable GIC CPU interface
    mmio_write32(GICC_BASE, GICC_PMR, 0xFF); // accept all priorities
    mmio_write32(GICC_BASE, GICC_CTLR, 1);

    // Enable physical timer interrupt
    enable_irq(IRQ_CNTPNS);

    // Read counter frequency and program timer
    let freq: u64;
    unsafe {
        core::arch::asm!("mrs {0}, cntfrq_el0", out(reg) freq, options(nostack, nomem));
    }
    CNTFRQ.store(freq, Ordering::Relaxed);
    program_timer(freq);

    // Unmask IRQs (clear DAIF.I)
    unsafe {
        core::arch::asm!("msr daifclr, #2", options(nostack, nomem));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_irq_handler(current: *mut Context) -> *const Context {
    let iar = mmio_read32(GICC_BASE, GICC_IAR);
    let id = iar & 0x3FF;

    let mut next: *const Context = current;
    if id == IRQ_CNTPNS {
        let t = TICKS.fetch_add(1, Ordering::Relaxed) + 1;
        let freq = CNTFRQ.load(Ordering::Relaxed);
        if freq != 0 {
            program_timer(freq);
        }

        // Switch threads every 5 ticks (~500ms with 100ms tick).
        if (t % 5) == 0 {
            next = super::preempt::switch_next(current);
        }
    }

    // End of interrupt
    mmio_write32(GICC_BASE, GICC_EOIR, iar);
    next
}

pub fn ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}


