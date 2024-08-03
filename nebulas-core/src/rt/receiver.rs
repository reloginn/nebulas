use crate::scheduler::event::Event;

pub trait Receiver
where
    Self: Clone + Send + Sync + 'static,
{
    type Error: Send + 'static;

    fn try_recv(&mut self) -> impl std::future::Future<Output = Result<Event, Self::Error>> + Send;
}
