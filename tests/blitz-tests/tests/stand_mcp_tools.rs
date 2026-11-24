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
//! marked as an error. A `lint` tool result on each lean task is one text block, the line of the
//! findings `Session::lint` reads on a session booted the same way, and its `tool_call` span
//! holds `lint` and parents its command. A `graph` tool result on each lean task is one text
//! block, the line of the graph `DioxusDocument::graph` reads on a session booted the same way,
//! and its `tool_call` span holds `graph` and parents one command that records `verb` alone; an
//! `actions` tool result is the same, the line of the nodes `Session::actions` reads. A
//! `layout` tool result on each lean task is one text block, the line `Session::run` writes for
//! the same id on a session booted the same way; one with no id is refused malformed and one
//! naming no element not-found, and the accepted call's `tool_call` parents one command that
//! records `verb` alone, the not-found one's `verb` and `cause`. A `canvas` tool result on the
//! canvas fixture, summary and full, is one text block, the line `Session::run` writes for the
//! same call, one with no id is refused malformed, and each accepted call's `tool_call` parents
//! one command that records `verb` alone. A
//! `graph` tool call with `part` answers the line of the part `Session::run` writes, and its
//! command records `verb` alone; a `press` with `ctrl` answers the run's own line; a `part` out of
//! its bound and a `ctrl` of another kind are refused malformed; on a `multiple` select fixture a
//! click on row 1, two `press` calls with `ctrl` on `arrow-down` and a `press space` read rows 1
//! and 3 selected and row 2 not. A `help`
//! tool call is answered by the server, with or without a held session: its text is the tool's
//! description of itself, and its `tool_call` span holds `help` and `accepted` and parents no
//! command. No check here sleeps or reads a clock. A failure message carries a layout mode and
//! a call index or a count — never an id, a name or a captured line.

use std::sync::{Arc, Mutex};

use blitz_test_harness::HarnessOptions;
use escher_driver::{CAUSES, Cause, Outcome, Session, SessionError, describe};
use seven_guis::stand::{self, LeanTask};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

mod fixtures;
mod session_common;
use session_common::{
    canvas, canvas_full, click, graph_part, hold, layout, mcp_tool_result, press, press_ctrl,
    screenshot, serve_lines, shot, slug, snapshot,
};

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
    serve_lines(requests, &TASKS.map(slug), boot(incremental))
}

/// The text block of a tool result's reply, unescaped over exactly the escapes the driver's
/// writer writes, and whether the result is marked as an error.
#[track_caller]
fn result(reply: &str) -> (bool, String) {
    let (is_error, text) = mcp_tool_result(reply);
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

/// The indices of the `tool_call` spans `recorded` holds whose `tool` reads `tool`, in order.
fn tool_calls(recorded: &Recorded, tool: &str) -> Vec<usize> {
    recorded
        .spans
        .iter()
        .enumerate()
        .filter(|(_, seen)| {
            seen.name == "tool_call"
                && seen
                    .fields
                    .iter()
                    .any(|(field, value)| *field == "tool" && value == tool)
        })
        .map(|(at, _)| at)
        .collect()
}

/// A read's exchange on `task` under a recorder of its own: `start`, the read `verb` with no
/// argument, the read naming one, `stop`; the recorder and the four replies, asserted four.
#[track_caller]
fn served_read(
    verb: &str,
    task: LeanTask,
    incremental: bool,
    mode: &str,
    row: usize,
) -> (Recorder, Vec<String>) {
    let start = format!(r#"{{"app":"{}"}}"#, slug(task));
    let requests = [
        tool_call(1, "start", &start),
        tool_call(2, verb, "{}"),
        tool_call(3, verb, r#"{"id":"x"}"#),
        tool_call(4, "stop", "{}"),
    ];
    let recorder = Recorder::default();
    let replies =
        tracing::subscriber::with_default(recorder.clone(), || serve(&requests, incremental));
    assert_eq!(replies.len(), 4, "{mode}: task {row}: one reply per call");
    (recorder, replies)
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
        let boot_fixture =
            |app: &str| Session::start(app, || fixtures::boot(fixtures::covered, incremental));
        let replies = tracing::subscriber::with_default(Recorder::default(), || {
            serve_lines(&requests, &["covered"], boot_fixture)
        });
        assert_eq!(replies.len(), 3, "{mode}: one reply per fixture call");
        let (is_error, text) = result(&replies[1]);
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
        let replies = tracing::subscriber::with_default(Recorder::default(), || {
            serve_lines(&requests, &["crud"], boot_without_font(incremental))
        });
        assert_eq!(replies.len(), 3, "{mode}: one reply per no-font call");
        let (is_error, text) = result(&replies[1]);
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
        let shots = tool_calls(&recorded, "screenshot");
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
fn a_lint_tool_call_answers_the_readers_findings_and_parents_its_command() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        for (row, task) in TASKS.into_iter().enumerate() {
            let (recorder, replies) = served_read("lint", task, incremental, mode, row);

            let (mut direct, _ticks) = hold(task, incremental);
            let findings = direct.lint().expect("the stand has a font");
            assert!(
                result(&replies[1]) == (false, Outcome::Linted { findings }.to_json()),
                "{mode}: task {row}: the text block is the line of the reader's own findings"
            );
            let (is_error, text) = result(&replies[2]);
            assert!(
                is_error && remedied(&text) == Some(Cause::Malformed),
                "{mode}: task {row}: a lint naming an argument is refused malformed"
            );

            let recorded = recorder.0.lock().unwrap();
            let lints = tool_calls(&recorded, "lint");
            assert_eq!(lints.len(), 2, "{mode}: task {row}: one tool_call per lint");
            let accepted = recorded.spans[lints[0]]
                .fields
                .iter()
                .any(|(field, value)| *field == "ending" && value == "accepted");
            let children = recorded
                .spans
                .iter()
                .filter(|seen| seen.name == "command" && seen.parent == Some(lints[0]))
                .count();
            assert!(
                accepted && children == 1,
                "{mode}: task {row}: the lint's tool_call is accepted and parents one command"
            );
        }
    }
}

#[test]
fn a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone() {
    a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone(
        "graph",
        |session| {
            let graph = session.harness().doc.graph();
            Outcome::Graphed { graph }.to_json()
        },
    );
}

#[test]
fn a_graph_part_and_a_press_with_ctrl_round_trip_over_mcp_as_the_run_answers_them() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let start = format!(r#"{{"app":"{}"}}"#, slug(LeanTask::Crud));
        let requests = [
            tool_call(1, "start", &start),
            tool_call(2, "graph", r#"{"part":1}"#),
            tool_call(3, "graph", r#"{"part":2}"#),
            tool_call(4, "graph", r#"{"part":0}"#),
            tool_call(5, "press", r#"{"key":"tab"}"#),
            tool_call(6, "press", r#"{"key":"arrow-down","ctrl":true}"#),
            tool_call(7, "press", r#"{"key":"tab","ctrl":"yes"}"#),
            tool_call(8, "stop", "{}"),
        ];
        let recorder = Recorder::default();
        let replies =
            tracing::subscriber::with_default(recorder.clone(), || serve(&requests, incremental));
        assert_eq!(replies.len(), 8, "{mode}: one reply per call");

        let (mut direct, _ticks) = hold(LeanTask::Crud, incremental);
        let calls = [
            graph_part(1),
            graph_part(2),
            press("tab", false),
            press_ctrl("arrow-down"),
        ];
        let expected: Vec<String> = calls
            .iter()
            .map(|call| {
                direct
                    .run(call)
                    .map(|outcome| outcome.to_json())
                    .unwrap_or_else(|_| panic!("{mode}: the direct run answers"))
            })
            .collect();
        for (row, (reply, line)) in [&replies[1], &replies[2], &replies[4], &replies[5]]
            .into_iter()
            .zip(&expected)
            .enumerate()
        {
            assert!(
                result(reply) == (false, line.clone()),
                "{mode}: row {row}: the text block is the run's own line"
            );
        }
        assert!(
            expected[0].ends_with(r#""part":1,"parts":1}"#)
                && expected[1].ends_with(r#""states":[],"part":2,"parts":1}"#),
            "{mode}: a paged graph result is its JSON line, part 1 of CRUD's one part"
        );
        for (row, reply) in [&replies[3], &replies[6]].into_iter().enumerate() {
            let (is_error, text) = result(reply);
            assert!(
                is_error && remedied(&text) == Some(Cause::Malformed),
                "{mode}: row {row}: an argument out of its bound or kind is refused malformed"
            );
        }

        let recorded = recorder.0.lock().unwrap();
        let graphs = tool_calls(&recorded, "graph");
        assert_eq!(graphs.len(), 3, "{mode}: one tool_call per graph");
        let commands: Vec<&Seen> = recorded
            .spans
            .iter()
            .filter(|seen| seen.name == "command" && seen.parent == Some(graphs[0]))
            .collect();
        assert!(
            commands.len() == 1
                && !commands[0].fields.is_empty()
                && commands[0]
                    .fields
                    .iter()
                    .all(|(field, value)| *field == "verb" && value == "graph"),
            "{mode}: a graph part's command records its verb alone, no part or count"
        );
    }
}

/// A `multiple` select of three rows, as `stand_act_select` holds it.
fn rows_page() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        div { id: "fx-root",
            select { id: "fx-rows", aria_label: "Rows", multiple: true,
                option { id: "fx-one", value: "1", "One" }
                option { id: "fx-two", value: "2", "Two" }
                option { id: "fx-three", value: "3", "Three" }
            }
        }
    }
}

#[test]
fn press_tool_calls_with_ctrl_build_a_set_of_rows_that_are_not_adjacent() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let requests = [
            tool_call(1, "start", r#"{"app":"rows"}"#),
            tool_call(2, "click", r#"{"id":"fx-one"}"#),
            tool_call(3, "press", r#"{"key":"arrow-down","ctrl":true}"#),
            tool_call(4, "press", r#"{"key":"arrow-down","ctrl":true}"#),
            tool_call(5, "press", r#"{"key":"space"}"#),
            tool_call(6, "snapshot", "{}"),
            tool_call(7, "stop", "{}"),
        ];
        let replies = tracing::subscriber::with_default(Recorder::default(), || {
            serve_lines(&requests, &["rows"], |app: &str| {
                Session::start(app, || fixtures::boot(rows_page, incremental))
            })
        });
        assert_eq!(replies.len(), 7, "{mode}: one reply per call");
        for (row, reply) in replies[1..6].iter().enumerate() {
            assert!(!result(reply).0, "{mode}: row {row}: the call is accepted");
        }
        let (_, screen) = result(&replies[5]);
        let reads = |id: &str| {
            screen
                .lines()
                .find(|line| line.contains(&format!("id=\"{id}\"")))
                .map(|line| line.split_whitespace().any(|word| word == "selected"))
        };
        assert!(
            (reads("fx-one"), reads("fx-two"), reads("fx-three"))
                == (Some(true), Some(false), Some(true)),
            "{mode}: rows 1 and 3 read selected and row 2 not"
        );
    }
}

#[test]
fn an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone() {
    a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone(
        "actions",
        |session| {
            let nodes = session.actions();
            Outcome::Actions { nodes }.to_json()
        },
    );
}

#[test]
fn a_layout_tool_call_answers_the_runs_line_and_parents_a_command_recording_its_verb_alone() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        for (row, task) in TASKS.into_iter().enumerate() {
            let (mut direct, _ticks) = hold(task, incremental);
            let id = direct
                .harness()
                .doc
                .snapshot()
                .nodes()
                .map(|node| node.id.clone())
                .find(|id| id.contains('-') && !id.contains(['"', '\\']))
                .unwrap_or_else(|| panic!("{mode}: task {row}: a node with a plain author key"));
            let expected = direct
                .run(&layout(&id))
                .map(|outcome| outcome.to_json())
                .unwrap_or_else(|_| panic!("{mode}: task {row}: the run answers"));

            let start = format!(r#"{{"app":"{}"}}"#, slug(task));
            let requests = [
                tool_call(1, "start", &start),
                tool_call(2, "layout", &format!(r#"{{"id":"{id}"}}"#)),
                tool_call(3, "layout", "{}"),
                tool_call(4, "layout", &format!(r#"{{"id":"{NOBODY}"}}"#)),
                tool_call(5, "stop", "{}"),
            ];
            let recorder = Recorder::default();
            let replies = tracing::subscriber::with_default(recorder.clone(), || {
                serve(&requests, incremental)
            });
            assert_eq!(replies.len(), 5, "{mode}: task {row}: one reply per call");
            assert!(
                result(&replies[1]) == (false, expected),
                "{mode}: task {row}: the text block is the line the run writes"
            );
            let (is_error, text) = result(&replies[2]);
            assert!(
                is_error && remedied(&text) == Some(Cause::Malformed),
                "{mode}: task {row}: a layout with no id is refused malformed"
            );
            let (is_error, text) = result(&replies[3]);
            assert!(
                is_error && remedied(&text) == Some(Cause::NotFound),
                "{mode}: task {row}: a layout naming no element is refused not-found"
            );

            let recorded = recorder.0.lock().unwrap();
            let calls = tool_calls(&recorded, "layout");
            assert_eq!(
                calls.len(),
                3,
                "{mode}: task {row}: one tool_call per layout"
            );
            let ending = |at: usize| {
                recorded.spans[at]
                    .fields
                    .iter()
                    .find(|(field, _)| *field == "ending")
                    .map(|(_, value)| value.clone())
            };
            assert!(
                ending(calls[0]).as_deref() == Some("accepted")
                    && ending(calls[2]).as_deref() == Some("refused"),
                "{mode}: task {row}: the endings read accepted and refused"
            );
            let children = |at: usize| -> Vec<&Seen> {
                recorded
                    .spans
                    .iter()
                    .filter(|seen| seen.name == "command" && seen.parent == Some(at))
                    .collect()
            };
            let accepted = children(calls[0]);
            assert!(
                accepted.len() == 1
                    && !accepted[0].fields.is_empty()
                    && accepted[0]
                        .fields
                        .iter()
                        .all(|(field, value)| *field == "verb" && value == "layout"),
                "{mode}: task {row}: the accepted layout's command records its verb alone"
            );
            let not_found = children(calls[2]);
            let mut fields: Vec<(&str, &str)> = not_found
                .iter()
                .flat_map(|seen| seen.fields.iter())
                .map(|(field, value)| (*field, value.as_str()))
                .collect();
            fields.sort_unstable();
            assert!(
                not_found.len() == 1 && fields == [("cause", "not-found"), ("verb", "layout")],
                "{mode}: task {row}: the refused layout's command records its verb and cause"
            );
        }
    }
}

#[test]
fn a_canvas_tool_call_answers_the_runs_line_and_parents_a_command_recording_its_verb_alone() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let hold_fixture = || fixtures::hold("canvas", fixtures::canvas_commands, incremental);
        let calls = [canvas("small-canvas"), canvas_full("small-canvas")];
        let expected: Vec<String> = calls
            .iter()
            .enumerate()
            .map(|(index, call)| {
                hold_fixture()
                    .run(call)
                    .map(|outcome| outcome.to_json())
                    .unwrap_or_else(|_| panic!("{mode}: call {index}: the run answers"))
            })
            .collect();
        let requests = [
            tool_call(1, "start", r#"{"app":"canvas"}"#),
            tool_call(2, "canvas", r#"{"id":"small-canvas"}"#),
            tool_call(3, "canvas", r#"{"id":"small-canvas","full":true}"#),
            tool_call(4, "canvas", "{}"),
            tool_call(5, "stop", "{}"),
        ];
        let recorder = Recorder::default();
        let replies = tracing::subscriber::with_default(recorder.clone(), || {
            serve_lines(&requests, &["canvas"], |app: &str| {
                Ok(fixtures::hold(app, fixtures::canvas_commands, incremental))
            })
        });
        assert_eq!(replies.len(), 5, "{mode}: one reply per call");
        for (index, line) in expected.iter().enumerate() {
            assert!(
                result(&replies[index + 1]) == (false, line.clone()),
                "{mode}: call {index}: the text block is the line the run writes"
            );
        }
        let (is_error, text) = result(&replies[3]);
        assert!(
            is_error && remedied(&text) == Some(Cause::Malformed),
            "{mode}: a canvas with no id is refused malformed"
        );

        let recorded = recorder.0.lock().unwrap();
        let calls = tool_calls(&recorded, "canvas");
        assert_eq!(calls.len(), 3, "{mode}: one tool_call per canvas");
        for (index, at) in calls[..2].iter().enumerate() {
            let children: Vec<&Seen> = recorded
                .spans
                .iter()
                .filter(|seen| seen.name == "command" && seen.parent == Some(*at))
                .collect();
            assert!(
                children.len() == 1
                    && !children[0].fields.is_empty()
                    && children[0]
                        .fields
                        .iter()
                        .all(|(field, value)| *field == "verb" && value == "canvas"),
                "{mode}: call {index}: the accepted canvas's command records its verb alone"
            );
        }
    }
}

/// On each lean task in both layout modes, a tool call of `verb`, a read that takes no
/// argument, is one text block, the line `read` answers on a session booted the same way; one
/// naming an argument is refused malformed; and each tool call's span holds `verb` and the
/// accepted one parents one command recording its verb alone.
fn a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone(
    verb: &str,
    read: fn(&mut Session) -> String,
) {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        for (row, task) in TASKS.into_iter().enumerate() {
            let (recorder, replies) = served_read(verb, task, incremental, mode, row);

            let (mut direct, _ticks) = hold(task, incremental);
            assert!(
                result(&replies[1]) == (false, read(&mut direct)),
                "{mode}: task {row}: the text block is the line of the reader's own answer"
            );
            let (is_error, text) = result(&replies[2]);
            assert!(
                is_error && remedied(&text) == Some(Cause::Malformed),
                "{mode}: task {row}: a read naming an argument is refused malformed"
            );

            let recorded = recorder.0.lock().unwrap();
            let calls = tool_calls(&recorded, verb);
            assert_eq!(calls.len(), 2, "{mode}: task {row}: one tool_call per read");
            let accepted = recorded.spans[calls[0]]
                .fields
                .iter()
                .any(|(field, value)| *field == "ending" && value == "accepted");
            let children: Vec<&Seen> = recorded
                .spans
                .iter()
                .filter(|seen| seen.name == "command" && seen.parent == Some(calls[0]))
                .collect();
            assert!(
                accepted && children.len() == 1,
                "{mode}: task {row}: the read's tool_call is accepted and parents one command"
            );
            assert!(
                children[0]
                    .fields
                    .iter()
                    .all(|(field, value)| *field == "verb" && value == verb)
                    && !children[0].fields.is_empty(),
                "{mode}: task {row}: the read's command records its verb alone"
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
