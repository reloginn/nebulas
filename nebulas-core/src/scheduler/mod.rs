use crate::{rt::Runtime, sleep::Sleep};
use std::marker::PhantomData;

use crate::rt::{receiver::Receiver, sender::Sender};
#[cfg(feature = "chrono")]
use chrono::{NaiveDateTime, Utc};

use event::Event;

pub mod event;

pub struct Scheduler<R, S>
where
    R: Runtime,
    S: Sleep,
{
    sender: R::Sender,
    receiver: R::Receiver,
    _phantom: PhantomData<(R, S)>,
}

impl<R, S> Scheduler<R, S>
where
    R: Runtime,
    S: Sleep,
{
    pub fn new() -> Self {
        let (sender, receiver) = R::channel();
        Self {
            sender,
            receiver,
            _phantom: PhantomData,
        }
    }
    pub fn via<F, Future>(&self, duration: S::T, f: F)
    where
        F: FnOnce() -> Future + Send + Sync + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        R::spawn(move || async move {
            S::sleep(duration).await;
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
                        if let Event::Destroy { to } = event {
                            if id == to {
                                break;
                            }
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
    pub async fn send_event(&self, event: Event) -> Result<(), <<R as Runtime>::Sender as Sender>::Error> {
        self.sender.send(event).await
    }
}
