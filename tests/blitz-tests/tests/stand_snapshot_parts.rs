//! A screen whose snapshot text passes `SNAPSHOT_TEXT_BUDGET` is read through the driver in
//! parts an agent can walk. On a fixture of 400 keyed buttons and a password input, booted with
//! the stand's options, in both layout modes: `snapshot` with no argument answers part 1 of at
//! least two; `snapshot` with `part` 1 to the last answers each part within the budget, the
//! parts in order are the whole text, and every node's line is on exactly one part, equal to its
//! line in the whole text; a part past the last answers the empty text with the same count and
//! changes nothing; an id read only in the last part is scrolled to and clicked, and a fresh
//! walk then reads exactly one line `focused`; a sentinel typed into the password input is on no
//! part, its line reading the mask. Over MCP, run in process on the same fixture, the same walk
//! — `snapshot` with no argument, each part and the part past the last — answers the JSON lines
//! `Session::run` writes for the same calls. On the
//! stand's Counter, inside the budget, `snapshot` answers the whole screen as before, and part 1
//! is that text as the one part. A failure message carries a layout mode and an index only.

use std::time::Duration;

use dioxus_native_dom::{MASKED_VALUE, SNAPSHOT_TEXT_BUDGET};
use escher_driver::{Outcome, Session, SessionError, serve_mcp};
use seven_guis::stand::LeanTask;

mod fixtures;
mod session_common;
use fixtures::many_items;
use session_common::{act, click, hold, part, scroll, snapshot, snapshot_part, type_into};

/// The text typed into the password input: synthetic, and no part of any id or name.
const SENTINEL: &str = "synthetic-pt-7Qv";

/// The label the fixture's session is held under.
const LABEL: &str = "parts";

/// Reads every part of the screen `session` holds, from part 1 to the last, as the driver
/// answers them: each part's text, and the count the first answer gives.
fn walk(session: &mut Session, mode: &str) -> (Vec<String>, u32) {
    let (first, at, parts) = part(session.run(&snapshot_part(1)).expect("part 1 is accepted"));
    assert!(at == 1 && parts >= 1, "{mode}: part 1 names itself");
    let mut texts = vec![first];
    for n in 2..=parts {
        let (text, at, count) = part(
            session
                .run(&snapshot_part(i64::from(n)))
                .expect("a part is accepted"),
        );
        assert!(at == n && count == parts, "{mode}: part {n} names itself");
        texts.push(text);
    }
    (texts, parts)
}

/// How many of `texts`' lines hold `needle`.
fn lines_holding(texts: &[String], needle: &str) -> usize {
    texts
        .iter()
        .flat_map(|text| text.lines())
        .filter(|line| line.contains(needle))
        .count()
}

#[test]
fn a_screen_over_the_budget_is_walked_in_parts() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let mut session = fixtures::hold(LABEL, many_items, incremental);

        // The condition first: the screen is over the budget, laid out with a real font.
        let doc = &session.harness().doc;
        assert!(
            doc.unkeyed_actionable().is_empty(),
            "{mode}: every control is keyed"
        );
        let screen = doc.snapshot();
        let whole = screen.to_text();
        assert!(
            whole.len() > SNAPSHOT_TEXT_BUDGET,
            "{mode}: over the budget"
        );
        let first = screen.get("item-001").expect("the first item has a node");
        assert!(
            first.bounds.width > 0.0 && first.bounds.height > 0.0,
            "{mode}: the first item is laid out"
        );
        let cut = screen.text_parts(SNAPSHOT_TEXT_BUDGET);

        // Without `part`, the screen answers its first part.
        let (text, at, parts) = part(session.run(&snapshot()).expect("snapshot is accepted"));
        assert!(at == 1 && parts >= 2, "{mode}: part 1 of several");
        assert!(text == cut[0].text, "{mode}: part 1 is the cut's first");
        assert!(
            usize::try_from(parts).ok() == Some(cut.len()),
            "{mode}: the count"
        );

        // The walk: each part within the budget, the parts the whole text.
        let (texts, walked) = walk(&mut session, mode);
        assert!(walked == parts, "{mode}: the walk's count");
        for (index, text) in texts.iter().enumerate() {
            assert!(
                text.len() <= SNAPSHOT_TEXT_BUDGET,
                "{mode}: part {index} within the budget"
            );
        }
        assert!(
            texts.concat() == whole,
            "{mode}: the parts are the whole text"
        );
        let lines: Vec<&str> = texts.iter().flat_map(|text| text.lines()).collect();
        assert!(
            lines == whole.lines().collect::<Vec<_>>(),
            "{mode}: each line as it reads whole"
        );
        for (index, node) in screen.nodes().enumerate() {
            let id = format!(" id={:?} ", node.id);
            assert!(
                lines_holding(&texts, &id) == 1,
                "{mode}: node {index} on one line"
            );
        }

        // Past the last part: the empty text, the same count, nothing changed.
        let (text, at, count) = part(
            session
                .run(&snapshot_part(i64::from(parts) + 1))
                .expect("a part past the last is accepted"),
        );
        assert!(
            text.is_empty() && at == parts + 1 && count == parts,
            "{mode}: the empty part"
        );
        assert!(
            session.harness().doc.snapshot().to_text() == whole,
            "{mode}: the screen is unchanged"
        );

        // An id read only in the last part is addressable.
        let last = cut.last().expect("there is a last part");
        assert!(
            last.ids.iter().any(|id| id == "item-400")
                && !cut[0].ids.iter().any(|id| id == "item-400"),
            "{mode}: the last item is read only late"
        );
        let scrolled = act(&mut session, &scroll("item-400")).expect("the scroll is accepted");
        assert!(
            scrolled.in_view == Some(true),
            "{mode}: the item is in view"
        );
        let clicked = act(&mut session, &click("item-400")).expect("the click is accepted");
        assert!(
            clicked.changed().contains(&"clicks"),
            "{mode}: the click is counted"
        );
        let (texts, _) = walk(&mut session, mode);
        assert!(
            lines_holding(&texts, " focused @") == 1,
            "{mode}: one line reads focused"
        );

        // A typed password is on no part, its line the mask.
        assert!(
            act(&mut session, &scroll("secret")).is_some(),
            "{mode}: the input is scrolled to"
        );
        assert!(
            act(&mut session, &type_into("secret", SENTINEL)).is_some(),
            "{mode}: the text is typed"
        );
        let (texts, _) = walk(&mut session, mode);
        assert!(
            lines_holding(&texts, SENTINEL) == 0,
            "{mode}: the sentinel is on no part"
        );
        let secret: Vec<&str> = texts
            .iter()
            .flat_map(|text| text.lines())
            .filter(|line| line.contains(" id=\"secret\" "))
            .collect();
        assert!(
            secret.len() == 1 && secret[0].contains(&format!(" value={MASKED_VALUE:?} ")),
            "{mode}: the input reads the mask"
        );
    }
}

fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
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
    (reply.ends_with(r#""isError":true}}"#), text)
}

#[test]
fn over_mcp_the_walk_answers_the_json_lines_of_the_same_calls() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };

        // The walk run directly: `snapshot`, then each part, then the part past the last.
        let mut direct = fixtures::hold(LABEL, many_items, incremental);
        let first = direct.run(&snapshot()).expect("snapshot is accepted");
        let Outcome::Part { parts, .. } = first else {
            panic!("{mode}: the screen is read in parts");
        };
        let mut calls = vec![(snapshot(), "{}".to_string())];
        for n in 1..=i64::from(parts) + 1 {
            calls.push((snapshot_part(n), format!(r#"{{"part":{n}}}"#)));
        }
        let expected: Vec<String> = calls
            .iter()
            .map(|(call, _)| {
                let outcome = direct.run(call).expect("the call is accepted");
                assert!(
                    matches!(outcome, Outcome::Part { .. }),
                    "{mode}: each call reads a part"
                );
                outcome.to_json()
            })
            .collect();

        // The same walk as MCP tool calls, on a session the server boots the same way.
        let mut requests = vec![tool_call(1, "start", &format!(r#"{{"app":"{LABEL}"}}"#))];
        for (index, (_, arguments)) in calls.iter().enumerate() {
            requests.push(tool_call(index + 2, "snapshot", arguments));
        }
        requests.push(tool_call(calls.len() + 2, "stop", "{}"));
        let mut input = requests.join("\n");
        input.push('\n');
        let mut output = Vec::new();
        let boot = |app: &str| -> Result<Session, SessionError> {
            Session::start(app, || fixtures::boot(many_items, incremental))
        };
        serve_mcp(
            input.as_bytes(),
            &mut output,
            &[LABEL],
            boot,
            Duration::from_secs(1800),
        )
        .expect("the exchange ends");
        let output = String::from_utf8(output).expect("replies are text");
        let replies: Vec<&str> = output.lines().collect();
        assert!(
            replies.len() == requests.len(),
            "{mode}: one reply per call"
        );
        for (index, line) in expected.iter().enumerate() {
            let (is_error, text) = result(replies[index + 1]);
            assert!(
                !is_error && text == *line,
                "{mode}: call {index} answers its JSON line"
            );
        }
    }
}

#[test]
fn a_screen_inside_the_budget_reads_whole_as_before() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let (mut session, _ticks) = hold(LeanTask::Counter, incremental);
        let whole = session.harness().doc.snapshot().to_text();
        assert!(
            whole.len() <= SNAPSHOT_TEXT_BUDGET,
            "{mode}: inside the budget"
        );
        let read = session.run(&snapshot()).expect("snapshot is accepted");
        assert!(
            read == Outcome::Screen {
                text: whole.clone()
            },
            "{mode}: the whole screen, no part"
        );
        assert!(
            read.to_json().starts_with("{\"text\":") && !read.to_json().contains("\"part\""),
            "{mode}: the line holds the text alone"
        );
        let (text, at, parts) = part(session.run(&snapshot_part(1)).expect("part 1 is accepted"));
        assert!(
            text == whole && at == 1 && parts == 1,
            "{mode}: part 1 of 1 is the whole text"
        );
    }
}
