use core::time::Duration;
use alloc::boxed::Box;

use async_trait::async_trait;

#[async_trait]
pub trait Delay {
    async fn delay(&self, duration: Duration);
}

#[cfg(test)]
pub struct TokioDelay;

#[cfg(test)]
#[async_trait]
impl Delay for TokioDelay {
    async fn delay(&self, duration: Duration) {
        tokio::time::sleep(duration).await
    }
}