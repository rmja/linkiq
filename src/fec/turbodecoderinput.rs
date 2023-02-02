use crate::bitreader::BitReader;

use super::{encodertermination::EncoderTermination, CodeRate, LlrMul};
use heapless::Vec;
use bitvec::prelude::Msb0;
use fastfec::{convolutional::bcjr::BcjrSymbol, ratematching::Puncturer, turbo::TurboSymbol, Llr};

pub(crate) struct TurboDecoderInput<const N: usize> {
    pub symbols: Vec<TurboSymbol, N>,
    pub first_termination: [BcjrSymbol; 3],
    pub second_termination: [BcjrSymbol; 3],
}

impl<const N: usize> TurboDecoderInput<N> {
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

        let mut symbols = Vec::new();
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
            symbols.push(TurboSymbol::new(systematic, first_parity, 0)).map_err(|_| ()).unwrap();
        }

        // Read second encoder parity
        for symbol in symbols.iter_mut().take(8 * block.len()) {
            if second_puncturer.read_output() {
                symbol.second_parity = parity_reader.read_bit().unwrap().mul(snr);
            }
        }

        Self {
            symbols,
            first_termination: first_termination.get_symbols(snr),
            second_termination: second_termination.get_symbols(snr),
        }
    }
}
