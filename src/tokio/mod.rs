pub mod sleep;
pub mod sender;
pub mod receiver;

use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use self::{sender::TokioSender, receiver::TokioReceiver, sleep::TokioSleep};

pub type Scheduler = nebulas_core::scheduler::Scheduler<Tokio, TokioSleep>;

pub struct Tokio;

impl nebulas_core::rt::Runtime for Tokio {
    type Sender = TokioSender;
    type Receiver = TokioReceiver;

    fn spawn<F, Future>(f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static {
        tokio::spawn(async move {
            f().await
        });
    }

    fn channel() -> (Self::Sender, Self::Receiver) {
        let (sender, receiver) = tokio::sync::mpsc::channel(512);
        (TokioSender(sender), TokioReceiver(Arc::new(Mutex::new(receiver))))
    }
}