use fastfec::{
    convolutional::EncoderOutput, ratematching::Puncturer, turbo::TurboEncoderOutputWriter,
};

use super::{CodeRate, EncoderTermination};
use bitvec::prelude::*;

pub(crate) struct TurboEncoderOutput {
    first_puncturer: Puncturer,
    second_puncturer: Puncturer,
    systematic: BitVec<u8, Msb0>,
    first_parity: BitVec<u8, Msb0>,
    second_parity: BitVec<u8>,
    first_termination: EncoderTermination,
    second_termination: EncoderTermination,
    written: usize,
}

pub struct EncodeResult {
    pub systematic: BitVec<u8, Msb0>,
    pub parity: BitVec<u8, Msb0>,
    first_termination: EncoderTermination,
    second_termination: EncoderTermination,
}

impl TurboEncoderOutput {
    pub fn new(rate: CodeRate, block_size: usize) -> Self {
        let (first_puncturer, second_puncturer) = match rate {
            CodeRate::OneThird => (Puncturer::default(), Puncturer::default()),
            CodeRate::OneHalf => (Puncturer::new(2, 0b10), Puncturer::new(2, 0b01)),
        };

        let parity_bits = match rate {
            CodeRate::OneThird => block_size,
            CodeRate::OneHalf => block_size / 2,
        };

        // The written parity bits for the second encoder is never exposed
        // but appended to the first parity writer - this is the reason why
        // no specific bit order is specified for that writer. Basically
        // the bit order for that writer must match the reader used during completion.

        Self {
            first_puncturer,
            second_puncturer,
            systematic: BitVec::with_capacity(block_size),
            first_parity: BitVec::with_capacity(2 * parity_bits),
            second_parity: BitVec::with_capacity(parity_bits),
            first_termination: EncoderTermination::default(),
            second_termination: EncoderTermination::default(),
            written: 0,
        }
    }

    pub fn get_result(mut self) -> EncodeResult {
        assert_eq!(0, (self.first_parity.len() + self.second_parity.len()) % 8);

        if self.second_parity.any() {
            for parity in self.second_parity {
                self.first_parity.push(parity);
            }
        }

        EncodeResult {
            systematic: self.systematic,
            parity: self.first_parity,
            first_termination: self.first_termination,
            second_termination: self.second_termination,
        }
    }
}

impl EncodeResult {
    pub fn termination(&self) -> usize {
        (self.first_termination.0 << 6) + self.second_termination.0
    }
}

impl TurboEncoderOutputWriter for TurboEncoderOutput {
    fn write_output(&mut self, output: EncoderOutput) {
        self.systematic.push((output & 1) != 0);
        self.written += 1;

        if self.first_puncturer.read_output() {
            self.first_parity.push(((output >> 1) & 1) != 0);
            self.written += 1;
        }

        if self.second_puncturer.read_output() {
            self.second_parity.push(((output >> 2) & 1) != 0);
            self.written += 1;
        }
    }

    fn write_termination_output(&mut self, encoder_index: usize, output: EncoderOutput) {
        if encoder_index == 0 {
            self.first_termination.append_output(output);
        } else {
            self.second_termination.append_output(output);
        }
        self.written += 2;
    }
}

#[cfg(test)]
mod tests {
    use fastfec::{
        catalog,
        convolutional::{ConvolutionalCodeExt, ConvolutionalEncoder},
        interleaver::Interleaver,
        turbo::TurboEncoder,
    };

    use crate::{interleaver, stack::phl::CRC};
    use bitvec::prelude::*;

    use super::*;

    #[test]
    fn can_encode_constituent_encoder_test_vectors() {
        can_encode_constituent_encoder_test_vector_case(
            &[0x5c, 0x06, 0x8d, 0xa5, 0x61, 0x83, 0xdb, 0x13],
            &[0x6f, 0x93, 0x89, 0x94, 0xd3, 0xf0, 0x53, 0x40],
            0b110010,
        );
        can_encode_constituent_encoder_test_vector_case(
            &[
                0xd7, 0xf9, 0xb6, 0xcc, 0x65, 0xdd, 0x8b, 0x20, 0x79, 0xb3, 0x96, 0xf9, 0x0a, 0x99,
                0xed, 0x96, 0x3d, 0xf6, 0x9c, 0xee,
            ],
            &[
                0x90, 0xd5, 0x88, 0x66, 0xa6, 0xee, 0x8d, 0x3c, 0xec, 0xfc, 0x9a, 0xa7, 0xb5, 0x75,
                0xe2, 0x23, 0x59, 0xf1, 0x73, 0x92,
            ],
            0b000000,
        );
        can_encode_constituent_encoder_test_vector_case(
            &[
                0xe1, 0xbb, 0xb5, 0x8d, 0xea, 0x19, 0x06, 0xd3, 0xe4, 0xa0, 0xf8, 0xcb, 0x0f, 0xc4,
                0x5e, 0x7e, 0xb0, 0x2f, 0xec, 0x3d, 0x16, 0x8a, 0xb5, 0x76, 0xb4, 0x3c, 0x92, 0x32,
                0x3a, 0x5e, 0x0d, 0x2e, 0x56, 0xdf, 0xd2, 0x86, 0xfd, 0x72, 0xed, 0x76, 0x8e, 0x82,
                0x6d, 0x52, 0x0c, 0x3e, 0x89, 0x1d, 0x5f, 0x80,
            ],
            &[
                0xb6, 0x16, 0x41, 0xa7, 0xbb, 0xdb, 0x04, 0xcb, 0xec, 0xce, 0xfa, 0x63, 0x56, 0xa2,
                0x30, 0xb5, 0xfe, 0x6a, 0x28, 0x92, 0xd1, 0x69, 0xf8, 0x71, 0x40, 0x93, 0xc1, 0x0f,
                0x5c, 0x89, 0x9e, 0x45, 0x4d, 0xb1, 0x96, 0xf6, 0x19, 0x04, 0x5b, 0xe6, 0x40, 0x14,
                0xa9, 0x38, 0x54, 0x29, 0x6b, 0x17, 0x6d, 0x4b,
            ],
            0b111001,
        );
        can_encode_constituent_encoder_test_vector_case(
            &[
                0x19, 0xc7, 0x49, 0x35, 0xae, 0x4f, 0x06, 0xac, 0x37, 0xaa, 0xcc, 0x3e, 0x13, 0x9b,
                0x3a, 0x86, 0x52, 0x2e, 0x56, 0x1a, 0xbd, 0x96, 0xcd, 0x94, 0xf9, 0xc1, 0x3e, 0x75,
                0xc6, 0x1e, 0xc4, 0xd9, 0x80, 0x80, 0xfa, 0x24, 0xc7, 0xaf, 0x57, 0x22, 0x36, 0x99,
                0x5f, 0xf6, 0xc6, 0xae, 0xfa, 0x85, 0x12, 0x4a, 0x7f, 0xd2, 0x9c, 0xa2, 0xcd, 0x80,
                0xae, 0xa4, 0xae, 0x3c, 0xb0, 0xc7, 0xa3, 0xdb, 0xc0, 0xea, 0xa1, 0x55, 0x15, 0x3d,
                0xb9, 0xe8, 0x52, 0x2b, 0xbf, 0x87, 0xe2, 0xd6, 0xf7, 0x37, 0x1c, 0xf7, 0xd2, 0x90,
                0xac, 0xc8, 0x3a, 0xe9, 0xdc, 0x6e, 0xbc, 0xcf, 0xf8, 0x8e, 0x54, 0xc9, 0x48, 0x24,
                0x9f, 0x84,
            ],
            &[
                0x10, 0x19, 0x77, 0x9d, 0x20, 0x96, 0x2a, 0xc6, 0xc2, 0x55, 0x14, 0x29, 0x8b, 0xbd,
                0x2e, 0xf6, 0xdd, 0xa0, 0x86, 0xab, 0x12, 0x23, 0xf0, 0xc5, 0xd5, 0xd6, 0xe2, 0x5d,
                0x6a, 0x15, 0xa2, 0xc7, 0x4b, 0x65, 0x6e, 0x3b, 0xa0, 0xb6, 0x4c, 0xf4, 0xc3, 0x90,
                0xfa, 0x3a, 0x8f, 0x0e, 0x1c, 0x11, 0x41, 0x5b, 0x9a, 0x01, 0xca, 0x91, 0xac, 0x65,
                0x0e, 0x70, 0x7c, 0x76, 0x35, 0x45, 0x75, 0x98, 0x40, 0xbb, 0x04, 0x4f, 0x18, 0x05,
                0x3b, 0xb8, 0x64, 0xd4, 0xa8, 0x85, 0x0d, 0x06, 0x15, 0x9e, 0x38, 0x15, 0x2f, 0x9e,
                0x9a, 0xd8, 0x2e, 0xb9, 0x78, 0xf7, 0x13, 0xaf, 0xfa, 0x1c, 0x3c, 0x85, 0x2a, 0xde,
                0x03, 0x10,
            ],
            0b010110,
        );
    }

    fn can_encode_constituent_encoder_test_vector_case(
        input: &[u8],
        parity: &[u8],
        termination: usize,
    ) {
        // Given
        let mut encoder = ConvolutionalEncoder::<catalog::UMTS>::default();
        let mut output = TurboEncoderOutput::new(CodeRate::OneThird, 10);
        output.second_puncturer = Puncturer::new(1, 1); // Puncture the entire second encoder

        // When
        for bit in input.view_bits::<Msb0>() {
            output.write_output(encoder.get_output(*bit));
        }
        for _ in 0..catalog::UMTS::mem() {
            output.write_termination_output(0, encoder.get_termination_output());
        }
        let written = output.written;
        let result = output.get_result();

        // Then
        assert_eq!(8 * (input.len() + parity.len()) + 6, written);
        assert_eq!(input, result.systematic.as_raw_slice());
        assert_eq!(parity, result.parity.as_raw_slice());
        assert_eq!(termination, result.first_termination.0);
    }

    #[test]
    #[rustfmt::skip]
    fn can_encode_turbo_test_vectors() {
        can_encode_turbo_test_vector_case(
            CodeRate::OneHalf,
            &[
                0x01, 0x37, 0x2c, 0x34, 0x12, 0x34, 0x12, 0x1b, 0x16, 0x60, 0x16, 0x61, 0x7a, 0x01, 0x00, 0x20, 0x05, 0x19, 0x32, 0x29, 0xbc, 0xe6, 0x4d, 0x65, 0x1f, 0x1d, 0xed, 0x42, 0x68, 0x73, 0x03, 0xb2, 0x9a, 0xf6, 0xa6, 0x80, 0x53, 0x36, 0x08, 0x4a, 0x0c, 0xc4, 0xb4, 0xb9, 0x23, 0x71, 0xa3, 0xca, 0xb9,
            ],
            &[
                0x09, 0xd3, 0x5f, 0xe3, 0xfb, 0x1e, 0x3b, 0x5a, 0x49, 0xa7, 0x1a, 0x34, 0x24, 0x39, 0x87, 0x30, 0x07, 0xbd, 0x8e, 0x41, 0x78, 0x77, 0x7a, 0x82, 0x7c, 0x72, 0x3b, 0x81, 0x49, 0xbe, 0x18, 0x74, 0x50, 0x08, 0xdb, 0x6e, 0x1f, 0x01, 0x33, 0x14, 0x96, 0x79, 0xac, 0x67, 0xa4, 0xe3, 0xfa, 0x08, 0x38, 0x42, 0x99, 0x18, 0x31,
            ],
            0b100100110010,
        );
        can_encode_turbo_test_vector_case(
            CodeRate::OneHalf,
            &[
                0x00, 0x2d, 0x2c, 0x89, 0x63, 0x00, 0x71, 0x34, 0x04, 0x40, 0x90, 0x01, 0x90, 0x0f, 0x00, 0x2c, 0x25, 0x45, 0x42, 0x01, 0x00, 0xc9, 0xfe, 0x78, 0x01, 0x18, 0xb7, 0xe8, 0x31, 0x7a, 0x12, 0x18, 0x40, 0x07, 0x10, 0x35, 0xcd, 0x99, 0x1d, 0xe9, 0xc5, 0x3c, 0x5d, 0xcc, 0x31, 0x05, 0x01, 0x87, 0x82, 0xd7, 0x2d, 0x1c, 0xdb, 0x39, 0xc5, 0xdb, 0x1b, 0x7c, 0x21, 0x82, 0x05, 0x7e, 0x19, 0x35, 0xd7, 0x73, 0xaf, 0xdA, 0xAa, 0x24, 0xf4, 0xfa, 0x17, 0x38, 0xe2, 0xbd, 0x8b, 0x13, 0xf3, 0xfc, 0x77, 0xa3, 0x2b, 0x68, 0xf1, 0xd1, 0x2e, 0x73, 0x66, 0xfe, 0xc6, 0x1d, 0x69, 0xd7, 0xe7, 0x81, 0xc2, 0x88, 0x65,
            ],
            &[
                0x04, 0x85, 0xf9, 0x43, 0x6d, 0x25, 0xed, 0xcf, 0xeb, 0xc7, 0xed, 0x35, 0x22, 0x39, 0x7f, 0x29, 0x4b, 0x44, 0x01, 0xfb, 0xab, 0xeb, 0xc6, 0x73, 0xbd, 0xda, 0xf1, 0xfa, 0xaa, 0x8b, 0x49, 0xb4, 0x10, 0x81, 0x7c, 0x15, 0xdf, 0x0d, 0xfe, 0x1b, 0xaa, 0x11, 0xcf, 0x05, 0xcf, 0x5f, 0x64, 0x31, 0x15, 0x34, 0xec, 0x65, 0x5e, 0x0c, 0x96, 0x6c, 0x93, 0xc8, 0x54, 0xbe, 0x53, 0x76, 0xdb, 0xd1, 0x85, 0x6d, 0x15, 0x9d, 0xc7, 0x7a, 0x94, 0xcc, 0xda, 0xe1, 0xe6, 0x9c, 0x74, 0x47, 0x63, 0xb2, 0xbd, 0xaa, 0xff, 0xf5, 0xbc, 0xd5, 0xec, 0xda, 0x2d, 0xba, 0xd8, 0xdb, 0x75, 0x60, 0x1c, 0x39, 0xf0, 0x4d, 0x6f, 0xd4, 0x33, 0x88, 0xfb,
            ],
            0b111001100100,
        );
        can_encode_turbo_test_vector_case(
            CodeRate::OneThird,
            &[
                0x00, 0x2d, 0x2c, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00, 0x40, 0xc0, 0xbe, 0x7a, 0x22, 0xab, 0xff, 0x2a, 0x10, 0x01, 0xff, 0xee, 0xdd, 0xcc, 0xe6, 0x0d, 0x1f, 0x01, 0xda, 0xb0, 0xe2, 0x83, 0x2a, 0x65, 0x18, 0x00, 0x3e, 0xe7, 0x42, 0x4e, 0xe8, 0x65, 0xdf, 0xee, 0x22, 0x53, 0xc0, 0xd6, 0x35, 0xee, 0xe6, 0x69, 0x77, 0xf4, 0x20, 0x4b, 0xa9, 0x3f, 0xd3, 0x44, 0x1c,
            ],
            &[
                0x00, 0x35, 0xd1, 0xc8, 0x5e, 0x90, 0xbf, 0x04, 0x5c, 0xc0, 0x8b, 0x4c, 0x0b, 0xf4, 0x26, 0x34, 0xfb, 0xd5, 0xca, 0xd1, 0xbc, 0xc0, 0xad, 0xc1, 0x9e, 0x66, 0x2f, 0x20, 0x8c, 0x0d, 0x67, 0xd5, 0xd4, 0x86, 0x5c, 0x90, 0x57, 0x26, 0x72, 0xb8, 0x43, 0x26, 0x92, 0x3f, 0x17, 0x6e, 0xcd, 0x0a, 0x77, 0x78, 0xae, 0x95, 0x17, 0xab, 0xe3, 0x9c, 0x06, 0xb9, 0xc7, 0x81, 0x7a, 0x97, 0x33, 0x10,
                0x22, 0x04, 0xa5, 0xe6, 0x78, 0x32, 0x50, 0xa0, 0xa6, 0x81, 0x19, 0x84, 0x92, 0x01, 0xc8, 0x02, 0xf2, 0xd9, 0x48, 0x6b, 0x44, 0xbb, 0xa8, 0xce, 0x91, 0xc6, 0x78, 0xa7, 0x6a, 0x7a, 0xbc, 0xe1, 0xf9, 0xf3, 0xdd, 0x29, 0xcf, 0xc9, 0xb1, 0x07, 0x5d, 0x88, 0x5b, 0x3d, 0x98, 0x83, 0x26, 0x5f, 0x8a, 0x70, 0xbd, 0xc7, 0x18, 0xc4, 0xbb, 0x22, 0x00, 0x90, 0xed, 0x2d, 0xa6, 0x3f, 0xad, 0x02,
            ],
            0b101111010110,
        );
        can_encode_turbo_test_vector_case(
            CodeRate::OneThird,
            &[
                0x00, 0x2d, 0x2c, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00, 0x40, 0xc0, 0xbe, 0x7a, 0x2a, 0x00, 0x00, 0x00, 0x0d, 0xfd, 0x09, 0xe3, 0x0a, 0x03, 0x01, 0x41, 0x7c, 0x03, 0x34, 0x53, 0x44, 0x0d, 0x42, 0x66, 0x1b, 0x01, 0x42, 0xfb, 0x1a, 0x42, 0x02, 0x44, 0x6d, 0x1e, 0x29, 0xab, 0x23,
            ],
            &[
                0x00, 0x35, 0xd1, 0xc8, 0x5e, 0x90, 0xbf, 0x04, 0x5c, 0xc0, 0x8b, 0x4c, 0x0b, 0xfb, 0xcb, 0x97, 0x2e, 0x55, 0x45, 0xeb, 0xb5, 0x0c, 0xe7, 0xe4, 0x56, 0x98, 0xbb, 0x79, 0xae, 0xe9, 0xc2, 0xb1, 0x6f, 0xd8, 0xca, 0x08, 0xf8, 0xd9, 0x54, 0xe6, 0x7e, 0xa9, 0x49, 0xa5, 0x9f, 0xdb, 0x0b, 0xc0, 0xbd, 0x3d,
                0x2d, 0xc5, 0x9d, 0xdb, 0x6f, 0x84, 0x71, 0x7a, 0x28, 0x77, 0xeb, 0x11, 0x3e, 0xf4, 0x71, 0xc7, 0x6c, 0xf7, 0x19, 0x0b, 0xd0, 0xf2, 0x89, 0x37, 0x1f, 0xa0, 0x3c, 0xc9, 0x50, 0x4d, 0x17, 0x09, 0xf9, 0xb4, 0x96, 0x31, 0xd6, 0x2c, 0xd5, 0x34, 0xf5, 0x66, 0x63, 0xd3, 0x33, 0xd2, 0xfb, 0xee, 0x5f, 0x76,
            ],
            0b111001010110,
        );
    }

    fn can_encode_turbo_test_vector_case(
        rate: CodeRate,
        input: &[u8],
        parity: &[u8],
        termination: usize,
    ) {
        // Given
        let mut input = input.to_vec();
        append_checksum(&mut input);

        let input_bits = input.view_bits::<Msb0>();
        let encoder = TurboEncoder::<catalog::UMTS>::new();
        let interleaver = interleaver::new(input_bits.len()).unwrap();
        let mut output = TurboEncoderOutput::new(rate, interleaver.len());

        // When
        encoder.encode(input_bits, &interleaver, &mut output);
        let written = output.written;
        let result = output.get_result();

        // Then
        assert_eq!(8 * (input.len() + parity.len()) + 12, written);
        assert_eq!(input, result.systematic.as_raw_slice());
        assert_eq!(parity, result.parity.as_raw_slice());
        assert_eq!(termination, result.termination());
    }

    fn append_checksum(input: &mut Vec<u8>) {
        let mut digest = CRC.digest();
        digest.update(&[input.len() as u8]);
        digest.update(input);
        let crc = digest.finalize();

        input.extend_from_slice(&crc.to_be_bytes());
    }
}
