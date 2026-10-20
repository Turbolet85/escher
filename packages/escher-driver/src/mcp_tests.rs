//! Drives `serve_mcp` end to end: each request line read by its rules, the `id` echo, the
//! protocol errors, the handshake, the tool listing read against the verb table, the tool
//! results and their bound. A child module of `mcp`, because the framing, listing and result
//! items it reads are private to it.

use std::io::BufReader;

use blitz_test_harness::Harness;

use super::framing::{MAX_ID_BYTES, MAX_MCP_REQUEST_BYTES};
use super::listing::{description, tool};
use super::*;
use crate::command::ArgValue;
use crate::refusal::Cause;
use crate::schema::{ArgKind, ArgSpec, KEY_NAMES, MAX_MILLISECONDS, VERBS};
use crate::wire::MAX_ANSWER_BYTES;

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
fn read_request_takes_each_line_with_its_terminator() {
    let input: &[u8] = b"a\n\nb\n";
    let expected = [
        Request::Line(b"a".to_vec()),
        Request::Line(Vec::new()),
        Request::Line(b"b".to_vec()),
        Request::End,
    ];
    let readers: Vec<Box<dyn BufRead>> = vec![
        Box::new(input),
        Box::new(BufReader::with_capacity(1, input)),
    ];
    assert_eq!(readers.len(), 2);
    for (row, mut reader) in readers.into_iter().enumerate() {
        let mut reads = Vec::new();
        for _ in 0..=expected.len() {
            let read = read_request(&mut reader).expect("a slice reads");
            let ended = read == Request::End;
            reads.push(read);
            if ended {
                break;
            }
        }
        for (index, due) in expected.iter().enumerate() {
            assert!(reads.get(index) == Some(due), "reader {row} read {index}");
        }
        assert!(reads.len() == expected.len(), "reader {row}");
    }
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
        version("2025-11-25", "2025-11-25"),
        version("2025-06-18", "2025-06-18"),
        version("2025-03-26", "2025-03-26"),
        version("2024-11-05", "2024-11-05"),
        version("2026-07-28", "2025-11-25"),
        version("2099-01-01", "2025-11-25"),
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
            tool_call(1, "capture", "{}"),
            error("1", -32602, "unknown tool"),
        ),
        (
            tool_call(1, "Stop", "{}"),
            error("1", -32602, "unknown tool"),
        ),
    ];
    assert_eq!(rows.len(), 20);
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
fn the_listing_is_the_twelve_verbs_of_the_table() {
    let Ok(Value::Object(result)) = serde_json::from_str::<Value>(&tool_list(&APPS)) else {
        panic!("the listing is an object");
    };
    let Some(Value::Array(tools)) = result.get("tools") else {
        panic!("the listing holds a list of tools");
    };
    assert_eq!(result.len(), 1, "no member beside the tools, no cursor");
    assert_eq!(tools.len(), VERBS.len());
    assert_eq!(VERBS.len(), 12);
    assert_eq!(
        tools.last().map(|tool| &tool["name"]),
        Some(&Value::from("help"))
    );
    for (row, (listed_tool, verb)) in tools.iter().zip(VERBS).enumerate() {
        assert!(listed_tool["name"] == verb.name, "row {row}");
        let Some(Value::String(described)) = listed_tool.get("description") else {
            panic!("row {row}: each tool is described");
        };
        assert!(described == &description(verb), "row {row}");
        assert!(described.starts_with(verb.help), "row {row}");
        for field in verb.fields {
            let named = format!("{} ({}): {}", field.name, field.kind.name(), field.help);
            assert!(described.contains(&named), "row {row}");
        }
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
                    && (property.get("maximum") == Some(&Value::from(MAX_MILLISECONDS))) == bounded,
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
                tool_call(9, "lint", "{}"),
                tool_call(10, "lint", r#"{"id":"save"}"#),
                tool_call(11, "start", r#"{"app":"counter"}"#),
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
                no_session.clone(),
                ToolResult::refused(Refusal::malformed(Fault::Unnamed)),
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
fn help_is_answered_by_the_server_with_or_without_a_session() {
    let help = ToolResult::accepted(describe());
    let sent = [
        tool_call(1, "help", "{}"),
        tool_call(2, "help", r#"{"verb":"click"}"#),
        tool_call(3, "start", r#"{"app":"counter"}"#),
        tool_call(4, "help", "{}"),
    ];
    let read = exchange(&lines(&sent), empty, Duration::from_secs(1800));
    assert_eq!(read.len(), 4);
    assert_eq!(read[0], tool_reply(1, &help));
    assert!(read[0].contains(r#""isError":false"#));
    assert_eq!(
        read[1],
        tool_reply(2, &ToolResult::refused(Refusal::malformed(Fault::Unnamed)))
    );
    assert_eq!(read[3], tool_reply(4, &help));
    // With nothing to boot, `help` still answers.
    let read = exchange(&lines(&sent[..1]), never, Duration::MAX);
    assert_eq!(read, [tool_reply(1, &help)]);
}

#[cfg(not(unix))]
#[test]
fn off_unix_a_recording_boot_answers_start_as_unsupported() {
    let boot = |app: &str| empty(app)?.recording_into(std::path::Path::new("kept-off-unix"));
    let sent = [tool_call(1, "start", r#"{"app":"counter"}"#)];
    let read = exchange(&lines(&sent), boot, Duration::from_secs(1800));
    assert_eq!(
        read,
        [tool_reply(
            1,
            &ToolResult::failed(SessionError::Unsupported)
        )]
    );
}

/// A fresh path under the workspace's `target/tmp`, named for the test: nothing is there.
#[cfg(unix)]
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp"))
        .join(format!("escher-driver-mcp-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.parent().expect("a parent")).expect("target/tmp is created");
    dir
}

#[cfg(unix)]
#[test]
fn each_session_a_recording_boot_starts_is_kept_as_one_check() {
    use crate::recording::{Check, answer_digest};

    let dir = scratch("recorded");
    let boot = |app: &str| empty(app)?.recording_into(&dir);
    let sent = [
        tool_call(1, "start", r#"{"app":"counter"}"#),
        tool_call(2, "snapshot", "{}"),
        tool_call(3, "click", r#"{"id":"names-nothing"}"#),
        tool_call(4, "status", "{}"),
        tool_call(5, "stop", "{}"),
        tool_call(6, "start", r#"{"app":"crud"}"#),
        tool_call(7, "help", "{}"),
        tool_call(8, "stop", "{}"),
    ];
    let read = exchange(&lines(&sent), boot, Duration::from_secs(1800));
    assert_eq!(read.len(), sent.len());
    assert!(read[1].contains(r#""isError":false"#));
    assert_eq!(
        read[2],
        tool_reply(3, &ToolResult::refused(Refusal::new(Cause::NotFound)))
    );

    let first = std::fs::read(dir.join("check-1.json")).expect("the first session's check");
    let first = Check::from_bytes(&first).expect("it reads back");
    assert_eq!(first.app(), "counter");
    let verbs: Vec<&str> = first
        .steps()
        .iter()
        .map(|step| step.call().verb.as_str())
        .collect();
    assert_eq!(verbs, ["snapshot", "click"]);
    let not_found = Err(Refusal::new(Cause::NotFound));
    assert!(!first.steps()[1].accepted());
    assert_eq!(first.steps()[1].digest(), answer_digest(&not_found));

    let second = std::fs::read(dir.join("check-2.json")).expect("the second session's check");
    let second = Check::from_bytes(&second).expect("it reads back");
    assert_eq!(second.app(), "crud");
    assert!(second.steps().is_empty());
    assert_eq!(std::fs::read_dir(&dir).expect("listed").count(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn a_call_that_ends_the_recording_is_answered_as_its_session_error() {
    let dir = scratch("ends");
    let boot = |app: &str| empty(app)?.recording_into(&dir);
    std::fs::create_dir(&dir).expect("the directory is created");
    std::fs::set_permissions(
        &dir,
        <std::fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o700),
    )
    .expect("made private");
    // The rewrite's temporary path taken: the first recorded call cannot be written.
    std::fs::write(dir.join(".check-1.json.tmp"), "").expect("the path is taken");
    let sent = [
        tool_call(1, "start", r#"{"app":"counter"}"#),
        tool_call(2, "snapshot", "{}"),
        tool_call(3, "snapshot", "{}"),
    ];
    let read = exchange(&lines(&sent), boot, Duration::from_secs(1800));
    assert_eq!(read.len(), 3);
    assert_eq!(
        read[1],
        tool_reply(2, &ToolResult::failed(SessionError::RecordingFailed))
    );
    assert!(read[1].contains(r#""isError":true"#));
    // The session goes on, recording nothing further.
    assert!(read[2].contains(r#""isError":false"#));
    assert!(!dir.join("check-1.json").exists());
    assert_eq!(std::fs::read_dir(&dir).expect("listed").count(), 1);
    let _ = std::fs::remove_dir_all(&dir);
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

    // A result that holds an image is bounded by its whole written form.
    let fits = MAX_ANSWER_BYTES / 4 * 3 - 300;
    assert_eq!(
        ToolResult::shot(1, 1, &vec![0; fits]).bounded().ending,
        Ending::Accepted
    );
    assert_eq!(
        ToolResult::shot(1, 1, &vec![0; MAX_ANSWER_BYTES / 4 * 3]).bounded(),
        ToolResult::failed(SessionError::AnswerTooLarge)
    );
}

#[test]
fn a_shot_is_written_as_its_image_block_then_its_text_block() {
    let written = ToolResult::shot(800, 600, b"foobar").written();
    assert_eq!(
        written,
        r#"{"content":[{"type":"image","data":"Zm9vYmFy","mimeType":"image/png"},{"type":"text","text":"{\"width\":800,\"height\":600}"}],"isError":false}"#
    );
    let Ok(Value::Object(result)) = serde_json::from_str::<Value>(&written) else {
        panic!("the result is an object");
    };
    let Some(Value::Array(content)) = result.get("content") else {
        panic!("the result holds its content");
    };
    assert_eq!(content.len(), 2);
    assert_eq!(content[0]["type"], "image");
    assert_eq!(content[1]["type"], "text");
    let refused = ToolResult::refused(Refusal::new(Cause::FontsUnavailable)).written();
    assert!(refused.starts_with(r#"{"content":[{"type":"text","#));
    assert!(refused.ends_with(r#""isError":true}"#));
}
