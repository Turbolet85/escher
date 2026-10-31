//! Cells, served by the stand's own boot and held as a driver session, is driven through
//! `Session::run` alone, in both layout modes. Its screen passes `SNAPSHOT_TEXT_BUDGET` and reads
//! in parts: a `snapshot` answers part 1 of the measured 5, every part fits the budget, the parts
//! read in order are the whole screen's text and a part past the last is empty. Every element
//! an agent can act on reads an author key. A value and a formula committed through the cell
//! editor read their values in the screen's first part. A cell past the viewport is scrolled
//! into view; a click at its centre right after the scroll lands on the scrolling root's overlay
//! scrollbar, which takes it as a thumb drag and opens nothing, while once an `advance` has let
//! the scrollbar fade the same click opens the editor, and a formula committed there reads its
//! value in the last part. By keyboard alone: `press` tabs to the grid, moves its current cell and
//! opens the editor, `type` supplies the characters `press` has no key for, and `press` commits a
//! value read in the first part with focus back on the grid. The lint at boot reads the 490
//! findings measured on a grid larger than its viewport. A screen's text is content: this file
//! prints none, and a failure message carries the layout mode and a step or part index only.

use dioxus_native_dom::SNAPSHOT_TEXT_BUDGET;
use escher_driver::{Call, Outcome, Session};
use seven_guis::stand;

mod session_common;
use session_common::{
    act, advance, click, focused, lint, linted, part, press, scroll, snapshot, snapshot_part,
    type_into,
};

/// The parts Cells' screen reads in at boot, measured.
const PARTS: u32 = 5;

/// The lint's findings on Cells at boot, measured: a grid wider and taller than the viewport
/// inside a clipping root.
const FINDINGS_AT_BOOT: usize = 490;

/// Time enough for the overlay scrollbars a scroll shows to fade, in milliseconds.
const FADE_MS: u32 = 5000;

/// A session holding Cells booted through the stand in one layout mode.
fn hold_cells(incremental: bool) -> Session {
    Session::start("cells", || stand::boot_cells(stand::options(incremental)))
        .expect("the label is one of the closed set")
}

/// Every part of the screen, read through the driver in order.
#[track_caller]
fn parts(session: &mut Session, mode: &str) -> Vec<String> {
    let (first, _, count) = part(session.run(&snapshot()).expect("a snapshot is answered"));
    let mut read = vec![first];
    for n in 2..=count {
        let (text, at, of) = part(
            session
                .run(&snapshot_part(i64::from(n)))
                .unwrap_or_else(|_| panic!("{mode}: part {n} is answered")),
        );
        assert!(at == n && of == count, "{mode}: part {n} reads its number");
        read.push(text);
    }
    read
}

/// Whether the line of `id` in `text` reads `name` as the node's name.
fn reads(text: &str, id: &str, name: &str) -> bool {
    text.lines()
        .any(|line| line.contains(&format!("\"{name}\" id=\"{id}\"")))
}

/// Runs `call` through the driver as step `step` and asserts it acts and returns settled.
#[track_caller]
fn step(session: &mut Session, call: &Call, mode: &str, step: usize) -> session_common::Acted {
    let acted = act(session, call).unwrap_or_else(|| panic!("{mode}: step {step} acts"));
    assert!(
        acted.settled && acted.busy.is_none(),
        "{mode}: step {step} returns settled"
    );
    acted
}

/// Commits `text` into the cell `id` names: a click opens its editor, a `type` fills it and
/// Enter commits it.
#[track_caller]
fn commit(session: &mut Session, id: &str, text: &str, mode: &str, first: usize) {
    let opened = step(session, &click(id), mode, first);
    assert!(
        opened.added() == ["cell-input"],
        "{mode}: step {first} opens the cell's editor"
    );
    step(session, &type_into("cell-input", text), mode, first + 1);
    let committed = step(session, &press("enter", false), mode, first + 2);
    assert!(
        committed.removed() == ["cell-input"] && committed.changed() == [id],
        "{mode}: step {} commits the cell",
        first + 2
    );
}

#[test]
fn cells_is_walked_in_parts_through_the_driver() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_cells(incremental);
        let whole = session.harness().doc.snapshot().to_text();
        assert!(
            whole.len() > SNAPSHOT_TEXT_BUDGET,
            "{mode}: the screen passes the budget"
        );

        let (_, first, count) = part(session.run(&snapshot()).expect("a snapshot is answered"));
        assert!(
            first == 1 && count == PARTS,
            "{mode}: a snapshot answers part 1 of the measured parts"
        );
        let read = parts(&mut session, &mode);
        for (index, text) in read.iter().enumerate() {
            assert!(
                !text.is_empty() && text.len() <= SNAPSHOT_TEXT_BUDGET,
                "{mode}: part {} fits the budget",
                index + 1
            );
        }
        assert!(
            read.concat() == whole,
            "{mode}: the parts read in order are the whole screen"
        );
        let (past, at, of) = part(
            session
                .run(&snapshot_part(i64::from(PARTS) + 1))
                .expect("a part past the last is answered"),
        );
        assert!(
            past.is_empty() && at == PARTS + 1 && of == PARTS,
            "{mode}: a part past the last is empty and names the count"
        );
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element reads an author key"
        );
    }
}

#[test]
fn committed_cells_read_their_values_and_a_cell_past_the_viewport_reads_in_the_last_part() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_cells(incremental);

        commit(&mut session, "cell-A0", "5", &mode, 0);
        commit(&mut session, "cell-B1", "=A0*2", &mode, 3);
        let read = parts(&mut session, &mode);
        assert!(
            reads(&read[0], "cell-A0", "5") && reads(&read[0], "cell-B1", "10"),
            "{mode}: the first part reads the value and the formula's value"
        );

        let scrolled = step(&mut session, &scroll("cell-Z25"), &mode, 6);
        assert!(
            scrolled.in_view == Some(true),
            "{mode}: step 6 brings the cell into view"
        );
        let faded = step(&mut session, &advance(i64::from(FADE_MS)), &mode, 7);
        assert!(
            faded.advanced_ms == Some(FADE_MS),
            "{mode}: step 7 moves the session's time"
        );
        commit(&mut session, "cell-Z25", "=B1+1", &mode, 8);

        let read = parts(&mut session, &mode);
        let last = read.len() - 1;
        assert!(
            read.len() == PARTS as usize && reads(&read[last], "cell-Z25", "11"),
            "{mode}: the last part reads the formula's value"
        );
        assert!(
            read[..last]
                .iter()
                .all(|text| !text.contains("id=\"cell-Z25\"")),
            "{mode}: no earlier part reads the cell"
        );
    }
}

#[test]
fn the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part() {
    const WALK: [&str; 4] = ["tab", "tab", "arrow-right", "arrow-down"];
    assert_eq!(WALK.len(), 4);
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_cells(incremental);
        for (index, key) in WALK.into_iter().enumerate() {
            step(&mut session, &press(key, false), &mode, index);
        }
        assert!(
            focused(&session).0 == ["cells-grid"],
            "{mode}: the walk leaves focus on the grid"
        );
        let opened = step(&mut session, &press("enter", false), &mode, 4);
        assert!(
            opened.added() == ["cell-input"] && focused(&session).0 == ["cell-input"],
            "{mode}: Enter opens the editor, focused"
        );
        step(&mut session, &type_into("cell-input", "42"), &mode, 5);
        let committed = step(&mut session, &press("enter", false), &mode, 6);
        assert!(
            committed.removed() == ["cell-input"],
            "{mode}: Enter closes the editor"
        );
        assert!(
            focused(&session).0 == ["cells-grid"],
            "{mode}: focus returns to the grid"
        );
        let read = parts(&mut session, &mode);
        assert!(
            reads(&read[0], "cell-B1", "42"),
            "{mode}: the first part reads the committed value"
        );
    }
}

#[test]
fn a_click_right_after_the_scroll_lands_on_the_shown_scrollbar() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_cells(incremental);
        let scrolled = step(&mut session, &scroll("cell-Z25"), &mode, 0);
        assert!(
            scrolled.in_view == Some(true),
            "{mode}: step 0 brings the cell into view"
        );
        let early = step(&mut session, &click("cell-Z25"), &mode, 1);
        assert!(
            early.diff.is_empty(),
            "{mode}: step 1 lands on the shown scrollbar and opens nothing"
        );
    }
}

#[test]
fn the_lint_at_boot_reads_the_measured_findings() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_cells(incremental);
        let findings = linted(&mut session, &lint());
        assert!(
            findings.is_some_and(|findings| findings.len() == FINDINGS_AT_BOOT),
            "{mode}: the lint reads the measured findings"
        );
        assert!(
            matches!(session.run(&snapshot()), Ok(Outcome::Part { .. })),
            "{mode}: the screen still reads in parts after the lint"
        );
    }
}
