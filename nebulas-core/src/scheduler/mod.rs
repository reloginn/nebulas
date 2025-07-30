use std::time::Duration;

use crate::rt::Runtime;
use futures::FutureExt;

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
    pub fn via<Future>(&self, id: usize, duration: Duration, future: Future)
    where
        Future: std::future::Future + Send + Sync + 'static,
        Future::Output: Send + Sync + 'static,
    {
        enum State {
            Freeze(std::time::Duration),
            Unfreeze,
        }

        let mut receiver = self.receiver.clone();
        R::spawn(async move {
            let mut state = State::Unfreeze;
            let events = async {
                loop {
                    if let Ok(event) = receiver.try_recv().await {
                        match event {
                            Event::Shutdown { to } if to == id => return true,
                            Event::Freeze { to, on } if to == id => {
                                state = State::Freeze(on);
                                return false;
                            }
                            _ => {}
                        }
                    }
                }
            };
            futures::select! {
                should_shutdown = events.fuse() => {
                    if should_shutdown {
                        return;
                    }
                },
                _ = R::sleep(duration).fuse() => {
                    match state {
                        State::Freeze(on) => R::sleep(on).await,
                        State::Unfreeze => {}
                    }
                }
            }
            future.await;
        });
    }
    pub fn every<F, Future>(&self, id: usize, every: Duration, f: F)
    where
        F: Fn() -> Future + Send + 'static,
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        let mut receiver = self.receiver.clone();
        R::spawn(async move {
            loop {
                if let Ok(event) = receiver.try_recv().await {
                    match event {
                        Event::Shutdown { to } if to == id => break,
                        Event::Freeze { to, on } if to == id => R::sleep(on).await,
                        _ => {}
                    }
                }
                R::sleep(every).await;
                f().await;
            }
        });
    }
    #[cfg(feature = "chrono")]
    pub fn on<F, Future>(&self, id: usize, naive: NaiveDateTime, f: Future)
    where
        Future: std::future::Future + Send + 'static,
        Future::Output: Send + Sync + 'static,
    {
        let mut receiver = self.receiver.clone();
        R::spawn(async move {
            let mut naive = naive;
            loop {
                match receiver.try_recv().await {
                    Ok(event) => match event {
                        Event::Freeze { to, on } if to == id => {
                            naive += on;
                            R::sleep(on).await
                        }
                        Event::Shutdown { to } if to == id => break,
                        _ => {}
                    },
                    Err(_) => {
                        let now = Utc::now();
                        if now.naive_utc() >= naive {
                            f.await;
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
