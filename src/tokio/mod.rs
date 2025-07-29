pub mod receiver;
pub mod sender;

use self::{receiver::TokioReceiver, sender::TokioSender};

pub type Scheduler = nebulas_core::scheduler::Scheduler<Tokio>;

pub struct Tokio;

impl nebulas_core::rt::Runtime for Tokio {
    type Sender = TokioSender;
    type Receiver = TokioReceiver;

    fn spawn<Future>(f: Future)
    where
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        tokio::spawn(f);
    }

    fn channel() -> (Self::Sender, Self::Receiver) {
        let (sender, receiver) = tokio::sync::mpsc::channel(128);
        (TokioSender::new(sender), TokioReceiver::new(receiver))
    }

    fn sleep(duration: std::time::Duration) -> impl std::future::Future<Output = ()> + Send {
        tokio::time::sleep(duration)
    }
}
