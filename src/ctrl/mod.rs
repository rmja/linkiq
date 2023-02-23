mod controller;
mod noicefloor;
pub mod traits;

pub use controller::Controller;
use embassy_time::Instant;

use crate::stack::{phl, Rssi};

pub struct Frame {
    pub timestamp: Instant,
    pub rssi: Option<Rssi>,
    buffer: [u8; phl::MAX_FRAME_LENGTH],
    received: usize,
    len: Option<usize>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            timestamp: Instant::now(),
            rssi: None,
            buffer: [0; phl::MAX_FRAME_LENGTH],
            received: 0,
            len: None,
        }
    }
}

#[allow(clippy::len_without_is_empty)]
impl Frame {
    pub fn len(&self) -> usize {
        self.len.unwrap()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buffer[0..self.len.unwrap()]
    }
}
