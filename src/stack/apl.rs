use super::{Layer, Packet, ReadError, Writer, WriteError, mbal};
use heapless::Vec;

pub const MBUS_DATA_MAX: usize = mbal::MBAL_MAX - mbal::HEADER_SIZE;

/// Application Layer
pub struct Apl;

impl Apl {
    pub const fn new() -> Self {
        Self
    }
}

impl Layer for Apl {
    fn read<const N: usize>(&self, packet: &mut Packet<N>, buffer: &[u8]) -> Result<(), ReadError> {
        packet.mbus_data = Vec::from_slice(buffer).map_err(|_| ReadError::Capacity)?;
        Ok(())
    }

    fn write<const N: usize>(&self, writer: &mut impl Writer, packet: &Packet<N>) -> Result<(), WriteError> {
        writer.write(&packet.mbus_data)
    }
}
