use fastfec::{
    convolutional::{bcjr::BcjrSymbol, EncoderOutput},
    Llr,
};

use super::LlrMul;

#[derive(Default, Clone, Copy)]
pub(crate) struct EncoderTermination(pub usize);


impl EncoderTermination {
    pub(crate) fn append_output(&mut self, output: EncoderOutput) {
        let mut value = self.0;
        value <<= 1;
        value |= (output & 1) << 3; // Set systematic
        value |= (output & 2) >> 1; // Set parity
        self.0 = value;
    }

    pub(crate) fn get_symbols(&self, snr: Llr) -> [BcjrSymbol; 3] {
        let v = self.0;
        [
            BcjrSymbol::new(((v & 0x20) != 0).mul(snr), ((v & 0x04) != 0).mul(snr)),
            BcjrSymbol::new(((v & 0x10) != 0).mul(snr), ((v & 0x02) != 0).mul(snr)),
            BcjrSymbol::new(((v & 0x08) != 0).mul(snr), ((v & 0x01) != 0).mul(snr)),
        ]
    }
}
