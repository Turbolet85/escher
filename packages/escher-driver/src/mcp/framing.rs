//! Framing and bounds: a request line read under its byte bound, the request's envelope and
//! its `id` in the forms a reply echoes, the fixed protocol errors, and the reply writers.

use std::io::{self, BufRead, Write};

use serde_json::Value;

use crate::json::{self, Object};

/// The longest request line the server reads, in bytes, its terminator not counted.
pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;

/// The longest string `id` a reply echoes, in bytes.
pub(super) const MAX_ID_BYTES: usize = 128;

/// What one read of the input returned.
#[derive(Debug, PartialEq)]
pub(super) enum Request {
    /// A line within the bound, its terminator removed.
    Line(Vec<u8>),
    /// A line over the bound, read to its terminator and dropped.
    TooLong,
    /// The input ended.
    End,
}

/// Reads one request line from `input`, which keeps whatever it read ahead for the next one.
/// A last line with no terminator is a line.
pub(super) fn read_request(input: &mut impl BufRead) -> io::Result<Request> {
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
pub(super) enum Id {
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
pub(super) enum ProtocolError {
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
pub(super) enum Envelope {
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
pub(super) fn envelope(line: &[u8]) -> Envelope {
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

pub(super) fn result_reply(id: &Id, result: &str) -> String {
    reply(id, "result", result)
}

pub(super) fn error_reply(id: &Id, error: ProtocolError) -> String {
    let mut object = Object::new();
    object.json("code", &error.code().to_string());
    object.text("message", error.message());
    reply(id, "error", &object.end())
}

pub(super) fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {
    output.write_all(line.as_bytes())?;
    output.write_all(b"\n")?;
    output.flush()
}

/// A list of already written values.
pub(super) fn list(items: impl IntoIterator<Item = String>) -> String {
    let items: Vec<String> = items.into_iter().collect();
    format!("[{}]", items.join(","))
}
