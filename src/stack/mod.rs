use alloc::vec::Vec;

pub mod apl;
pub(crate) mod channel;
pub mod mbal;
pub mod phl;

/// The LinkIQ protocol stack
pub struct Stack {
    phl: phl::Phl<mbal::Mbal<apl::Apl>>,
}

/// Layer trait
pub trait Layer {
    fn read(&self, packet: &mut Packet, buffer: &[u8]) -> Result<(), ReadError>;
    fn write(&self, writer: &mut Vec<u8>, packet: &Packet);
}

/// A LinkIQ packet
#[derive(Default)]
pub struct Packet {
    pub rssi: Option<Rssi>,
    pub phl: Option<phl::PhlFields>,
    pub mbal: Option<mbal::MbalFields>,
    pub mbus_data: Vec<u8>,
}

pub type Rssi = i8;
pub use channel::Channel;

#[derive(Debug)]
pub enum ReadError {
    NotEnoughBytes,
    PhlDecodeError,
    MBalCrcError,
    MBalControlError,
    MBalAddressError,
    MBalCommandError,
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

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
