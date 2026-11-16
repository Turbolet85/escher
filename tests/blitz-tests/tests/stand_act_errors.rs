//! An acting call's answer carries what its step caused, by node and handler: on a fixture page
//! booted with the stand's options, in both layout modes, a `click` whose handler panics answers
//! one `panic` item, a `click` whose handler returns `Err` one `error` item, a `click` whose
//! handler logs at `info` and `warn` two `log` items, a `type` whose `oninput` logs at `debug` a
//! `log` item, and a `press` Enter on the focused logging button its `click`'s items — each tied
//! to the stable id, owner id and event of the handler the step reached, closed words and counts
//! alone. An `advance`, a `scroll` and a click that caused nothing answer no `caused` key, and a
//! refused call carries none. After the caught panic the held session still answers and a click
//! acts. No answer line holds a byte of the panic payload or a log message; the `Err` the app's
//! root error boundary takes replaces the app with that boundary's own rendering of the error,
//! so its case runs on a session of its own and reads no byte of the error in `caused`. The same
//! `caused` crosses `serve_mcp` in process and a kept check, which replays holding, and a copy
//! with a count flipped stops at its step naming `caused`. The `command` span around a panicking
//! call keeps its eight fields and holds nothing of `caused`, read through a recorder this check
//! installs. A failure message names a layout mode and an index only.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_core::CapturedError;
use escher_driver::{
    Call, Cause, Caused, CausedKind, CausedLevel, Outcome, Refusal, Session, serve_mcp,
};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

mod fixtures;
mod session_common;
use session_common::{advance, click, press, scroll, type_into};

const PANIC_SENTINEL: &str = "sentinel-panic-payload-7f3a";
const ERR_SENTINEL: &str = "sentinel-err-text-91c2";
const INFO_SENTINEL: &str = "sentinel-info-message-4d0e";
const WARN_SENTINEL: &str = "sentinel-warn-message-b81f";
const DEBUG_SENTINEL: &str = "sentinel-debug-message-2a6c";

const SENTINELS: [&str; 5] = [
    PANIC_SENTINEL,
    ERR_SENTINEL,
    INFO_SENTINEL,
    WARN_SENTINEL,
    DEBUG_SENTINEL,
];

const LABEL: &str = "errors";

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// The panicking button's handler: a unit function, so the handler is no diverging closure.
fn fail() {
    panic!("{PANIC_SENTINEL}");
}

fn log_twice() {
    tracing::info!("{INFO_SENTINEL}");
    tracing::warn!("{WARN_SENTINEL}");
}

/// Buttons that panic, return `Err`, log and count, a logging input, and a scrolling box holding
/// one item out of view.
fn caused_page() -> Element {
    let mut clicks = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button { id: "panic-button", onclick: move |_| fail(), "Panic" }
            button {
                id: "err-button",
                onclick: move |_| -> Result<(), CapturedError> {
                    Err(CapturedError::from_display(ERR_SENTINEL))
                },
                "Err"
            }
            button { id: "log-button", onclick: move |_| log_twice(), "Log" }
            input {
                id: "log-input",
                oninput: move |_| tracing::debug!("{DEBUG_SENTINEL}"),
            }
            button { id: "count", onclick: move |_| clicks += 1, "{clicks}" }
            div {
                id: "scroll-box",
                style: "height: 40px; overflow: auto;",
                div { style: "height: 400px;" }
                button { id: "far-item", "Far" }
            }
        }
    }
}

fn hold(incremental: bool) -> Session {
    fixtures::hold(LABEL, caused_page, incremental)
}

/// The `caused` of an accepted acting answer, and its written line.
#[track_caller]
fn answered(session: &mut Session, call: &Call, at: &str) -> (Vec<Caused>, String) {
    let outcome = session
        .run(call)
        .unwrap_or_else(|_| panic!("{at}: the call is accepted"));
    let line = outcome.to_json();
    let caused = match outcome {
        Outcome::Acted { caused, .. }
        | Outcome::Advanced { caused, .. }
        | Outcome::Scrolled { caused, .. } => caused,
        _ => panic!("{at}: an acting answer"),
    };
    (caused, line)
}

/// The handler a step tied to, as the screen's UI graph reads it: its id, owner id and event.
fn handler(
    session: &Session,
    id: &str,
    event: &str,
) -> (Option<String>, Option<String>, Option<String>) {
    let owner = session
        .harness()
        .doc
        .graph()
        .nodes
        .into_iter()
        .find(|node| node.id == id)
        .and_then(|node| node.owner);
    (Some(id.to_string()), owner, Some(event.to_string()))
}

fn item(
    kind: CausedKind,
    level: CausedLevel,
    (id, owner, event): (Option<String>, Option<String>, Option<String>),
    count: u32,
) -> Caused {
    Caused {
        kind,
        level,
        id,
        owner,
        event,
        count,
    }
}

/// The written `caused` member of a line, from its key to the line's end.
fn caused_member(line: &str) -> Option<&str> {
    line.find(r#""caused":"#).map(|at| &line[at..])
}

#[track_caller]
fn holds_no_sentinel(text: &str, at: &str) {
    for (index, sentinel) in SENTINELS.iter().enumerate() {
        assert!(!text.contains(sentinel), "{at}: sentinel {index} is absent");
    }
}

/// The text of the `count` button, read from the held screen.
fn count_text(session: &Session) -> String {
    let doc = session.harness().doc.inner.borrow();
    doc.get_element_by_id("count")
        .and_then(|id| doc.get_node(id))
        .map(|node| node.text_content())
        .unwrap_or_default()
}

#[track_caller]
fn count_acts(session: &mut Session, expected: u32, at: &str) {
    let (caused, line) = answered(session, &click("count"), at);
    assert!(caused.is_empty(), "{at}: the count click caused nothing");
    assert!(caused_member(&line).is_none(), "{at}: no caused key");
    assert!(
        count_text(session) == expected.to_string(),
        "{at}: the count acted"
    );
}

#[test]
fn each_acting_call_answers_what_its_step_caused_by_its_handler() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);
        assert!(
            count_text(&session) == "0",
            "{mode}: the fixture boots with its count at 0"
        );

        // 0: a click whose handler panics.
        let at = format!("{mode}: 0");
        let tied = handler(&session, "panic-button", "click");
        let (caused, line) = answered(&mut session, &click("panic-button"), &at);
        assert!(
            caused == [item(CausedKind::Panic, CausedLevel::Error, tied, 1)],
            "{at}: one panic item tied to the button's click"
        );
        holds_no_sentinel(&line, &at);
        count_acts(&mut session, 1, &format!("{mode}: 0 after"));

        // 1: a click whose handler logs at info and warn.
        let at = format!("{mode}: 1");
        let tied = handler(&session, "log-button", "click");
        let (caused, line) = answered(&mut session, &click("log-button"), &at);
        assert!(
            caused
                == [
                    item(CausedKind::Log, CausedLevel::Warn, tied.clone(), 1),
                    item(CausedKind::Log, CausedLevel::Info, tied.clone(), 1),
                ],
            "{at}: warn then info, each once"
        );
        holds_no_sentinel(&line, &at);

        // 2: Enter on the focused logging button reads its click's items.
        let at = format!("{mode}: 2");
        let (caused, line) = answered(&mut session, &press("enter", false), &at);
        assert!(
            caused
                == [
                    item(CausedKind::Log, CausedLevel::Warn, tied.clone(), 1),
                    item(CausedKind::Log, CausedLevel::Info, tied, 1),
                ],
            "{at}: the press reads the click's items"
        );
        holds_no_sentinel(&line, &at);
        count_acts(&mut session, 2, &format!("{mode}: 2 after"));

        // 3: a type whose oninput logs at debug, once per character typed.
        let at = format!("{mode}: 3");
        let tied = handler(&session, "log-input", "input");
        let (caused, line) = answered(&mut session, &type_into("log-input", "ab"), &at);
        assert!(
            caused == [item(CausedKind::Log, CausedLevel::Debug, tied, 2)],
            "{at}: one debug item tied to the input"
        );
        holds_no_sentinel(&line, &at);
        count_acts(&mut session, 3, &format!("{mode}: 3 after"));

        // 4: an advance reaches no handler and causes nothing.
        let at = format!("{mode}: 4");
        let (caused, line) = answered(&mut session, &advance(100), &at);
        assert!(caused.is_empty(), "{at}: nothing caused");
        assert!(caused_member(&line).is_none(), "{at}: no caused key");

        // 5: a scroll reaches no handler and causes nothing.
        let at = format!("{mode}: 5");
        let (caused, line) = answered(&mut session, &scroll("far-item"), &at);
        assert!(caused.is_empty(), "{at}: nothing caused");
        assert!(caused_member(&line).is_none(), "{at}: no caused key");

        // 6: a refused call carries nothing.
        let at = format!("{mode}: 6");
        let refusal = session
            .run(&click("names-no-element"))
            .err()
            .unwrap_or_else(|| panic!("{at}: the call is refused"));
        assert!(
            refusal.cause() == Cause::NotFound,
            "{at}: refused not-found"
        );
        let line = Refusal::to_json(&refusal);
        assert!(caused_member(&line).is_none(), "{at}: no caused key");
        count_acts(&mut session, 4, &format!("{mode}: 6 after"));
    }
}

#[test]
fn an_err_the_root_boundary_takes_reads_one_error_item_on_a_session_of_its_own() {
    for incremental in [false, true] {
        let at = format!("{}: 0", mode_name(incremental));
        let mut session = hold(incremental);
        assert!(
            session.harness().doc.root_error_mark().is_none(),
            "{at}: no error stands at boot"
        );
        let tied = handler(&session, "err-button", "click");
        let (caused, line) = answered(&mut session, &click("err-button"), &at);
        assert!(
            caused == [item(CausedKind::Error, CausedLevel::Error, tied, 1)],
            "{at}: one error item tied to the button's click"
        );
        let member = caused_member(&line).unwrap_or_else(|| panic!("{at}: a caused key"));
        holds_no_sentinel(member, &at);
        assert!(
            session.harness().doc.root_error_mark().is_some(),
            "{at}: the root boundary holds the error"
        );
        let doc = session.harness().doc.inner.borrow();
        assert!(
            doc.get_element_by_id("count").is_none(),
            "{at}: the boundary's rendering replaced the app"
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

#[test]
fn the_same_caused_crosses_serve_mcp_in_process() {
    for incremental in [false, true] {
        let at = format!("{}: 0", mode_name(incremental));
        let mut session = hold(incremental);
        let (_, line) = answered(&mut session, &click("panic-button"), &at);
        let member = caused_member(&line).unwrap_or_else(|| panic!("{at}: a caused key"));

        let replies = serve(
            &[
                tool_call(1, "start", &format!(r#"{{"app":"{LABEL}"}}"#)),
                tool_call(2, "click", r#"{"id":"panic-button"}"#),
                tool_call(3, "stop", "{}"),
            ],
            incremental,
        );
        assert!(replies.len() == 3, "{at}: three replies");
        let text = replies[1].replace(r#"\""#, "\"");
        assert!(
            text.contains(r#""isError":false"#),
            "{at}: the click's result is no error"
        );
        assert!(
            text.contains(member.trim_end_matches('}')),
            "{at}: the tool result carries the same caused"
        );
        holds_no_sentinel(&replies[1], &at);
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_replays_its_caused_and_a_flipped_count_stops_naming_caused() {
    use escher_driver::Check;
    use session_common::state_dir;

    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let rows: [(&str, Vec<Call>); 2] = [
            ("panic", vec![click("panic-button"), click("count")]),
            ("err", vec![click("err-button")]),
        ];
        assert_eq!(rows.len(), 2);
        for (index, (name, calls)) in rows.into_iter().enumerate() {
            let at = format!("{mode}: {index}");
            let dir = state_dir(&format!("act-errors-{name}-{mode}"));
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
            // The root boundary renders the error on the screen, so the Err row reads its
            // kept caused alone.
            let caused_at = text
                .find(r#""caused":"#)
                .unwrap_or_else(|| panic!("{at}: the kept answer holds caused"));
            let caused_end = caused_at + text[caused_at..].find(']').unwrap_or(0);
            holds_no_sentinel(&text[caused_at..=caused_end], &at);
            if name == "panic" {
                holds_no_sentinel(text, &at);
            }
            let kind = format!(r#""kind":"{}""#, if name == "err" { "error" } else { name });
            let item = text
                .find(&kind)
                .unwrap_or_else(|| panic!("{at}: the kept answer holds the item"));

            let check =
                Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
            let replay = check.replay(&mut hold(incremental));
            assert!(replay.holds(), "{at}: the replay holds");
            assert!(replay.ran() == calls.len(), "{at}: every step ran");

            let count = item
                + text[item..]
                    .find(r#""count":1"#)
                    .unwrap_or_else(|| panic!("{at}: the item counts 1"));
            let flipped = format!(
                "{}{}{}",
                &text[..count],
                r#""count":2"#,
                &text[count + r#""count":1"#.len()..]
            );
            let check = Check::from_bytes(flipped.as_bytes())
                .unwrap_or_else(|_| panic!("{at}: the flipped check reads"));
            let replay = check.replay(&mut hold(incremental));
            let change = replay
                .change()
                .unwrap_or_else(|| panic!("{at}: the flipped check stops"));
            assert!(change.step() == 0, "{at}: it stops at its step");
            assert!(change.keys() == ["caused"], "{at}: naming caused");
        }
    }
}

/// One span the recorder saw: its name, its declared fields and the fields recorded on it.
#[derive(Clone, Default)]
struct Seen {
    name: &'static str,
    declared: Vec<&'static str>,
    recorded: Vec<(&'static str, String)>,
}

#[derive(Clone, Default)]
struct Recorder {
    spans: Arc<Mutex<Vec<Seen>>>,
}

struct Values<'a>(&'a mut Vec<(&'static str, String)>);

impl Visit for Values<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.push((field.name(), format!("{value:?}")));
    }
}

impl Subscriber for Recorder {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attributes: &Attributes<'_>) -> Id {
        let mut spans = self.spans.lock().unwrap_or_else(|e| e.into_inner());
        let mut seen = Seen {
            name: attributes.metadata().name(),
            declared: attributes
                .metadata()
                .fields()
                .iter()
                .map(|f| f.name())
                .collect(),
            recorded: Vec::new(),
        };
        attributes.record(&mut Values(&mut seen.recorded));
        spans.push(seen);
        Id::from_u64(spans.len() as u64)
    }

    fn record(&self, span: &Id, values: &Record<'_>) {
        let mut spans = self.spans.lock().unwrap_or_else(|e| e.into_inner());
        let index = usize::try_from(span.into_u64())
            .unwrap_or(0)
            .saturating_sub(1);
        if let Some(seen) = spans.get_mut(index) {
            values.record(&mut Values(&mut seen.recorded));
        }
    }

    fn record_follows_from(&self, _: &Id, _: &Id) {}
    fn event(&self, _: &Event<'_>) {}
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
}

#[test]
fn the_command_span_around_a_panicking_call_keeps_its_eight_fields() {
    const FIELDS: [&str; 8] = [
        "verb", "cause", "settled", "busy", "passes", "added", "removed", "changed",
    ];
    // The capture's dispatchers registered before the recorder: with the recorder alone
    // registered, another test thread reaching the command span first would cache it `never`.
    let _ = escher_telemetry::capture(|| ());
    for incremental in [false, true] {
        let at = format!("{}: 0", mode_name(incremental));
        let mut session = hold(incremental);
        let recorder = Recorder::default();
        let caused = tracing::subscriber::with_default(recorder.clone(), || {
            answered(&mut session, &click("panic-button"), &at).0
        });
        assert!(
            caused.len() == 1 && caused[0].kind == CausedKind::Panic,
            "{at}: the call caused one panic"
        );
        let spans = recorder.spans.lock().unwrap_or_else(|e| e.into_inner());
        let commands: Vec<&Seen> = spans.iter().filter(|seen| seen.name == "command").collect();
        assert!(commands.len() == 1, "{at}: one command span");
        let command = commands[0];
        assert!(command.declared == FIELDS, "{at}: its eight fields");
        for (index, (name, value)) in command.recorded.iter().enumerate() {
            assert!(
                FIELDS.contains(name),
                "{at}: recorded field {index} is one of the eight"
            );
            assert!(
                !value.contains("caused"),
                "{at}: recorded field {index} holds no caused"
            );
            holds_no_sentinel(value, &at);
        }
        assert!(
            command
                .recorded
                .iter()
                .any(|(name, value)| *name == "verb" && value == "\"click\""),
            "{at}: its verb is click"
        );
    }
}
