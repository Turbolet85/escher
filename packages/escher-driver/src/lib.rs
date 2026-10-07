//! The escher driver's session: one headless app instance held by one process while commands
//! arrive.
//!
//! A [`Session`] owns the instance its caller boots — this crate names no app, so the boot is
//! a closure the caller supplies — and keeps it alive between commands: what one command
//! leaves is what the next one reads. [`Session::act`] runs one step on the instance and
//! returns once it has settled, so a step's consequences are on the screen when it returns.
//!
//! [`serve`] hosts one session for the life of a process, on the thread that owns the
//! instance. Another process drives the lifecycle with [`start`], [`attach`] and [`stop`];
//! each edge of it has one named outcome, a [`SessionError`].
//!
//! The two processes meet on a Unix-domain socket in an owner-only state directory the caller
//! names. Two requests cross it, `hello` and `stop`, answered with the host's process id, the
//! session's label and a count of the requests answered so far. Nothing of the screen crosses
//! it: no element id, no accessible name and no control value.
//!
//! The socket is unix-only: on other platforms [`serve`], [`start`], [`attach`] and [`stop`]
//! return [`SessionError::Unsupported`]. A [`Session`] held in process works everywhere.

#![deny(missing_docs)]

mod client;
mod error;
mod host;
mod session;
#[cfg_attr(not(unix), allow(dead_code))]
mod wire;

pub use blitz_test_harness::{Busy, Settled};
pub use client::{Hello, Started, attach, start, stop};
pub use error::SessionError;
pub use host::serve;
pub use session::Session;
