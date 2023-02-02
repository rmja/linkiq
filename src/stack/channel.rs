#[derive(FromPrimitive, ToPrimitive, PartialEq, Clone, Copy, Debug)]
pub enum Channel {
    A,
    B,
    C,
    D,
}

impl Channel {
    pub const fn frequency(&self) -> u32 {
        let channel = *self as u32;
        868_450_000 + 40_000 * channel
    }
}
