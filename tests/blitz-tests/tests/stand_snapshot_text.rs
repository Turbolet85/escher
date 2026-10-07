//! The text form of each lean stand task's snapshot, booted headlessly in both layout modes:
//! every snapshot node is one line carrying its role, name, id, state and bounds, nested by
//! indent; the text is the same across two calls, the two layout modes and a second boot; and
//! each screen stays inside its recorded ceiling and the snapshot text budget. On two minimal
//! fixtures booted with the stand's options, a typed password and a file input's path read
//! the fixed mask and occur nowhere in the text. The text is content: this file binds it only
//! to `text`-named variables, never formats one into a message and prints nothing.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{
    DioxusDocument, MASKED_VALUE, SNAPSHOT_TEXT_BUDGET, Snapshot, SnapshotNode,
};
use keyboard_types::Key;
use seven_guis::stand::{self, LeanTask};

mod common;
use common::{boot, editor_text, rendered};

/// The text typed into the fixture's password input: synthetic, and no part of any name.
const SECRET: &str = "synthetic-pw-7Qz";

/// The value the fixture's file input is authored with: a synthetic path naming no real file.
const UPLOAD_PATH: &str = "/synthetic/dir/chosen-report.txt";

/// The file name of [`UPLOAD_PATH`].
const UPLOAD_NAME: &str = "chosen-report";

/// The ceiling each lean task's screen is held under, in bytes of its text: the larger of
/// its two layout modes' measured lengths rounded down to a multiple of 256, plus 512.
fn ceiling(task: LeanTask) -> usize {
    match task {
        LeanTask::Counter => 1024,
        LeanTask::FlightBooker => 1536,
        LeanTask::Timer => 1536,
        LeanTask::Crud => 2304,
    }
}

/// The snapshot's nodes in pre-order, each with its depth below a root.
fn with_depths(snapshot: &Snapshot) -> Vec<(&SnapshotNode, usize)> {
    let mut out = Vec::new();
    let mut stack: Vec<(&SnapshotNode, usize)> =
        snapshot.roots.iter().rev().map(|root| (root, 0)).collect();
    while let Some((node, depth)) = stack.pop() {
        out.push((node, depth));
        stack.extend(node.children.iter().rev().map(|child| (child, depth + 1)));
    }
    out
}

/// The number of spaces a line opens with.
fn indent_of(text_line: &str) -> usize {
    text_line.len() - text_line.trim_start_matches(' ').len()
}

/// How a line writes `id` as its id field.
fn id_field(id: &str) -> String {
    format!(" id={id:?} ")
}

/// The one line of `text` carrying `id` as its id, with its index. `id` is one this file
/// names, so the failure message may.
#[track_caller]
fn line_of<'t>(text: &'t str, id: &str) -> (usize, &'t str) {
    let field = id_field(id);
    let mut text_found = text
        .lines()
        .enumerate()
        .filter(|(_, text_line)| text_line.contains(&field));
    let text_first = text_found
        .next()
        .unwrap_or_else(|| panic!("{id:?} has a line"));
    assert!(text_found.next().is_none(), "{id:?} has one line only");
    text_first
}

/// The tokens a line holds outside its quoted strings: the role, `id=`, the state tokens and
/// the bounds. A quoted string is escaped, so inside one a `"` always follows a backslash.
fn bare_tokens(text_line: &str) -> Vec<String> {
    let mut text_bare = String::new();
    let mut quoted = false;
    let mut chars = text_line.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => quoted = !quoted,
            '\\' if quoted => {
                chars.next();
            }
            _ if quoted => {}
            _ => text_bare.push(c),
        }
    }
    text_bare.split_whitespace().map(str::to_string).collect()
}

/// Whether a line holds `token` as a state token.
fn holds(text_line: &str, token: &str) -> bool {
    bare_tokens(text_line).iter().any(|bare| bare == token)
}

/// `task`'s screen in one layout mode: one line per snapshot node, in the order of `nodes()`,
/// each carrying its node's fields, with the rendered ids among them.
fn a_screen_is_one_line_per_node(task: LeanTask, incremental: bool) {
    let mode = format!("{task:?} incremental={incremental}");
    let snapshot = boot(task, incremental).doc.snapshot();
    let nodes = with_depths(&snapshot);
    assert!(!nodes.is_empty(), "{mode}: the snapshot has nodes");
    assert!(
        nodes.len() == snapshot.nodes().count()
            && nodes
                .iter()
                .zip(snapshot.nodes())
                .all(|((node, _), listed)| std::ptr::eq(*node, listed)),
        "{mode}: the depth walk follows the order of nodes()"
    );

    let text = snapshot.to_text();
    let text_lines: Vec<&str> = text.lines().collect();
    assert!(text.ends_with('\n'), "{mode}: the last line is ended");
    assert!(
        text_lines.len() == nodes.len(),
        "{mode}: {} lines for {} nodes",
        text_lines.len(),
        nodes.len()
    );

    each_line_carries_its_nodes_fields(&mode, &nodes, &text_lines);

    for id in rendered(task) {
        line_of(&text, id);
    }
    if task == LeanTask::Crud {
        the_rows_nest_under_the_list(&mode, &text, &text_lines);
    }
}

/// Each line is indented for its node's depth, opens with the node's role, name and id, ends
/// with its bounds, and is the one line carrying that id.
fn each_line_carries_its_nodes_fields(
    mode: &str,
    nodes: &[(&SnapshotNode, usize)],
    text_lines: &[&str],
) {
    for (index, ((node, depth), text_line)) in nodes.iter().zip(text_lines).enumerate() {
        assert!(
            indent_of(text_line) == 2 * depth,
            "{mode}: line {index} is indented {} for depth {depth}",
            indent_of(text_line)
        );
        let head = format!("{:?} {:?}{}", node.role, node.name, id_field(&node.id));
        assert!(
            text_line.trim_start_matches(' ').starts_with(&head),
            "{mode}: line {index} opens with its node's role, name and id"
        );
        let rect = node.bounds;
        let bounds = format!(" @{},{} {}x{}", rect.x, rect.y, rect.width, rect.height);
        assert!(
            text_line.ends_with(&bounds),
            "{mode}: line {index} ends with its node's bounds"
        );
        let field = id_field(&node.id);
        let carriers = text_lines
            .iter()
            .filter(|text_other| text_other.contains(&field))
            .count();
        assert!(
            carriers == 1,
            "{mode}: the id of line {index} is on {carriers} lines"
        );
    }
}

/// The three CRUD rows come after `crud-list` in fixture order, each nested under it.
fn the_rows_nest_under_the_list(mode: &str, text: &str, text_lines: &[&str]) {
    let (list_at, text_list) = line_of(text, "crud-list");
    let mut before = list_at;
    for id in ["crud-person-0", "crud-person-1", "crud-person-2"] {
        let (row_at, _) = line_of(text, id);
        assert!(
            row_at > before,
            "{mode}: {id:?} comes after crud-list and the row before it"
        );
        assert!(
            text_lines[list_at + 1..=row_at]
                .iter()
                .all(|text_under| indent_of(text_under) > indent_of(text_list)),
            "{mode}: {id:?} is nested under crud-list"
        );
        before = row_at;
    }
}

#[test]
fn every_node_is_one_line_with_its_five_fields() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            a_screen_is_one_line_per_node(task, incremental);
        }
    }
}

#[test]
fn state_reads_as_the_model_holds_it() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let mode = format!("{task:?} incremental={incremental}");
            let mut harness = boot(task, incremental);
            let text = harness.doc.snapshot().to_text();
            assert!(!text.is_empty(), "{mode}: the screen has lines");
            assert!(
                text.lines().all(|text_line| !holds(text_line, "focused")),
                "{mode}: no line reads focused at boot"
            );

            harness.press(Key::Tab);
            let text = harness.doc.snapshot().to_text();
            let focused = text
                .lines()
                .filter(|text_line| holds(text_line, "focused"))
                .count();
            assert!(
                focused == 1,
                "{mode}: {focused} lines read focused after Tab"
            );
            let (_, text_back) = line_of(&text, "back-btn");
            assert!(
                holds(text_back, "focused"),
                "{mode}: the focused line is back-btn's"
            );
        }
    }

    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let flight = boot(LeanTask::FlightBooker, incremental);
        let text = flight.doc.snapshot().to_text();
        let (_, text_return) = line_of(&text, "flight-return-date");
        assert!(
            holds(text_return, "disabled") && !holds(text_return, "enabled"),
            "{mode}: the return date reads disabled, one-way at boot"
        );
        let date = editor_text(&flight, "flight-start");
        assert!(!date.is_empty(), "{mode}: the fixture fills the start date");
        let (_, text_start) = line_of(&text, "flight-start");
        assert!(
            holds(text_start, "enabled") && !holds(text_start, "disabled"),
            "{mode}: the start date reads enabled"
        );
        assert!(
            text_start.contains(&format!(" value={date:?} ")),
            "{mode}: the start date's line holds the date the editor holds"
        );

        let mut counter = boot(LeanTask::Counter, incremental);
        let text = counter.doc.snapshot().to_text();
        let (_, text_value) = line_of(&text, "counter-value");
        assert!(
            text_value.contains(" \"0\" id=\"counter-value\" "),
            "{mode}: the counter is named 0 at boot"
        );
        counter.click("#counter-increment");
        let text = counter.doc.snapshot().to_text();
        let (_, text_value) = line_of(&text, "counter-value");
        assert!(
            text_value.contains(" \"1\" id=\"counter-value\" "),
            "{mode}: the counter is named 1 after a click"
        );
    }
}

#[test]
fn the_text_is_deterministic() {
    for task in LeanTask::ALL {
        let mut texts = Vec::new();
        for incremental in [false, true] {
            let mode = format!("{task:?} incremental={incremental}");
            let harness = boot(task, incremental);
            let text_first = harness.doc.snapshot().to_text();
            assert!(!text_first.is_empty(), "{mode}: the screen has lines");
            let text_again = harness.doc.snapshot().to_text();
            assert!(text_first == text_again, "{mode}: a second call differs");
            let text_reboot = boot(task, incremental).doc.snapshot().to_text();
            assert!(text_first == text_reboot, "{mode}: a second boot differs");
            texts.push(text_first);
        }
        assert!(
            texts[0] == texts[1],
            "{task:?}: incremental false and true differ"
        );
    }
}

#[test]
fn every_screen_fits_the_budget() {
    let mut over = Vec::new();
    for task in LeanTask::ALL {
        let ceiling = ceiling(task);
        assert!(
            ceiling <= SNAPSHOT_TEXT_BUDGET,
            "{task:?}: the ceiling {ceiling} is over the budget {SNAPSHOT_TEXT_BUDGET}"
        );
        for incremental in [false, true] {
            let len = boot(task, incremental).doc.snapshot().to_text().len();
            assert!(len > 0, "{task:?} incremental={incremental}: no line");
            if len > ceiling {
                over.push(format!(
                    "{task:?} incremental={incremental}: {len} bytes, ceiling {ceiling}"
                ));
            }
        }
    }
    assert!(over.is_empty(), "screens over their ceiling: {over:#?}");

    for incremental in [false, true] {
        let mut crud = boot(LeanTask::Crud, incremental);
        let at_boot = crud.doc.snapshot().to_text().len();
        crud.click("#crud-create");
        let after = crud.doc.snapshot().to_text().len();
        assert!(
            after > at_boot,
            "Crud incremental={incremental}: {after} bytes after Create, {at_boot} at boot"
        );
        assert!(
            after <= SNAPSHOT_TEXT_BUDGET,
            "Crud incremental={incremental}: {after} bytes after Create, budget {SNAPSHOT_TEXT_BUDGET}"
        );
    }
}

/// A labelled password input, the control the lean tasks lack.
fn password_fixture() -> Element {
    rsx! {
        div {
            label { r#for: "fx-secret", "Passphrase" }
            input { id: "fx-secret", r#type: "password" }
        }
    }
}

/// A labelled file input authored with a path as its value, as a chosen file leaves it.
fn file_fixture() -> Element {
    rsx! {
        div {
            label { r#for: "fx-upload", "Attachment" }
            input { id: "fx-upload", r#type: "file", value: UPLOAD_PATH }
        }
    }
}

fn boot_fixture(fixture: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(fixture), stand::options(incremental))
}

/// How a line writes the mask as its value.
fn masked_field() -> String {
    format!(" value={MASKED_VALUE:?} ")
}

#[test]
fn a_masked_password_stays_masked_in_the_text() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot_fixture(password_fixture, incremental);
        let text = harness.doc.snapshot().to_text();
        let (_, text_secret) = line_of(&text, "fx-secret");
        assert!(
            text_secret.contains(" value=\"\" "),
            "{mode}: an empty password writes an empty value"
        );

        harness.click("#fx-secret");
        harness.type_text(SECRET);
        assert!(
            editor_text(&harness, "fx-secret") == SECRET,
            "{mode}: the password input holds the typed text"
        );
        let text = harness.doc.snapshot().to_text();
        let (_, text_secret) = line_of(&text, "fx-secret");
        assert!(
            text_secret.contains(&masked_field()),
            "{mode}: a password holding text writes the mask"
        );
        assert!(
            !text.contains(SECRET),
            "{mode}: the typed text occurs in the screen's text"
        );
    }
}

#[test]
fn a_masked_file_input_stays_masked_in_the_text() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = boot_fixture(file_fixture, incremental);
        assert!(
            harness.attr("#fx-upload", "value").as_deref() == Some(UPLOAD_PATH),
            "{mode}: the fixture's file input holds the path"
        );
        let text = harness.doc.snapshot().to_text();
        let (_, text_upload) = line_of(&text, "fx-upload");
        assert!(
            text_upload.contains(&masked_field()),
            "{mode}: a file input holding a path writes the mask"
        );
        assert!(
            !text.contains(UPLOAD_PATH) && !text.contains(UPLOAD_NAME),
            "{mode}: the path occurs in the screen's text"
        );
    }
}
