use core::time::Duration;

use alloc::vec::Vec;

pub mod apl;
pub mod mbal;
pub mod phl;

#[derive(Debug)]
pub enum ReadError {
    NotEnoughBytes,
    PhlDecodeError,
    MBalCrcError,
    MBalControlError,
    MBalAddressError,
    MBalCommandError,
}

/// The LinkIQ protocol stack
pub struct Stack {
    phl: phl::Phl<mbal::Mbal<apl::Apl>>,
}

/// Layer trait that must be implemented by any layer.
pub trait Layer {
    fn read(&self, packet: &mut Packet, buffer: &[u8]) -> Result<(), ReadError>;
    fn write(&self, writer: &mut Vec<u8>, packet: &Packet);
}

/// A single packet
#[derive(Default)]
pub struct Packet {
    pub uptime: Option<Duration>,
    pub phl: Option<phl::PhlFields>,
    pub mbal: Option<mbal::MbalFields>,
    pub mbus_data: Vec<u8>,
}

impl Stack {
    /// Create a new LinkIQ stack
    pub fn new() -> Self {
        Self {
            phl: phl::Phl::new(mbal::Mbal::new(apl::Apl::new())),
        }
    }

    /// Read a packet from a byte buffer
    pub fn read(&self, buffer: &[u8]) -> Result<Packet, ReadError> {
        let mut packet = Packet::default();
        self.phl.read(&mut packet, buffer)?;
        Ok(packet)
    }

    /// Write a packet
    pub fn write(&self, writer: &mut Vec<u8>, packet: &Packet) {
        self.phl.write(writer, packet)
    }
}
