//! The escher driver's session: one headless app instance held by one process while commands
//! arrive.
//!
//! A [`Session`] owns the instance its caller boots — this crate names no app, so the boot is
//! a closure the caller supplies — and keeps it alive between commands: what one command
//! leaves is what the next one reads. [`Session::act`] runs one step on the instance and
//! returns once it has settled, so a step's consequences are on the screen when it returns.
//!
//! [`serve`] hosts one session for the life of a process, on the thread that owns the
//! instance, until it is stopped or has had no request for its idle expiry. Another process
//! drives the lifecycle with [`start`], [`attach`] and [`stop`] and sends the session a call
//! with [`call`]; each edge of that has one named outcome, a [`SessionError`].
//!
//! The two processes meet on a Unix-domain socket in an owner-only state directory the caller
//! names. Three requests cross it: `hello` and `stop`, answered with the host's process id,
//! the session's label, a count of the requests answered so far and the idle expiry; and a
//! call, answered with its outcome or its refusal as one line of JSON. A call carries the
//! element id and the text it names, and its answer holds what the screen reads — ids,
//! accessible names, control values (a password's or a file input's as a fixed mask), the
//! snapshot's text and the diff. Access is the directory's and the socket's owner-only modes;
//! a request is at most 16,384 bytes and an answer at most 1,048,576.
//!
//! The socket is unix-only: on other platforms [`serve`], [`start`], [`attach`], [`stop`] and
//! [`call`] return [`SessionError::Unsupported`]. A [`Session`] held in process works
//! everywhere.
//!
//! What can be asked of the driver, and how it says no, is stated once, as data held in
//! process: [`VERBS`] lists each verb with its level, its argument shapes and its result
//! shape, [`validate`] turns a [`Call`] into a typed [`Command`] or a [`Refusal`] before
//! anything runs, and every refusal names a [`Cause`] with a fixed remedy. Six verbs are asked
//! of the held instance: `snapshot`, `click`, `type`, `press`, `advance` and `scroll`. Three
//! are asked of the session itself — `start`, `status` and `stop` — and are checked by
//! [`validate_session`]; a session runs none of them.
//!
//! A call runs through a session: [`Session::run`] checks it, runs its verb on the held
//! instance, settles the instance and returns an [`Outcome`] — the screen's text for
//! `snapshot`, and for an acting verb whether the instance went quiet and the diff of the
//! screen before and after, named by stable element id. `type` replaces what the element
//! holds, and an empty text clears it; `scroll` brings the element an id names into view and
//! says whether it then is. Time moves only by `advance`, through the step a session's caller
//! hands it ([`Session::with_time`]). An outcome, a refusal and a session error each have one
//! written form, a line of JSON keyed by the schema's own words ([`Outcome::to_json`],
//! [`Refusal::to_json`], [`SessionError::to_json`]).
//!
//! An action aimed at a target that cannot take it is refused before anything is dispatched,
//! and the refusal names why: the id names nothing on the screen ([`Cause::NotFound`], or
//! [`Cause::Stale`] when an earlier screen of the session read it), or the element is not
//! enabled ([`Cause::Disabled`]), out of view ([`Cause::OffScreen`]) or under another element
//! ([`Cause::Covered`]). A refused call leaves the held instance as it was.
//!
//! An element's `bounds` are where it stands on the screen — a box reads the same bounds
//! however far its content is scrolled — and a hit stops where a box clips by `overflow`, so
//! content scrolled out of a box covers nothing. One reading is known to be off, measured on
//! the engine as built and stated where the schema describes it: a hit reaches content clipped
//! by `contain: paint`, so an element lying where such content extends can read
//! [`Cause::Covered`] though nothing shows over it.
//!
//! [`command_line`] is the whole command line of a binary that boots apps: every verb as one
//! run, answered with one line of JSON on stdout and a status that tells accepted from
//! refused, from a usage error and from a session error.
//!
//! A call run through [`Session::run`] leaves one `tracing` span, `command`, at INFO under the
//! target `escher_driver`. Its fields say which verb ran (`verb`), why a refused call was
//! refused (`cause`, the name of a [`Cause`]), how an acting verb's settle went (`settled`;
//! `passes` when the instance went quiet; `busy`, one of [`BUSY_CLASSES`], when it did not) and
//! how many nodes the returned diff names (`added`, `removed`, `changed`). Each holds a fixed
//! word of the schema or a count — nothing a call supplied and nothing a screen reads — and a
//! field that does not apply to a call is absent. This crate installs no subscriber: a caller
//! that wants the record installs one.

#![deny(missing_docs)]

mod cli;
mod client;
mod command;
mod error;
mod execute;
mod host;
mod json;
mod refusal;
mod schema;
mod session;
#[cfg_attr(not(unix), allow(dead_code))]
mod wire;

pub use blitz_test_harness::{Busy, Settled};
pub use cli::command_line;
pub use client::{Answer, Hello, Started, attach, call, start, stop};
pub use command::{ArgValue, Call, Command, Key, SessionCommand, validate, validate_session};
pub use error::SessionError;
pub use execute::Outcome;
pub use host::{IDLE_EXPIRY, serve};
pub use refusal::{CAUSES, Cause, Fault, Refusal};
pub use schema::{
    ArgKind, ArgSpec, BUSY_CLASSES, FieldKind, FieldSpec, KEY_NAMES, Level, MAX_ID_BYTES,
    MAX_MILLISECONDS, MAX_TEXT_BYTES, NODE_FIELDS, VERBS, VerbSpec, verb,
};
pub use session::Session;
