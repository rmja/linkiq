use crate::{bitreader::BitReader};

use super::{encodertermination::EncoderTermination, LlrMul, CodeRate};
use alloc::vec::Vec;
use bitvec::prelude::Msb0;
use fastfec::{convolutional::bcjr::BcjrSymbol, ratematching::Puncturer, turbo::TurboSymbol, Llr};

pub(crate) struct TurboDecoderInput {
    pub symbols: Vec<TurboSymbol>,
    pub first_termination: [BcjrSymbol; 3],
    pub second_termination: [BcjrSymbol; 3],
}

impl TurboDecoderInput {
    pub fn new(
        rate: CodeRate,
        block: &[u8],
        parity: &[u8],
        first_termination: EncoderTermination,
        second_termination: EncoderTermination,
        snr: Llr,
    ) -> Self {
        // Get the encoder puncturers for the given code rate
        let (mut first_puncturer, mut second_puncturer) = if rate == CodeRate::OneThird {
            (Puncturer::default(), Puncturer::default())
        } else {
            (Puncturer::new(2, 0b10), Puncturer::new(2, 0b01))
        };

        let mut symbols = Vec::with_capacity(8 * block.len());
        let mut systematic_reader = BitReader::<u8, Msb0>::from_slice(block);
        let mut parity_reader = BitReader::<u8, Msb0>::from_slice(parity);

        // Read systematic and first encoder parity
        for _ in 0..8 * block.len() {
            let systematic = systematic_reader.read_bit().unwrap().mul(snr);
            let first_parity = if first_puncturer.read_output() {
                parity_reader.read_bit().unwrap().mul(snr)
            } else {
                0
            };
            symbols.push(TurboSymbol::new(systematic, first_parity, 0));
        }

        // Read second encoder parity
        for i in 0..8 * block.len() {
            if second_puncturer.read_output() {
                symbols[i].second_parity = parity_reader.read_bit().unwrap().mul(snr);
            }
        }

        Self {
            symbols,
            first_termination: first_termination.get_symbols(snr),
            second_termination: second_termination.get_symbols(snr),
        }
    }
}
