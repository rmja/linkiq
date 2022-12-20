#[cfg(test)]
use mockall::automock;

use super::{Channel, Rssi, TransceiverError};

#[cfg_attr(test, automock(type Timestamp = core::time::Duration;))]
pub trait Transceiver {
    type Timestamp;

    /// Setup the transceiver and enter idle state.
    async fn init(&mut self) -> Result<(), TransceiverError>;

    /// Set the current channel.
    /// This may be called when idle or when listening.
    async fn set_channel(&mut self, channel: Channel);

    /// Prepare bytes for transmission.
    async fn write(&mut self, buffer: &[u8]);

    /// Transmit already prepared bytes and return to idle state.
    async fn transmit(&mut self) -> Result<(), TransceiverError>;

    /// Start receiver.
    async fn listen(&mut self);

    /// Try and receive a packet.
    /// The future will complete when a packet is detected.
    async fn receive(&mut self) -> Self::Timestamp;

    /// Read bytes for the packet currently being received.
    async fn read<'a>(
        &'a mut self,
        buffer: &mut [u8],
        frame_length: Option<usize>,
    ) -> Result<usize, TransceiverError>;

    /// Get the current rssi.
    async fn get_rssi(&mut self) -> Rssi;

    /// Enter idle state.
    async fn idle(&mut self);
}

pub trait Delay {
    async fn delay_micros(&self, micros: u32);
}

#[cfg(test)]
mockall::mock! {
    pub(crate) AsyncTransceiver {
    }

    impl Transceiver for AsyncTransceiver {
        type Timestamp = core::time::Duration;

        fn init(&mut self) -> impl futures::future::Future<Output = Result<(), TransceiverError>>;
        fn idle(&mut self) -> impl futures::future::Future<Output = ()>;
        fn set_channel(&mut self, channel: Channel) -> impl futures::future::Future<Output = ()>;
        fn write(&mut self, buffer: &[u8]) -> impl futures::future::Future<Output = ()>;
        fn transmit(&mut self) -> impl futures::future::Future<Output = Result<(), TransceiverError>>;
        fn listen(&mut self) -> impl futures::future::Future<Output = ()>;
        fn receive(&mut self) -> impl futures::future::Future<Output = core::time::Duration>;
        fn read(&mut self, buffer: &mut [u8], frame_length: Option<usize>) -> impl futures::future::Future<Output = Result<usize, TransceiverError>>;
        fn get_rssi(&mut self) -> impl futures::future::Future<Output = Rssi>;
    }
}
