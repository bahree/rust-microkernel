use core::ptr::{read_volatile, write_volatile};

use super::UartLogger;

// QEMU virt RAM (we force -m 256M in the run script)
const RAM_START: u64 = 0x4000_0000;
const RAM_SIZE: u64 = 256 * 1024 * 1024;
const RAM_END: u64 = RAM_START + RAM_SIZE;

const PAGE_SIZE: u64 = 4096;

extern "C" {
    static __stack_top: u8;
}

#[inline(always)]
fn align_up(x: u64, align: u64) -> u64 {
    (x + align - 1) & !(align - 1)
}

struct FrameAlloc {
    next: u64,
    end: u64,
}

impl FrameAlloc {
    fn new(start: u64, end: u64) -> Self {
        Self { next: start, end }
    }

    fn alloc(&mut self) -> Option<u64> {
        let p = self.next;
        if p + PAGE_SIZE > self.end {
            return None;
        }
        self.next += PAGE_SIZE;
        Some(p)
    }
}

// AArch64 4k-page tables.
#[repr(align(4096))]
struct PageTable {
    entries: [u64; 512],
}

impl PageTable {
    const fn new() -> Self {
        Self { entries: [0; 512] }
    }
}

extern "C" {
    fn enable_mmu(ttbr0: u64);
}

// Descriptor bits (4k granule).
const DESC_VALID: u64 = 1 << 0;
const DESC_TABLE: u64 = 1 << 1; // when valid and bit1=1 => table/page
const DESC_BLOCK: u64 = 0 << 1; // when valid and bit1=0 => block

const AF: u64 = 1 << 10;
const SH_INNER: u64 = 0b11 << 8;
const ATTRIDX0: u64 = 0 << 2; // normal memory
const ATTRIDX1: u64 = 1 << 2; // device memory

// Device should be XN.
const PXN: u64 = 1 << 53;
const UXN: u64 = 1 << 54;

static mut TT_L0: PageTable = PageTable::new();
static mut TT_L1: PageTable = PageTable::new();
static mut TT_L2_0: PageTable = PageTable::new(); // VA 0..1GB (UART)
static mut TT_L2_1: PageTable = PageTable::new(); // VA 1..2GB (RAM)
static mut TT_L2_2: PageTable = PageTable::new(); // VA 2..3GB (test VA)
static mut TT_L3_TEST: PageTable = PageTable::new();

fn build_tables(frame0: u64) -> (u64, u64) {
    // We'll map a test VA in the low VA space so TTBR0 can translate it.
    let test_va: u64 = 0x8000_0000; // 2GB

    unsafe {
        TT_L0.entries = [0; 512];
        TT_L1.entries = [0; 512];
        TT_L2_0.entries = [0; 512];
        TT_L2_1.entries = [0; 512];
        TT_L2_2.entries = [0; 512];
        TT_L3_TEST.entries = [0; 512];

        // L0[0] -> L1 (covers low VA range)
        TT_L0.entries[0] = (&raw const TT_L1 as *const _ as u64) | DESC_VALID | DESC_TABLE;

        // L1[0] (0..1GB) -> L2_0
        TT_L1.entries[0] = (&raw const TT_L2_0 as *const _ as u64) | DESC_VALID | DESC_TABLE;
        // L1[1] (1..2GB) -> L2_1 (RAM at 0x4000_0000)
        TT_L1.entries[1] = (&raw const TT_L2_1 as *const _ as u64) | DESC_VALID | DESC_TABLE;
        // L1[2] (2..3GB) -> L2_2 (test VA)
        TT_L1.entries[2] = (&raw const TT_L2_2 as *const _ as u64) | DESC_VALID | DESC_TABLE;

        // Map UART 0x0900_0000 as a 2MB device block (identity).
        let uart_va: u64 = 0x0900_0000;
        let uart_l2 = ((uart_va >> 21) & 0x1FF) as usize;
        TT_L2_0.entries[uart_l2] =
            (uart_va & 0xFFFF_FFFF_FFE0_0000) | DESC_VALID | DESC_BLOCK | ATTRIDX1 | AF | PXN | UXN;

        // Map RAM 0x4000_0000..0x5000_0000 as 2MB blocks (identity), normal memory.
        let blocks = RAM_SIZE / (2 * 1024 * 1024);
        for i in 0..blocks {
            let va = RAM_START + i * 2 * 1024 * 1024;
            let pa = va;
            let idx = ((va >> 21) & 0x1FF) as usize; // within L2_1
            TT_L2_1.entries[idx] =
                (pa & 0xFFFF_FFFF_FFE0_0000) | DESC_VALID | DESC_BLOCK | ATTRIDX0 | AF | SH_INNER;
        }

        // Map test_va -> frame0 as a single 4k page:
        // L2 entry points to L3 table.
        let test_l2 = ((test_va >> 21) & 0x1FF) as usize;
        TT_L2_2.entries[test_l2] =
            (&raw const TT_L3_TEST as *const _ as u64) | DESC_VALID | DESC_TABLE;
        let test_l3 = ((test_va >> 12) & 0x1FF) as usize;
        TT_L3_TEST.entries[test_l3] = (frame0 & 0xFFFF_FFFF_FFFF_F000)
            | DESC_VALID
            | DESC_TABLE
            | ATTRIDX0
            | AF
            | SH_INNER;
    }

    let ttbr0 = &raw const TT_L0 as *const _ as u64;
    (ttbr0, test_va)
}

fn put_hex(prefix: &str, v: u64) {
    UartLogger::puts(prefix);
    // very small hex printer
    let mut buf = [0u8; 16];
    for i in 0..16 {
        let shift = 60 - (i * 4);
        let nib = ((v >> shift) & 0xF) as u8;
        buf[i] = match nib {
            0..=9 => b'0' + nib,
            _ => b'A' + (nib - 10),
        };
    }
    for b in buf {
        UartLogger::puts(core::str::from_utf8(&[b]).unwrap_or("?"));
    }
    UartLogger::puts("\n");
}

pub fn demo() {
    UartLogger::puts("mm: demo start\n");

    let kernel_end = unsafe { &__stack_top as *const u8 as u64 };
    let free_start = align_up(kernel_end, PAGE_SIZE);

    put_hex("mm: kernel_end=0x", kernel_end);
    put_hex("mm: free_start=0x", free_start);
    put_hex("mm: ram_end=0x", RAM_END);

    let mut fa = FrameAlloc::new(free_start, RAM_END);

    // Allocate a few frames and write/read patterns.
    let f0 = fa.alloc().expect("no frame");
    let f1 = fa.alloc().expect("no frame");
    put_hex("mm: frame0=0x", f0);
    put_hex("mm: frame1=0x", f1);

    unsafe {
        write_volatile(f0 as *mut u32, 0xAABB_CCDD);
        write_volatile(f1 as *mut u32, 0x1122_3344);
        let r0 = read_volatile(f0 as *const u32) as u64;
        let r1 = read_volatile(f1 as *const u32) as u64;
        put_hex("mm: read0=0x", r0);
        put_hex("mm: read1=0x", r1);
    }

    // Build real page tables for TTBR0 and enable MMU.
    let (ttbr0, test_va) = build_tables(f0);
    put_hex("mm: ttbr0=0x", ttbr0);
    put_hex("mm: test_va=0x", test_va);

    UartLogger::puts("mm: enabling MMU (caches off)...\n");
    unsafe { enable_mmu(ttbr0) };

    // If we survived, translation is live. Read/write through test_va.
    unsafe {
        let p = test_va as *mut u32;
        write_volatile(p, 0xDEAD_BEEF);
        let r = read_volatile(p) as u64;
        put_hex("mm: test_va_read=0x", r);
    }

    UartLogger::puts("mm: demo done (MMU is ON)\n");
}


