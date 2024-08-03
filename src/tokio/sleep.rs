use std::future::Future;
use nebulas_core::sleep::Sleep;

pub struct TokioSleep;

impl Sleep for TokioSleep {
    type T = tokio::time::Duration;
    
    fn sleep(duration: Self::T) -> impl Future<Output=()> + Send {
        async move { tokio::time::sleep(duration).await; }
    }
}