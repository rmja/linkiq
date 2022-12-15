use core::time::Duration;

use alloc::boxed::Box;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use super::{Channel, Rssi};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transceiver {
    /// Setup the transceiver 
    async fn init(&mut self) -> Result<(), TransceiverError>;

    /// Enter idle state.
    async fn idle(&mut self);

    /// Set the current channel.
    /// This may be called when idle or when listening.
    async fn set_channel(&mut self, channel: Channel);

    /// Prepare bytes for transmission.
    async fn write(&mut self, buffer: &[u8]);

    /// Transmit already prepared bytes.
    async fn transmit(&mut self) -> Result<(), TransceiverError>;

    /// Start receiver.
    async fn listen(&mut self);

    /// Try and receive a packet.
    /// The future will complete when a packet is detected.
    async fn receive(&mut self) -> Duration;

    /// Read bytes for the packet currently being received.
    async fn read<'a>(
        &'a mut self,
        buffer: &mut [u8],
        frame_length: Option<usize>,
    ) -> Result<usize, TransceiverError>;

    /// Get the current rssi.
    async fn get_rssi(&self) -> Rssi;
}

#[derive(Debug)]
pub enum TransceiverError {
    /// The transceiver was not found to be present
    NotPresent,
    Timeout,
}

#[cfg(test)]
mockall::mock! {
    pub(crate) AsyncTransceiver {
    }

    impl Transceiver for AsyncTransceiver {
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
        fn receive<'a, 'async_trait>(&'a mut self) -> impl futures::future::Future<Output = Duration> + Send + 'async_trait
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
        fn get_rssi<'a, 'async_trait>(&'a self) -> impl futures::future::Future<Output = Rssi> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
    }
}
