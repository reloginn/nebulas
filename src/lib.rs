#[cfg(any(feature = "tokio", not(feature = "other-rt")))]
pub mod tokio;

#[cfg(any(feature = "tokio", not(feature = "other-rt")))]
pub use self::tokio::{Scheduler, sender::TokioSender, receiver::TokioReceiver, sleep::TokioSleep, Tokio};

#[cfg(any(feature = "other-rt", not(feature = "tokio")))]
pub use nebulas_core::{sleep::Sleep, rt::{Runtime, receiver::Receiver, sender::Sender}, scheduler::{Scheduler, event::Event}};