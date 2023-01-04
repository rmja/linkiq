#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Channel {
    A,
    B,
    C,
    D,
}

impl Channel {
    pub fn index(&self) -> usize {
        match self {
            Channel::A => 0,
            Channel::B => 1,
            Channel::C => 2,
            Channel::D => 3,
        }
    }
}
pub(crate) const CHANNEL_COUNT: usize = 4;