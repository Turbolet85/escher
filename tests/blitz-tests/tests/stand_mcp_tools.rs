//! The driver's MCP server, run in process over an in-memory request buffer, completes the
//! stand's Counter flow by tool calls alone in both layout modes, and each tool call parents
//! the command it runs. With a boot that boots the stand's lean tasks as the `escher-session`
//! binary does: a `snapshot` tool result's text block is, byte for byte, the screen text
//! `Session::run` returns on a session booted the same way; each `click`'s text block is the
//! line of JSON that click returns run directly on that session; the second snapshot reads 3;
//! and the diff of each click reads the same in the two layout modes. A held Timer gets its
//! time step through `start`, so `advance` moves it as it moves the session's clock, which an
//! `advance` moves on every task. A subscriber of this file's own, which
//! keeps the stack of entered spans, reads every `command` span's parent as a `tool_call` span,
//! one `tool_call` per tool call sent, none under a session-level call or an unknown tool, and
//! each `tool_call` holding the table's word for its verb as `tool` and its ending as `ending`,
//! and no other field. Every refusal cause a tool call can reach is read on its served form
//! with the remedy its cause states: `malformed`, `not-found`, `stale`, `disabled` and
//! `off-screen` on the lean tasks, where an `advance` on the counter is accepted, `covered` on an overlay fixture beside them, and
//! `fonts-unavailable` on the CRUD task booted with a font context that holds no family (an
//! unknown tool is a protocol error, so `unknown-verb` is read through the binary). A
//! `screenshot` tool result is an image block, then a text block holding its line without the
//! image: the image's data decodes to the PNG `Session::run` returns for the same state, and its
//! `tool_call` span holds `screenshot` and parents its command; a refused one is one text block
//! marked as an error. A `help`
//! tool call is answered by the server, with or without a held session: its text is the tool's
//! description of itself, and its `tool_call` span holds `help` and `accepted` and parents no
//! command. No check here sleeps or reads a clock. A failure message carries a layout mode and
//! a call index or a count — never an id, a name or a captured line.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use blitz_test_harness::{Harness, HarnessOptions};
use dioxus::prelude::*;
use escher_driver::{CAUSES, Cause, Outcome, Session, SessionError, describe, serve_mcp};
use seven_guis::stand::{self, LeanTask};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

mod session_common;
use session_common::{click, hold, screenshot, shot, slug, snapshot};

/// The lean tasks, as the binary hands them to the server.
const TASKS: [LeanTask; 4] = [
    LeanTask::Counter,
    LeanTask::FlightBooker,
    LeanTask::Timer,
    LeanTask::Crud,
];

/// An id no element of the stand carries.
const NOBODY: &str = "mcp-tools-names-no-element";

/// The Counter's display reading 3, as its snapshot line writes it.
const READS_THREE: &str = " \"3\" id=\"counter-value\" ";

/// The words a `tool_call` span's `ending` may hold.
const ENDINGS: [&str; 3] = ["accepted", "refused", "error"];

/// Boots the lean task an app's name names, as the binary does: the Timer with its step.
fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {
    move |app| {
        let task = TASKS
            .into_iter()
            .find(|task| slug(*task) == app)
            .ok_or(SessionError::InvalidLabel)?;
        Ok(hold(task, incremental).0)
    }
}

fn request(id: usize, method: &str, params: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#)
}

fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    request(
        id,
        "tools/call",
        &format!(r#"{{"name":"{name}","arguments":{arguments}}}"#),
    )
}

/// The replies `serve_mcp` writes for `requests`, one per line.
fn serve(requests: &[String], incremental: bool) -> Vec<String> {
    let mut input = requests.join("\n");
    input.push('\n');
    let apps = TASKS.map(slug);
    let mut output = Vec::new();
    serve_mcp(
        input.as_bytes(),
        &mut output,
        &apps,
        boot(incremental),
        Duration::from_secs(1800),
    )
    .expect("the exchange ends");
    String::from_utf8(output)
        .expect("replies are text")
        .lines()
        .map(str::to_string)
        .collect()
}

/// The text block of a tool result's reply, unescaped over exactly the escapes the driver's
/// writer writes, and whether the result is marked as an error.
#[track_caller]
fn result(reply: &str) -> (bool, String) {
    let (_, rest) = reply
        .split_once(r#""content":[{"type":"text","text":""#)
        .expect("the reply is a tool result");
    let mut text = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next().expect("the text block ends") {
            '"' => break,
            '\\' => match chars.next().expect("an escape is whole") {
                '"' => text.push('"'),
                '\\' => text.push('\\'),
                'n' => text.push('\n'),
                'r' => text.push('\r'),
                't' => text.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    let code = u32::from_str_radix(&hex, 16).expect("four hex digits");
                    text.push(char::from_u32(code).expect("a character"));
                }
                _ => panic!("an escape the writer does not write"),
            },
            other => text.push(other),
        }
    }
    let is_error = reply.ends_with(r#""isError":true}}"#);
    assert!(
        is_error || reply.ends_with(r#""isError":false}}"#),
        "the result says whether it is an error"
    );
    (is_error, text)
}

/// One span the recorder saw: its target, its name, the index of its parent and the fields
/// recorded on it.
struct Seen {
    target: &'static str,
    name: &'static str,
    parent: Option<usize>,
    fields: Vec<(&'static str, String)>,
}

#[derive(Default)]
struct Recorded {
    spans: Vec<Seen>,
    /// The spans entered on the thread, innermost last.
    stack: Vec<usize>,
}

/// A subscriber that records each new span with the span current when it was made.
#[derive(Clone, Default)]
struct Recorder(Arc<Mutex<Recorded>>);

struct Fields<'a>(&'a mut Vec<(&'static str, String)>);

impl Visit for Fields<'_> {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.push((field.name(), value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.push((field.name(), format!("{value:?}")));
    }
}

fn index(id: &Id) -> usize {
    (id.into_u64() - 1) as usize
}

impl Subscriber for Recorder {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attributes: &Attributes<'_>) -> Id {
        let mut recorded = self.0.lock().unwrap();
        let parent = if attributes.is_contextual() {
            recorded.stack.last().copied()
        } else {
            attributes.parent().map(index)
        };
        let mut fields = Vec::new();
        attributes.record(&mut Fields(&mut fields));
        recorded.spans.push(Seen {
            target: attributes.metadata().target(),
            name: attributes.metadata().name(),
            parent,
            fields,
        });
        Id::from_u64(recorded.spans.len() as u64)
    }

    fn record(&self, span: &Id, values: &Record<'_>) {
        let mut recorded = self.0.lock().unwrap();
        let seen = &mut recorded.spans[index(span)];
        values.record(&mut Fields(&mut seen.fields));
    }

    fn record_follows_from(&self, _: &Id, _: &Id) {}

    fn event(&self, _: &Event<'_>) {}

    fn enter(&self, span: &Id) {
        self.0.lock().unwrap().stack.push(index(span));
    }

    fn exit(&self, span: &Id) {
        let mut recorded = self.0.lock().unwrap();
        if let Some(at) = recorded
            .stack
            .iter()
            .rposition(|entered| *entered == index(span))
        {
            recorded.stack.remove(at);
        }
    }
}

#[test]
fn a_scripted_mcp_session_completes_the_counter_flow_and_parents_each_command() {
    let mut clicks_by_mode = Vec::new();
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };

        // The flow, each tool call with the verb word its span records (none for an unknown
        // tool), the ending it records and whether it runs on the held instance.
        let increment = r#"{"id":"counter-increment"}"#;
        let nobody = format!(r#"{{"id":"{NOBODY}"}}"#);
        let calls: [(&str, &str, Option<&str>, &str, bool); 11] = [
            (
                "start",
                r#"{"app":"counter"}"#,
                Some("start"),
                "accepted",
                false,
            ),
            ("snapshot", "{}", Some("snapshot"), "accepted", true),
            ("click", increment, Some("click"), "accepted", true),
            ("click", increment, Some("click"), "accepted", true),
            ("click", increment, Some("click"), "accepted", true),
            ("snapshot", "{}", Some("snapshot"), "accepted", true),
            ("status", "{}", Some("status"), "accepted", false),
            ("click", &nobody, Some("click"), "refused", true),
            ("stop", "{}", Some("stop"), "accepted", false),
            ("status", "{}", Some("status"), "error", false),
            ("capture-mt9", "{}", None, "error", false),
        ];
        let mut requests = vec![request(
            0,
            "initialize",
            r#"{"protocolVersion":"2025-06-18"}"#,
        )];
        requests.extend(
            calls
                .iter()
                .enumerate()
                .map(|(index, (name, arguments, ..))| tool_call(index + 1, name, arguments)),
        );

        let recorder = Recorder::default();
        let replies =
            tracing::subscriber::with_default(recorder.clone(), || serve(&requests, incremental));
        assert_eq!(
            replies.len(),
            requests.len(),
            "{mode}: one reply per request"
        );

        // The same calls run directly on a session booted the same way.
        let (mut direct, _ticks) = hold(LeanTask::Counter, incremental);
        let screen = |outcome: Result<Outcome, _>| match outcome {
            Ok(Outcome::Screen { text }) => text,
            _ => panic!("a snapshot runs"),
        };
        let first = screen(direct.run(&snapshot()));
        let clicked: Vec<String> = (0..3)
            .map(|_| {
                direct
                    .run(&click("counter-increment"))
                    .expect("the click runs")
                    .to_json()
            })
            .collect();
        let second = screen(direct.run(&snapshot()));

        let read: Vec<(bool, String)> = replies[1..11].iter().map(|reply| result(reply)).collect();
        assert!(
            read[1] == (false, first),
            "{mode}: the first snapshot is the screen's text"
        );
        for (index, json) in clicked.iter().enumerate() {
            assert!(
                read[2 + index] == (false, json.clone()),
                "{mode}: click {index}'s text block is its line of JSON"
            );
        }
        assert!(
            read[5] == (false, second.clone()),
            "{mode}: the second snapshot too"
        );
        assert!(second.contains(READS_THREE), "{mode}: the counter reads 3");
        assert!(read[7].0, "{mode}: the refused call is marked as an error");
        assert!(
            read[9].0,
            "{mode}: a status with nothing held is marked as an error"
        );
        assert!(
            replies[11].contains(r#""error":{"code":-32602,"message":"unknown tool"}"#),
            "{mode}: an unknown tool is a protocol error"
        );
        clicks_by_mode.push(clicked);

        // The parent: every command under a tool call, one tool call per call sent.
        let recorded = recorder.0.lock().unwrap();
        let driver: Vec<(usize, &Seen)> = recorded
            .spans
            .iter()
            .enumerate()
            .filter(|(_, seen)| seen.target == "escher_driver")
            .collect();
        let tool_calls: Vec<(usize, &Seen)> = driver
            .iter()
            .copied()
            .filter(|(_, seen)| seen.name == "tool_call")
            .collect();
        let commands: Vec<&Seen> = driver
            .iter()
            .map(|(_, seen)| *seen)
            .filter(|seen| seen.name == "command")
            .collect();
        assert_eq!(
            driver.len(),
            tool_calls.len() + commands.len(),
            "{mode}: the driver makes no third span"
        );
        assert_eq!(
            tool_calls.len(),
            calls.len(),
            "{mode}: one tool_call per call"
        );
        for (index, ((at, seen), (_, _, tool, ending, instance))) in
            tool_calls.iter().zip(&calls).enumerate()
        {
            assert!(seen.parent.is_none(), "{mode}: tool_call {index} is a root");
            let named = |name: &str| {
                seen.fields
                    .iter()
                    .filter(|(field, _)| *field == name)
                    .map(|(_, value)| value.as_str())
                    .collect::<Vec<_>>()
            };
            assert!(
                named("tool") == tool.iter().copied().collect::<Vec<_>>(),
                "{mode}: tool_call {index} records its verb's word"
            );
            assert!(
                named("ending") == [*ending] && ENDINGS.contains(ending),
                "{mode}: tool_call {index} records its ending"
            );
            assert!(
                seen.fields.len() == named("tool").len() + 1,
                "{mode}: tool_call {index} records no other field"
            );
            let children = commands
                .iter()
                .filter(|command| command.parent == Some(*at))
                .count();
            assert!(
                children == usize::from(*instance),
                "{mode}: tool_call {index} parents {children} commands"
            );
        }
        let parented = commands
            .iter()
            .filter(|command| {
                command
                    .parent
                    .is_some_and(|parent| recorded.spans[parent].name == "tool_call")
            })
            .count();
        assert!(
            parented == commands.len(),
            "{mode}: {parented} of {} commands are under a tool_call",
            commands.len()
        );
    }
    assert!(
        clicks_by_mode[0] == clicks_by_mode[1],
        "the clicks' diffs read the same in the two layout modes"
    );
}

/// A button under an opaque overlay with a `z-index` above it.
fn covered_fixture() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-under",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px;",
                "Under"
            }
            div {
                id: "fx-cover",
                style: "position: absolute; left: 0; top: 0; width: 200px; height: 140px; z-index: 5; background: #cccccc;",
            }
        }
    }
}

/// The cause a refusal's text names, when its remedy is the one the cause states.
fn remedied(text: &str) -> Option<Cause> {
    let cause = CAUSES
        .into_iter()
        .find(|cause| text.starts_with(&format!(r#"{{"refused":{{"cause":"{}","#, cause.name())))?;
    text.contains(&format!(r#""remedy":"{}""#, cause.remedy()))
        .then_some(cause)
}

#[test]
fn every_cause_a_tool_call_reaches_is_served_with_its_remedy() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let nobody = format!(r#"{{"id":"{NOBODY}"}}"#);
        let create = r#"{"id":"crud-create"}"#;
        let last_row = r#"{"id":"crud-person-14"}"#;
        // Each call, with the cause it is refused with, or none where it runs.
        let mut calls: Vec<(&str, &str, Option<Cause>)> = vec![
            ("start", r#"{"app":"counter"}"#, None),
            ("click", "{}", Some(Cause::Malformed)),
            ("click", &nobody, Some(Cause::NotFound)),
            ("advance", r#"{"ms":100}"#, None),
            ("stop", "{}", None),
            ("start", r#"{"app":"flight-booker"}"#, None),
            (
                "click",
                r#"{"id":"flight-return-date"}"#,
                Some(Cause::Disabled),
            ),
            ("stop", "{}", None),
            ("start", r#"{"app":"crud"}"#, None),
        ];
        // Twelve rows more put the last one past the list's box.
        calls.extend(std::iter::repeat_n(("click", create, None), 12));
        calls.extend([
            ("click", last_row, Some(Cause::OffScreen)),
            ("scroll", last_row, None),
            ("click", last_row, None),
            ("click", r#"{"id":"crud-delete"}"#, None),
            ("click", last_row, Some(Cause::Stale)),
            ("stop", "{}", None),
        ]);
        let requests: Vec<String> = calls
            .iter()
            .enumerate()
            .map(|(index, (name, arguments, _))| tool_call(index + 1, name, arguments))
            .collect();
        let recorder = Recorder::default();
        let replies =
            tracing::subscriber::with_default(recorder.clone(), || serve(&requests, incremental));
        assert_eq!(replies.len(), calls.len(), "{mode}: one reply per call");
        let mut served = Vec::new();
        for (index, (reply, (_, _, cause))) in replies.iter().zip(&calls).enumerate() {
            let (is_error, text) = result(reply);
            assert!(
                is_error == cause.is_some(),
                "{mode}: call {index} ends as stated"
            );
            if let Some(cause) = cause {
                assert!(
                    remedied(&text) == Some(*cause),
                    "{mode}: call {index} is refused with its cause and that cause's remedy"
                );
                served.push(*cause);
            }
        }

        // The accepted advance: its tool_call span ends accepted and parents its command.
        {
            let recorded = recorder.0.lock().unwrap();
            let named = |seen: &Seen, name: &str| -> Vec<String> {
                seen.fields
                    .iter()
                    .filter(|(field, _)| *field == name)
                    .map(|(_, value)| value.clone())
                    .collect()
            };
            let advances: Vec<usize> = recorded
                .spans
                .iter()
                .enumerate()
                .filter(|(_, seen)| seen.target == "escher_driver" && seen.name == "tool_call")
                .filter(|(_, seen)| named(seen, "tool") == ["advance"])
                .map(|(at, _)| at)
                .collect();
            assert!(
                advances.len() == 1,
                "{mode}: {} advance tool_call spans",
                advances.len()
            );
            let at = advances[0];
            assert!(
                named(&recorded.spans[at], "ending") == ["accepted"],
                "{mode}: the advance's tool_call ends accepted"
            );
            let children: Vec<&Seen> = recorded
                .spans
                .iter()
                .filter(|seen| seen.name == "command" && seen.parent == Some(at))
                .collect();
            assert!(
                children.len() == 1
                    && named(children[0], "verb") == ["advance"]
                    && named(children[0], "cause").is_empty(),
                "{mode}: the advance's tool_call parents one accepted advance command"
            );
        }

        // The covered cause, on a fixture beside the stand.
        let requests = [
            tool_call(1, "start", r#"{"app":"covered"}"#),
            tool_call(2, "click", r#"{"id":"fx-under"}"#),
            tool_call(3, "stop", "{}"),
        ];
        let mut input = requests.join("\n");
        input.push('\n');
        let mut output = Vec::new();
        let boot_fixture = |app: &str| {
            Session::start(app, || {
                Harness::from_vdom(
                    VirtualDom::new(covered_fixture),
                    stand::options(incremental),
                )
            })
        };
        tracing::subscriber::with_default(Recorder::default(), || {
            serve_mcp(
                input.as_bytes(),
                &mut output,
                &["covered"],
                boot_fixture,
                Duration::from_secs(1800),
            )
        })
        .expect("the exchange ends");
        let output = String::from_utf8(output).expect("replies are text");
        let replies: Vec<&str> = output.lines().collect();
        assert_eq!(replies.len(), 3, "{mode}: one reply per fixture call");
        let (is_error, text) = result(replies[1]);
        assert!(
            is_error && remedied(&text) == Some(Cause::Covered),
            "{mode}: the covered button is refused with its cause and that cause's remedy"
        );
        served.push(Cause::Covered);

        // The fonts-unavailable cause, on the CRUD task booted with no usable font.
        let requests = [
            tool_call(1, "start", r#"{"app":"crud"}"#),
            tool_call(2, "screenshot", "{}"),
            tool_call(3, "stop", "{}"),
        ];
        let mut input = requests.join("\n");
        input.push('\n');
        let mut output = Vec::new();
        tracing::subscriber::with_default(Recorder::default(), || {
            serve_mcp(
                input.as_bytes(),
                &mut output,
                &["crud"],
                boot_without_font(incremental),
                Duration::from_secs(1800),
            )
        })
        .expect("the exchange ends");
        let output = String::from_utf8(output).expect("replies are text");
        let replies: Vec<&str> = output.lines().collect();
        assert_eq!(replies.len(), 3, "{mode}: one reply per no-font call");
        let (is_error, text) = result(replies[1]);
        assert!(
            is_error && remedied(&text) == Some(Cause::FontsUnavailable),
            "{mode}: the shot with no font is refused with its cause and that cause's remedy"
        );
        served.push(Cause::FontsUnavailable);

        let reached: Vec<Cause> = CAUSES
            .into_iter()
            .filter(|cause| served.contains(cause))
            .collect();
        assert!(
            reached.len() == 7 && !reached.contains(&Cause::UnknownVerb),
            "{mode}: {} causes are served through MCP",
            reached.len()
        );
    }
}

/// Boots the CRUD task with a font context built from bytes that decode as no font.
fn boot_without_font(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {
    move |app| {
        Session::start(app, || {
            stand::boot(
                LeanTask::Crud,
                HarnessOptions {
                    font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
                    ..stand::options(incremental)
                },
            )
        })
    }
}

/// Reads RFC 4648 base64 with padding: `None` for any other text.
fn decode_base64(text: &str) -> Option<Vec<u8>> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if text.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::new();
    for chunk in text.as_bytes().chunks(4) {
        let padding = chunk.iter().rev().take_while(|byte| **byte == b'=').count();
        let mut n = 0u32;
        for byte in &chunk[..4 - padding] {
            n = n << 6 | ALPHABET.iter().position(|letter| letter == byte)? as u32;
        }
        n <<= 6 * padding as u32;
        out.extend_from_slice(&n.to_be_bytes()[1..4 - padding]);
    }
    Some(out)
}

/// A screenshot tool result's two blocks: the image block's data and MIME type, then the text
/// block's text, and whether the result is marked as an error. `None` when the reply does not
/// hold an image block first.
fn shot_result(reply: &str) -> Option<(String, String, String, bool)> {
    let (_, rest) = reply.split_once(r#""content":[{"type":"image","data":""#)?;
    let (data, rest) = rest.split_once('"')?;
    let (_, rest) = rest.split_once(r#""mimeType":""#)?;
    let (mime, rest) = rest.split_once('"')?;
    let (_, rest) = rest.split_once(r#"},{"type":"text","text":""#)?;
    let (text, rest) = rest.split_once(r#""}]"#)?;
    Some((
        data.to_string(),
        mime.to_string(),
        text.replace("\\\"", "\""),
        rest.contains(r#""isError":true"#),
    ))
}

#[test]
fn a_screenshot_tool_call_answers_its_image_then_its_text_and_parents_its_command() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let nobody = format!(r#"{{"id":"{NOBODY}"}}"#);
        let requests = [
            tool_call(1, "start", r#"{"app":"crud"}"#),
            tool_call(2, "screenshot", "{}"),
            tool_call(3, "screenshot", &nobody),
            tool_call(4, "stop", "{}"),
        ];
        let recorder = Recorder::default();
        let replies =
            tracing::subscriber::with_default(recorder.clone(), || serve(&requests, incremental));
        assert_eq!(replies.len(), 4, "{mode}: one reply per call");

        let (mut direct, _ticks) = hold(LeanTask::Crud, incremental);
        let direct = shot(&mut direct, &screenshot()).expect("a screenshot runs");

        let Some((data, mime, text, is_error)) = shot_result(&replies[1]) else {
            panic!("{mode}: the shot is an image block, then a text block");
        };
        assert!(!is_error, "{mode}: the shot is not marked as an error");
        assert!(
            mime == "image/png",
            "{mode}: the image block says it is a PNG"
        );
        assert!(
            decode_base64(&data) == Some(direct.png),
            "{mode}: the image is the PNG Session::run returns for the same state"
        );
        assert!(
            text == r#"{"width":800,"height":600}"#,
            "{mode}: the text block is the shot's line without the image"
        );

        let (is_error, text) = result(&replies[2]);
        assert!(
            is_error && remedied(&text) == Some(Cause::NotFound),
            "{mode}: the refused shot is one text block marked as an error"
        );
        assert!(
            !replies[2].contains(r#""type":"image""#),
            "{mode}: a refused shot holds no image"
        );

        let recorded = recorder.0.lock().unwrap();
        let shots: Vec<usize> = recorded
            .spans
            .iter()
            .enumerate()
            .filter(|(_, seen)| {
                seen.name == "tool_call"
                    && seen
                        .fields
                        .iter()
                        .any(|(field, value)| *field == "tool" && value == "screenshot")
            })
            .map(|(at, _)| at)
            .collect();
        assert_eq!(shots.len(), 2, "{mode}: one tool_call per screenshot call");
        for (index, at) in shots.iter().enumerate() {
            let children = recorded
                .spans
                .iter()
                .filter(|seen| seen.name == "command" && seen.parent == Some(*at))
                .count();
            assert!(
                children == 1,
                "{mode}: screenshot {index} parents {children} commands"
            );
        }
    }
}

#[test]
fn help_is_answered_by_the_server_and_parents_no_command() {
    let requests = [
        request(0, "initialize", r#"{"protocolVersion":"2025-11-25"}"#),
        tool_call(1, "help", "{}"),
        tool_call(2, "start", r#"{"app":"counter"}"#),
        tool_call(3, "help", "{}"),
        tool_call(4, "stop", "{}"),
    ];
    let recorder = Recorder::default();
    let replies = tracing::subscriber::with_default(recorder.clone(), || serve(&requests, true));
    assert_eq!(replies.len(), requests.len(), "one reply per request");
    assert!(
        replies[0].contains(r#""protocolVersion":"2025-11-25""#),
        "initialize answers the revision asked for"
    );
    for index in [1, 3] {
        assert!(
            result(&replies[index]) == (false, describe()),
            "call {index}: help answers the tool's description of itself"
        );
    }

    let recorded = recorder.0.lock().unwrap();
    let driver: Vec<&Seen> = recorded
        .spans
        .iter()
        .filter(|seen| seen.target == "escher_driver")
        .collect();
    assert!(
        driver.iter().all(|seen| seen.name == "tool_call"),
        "no call here runs a command"
    );
    let helps: Vec<&&Seen> = driver
        .iter()
        .filter(|seen| {
            seen.fields
                .iter()
                .any(|(field, value)| *field == "tool" && value == "help")
        })
        .collect();
    assert_eq!(helps.len(), 2, "one tool_call per help call");
    for (index, seen) in helps.iter().enumerate() {
        assert!(
            seen.fields.len() == 2
                && seen
                    .fields
                    .iter()
                    .any(|(field, value)| *field == "ending" && value == "accepted"),
            "help {index}: the span holds its tool and an accepted ending, and nothing else"
        );
    }
}

#[test]
fn a_held_timer_gets_its_time_step_through_start() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let requests = [
            tool_call(1, "start", r#"{"app":"timer"}"#),
            tool_call(2, "advance", r#"{"ms":1000}"#),
            tool_call(3, "stop", "{}"),
        ];
        // Under a recorder of its own: a thread that reaches the driver's spans with no
        // subscriber leaves their callsites cached as uninteresting, which a check running
        // beside it would read as no span at all.
        let replies = tracing::subscriber::with_default(Recorder::default(), || {
            serve(&requests, incremental)
        });
        assert_eq!(replies.len(), 3, "{mode}: one reply per request");
        let (is_error, advanced) = result(&replies[1]);
        assert!(!is_error, "{mode}: advance runs on a held Timer");
        assert!(
            advanced.contains(r#""advanced_ms":1000"#),
            "{mode}: the Timer moved the time asked"
        );
    }
}
