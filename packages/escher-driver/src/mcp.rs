//! The driver's verbs as MCP tools: newline-delimited JSON-RPC 2.0 over a reader and a writer,
//! in the protocol's handshake revisions (`initialize`, `tools/list`, `tools/call`).
//!
//! Requests are read with `serde_json`; every reply is written by the crate's own JSON writer.
//! A request is untrusted input: it is read under a byte bound, a line outside JSON or outside
//! the protocol is answered with a fixed JSON-RPC error and runs nothing, and a tool call is
//! checked against the verb table before anything runs. Nothing a client sent is written back
//! but the request's `id`, and only in the two forms admitted for it.

use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

use serde_json::{Map, Value};
use tracing::Span;
use tracing::field::Empty;

use crate::client::Hello;
use crate::command::{ArgValue, Call, SessionCommand, validate, validate_session};
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::json::{self, Object};
use crate::refusal::{Fault, Refusal};
use crate::schema::{self, ArgKind, KEY_NAMES, Level, MAX_MILLISECONDS, VERBS, VerbSpec};
use crate::session::Session;
use crate::wire::MAX_ANSWER_BYTES;

/// The longest request line the server reads, in bytes, its terminator not counted.
const MAX_MCP_REQUEST_BYTES: usize = 65_536;

/// The longest string `id` a reply echoes, in bytes.
const MAX_ID_BYTES: usize = 128;

/// The protocol revision answered when a client asks for one this server does not speak.
const LATEST_VERSION: &str = "2025-06-18";

/// The handshake revisions this server speaks.
const PROTOCOL_VERSIONS: [&str; 3] = [LATEST_VERSION, "2025-03-26", "2024-11-05"];

/// The name the server gives itself in its `initialize` answer.
const SERVER_NAME: &str = "escher";

/// The table's name for `start`'s one argument, which an app the binary does not boot is
/// refused as outside the bound of.
const START_APP: &str = match schema::START.args {
    [app] => app.name,
    _ => "",
};

/// Serves the driver's verbs as MCP tools: reads one request per line from `input` and writes
/// one reply per line to `output`, until `input` ends.
///
/// The protocol is JSON-RPC 2.0 in the handshake revisions `2025-06-18`, `2025-03-26` and
/// `2024-11-05`. `initialize` answers the revision the client asks for when it is one of
/// these, and `2025-06-18` otherwise; `ping`, `tools/list` and `tools/call` are answered as
/// the revisions state, and every other method, `server/discover` included, as one not found.
/// A request with no `id` is a notification and is not answered.
///
/// The tools are the verbs of [`VERBS`], in the table's order, each described by its help and
/// its arguments, and `start`'s app limited to `apps`. A tool call runs as the command line
/// runs its verb, except that the session lives in this process: `start` boots the app `boot`
/// builds and holds it, `status` reaches it, `stop` drops it, and the six instance verbs run on
/// it through [`Session::run`]. A held session that has had no tool call for `idle_expiry` is
/// dropped when the next request arrives.
///
/// A tool result holds one text block: the screen's text for `snapshot`, the command's line of
/// JSON for every other call that ran, and its refusal or its session error, marked as an
/// error, for one that did not. A text over 1,048,576 bytes is replaced by
/// [`SessionError::AnswerTooLarge`]'s: the call ran and is not rolled back.
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
/// span, when it runs on the held session, is its child.
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

/// What one read of the input returned.
#[derive(Debug, PartialEq)]
enum Request {
    /// A line within the bound, its terminator removed.
    Line(Vec<u8>),
    /// A line over the bound, read to its terminator and dropped.
    TooLong,
    /// The input ended.
    End,
}

/// Reads one request line from `input`, which keeps whatever it read ahead for the next one.
/// A last line with no terminator is a line.
fn read_request(input: &mut impl BufRead) -> io::Result<Request> {
    let mut line = Vec::new();
    let mut too_long = false;
    loop {
        let available = match input.fill_buf() {
            Ok(available) => available,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        if available.is_empty() {
            return Ok(match (too_long, line.is_empty()) {
                (true, _) => Request::TooLong,
                (false, true) => Request::End,
                (false, false) => Request::Line(line),
            });
        }
        let end = available.iter().position(|byte| *byte == b'\n');
        let (chunk, used) = match end.and_then(|end| available.split_at_checked(end)) {
            Some((chunk, _)) => (chunk, chunk.len() + 1),
            None => (available, available.len()),
        };
        if !too_long {
            if line.len() + chunk.len() > MAX_MCP_REQUEST_BYTES {
                too_long = true;
                line = Vec::new();
            } else {
                line.extend_from_slice(chunk);
            }
        }
        input.consume(used);
        if end.is_some() {
            return Ok(if too_long {
                Request::TooLong
            } else {
                Request::Line(line)
            });
        }
    }
}

/// A request's `id`, in a form a reply echoes.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Id {
    Null,
    Number(i64),
    Text(String),
}

impl Id {
    /// The `id` a request carries, when it is in a form a reply echoes.
    fn admitted(value: &Value) -> Option<Id> {
        match value {
            Value::Number(number) => number.as_i64().map(Id::Number),
            Value::String(text) if text.len() <= MAX_ID_BYTES => Some(Id::Text(text.clone())),
            _ => None,
        }
    }

    fn written(&self) -> String {
        match self {
            Id::Null => String::from("null"),
            Id::Number(number) => number.to_string(),
            Id::Text(text) => {
                let mut out = String::new();
                json::string(&mut out, text);
                out
            }
        }
    }
}

/// A request the server answers with a JSON-RPC error: a fixed code and a fixed message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProtocolError {
    Parse,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    UnknownTool,
}

impl ProtocolError {
    const fn code(self) -> i32 {
        match self {
            ProtocolError::Parse => -32700,
            ProtocolError::InvalidRequest => -32600,
            ProtocolError::MethodNotFound => -32601,
            ProtocolError::InvalidParams | ProtocolError::UnknownTool => -32602,
        }
    }

    const fn message(self) -> &'static str {
        match self {
            ProtocolError::Parse => "parse error",
            ProtocolError::InvalidRequest => "invalid request",
            ProtocolError::MethodNotFound => "method not found",
            ProtocolError::InvalidParams => "invalid params",
            ProtocolError::UnknownTool => "unknown tool",
        }
    }
}

/// What a request line asks for.
#[derive(Debug, PartialEq)]
enum Envelope {
    /// A line answered with an error before any method is looked up.
    Error(Id, ProtocolError),
    /// A request with no `id`: not answered.
    Notification,
    /// A request to answer.
    Request {
        id: Id,
        method: String,
        params: Option<Value>,
    },
}

/// Reads a request line: text, then JSON, then an object with `jsonrpc` `"2.0"` and a string
/// `method`, then an `id` — absent for a notification, else in a form a reply echoes.
fn envelope(line: &[u8]) -> Envelope {
    let Ok(text) = std::str::from_utf8(line) else {
        return Envelope::Error(Id::Null, ProtocolError::Parse);
    };
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        return Envelope::Error(Id::Null, ProtocolError::Parse);
    };
    let Value::Object(mut request) = value else {
        return Envelope::Error(Id::Null, ProtocolError::InvalidRequest);
    };
    let id = request.get("id").map(Id::admitted);
    let spoken = request.get("jsonrpc").and_then(Value::as_str) == Some("2.0");
    let method = match (spoken, request.remove("method")) {
        (true, Some(Value::String(method))) => method,
        _ => {
            let id = id.flatten().unwrap_or(Id::Null);
            return Envelope::Error(id, ProtocolError::InvalidRequest);
        }
    };
    match id {
        None => Envelope::Notification,
        Some(None) => Envelope::Error(Id::Null, ProtocolError::InvalidRequest),
        Some(Some(id)) => Envelope::Request {
            id,
            method,
            params: request.remove("params"),
        },
    }
}

fn reply(id: &Id, member: &str, value: &str) -> String {
    let mut object = Object::new();
    object.text("jsonrpc", "2.0");
    object.json("id", &id.written());
    object.json(member, value);
    object.end()
}

fn result_reply(id: &Id, result: &str) -> String {
    reply(id, "result", result)
}

fn error_reply(id: &Id, error: ProtocolError) -> String {
    let mut object = Object::new();
    object.json("code", &error.code().to_string());
    object.text("message", error.message());
    reply(id, "error", &object.end())
}

fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {
    output.write_all(line.as_bytes())?;
    output.write_all(b"\n")?;
    output.flush()
}

/// A list of already written values.
fn list(items: impl IntoIterator<Item = String>) -> String {
    let items: Vec<String> = items.into_iter().collect();
    format!("[{}]", items.join(","))
}

/// A list of strings.
fn strings<'a>(items: impl IntoIterator<Item = &'a str>) -> String {
    list(items.into_iter().map(|item| {
        let mut out = String::new();
        json::string(&mut out, item);
        out
    }))
}

/// What `initialize` answers: the revision spoken, the tools capability and the server's name
/// and version.
fn initialized(params: Option<&Value>) -> String {
    let asked = params
        .and_then(|params| params.get("protocolVersion"))
        .and_then(Value::as_str);
    let version = PROTOCOL_VERSIONS
        .into_iter()
        .find(|spoken| Some(*spoken) == asked)
        .unwrap_or(LATEST_VERSION);
    let mut capabilities = Object::new();
    capabilities.json("tools", &Object::new().end());
    let mut info = Object::new();
    info.text("name", SERVER_NAME);
    info.text("version", env!("CARGO_PKG_VERSION"));
    let mut result = Object::new();
    result.text("protocolVersion", version);
    result.json("capabilities", &capabilities.end());
    result.json("serverInfo", &info.end());
    result.end()
}

/// One tool: a verb of the table, with its help as description and an input schema built from
/// its arguments.
fn tool(verb: &VerbSpec, apps: &[&str]) -> String {
    let mut properties = Object::new();
    for arg in verb.args {
        let mut property = Object::new();
        match arg.kind {
            ArgKind::Id | ArgKind::Text => property.text("type", "string"),
            ArgKind::Name => {
                property.text("type", "string");
                if verb.name == schema::START.name {
                    property.json("enum", &strings(apps.iter().copied()));
                }
            }
            ArgKind::Key => {
                property.text("type", "string");
                property.json("enum", &strings(KEY_NAMES));
            }
            ArgKind::Flag => property.text("type", "boolean"),
            ArgKind::Milliseconds => {
                property.text("type", "integer");
                property.count("minimum", 1);
                property.count("maximum", u64::from(MAX_MILLISECONDS));
            }
        }
        property.text(
            "description",
            &format!("{} ({})", arg.help, arg.kind.bound()),
        );
        properties.json(arg.name, &property.end());
    }
    let mut input_schema = Object::new();
    input_schema.text("type", "object");
    input_schema.json("properties", &properties.end());
    input_schema.json(
        "required",
        &strings(
            verb.args
                .iter()
                .filter(|arg| arg.required)
                .map(|arg| arg.name),
        ),
    );
    input_schema.flag("additionalProperties", false);
    let mut object = Object::new();
    object.text("name", verb.name);
    object.text("description", verb.help);
    object.json("inputSchema", &input_schema.end());
    object.end()
}

/// What `tools/list` answers: every verb of the table as a tool, in the table's order.
fn tool_list(apps: &[&str]) -> String {
    let mut result = Object::new();
    result.json("tools", &list(VERBS.iter().map(|verb| tool(verb, apps))));
    result.end()
}

/// The call a tool call's arguments spell: one argument per member. A value that is neither
/// text, a whole number nor a flag refuses the call as malformed.
fn call_of(verb: &'static VerbSpec, arguments: &Map<String, Value>) -> Result<Call, Refusal> {
    let mut args = Vec::with_capacity(arguments.len());
    for (name, value) in arguments {
        let value = match value {
            Value::String(text) => Some(ArgValue::Text(text.clone())),
            Value::Bool(flag) => Some(ArgValue::Flag(*flag)),
            Value::Number(number) => number.as_i64().map(ArgValue::Number),
            Value::Null | Value::Array(_) | Value::Object(_) => None,
        };
        let Some(value) = value else {
            let fault = match verb.args.iter().find(|arg| arg.name == name) {
                Some(arg) => Fault::WrongKind(arg.name),
                None => Fault::Unnamed,
            };
            return Err(Refusal::malformed(fault));
        };
        args.push((name.clone(), value));
    }
    Ok(Call {
        verb: verb.name.to_string(),
        args,
    })
}

/// How a tool call ended, as its span records it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    Accepted,
    Refused,
    Error,
}

impl Ending {
    const fn word(self) -> &'static str {
        match self {
            Ending::Accepted => "accepted",
            Ending::Refused => "refused",
            Ending::Error => "error",
        }
    }
}

/// A tool call's result: its one text block and how it ended.
#[derive(Debug, Clone, PartialEq)]
struct ToolResult {
    text: String,
    ending: Ending,
}

impl ToolResult {
    fn accepted(text: String) -> ToolResult {
        ToolResult {
            text,
            ending: Ending::Accepted,
        }
    }

    fn refused(refusal: Refusal) -> ToolResult {
        ToolResult {
            text: refusal.to_json(),
            ending: Ending::Refused,
        }
    }

    fn failed(error: SessionError) -> ToolResult {
        ToolResult {
            text: error.to_json(),
            ending: Ending::Error,
        }
    }

    /// The result, or the session's answer-size error in its place when its text is over the
    /// bound.
    fn bounded(self) -> ToolResult {
        if self.text.len() > MAX_ANSWER_BYTES {
            ToolResult::failed(SessionError::AnswerTooLarge)
        } else {
            self
        }
    }

    fn written(&self) -> String {
        let mut block = Object::new();
        block.text("type", "text");
        block.text("text", &self.text);
        let mut result = Object::new();
        result.json("content", &list([block.end()]));
        result.flag("isError", self.ending != Ending::Accepted);
        result.end()
    }
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
                match held.session.run(call) {
                    Ok(Outcome::Screen { text }) => ToolResult::accepted(text),
                    Ok(outcome) => ToolResult::accepted(outcome.to_json()),
                    Err(refusal) => ToolResult::refused(refusal),
                }
            }
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
mod tests {
    use std::io::BufReader;

    use blitz_test_harness::Harness;

    use super::*;
    use crate::refusal::Cause;
    use crate::schema::ArgSpec;

    const APPS: [&str; 2] = ["counter", "crud"];

    fn never(_: &str) -> Result<Session, SessionError> {
        Err(SessionError::InvalidLabel)
    }

    /// A session over an empty app: no virtual DOM of a stand task is needed.
    fn empty(app: &str) -> Result<Session, SessionError> {
        Session::start(app, || Harness::from_component(|| Ok(Default::default())))
    }

    /// The replies `serve_mcp` writes for `input`, one per line.
    fn exchange(
        input: &[u8],
        boot: impl FnMut(&str) -> Result<Session, SessionError>,
        idle_expiry: Duration,
    ) -> Vec<String> {
        let mut output = Vec::new();
        serve_mcp(input, &mut output, &APPS, boot, idle_expiry).expect("the exchange ends");
        let text = String::from_utf8(output).expect("replies are text");
        assert!(text.is_empty() || text.ends_with('\n'));
        text.lines().map(str::to_string).collect()
    }

    fn request(id: &str, method: &str, params: &str) -> String {
        format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#)
    }

    fn tool_call(id: u32, name: &str, arguments: &str) -> String {
        request(
            &id.to_string(),
            "tools/call",
            &format!(r#"{{"name":"{name}","arguments":{arguments}}}"#),
        )
    }

    fn error(id: &str, code: i32, message: &str) -> String {
        format!(r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":"{message}"}}}}"#)
    }

    fn tool_reply(id: u32, result: &ToolResult) -> String {
        result_reply(&Id::Number(i64::from(id)), &result.written())
    }

    fn lines(requests: &[String]) -> Vec<u8> {
        let mut input = requests.join("\n");
        input.push('\n');
        input.into_bytes()
    }

    #[test]
    fn each_line_is_read_by_its_rules() {
        let at_bound = format!(
            "{}{}",
            request("1", "ping", "{}"),
            " ".repeat(MAX_MCP_REQUEST_BYTES - request("1", "ping", "{}").len())
        );
        assert_eq!(at_bound.len(), MAX_MCP_REQUEST_BYTES);
        let over_bound = format!("{at_bound} ");
        let parse = error("null", -32700, "parse error");
        let invalid = error("null", -32600, "invalid request");
        let pong = result_reply(&Id::Number(1), "{}");
        let rows: Vec<(Vec<u8>, Vec<String>)> = vec![
            (over_bound.into_bytes(), vec![parse.clone()]),
            (at_bound.into_bytes(), vec![pong.clone()]),
            (Vec::new(), Vec::new()),
            (vec![b'{', 0xff, b'}'], vec![parse.clone()]),
            (b"ping".to_vec(), vec![parse.clone()]),
            (
                br#"{"jsonrpc":"2.0","id":1,"method":"ping""#.to_vec(),
                vec![parse],
            ),
            (br#"[1,2]"#.to_vec(), vec![invalid.clone()]),
            (br#""ping""#.to_vec(), vec![invalid.clone()]),
            (
                br#"{"id":1,"method":"ping"}"#.to_vec(),
                vec![error("1", -32600, "invalid request")],
            ),
            (
                br#"{"jsonrpc":"1.0","id":1,"method":"ping"}"#.to_vec(),
                vec![error("1", -32600, "invalid request")],
            ),
            (
                br#"{"jsonrpc":"2.0","id":1,"method":7}"#.to_vec(),
                vec![error("1", -32600, "invalid request")],
            ),
            (br#"{"jsonrpc":"2.0","method":"ping"}"#.to_vec(), Vec::new()),
            (
                br#"{"jsonrpc":"2.0","method":"tools/call","params":{"name":"stop"}}"#.to_vec(),
                Vec::new(),
            ),
            (
                br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#.to_vec(),
                Vec::new(),
            ),
            (
                format!("{}\r", request("1", "ping", "{}")).into_bytes(),
                vec![pong],
            ),
        ];
        assert_eq!(rows.len(), 15);
        for (row, (line, replies)) in rows.into_iter().enumerate() {
            let mut input = line;
            input.push(b'\n');
            let read = exchange(&input, never, Duration::MAX);
            assert!(read == replies, "row {row}");
        }
    }

    #[test]
    fn a_line_is_read_whole_across_reads_and_an_unended_last_line_is_one() {
        let first = request("1", "ping", "{}");
        let second = request("2", "ping", "{}");
        let joined = format!("{first}\n{second}\n");
        let rows: [(usize, &str, usize); 4] = [
            (8192, &joined, 2),
            (7, &joined, 2),
            (1, &joined, 2),
            (5, joined.trim_end(), 2),
        ];
        assert_eq!(rows.len(), 4);
        for (row, (capacity, input, count)) in rows.into_iter().enumerate() {
            let reader = BufReader::with_capacity(capacity, input.as_bytes());
            let mut output = Vec::new();
            serve_mcp(reader, &mut output, &APPS, never, Duration::MAX).expect("it ends");
            let text = String::from_utf8(output).expect("replies are text");
            let replies: Vec<&str> = text.lines().collect();
            assert!(replies.len() == count, "row {row}");
            assert!(
                replies[0] == result_reply(&Id::Number(1), "{}")
                    && replies[1] == result_reply(&Id::Number(2), "{}"),
                "row {row}"
            );
        }
        // An over-bound line read in small pieces is dropped whole, and the next line is read.
        let long = format!("{}\n{first}\n", "x".repeat(MAX_MCP_REQUEST_BYTES + 10));
        let reader = BufReader::with_capacity(64, long.as_bytes());
        let mut output = Vec::new();
        serve_mcp(reader, &mut output, &APPS, never, Duration::MAX).expect("it ends");
        assert_eq!(
            String::from_utf8(output).expect("replies are text"),
            format!(
                "{}\n{}\n",
                error("null", -32700, "parse error"),
                result_reply(&Id::Number(1), "{}")
            )
        );
    }

    #[test]
    fn an_id_is_echoed_only_in_its_two_forms() {
        let edge = "i".repeat(MAX_ID_BYTES);
        let over = "i".repeat(MAX_ID_BYTES + 1);
        let echo = |id: &str| (id.to_string(), Some(id.to_string()));
        let rows: Vec<(String, Option<String>)> = vec![
            echo("7"),
            echo("0"),
            echo("-3"),
            echo("9223372036854775807"),
            echo(r#""a-1""#),
            echo(r#""""#),
            echo(r#""a\"\n""#),
            echo(&format!("\"{edge}\"")),
            (format!("\"{over}\""), None),
            ("9223372036854775808".to_string(), None),
            ("1.5".to_string(), None),
            ("1e3".to_string(), None),
            ("null".to_string(), None),
            ("true".to_string(), None),
            ("[1]".to_string(), None),
            (r#"{"a":1}"#.to_string(), None),
        ];
        assert_eq!(rows.len(), 16);
        for (row, (id, echoed)) in rows.into_iter().enumerate() {
            let read = exchange(&lines(&[request(&id, "ping", "{}")]), never, Duration::MAX);
            let expected = match echoed {
                Some(echoed) => format!(r#"{{"jsonrpc":"2.0","id":{echoed},"result":{{}}}}"#),
                None => error("null", -32600, "invalid request"),
            };
            assert!(read == [expected], "row {row}");
        }
    }

    #[test]
    fn each_method_gets_its_reply() {
        let version = |asked: &str, answered: &str| {
            (
                request(
                    "1",
                    "initialize",
                    &format!(
                        r#"{{"protocolVersion":"{asked}","capabilities":{{}},"clientInfo":{{"name":"c","version":"9"}}}}"#
                    ),
                ),
                result_reply(
                    &Id::Number(1),
                    &format!(
                        r#"{{"protocolVersion":"{answered}","capabilities":{{"tools":{{}}}},"serverInfo":{{"name":"escher","version":"{}"}}}}"#,
                        env!("CARGO_PKG_VERSION")
                    ),
                ),
            )
        };
        let not_found = error("1", -32601, "method not found");
        let invalid_params = error("1", -32602, "invalid params");
        let rows: Vec<(String, String)> = vec![
            version("2025-06-18", "2025-06-18"),
            version("2025-03-26", "2025-03-26"),
            version("2024-11-05", "2024-11-05"),
            version("2026-07-28", "2025-06-18"),
            version("2099-01-01", "2025-06-18"),
            (
                r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#.to_string(),
                result_reply(&Id::Number(1), &initialized(None)),
            ),
            (
                request("1", "ping", "{}"),
                result_reply(&Id::Number(1), "{}"),
            ),
            (
                request("1", "tools/list", "{}"),
                result_reply(&Id::Number(1), &tool_list(&APPS)),
            ),
            (request("1", "server/discover", "{}"), not_found.clone()),
            (request("1", "resources/list", "{}"), not_found.clone()),
            (
                request("1", "notifications/initialized", "{}"),
                not_found.clone(),
            ),
            (request("1", "Ping", "{}"), not_found),
            (request("1", "tools/call", "{}"), invalid_params.clone()),
            (request("1", "tools/call", "[]"), invalid_params.clone()),
            (
                request("1", "tools/call", r#"{"name":7}"#),
                invalid_params.clone(),
            ),
            (
                request("1", "tools/call", r#"{"name":"stop","arguments":[]}"#),
                invalid_params.clone(),
            ),
            (
                request("1", "tools/call", r#"{"name":"stop","arguments":null}"#),
                invalid_params,
            ),
            (
                tool_call(1, "screenshot", "{}"),
                error("1", -32602, "unknown tool"),
            ),
            (
                tool_call(1, "Stop", "{}"),
                error("1", -32602, "unknown tool"),
            ),
        ];
        assert_eq!(rows.len(), 19);
        for (row, (sent, answered)) in rows.into_iter().enumerate() {
            let read = exchange(&lines(&[sent]), never, Duration::MAX);
            assert!(read == [answered], "row {row}");
        }
    }

    #[test]
    fn each_argument_value_becomes_its_kind_or_refuses_the_call() {
        let click = &schema::CLICK;
        let text = |value: &str| ArgValue::Text(value.to_string());
        let call = |args: Vec<(&str, ArgValue)>| {
            Ok(Call {
                verb: "click".to_string(),
                args: args
                    .into_iter()
                    .map(|(name, value)| (name.to_string(), value))
                    .collect(),
            })
        };
        let rows: Vec<(&str, Result<Call, Refusal>)> = vec![
            (r#"{}"#, call(vec![])),
            (r#"{"id":"save"}"#, call(vec![("id", text("save"))])),
            (r#"{"id":""}"#, call(vec![("id", text(""))])),
            (r#"{"id":5}"#, call(vec![("id", ArgValue::Number(5))])),
            (r#"{"id":-5}"#, call(vec![("id", ArgValue::Number(-5))])),
            (r#"{"id":true}"#, call(vec![("id", ArgValue::Flag(true))])),
            (r#"{"id":false}"#, call(vec![("id", ArgValue::Flag(false))])),
            (r#"{"x":"save"}"#, call(vec![("x", text("save"))])),
            (
                r#"{"id":null}"#,
                Err(Refusal::malformed(Fault::WrongKind("id"))),
            ),
            (
                r#"{"id":1.5}"#,
                Err(Refusal::malformed(Fault::WrongKind("id"))),
            ),
            (
                r#"{"id":18446744073709551615}"#,
                Err(Refusal::malformed(Fault::WrongKind("id"))),
            ),
            (
                r#"{"id":["save"]}"#,
                Err(Refusal::malformed(Fault::WrongKind("id"))),
            ),
            (
                r#"{"id":{"a":1}}"#,
                Err(Refusal::malformed(Fault::WrongKind("id"))),
            ),
            (r#"{"x":null}"#, Err(Refusal::malformed(Fault::Unnamed))),
            (r#"{"x":[]}"#, Err(Refusal::malformed(Fault::Unnamed))),
        ];
        assert_eq!(rows.len(), 15);
        for (row, (arguments, expected)) in rows.into_iter().enumerate() {
            let Ok(Value::Object(arguments)) = serde_json::from_str::<Value>(arguments) else {
                panic!("row {row}: the arguments are an object");
            };
            assert!(call_of(click, &arguments) == expected, "row {row}");
        }
        // A refused value refuses the call through the protocol as a tool result, run nothing.
        let read = exchange(
            &lines(&[tool_call(1, "click", r#"{"id":null}"#)]),
            never,
            Duration::MAX,
        );
        assert_eq!(
            read,
            [tool_reply(
                1,
                &ToolResult::refused(Refusal::malformed(Fault::WrongKind("id")))
            )]
        );
    }

    /// A tool's input schema as `tool` should write it for `verb`, read off the table.
    fn listed(verb: &VerbSpec) -> (Vec<&'static str>, Vec<&'static str>, Vec<&'static str>) {
        let names = verb.args.iter().map(|arg| arg.name).collect();
        let required = verb
            .args
            .iter()
            .filter(|arg| arg.required)
            .map(|arg| arg.name)
            .collect();
        let types = verb
            .args
            .iter()
            .map(|arg: &ArgSpec| match arg.kind {
                ArgKind::Id | ArgKind::Text | ArgKind::Key | ArgKind::Name => "string",
                ArgKind::Flag => "boolean",
                ArgKind::Milliseconds => "integer",
            })
            .collect();
        (names, required, types)
    }

    #[test]
    fn the_listing_is_the_nine_verbs_of_the_table() {
        let Ok(Value::Object(result)) = serde_json::from_str::<Value>(&tool_list(&APPS)) else {
            panic!("the listing is an object");
        };
        let Some(Value::Array(tools)) = result.get("tools") else {
            panic!("the listing holds a list of tools");
        };
        assert_eq!(result.len(), 1, "no member beside the tools, no cursor");
        assert_eq!(tools.len(), VERBS.len());
        assert_eq!(VERBS.len(), 9);
        for (row, (listed_tool, verb)) in tools.iter().zip(VERBS).enumerate() {
            assert!(listed_tool["name"] == verb.name, "row {row}");
            assert!(listed_tool["description"] == verb.help, "row {row}");
            let schema = &listed_tool["inputSchema"];
            assert!(schema["type"] == "object", "row {row}");
            assert!(schema["additionalProperties"] == false, "row {row}");
            let (names, required, types) = listed(verb);
            let Some(Value::Object(properties)) = schema.get("properties") else {
                panic!("row {row}: the schema holds its properties");
            };
            // serde_json reads an object's members in key order.
            let mut sorted = names.clone();
            sorted.sort_unstable();
            assert!(
                properties.keys().cloned().collect::<Vec<_>>() == sorted,
                "row {row}"
            );
            // The written form keeps the table's order.
            let written = tool(verb, &APPS);
            let order: Vec<usize> = names
                .iter()
                .map(|name| {
                    written
                        .find(&format!("\"{name}\":{{\"type\""))
                        .expect("each property is written")
                })
                .collect();
            assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "row {row}");
            assert!(schema["required"] == Value::from(required), "row {row}");
            for ((name, kind), arg) in names.iter().zip(&types).zip(verb.args) {
                let property = &properties[*name];
                assert!(property["type"] == *kind, "row {row}");
                assert!(
                    property["description"] == format!("{} ({})", arg.help, arg.kind.bound()),
                    "row {row}"
                );
                let listed_enum = match arg.kind {
                    ArgKind::Key => Some(KEY_NAMES.to_vec()),
                    ArgKind::Name => Some(APPS.to_vec()),
                    _ => None,
                };
                match listed_enum {
                    Some(values) => assert!(property["enum"] == Value::from(values), "row {row}"),
                    None => assert!(property.get("enum").is_none(), "row {row}"),
                }
                let bounded = arg.kind == ArgKind::Milliseconds;
                assert!(
                    (property.get("minimum") == Some(&Value::from(1))) == bounded
                        && (property.get("maximum") == Some(&Value::from(MAX_MILLISECONDS)))
                            == bounded,
                    "row {row}"
                );
            }
        }
    }

    #[test]
    fn the_session_verbs_are_answered_by_the_server() {
        let hello = |served: u64| Hello {
            pid: std::process::id(),
            label: "counter".to_string(),
            served,
            idle_expiry_s: 1800,
        };
        let idle = Duration::from_secs(1800);
        let no_session = ToolResult::failed(SessionError::NoSession);
        let rows: Vec<(Vec<String>, Vec<ToolResult>)> = vec![
            // Nothing held: every verb but `start` is a session error, and a refused call is
            // still checked first.
            (
                vec![
                    tool_call(1, "status", "{}"),
                    tool_call(2, "stop", "{}"),
                    tool_call(3, "snapshot", "{}"),
                    tool_call(4, "click", r#"{"id":"save"}"#),
                    tool_call(5, "click", "{}"),
                    tool_call(6, "start", r#"{"app":"timer"}"#),
                    tool_call(7, "start", r#"{"app":"Counter"}"#),
                    tool_call(8, "start", "{}"),
                    tool_call(9, "start", r#"{"app":"counter"}"#),
                ],
                vec![
                    no_session.clone(),
                    no_session.clone(),
                    no_session.clone(),
                    no_session.clone(),
                    ToolResult::refused(Refusal::malformed(Fault::Missing("id"))),
                    ToolResult::refused(Refusal::malformed(Fault::OutOfBound("app"))),
                    ToolResult::refused(Refusal::malformed(Fault::OutOfBound("app"))),
                    ToolResult::refused(Refusal::malformed(Fault::Missing("app"))),
                    ToolResult::failed(SessionError::InvalidLabel),
                ],
            ),
        ];
        assert_eq!(rows.len(), 1);
        for (row, (sent, results)) in rows.into_iter().enumerate() {
            let read = exchange(&lines(&sent), never, idle);
            let expected: Vec<String> = results
                .iter()
                .enumerate()
                .map(|(index, result)| tool_reply(index as u32 + 1, result))
                .collect();
            assert!(read == expected, "row {row}");
        }

        // A held session: `start` answers its label, `status` the calls answered since,
        // a second `start` is already running, and `stop` drops it.
        let sent = [
            tool_call(1, "start", r#"{"app":"counter"}"#),
            tool_call(2, "status", "{}"),
            tool_call(3, "start", r#"{"app":"crud"}"#),
            tool_call(4, "snapshot", "{}"),
            tool_call(5, "click", r#"{"id":"names-nothing"}"#),
            tool_call(6, "status", "{}"),
            tool_call(7, "stop", "{}"),
            tool_call(8, "status", "{}"),
        ];
        let read = exchange(&lines(&sent), empty, idle);
        assert_eq!(read.len(), sent.len());
        let expected = [
            ToolResult::accepted(json::started(&hello(0))),
            ToolResult::accepted(json::status(&hello(0))),
            ToolResult::failed(SessionError::AlreadyRunning),
            ToolResult::accepted(String::new()),
            ToolResult::refused(Refusal::new(Cause::NotFound)),
            ToolResult::accepted(json::status(&hello(4))),
            ToolResult::accepted(json::stopped()),
            ToolResult::failed(SessionError::NoSession),
        ];
        assert_eq!(expected.len(), 8);
        for (index, result) in expected.iter().enumerate() {
            if index == 3 {
                // The empty app's screen: a text block, not marked as an error.
                assert!(read[index].contains(r#""isError":false"#), "call {index}");
                assert!(!read[index].contains(r#"{\"text\""#), "call {index}");
                continue;
            }
            assert!(
                read[index] == tool_reply(index as u32 + 1, result),
                "call {index}"
            );
        }
    }

    #[test]
    fn a_session_idle_for_its_expiry_is_dropped_when_a_request_arrives() {
        let sent = [
            tool_call(1, "start", r#"{"app":"counter"}"#),
            tool_call(2, "status", "{}"),
        ];
        let read = exchange(&lines(&sent), empty, Duration::ZERO);
        assert_eq!(read.len(), 2);
        assert!(read[0].contains(r#"\"label\":\"counter\""#));
        assert_eq!(
            read[1],
            tool_reply(2, &ToolResult::failed(SessionError::NoSession))
        );
        // Within the expiry the same session answers.
        let read = exchange(&lines(&sent), empty, Duration::from_secs(1800));
        assert!(read[1].contains(r#"\"served\":0"#));
    }

    #[test]
    fn a_result_over_the_answer_bound_is_replaced_by_the_session_error() {
        let rows = [
            (MAX_ANSWER_BYTES, Ending::Accepted),
            (MAX_ANSWER_BYTES + 1, Ending::Error),
        ];
        assert_eq!(rows.len(), 2);
        for (row, (length, ending)) in rows.into_iter().enumerate() {
            let bounded = ToolResult::accepted("x".repeat(length)).bounded();
            assert!(bounded.ending == ending, "row {row}");
        }
        assert_eq!(
            ToolResult::accepted("x".repeat(MAX_ANSWER_BYTES + 1)).bounded(),
            ToolResult::failed(SessionError::AnswerTooLarge)
        );
        let written = ToolResult::refused(Refusal::new(Cause::Stale)).written();
        assert!(written.ends_with(r#""isError":true}"#));
        assert!(!written.contains('\n'));
    }
}
