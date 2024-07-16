use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use std::{marker::PhantomData, time::Duration};
use tokio::sync::oneshot::{channel, Receiver, Sender};

pub struct Schedule<F, Data, Future>
where
    Data: Send + 'static,
    F: FnOnce(Sender<Data>) -> Future + Send + 'static,
    Future: std::future::Future + Send,
    Future::Output: Send + 'static,
{
    f: F,
    _phantom: PhantomData<Data>,
}

impl<F, Data, Future> Schedule<F, Data, Future>
where
    Data: Send + 'static,
    F: FnOnce(Sender<Data>) -> Future + Send + 'static,
    Future: std::future::Future + Send,
    Future::Output: Send + 'static,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
    pub fn via(self, duration: Duration) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            f(sender).await
        });
        receiver
    }
    pub fn on_date_and_time(self, date_and_time: NaiveDateTime) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now().naive_utc();
                if now == date_and_time {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    pub fn on_date(self, date: NaiveDate) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now().date_naive();
                if now == date {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    pub fn on_time(self, time: NaiveTime) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now().time();
                if now == time {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    pub fn on_weekday(self, weekday: Weekday) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now().weekday();
                if now == weekday {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
}
