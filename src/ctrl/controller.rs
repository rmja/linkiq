use alloc::boxed::Box;
use futures::{
    future::{self, Either},
    pin_mut, Stream,
};
use futures_async_stream::stream;

use crate::stack::phl;

use super::{noicefloor::NoiceFloor, traits, Channel, Rssi, TransceiverError, CHANNEL_COUNT};

/// LinkIQ Transceiver Controller
pub struct Controller<Transceiver: traits::Transceiver, Delay: traits::Delay> {
    transceiver: Transceiver,
    delay: Delay,
    listening: bool,
    current_channel: Channel,
    min_snr: i8,
    noisefloor: [NoiceFloor; CHANNEL_COUNT],
}

pub struct Frame<Timestamp> {
    pub timestamp: Timestamp,
    pub rssi: Rssi,
    buffer: [u8; phl::MAX_FRAME_LENGTH],
    received: usize,
    length: Option<usize>,
}

impl<Timestamp> Frame<Timestamp> {
    const fn new(timestamp: Timestamp, rssi: Rssi) -> Self {
        Self {
            timestamp,
            rssi,
            buffer: [0; phl::MAX_FRAME_LENGTH],
            received: 0,
            length: None,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buffer[0..self.length.unwrap()]
    }
}

impl<Transceiver, Delay> Controller<Transceiver, Delay>
where
    Transceiver: traits::Transceiver,
    Delay: traits::Delay,
{
    /// Create a new controller
    pub const fn new(transceiver: Transceiver, delay: Delay) -> Self {
        Self {
            transceiver,
            delay,
            listening: false,
            current_channel: Channel::A,
            min_snr: 4,
            noisefloor: [
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
            ],
        }
    }

    /// Prepare bytes for transmission.
    /// All bytes for the transmission must be written before the transmission is started.
    pub async fn write(&mut self, buffer: &[u8]) {
        assert!(!self.listening);
        self.transceiver.write(buffer).await;
    }

    /// Transmit pre-written bytes.
    /// The transmitter enters idle after the transmission completes.
    pub async fn transmit(&mut self, channel: Channel) -> Result<(), TransceiverError> {
        assert!(!self.listening);
        self.current_channel = channel;
        self.transceiver.set_channel(channel).await;
        self.transceiver.transmit().await
    }

    /// Start and run receiver.
    /// Note that the receiver is _not_ stopped when the stream is dropped, so idle() must be called manually after the stream is dropped.
    pub async fn receive<'a>(
        &'a mut self,
    ) -> impl Stream<Item = Frame<Transceiver::Timestamp>> + 'a {
        assert!(!self.listening);
        self.transceiver.set_channel(self.current_channel).await;
        self.transceiver.listen().await;
        self.listening = true;

        self.receive_stream()
    }

    #[stream(boxed_local, item = Frame<Transceiver::Timestamp>)]
    async fn receive_stream(&mut self) {
        loop {
            let rssi = self.transceiver.get_rssi().await;
            let noicefloor = &mut self.noisefloor[self.current_channel.index()];
            let mut frame = if rssi > noicefloor.value() + self.min_snr {
                let frame = {
                    let receive_future = self.transceiver.receive();
                    let timeout_future = self.delay.delay_micros(12_000);
                    pin_mut!(receive_future);
                    pin_mut!(timeout_future);

                    if let Either::Left((timestamp, _)) =
                        future::select(receive_future, timeout_future).await
                    {
                        Some(Frame::new(timestamp, rssi))
                    } else {
                        // Timeout
                        None
                    }
                };

                if let Some(frame) = frame {
                    frame
                } else {
                    // Timeout
                    self.set_next_channel().await;
                    continue;
                }
            } else {
                noicefloor.add(rssi);

                self.set_next_channel().await;
                continue;
            };

            // Frame was detected - read all frame bytes...
            loop {
                let mut buffer = &mut frame.buffer[frame.received..];
                let received = self.transceiver.read(&mut buffer, frame.length).await;

                if let Ok(received) = received {
                    frame.received += received;

                    if let Some(framelen) = frame.length {
                        if frame.received >= framelen {
                            // Frame is fully received
                            yield frame;
                            self.set_next_channel().await;
                            break;
                        }
                    } else {
                        // Try and derive the frame length
                        frame.length = phl::get_frame_length(&frame.buffer[..frame.received]).ok();
                    }
                } else {
                    // Error during read - restart receiver
                    self.transceiver.idle().await;
                    self.transceiver.listen().await;
                    break;
                }
            }
        }
    }

    async fn set_next_channel(&mut self) {
        self.current_channel = match self.current_channel {
            Channel::A => Channel::B,
            Channel::B => Channel::C,
            Channel::C => Channel::D,
            Channel::D => Channel::A,
        };

        // self.transceiver.set_channel(self.current_channel).await;
    }

    // Stop the receiver.
    pub async fn idle(&mut self) {
        self.transceiver.idle().await;
        self.listening = false;
    }

    /// Release the transceiver
    pub fn release(self) -> Transceiver {
        self.transceiver
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use futures::{future::Either, pin_mut, prelude::*};
    use mockall::{predicate::eq, Sequence};
    use tokio::time;

    use crate::ctrl::{
        adapters::tokio::TokioDelay,
        traits::{MockAsyncTransceiver, MockTransceiver},
    };

    use super::*;

    #[tokio::test]
    async fn can_transmit() {
        // Given
        let mut seq = Sequence::new();
        let mut transceiver = MockTransceiver::new();
        transceiver
            .expect_write()
            .withf(|buf: &[u8]| buf == &[0x01, 0x23])
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        transceiver
            .expect_write()
            .withf(|buf: &[u8]| buf == &[0x45, 0x67])
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        transceiver
            .expect_set_channel()
            .with(eq(Channel::C))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        transceiver
            .expect_transmit()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| Ok(()));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        ctrl.write(&[0x01, 0x23]).await;
        ctrl.write(&[0x45, 0x67]).await;
        ctrl.transmit(Channel::C).await.unwrap();

        // Then
        assert!(!ctrl.listening);
    }

    #[tokio::test]
    async fn can_receive_without_consuming_stream() {
        // Given
        let mut seq = Sequence::new();
        let mut transceiver = MockTransceiver::new();
        transceiver
            .expect_set_channel()
            .with(eq(Channel::A))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        transceiver
            .expect_listen()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        transceiver
            .expect_idle()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        let stream = ctrl.receive().await;
        drop(stream);
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await;
        assert!(!ctrl.listening);
    }

    #[tokio::test]
    async fn can_receive_aborted_before_any_frames() {
        // Given
        let mut transceiver = MockAsyncTransceiver::new();
        transceiver
            .expect_set_channel()
            .withf(|_channel| true)
            .returning(|_| Box::pin(future::ready(())));
        transceiver
            .expect_listen()
            .returning(|| Box::pin(future::ready(())));
        transceiver.expect_get_rssi().returning(|| {
            Box::pin(async {
                time::sleep(Duration::from_millis(1)).await;
                -120
            })
        });
        transceiver
            .expect_idle()
            .returning(|| Box::pin(future::ready(())));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        {
            let stream = ctrl.receive().await;
            let timeout = time::sleep(Duration::from_millis(500));
            pin_mut!(stream);
            pin_mut!(timeout);

            if let Either::Left((_frame, _)) = futures::future::select(stream.next(), timeout).await
            {
                assert!(false); // No frame is received
            }
        }
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await;
        assert!(!ctrl.listening);
    }

    #[tokio::test]
    async fn can_receive_frame() {
        // Given
        let mut transceiver = MockAsyncTransceiver::new();
        transceiver
            .expect_set_channel()
            .withf(|_channel| true)
            .returning(|_| Box::pin(future::ready(())));
        transceiver
            .expect_listen()
            .returning(|| Box::pin(future::ready(())));
        transceiver.expect_get_rssi().returning(|| {
            Box::pin(async {
                time::sleep(Duration::from_millis(1)).await;
                -100
            })
        });
        transceiver
            .expect_receive()
            .times(1)
            .returning(|| Box::pin(future::ready(Duration::from_secs(1000))));
        transceiver
            .expect_read()
            .withf(|_buffer, _frame_length| true)
            .returning(|_buffer, _frame_length| Box::pin(future::ready(Ok(10))));
        transceiver
            .expect_idle()
            .returning(|| Box::pin(future::ready(())));

        let mut ctrl = Controller::new(transceiver, TokioDelay);
        let mut received = None;

        // When
        {
            let stream = ctrl.receive().await;
            let timeout = time::sleep(Duration::from_millis(500));
            pin_mut!(stream);
            pin_mut!(timeout);

            while let Either::Left((frame, _)) =
                futures::future::select(stream.next(), timeout).await
            {
                received = frame;

                break;
            }
        }
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await;
        assert!(!ctrl.listening);

        // Then
        let frame = received.unwrap();
        assert_eq!(Duration::from_secs(1000), frame.timestamp);
        assert_eq!(72, frame.length.unwrap());
        assert_eq!(80, frame.received);
    }
}
