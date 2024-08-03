use std::future::Future;
use nebulas_core::scheduler::event::Event;

pub struct TokioSender(pub tokio::sync::mpsc::Sender<Event>);

impl nebulas_core::rt::sender::Sender for TokioSender {
    type Error = tokio::sync::mpsc::error::SendError<Event>;
    
    fn send(&self, event: Event) -> impl Future<Output=Result<(), Self::Error>> + Send {
        self.0.send(event)
    }
}