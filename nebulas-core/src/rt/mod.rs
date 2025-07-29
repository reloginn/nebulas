pub mod receiver;
pub mod sender;

pub trait Runtime {
    type Sender: sender::Sender;
    type Receiver: receiver::Receiver;

    fn spawn<Future>(f: Future)
    where
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static;

    fn channel() -> (Self::Sender, Self::Receiver);

    fn sleep(duration: std::time::Duration) -> impl std::future::Future<Output = ()> + Send;
}
