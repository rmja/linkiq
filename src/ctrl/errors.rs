#[derive(Debug)]
pub enum TransmitError {}

#[derive(Debug)]
pub enum ReceiveError {
    Timeout,
}
