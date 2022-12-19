#[cfg(test)]
pub mod tokio {
    use core::time::Duration;

    use async_trait::async_trait;

    use crate::ctrl::traits;

    pub struct TokioTimer;

    #[async_trait]
    impl traits::Timer for TokioTimer {
        async fn sleep_micros(&self, micros: u32) {
            tokio::time::sleep(Duration::from_micros(micros as u64)).await
        }
    }
}
