pub mod receiver;
pub mod sender;

pub trait Runtime {
    type Sender: sender::Sender;
    type Receiver: receiver::Receiver;

    fn spawn<F, Future>(f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static;

    fn channel() -> (Self::Sender, Self::Receiver);
}
