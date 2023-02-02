use embedded_hal_async::delay::DelayUs;
use futures::{
    future::{self, Either},
    pin_mut, Stream,
};
use futures_async_stream::stream;

use crate::{
    ctrl::traits::RxToken,
    stack::{phl, Channel, ReadError, Rssi},
};

use super::{noicefloor::NoiceFloor, traits};

const CHANNEL_COUNT: usize = 4;

/// LinkIQ Transceiver Controller
pub struct Controller<Transceiver: traits::Transceiver, Delay: DelayUs> {
    transceiver: Transceiver,
    delay: Delay,
    listening: bool,
    current_channel: Channel,
    min_snr: i8,
    noisefloor: [NoiceFloor; CHANNEL_COUNT],
}

pub struct Frame<Timestamp> {
    pub timestamp: Option<Timestamp>,
    pub rssi: Option<Rssi>,
    buffer: [u8; phl::MAX_FRAME_LENGTH],
    received: usize,
    length: Option<usize>,
}

impl<Timestamp> const Default for Frame<Timestamp> {
    fn default() -> Self {
        Self {
            timestamp: None,
            rssi: None,
            buffer: [0; phl::MAX_FRAME_LENGTH],
            received: 0,
            length: None,
        }
    }
}

impl<Timestamp> Frame<Timestamp> {
    pub fn bytes(&self) -> &[u8] {
        &self.buffer[0..self.length.unwrap()]
    }
}

impl<Transceiver, Delay> Controller<Transceiver, Delay>
where
    Transceiver: traits::Transceiver,
    Delay: DelayUs,
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

    /// Setup the transceiver and enter idle state.
    pub async fn init(&mut self) -> Result<(), Transceiver::Error> {
        self.listening = false;
        self.transceiver.init().await
    }

    /// Prepare bytes for transmission.
    /// All bytes for the transmission must be written before the transmission is started.
    pub async fn write(&mut self, buffer: &[u8]) -> Result<(), Transceiver::Error> {
        assert!(!self.listening);
        self.transceiver.write(buffer).await
    }

    /// Transmit pre-written bytes.
    /// The transmitter enters idle after the transmission completes.
    pub async fn transmit(&mut self, channel: Channel) -> Result<(), Transceiver::Error> {
        assert!(!self.listening);
        self.current_channel = channel;
        self.transceiver.set_channel(channel).await?;
        self.transceiver.transmit().await
    }

    /// Start and run receiver.
    /// Note that the receiver is _not_ stopped when the stream is dropped, so idle() must be called manually after the stream is dropped.
    pub async fn receive<'a>(
        &'a mut self,
    ) -> Result<impl Stream<Item = Frame<Transceiver::Timestamp>> + 'a, Transceiver::Error> {
        assert!(!self.listening);
        self.transceiver.set_channel(self.current_channel).await?;

        // Start the receiver on the chip
        self.transceiver.listen().await?;
        self.listening = true;

        Ok(self.receive_stream())
    }

    #[stream(item = Frame<Transceiver::Timestamp>)]
    async fn receive_stream(&mut self) {
        loop {
            // Make time for test to yield as all mocked futures are completed
            #[cfg(test)]
            self.delay.delay_us(0).await.unwrap();

            let rssi = self.transceiver.get_rssi().await.unwrap();
            let noicefloor = &mut self.noisefloor[self.current_channel as usize];
            let mut token = if rssi > noicefloor.value() + self.min_snr as Rssi {
                let token = {
                    let token_future = self.transceiver.receive(phl::HEADER_SIZE);
                    let timeout_future = self.delay.delay_us(12_000);
                    pin_mut!(token_future);
                    pin_mut!(timeout_future);

                    if let Either::Left((token, _)) =
                        future::select(token_future, timeout_future).await
                    {
                        Some(token.unwrap())
                    } else {
                        // Timeout
                        None
                    }
                };

                if let Some(token) = token {
                    token
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
            let mut frame = Frame::default();
            frame.timestamp = token.timestamp();
            frame.rssi = Some(rssi);

            loop {
                let buffer = &mut frame.buffer[frame.received..];
                let received = self.transceiver.read(&mut token, buffer).await;

                if let Ok(received) = received {
                    frame.received += received;

                    if frame.length.is_none() {
                        match phl::get_frame_length(&frame.buffer[..frame.received]) {
                            Ok(length) => {
                                self.transceiver.accept(&mut token, length).await.unwrap();
                                frame.length = Some(length);
                            }
                            Err(ReadError::NotEnoughBytes) => {
                                // We need more bytes to derive the frame length
                                continue;
                            }
                            Err(_) => {
                                // Invalid frame length - wait for a new frame to be received
                                break;
                            }
                        }
                    }

                    if let Some(frame_length) = frame.length && frame.received >= frame_length {
                            // Frame is fully received
                            yield frame;
                            self.set_next_channel().await;
                            break;
                    }
                } else {
                    // Error during read - restart receiver
                    self.transceiver.idle().await.unwrap();
                    self.transceiver.listen().await.unwrap();
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
    pub async fn idle(&mut self) -> Result<(), Transceiver::Error> {
        self.transceiver.idle().await?;
        self.listening = false;

        Ok(())
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
        traits::{stubs::RxTokenStub, MockTransceiver},
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
            .return_const(Ok(()));
        transceiver
            .expect_write()
            .withf(|buf: &[u8]| buf == &[0x45, 0x67])
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        transceiver
            .expect_set_channel()
            .with(eq(Channel::C))
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        transceiver
            .expect_transmit()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        ctrl.write(&[0x01, 0x23]).await.unwrap();
        ctrl.write(&[0x45, 0x67]).await.unwrap();
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
            .return_const(Ok(()));
        transceiver
            .expect_listen()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        transceiver
            .expect_idle()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        let stream = ctrl.receive().await;
        drop(stream);
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await.unwrap();
        assert!(!ctrl.listening);
    }

    #[tokio::test]
    async fn can_receive_aborted_before_any_frames() {
        // Given
        let mut transceiver = MockTransceiver::new();
        transceiver
            .expect_set_channel()
            .withf(|_channel| true)
            .return_const(Ok(()));
        transceiver.expect_listen().return_const(Ok(()));
        transceiver.expect_get_rssi().return_const(Ok(-120));
        transceiver.expect_idle().return_const(Ok(()));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        {
            let stream = ctrl.receive().await.unwrap();
            let timeout = time::sleep(Duration::from_millis(100));
            pin_mut!(stream);
            pin_mut!(timeout);

            if let Either::Left((_frame, _)) = futures::future::select(stream.next(), timeout).await
            {
                assert!(false); // No frame is received
            }
        }
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await.unwrap();
        assert!(!ctrl.listening);
    }

    #[tokio::test]
    async fn can_receive_frame() {
        // Given
        let mut transceiver = MockTransceiver::new();
        transceiver
            .expect_set_channel()
            .withf(|_channel| true)
            .return_const(Ok(()));
        transceiver.expect_listen().return_const(Ok(()));
        transceiver.expect_get_rssi().return_const(Ok(-100));
        transceiver
            .expect_receive()
            .times(1)
            .returning(|_min_frame_length| {
                Ok(RxTokenStub {
                    timestamp: Duration::from_secs(1000),
                })
            });
        transceiver
            .expect_read()
            .times(8)
            .withf(|_token, _buffer| true)
            .return_const(Ok(10));
        transceiver
            .expect_accept()
            .times(1)
            .withf(|_token, length| *length == 72)
            .return_const(Ok(()));
        transceiver.expect_idle().return_const(Ok(()));

        let mut ctrl = Controller::new(transceiver, TokioDelay);

        // When
        let received = {
            let stream = ctrl.receive().await.unwrap();
            pin_mut!(stream);

            stream.next().await
        };
        assert!(ctrl.listening); // Receiver is still running

        ctrl.idle().await.unwrap();
        assert!(!ctrl.listening);

        // Then
        let frame = received.unwrap();
        assert_eq!(Duration::from_secs(1000), frame.timestamp.unwrap());
        assert_eq!(72, frame.length.unwrap());
        assert_eq!(80, frame.received);
    }
}
