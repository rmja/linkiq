use alloc::boxed::Box;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use super::{Channel, Rssi, TransceiverError};

#[cfg_attr(test, automock(type Timestamp = core::time::Duration;))]
#[async_trait]
pub trait Transceiver: Send {
    type Timestamp: Send;

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

#[async_trait]
pub trait Timer: Send {
    async fn sleep_micros(&self, micros: u32);
}

#[cfg(test)]
mockall::mock! {
    pub(crate) AsyncTransceiver {
    }

    impl Transceiver for AsyncTransceiver {
        type Timestamp = core::time::Duration;

        fn init<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = Result<(), TransceiverError>> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn idle<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = ()> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn set_channel<'a, 'async_trait>(
            &'a mut self,
            channel: Channel,
        ) -> impl futures::future::Future<Output = ()> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn write<'a, 'b, 'async_trait>(
            &'a mut self,
            buffer: &'b [u8],
        ) -> impl futures::future::Future<Output = ()> + Send + 'async_trait
        where
            'a: 'async_trait,
            'b: 'async_trait,
            Self: 'async_trait;
        fn transmit<'a, 'async_trait>(
            &'a mut self,
        ) -> impl futures::future::Future<Output = Result<(), TransceiverError>> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn listen<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = ()> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn receive<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = core::time::Duration> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
        fn read<'a, 'b, 'async_trait>(
            &'a mut self,
            buffer: &'b mut [u8],
            frame_length: Option<usize>,
        ) -> impl futures::future::Future<Output = Result<usize, TransceiverError>> + Send + 'async_trait
        where
            'a: 'async_trait,
            'b: 'async_trait,
            Self: 'async_trait;
        fn get_rssi<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = Rssi> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
    }
}
