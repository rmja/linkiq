use embassy_time::{with_timeout, Duration, TimeoutError, Timer};
use futures::Stream;
use futures_async_stream::stream;

use crate::{
    ctrl::traits::RxToken,
    stack::{phl, Channel, ReadError, Rssi},
};

use super::{noicefloor::NoiceFloor, traits, Frame};

const CHANNEL_COUNT: usize = 4;

/// LinkIQ Transceiver Controller
pub struct Controller<Transceiver: traits::Transceiver> {
    transceiver: Transceiver,
    listening: bool,
    current_channel: Channel,
    min_snr: i8,
    noise_floor: [NoiceFloor; CHANNEL_COUNT],
}

impl<Transceiver> Controller<Transceiver>
where
    Transceiver: traits::Transceiver,
{
    /// Create a new controller
    pub const fn new(transceiver: Transceiver) -> Self {
        Self {
            transceiver,
            listening: false,
            current_channel: Channel::A,
            min_snr: 4,
            noise_floor: [
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
                NoiceFloor::new(-110),
            ],
        }
    }

    pub fn noise_floor(&self) -> [Rssi; CHANNEL_COUNT] {
        let mut res = [0; CHANNEL_COUNT];
        for (i, nf) in self.noise_floor.iter().enumerate() {
            res[i] = nf.value();
        }
        res
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
    ) -> Result<impl Stream<Item = Frame> + 'a, Transceiver::Error> {
        assert!(!self.listening);
        self.transceiver.set_channel(self.current_channel).await?;

        // Start the receiver on the chip
        self.transceiver.listen().await?;
        self.listening = true;

        Ok(self.receive_stream())
    }

    #[stream(item = Frame)]
    async fn receive_stream(&mut self) {
        loop {
            // Make time for test to yield as all mocked futures are completed
            #[cfg(test)]
            Timer::after(Duration::from_ticks(0)).await;

            let rssi = loop {
                if let Ok(rssi) = self.transceiver.get_rssi().await {
                    break rssi;
                }
            };
            let noicefloor = &mut self.noise_floor[self.current_channel as usize];
            let mut token = if rssi > noicefloor.value() + self.min_snr as Rssi {
                let token = {
                    match with_timeout(
                        Duration::from_millis(12),
                        self.transceiver.receive(phl::HEADER_SIZE),
                    )
                    .await
                    {
                        Ok(token) => Some(token.unwrap()),
                        Err(TimeoutError) => None,
                    }
                };

                if let Some(token) = token {
                    token
                } else {
                    // Timeout
                    self.set_next_channel().await.unwrap();
                    continue;
                }
            } else {
                noicefloor.add(rssi);

                self.set_next_channel().await.unwrap();
                continue;
            };

            // Frame was detected - read all frame bytes...
            let mut frame = Frame {
                timestamp: token.timestamp(),
                rssi: Some(rssi),
                ..Default::default()
            };

            loop {
                let buffer = &mut frame.buffer[frame.received..];
                let received = self.transceiver.read(&mut token, buffer).await;

                if let Ok(received) = received {
                    frame.received += received;

                    if frame.len.is_none() {
                        match phl::get_frame_length(&frame.buffer[..frame.received]) {
                            Ok(length) => {
                                self.transceiver.accept(&mut token, length).await.unwrap();
                                frame.len = Some(length);
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

                    if let Some(frame_length) = frame.len
                        && frame.received >= frame_length
                    {
                        // Frame is fully received
                        yield frame;
                        self.set_next_channel().await.unwrap();
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

    async fn set_next_channel(&mut self) -> Result<(), Transceiver::Error> {
        self.current_channel = match self.current_channel {
            Channel::A => Channel::B,
            Channel::B => Channel::C,
            Channel::C => Channel::D,
            Channel::D => Channel::A,
        };

        self.transceiver.set_channel(self.current_channel).await
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
    use embassy_time::{Duration, Instant};
    use futures::{pin_mut, prelude::*};
    use mockall::{predicate::eq, Sequence};

    use crate::ctrl::traits::{stubs::RxTokenStub, MockTransceiver};

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

        let mut ctrl = Controller::new(transceiver);

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

        let mut ctrl = Controller::new(transceiver);

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

        let mut ctrl = Controller::new(transceiver);

        // When
        {
            let stream = ctrl.receive().await.unwrap();
            pin_mut!(stream);

            // No frame is received
            assert!(with_timeout(Duration::from_millis(100), stream.next())
                .await
                .is_err());

            // pin_mut!(stream);
            // pin_mut!(timeout);

            // if let Either::Left((_frame, _)) = futures::future::select(stream.next(), timeout).await
            // {
            //     assert!(false); // No frame is received
            // }
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
            .returning(|_min_frame_length| Ok(RxTokenStub(Instant::now())));
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

        let mut ctrl = Controller::new(transceiver);

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
        assert_eq!(72, frame.len.unwrap());
        assert_eq!(80, frame.received);
    }
}
