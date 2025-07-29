#![allow(clippy::manual_async_fn)]

#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(feature = "tokio")]
pub use self::tokio::{Scheduler, Tokio, receiver::TokioReceiver, sender::TokioSender};

#[cfg(not(feature = "tokio"))]
pub use nebulas_core::{
    rt::{Runtime, receiver::Receiver, sender::Sender},
    scheduler::{Scheduler, event::Event},
};
