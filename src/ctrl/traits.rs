use core::fmt::Debug;

use embassy_time::Instant;
#[cfg(test)]
use mockall::automock;

use crate::stack::{Channel, Rssi};

#[cfg_attr(test, automock(type RxToken = stubs::RxTokenStub; type Error = ();))]
pub trait Transceiver {
    type RxToken: RxToken;
    type Error: Debug;

    /// Setup the transceiver and enter idle state.
    async fn init(&mut self) -> Result<(), Self::Error>;

    /// Set the current channel.
    /// This may be called when idle or when listening.
    async fn set_channel(&mut self, channel: Channel) -> Result<(), Self::Error>;

    /// Prepare bytes for transmission.
    async fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error>;

    /// Transmit already prepared bytes and return to idle state.
    async fn transmit(&mut self) -> Result<(), Self::Error>;

    /// Start the receiver.
    async fn listen(&mut self) -> Result<(), Self::Error>;

    /// Get the current rssi.
    async fn get_rssi(&mut self) -> Result<Rssi, Self::Error>;

    /// Try and receive a frame.
    /// The future will complete when `min_frame_length` frame bytes are received.
    /// The receiver will continue to receive the frame until either `accept` is invoked or `receive` are re-invoked.
    async fn receive(&mut self, min_frame_length: usize) -> Result<Self::RxToken, Self::Error>;

    /// Read bytes for the frame currently being received.
    async fn read<'a>(
        &'a mut self,
        token: &mut Self::RxToken,
        buffer: &mut [u8],
    ) -> Result<usize, Self::Error>;

    /// Notify the receiver about the final frame length for the current receive.
    /// The receiver shoud re-start when this frame length has been received.
    async fn accept(
        &mut self,
        token: &mut Self::RxToken,
        frame_length: usize,
    ) -> Result<(), Self::Error>;

    /// Enter idle state.
    async fn idle(&mut self) -> Result<(), Self::Error>;
}

pub trait RxToken {
    /// Get the start-of-frame timestamp
    fn timestamp(&self) -> Instant;
}

#[cfg(test)]
pub mod stubs {
    use embassy_time::Instant;

    use super::RxToken;

    pub struct RxTokenStub(pub Instant);

    impl RxToken for RxTokenStub {
        fn timestamp(&self) -> Instant {
            self.0
        }
    }
}
