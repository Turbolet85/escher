//! The `canvas` verb across every carrier, on a test-only fixture page of two canvases painted
//! from Rust and a button (`fixtures::canvas_commands`), in both layout modes. Over the session
//! socket from a test host, and as an MCP tool call through `serve_mcp`, each call answers the
//! line `Outcome::to_json` writes for the same call on a fresh in-process session: the small
//! canvas's summary, full dump and part 1, the large canvas's full dump, over the budget, and
//! its parts 1 and 2, and the button, which holds no canvas. An MCP call with no id is one text
//! block marked as an error. A kept check of the small canvas's three reads and a large part is
//! read back, replays green on a fresh session, and stops at its first step naming `digest` once
//! that kept field is flipped. No answer, MCP result or kept check holds the text the small
//! canvas shapes into glyphs or the font's family name: a glyph run crosses as its count, its
//! size, its brush, its origins' box and a digest of its ids. A failure names a layout mode, a
//! carrier and an index only.

use escher_driver::{Call, Session, SessionError};

mod fixtures;
mod session_common;
use session_common::{canvas, canvas_full, canvas_part, mcp_tool_result, serve_lines, tool_call};

/// The label the fixture's sessions are started with.
const LABEL: &str = "canvas";

/// The family the bundled font is named by: no answer may hold it.
const FAMILY: &str = "DejaVu";

/// The calls every carrier answers, with the MCP arguments that spell each.
fn crossing() -> [(Call, &'static str); 6] {
    [
        (canvas("small-canvas"), r#"{"id":"small-canvas"}"#),
        (
            canvas_full("small-canvas"),
            r#"{"id":"small-canvas","full":true}"#,
        ),
        (
            canvas_part("small-canvas", 1),
            r#"{"id":"small-canvas","full":true,"part":1}"#,
        ),
        (
            canvas_full("large-canvas"),
            r#"{"id":"large-canvas","full":true}"#,
        ),
        (
            canvas_part("large-canvas", 2),
            r#"{"id":"large-canvas","full":true,"part":2}"#,
        ),
        (canvas("fx-button"), r#"{"id":"fx-button"}"#),
    ]
}

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// A session on the fixture page.
fn hold(incremental: bool) -> Session {
    fixtures::hold(LABEL, fixtures::canvas_commands, incremental)
}

/// The line a fresh in-process session writes for `call`.
#[track_caller]
fn expected(call: &Call, incremental: bool, at: &str) -> String {
    hold(incremental)
        .run(call)
        .map(|outcome| outcome.to_json())
        .unwrap_or_else(|_| panic!("{at}: the run accepts the call"))
}

#[track_caller]
fn holds_no_text(bytes: &str, at: &str) {
    assert!(
        !bytes.contains(fixtures::CANVAS_TEXT) && !bytes.contains(FAMILY),
        "{at}: no text and no family name"
    );
}

#[test]
fn the_fixture_paints_its_canvases_and_the_large_dump_is_paged() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let lines: Vec<String> = crossing()
            .iter()
            .enumerate()
            .map(|(index, (call, _))| expected(call, incremental, &format!("{mode}: {index}")))
            .collect();
        assert!(
            lines[0].starts_with(r#"{"canvas":true,"#) && lines[0].contains(r#""glyphs":1"#),
            "{mode}: the small canvas reads its glyph run"
        );
        assert!(
            lines[3].contains(r#""part":1,"parts":"#),
            "{mode}: the large dump is read in parts"
        );
        assert!(
            lines[5].starts_with(r#"{"canvas":false,"#),
            "{mode}: the button holds no canvas"
        );
    }
}

#[test]
fn every_mcp_canvas_call_answers_the_runs_line() {
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let calls = crossing();
        let mut requests = vec![tool_call(1, "start", &format!(r#"{{"app":"{LABEL}"}}"#))];
        requests.extend(
            calls
                .iter()
                .enumerate()
                .map(|(index, (_, arguments))| tool_call(index + 2, "canvas", arguments)),
        );
        requests.push(tool_call(calls.len() + 2, "canvas", "{}"));
        requests.push(tool_call(calls.len() + 3, "stop", "{}"));
        let boot = |app: &str| -> Result<Session, SessionError> {
            Ok(fixtures::hold(app, fixtures::canvas_commands, incremental))
        };
        let replies = serve_lines(&requests, &[LABEL], boot);
        assert!(
            replies.len() == requests.len(),
            "{mode}: one reply per call"
        );
        for (index, (call, _)) in calls.iter().enumerate() {
            let at = format!("{mode}: mcp {index}");
            let (is_error, text) = mcp_tool_result(&replies[index + 1]);
            assert!(
                !is_error && text == expected(call, incremental, &at),
                "{at}: the text block is the run's line"
            );
            holds_no_text(&replies[index + 1], &at);
        }
        let (is_error, _) = mcp_tool_result(&replies[calls.len() + 1]);
        assert!(is_error, "{mode}: a canvas with no id is an error");
    }
}

#[test]
fn the_fixtures_answers_hold_no_glyph_id_and_no_text() {
    for incremental in [false, true] {
        let at = format!("{}: in process", mode_name(incremental));
        let dump = expected(&canvas_full("small-canvas"), incremental, &at);
        let text = session_common::mcp_result_text(&dump);
        let glyphs: Vec<&str> = text
            .lines()
            .map(str::trim_start)
            .filter(|line| line.starts_with("glyphs "))
            .collect();
        assert!(glyphs.len() == 1, "{at}: one glyph run");
        let pairs: Vec<(&str, &str)> = glyphs[0]
            .split(' ')
            .skip(1)
            .filter_map(|pair| pair.split_once('='))
            .collect();
        let keys: Vec<&str> = pairs.iter().map(|(key, _)| *key).collect();
        assert!(
            keys == ["font", "size", "brush", "t", "n", "at", "ids"],
            "{at}: the glyph line holds its keys alone"
        );
        let value = |key: &str| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map_or("", |(_, value)| *value)
        };
        assert!(value("n") == "3", "{at}: the run counts its three glyphs");
        let ids = value("ids");
        assert!(
            ids.len() == 16
                && ids
                    .chars()
                    .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)),
            "{at}: the ids are one digest of 16 lowercase hex"
        );
        for (index, (call, _)) in crossing().iter().enumerate() {
            holds_no_text(&expected(call, incremental, &at), &format!("{at} {index}"));
        }
    }
}

#[cfg(unix)]
const HOST_DIRS: [&str; 2] = ["scc-full", "scc-incr"];

#[cfg(unix)]
#[test]
fn every_socket_canvas_call_answers_the_runs_line() {
    use escher_driver::{call, start, stop};
    use session_common::{Host, READY, clear, host_command, state_dir};

    let children = ["canvas_host_full", "canvas_host_incremental"];
    for (row, incremental) in [false, true].into_iter().enumerate() {
        let mode = mode_name(incremental);
        let dir = state_dir(HOST_DIRS[row]);
        clear(&dir);
        let started = start(&dir, host_command(children[row]), READY)
            .unwrap_or_else(|_| panic!("{mode}: the host starts"));
        let mut host = Host::new(started.child);
        for (index, (canvas_call, _)) in crossing().iter().enumerate() {
            let at = format!("{mode}: socket {index}");
            let answer =
                call(&dir, canvas_call).unwrap_or_else(|_| panic!("{at}: the socket answers"));
            assert!(answer.accepted, "{at}: accepted");
            assert!(
                answer.json == expected(canvas_call, incremental, &at),
                "{at}: the socket carries the run's line"
            );
            holds_no_text(&answer.json, &at);
        }
        assert!(stop(&dir).is_ok(), "{mode}: the host stops");
        host.kill();
        clear(&dir);
    }
}

#[cfg(unix)]
fn serve_host(row: usize, incremental: bool) {
    use session_common::state_dir;
    escher_driver::serve(
        &state_dir(HOST_DIRS[row]),
        hold(incremental),
        std::time::Duration::from_secs(120),
    )
    .expect("the host serves until it is stopped");
}

#[cfg(unix)]
#[test]
#[ignore = "spawned as the session host by every_socket_canvas_call_answers_the_runs_line"]
fn canvas_host_full() {
    serve_host(0, false);
}

#[cfg(unix)]
#[test]
#[ignore = "spawned as the session host by every_socket_canvas_call_answers_the_runs_line"]
fn canvas_host_incremental() {
    serve_host(1, true);
}

#[cfg(unix)]
#[test]
fn a_kept_canvas_check_replays_green_and_a_flipped_digest_stops_it() {
    use escher_driver::Check;
    use session_common::{record_check, state_dir};

    let calls = [
        canvas("small-canvas"),
        canvas_full("small-canvas"),
        canvas_part("small-canvas", 1),
        canvas_part("large-canvas", 2),
    ];
    for incremental in [false, true] {
        let at = format!("{}: kept check", mode_name(incremental));
        let dir = state_dir(&format!("scc-check-{}", mode_name(incremental)));
        let bytes = record_check(&dir, || hold(incremental), &calls, &at);
        let text = std::str::from_utf8(&bytes).expect("a check is text");
        holds_no_text(text, &at);
        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        assert!(
            check.steps().len() == calls.len(),
            "{at}: every step is kept"
        );
        let replay = check.replay(&mut hold(incremental));
        assert!(
            replay.holds() && replay.ran() == calls.len(),
            "{at}: the replay holds"
        );

        // The summary's own digest, kept in its answer after `depth`.
        let key = r#","digest":""#;
        let depth = text.find(r#""depth":"#).expect("the summary is kept");
        let at_digest = depth + text[depth..].find(key).expect("its digest is kept") + key.len();
        let mut flipped = text.to_string();
        let first = &text[at_digest..=at_digest];
        let other = if first == "0" { "1" } else { "0" };
        flipped.replace_range(at_digest..=at_digest, other);
        let check = Check::from_bytes(flipped.as_bytes())
            .unwrap_or_else(|_| panic!("{at}: the flipped check reads"));
        let replay = check.replay(&mut hold(incremental));
        let change = replay
            .change()
            .unwrap_or_else(|| panic!("{at}: a step changed"));
        assert!(
            !replay.holds() && replay.ran() == 1,
            "{at}: the replay stops at its first step"
        );
        assert!(
            change.step() == 0 && change.keys() == ["digest"],
            "{at}: the change names digest"
        );
    }
}
