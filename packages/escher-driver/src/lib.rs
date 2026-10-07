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
//!
//! What can be asked of the driver, and how it says no, is stated once, as data held in
//! process: [`VERBS`] lists each verb with its argument and result shapes, [`validate`] turns
//! a [`Call`] into a typed [`Command`] or a [`Refusal`] before anything runs, and every
//! refusal names a [`Cause`] with a fixed remedy.
//!
//! A call runs through a session in process: [`Session::run`] checks it, runs its verb on the
//! held instance, settles the instance and returns an [`Outcome`] — the screen's text for
//! `snapshot`, and for an acting verb whether the instance went quiet and the diff of the
//! screen before and after, named by stable element id. Time moves only by `advance`, through
//! the step a session's caller hands it ([`Session::with_time`]). Nothing of the schema, of a
//! call or of an outcome crosses the socket: they are passed and returned as values.

#![deny(missing_docs)]

mod client;
mod command;
mod error;
mod execute;
mod host;
mod refusal;
mod schema;
mod session;
#[cfg_attr(not(unix), allow(dead_code))]
mod wire;

pub use blitz_test_harness::{Busy, Settled};
pub use client::{Hello, Started, attach, start, stop};
pub use command::{ArgValue, Call, Command, Key, validate};
pub use error::SessionError;
pub use execute::Outcome;
pub use host::serve;
pub use refusal::{CAUSES, Cause, Fault, Refusal};
pub use schema::{
    ArgKind, ArgSpec, BUSY_CLASSES, FieldKind, FieldSpec, KEY_NAMES, MAX_ID_BYTES,
    MAX_MILLISECONDS, MAX_TEXT_BYTES, NODE_FIELDS, VERBS, VerbSpec, verb,
};
pub use session::Session;
