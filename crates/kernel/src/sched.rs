use crate::ipc::{self, EndpointId, MsgType};
use hal::log::Logger;

pub trait Task {
    fn id(&self) -> EndpointId;
    fn poll(&mut self, logger: &dyn Logger, ipc: &mut ipc::Router, tick: u64);
}

pub fn run(tasks: &mut [&mut dyn Task], logger: &dyn Logger, ipc: &mut ipc::Router) -> ! {
    let mut tick: u64 = 0;
    logger.log("sched: starting\n");
    loop {
        for t in tasks.iter_mut() {
            t.poll(logger, ipc, tick);
        }
        tick = tick.wrapping_add(1);
        // Sleep until the next interrupt (timer tick wakes us).
        hal::arch::halt();
    }
}

pub struct PingTask {
    seq: u32,
    waiting: bool,
}

impl PingTask {
    pub const fn new() -> Self {
        Self {
            seq: 1,
            waiting: false,
        }
    }
}

impl Task for PingTask {
    fn id(&self) -> EndpointId {
        EndpointId::Ping
    }

    fn poll(&mut self, logger: &dyn Logger, ipc: &mut ipc::Router, tick: u64) {
        if tick == 0 {
            logger.log("task/ping: poll\n");
        }
        // Check replies first.
        if let Some(msg) = ipc.recv(self.id()) {
            if matches!(msg.header.ty, MsgType::Pong) {
                self.waiting = false;
                logger.log("task/ping: got pong\n");
            }
        }

        // Send a ping periodically when not waiting for a reply.
        // (Tuned to be visible even without a real timer interrupt.)
        // With a 100ms timer tick, this sends roughly once every ~1s.
        if !self.waiting && (tick % 10) == 0 {
            let mut payload = [0u8; ipc::MAX_PAYLOAD];
            ipc::write_u32_le(&mut payload[0..4], self.seq);
            let msg = ipc::Message {
                header: ipc::MsgHeader {
                    src: EndpointId::Ping,
                    dst: EndpointId::Pong,
                    ty: MsgType::Ping,
                    len: 4,
                    seq: self.seq,
                },
                payload,
            };

            match ipc.send(msg) {
                Ok(()) => {
                    logger.log("task/ping: sent ping\n");
                    self.waiting = true;
                    self.seq = self.seq.wrapping_add(1);
                }
                Err(_) => {
                    logger.log("task/ping: send failed (queue full)\n");
                }
            }
        }
    }
}

pub struct PongTask;

impl PongTask {
    pub const fn new() -> Self {
        Self
    }
}

impl Task for PongTask {
    fn id(&self) -> EndpointId {
        EndpointId::Pong
    }

    fn poll(&mut self, logger: &dyn Logger, ipc: &mut ipc::Router, _tick: u64) {
        if let Some(msg) = ipc.recv(self.id()) {
            if matches!(msg.header.ty, MsgType::Ping) {
                let seq = ipc::read_u32_le(&msg.payload[0..4]);
                logger.log("task/pong: got ping\n");

                let mut payload = [0u8; ipc::MAX_PAYLOAD];
                ipc::write_u32_le(&mut payload[0..4], seq);
                let reply = ipc::Message {
                    header: ipc::MsgHeader {
                        src: EndpointId::Pong,
                        dst: EndpointId::Ping,
                        ty: MsgType::Pong,
                        len: 4,
                        seq,
                    },
                    payload,
                };
                let _ = ipc.send(reply);
            }
        }
    }
}


