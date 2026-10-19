//! The driver's verbs as MCP tools: newline-delimited JSON-RPC 2.0 over a reader and a writer,
//! in the protocol's handshake revisions (`initialize`, `tools/list`, `tools/call`).
//!
//! Requests are read with `serde_json`; every reply is written by the crate's own JSON writer.
//! A request is untrusted input: it is read under a byte bound, a line outside JSON or outside
//! the protocol is answered with a fixed JSON-RPC error and runs nothing, and a tool call is
//! checked against the verb table before anything runs. Nothing a client sent is written back
//! but the request's `id`, and only in the two forms admitted for it.

mod framing;
mod listing;
mod result;

use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

use serde_json::{Map, Value};
use tracing::Span;
use tracing::field::Empty;

use crate::client::Hello;
use crate::command::{Call, SessionCommand, call_of, validate, validate_session};
use crate::describe::describe;
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::json::{self, Object};
use crate::refusal::{Fault, Refusal};
use crate::schema::{self, Level, VerbSpec};
use crate::session::Session;
use framing::{
    Envelope, Id, ProtocolError, Request, envelope, error_reply, read_request, result_reply,
    write_line,
};
use listing::{initialized, tool_list};
use result::{Ending, ToolResult};

/// The table's name for `start`'s one argument, which an app the binary does not boot is
/// refused as outside the bound of.
const START_APP: &str = match schema::START.args {
    [app] => app.name,
    _ => "",
};

/// Serves the driver's verbs as MCP tools: reads one request per line from `input` and writes
/// one reply per line to `output`, until `input` ends.
///
/// The protocol is JSON-RPC 2.0 in the handshake revisions `2025-11-25`, `2025-06-18`,
/// `2025-03-26` and `2024-11-05`. `initialize` answers the revision the client asks for when it
/// is one of these, and `2025-11-25` otherwise; `ping`, `tools/list` and `tools/call` are answered as
/// the revisions state, and every other method, `server/discover` included, as one not found.
/// A request with no `id` is a notification and is not answered.
///
/// The tools are the verbs of [`VERBS`](schema::VERBS), in the table's order, each described by its help and
/// its result fields and given an input schema of its arguments, and `start`'s app limited to
/// `apps`. A tool call runs as the command line runs its verb, except that the session lives in
/// this process: `start` boots the app `boot` builds and holds it, `status` reaches it, `stop`
/// drops it, `help` answers the tool's description of itself ([`describe`]) whether or not a
/// session is held, and the seven instance verbs run on the held session through [`Session::run`]. A held session that has had no tool call for `idle_expiry` is
/// dropped when the next request arrives.
///
/// A tool result holds one text block: the screen's text for `snapshot`, the document for
/// `help`, the command's line of JSON for every other call that ran, and its refusal or its session error, marked as an
/// error, for one that did not. A `screenshot` that ran holds two blocks: an `image` block, the
/// PNG as base64 with its MIME type, then the text block of its line of JSON without the image.
/// A text over 1,048,576 bytes, or a `screenshot` result whose written form is, is replaced by
/// [`SessionError::AnswerTooLarge`]'s, and the result of a call that ended the held session's
/// recording by [`SessionError::RecordingFailed`]'s: either way the call ran and is not rolled
/// back.
///
/// A request line is at most 65,536 bytes; a longer one, a line that is not JSON and one
/// outside the protocol are answered with a fixed JSON-RPC error and run nothing. A reply
/// echoes the request's `id` when it is an integer or a string of at most 128 bytes; another
/// form is answered as an invalid request, with a null `id`. Nothing else a client sent is
/// written back.
///
/// Each `tools/call` leaves one `tracing` span, `tool_call`, at INFO under the target
/// `escher_driver`: `tool`, the table's word for the verb the call names (absent when it names
/// none), and `ending`, one of `accepted`, `refused` and `error`. The call's own `command`
/// span, when it runs on the held session, is its child; a session verb and `help`, answered by
/// the server, leave none.
///
/// It returns once `input` ends; a failed read or write is [`SessionError::Io`]. It installs no
/// log subscriber, reads no environment variable and writes nothing to `output` but replies.
pub fn serve_mcp(
    mut input: impl BufRead,
    mut output: impl Write,
    apps: &[&str],
    boot: impl FnMut(&str) -> Result<Session, SessionError>,
    idle_expiry: Duration,
) -> Result<(), SessionError> {
    let mut server = Server {
        apps,
        boot,
        held: None,
        idle_expiry,
    };
    loop {
        let handled = match read_request(&mut input).map_err(io_error)? {
            Request::End => return Ok(()),
            Request::Line(line) if line.is_empty() => Ok(()),
            Request::Line(line) => {
                server.expire();
                server.handle(&line, &mut output)
            }
            Request::TooLong => {
                server.expire();
                write_line(&mut output, &error_reply(&Id::Null, ProtocolError::Parse))
            }
        };
        handled.map_err(io_error)?;
    }
}

fn io_error(error: io::Error) -> SessionError {
    SessionError::Io(error.kind())
}

/// The span of one tool call, its fields unrecorded.
fn tool_call_span() -> Span {
    tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
}

/// The session the server holds, with what `status` reports of it.
struct Held {
    session: Session,
    label: String,
    /// The tool calls answered on the session since its `start`.
    served: u64,
    last_answer: Instant,
}

struct Server<'a, B> {
    apps: &'a [&'a str],
    boot: B,
    held: Option<Held>,
    idle_expiry: Duration,
}

impl<B: FnMut(&str) -> Result<Session, SessionError>> Server<'_, B> {
    /// Drops the held session when it has had no tool call for the idle expiry.
    fn expire(&mut self) {
        if let Some(held) = &self.held
            && held.last_answer.elapsed() >= self.idle_expiry
        {
            self.held = None;
        }
    }

    fn handle(&mut self, line: &[u8], output: &mut impl Write) -> io::Result<()> {
        let (id, method, params) = match envelope(line) {
            Envelope::Notification => return Ok(()),
            Envelope::Error(id, error) => return write_line(output, &error_reply(&id, error)),
            Envelope::Request { id, method, params } => (id, method, params),
        };
        let reply = match method.as_str() {
            "initialize" => result_reply(&id, &initialized(params.as_ref())),
            "ping" => result_reply(&id, &Object::new().end()),
            "tools/list" => result_reply(&id, &tool_list(self.apps)),
            "tools/call" => {
                let span = tool_call_span();
                let _entered = span.enter();
                let reply = match self.tool_call(params.as_ref(), &span) {
                    Ok(result) => {
                        span.record("ending", result.ending.word());
                        result_reply(&id, &result.written())
                    }
                    Err(error) => {
                        span.record("ending", Ending::Error.word());
                        error_reply(&id, error)
                    }
                };
                return write_line(output, &reply);
            }
            _ => error_reply(&id, ProtocolError::MethodNotFound),
        };
        write_line(output, &reply)
    }

    fn tool_call(
        &mut self,
        params: Option<&Value>,
        span: &Span,
    ) -> Result<ToolResult, ProtocolError> {
        let Some(Value::Object(params)) = params else {
            return Err(ProtocolError::InvalidParams);
        };
        let Some(Value::String(name)) = params.get("name") else {
            return Err(ProtocolError::InvalidParams);
        };
        let verb = schema::verb(name);
        if let Some(verb) = verb {
            span.record("tool", verb.name);
        }
        let none = Map::new();
        let arguments = match params.get("arguments") {
            None => &none,
            Some(Value::Object(arguments)) => arguments,
            Some(_) => return Err(ProtocolError::InvalidParams),
        };
        let Some(verb) = verb else {
            return Err(ProtocolError::UnknownTool);
        };
        let was_held = self.held.is_some();
        let result = match call_of(verb, arguments) {
            Ok(call) => self.run(verb, &call),
            Err(refusal) => ToolResult::refused(refusal),
        };
        if was_held && let Some(held) = &mut self.held {
            held.served += 1;
            held.last_answer = Instant::now();
        }
        Ok(result.bounded())
    }

    fn run(&mut self, verb: &'static VerbSpec, call: &Call) -> ToolResult {
        match verb.level {
            Level::Instance => {
                let Some(held) = &mut self.held else {
                    // Checked before the session is asked for, as on the command line.
                    return match validate(call) {
                        Ok(_) => ToolResult::failed(SessionError::NoSession),
                        Err(refusal) => ToolResult::refused(refusal),
                    };
                };
                let answer = held.session.run(call);
                if held.session.recording_failed() {
                    return ToolResult::failed(SessionError::RecordingFailed);
                }
                match answer {
                    Ok(Outcome::Screen { text }) => ToolResult::accepted(text),
                    Ok(Outcome::Shot { width, height, png }) => {
                        ToolResult::shot(width, height, &png)
                    }
                    Ok(outcome) => ToolResult::accepted(outcome.to_json()),
                    Err(refusal) => ToolResult::refused(refusal),
                }
            }
            Level::Tool => match validate_session(call) {
                Ok(_) => ToolResult::accepted(describe()),
                Err(refusal) => ToolResult::refused(refusal),
            },
            Level::Session => match validate_session(call) {
                Err(refusal) => ToolResult::refused(refusal),
                Ok(SessionCommand::Start { app }) => self.start(app),
                Ok(SessionCommand::Status) => match &self.held {
                    Some(held) => ToolResult::accepted(json::status(&self.hello(held))),
                    None => ToolResult::failed(SessionError::NoSession),
                },
                Ok(SessionCommand::Stop) => match self.held.take() {
                    Some(_) => ToolResult::accepted(json::stopped()),
                    None => ToolResult::failed(SessionError::NoSession),
                },
                Ok(SessionCommand::Help) => ToolResult::accepted(describe()),
            },
        }
    }

    fn start(&mut self, app: String) -> ToolResult {
        if !self.apps.contains(&app.as_str()) {
            return ToolResult::refused(Refusal::malformed(Fault::OutOfBound(START_APP)));
        }
        if self.held.is_some() {
            return ToolResult::failed(SessionError::AlreadyRunning);
        }
        match (self.boot)(&app) {
            Ok(session) => {
                let held = Held {
                    session,
                    label: app,
                    served: 0,
                    last_answer: Instant::now(),
                };
                let answer = json::started(&self.hello(&held));
                self.held = Some(held);
                ToolResult::accepted(answer)
            }
            Err(error) => ToolResult::failed(error),
        }
    }

    fn hello(&self, held: &Held) -> Hello {
        Hello {
            pid: std::process::id(),
            label: held.label.clone(),
            served: held.served,
            idle_expiry_s: self.idle_expiry.as_secs(),
        }
    }
}

#[cfg(test)]
#[path = "mcp_tests.rs"]
mod tests;
