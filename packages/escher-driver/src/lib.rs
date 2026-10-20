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
//! anything runs, and every refusal names one of eight [`Cause`]s ([`CAUSES`]) with a fixed
//! remedy. Twelve verbs stand in
//! three levels. Eight are asked of the held instance: `snapshot`, `click`, `type`, `press`,
//! `advance`, `scroll`, `screenshot` and `lint`. Three are asked of the session itself — `start`, `status` and
//! `stop` — and one of the tool itself, `help`; these four are checked by [`validate_session`],
//! and a session runs none of them.
//!
//! The tool describes itself through `help`: [`describe`] writes, from the table alone, one
//! document of every verb with its arguments and result fields, the keys of a node
//! ([`NODE_FIELDS`], each with its help), the key names, the busy classes, the causes with
//! their remedies, the command line's exit statuses ([`EXIT_STATUSES`]) and the session errors
//! with their remedies ([`SessionError::remedy`]). The command line and the MCP server serve the
//! same document.
//!
//! A call runs through a session: [`Session::run`] checks it, runs its verb on the held
//! instance, settles the instance and returns an [`Outcome`] — the screen's text for
//! `snapshot`, and for an acting verb whether the instance went quiet and the diff of the
//! screen before and after, named by stable element id. `type` replaces what the element
//! holds, and an empty text clears it; `scroll` brings the element an id names into view and
//! says whether it then is. `screenshot` paints the screen, or the element an id names, with no
//! display and returns it as a PNG ([`Outcome::Shot`]); an app with no font it can paint text
//! with is refused ([`Cause::FontsUnavailable`]). Time moves only by `advance`: it moves the held
//! instance's clock — the one clock every time the engine reads takes, the overlay scrollbar
//! fade and double-click detection included — by exactly the milliseconds asked, on every
//! session, then hands them to the step a session's caller supplies for the app's own time
//! ([`Session::with_time`]), where one was supplied. An outcome, a refusal and a session error each have one
//! written form, a line of JSON keyed by the schema's own words ([`Outcome::to_json`],
//! [`Refusal::to_json`], [`SessionError::to_json`]), and three surfaces read it: the session's
//! socket, the command line and the MCP server.
//!
//! An action aimed at a target that cannot take it is refused before anything is dispatched,
//! and the refusal names why: the id names nothing on the screen ([`Cause::NotFound`], or
//! [`Cause::Stale`] when an earlier screen of the session read it), or the element is not
//! enabled ([`Cause::Disabled`]), out of view ([`Cause::OffScreen`]) or under another element
//! ([`Cause::Covered`]). A refused call leaves the held instance as it was.
//!
//! An element's `bounds` are where it stands on the screen — a box reads the same bounds
//! however far its content is scrolled — and a hit stops where a box clips by `overflow`, so
//! content scrolled out of a box covers nothing. An element that states itself selected,
//! pressed or invalid in its markup (`aria-selected`, `aria-pressed`, `aria-invalid`) reads so
//! as true or false, with two limits, each stated in its node key's help: a mixed toggle reads
//! not pressed, and a grammar or spelling error reads invalid. A changed node names the keys
//! whose reading differs (`differs`). Three readings are known to be off, measured on the engine
//! as built and stated where the schema describes them: a box clipped on one axis stops a hit
//! at its whole padding box, so an element lying outside it along the axis it does not clip can
//! read [`Cause::Covered`] though it shows; a box with no width or no height is hit at its
//! centre on the element that holds it, so it can read [`Cause::Covered`] though nothing covers
//! it; and a scrolling box under a transform is read where it stands untransformed, so an
//! element inside it can read [`Cause::OffScreen`] though it shows. A screenshot no longer
//! depends on wall time: the overlay scrollbar fade reads the session's clock, so two shots of
//! one state with no `advance` between them read the same pixels.
//!
//! [`Session::lint`] reads the screen of the held instance for six defects: an element that
//! overlaps another, is clipped, has text cut from view, lies off-screen, holds content in a box
//! with no area or reads below its text contrast ratio. Each [`Finding`] names the element's
//! stable id, its [`Defect`] and the measured [`Reason`], numbers and ids only. It runs no step
//! and changes nothing. The verb `lint` answers its findings on every surface
//! ([`Outcome::Linted`]), and a recorded session keeps them as it keeps every answer.
//!
//! A session can keep itself as a replayable check ([`Session::recording_into`]): every call
//! [`Session::run`] answers, written to an owner-only file of a directory its launcher names as
//! the call crossed in, with whether it was accepted, the [`answer_digest`] of its answer and the
//! answer itself, whole as the driver wrote it but for a shot's pixels — masked as on every
//! carrier, and the text typed into a masked control as [`MASKED_STAND_IN`].
//! [`Check::from_bytes`] reads one back, strictly and under its bounds, into [`Step`]s, and
//! [`Check::replay`] runs their calls again through [`Session::run`] and says in a [`Replay`]
//! whether every answer is the kept one, or names the first step that changed and the keys of
//! its answer that differ ([`Change`]). Nothing of a recording or a replay is logged.
//!
//! [`command_line`] is the whole command line of a binary that boots apps: every verb as one
//! run, answered with one line of JSON on stdout and a status that tells accepted from
//! refused, from a usage error and from a session error.
//!
//! [`serve_mcp`] serves the verbs as MCP tools over a reader and a writer — newline-delimited
//! JSON-RPC 2.0 in the protocol's handshake revisions — and holds the session it starts in its
//! own process. A `snapshot` tool result is the screen's text, a `screenshot` result the image
//! as an MCP image block beside its line of JSON, and every other result the command's line of
//! JSON. Each tool call leaves one `tracing` span, `tool_call`, under the
//! target `escher_driver`, with two fields — `tool`, the table's word for its verb, and
//! `ending`, `accepted`, `refused` or `error` — and the call's `command` span is its child.
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
mod describe;
mod error;
mod execute;
mod host;
mod image;
mod json;
mod lint;
mod mcp;
mod recording;
mod refusal;
mod replay;
mod schema;
mod session;
#[cfg_attr(not(unix), allow(dead_code))]
mod wire;

pub use blitz_test_harness::{Busy, Settled};
pub use cli::command_line;
pub use client::{Answer, Hello, Started, attach, call, start, stop};
pub use command::{ArgValue, Call, Command, Key, SessionCommand, validate, validate_session};
pub use describe::describe;
pub use error::SessionError;
pub use execute::Outcome;
pub use host::{IDLE_EXPIRY, serve};
pub use lint::{Bounds, Defect, Finding, Reason};
pub use mcp::serve_mcp;
pub use recording::{Check, MASKED_STAND_IN, Step, answer_digest};
pub use refusal::{CAUSES, Cause, Fault, Refusal};
pub use replay::{Change, Replay};
pub use schema::{
    ArgKind, ArgSpec, BUSY_CLASSES, EXIT_STATUSES, ExitStatus, FieldKind, FieldSpec, KEY_NAMES,
    Level, MAX_ID_BYTES, MAX_MILLISECONDS, MAX_TEXT_BYTES, NODE_FIELDS, NodeFieldSpec, VERBS,
    VerbSpec, verb,
};
pub use session::{LOAD_WAIT, Session};
