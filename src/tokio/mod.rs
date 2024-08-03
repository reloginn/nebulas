pub mod receiver;
pub mod sender;

use self::{receiver::TokioReceiver, sender::TokioSender};

pub type Scheduler = nebulas_core::scheduler::Scheduler<Tokio>;

pub struct Tokio;

impl nebulas_core::rt::Runtime for Tokio {
    type Duration = tokio::time::Duration;
    type Sender = TokioSender;
    type Receiver = TokioReceiver;

    fn spawn<F, Future>(f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        tokio::spawn(async move { f().await });
    }

    fn channel() -> (Self::Sender, Self::Receiver) {
        let (sender, receiver) = tokio::sync::mpsc::channel(512);
        (TokioSender::new(sender), TokioReceiver::new(receiver))
    }

    fn sleep(duration: Self::Duration) -> impl std::future::Future<Output = ()> + Send {
        async move { tokio::time::sleep(duration).await }
    }
}
