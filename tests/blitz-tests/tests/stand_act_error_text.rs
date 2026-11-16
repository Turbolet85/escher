//! An agent reading an acting answer alone can say what failed, where, and from which source: on
//! a fixture page whose controls fail as apps fail, booted with the stand's options, in both
//! layout modes, each failing click's written line carries values — a panic's payload and its
//! location in this file, an `Err`'s top line, its whole three-link cause chain and the boundary
//! that took it (the root one, or one of the app's own), the UI framework's render error naming
//! the component and its error, and, in a build that compiled the engine's `tracing`, the
//! engine's two warnings, each message whole — each record with its source and target, and the
//! handler's id and event. The scrub hides by field name alone: a `password` and a `value` field
//! read `[redacted]`, and so do the engine's `href` and `url` and the style's `value`, while every
//! message and the other fields cross whole; a typed password occurs nowhere in the answer, and
//! the raw `href` nowhere either. A flood of 40 records of 3000 bytes, 12 fields each, answers 16
//! items, each message cut with its cut said, its fields past 8 counted, and the rest counted
//! by kind and source, the whole line under the carriers' 1,048,576-byte bound. The same `caused`
//! crosses `serve_mcp` in process, the session socket (unix: a host child of this test binary)
//! and a kept check that replays holding, a copy with a cut changed stopping at its step naming
//! `caused`. Under escher's own sink at `trace` no sentinel, payload or raw `href` reaches a line.
//! A failure message names a layout mode, a build and an index only.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_core::{CapturedError, ErrorContext};
use dioxus_native_dom::{Boundary, MASKED_VALUE};
use escher_driver::{
    Call, Caused, CausedKind, CausedLevel, CausedPast, Outcome, Session, serve_mcp,
};
use escher_telemetry::{FIELDS_KEPT, REDACTED, Source};

mod fixtures;
mod session_common;
use session_common::{click, type_into};

const LABEL: &str = "error-text";

const WARN_MESSAGE: &str = "booking rejected: 4 seats over the limit";
const PASSWORD_SENTINEL: &str = "sentinel-password-field-3c7e";
const TYPED: &str = "pw7k2q";
const RAW_HREF: &str = "http://[::1";
const INDEX_PAYLOAD: &str = "index out of bounds: the len is 0 but the index is 3";
const UNWRAP_PAYLOAD: &str =
    "called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }";
const LOAD_ERROR: &str = "could not load the booking form";
const SETTINGS_ERROR: &str = "invalid value for setting `seats`";
const PARSE_ERROR: &str = "invalid digit found in string";
const LINK_MESSAGE: &str = "Clicked link's href is not parseable as a url";
const STYLE_MESSAGE: &str = "Invalid property value";
const FLOOD_RECORDS: usize = 40;
const FLOOD_BYTES: usize = 3000;
const FLOOD_FIELDS: usize = 12;
const ANSWER_BOUND: usize = 1_048_576;
/// The cut the measurement set: a text past it arrives as its first 1024 bytes.
const CUT: usize = 1024;

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

fn build_name() -> &'static str {
    if blitz_dom::TRACING_COMPILED {
        "engine-tracing"
    } else {
        "package-alone"
    }
}

#[derive(Debug)]
struct SettingsError {
    source: std::num::ParseIntError,
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(SETTINGS_ERROR)
    }
}

impl std::error::Error for SettingsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct LoadError {
    source: SettingsError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(LOAD_ERROR)
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn parse_seats(text: &str) -> Result<u32, LoadError> {
    text.parse::<u32>().map_err(|source| LoadError {
        source: SettingsError { source },
    })
}

const INDEX_LINE: u32 = line!() + 3;
fn index_past_end() {
    let rows: Vec<u32> = Vec::new();
    let _ = rows[3];
}

const UNWRAP_LINE: u32 = line!() + 2;
fn unwrap_parse() {
    let _: u32 = "twelve".parse().unwrap();
}

fn warn_booking() {
    tracing::warn!(seats = 4, password = PASSWORD_SENTINEL, "{WARN_MESSAGE}");
}

fn flood() {
    let text = "q".repeat(FLOOD_BYTES);
    for _ in 0..FLOOD_RECORDS {
        tracing::info!(
            f0 = 0,
            f1 = 1,
            f2 = 2,
            f3 = 3,
            f4 = 4,
            f5 = 5,
            f6 = 6,
            f7 = 7,
            f8 = 8,
            f9 = 9,
            f10 = 10,
            f11 = 11,
            "{text}"
        );
    }
}

fn failing_child() -> Element {
    let seats: u32 = "seven".parse()?;
    rsx! { p { "{seats}" } }
}

#[component]
fn OwnErr() -> Element {
    rsx! {
        button {
            id: "own-err",
            onclick: move |_| -> Result<(), CapturedError> {
                parse_seats("4x")?;
                Ok(())
            },
            "Own Err"
        }
    }
}

/// Controls that fail as apps fail: each clicked element a block box with a size.
fn real_failures() -> Element {
    let mut mounted = use_signal(|| false);
    let mut bad = use_signal(|| false);
    rsx! {
        div { id: "fx-root",
            button { id: "idx", onclick: move |_| index_past_end(), "Index" }
            button { id: "unwrap", onclick: move |_| unwrap_parse(), "Unwrap" }
            button {
                id: "root-err",
                onclick: move |_| -> Result<(), CapturedError> {
                    parse_seats("many")?;
                    Ok(())
                },
                "Root Err"
            }
            ErrorBoundary {
                handle_error: |_: ErrorContext| rsx! { div { id: "own-fallback", "Own fallback" } },
                OwnErr {}
            }
            button { id: "mount-bad", onclick: move |_| mounted.set(true), "Mount" }
            ErrorBoundary {
                handle_error: |_: ErrorContext| rsx! { div { id: "render-fallback", "Render fallback" } },
                if mounted() {
                    failing_child {}
                }
            }
            a {
                id: "bad-link",
                href: RAW_HREF,
                style: "display: block; width: 120px; height: 24px;",
                "Bad link"
            }
            button { id: "bad-style", onclick: move |_| bad.set(true), "Style" }
            div { id: "styled", color: if bad() { "notacolour" } else { "red" }, "Styled" }
            button { id: "warn-button", onclick: move |_| warn_booking(), "Warn" }
            input {
                id: "pw",
                r#type: "password",
                oninput: move |evt| tracing::info!(value = %evt.value(), "password typed"),
            }
            button { id: "flood", onclick: move |_| flood(), "Flood" }
        }
    }
}

fn hold(incremental: bool) -> Session {
    fixtures::hold(LABEL, real_failures, incremental)
}

/// The `caused` and `caused_past` of an accepted acting answer, and its written line.
#[track_caller]
fn answered(
    session: &mut Session,
    call: &Call,
    at: &str,
) -> (Vec<Caused>, Vec<CausedPast>, String) {
    let outcome = session
        .run(call)
        .unwrap_or_else(|_| panic!("{at}: the call is accepted"));
    let line = outcome.to_json();
    match outcome {
        Outcome::Acted {
            caused,
            caused_past,
            ..
        } => (caused, caused_past, line),
        _ => panic!("{at}: an acting answer"),
    }
}

/// A string as the answer writes it.
fn written(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if control < '\u{20}' => {
                out.push_str(&format!("\\u{:04x}", u32::from(control)))
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// Asserts the line writes `"key":value`, `value` a string.
#[track_caller]
fn writes(line: &str, key: &str, value: &str, at: &str) {
    let member = format!("{}:{}", written(key), written(value));
    assert!(
        line.contains(&member),
        "{at}: the line writes the member read back"
    );
}

/// Asserts the line writes `"key":n`.
#[track_caller]
fn writes_count(line: &str, key: &str, value: u64, at: &str) {
    let member = format!("{}:{value}", written(key));
    assert!(
        line.contains(&member),
        "{at}: the line writes the count read back"
    );
}

/// The written `caused` member of a line, from its key to the line's end.
fn caused_member(line: &str) -> &str {
    line.find(r#""caused":"#).map_or("", |at| &line[at..])
}

#[track_caller]
fn tied_to(caused: &[Caused], id: &str, at: &str) {
    for (index, item) in caused.iter().enumerate() {
        assert!(
            item.id.as_deref() == Some(id) && item.event.as_deref() == Some("click"),
            "{at}: item {index} is tied to the handler's id and event"
        );
    }
}

#[track_caller]
fn the_panic(caused: &[Caused], line: &str, payload: &str, at_line: u32, at: &str) {
    let panics: Vec<&Caused> = caused
        .iter()
        .filter(|item| item.kind == CausedKind::Panic)
        .collect();
    assert!(panics.len() == 1, "{at}: one panic item");
    let panic = panics[0];
    assert!(panic.level == CausedLevel::Error, "{at}: at error");
    assert!(panic.message.text == payload, "{at}: its payload");
    assert!(panic.file.as_deref() == Some(file!()), "{at}: in this file");
    assert!(panic.line == Some(at_line), "{at}: at the failing line");
    writes(line, "kind", "panic", at);
    writes(line, "message", payload, at);
    writes(line, "file", file!(), at);
    writes_count(line, "line", u64::from(at_line), at);
}

#[track_caller]
fn the_chain(item: &Caused, line: &str, boundary: Boundary, at: &str) {
    assert!(item.kind == CausedKind::Error, "{at}: an error item");
    assert!(item.message.text == LOAD_ERROR, "{at}: its top line");
    assert!(
        item.boundary == Some(boundary),
        "{at}: the boundary that took it"
    );
    let chain: Vec<&str> = item.chain.iter().map(|link| link.text.as_str()).collect();
    assert!(
        chain == [SETTINGS_ERROR, PARSE_ERROR],
        "{at}: its whole chain in order"
    );
    let word = escher_driver::boundary_word(boundary);
    let written_chain = format!(
        r#""message":{},"boundary":{},"chain":[{{"message":{}}},{{"message":{}}}],"chain_past":0"#,
        written(LOAD_ERROR),
        written(word),
        written(SETTINGS_ERROR),
        written(PARSE_ERROR)
    );
    assert!(
        line.contains(&written_chain),
        "{at}: the chain written in order"
    );
}

/// The engine's one item of a step, in a build that compiled its `tracing`; asserts none where
/// it did not.
#[track_caller]
fn the_engine_item<'a>(
    caused: &'a [Caused],
    target: &str,
    message: &str,
    at: &str,
) -> Option<&'a Caused> {
    let engine: Vec<&Caused> = caused
        .iter()
        .filter(|item| item.source == Some(Source::Engine))
        .collect();
    if !blitz_dom::TRACING_COMPILED {
        assert!(engine.is_empty(), "{at}: no engine item in this build");
        return None;
    }
    let found: Vec<&&Caused> = engine
        .iter()
        .filter(|item| item.target.as_deref() == Some(target))
        .collect();
    assert!(found.len() == 1, "{at}: one engine item at its target");
    let item = *found[0];
    assert!(
        item.kind == CausedKind::Log && item.level == CausedLevel::Warn,
        "{at}: a warn record"
    );
    assert!(
        item.message.text == message && item.message.cut == 0,
        "{at}: its message whole"
    );
    Some(item)
}

fn field<'a>(item: &'a Caused, name: &str) -> Option<&'a str> {
    item.fields
        .iter()
        .find(|field| field.name == name)
        .map(|field| field.value.text.as_str())
}

/// Runs every failing step of the main session, asserting each.
fn every_failure(session: &mut Session, mode: &str) -> Vec<String> {
    let build = build_name();
    let mut lines = Vec::new();

    // 0: a click indexing past a Vec's end.
    let at = format!("{mode} {build}: 0");
    let (caused, _, line) = answered(session, &click("idx"), &at);
    the_panic(&caused, &line, INDEX_PAYLOAD, INDEX_LINE, &at);
    tied_to(&caused, "idx", &at);
    writes(&line, "id", "idx", &at);
    writes(&line, "event", "click", &at);
    lines.push(line);

    // 1: a click that unwraps a failed parse.
    let at = format!("{mode} {build}: 1");
    let (caused, _, line) = answered(session, &click("unwrap"), &at);
    the_panic(&caused, &line, UNWRAP_PAYLOAD, UNWRAP_LINE, &at);
    tied_to(&caused, "unwrap", &at);
    lines.push(line);

    // 2: an Err under a boundary of the app's own.
    let at = format!("{mode} {build}: 2");
    let (caused, _, line) = answered(session, &click("own-err"), &at);
    let errors: Vec<&Caused> = caused
        .iter()
        .filter(|item| item.kind == CausedKind::Error)
        .collect();
    assert!(errors.len() == 1, "{at}: one error item");
    the_chain(errors[0], &line, Boundary::Own, &at);
    tied_to(&caused, "own-err", &at);
    assert!(
        session
            .harness()
            .doc
            .inner
            .borrow()
            .get_element_by_id("own-fallback")
            .is_some(),
        "{at}: the app's own boundary rendered its fallback"
    );
    lines.push(line);

    // 3: a component whose render returns Err, mounted under a boundary of the app's own.
    let at = format!("{mode} {build}: 3");
    let (caused, _, line) = answered(session, &click("mount-bad"), &at);
    let framework: Vec<&Caused> = caused
        .iter()
        .filter(|item| item.source == Some(Source::Framework))
        .collect();
    assert!(framework.len() == 1, "{at}: one framework item");
    let item = framework[0];
    assert!(
        item.kind == CausedKind::Log && item.level == CausedLevel::Error,
        "{at}: an error record"
    );
    assert!(
        item.target.as_deref() == Some("dioxus_core::scope_arena"),
        "{at}: the framework's target"
    );
    let expected = format!(
        "Error while rendering component `{}::failing_child`: {PARSE_ERROR}",
        module_path!()
    );
    assert!(
        item.message.text == expected,
        "{at}: the component and its error"
    );
    assert!(
        item.file
            .as_deref()
            .is_some_and(|file| file.ends_with("scope_arena.rs")),
        "{at}: the framework's file"
    );
    writes(&line, "source", "framework", &at);
    writes(&line, "target", "dioxus_core::scope_arena", &at);
    writes(&line, "message", &expected, &at);
    tied_to(&caused, "mount-bad", &at);
    lines.push(line);

    // 4: a link whose href will not parse.
    let at = format!("{mode} {build}: 4");
    let (caused, _, line) = answered(session, &click("bad-link"), &at);
    if let Some(item) = the_engine_item(&caused, "blitz_dom::events::pointer", LINK_MESSAGE, &at) {
        assert!(field(item, "href") == Some(REDACTED), "{at}: href hidden");
        assert!(field(item, "url") == Some(REDACTED), "{at}: url hidden");
        assert!(
            item.file
                .as_deref()
                .is_some_and(|file| file.ends_with("pointer.rs")),
            "{at}: the engine's own file"
        );
        writes(&line, "source", "engine", &at);
        writes(&line, "message", LINK_MESSAGE, &at);
        writes(&line, "target", "blitz_dom::events::pointer", &at);
        assert!(
            item.id.is_none() && item.event.is_none(),
            "{at}: a link with no handler of the app's ties to none"
        );
    }
    assert!(
        !line.contains(RAW_HREF),
        "{at}: the raw href occurs 0 times"
    );
    lines.push(line);

    // 5: an invalid inline style value.
    let at = format!("{mode} {build}: 5");
    let (caused, _, line) = answered(session, &click("bad-style"), &at);
    if let Some(item) = the_engine_item(&caused, "blitz_dom::node::element", STYLE_MESSAGE, &at) {
        assert!(
            field(item, "property") == Some("color"),
            "{at}: property whole"
        );
        assert!(field(item, "value") == Some(REDACTED), "{at}: value hidden");
        assert!(
            item.file
                .as_deref()
                .is_some_and(|file| file.ends_with("element.rs")),
            "{at}: the engine's own file"
        );
        writes(&line, "message", STYLE_MESSAGE, &at);
    }
    assert!(
        !line.contains("notacolour"),
        "{at}: the style value occurs 0 times"
    );
    lines.push(line);

    // 6: a warn record with a seats and a password field.
    let at = format!("{mode} {build}: 6");
    let (caused, _, line) = answered(session, &click("warn-button"), &at);
    assert!(caused.len() == 1, "{at}: one item");
    let item = &caused[0];
    assert!(
        item.kind == CausedKind::Log
            && item.level == CausedLevel::Warn
            && item.source == Some(Source::App)
            && item.target.as_deref() == Some(module_path!()),
        "{at}: an app warn record under this crate's target"
    );
    assert!(item.message.text == WARN_MESSAGE, "{at}: its message whole");
    assert!(field(item, "seats") == Some("4"), "{at}: seats whole");
    assert!(
        field(item, "password") == Some(REDACTED),
        "{at}: password hidden"
    );
    assert!(
        item.file.as_deref() == Some(file!()),
        "{at}: logged in this file"
    );
    writes(&line, "message", WARN_MESSAGE, &at);
    writes(&line, "source", "app", &at);
    assert!(
        line.contains(r#"{"name":"seats","value":"4"},{"name":"password","value":"[redacted]"}"#),
        "{at}: the fields written with the hidden one's name"
    );
    assert!(
        !line.contains(PASSWORD_SENTINEL),
        "{at}: the password occurs 0 times"
    );
    tied_to(&caused, "warn-button", &at);
    lines.push(line);

    // 7: a password input whose oninput logs its value in a field named value.
    let at = format!("{mode} {build}: 7");
    let (caused, _, line) = answered(session, &type_into("pw", TYPED), &at);
    assert!(
        caused.len() == TYPED.len(),
        "{at}: one item per character typed"
    );
    for (index, item) in caused.iter().enumerate() {
        assert!(
            item.message.text == "password typed",
            "{at}: item {index} message"
        );
        assert!(
            field(item, "value") == Some(REDACTED),
            "{at}: item {index} value hidden"
        );
    }
    assert!(!line.contains(TYPED), "{at}: the typed text occurs 0 times");
    assert!(
        line.contains(MASKED_VALUE),
        "{at}: the snapshot's value reads the mask"
    );
    lines.push(line);

    lines
}

#[test]
fn each_failure_answers_what_failed_where_and_from_which_source() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);
        assert!(
            session.harness().doc.boundary_marks().is_empty(),
            "{mode}: no error stands at boot"
        );
        every_failure(&mut session, mode);
    }
}

#[test]
fn an_err_the_root_boundary_takes_answers_its_chain_on_a_session_of_its_own() {
    for incremental in [false, true] {
        let at = format!("{} {}: 0", mode_name(incremental), build_name());
        let mut session = hold(incremental);
        let (caused, _, line) = answered(&mut session, &click("root-err"), &at);
        assert!(caused.len() == 1, "{at}: one item");
        the_chain(&caused[0], &line, Boundary::Root, &at);
        tied_to(&caused, "root-err", &at);
        assert!(
            session
                .harness()
                .doc
                .inner
                .borrow()
                .get_element_by_id("idx")
                .is_none(),
            "{at}: the root boundary's rendering replaced the app"
        );
    }
}

#[test]
fn a_flood_is_cut_counted_and_bounded() {
    for incremental in [false, true] {
        let at = format!("{} {}: 0", mode_name(incremental), build_name());
        let mut session = hold(incremental);
        let (caused, past, line) = answered(&mut session, &click("flood"), &at);
        assert!(caused.len() == escher_driver::CAUSED_KEPT, "{at}: 16 items");
        for (index, item) in caused.iter().enumerate() {
            assert!(
                item.message.text.len() == CUT && item.message.cut == FLOOD_BYTES - CUT,
                "{at}: item {index} cut with its cut said"
            );
            assert!(
                item.fields.len() == FIELDS_KEPT,
                "{at}: item {index} keeps 8 fields"
            );
            assert!(
                item.fields_past as usize == FLOOD_FIELDS - FIELDS_KEPT,
                "{at}: item {index} counts the rest"
            );
        }
        let rest = u32::try_from(FLOOD_RECORDS - escher_driver::CAUSED_KEPT).unwrap_or(0);
        assert!(
            past == [CausedPast {
                kind: CausedKind::Log,
                source: Some(Source::App),
                count: rest,
            }],
            "{at}: the rest counted under app"
        );
        writes_count(&line, "cut", (FLOOD_BYTES - CUT) as u64, &at);
        writes_count(
            &line,
            "fields_past",
            (FLOOD_FIELDS - FIELDS_KEPT) as u64,
            &at,
        );
        assert!(
            line.contains(&format!(
                r#""caused_past":[{{"kind":"log","source":"app","count":{rest}}}]"#
            )),
            "{at}: caused_past written"
        );
        let length = line.len();
        assert!(
            length < ANSWER_BOUND,
            "{at}: the line, {length} bytes, is under the bound"
        );
    }
}

/// The replies `serve_mcp` writes for `requests`, one per line.
fn serve(requests: &[String], incremental: bool) -> Vec<String> {
    let mut input = requests.join("\n");
    input.push('\n');
    let mut output = Vec::new();
    serve_mcp(
        input.as_bytes(),
        &mut output,
        &[LABEL],
        |_: &str| Ok(hold(incremental)),
        Duration::from_secs(1800),
    )
    .expect("the exchange ends");
    String::from_utf8(output)
        .expect("replies are text")
        .lines()
        .map(str::to_string)
        .collect()
}

fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

/// The text block of an MCP tool result, read back from its JSON string.
fn result_text(reply: &str) -> String {
    let key = r#""text":""#;
    let Some(start) = reply.find(key) else {
        return String::new();
    };
    let mut out = String::new();
    let mut characters = reply[start + key.len()..].chars();
    while let Some(character) = characters.next() {
        match character {
            '"' => break,
            '\\' => match characters.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('u') => {
                    let hex: String = characters.by_ref().take(4).collect();
                    if let Some(decoded) =
                        u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32)
                    {
                        out.push(decoded);
                    }
                }
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    out
}

const CROSSING: [&str; 2] = ["idx", "warn-button"];

#[test]
fn the_same_caused_crosses_serve_mcp_in_process() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);
        let mut requests = vec![tool_call(1, "start", &format!(r#"{{"app":"{LABEL}"}}"#))];
        for (index, id) in CROSSING.iter().enumerate() {
            requests.push(tool_call(
                index + 2,
                "click",
                &format!(r#"{{"id":"{id}"}}"#),
            ));
        }
        requests.push(tool_call(9, "stop", "{}"));
        let replies = serve(&requests, incremental);
        assert!(
            replies.len() == CROSSING.len() + 2,
            "{mode}: a reply per request"
        );
        for (index, id) in CROSSING.iter().enumerate() {
            let at = format!("{mode}: {index}");
            let (_, _, line) = answered(&mut session, &click(id), &at);
            let text = result_text(&replies[index + 1]);
            assert!(
                replies[index + 1].contains(r#""isError":false"#),
                "{at}: the result is no error"
            );
            assert!(
                caused_member(&text) == caused_member(&line) && !caused_member(&line).is_empty(),
                "{at}: the tool result carries the same caused"
            );
        }
    }
}

#[cfg(unix)]
const HOST_DIRS: [&str; 2] = ["aet-full", "aet-incr"];

#[cfg(unix)]
#[test]
fn the_same_caused_crosses_the_session_socket() {
    use escher_driver::{call, start, stop};
    use session_common::{Host, READY, clear, host_command, state_dir};

    let children = ["error_text_host_full", "error_text_host_incremental"];
    for (row, incremental) in [false, true].into_iter().enumerate() {
        let mode = mode_name(incremental);
        let dir = state_dir(HOST_DIRS[row]);
        clear(&dir);
        let started = start(&dir, host_command(children[row]), READY)
            .unwrap_or_else(|_| panic!("{mode}: the host starts"));
        let mut host = Host::new(started.child);
        let mut session = hold(incremental);
        for (index, id) in CROSSING.iter().enumerate() {
            let at = format!("{mode}: {index}");
            let answer =
                call(&dir, &click(id)).unwrap_or_else(|_| panic!("{at}: the socket answers"));
            assert!(answer.accepted, "{at}: accepted");
            let (_, _, line) = answered(&mut session, &click(id), &at);
            assert!(
                caused_member(&answer.json) == caused_member(&line)
                    && !caused_member(&line).is_empty(),
                "{at}: the socket carries the same caused"
            );
        }
        assert!(stop(&dir).is_ok(), "{mode}: the host stops");
        host.kill();
    }
}

#[cfg(unix)]
fn serve_host(row: usize, incremental: bool) {
    use session_common::state_dir;
    escher_driver::serve(
        &state_dir(HOST_DIRS[row]),
        hold(incremental),
        Duration::from_secs(120),
    )
    .expect("the host serves until it is stopped");
}

#[cfg(unix)]
#[test]
#[ignore = "spawned as the session host by the_same_caused_crosses_the_session_socket"]
fn error_text_host_full() {
    serve_host(0, false);
}

#[cfg(unix)]
#[test]
#[ignore = "spawned as the session host by the_same_caused_crosses_the_session_socket"]
fn error_text_host_incremental() {
    serve_host(1, true);
}

#[cfg(unix)]
#[test]
fn a_kept_check_replays_its_caused_and_a_changed_cut_stops_naming_caused() {
    use escher_driver::Check;
    use session_common::state_dir;

    let calls = [click("idx"), click("warn-button"), click("flood")];
    for incremental in [false, true] {
        let at = format!("{}: 0", mode_name(incremental));
        let dir = state_dir(&format!("aet-check-{}", mode_name(incremental)));
        let _ = std::fs::remove_dir_all(&dir);
        {
            let mut session = hold(incremental)
                .recording_into(&dir)
                .unwrap_or_else(|_| panic!("{at}: the recording starts"));
            for call in &calls {
                assert!(session.run(call).is_ok(), "{at}: each step is accepted");
            }
            assert!(!session.recording_failed(), "{at}: the recording stands");
        }
        let bytes = std::fs::read(dir.join("check-1.json"))
            .unwrap_or_else(|_| panic!("{at}: the check is kept"));
        let _ = std::fs::remove_dir_all(&dir);
        let text = std::str::from_utf8(&bytes).expect("a check is text");
        assert!(
            text.contains(INDEX_PAYLOAD),
            "{at}: the kept answer holds the payload"
        );
        assert!(
            text.contains(WARN_MESSAGE),
            "{at}: the kept answer holds the message"
        );

        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        let replay = check.replay(&mut hold(incremental));
        assert!(replay.holds(), "{at}: the replay holds");
        assert!(replay.ran() == calls.len(), "{at}: every step ran");

        let cut = format!(r#""cut":{}"#, FLOOD_BYTES - CUT);
        let first = text
            .find(&cut)
            .unwrap_or_else(|| panic!("{at}: the flood's cut is kept"));
        let changed = format!(
            "{}{}{}",
            &text[..first],
            format_args!(r#""cut":{}"#, FLOOD_BYTES - CUT - 1),
            &text[first + cut.len()..]
        );
        let check = Check::from_bytes(changed.as_bytes())
            .unwrap_or_else(|_| panic!("{at}: the changed check reads"));
        let replay = check.replay(&mut hold(incremental));
        let change = replay
            .change()
            .unwrap_or_else(|| panic!("{at}: the changed check stops"));
        assert!(change.step() == 2, "{at}: it stops at the flood's step");
        assert!(change.keys() == ["caused"], "{at}: naming caused");
    }
}

/// An in-memory writer the sink writes its lines to.
#[derive(Clone, Default)]
struct Lines(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Lines {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn no_sentinel_payload_or_raw_href_reaches_the_sinks_lines() {
    let needles = [
        PASSWORD_SENTINEL,
        TYPED,
        RAW_HREF,
        INDEX_PAYLOAD,
        "twelve",
        LOAD_ERROR,
        SETTINGS_ERROR,
        WARN_MESSAGE,
        "notacolour",
        "qqqqqqqq",
    ];
    // The capture's dispatchers registered before the sink: with the sink alone registered,
    // another test thread reaching a call site first would cache it from its own default.
    let _ = escher_telemetry::capture(|| ());
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let lines = Lines::default();
        let writer = lines.clone();
        let sink = tracing::Dispatch::new(escher_telemetry::subscriber(
            escher_telemetry::service_identity!(),
            move || writer.clone(),
            tracing::level_filters::LevelFilter::TRACE,
        ));
        tracing::dispatcher::with_default(&sink, || {
            let mut session = hold(incremental);
            every_failure(&mut session, mode);
            let at = format!("{mode} {}: flood", build_name());
            answered(&mut session, &click("flood"), &at);
            let mut root = hold(incremental);
            answered(&mut root, &click("root-err"), &format!("{mode}: root"));
        });
        let written = lines
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        let written = String::from_utf8_lossy(&written);
        assert!(
            written.contains(r#"span="command""#),
            "{mode} {}: the sink wrote its command lines",
            build_name()
        );
        for (index, needle) in needles.iter().enumerate() {
            assert!(
                written.matches(needle).count() == 0,
                "{mode} {}: needle {index} occurs 0 times",
                build_name()
            );
        }
    }
}
