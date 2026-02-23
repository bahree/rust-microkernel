#[inline(always)]
pub fn halt() {
    // Low-power halt per arch. Keep it tiny and dependency-free.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        // Use WFI (wait-for-interrupt) so we reliably sleep until the next IRQ.
        // WFE can return immediately if an event is already latched.
        core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        loop {}
    }
}


