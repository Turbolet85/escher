//! The `escher-session` binary in its `mcp` role, run as a child the way an MCP client runs it:
//! it answers `initialize` in the revision asked for, lists exactly the nine verbs of the
//! driver's table as tools, and completes the Counter flow by tool calls alone — `start`, a
//! snapshot, three clicks, a snapshot reading 3, `status`, `stop`. A refused call, a malformed
//! call, an unknown method, a line that is not JSON, a line over the bound and an unknown tool
//! are each answered and run nothing, and no reply holds what they named. Stdout holds one
//! protocol line per request answered, with no ESC byte; closing stdin ends the process with
//! status 0. A fresh Counter's snapshot reads the same through `mcp` as through the command
//! line. A failure message names a kind, a call and a count — never a captured line.

#![cfg(not(target_arch = "wasm32"))]

mod common;

use std::process::Command;

use common::{Speaker, cause, number, string_at};
use escher_driver::VERBS;

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// An id no element of the stand carries.
const NOBODY: &str = "mcp-server-names-no-element";

/// A tool name the table lacks.
const UNKNOWN_TOOL: &str = "capture-mq4";

/// The byte an over-bound line is made of: no reply may hold a run of it.
const FILLER: &str = "Q";

/// The longest request line the server reads, in bytes.
const MAX_MCP_REQUEST_BYTES: usize = 65_536;

/// The Counter's display reading 3, as its snapshot line writes it.
const READS_THREE: &str = " \"3\" id=\"counter-value\" ";

fn mcp() -> Speaker {
    let mut command = Command::new(BINARY);
    command.arg("mcp").env_remove("RUST_LOG");
    Speaker::spawn(command)
}

fn tool_call(id: u32, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

/// One tool call's reply read off: whether it is marked as an error, and its text block.
#[track_caller]
fn result(reply: &str, id: u32) -> (bool, String) {
    assert!(
        reply.starts_with(&format!(
            r#"{{"jsonrpc":"2.0","id":{id},"result":{{"content":[{{"type":"text","text":""#
        )),
        "call {id}: the reply is a tool result for its request"
    );
    let is_error = if reply.ends_with(r#""isError":false}}"#) {
        false
    } else {
        assert!(
            reply.ends_with(r#""isError":true}}"#),
            "call {id}: the result says whether it is an error"
        );
        true
    };
    let text = string_at(reply, "text").expect("a tool result holds a text block");
    (is_error, text)
}

/// Sends a tool call and returns its result.
#[track_caller]
fn call(speaker: &mut Speaker, id: u32, name: &str, arguments: &str) -> (bool, String) {
    speaker.send(tool_call(id, name, arguments).as_bytes());
    result(&speaker.reply(), id)
}

/// Sends a tool call that must run and returns its text block.
#[track_caller]
fn accepted(speaker: &mut Speaker, id: u32, name: &str, arguments: &str) -> String {
    let (is_error, text) = call(speaker, id, name, arguments);
    assert!(!is_error, "call {id}: it ran");
    text
}

/// The string values a listing holds under `name`, in order: the tools' names.
fn names(listing: &str) -> Vec<String> {
    listing
        .split("\"name\":\"")
        .skip(1)
        .filter_map(|rest| rest.split_once('"').map(|(name, _)| name.to_string()))
        .collect()
}

#[test]
fn the_mcp_role_completes_the_counter_flow_by_tool_calls_alone() {
    let mut speaker = mcp();
    let mut answered = 0;

    speaker.send(
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"check","version":"1"}}}"#,
    );
    let initialized = speaker.reply();
    answered += 1;
    assert!(
        initialized
            .starts_with(r#"{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18","#),
        "initialize answers the revision asked for"
    );
    // A notification is not answered: the next reply is the next request's.
    speaker.send(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);

    speaker.send(br#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
    let listing = speaker.reply();
    answered += 1;
    assert!(
        listing.starts_with(r#"{"jsonrpc":"2.0","id":2,"result":{"tools":["#),
        "tools/list answers the tools"
    );
    let verbs: Vec<&str> = VERBS.iter().map(|verb| verb.name).collect();
    assert_eq!(verbs.len(), 9);
    assert!(
        names(&listing) == verbs,
        "the tools are the table's nine verbs in its order"
    );

    let started = accepted(&mut speaker, 3, "start", r#"{"app":"counter"}"#);
    assert!(
        string_at(&started, "label").as_deref() == Some("counter"),
        "start answers the app's name"
    );
    let first = accepted(&mut speaker, 4, "snapshot", "{}");
    assert!(
        first.contains(" \"0\" id=\"counter-value\" "),
        "the counter reads 0 at boot"
    );
    for id in 5..8 {
        let clicked = accepted(&mut speaker, id, "click", r#"{"id":"counter-increment"}"#);
        assert!(
            clicked.starts_with(r#"{"settled":true,"#),
            "call {id}: the click's result is its line of JSON"
        );
    }
    let three = accepted(&mut speaker, 8, "snapshot", "{}");
    assert!(three.contains(READS_THREE), "the counter reads 3");
    let status = accepted(&mut speaker, 9, "status", "{}");
    assert_eq!(
        number(&status, "served"),
        Some(5),
        "status counts the calls answered since start"
    );
    answered += 7;

    // A refused call and a malformed call run nothing.
    let (is_error, refused) = call(
        &mut speaker,
        10,
        "click",
        &format!(r#"{{"id":"{NOBODY}"}}"#),
    );
    assert!(
        is_error && cause(&refused) == Some("not-found"),
        "call 10: refused not-found"
    );
    assert!(
        accepted(&mut speaker, 11, "snapshot", "{}") == three,
        "call 11: nothing ran"
    );
    let (is_error, malformed) = call(&mut speaker, 12, "type", r#"{"id":"counter-increment"}"#);
    assert!(
        is_error && cause(&malformed) == Some("malformed"),
        "call 12: refused malformed"
    );
    assert!(
        accepted(&mut speaker, 13, "snapshot", "{}") == three,
        "call 13: nothing ran"
    );
    answered += 4;

    // Lines outside the protocol are answered with a fixed error and run nothing.
    let mut errors = Vec::new();
    speaker.send(br#"{"jsonrpc":"2.0","id":14,"method":"server/discover"}"#);
    errors.push((speaker.reply(), "14", -32601));
    speaker.send(b"this line is not JSON");
    errors.push((speaker.reply(), "null", -32700));
    let over = FILLER.repeat(MAX_MCP_REQUEST_BYTES + 1);
    speaker.send(over.as_bytes());
    errors.push((speaker.reply(), "null", -32700));
    speaker.send(tool_call(15, UNKNOWN_TOOL, "{}").as_bytes());
    errors.push((speaker.reply(), "15", -32602));
    for (row, (reply, id, code)) in errors.iter().enumerate() {
        assert!(
            reply.starts_with(&format!(
                r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":{code},"message":""#
            )),
            "row {row}: the fixed error for its line"
        );
    }
    answered += errors.len();
    assert!(
        accepted(&mut speaker, 16, "snapshot", "{}") == three,
        "call 16: nothing ran"
    );
    let stopped = accepted(&mut speaker, 17, "stop", "{}");
    assert_eq!(stopped, r#"{"stopped":true}"#, "stop answers stopped");
    answered += 2;

    let spoke = speaker.close();
    assert_eq!(
        spoke.status,
        Some(0),
        "closing stdin ends the process with status 0"
    );
    assert!(
        spoke.stdout.is_empty(),
        "{} stdout lines past the {answered} replies",
        spoke.stdout.len()
    );

    // No reply holds what a refused or rejected line named.
    let replies = [&refused, &malformed]
        .into_iter()
        .cloned()
        .chain(errors.iter().map(|(reply, _, _)| reply.clone()))
        .collect::<Vec<_>>();
    let leaks = replies
        .iter()
        .filter(|reply| {
            reply.contains(NOBODY)
                || reply.contains(UNKNOWN_TOOL)
                || reply.contains(&FILLER.repeat(16))
        })
        .count();
    assert_eq!(leaks, 0, "{leaks} replies hold what their request named");
}

#[cfg(unix)]
#[test]
fn a_fresh_counters_snapshot_reads_the_same_through_mcp_and_the_command_line() {
    use common::{Started, clear, client, state_dir, text};

    let mut speaker = mcp();
    accepted(&mut speaker, 1, "start", r#"{"app":"counter"}"#);
    let through_mcp = accepted(&mut speaker, 2, "snapshot", "{}");
    accepted(&mut speaker, 3, "stop", "{}");
    assert_eq!(
        speaker.close().status,
        Some(0),
        "the mcp role ends with status 0"
    );

    let dir = state_dir("ms-same");
    clear(&dir);
    let at = text(&dir);
    let ran = client(BINARY, &["start", "counter", "--session", at]);
    assert_eq!(ran.status, Some(0), "the command line starts a session");
    let _started = Started {
        dir: dir.clone(),
        pid: number(ran.line(), "pid"),
    };
    let ran = client(BINARY, &["snapshot", "--session", at]);
    assert_eq!(ran.status, Some(0), "the command line reads the screen");
    let through_command_line =
        string_at(ran.line(), "text").expect("a snapshot's answer holds its text");
    let ran = client(BINARY, &["stop", "--session", at]);
    assert_eq!(ran.status, Some(0), "the command line stops the session");

    assert!(
        through_mcp.contains(" id=\"counter-value\" "),
        "the screen read through mcp is the Counter's"
    );
    assert!(
        through_mcp == through_command_line,
        "the screen reads {} bytes through mcp and {} through the command line",
        through_mcp.len(),
        through_command_line.len()
    );
}
