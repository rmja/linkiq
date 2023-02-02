pub mod apl;
pub(crate) mod channel;
pub mod mbal;
pub mod phl;

use heapless::Vec;

/// The LinkIQ protocol stack
pub struct Stack {
    phl: phl::Phl<mbal::Mbal<apl::Apl>>,
}

/// Layer trait
pub trait Layer {
    fn read<const N: usize>(&self, packet: &mut Packet<N>, buffer: &[u8]) -> Result<(), ReadError>;
    fn write<const N: usize>(&self, writer: &mut impl Writer, packet: &Packet<N>) -> Result<(), WriteError>;
}

/// A LinkIQ packet
#[derive(Default)]
pub struct Packet<const N: usize = {apl::MBUS_DATA_MAX}> {
    pub rssi: Option<Rssi>,
    pub phl: Option<phl::PhlFields>,
    pub mbal: Option<mbal::MbalFields>,
    pub mbus_data: Vec<u8, N>,
}

pub type Rssi = i16;

pub trait Writer {
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError>;
}

impl<const N: usize> Writer for Vec<u8, N> {
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError> {
        self.extend_from_slice(buf)
            .map_err(|_| WriteError::Capacity)
    }
}

#[cfg(feature = "alloc")]
impl Writer for alloc::vec::Vec<u8> {
    fn write(&mut self, buf: &[u8]) -> Result<(), WriteError> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

pub use channel::Channel;

#[derive(Debug)]
pub enum ReadError {
    Capacity,
    NotEnoughBytes,
    PhlDecodeError,
    MBalCrcError,
    MBalControlError,
    MBalAddressError,
    MBalCommandError,
}

#[derive(Debug, PartialEq)]
pub enum WriteError {
    Capacity,
}

impl Stack {
    /// Create a new LinkIQ stack
    pub fn new() -> Self {
        Self {
            phl: phl::Phl::new(mbal::Mbal::new(apl::Apl::new())),
        }
    }

    /// Read a packet from a byte buffer
    pub fn read<const N: usize>(&self, buffer: &[u8]) -> Result<Packet<N>, ReadError> {
        let mut packet = Packet::default();
        self.phl.read(&mut packet, buffer)?;
        Ok(packet)
    }

    /// Write a packet
    pub fn write<const N: usize>(&self, writer: &mut impl Writer, packet: &Packet<N>) -> Result<(), WriteError> {
        self.phl.write(writer, packet)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
