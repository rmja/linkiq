use core::time::Duration;

use alloc::boxed::Box;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use super::{Channel, ReceiveError, Rssi, TransmitError};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Transceiver {
    /// Enter idle state.
    async fn idle(&mut self);

    /// Set the current channel.
    /// This may be called when idle or when listening.
    async fn set_channel(&mut self, channel: Channel);

    /// Prepare bytes for transmission.
    async fn write(&mut self, buffer: &[u8]);

    /// Transmit already prepared bytes.
    async fn transmit(&mut self) -> Result<(), TransmitError>;

    /// Start receiver.
    async fn listen(&mut self);

    /// Try and receive a packet
    /// The future will complete when a packet is detected.
    async fn receive(&mut self) -> Duration;
    async fn read<'a>(
        &'a mut self,
        buffer: &mut [u8],
        frame_length: Option<usize>,
    ) -> Result<usize, ReceiveError>;

    async fn read_rssi(&self) -> Rssi;
}

#[cfg(test)]
mockall::mock! {
    pub AsyncTransceiver {
    }

    impl Transceiver for AsyncTransceiver {
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
        ) -> impl futures::future::Future<Output = Result<(), TransmitError>> + Send + 'async_trait
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
        ) -> impl futures::future::Future<Output = Result<usize, ReceiveError>> + Send + 'async_trait
        where
            'a: 'async_trait,
            'b: 'async_trait,
            Self: 'async_trait;
        fn read_rssi<'a, 'async_trait>(&'a self) -> impl futures::future::Future<Output = Rssi> + Send + 'async_trait
        where
            'a: 'async_trait,
            Self: 'async_trait;
    }
}
