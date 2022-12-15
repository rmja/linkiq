use crate::wmbus::WMBusAddress;

use alloc::vec::Vec;
use crc::{Crc, CRC_16_EN_13757};
use num_traits::FromPrimitive;

use super::{Layer, Packet, ReadError};

const HEADER_SIZE: usize = 12;
const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_EN_13757);

/// M-Bus Adaption Layer
pub struct Mbal<A: Layer> {
    above: A,
}

/// M-Bus Adaption Layer Fields
pub struct MbalFields {
    pub control: MbalControl,
    pub address: WMBusAddress,
    pub command: MbalCommand,
}

pub struct MbalControl {
    pub is_prioritized: bool,
}

pub struct MbalCommand {
    pub function_code: MbalFunctionCode,
}

#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Copy)]
pub enum MbalFunctionCode {
    /// SND-NR
    SendUnsolicitedApplicationData = 4,

    /// SND-IR
    SendInstallationRequest = 6,
}

impl<A: Layer> Mbal<A> {
    pub fn new(above: A) -> Self {
        Self { above }
    }
}

impl<A: Layer> Layer for Mbal<A> {
    fn read(&self, packet: &mut Packet, buffer: &[u8]) -> Result<(), ReadError> {
        if buffer.len() < HEADER_SIZE {
            return Err(ReadError::NotEnoughBytes);
        } else if !is_valid_crc(&buffer[..HEADER_SIZE]) {
            return Err(ReadError::MBalCrcError);
        }

        let control = MbalControl {
            is_prioritized: match buffer[0] {
                0 => false,
                1 => true,
                _ => return Err(ReadError::MBalControlError),
            },
        };

        let address = buffer[1..]
            .try_into()
            .map_err(|_| ReadError::MBalAddressError)?;

        let command = MbalCommand {
            // TODO: Why the shifts
            function_code: match MbalFunctionCode::from_u8(buffer[9] >> 4) {
                Some(code) => code,
                None => return Err(ReadError::MBalCommandError),
            },
        };

        packet.mbal = Some(MbalFields {
            control,
            address,
            command,
        });

        self.above.read(packet, &buffer[HEADER_SIZE..])
    }

    fn write(&self, writer: &mut Vec<u8>, packet: &Packet) {
        let fields = packet.mbal.as_ref().unwrap();

        let start = writer.len();
        writer.reserve(HEADER_SIZE);
        writer.push(fields.control.is_prioritized as u8);
        writer.extend_from_slice(fields.address.get_bytes().as_slice());

        // TODO: WHY the shifts?
        writer.push((fields.command.function_code as u8) << 4);

        // Append CRC
        let mut digest = CRC.digest();
        digest.update(&writer[start..]);
        let crc = digest.finalize();
        writer.extend_from_slice(crc.to_be_bytes().as_slice());

        self.above.write(writer, packet)
    }
}

fn is_valid_crc(block: &[u8]) -> bool {
    let index = block.len() - 2;

    let mut digest = CRC.digest();
    digest.update(&block[..index]);
    let actual = digest.finalize();

    let expected = u16::from_be_bytes(block[index..].try_into().unwrap());

    actual == expected
}
