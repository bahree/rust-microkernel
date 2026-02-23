#![no_std]

use hal::log::Logger;

mod ipc;
mod sched;

use core::cell::UnsafeCell;

#[repr(transparent)]
struct RouterCell(UnsafeCell<ipc::Router>);
unsafe impl Sync for RouterCell {}

// Force the router into a writable section. On some bare-metal targets, a `static`
// with interior mutability can otherwise end up in a read-only segment, causing
// a data abort when we first write to it (exactly what we saw on aarch64 QEMU virt).
#[link_section = ".data"]
static ROUTER: RouterCell = RouterCell(UnsafeCell::new(ipc::Router::new()));

pub fn kmain(logger: &dyn Logger) -> ! {
    logger.log("rustOS: kernel online\n");
    logger.log("rustOS: microkernel step 1 (IPC + cooperative scheduling)\n");

    let router: &mut ipc::Router = unsafe { &mut *ROUTER.0.get() };

    let mut ping = sched::PingTask::new();
    let mut pong = sched::PongTask::new();
    let mut tasks: [&mut dyn sched::Task; 2] = [&mut ping, &mut pong];

    sched::run(&mut tasks, logger, router)
}


