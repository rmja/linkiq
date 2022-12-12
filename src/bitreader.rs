use bitvec::prelude::*;
use funty::Integral;

pub(crate) struct BitReader<'a, T: BitStore = usize, O: BitOrder = Lsb0> {
    slice: &'a BitSlice<T, O>,
    pos: usize,
}

impl<'a, T: BitStore, O: BitOrder> BitReader<'a, T, O> {
    pub fn from_slice(slice: &'a [T]) -> Self {
        Self {
            slice: BitSlice::from_slice(slice),
            pos: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        let value = self.slice.get(self.pos);
        self.pos += 1;
        value.map(|v| *v)
    }
}

pub trait BitField {
    fn read_bits<I: Integral>(&mut self, count: usize) -> Option<I>;
}

impl<T: BitStore> BitField for BitReader<'_, T, Lsb0> {
    fn read_bits<I: Integral>(&mut self, count: usize) -> Option<I> {
        let bits = self.slice.get(self.pos..self.pos + count)?;
        let value = bits.load_le::<I>();
        self.pos += count;
        Some(value)
    }
}

impl<T: BitStore> BitField for BitReader<'_, T, Msb0> {
    fn read_bits<I: Integral>(&mut self, count: usize) -> Option<I> {
        let bits = self.slice.get(self.pos..self.pos + count)?;
        let value = bits.load_be::<I>();
        self.pos += count;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_few_bits_lsb0() {
        // Given
        let buffer = [0x1A];
        let mut reader: BitReader<u8, Lsb0> = BitReader::from_slice(&buffer);

        // When
        assert_eq!(0b10u32, reader.read_bits(2).unwrap());
        assert_eq!(0b110u32, reader.read_bits(3).unwrap());
    }

    #[test]
    fn can_read_few_bits_msb0() {
        // Given
        let buffer = [0xB0];
        let mut reader: BitReader<u8, Msb0> = BitReader::from_slice(&buffer);

        // When
        assert_eq!(0b10u32, reader.read_bits(2).unwrap());
        assert_eq!(0b110u32, reader.read_bits(3).unwrap());
    }

    #[test]
    fn can_read_bits_in_multiple_bytes_lsb0() {
        // Given
        let buffer = [0xFA, 0xCB, 0xD1];
        let mut reader: BitReader<u8, Lsb0> = BitReader::from_slice(&buffer);

        // When
        assert_eq!(0b10u32, reader.read_bits(2).unwrap());
        assert_eq!(0b110u32, reader.read_bits(3).unwrap());
        assert_eq!(0b1101000111001011111u32, reader.read_bits(19).unwrap());
    }

    #[test]
    fn can_read_bits_in_multiple_bytes_msb0() {
        // Given
        let buffer = [0xB6, 0x8E, 0x5F];
        let mut reader: BitReader<u8, Msb0> = BitReader::from_slice(&buffer);

        // When
        assert_eq!(0b10u32, reader.read_bits(2).unwrap());
        assert_eq!(0b110u32, reader.read_bits(3).unwrap());
        assert_eq!(0b1101000111001011111u32, reader.read_bits(19).unwrap());
    }
}
