//! The `escher-session` binary in its `mcp` role, its telemetry sink installed, writes nothing a
//! tool call handled to stderr at `RUST_LOG=trace`: no stable element id, no accessible name, no
//! typed text and no image byte — while a CRUD flow runs through it by tool calls, a `type` and
//! two `screenshot`s, the whole screen and one element, among them — and
//! each tool call leaves exactly one `tool_call` line there, holding only its `tool` and its
//! `ending` beside the sink's own fields, and each call run on the held instance one `command`
//! line. Stdout holds protocol lines only. The check reads the binary as its own build makes
//! it; the workspace build compiles the engine's tracing call sites in, and the same check is
//! run on that build too. A failure message names a kind and a count, never a captured line.

#![cfg(not(target_arch = "wasm32"))]

mod common;

use std::process::Command;

use common::{Speaker, string_at};
use seven_guis::stand::{self, LeanTask};

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// The text the `type` types: synthetic, and no part of any id or name.
const SENTINEL: &str = "Typed-Sentinel-Mq83vn";

/// An id no element of the stand carries.
const NOBODY: &str = "mcp-log-names-no-element";

/// The field names a `tool_call` line may carry after the service identity: the span's name,
/// its two own fields, and the three the sink adds when a span closes.
const TOOL_CALL_FIELDS: [&str; 6] = [
    "span",
    "tool",
    "ending",
    "message",
    "time.busy",
    "time.idle",
];

/// The words a `tool_call` line's `ending` may hold.
const ENDINGS: [&str; 3] = ["accepted", "refused", "error"];

fn tool_call(id: u32, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

#[test]
fn the_mcp_role_logs_no_id_no_name_and_no_typed_text_at_trace() {
    // The needles are read from the screen the role serves, booted here in process, as the
    // host's own log check reads them. An id is a needle when it is an author key (it holds no
    // `/`) and holds a `-`: that leaves out `main`, an ordinary word.
    let harness = stand::boot(LeanTask::Crud, stand::options(true));
    let ids: Vec<String> = harness
        .doc
        .element_ids()
        .into_iter()
        .map(|(_, id)| id)
        .filter(|id| !id.contains('/') && id.contains('-'))
        .collect();
    let text_of = |selector: &str| -> Vec<String> {
        let nodes = harness.query_all(selector);
        let doc = harness.base();
        nodes
            .into_iter()
            .map(|node| doc.get_node(node).expect("a queried node exists"))
            .map(|node| node.text_content())
            .collect()
    };
    let rows = text_of("#crud-list > div");
    let list_name = harness.attr("#crud-list", "aria-label");
    assert!(list_name.is_some(), "the list's name was read");
    let names: Vec<String> = rows
        .iter()
        .cloned()
        .chain(text_of("label"))
        .chain(list_name)
        .collect();
    assert!(
        ids.iter().any(|id| id == "crud-name"),
        "the screen's author keys are among the id needles"
    );
    assert!(ids.len() >= 15, "{} id needles were read", ids.len());
    assert!(rows.len() >= 3, "{} row names were read", rows.len());
    assert!(
        ids.iter().chain(&names).all(|needle| !needle.is_empty()
            && !needle.contains(SENTINEL)
            && !SENTINEL.contains(needle.as_str())),
        "no needle is empty or a part of the sentinel"
    );

    let mut command = Command::new(BINARY);
    command.arg("mcp").env("RUST_LOG", "trace");
    let mut speaker = Speaker::spawn(command);

    speaker.send(
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#,
    );
    let mut replies = vec![speaker.reply()];
    // The flow: each tool call with whether it runs on the held instance and whether its
    // result is marked as an error.
    let calls = [
        ("start", r#"{"app":"crud"}"#.to_string(), false, false),
        ("snapshot", "{}".to_string(), true, false),
        (
            "type",
            format!(r#"{{"id":"crud-name","text":"{SENTINEL}"}}"#),
            true,
            false,
        ),
        ("click", r#"{"id":"crud-create"}"#.to_string(), true, false),
        ("click", format!(r#"{{"id":"{NOBODY}"}}"#), true, true),
        ("screenshot", "{}".to_string(), true, false),
        (
            "screenshot",
            r#"{"id":"crud-list"}"#.to_string(),
            true,
            false,
        ),
        ("stop", "{}".to_string(), false, false),
    ];
    assert_eq!(calls.len(), 8);
    for (index, (name, arguments, _, is_error)) in calls.iter().enumerate() {
        let id = index as u32 + 2;
        speaker.send(tool_call(id, name, arguments).as_bytes());
        let reply = speaker.reply();
        let marked = if *is_error {
            r#""isError":true}}"#
        } else {
            r#""isError":false}}"#
        };
        assert!(reply.ends_with(marked), "call {id}: marked as expected");
        replies.push(reply);
    }
    // What the flow handled is in the replies, so the absences read below are of things the
    // role did handle.
    let snapshot = string_at(&replies[2], "text").expect("the snapshot's text block");
    let ids_answered = ids
        .iter()
        .filter(|id| snapshot.contains(&format!("id=\"{id}\"")))
        .count();
    assert!(
        ids_answered == ids.len(),
        "the snapshot holds {ids_answered} of {} id needles",
        ids.len()
    );
    assert!(
        replies[3].contains(SENTINEL),
        "the type's result holds the typed text as the control's value"
    );
    // The first 64 characters of each shot's image block: the image as it crosses.
    let images: Vec<String> = replies[6..8]
        .iter()
        .map(|reply| {
            let (_, data) = reply
                .split_once(r#"{"type":"image","data":""#)
                .expect("a shot's result holds its image block");
            data.chars().take(64).collect()
        })
        .collect();
    assert!(
        images.iter().all(|image| image.len() == 64),
        "each shot's result holds an image"
    );

    let spoke = speaker.close();
    assert_eq!(
        spoke.status,
        Some(0),
        "closing stdin ends the process with status 0"
    );
    assert!(
        spoke.stdout.is_empty(),
        "{} stdout lines past the replies",
        spoke.stdout.len()
    );
    let protocol = replies
        .iter()
        .filter(|reply| reply.starts_with(r#"{"jsonrpc":"2.0","id":"#))
        .count();
    assert!(
        protocol == calls.len() + 1,
        "{protocol} of {} stdout lines are protocol replies",
        replies.len()
    );

    let stderr = String::from_utf8_lossy(&spoke.stderr);
    let lines = stderr.lines().count();
    let unstamped = stderr
        .lines()
        .filter(|line| !line.contains("service.name=seven_guis"))
        .count();
    assert!(lines >= 1, "the sink's install line reaches stderr");
    assert_eq!(
        unstamped, 0,
        "{unstamped} of {lines} stderr lines carry no service name"
    );

    let span_lines = |span: &str| -> Vec<&str> {
        stderr
            .lines()
            .filter(|line| {
                line.contains(" escher_driver ") && line.contains(&format!(" span=\"{span}\""))
            })
            .collect()
    };
    let tool_calls = span_lines("tool_call");
    assert_eq!(
        tool_calls.len(),
        calls.len(),
        "{} tool_call lines for {} tool calls sent",
        tool_calls.len(),
        calls.len()
    );
    for (index, line) in tool_calls.iter().enumerate() {
        // `{time} {LEVEL} {target} service.name=… service.version=…`, then the span's pairs.
        let pairs: Vec<(&str, &str)> = line
            .split(' ')
            .skip(5)
            .filter_map(|pair| pair.split_once('='))
            .collect();
        let outside = pairs
            .iter()
            .filter(|(name, _)| !TOOL_CALL_FIELDS.contains(name))
            .count();
        assert_eq!(
            outside, 0,
            "tool_call line {index}: {outside} fields outside the set"
        );
        let endings = pairs
            .iter()
            .filter(|(name, value)| *name == "ending" && ENDINGS.contains(&value.trim_matches('"')))
            .count();
        assert_eq!(
            endings, 1,
            "tool_call line {index}: one ending of the three words"
        );
    }
    let refused = tool_calls
        .iter()
        .filter(|line| line.contains(" ending=\"refused\" "))
        .count();
    assert_eq!(refused, 1, "{refused} tool_call lines read refused");

    let instance_calls = calls.iter().filter(|(_, _, instance, _)| *instance).count();
    let commands = span_lines("command").len();
    assert_eq!(
        commands, instance_calls,
        "{commands} command lines for {instance_calls} calls run on the held instance"
    );

    let found = |needles: &[String]| {
        needles
            .iter()
            .filter(|needle| stderr.contains(needle.as_str()))
            .count()
    };
    let (ids_found, names_found) = (found(&ids), found(&names));
    let sentinels = stderr.matches(SENTINEL).count();
    let nobodies = stderr.matches(NOBODY).count();
    let image_bytes: usize = images
        .iter()
        .map(|image| stderr.matches(image.as_str()).count())
        .sum();
    assert!(
        ids_found == 0 && names_found == 0 && sentinels == 0 && nobodies == 0 && image_bytes == 0,
        "{ids_found} of {} id needles, {names_found} of {} name needles, {sentinels} \
         occurrences of the typed text, {nobodies} of the refused id and {image_bytes} of an \
         image found in {lines} stderr lines",
        ids.len(),
        names.len()
    );
}
