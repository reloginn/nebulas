use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use std::{marker::PhantomData, time::Duration};
use tokio::sync::oneshot::{channel, Receiver, Sender};

/// Same as [`Schedule::new`], but as a function
pub fn schedule<Func, Data, Future>(f: Func) -> Schedule<Func, Data, Future>
where
    Data: Send + 'static,
    Func: FnOnce(Sender<Data>) -> Future + Send + 'static,
    Future: std::future::Future + Send,
    Future::Output: Send + 'static,
{
    Schedule::new(f)
}

pub struct Schedule<Func, Data, Future>
where
    Data: Send + 'static,
    Func: FnOnce(Sender<Data>) -> Future + Send + 'static,
    Future: std::future::Future + Send,
    Future::Output: Send + 'static,
{
    f: Func,
    _phantom: PhantomData<Data>,
}

impl<Func, Data, Future> Schedule<Func, Data, Future>
where
    Data: Send + 'static,
    Func: FnOnce(Sender<Data>) -> Future + Send + 'static,
    Future: std::future::Future + Send,
    Future::Output: Send + 'static,
{
    /// Creates an instance of the [`Schedule`] with a defined function
    ///
    /// # Example
    /// ```rust
    /// let schedule = Schedule::new(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.via(std::time::Duration::from_secs(1));
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn new(f: Func) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
    /// Executes the function that [`Schedule`] contains after a certain time ([`std::time::Duration`])
    /// # Example
    /// ```rust
    /// let schedule = schedule(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.via(std::time::Duration::from_secs(10));
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn via(self, duration: Duration) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            f(sender).await
        });
        receiver
    }
    /// Executes the function that [`Schedule`] contains on the given [`NaiveDateTime`]
    ///
    /// # Example
    /// ```rust
    /// use std::ops::Add;
    ///
    /// let schedule = schedule(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.on_date_and_time(chrono::Utc::now().add(chrono::TimeDelta::seconds(10)));
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn on_date_and_time(self, naive_utc: NaiveDateTime) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                if now.naive_utc() == naive_utc {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    /// Executes the function that [`Schedule`] contains on the given [`NaiveDate`].
    ///
    /// # Example
    /// ```rust
    /// use std::ops::Add;
    ///
    /// let schedule = schedule(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.on_date(chrono::Utc::now().date_naive());
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn on_date(self, date: NaiveDate) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                if now.date_naive() == date {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    /// Executes the function that [`Schedule`] contains on the given [`NaiveTime`].
    /// # Example
    /// ```rust
    /// use std::ops::Add;
    ///
    /// let schedule = schedule(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.on_time(chrono::Utc::now().time().add(chrono::TimeDelta::seconds(10)));
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn on_time(self, time: NaiveTime) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                if now.time() == time {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
    /// Executes the function that [`Schedule`] contains on the given [`Weekday`].
    /// # Example
    /// ```rust
    /// use std::ops::Add;
    /// use chrono::Datelike;
    ///
    /// let schedule = schedule(|sender| async move {
    ///     println!("I'm executing!");
    ///     sender.send(0usize).unwrap();
    /// });
    /// let receiver = schedule.on_time(chrono::Utc::now().weekday());
    ///
    /// loop {
    ///     match receiver.try_recv().await {
    ///         Ok(received) => assert!(received == 0),
    ///         Err(_) => continue
    ///     }
    /// }
    /// ```
    pub fn on_weekday(self, weekday: Weekday) -> Receiver<Data> {
        let (sender, receiver) = channel::<Data>();
        let f = self.f;
        tokio::spawn(async move {
            loop {
                let now = chrono::Utc::now();
                if now.weekday() == weekday {
                    f(sender).await;
                    break;
                }
            }
        });
        receiver
    }
}
