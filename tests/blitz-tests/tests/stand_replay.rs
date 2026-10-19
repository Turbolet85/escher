//! Kept checks replay as ordinary tests, red when the app's behaviour changes and green
//! otherwise. In both layout modes. The two Counter sessions committed under
//! `tests/blitz-tests/checks/` — one recorded through the command line, one through the `mcp`
//! role, each as the binary wrote it — are read through `Check::from_bytes` and replayed by
//! `Check::replay`, every step's call run again through `Session::run`. *On the stand's Counter,
//! freshly booted,* each holds with every step run, and neither holds a `png` key or the first
//! 64 base64 characters of the screen's shot. *On a changed Counter* — a variant in this file
//! whose click adds 2, mounted in the stand's own chrome — neither holds: its first change is
//! the check's first activation step (a `click` on the Counter's button, or `enter` or `space`
//! pressed on it), accepted then and now, with `changed` among the keys that differ, and every
//! step before it held. *On a session of another app* a check runs nothing, leaves the screen as
//! it was and does not hold. A failure message names the layout mode, the check's file and a
//! step index, never an id, a name or a reading.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use escher_driver::{ArgValue, Check, Session, Step};
use seven_guis::app::TaskShell;
use seven_guis::stand::{self, LeanTask};

mod session_common;
use session_common::{hold, shot_prefix, slug};

/// The committed checks: each file's name, its bytes and the index of its first activation step.
const CHECKS: [(&str, &[u8], usize); 2] = [
    (
        "counter-cli.json",
        include_bytes!("../checks/counter-cli.json"),
        1,
    ),
    (
        "counter-mcp.json",
        include_bytes!("../checks/counter-mcp.json"),
        3,
    ),
];

/// The Counter's button, which an activation step acts on.
const BUTTON: &str = "counter-increment";

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// Each committed check, read and checked to name the Counter, with its file's text.
fn checks() -> Vec<(&'static str, &'static str, Check, usize)> {
    CHECKS
        .iter()
        .map(|(file, bytes, first)| {
            let check = Check::from_bytes(bytes).expect("a committed check reads");
            assert!(
                check.app() == slug(LeanTask::Counter),
                "{file}: names the Counter"
            );
            let text = std::str::from_utf8(bytes).expect("a check is text");
            (*file, text, check, *first)
        })
        .collect()
}

fn text_arg<'a>(step: &'a Step, name: &str) -> Option<&'a str> {
    step.call()
        .args
        .iter()
        .find_map(|(arg, value)| match value {
            ArgValue::Text(text) if arg == name => Some(text.as_str()),
            _ => None,
        })
}

/// Whether `step` presses the Counter's button: a `click` on it, or `enter` or `space` pressed
/// while it holds focus, which a check's scripts give it by Tab before any such press.
fn activates(step: &Step) -> bool {
    match step.call().verb.as_str() {
        "click" => text_arg(step, "id") == Some(BUTTON),
        "press" => matches!(text_arg(step, "key"), Some("enter" | "space")),
        _ => false,
    }
}

#[test]
fn each_committed_check_holds_on_the_unchanged_stand() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (file, text, check, first) in checks() {
            let first_activation = check.steps().iter().position(activates);
            assert!(
                first_activation == Some(first),
                "{mode} {file}: the first activation step is step {first}"
            );
            let (mut session, _ticks) = hold(LeanTask::Counter, incremental);
            let replay = check.replay(&mut session);
            let changed_at = replay.change().map(|change| change.step());
            assert!(
                replay.holds(),
                "{mode} {file}: the check holds, its first change at step {changed_at:?}"
            );
            assert!(
                replay.ran() == check.steps().len(),
                "{mode} {file}: every step ran"
            );

            let prefix = shot_prefix(&mut session);
            assert!(
                !text.contains(r#""png""#) && !text.contains(&prefix),
                "{mode} {file}: the check holds a shot's image"
            );
        }
    }
}

/// The stand's Counter with one change: a click adds 2. Named `Counter`, as the stand's is, so
/// every element reads the stand's own id.
mod variant {
    use dioxus::prelude::*;

    #[component]
    pub fn Counter() -> Element {
        let mut count = use_signal(|| 0i32);

        rsx! {
            div { class: "counter-root",
                style { {CSS} }
                div { class: "counter-card",
                    p { id: "counter-value", class: "counter-display", "{count}" }
                    button {
                        id: "counter-increment",
                        class: "counter-btn",
                        onclick: move |_| { count += 2 },
                        "Count"
                    }
                }
            }
        }
    }

    const CSS: &str = r#"
.counter-root {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    background-color: #f5f5f5;
    font-family: sans-serif;
}

.counter-card {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 16px;
    background: #ffffff;
    border: 1px solid #d0d0d0;
    border-radius: 8px;
    padding: 24px 32px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.counter-display {
    font-size: 24px;
    font-weight: 600;
    color: #1a1a1a;
    min-width: 60px;
    text-align: center;
    margin: 0;
}

.counter-btn {
    font-size: 16px;
    font-weight: 500;
    color: #ffffff;
    background-color: #3a5ce5;
    border: none;
    border-radius: 6px;
    padding: 10px 24px;
    cursor: pointer;
}

.counter-btn:hover {
    background-color: #2a4cd3;
}

.counter-btn:active {
    background-color: #1f3fc1;
}
"#;
}

fn variant_root() -> Element {
    rsx! {
        TaskShell { title: "Counter", on_back: |_| {},
            variant::Counter {}
        }
    }
}

/// A session on the variant Counter, started as the Counter's.
fn hold_variant(incremental: bool) -> Session {
    Session::start(slug(LeanTask::Counter), || {
        Harness::from_vdom(VirtualDom::new(variant_root), stand::options(incremental))
    })
    .expect("the Counter's slug is a label")
}

#[test]
fn each_committed_check_turns_red_at_its_first_activation_on_a_changed_counter() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (file, _, check, first) in checks() {
            let replay = check.replay(&mut hold_variant(incremental));
            assert!(!replay.holds(), "{mode} {file}: the check does not hold");
            assert!(
                !replay.another_app(),
                "{mode} {file}: the variant runs as the Counter"
            );
            let change = replay.change().expect("a step changed");
            assert!(
                change.step() == first,
                "{mode} {file}: the first change is at step {}, not step {first}",
                change.step()
            );
            assert!(
                replay.ran() == first + 1,
                "{mode} {file}: every step before step {first} held"
            );
            assert!(
                change.accepted_then() && change.accepted_now(),
                "{mode} {file}: step {first} is accepted then and now"
            );
            assert!(
                change.keys().contains(&"changed"),
                "{mode} {file}: step {first} differs in its changed nodes"
            );
        }
    }
}

#[test]
fn a_check_on_a_session_of_another_app_runs_nothing() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (file, _, check, _) in checks() {
            let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
            let before = session.harness().doc.snapshot();
            let replay = check.replay(&mut session);
            assert!(
                !replay.holds() && replay.another_app(),
                "{mode} {file}: the check does not hold on another app"
            );
            assert!(
                replay.ran() == 0 && replay.change().is_none(),
                "{mode} {file}: no step ran"
            );
            assert!(
                session.harness().doc.snapshot() == before,
                "{mode} {file}: the screen is as it was"
            );
        }
    }
}
