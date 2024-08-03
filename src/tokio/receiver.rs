use nebulas_core::scheduler::event::Event;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct TokioReceiver(Arc<Mutex<tokio::sync::mpsc::Receiver<Event>>>);

impl TokioReceiver {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<Event>) -> Self {
        Self(Arc::new(Mutex::new(receiver)))
    }
}

impl nebulas_core::rt::receiver::Receiver for TokioReceiver {
    type Error = tokio::sync::mpsc::error::TryRecvError;

    fn try_recv(&mut self) -> impl std::future::Future<Output = Result<Event, Self::Error>> + Send {
        async move {
            let mut lock = self.0.lock().await;
            let maybe_event = lock.try_recv();
            drop(lock);
            maybe_event
        }
    }
}
