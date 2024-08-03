use crate::scheduler::event::Event;

pub trait Sender
where
    Self: Send + Sync + 'static,
{
    type Error: Send + 'static;

    fn send(
        &self,
        event: Event,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;
}
