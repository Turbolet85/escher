//! The mask holds on the answers 0.3.0 added. A fixture page beside the stand holds a password
//! input, typed a synthetic sentinel through the driver, and a file input holding a chosen
//! path, booted with the stand's options in both layout modes. The `graph`, `actions` and
//! `layout` answers, written as the driver writes them, hold neither the sentinel nor the path,
//! while the snapshot reads both values as the mask. A session recorded through `type`,
//! `snapshot`, `graph`, `actions` and `layout` steps keeps the `type` as the stand-in and holds
//! neither in its check, which replays holding on a fresh session; every path an exploration
//! keeps holds neither. Over MCP, in process, the snapshot reads the file input's value as the
//! mask and the path nowhere. A `caused` item holds what its author wrote and stays outside the
//! mask. A failure message names a layout mode and an index only.

#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::path::Path;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_native_dom::MASKED_VALUE;
#[cfg(unix)]
use escher_driver::{ArgValue, Check, MASKED_STAND_IN, StepBudget, explore};
use escher_driver::{Call, Outcome, Session, SessionError, serve_mcp};

mod fixtures;
mod session_common;
#[cfg(unix)]
use session_common::state_dir;
use session_common::{actions, graph, layout, mcp_result_text, snapshot, type_into};

/// What the check types into the password input: synthetic, and no part of any id or name.
const SENTINEL: &str = "Masks-Typed-Sentinel-7qZr";

/// The path the file input holds: synthetic.
const UPLOAD_PATH: &str = "/synthetic/dir/masked-upload.txt";

/// The file name of [`UPLOAD_PATH`].
const UPLOAD_NAME: &str = "masked-upload.txt";

/// The label the fixture is held under.
const APP: &str = "masks";

const SECRET: &str = "fx-secret";
const UPLOAD: &str = "fx-upload";

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// A password input and a file input holding [`UPLOAD_PATH`], each labelled.
fn masked_inputs() -> Element {
    rsx! {
        div { id: "fx-root",
            label { r#for: SECRET, "Passphrase" }
            input { id: SECRET, r#type: "password" }
            label { r#for: UPLOAD, "Attachment" }
            input { id: UPLOAD, r#type: "file", value: UPLOAD_PATH }
        }
    }
}

/// The fixture held in a session, the sentinel typed into its password input.
#[track_caller]
fn hold_typed(incremental: bool) -> Session {
    let mut session = fixtures::hold(APP, masked_inputs, incremental);
    let typed = session.run(&type_into(SECRET, SENTINEL));
    assert!(
        typed.is_ok(),
        "{}: the sentinel is typed",
        mode_name(incremental)
    );
    session
}

/// The line the driver writes for `call`'s accepted answer.
#[track_caller]
fn answer(session: &mut Session, call: &Call, at: &str, index: usize) -> String {
    match session.run(call) {
        Ok(outcome) => outcome.to_json(),
        Err(_) => panic!("{at}: call {index} is accepted"),
    }
}

/// Whether `text` holds neither the sentinel nor the file input's path or file name.
fn holds_no_value(text: &str) -> bool {
    !text.contains(SENTINEL) && !text.contains(UPLOAD_PATH) && !text.contains(UPLOAD_NAME)
}

/// The value field of the screen text's line for `id`.
#[track_caller]
fn reads_the_mask(screen: &str, id: &str, at: &str) -> bool {
    let Some(line) = screen
        .lines()
        .find(|line| line.contains(&format!(" id=\"{id}\" ")))
    else {
        panic!("{at}: the screen lists the input");
    };
    line.contains(&format!(" value={MASKED_VALUE:?} "))
}

#[test]
fn the_graph_actions_and_layout_answers_hold_no_masked_value() {
    assert!(!SENTINEL.contains(MASKED_VALUE) && !MASKED_VALUE.is_empty());
    for incremental in [false, true] {
        let at = mode_name(incremental);
        let mut session = hold_typed(incremental);
        let Ok(Outcome::Screen { text }) = session.run(&snapshot()) else {
            panic!("{at}: a snapshot answers the screen");
        };
        assert!(
            reads_the_mask(&text, SECRET, at) && reads_the_mask(&text, UPLOAD, at),
            "{at}: the snapshot reads both inputs' values as the mask"
        );
        assert!(holds_no_value(&text), "{at}: the snapshot holds no value");

        let calls = [graph(), actions(), layout(SECRET), layout(UPLOAD)];
        for (index, call) in calls.iter().enumerate() {
            let line = answer(&mut session, call, at, index);
            assert!(holds_no_value(&line), "{at}: answer {index} holds no value");
        }
    }
}

/// The steps the recorded session runs after its `type`.
#[cfg(unix)]
fn reads() -> [Call; 5] {
    [
        snapshot(),
        graph(),
        actions(),
        layout(SECRET),
        layout(UPLOAD),
    ]
}

/// A fresh directory under the target's test directory, owner-only once a recording makes it.
#[cfg(unix)]
fn record_dir(name: &str) -> std::path::PathBuf {
    let dir = state_dir(name);
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The bytes of every file in `dir`.
#[cfg(unix)]
fn kept_bytes(dir: &Path) -> Vec<Vec<u8>> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| fs::read(entry.path()).ok())
        .collect()
}

#[cfg(unix)]
#[test]
fn a_kept_check_holds_no_masked_value_and_replays() {
    for incremental in [false, true] {
        let at = mode_name(incremental);
        let dir = record_dir(&format!("masks-kept-{at}"));
        let mut session = fixtures::hold(APP, masked_inputs, incremental)
            .recording_into(&dir)
            .unwrap_or_else(|_| panic!("{at}: the session records"));
        let typed = session.run(&type_into(SECRET, SENTINEL));
        assert!(typed.is_ok(), "{at}: the sentinel is typed");
        for (index, call) in reads().iter().enumerate() {
            answer(&mut session, call, at, index);
        }
        drop(session);

        let kept = kept_bytes(&dir);
        assert!(kept.len() == 1, "{at}: one check is kept");
        let text = String::from_utf8_lossy(&kept[0]);
        assert!(
            text.contains(MASKED_VALUE),
            "{at}: the check reads the mask"
        );
        assert!(holds_no_value(&text), "{at}: the check holds no value");

        let Ok(check) = Check::from_bytes(&kept[0]) else {
            panic!("{at}: the check reads back");
        };
        assert!(check.steps().len() == 6, "{at}: the check keeps every call");
        let stand_in = ArgValue::Text(MASKED_STAND_IN.to_string());
        assert!(
            check.steps()[0]
                .call()
                .args
                .iter()
                .any(|(name, value)| name == "text" && *value == stand_in),
            "{at}: the kept type holds the stand-in"
        );
        let mut fresh = fixtures::hold(APP, masked_inputs, incremental);
        assert!(check.replay(&mut fresh).holds(), "{at}: the check replays");
        let _ = fs::remove_dir_all(&dir);
    }
}

#[cfg(unix)]
#[test]
fn every_explored_paths_check_holds_no_masked_value() {
    for incremental in [false, true] {
        let at = mode_name(incremental);
        let dir = record_dir(&format!("masks-explore-{at}"));
        let Some(budget) = StepBudget::new(64) else {
            panic!("{at}: a budget within its bounds");
        };
        let exploration = explore(|| Ok(hold_typed(incremental)), budget, Some(&dir))
            .unwrap_or_else(|_| panic!("{at}: explores"));
        let kept = kept_bytes(&dir);
        assert!(
            !kept.is_empty() && kept.len() == exploration.paths.len(),
            "{at}: one check per path"
        );
        for (index, bytes) in kept.iter().enumerate() {
            assert!(
                holds_no_value(&String::from_utf8_lossy(bytes)),
                "{at}: check {index} holds no value"
            );
        }
        let _ = fs::remove_dir_all(&dir);
    }
}

fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {
    move |app| Session::start(app, || fixtures::boot(masked_inputs, incremental))
}

#[test]
fn over_mcp_the_file_inputs_value_reads_the_mask() {
    for incremental in [false, true] {
        let at = mode_name(incremental);
        let requests = [
            tool_call(1, "start", &format!(r#"{{"app":"{APP}"}}"#)),
            tool_call(
                2,
                "type",
                &format!(r#"{{"id":"{SECRET}","text":"{SENTINEL}"}}"#),
            ),
            tool_call(3, "snapshot", "{}"),
        ];
        let mut input = requests.join("\n");
        input.push('\n');
        let mut output = Vec::new();
        let served = serve_mcp(
            input.as_bytes(),
            &mut output,
            &[APP],
            boot(incremental),
            Duration::from_secs(1800),
        );
        assert!(served.is_ok(), "{at}: the exchange ends");
        let output = String::from_utf8_lossy(&output);
        let replies: Vec<&str> = output.lines().collect();
        assert!(replies.len() == 3, "{at}: one reply per call");
        assert!(holds_no_value(&output), "{at}: the replies hold no value");
        let screen = mcp_result_text(replies[2]);
        assert!(
            reads_the_mask(&screen, UPLOAD, at) && reads_the_mask(&screen, SECRET, at),
            "{at}: the snapshot reads both inputs' values as the mask"
        );
    }
}
