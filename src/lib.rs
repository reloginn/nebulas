#![allow(clippy::manual_async_fn)]

#[cfg(any(feature = "tokio", not(feature = "other-rt")))]
pub mod tokio;

#[cfg(any(feature = "tokio", not(feature = "other-rt")))]
pub use self::tokio::{receiver::TokioReceiver, sender::TokioSender, Scheduler, Tokio};

#[cfg(any(feature = "other-rt", not(feature = "tokio")))]
pub use nebulas_core::{
    rt::{receiver::Receiver, sender::Sender, Runtime},
    scheduler::{event::Event, Scheduler},
};
