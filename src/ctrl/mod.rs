pub mod adapters;
mod controller;
mod delay;
mod noicefloor;

pub type Rssi = i8;
pub use controller::Controller;

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

pub const CHANNEL_COUNT: usize = 4;
