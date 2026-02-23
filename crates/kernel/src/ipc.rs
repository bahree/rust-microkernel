#![allow(dead_code)]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EndpointId {
    Ping = 1,
    Pong = 2,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum MsgType {
    Ping = 1,
    Pong = 2,
}

// Keep these small during early bring-up so IPC queues fit comfortably on the stack
// across all targets (we'll grow them once we have robust MMU + fault handling).
pub const MAX_PAYLOAD: usize = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MsgHeader {
    pub src: EndpointId,
    pub dst: EndpointId,
    pub ty: MsgType,
    pub len: u8,
    pub seq: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Message {
    pub header: MsgHeader,
    pub payload: [u8; MAX_PAYLOAD],
}

#[derive(Copy, Clone, Debug)]
pub enum SendError {
    MailboxFull,
}

#[derive(Copy, Clone)]
struct Mailbox {
    full: bool,
    msg: Message,
}

impl Mailbox {
    const EMPTY: Message = Message {
        header: MsgHeader {
            src: EndpointId::Ping,
            dst: EndpointId::Ping,
            ty: MsgType::Ping,
            len: 0,
            seq: 0,
        },
        payload: [0; MAX_PAYLOAD],
    };

    const fn new() -> Self {
        Self {
            full: false,
            msg: Self::EMPTY,
        }
    }

    fn put(&mut self, msg: Message) -> Result<(), SendError> {
        if self.full {
            return Err(SendError::MailboxFull);
        }
        self.msg = msg;
        self.full = true;
        Ok(())
    }

    fn take(&mut self) -> Option<Message> {
        if !self.full {
            return None;
        }
        self.full = false;
        Some(self.msg)
    }
}

pub struct Router {
    ping: Mailbox,
    pong: Mailbox,
}

impl Router {
    pub const fn new() -> Self {
        Self {
            ping: Mailbox::new(),
            pong: Mailbox::new(),
        }
    }

    pub fn send(&mut self, msg: Message) -> Result<(), SendError> {
        match msg.header.dst {
            EndpointId::Ping => self.ping.put(msg),
            EndpointId::Pong => self.pong.put(msg),
        }
    }

    pub fn recv(&mut self, dst: EndpointId) -> Option<Message> {
        match dst {
            EndpointId::Ping => self.ping.take(),
            EndpointId::Pong => self.pong.take(),
        }
    }
}

pub fn write_u32_le(dst: &mut [u8], v: u32) {
    dst[0] = (v & 0xFF) as u8;
    dst[1] = ((v >> 8) & 0xFF) as u8;
    dst[2] = ((v >> 16) & 0xFF) as u8;
    dst[3] = ((v >> 24) & 0xFF) as u8;
}

pub fn read_u32_le(src: &[u8]) -> u32 {
    (src[0] as u32)
        | ((src[1] as u32) << 8)
        | ((src[2] as u32) << 16)
        | ((src[3] as u32) << 24)
}


