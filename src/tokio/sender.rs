use nebulas_core::scheduler::event::Event;

pub struct TokioSender(tokio::sync::mpsc::Sender<Event>);

impl TokioSender {
    pub fn new(sender: tokio::sync::mpsc::Sender<Event>) -> Self {
        Self(sender)
    }
}

impl nebulas_core::rt::sender::Sender for TokioSender {
    type Error = tokio::sync::mpsc::error::SendError<Event>;

    fn send(
        &self,
        event: Event,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        self.0.send(event)
    }
}
