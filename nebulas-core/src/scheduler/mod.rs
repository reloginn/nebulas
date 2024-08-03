use crate::rt::Runtime;

use crate::rt::{receiver::Receiver, sender::Sender};
#[cfg(feature = "chrono")]
use chrono::{NaiveDateTime, Utc};

use event::Event;

pub mod event;

pub struct Scheduler<R>
where
    R: Runtime,
{
    sender: R::Sender,
    receiver: R::Receiver,
}

impl<R> Default for Scheduler<R>
where
    R: Runtime,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Scheduler<R>
where
    R: Runtime,
{
    pub fn new() -> Self {
        let (sender, receiver) = R::channel();
        Self { sender, receiver }
    }
    pub fn via<F, Future>(&self, duration: R::Duration, f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        R::spawn(move || async move {
            R::sleep(duration).await;
            f().await
        });
    }
    #[cfg(feature = "chrono")]
    pub fn on<F, Future>(&self, id: &'static [u8], naive: NaiveDateTime, f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        let mut receiver = self.receiver.clone();
        R::spawn(move || async move {
            loop {
                match receiver.try_recv().await {
                    Ok(event) => {
                        let Event::Destroy { to } = event;
                        if id == to {
                            break;
                        }
                    }
                    Err(_) => {
                        let now = Utc::now();
                        if now.naive_utc() >= naive {
                            f().await;
                            break;
                        }
                    }
                }
            }
        });
    }
    pub async fn send_event(
        &self,
        event: Event,
    ) -> Result<(), <<R as Runtime>::Sender as Sender>::Error> {
        self.sender.send(event).await
    }
}
