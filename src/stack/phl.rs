use bitvec::prelude::*;
use crc::{Algorithm, Crc};
use fastfec::{
    turbo::{umts::UmtsTurboDecoder, TurboEncoder},
    Llr,
};
use funty::Integral;
use heapless::Vec;

use crate::{
    bitreader::{BitField, BitReader},
    fec::{CodeRate, EncoderTermination, TurboDecoderInput, TurboEncoderOutput},
    phycodedheader::PhyCodedHeader,
    phyinterleaver,
    stack::mbal,
};

use super::{Layer, Packet, ReadError, WriteError, Writer};

pub const HEADER_SIZE: usize = 12;
const CRC_ALGORITHM: Algorithm<u32> = Algorithm::<u32> {
    poly: 0xf4acfb13,
    init: 0x00000000,
    refin: false,
    refout: false,
    xorout: 0x00000000,
    check: 0x6C9F84A8,
    residue: 0x00000000,
};
pub(crate) const CRC: Crc<u32> = Crc::<u32>::new(&CRC_ALGORITHM);

/// Physical Layer
pub struct Phl<A: Layer> {
    above: A,
    encoder: TurboEncoder,
    decoder: UmtsTurboDecoder,
    pub max_decode_iterations: usize,
}

/// Physical Layer Fields
pub struct PhlFields {
    pub code_rate: CodeRate,
    pub header_distance: usize,
    pub decode_iterations: usize,
    pub decode_distance: usize,
}

pub const MAX_FRAME_LENGTH: usize = HEADER_SIZE + 3 * mbal::MBAL_MAX;

pub fn get_frame_length(buffer: &[u8]) -> Result<usize, ReadError> {
    if buffer.len() < HEADER_SIZE {
        return Err(ReadError::NotEnoughBytes);
    }

    let mut reader = BitReader::from_slice(buffer);
    reader.read_bits::<usize>(2).unwrap(); // Discard the two padding bits

    let (header, _) = PhyCodedHeader::read(&mut reader).unwrap();
    let frame_length = get_frame_length_from_header(&header);
    Ok(frame_length)
}

fn get_frame_length_from_header(header: &PhyCodedHeader) -> usize {
    let block_length = header.data_length + 4;
    #[allow(clippy::identity_op)]
    let parity_bits = match header.rate {
        CodeRate::OneThird => (3 - 1) * (block_length * 8),
        CodeRate::OneHalf => (2 - 1) * (block_length * 8),
    };
    HEADER_SIZE + block_length + (parity_bits + 7) / 8
}

impl<A: Layer> Phl<A> {
    pub fn new(above: A) -> Self {
        Self {
            above,
            encoder: TurboEncoder::new(fastfec::catalog::UMTS),
            decoder: UmtsTurboDecoder::new(fastfec::catalog::UMTS),
            max_decode_iterations: 10,
        }
    }

    fn distance<T: Integral>(first: &[T], second: &[T]) -> usize {
        assert_eq!(first.len(), second.len());

        let mut distance = 0;
        for i in 0..first.len() {
            distance += (first[i] ^ second[i]).count_ones() as usize;
        }
        distance
    }

    fn run_decoder<const N: usize>(
        &self,
        data_length: usize,
        input: &TurboDecoderInput<N>,
    ) -> Option<(Vec<u8, N>, usize)> {
        let interleaver = phyinterleaver::create(input.symbols.len())?;
        let mut decoding = self.decoder.decode(
            &input.symbols,
            interleaver,
            &input.first_termination,
            &input.second_termination,
        );

        let mut hard = BitVec::<u8, Msb0>::with_capacity(input.symbols.len());

        for iteration in 1..=self.max_decode_iterations {
            decoding.run_decode_iteration();

            for llr in decoding.get_result() {
                hard.push(*llr > 0);
            }

            if is_valid_crc(data_length, hard.as_raw_slice()) {
                return Some((Vec::from_slice(hard.as_raw_slice()).unwrap(), iteration));
            }

            hard.clear();
        }

        None
    }
}

impl<A: Layer> Layer for Phl<A> {
    fn read<const N: usize>(&self, packet: &mut Packet<N>, buffer: &[u8]) -> Result<(), ReadError> {
        let mut reader = BitReader::from_slice(buffer);
        reader
            .read_bits::<usize>(2)
            .ok_or(ReadError::NotEnoughBytes)?; // Discard the two padding bits

        let (header, header_distance) =
            PhyCodedHeader::read(&mut reader).ok_or(ReadError::NotEnoughBytes)?;
        let frame_length = get_frame_length_from_header(&header);
        if buffer.len() < frame_length {
            return Err(ReadError::NotEnoughBytes);
        }

        let first_termination = EncoderTermination(reader.read_bits(6).unwrap());
        let second_termination = EncoderTermination(reader.read_bits(6).unwrap());

        let data_length = header.data_length;
        let block_length = data_length + 4; // CRC32 is part of the encoded block
        let block_end = HEADER_SIZE + block_length;
        let block = &buffer[HEADER_SIZE..block_end];

        if is_valid_crc(data_length, block) {
            packet.phl = Some(PhlFields {
                code_rate: header.rate,
                header_distance,
                decode_iterations: 0,
                decode_distance: 0,
            });

            self.above.read(packet, &block[..data_length])
        } else {
            let parity = &buffer[HEADER_SIZE + block_length..];
            // TODO
            const SNR: Llr = 4;
            let input = TurboDecoderInput::<{8 * (mbal::MBAL_MAX + 4)}>::new(
                header.rate,
                block,
                parity,
                first_termination,
                second_termination,
                SNR,
            );
            let result = self
                .run_decoder(data_length, &input)
                .ok_or(ReadError::PhlDecodeError)?;

            packet.phl = Some(PhlFields {
                code_rate: header.rate,
                header_distance,
                decode_iterations: result.1,
                decode_distance: Self::distance(block, &result.0),
            });

            self.above.read(packet, &result.0[..data_length])
        }
    }

    fn write<const N: usize>(
        &self,
        writer: &mut impl Writer,
        packet: &Packet<N>,
    ) -> Result<(), WriteError> {
        let fields = packet.phl.as_ref().unwrap();
        let mut block = Vec::<u8, { mbal::MBAL_MAX + 4 }>::new();

        // Write above layers to block
        self.above.write(&mut block, packet)?;

        // Compute CRC
        let mut digest = CRC.digest();
        digest.update(&[block.len() as u8]);
        digest.update(&block);
        let crc = digest.finalize();

        // Append CRC to block
        block
            .extend_from_slice(crc.to_be_bytes().as_slice())
            .map_err(|_| WriteError::Capacity)?;

        // Run Turbo encoder
        let input = block.view_bits::<Msb0>();
        debug_assert_eq!(8 * block.len(), input.len());

        let interleaver = phyinterleaver::create(input.len()).unwrap();
        let mut output = TurboEncoderOutput::new(fields.code_rate, input.len());
        self.encoder.encode(input, interleaver, &mut output);
        let result = output.get_result();

        // Prepare header by writing bits
        let mut header = BitVec::<u8, Msb0>::with_capacity(96);
        header.push(true);
        header.push(true);
        PhyCodedHeader::new(fields.code_rate, block.len() - 4).write(&mut header);

        let index = header.len();
        header.resize(header.len() + 2 * 6, false);

        let termination = header.split_at_mut(index).1;
        termination.store_be(result.termination());

        // Write header
        assert_eq!(96, header.len());
        writer.write(header.as_raw_slice())?;

        // Write systematic
        writer.write(result.systematic.as_raw_slice())?;

        // Write parity
        writer.write(result.parity.as_raw_slice())?;

        Ok(())
    }
}

fn is_valid_crc(data_length: usize, block: &[u8]) -> bool {
    assert_eq!(data_length + 4, block.len());

    let mut digest = CRC.digest();
    digest.update(&[data_length as u8]);
    digest.update(&block[..data_length]);
    let actual = digest.finalize();

    let expected = u32::from_be_bytes(block[data_length..].try_into().unwrap());

    actual == expected
}
