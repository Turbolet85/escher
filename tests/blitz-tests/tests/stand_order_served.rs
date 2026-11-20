//! The order page the stand serves answers, per control, what its step caused: booted through
//! `stand::boot_order`, in both layout modes, its snapshot text reads under the budget, its lint
//! no finding and every actionable element a key. A Submit with the quantity at `0` or `5` answers
//! one `error` item its own boundary took, holding the quantity rule, the screen showing only the
//! failure and a Retry; a Retry answers no `caused` key and brings Submit back; Enter on the
//! focused Submit at `12` answers one `info` log of the page's own, its message whole and no
//! field, and places the order; Export answers one `panic` item holding its payload and its line
//! in the page's file, and the session still acts after it. Each item is tied to its control's
//! id, owner and event, and no written line holds `[redacted]`. The same `caused` crosses
//! `serve_mcp` in process, and a kept check of those steps replays holding. A failure message
//! names a layout mode and an index only.

use dioxus_native_dom::{Boundary, SNAPSHOT_TEXT_BUDGET};
use escher_driver::{Call, Caused, CausedKind, CausedLevel, Outcome, Session};
use escher_telemetry::{REDACTED, Source};
use seven_guis::stand;
use seven_guis::tasks::order::{EXPORT_PANIC_LINE, EXPORT_UNBUILT, ORDER_PLACED, QUANTITY_RULE};

mod session_common;
use session_common::{
    click, focused, lint, linted, mcp_result_text, press, serve_lines, tool_call, type_into,
};

const LABEL: &str = "order";

/// The page module's path, the target of the record it logs.
const TARGET: &str = "seven_guis::tasks::order";

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

fn hold(incremental: bool) -> Session {
    Session::start(LABEL, || stand::boot_order(stand::options(incremental)))
        .expect("the label is one of the closed set")
}

/// The `caused` and the diff's added and removed ids of an accepted acting answer, and its
/// written line.
#[track_caller]
fn answered(session: &mut Session, call: &Call, at: &str) -> (Vec<Caused>, Vec<String>, String) {
    let outcome = session
        .run(call)
        .unwrap_or_else(|_| panic!("{at}: the call is accepted"));
    let line = outcome.to_json();
    assert!(!line.contains(REDACTED), "{at}: no field reads redacted");
    let (caused, diff) = match outcome {
        Outcome::Acted { caused, diff, .. } => (caused, diff),
        _ => panic!("{at}: an acting answer"),
    };
    let mut moved: Vec<String> = diff.added.iter().map(|node| node.id.clone()).collect();
    moved.extend(diff.removed.iter().map(|id| format!("-{id}")));
    (caused, moved, line)
}

/// What an item reads: its kind, level, source, target, message, and the handler it is tied to.
type Read = (
    CausedKind,
    CausedLevel,
    Option<Source>,
    Option<String>,
    String,
    (Option<String>, Option<String>, Option<String>),
);

fn read(caused: &[Caused]) -> Vec<Read> {
    caused
        .iter()
        .map(|item| {
            (
                item.kind,
                item.level,
                item.source,
                item.target.clone(),
                item.message.text.clone(),
                (item.id.clone(), item.owner.clone(), item.event.clone()),
            )
        })
        .collect()
}

/// The handler an item is tied to: the control's id and its `click`, with no owner — the page
/// keys every control by its own HTML `id` under no keyed element, so the UI graph reads none.
fn tied(id: &str) -> (Option<String>, Option<String>, Option<String>) {
    (Some(id.to_string()), None, Some("click".to_string()))
}

fn quantity_error() -> Read {
    (
        CausedKind::Error,
        CausedLevel::Error,
        None,
        None,
        QUANTITY_RULE.to_string(),
        tied("order-submit"),
    )
}

/// The written `caused` member of a line, from its key to the line's end.
fn caused_member(line: &str) -> Option<&str> {
    line.find(r#""caused":"#).map(|at| &line[at..])
}

/// The text of the element `id`, read from the held screen.
fn text_of(session: &Session, id: &str) -> Option<String> {
    let doc = session.harness().doc.inner.borrow();
    doc.get_element_by_id(id)
        .and_then(|node| doc.get_node(node))
        .map(|node| node.text_content())
}

#[test]
fn the_order_page_is_keyed_within_budget_and_reads_no_finding_at_boot() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);
        assert!(
            session.harness().doc.snapshot().to_text().len() < SNAPSHOT_TEXT_BUDGET,
            "{mode}: the snapshot text reads under the budget"
        );
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element reads an author key"
        );
        assert!(
            linted(&mut session, &lint()).is_some_and(|findings| findings.is_empty()),
            "{mode}: the lint reads no finding"
        );
    }
}

#[test]
fn each_control_answers_what_it_caused() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);

        let graph = session.harness().doc.graph();
        assert!(
            ["order-submit", "order-export"].iter().all(|id| graph
                .nodes
                .iter()
                .any(|node| node.id == *id && node.owner.is_none())),
            "{mode}: the graph reads the controls with no owner"
        );

        // 0: Submit at the boot quantity fails, the page's own boundary taking it.
        let at = format!("{mode}: 0");
        let (caused, moved, line) = answered(&mut session, &click("order-submit"), &at);
        assert!(
            read(&caused) == [quantity_error()],
            "{at}: one error item holding the rule"
        );
        assert!(
            caused[0].boundary == Some(Boundary::Own) && caused[0].chain.is_empty(),
            "{at}: the page's own boundary took it, a chain of none"
        );
        assert!(line.contains(r#""boundary":"own""#), "{at}: written own");
        assert!(
            moved == ["order-failed", "order-retry", "-order-submit"],
            "{at}: the failure and Retry replace Submit"
        );
        assert!(
            text_of(&session, "order-failed").as_deref() == Some("Order failed"),
            "{at}: the screen shows the failure"
        );

        // 1: Retry clears the boundary and causes nothing.
        let at = format!("{mode}: 1");
        let (caused, moved, line) = answered(&mut session, &click("order-retry"), &at);
        assert!(caused.is_empty(), "{at}: nothing caused");
        assert!(caused_member(&line).is_none(), "{at}: no caused key");
        assert!(
            moved == ["order-submit", "-order-failed", "-order-retry"],
            "{at}: Submit is back"
        );

        // 2: a quantity below the rule fails again, a new error after the Retry.
        let at = format!("{mode}: 2");
        let (caused, _, _) = answered(&mut session, &type_into("order-quantity", "5"), &at);
        assert!(caused.is_empty(), "{at}: typing causes nothing");
        let (caused, _, _) = answered(&mut session, &click("order-submit"), &at);
        assert!(
            read(&caused) == [quantity_error()],
            "{at}: a second error item"
        );

        // 3: a good quantity, Enter on the focused Submit, logs and places the order.
        let at = format!("{mode}: 3");
        answered(&mut session, &click("order-retry"), &at);
        answered(&mut session, &type_into("order-quantity", "12"), &at);
        answered(&mut session, &press("tab", false), &at);
        assert!(
            focused(&session).0 == ["order-submit"],
            "{at}: Tab reaches Submit"
        );
        let (caused, _, line) = answered(&mut session, &press("enter", false), &at);
        assert!(
            read(&caused)
                == [(
                    CausedKind::Log,
                    CausedLevel::Info,
                    Some(Source::App),
                    Some(TARGET.to_string()),
                    ORDER_PLACED.to_string(),
                    tied("order-submit"),
                )],
            "{at}: one info log of the page's own"
        );
        assert!(
            caused[0].fields.is_empty() && caused[0].fields_past == 0,
            "{at}: the log holds no field"
        );
        assert!(
            line.contains(ORDER_PLACED),
            "{at}: the message crosses whole"
        );
        assert!(
            text_of(&session, "order-status").as_deref() == Some("Order placed"),
            "{at}: the order is placed"
        );

        // 4: Export panics, its payload and its line in the page's file.
        let at = format!("{mode}: 4");
        let (caused, _, line) = answered(&mut session, &click("order-export"), &at);
        assert!(
            read(&caused)
                == [(
                    CausedKind::Panic,
                    CausedLevel::Error,
                    None,
                    None,
                    EXPORT_UNBUILT.to_string(),
                    tied("order-export"),
                )],
            "{at}: one panic item holding its payload"
        );
        let file = caused[0]
            .file
            .as_deref()
            .map(|file| file.replace('\\', "/"));
        assert!(
            file.is_some_and(|file| file.ends_with("seven_guis/src/tasks/order.rs"))
                && caused[0].line == Some(EXPORT_PANIC_LINE),
            "{at}: the panic's location is the page's line"
        );
        assert!(line.contains(EXPORT_UNBUILT), "{at}: the payload crosses");

        // 5: the session still acts after the caught panic.
        let at = format!("{mode}: 5");
        let (caused, _, _) = answered(&mut session, &type_into("order-quantity", "15"), &at);
        assert!(caused.is_empty(), "{at}: the step acts and causes nothing");
    }
}

#[test]
fn the_same_caused_crosses_serve_mcp_in_process() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut session = hold(incremental);
        let calls = [click("order-submit"), click("order-export")];
        let members: Vec<String> = calls
            .iter()
            .enumerate()
            .map(|(index, call)| {
                let at = format!("{mode}: {index}");
                let (_, _, line) = answered(&mut session, call, &at);
                caused_member(&line)
                    .unwrap_or_else(|| panic!("{at}: a caused key"))
                    .to_string()
            })
            .collect();

        let replies = serve_lines(
            &[
                tool_call(1, "start", &format!(r#"{{"app":"{LABEL}"}}"#)),
                tool_call(2, "click", r#"{"id":"order-submit"}"#),
                tool_call(3, "click", r#"{"id":"order-export"}"#),
                tool_call(4, "stop", "{}"),
            ],
            &[LABEL],
            |_: &str| Ok(hold(incremental)),
        );
        assert!(replies.len() == 4, "{mode}: four replies");
        for (index, member) in members.iter().enumerate() {
            let at = format!("{mode}: {index}");
            let reply = &replies[index + 1];
            assert!(
                reply.contains(r#""isError":false"#),
                "{at}: the result is no error"
            );
            assert!(
                caused_member(&mcp_result_text(reply)) == Some(member.as_str()),
                "{at}: the tool result carries the same caused"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_of_the_order_steps_replays_holding() {
    use escher_driver::Check;
    use session_common::{record_check, state_dir};

    for incremental in [false, true] {
        let at = mode_name(incremental);
        let calls = [
            click("order-submit"),
            click("order-retry"),
            type_into("order-quantity", "12"),
            click("order-submit"),
            click("order-export"),
        ];
        let dir = state_dir(&format!("order-served-{at}"));
        let bytes = record_check(&dir, || hold(incremental), &calls, at);
        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        let replay = check.replay(&mut hold(incremental));
        assert!(replay.holds(), "{at}: the replay holds");
        assert!(replay.ran() == calls.len(), "{at}: every step ran");
    }
}
