use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use nebulas_core::scheduler::event::Event;

#[derive(Clone)]
pub struct TokioReceiver(pub Arc<Mutex<tokio::sync::mpsc::Receiver<Event>>>);

impl nebulas_core::rt::receiver::Receiver for TokioReceiver {
    type Error = tokio::sync::mpsc::error::TryRecvError;
    
    fn try_recv(&mut self) -> impl Future<Output=Result<Event, Self::Error>> + Send {
        async move {
            let mut lock = self.0.lock().await;
            let maybe_event = lock.try_recv();
            drop(lock);
            maybe_event
        }
    }
}