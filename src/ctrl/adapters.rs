#[cfg(test)]
pub mod tokio {
    use core::time::Duration;

    use crate::ctrl::traits;

    pub struct TokioDelay;

    impl traits::Delay for TokioDelay {
        async fn delay_micros(&mut self, micros: u32) {
            tokio::time::sleep(Duration::from_micros(micros as u64)).await
        }
    }
}
