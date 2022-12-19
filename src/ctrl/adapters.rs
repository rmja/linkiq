#[cfg(test)]
pub mod tokio {
    use core::time::Duration;

    use async_trait::async_trait;

    use crate::ctrl::traits;

    pub struct TokioTimer;

    #[async_trait]
    impl traits::Timer for TokioTimer {
        async fn sleep(&self, duration: Duration) {
            tokio::time::sleep(duration).await
        }
    }
}
