//! The diff of the snapshots taken before and after a step names exactly the nodes the step
//! changed, proven through real input on the stand, booted headlessly in both layout modes: a
//! click names the value it rewrote, Create and Delete name a row added and removed, a focus
//! move names the controls whose focus moved, a typed character its text input and delivered
//! ticks the elapsed time — and a step that changes nothing reads an empty diff and a false
//! change flag. The cases the lean tasks lack are proven on three minimal fixtures booted with
//! the stand's options: an element hidden and shown by attribute, a password input and a
//! label whose text is rewritten. Every step is also checked against this file's own reading
//! of the two snapshots. A diff is content: this file formats none into a message and prints
//! nothing; a message carries the layout mode and ids this file names.

use std::collections::{BTreeSet, HashMap};

use accesskit::Role;
use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{
    DiffNode, DioxusDocument, MASKED_VALUE, Snapshot, SnapshotDiff, SnapshotNode,
};
use keyboard_types::{Key, Modifiers};
use seven_guis::stand::{self, LeanTask};

mod common;
use common::editor_text;

/// The text typed into the fixture's password input: synthetic, and no part of any name.
const SECRET: &str = "synthetic-pw-7Qz";

/// What one step read.
struct Step {
    /// Whether the engine's change flag read true after the step.
    flag: bool,
    /// The diff of the snapshots taken before and after the step.
    diff: SnapshotDiff,
    /// The ids of the nodes in both snapshots whose `focused` reading differs, in the later
    /// snapshot's order.
    focus_moved: Vec<String>,
}

impl Step {
    fn added(&self) -> Vec<&str> {
        self.diff.added.iter().map(|entry| &*entry.id).collect()
    }

    fn changed(&self) -> Vec<&str> {
        self.diff.changed.iter().map(|entry| &*entry.id).collect()
    }

    fn removed(&self) -> Vec<&str> {
        self.diff.removed.iter().map(String::as_str).collect()
    }

    /// Whether the diff adds or removes no node.
    fn keeps_every_node(&self) -> bool {
        self.diff.added.is_empty() && self.diff.removed.is_empty()
    }

    /// The entry the diff names `id` with, added or changed. `id` is one this file names, so
    /// the failure message may.
    #[track_caller]
    fn entry(&self, id: &str) -> &DiffNode {
        self.diff
            .added
            .iter()
            .chain(&self.diff.changed)
            .find(|entry| entry.id == id)
            .unwrap_or_else(|| panic!("{id:?} is an entry of the step's diff"))
    }
}

/// One booted screen in one layout mode, and the diffs its steps read.
struct Screen {
    harness: Harness<DioxusDocument>,
    diffs: Vec<SnapshotDiff>,
}

impl Screen {
    fn of(harness: Harness<DioxusDocument>) -> Self {
        Screen {
            harness,
            diffs: Vec::new(),
        }
    }

    fn boot(task: LeanTask, incremental: bool) -> Self {
        Self::of(stand::boot(task, stand::options(incremental)))
    }

    fn fixture(fixture: fn() -> Element, incremental: bool) -> Self {
        Self::of(Harness::from_vdom(
            VirtualDom::new(fixture),
            stand::options(incremental),
        ))
    }

    /// Take one step: read the snapshot, drain the changed set, run `act`, read the change
    /// flag and the snapshot again, and diff the two — asserted exact by [`assert_exact`].
    #[track_caller]
    fn step(&mut self, mode: &str, act: impl FnOnce(&mut Harness<DioxusDocument>)) -> Step {
        let before = self.harness.doc.snapshot();
        self.harness.base_mut().take_changed_nodes();
        act(&mut self.harness);
        let flag = self.harness.base().has_changes();
        let after = self.harness.doc.snapshot();
        let diff = before.diff(&after);
        assert_exact(mode, &before, &after, &diff);
        let focus_moved = after
            .nodes()
            .filter(|now| {
                before
                    .get(&now.id)
                    .is_some_and(|was| was.state.focused != now.state.focused)
            })
            .map(|now| now.id.clone())
            .collect();
        self.diffs.push(diff.clone());
        Step {
            flag,
            diff,
            focus_moved,
        }
    }

    /// The node `id` names on the screen now. `id` is one this file names.
    #[track_caller]
    fn node(&self, id: &str) -> SnapshotNode {
        self.harness
            .doc
            .snapshot()
            .get(id)
            .unwrap_or_else(|| panic!("{id:?} is a snapshot node"))
            .clone()
    }

    fn holds(&self, id: &str) -> bool {
        self.harness.doc.snapshot().get(id).is_some()
    }
}

/// Each id's parent id, read by this file's own walk of the tree. The first node carrying an
/// id stands for it, as `Snapshot::get` reads it.
fn parents(snapshot: &Snapshot) -> HashMap<&str, Option<&str>> {
    fn walk<'s>(
        node: &'s SnapshotNode,
        parent: Option<&'s str>,
        out: &mut HashMap<&'s str, Option<&'s str>>,
    ) {
        out.entry(node.id.as_str()).or_insert(parent);
        for child in &node.children {
            walk(child, Some(node.id.as_str()), out);
        }
    }
    let mut out = HashMap::new();
    for root in &snapshot.roots {
        walk(root, None, &mut out);
    }
    out
}

/// Whether `ids` keeps the order `snapshot` lists its nodes in.
fn follows(ids: &[&str], snapshot: &Snapshot) -> bool {
    let mut order = snapshot.nodes().map(|node| node.id.as_str());
    ids.iter().all(|id| order.any(|listed| listed == *id))
}

/// This file's own reading of what differs between two snapshots: an id is added or removed
/// when `Snapshot::get` finds it in one only, and changed when its role, name, state, bounds
/// or parent's id differ. Asserts the diff names every id of either snapshot that way and
/// names nothing else, that each added or changed entry carries what `after` reads, and that
/// the three lists keep their snapshot's order.
#[track_caller]
fn assert_exact(mode: &str, before: &Snapshot, after: &Snapshot, diff: &SnapshotDiff) {
    let parents_before = parents(before);
    let parents_after = parents(after);
    let ids: BTreeSet<&str> = before
        .nodes()
        .chain(after.nodes())
        .map(|node| node.id.as_str())
        .collect();
    assert!(!ids.is_empty(), "{mode}: the snapshots have nodes");

    let mut misnamed = 0;
    let mut expected_entries = 0;
    for id in &ids {
        let named = (
            diff.added.iter().filter(|entry| entry.id == *id).count(),
            diff.removed.iter().filter(|gone| gone == id).count(),
            diff.changed.iter().filter(|entry| entry.id == *id).count(),
        );
        let expected = match (before.get(id), after.get(id)) {
            (None, Some(_)) => (1, 0, 0),
            (Some(_), None) => (0, 1, 0),
            (Some(was), Some(now)) => {
                let differs = was.role != now.role
                    || was.name != now.name
                    || was.state != now.state
                    || was.bounds != now.bounds
                    || parents_before[id] != parents_after[id];
                (0, 0, usize::from(differs))
            }
            (None, None) => unreachable!("an id of neither snapshot"),
        };
        expected_entries += expected.0 + expected.1 + expected.2;
        if named != expected {
            misnamed += 1;
        }
    }
    assert!(
        misnamed == 0,
        "{mode}: the diff names {misnamed} ids otherwise than the two snapshots read"
    );
    let entries = diff.added.len() + diff.removed.len() + diff.changed.len();
    assert!(
        entries == expected_entries,
        "{mode}: the diff holds {entries} entries, the two snapshots read {expected_entries}"
    );

    let stale = diff
        .added
        .iter()
        .chain(&diff.changed)
        .filter(|entry| {
            after.get(&entry.id).is_none_or(|now| {
                entry.role != now.role
                    || entry.name != now.name
                    || entry.state != now.state
                    || entry.bounds != now.bounds
                    || entry.parent.as_deref() != parents_after[&*entry.id]
            })
        })
        .count();
    assert!(
        stale == 0,
        "{mode}: {stale} entries read otherwise than the snapshot after the step"
    );

    let added: Vec<&str> = diff.added.iter().map(|entry| &*entry.id).collect();
    let changed: Vec<&str> = diff.changed.iter().map(|entry| &*entry.id).collect();
    let removed: Vec<&str> = diff.removed.iter().map(String::as_str).collect();
    assert!(
        follows(&added, after) && follows(&changed, after) && follows(&removed, before),
        "{mode}: added and changed keep the later snapshot's order, removed the earlier one's"
    );
}

/// Asserts the two layout modes read the same diffs, step by step.
#[track_caller]
fn assert_modes_agree(runs: &[Vec<SnapshotDiff>]) {
    assert!(runs.len() == 2, "one run per layout mode");
    assert!(!runs[0].is_empty(), "a run takes a step");
    assert!(
        runs[0] == runs[1],
        "the steps' diffs differ between incremental false and true"
    );
}

#[test]
fn a_click_on_the_counter_names_its_value() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut counter = Screen::boot(LeanTask::Counter, incremental);
        assert!(
            counter.node("counter-value").name == "0",
            "{mode}: the counter is named 0 at boot"
        );
        for id in ["task-header", "back-btn", "task-title"] {
            assert!(counter.holds(id), "{mode}: {id:?} is a snapshot node");
        }

        let click = counter.step(&mode, |harness| harness.click("#counter-increment"));
        assert!(click.flag, "{mode}: the click reads as a change");
        assert!(
            click.changed() == ["counter-value"],
            "{mode}: the click names counter-value alone, and nothing in the header"
        );
        assert!(
            click.entry("counter-value").name == "1",
            "{mode}: counter-value reads its name after the click"
        );
        assert!(click.keeps_every_node(), "{mode}: no node comes or goes");

        runs.push(counter.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn create_names_the_new_row_added_and_delete_names_it_removed() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut crud = Screen::boot(LeanTask::Crud, incremental);
        crud.step(&mode, |harness| harness.click("#crud-name"));
        crud.step(&mode, |harness| harness.type_text("Ada"));
        crud.step(&mode, |harness| harness.click("#crud-surname"));
        crud.step(&mode, |harness| harness.type_text("Lovelace"));
        assert!(
            crud.holds("crud-person-2") && !crud.holds("crud-person-3"),
            "{mode}: the list holds three rows before Create"
        );

        let create = crud.step(&mode, |harness| harness.click("#crud-create"));
        assert!(create.flag, "{mode}: Create reads as a change");
        assert!(
            create.added() == ["crud-person-3"],
            "{mode}: Create names crud-person-3 added, and no other node"
        );
        let row = create.entry("crud-person-3");
        assert!(
            row.parent.as_deref() == Some("crud-list"),
            "{mode}: the new row is under crud-list"
        );
        assert!(
            row.name.contains("Ada") && row.name.contains("Lovelace"),
            "{mode}: the new row is named from the typed fields"
        );
        assert!(
            create.diff.removed.is_empty(),
            "{mode}: Create removes none"
        );

        let select = crud.step(&mode, |harness| harness.click("#crud-person-3"));
        assert!(select.keeps_every_node(), "{mode}: a selection keeps rows");
        assert!(
            crud.node("crud-delete").state.enabled == Some(true),
            "{mode}: the selection enables Delete"
        );

        let delete = crud.step(&mode, |harness| harness.click("#crud-delete"));
        assert!(delete.flag, "{mode}: Delete reads as a change");
        assert!(
            delete.removed() == ["crud-person-3"],
            "{mode}: Delete names crud-person-3 removed, and no other node"
        );
        assert!(delete.diff.added.is_empty(), "{mode}: Delete adds none");

        runs.push(crud.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_focus_move_names_exactly_the_controls_whose_focus_moved() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut crud = Screen::boot(LeanTask::Crud, incremental);
        assert!(
            crud.harness
                .doc
                .snapshot()
                .nodes()
                .all(|node| !node.state.focused),
            "{mode}: nothing is focused at boot"
        );

        let click = crud.step(&mode, |harness| harness.click("#crud-name"));
        assert!(click.flag, "{mode}: a click on a text input is a change");
        assert!(
            click.changed() == ["crud-name"] && click.changed() == click.focus_moved,
            "{mode}: the click names crud-name alone"
        );
        assert!(
            click.entry("crud-name").state.focused,
            "{mode}: crud-name reads focused after the click"
        );

        let tab = crud.step(&mode, |harness| harness.press(Key::Tab));
        assert!(tab.flag, "{mode}: Tab reads as a change");
        assert!(
            tab.changed() == ["crud-name", "crud-surname"] && tab.changed() == tab.focus_moved,
            "{mode}: Tab names the control that lost focus and the one that gained it"
        );
        assert!(
            !tab.entry("crud-name").state.focused && tab.entry("crud-surname").state.focused,
            "{mode}: focus reads on crud-surname after Tab"
        );

        let back = crud.step(&mode, |harness| {
            harness.press_with(Key::Tab, Modifiers::SHIFT)
        });
        assert!(back.flag, "{mode}: Shift+Tab reads as a change");
        assert!(
            back.changed() == ["crud-name", "crud-surname"] && back.changed() == back.focus_moved,
            "{mode}: Shift+Tab names the same two controls"
        );
        assert!(
            back.entry("crud-name").state.focused && !back.entry("crud-surname").state.focused,
            "{mode}: focus reads on crud-name after Shift+Tab"
        );
        for (name, step) in [("click", &click), ("Tab", &tab), ("Shift+Tab", &back)] {
            assert!(step.keeps_every_node(), "{mode}: {name} keeps every node");
        }

        let mut fresh = Screen::boot(LeanTask::Crud, incremental);
        let first = fresh.step(&mode, |harness| harness.press(Key::Tab));
        assert!(first.flag, "{mode}: the first Tab reads as a change");
        assert!(
            first.changed() == ["back-btn"] && first.changed() == first.focus_moved,
            "{mode}: the first Tab on a fresh boot names back-btn alone"
        );
        assert!(
            first.entry("back-btn").state.focused && first.keeps_every_node(),
            "{mode}: back-btn reads focused after the first Tab"
        );

        runs.push([crud.diffs, fresh.diffs].concat());
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_typed_character_names_its_text_input() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut crud = Screen::boot(LeanTask::Crud, incremental);
        crud.step(&mode, |harness| harness.click("#crud-name"));
        assert!(
            crud.node("crud-name").state.value.as_deref() == Some(""),
            "{mode}: crud-name is empty before the character"
        );

        let typed = crud.step(&mode, |harness| harness.type_text("A"));
        assert!(typed.flag, "{mode}: a typed character reads as a change");
        assert!(
            typed.changed() == ["crud-name"],
            "{mode}: the character names crud-name alone"
        );
        assert!(
            typed.entry("crud-name").state.value.as_deref() == Some("A"),
            "{mode}: crud-name reads the character as its value"
        );
        assert!(typed.keeps_every_node(), "{mode}: no node comes or goes");

        runs.push(crud.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn delivered_ticks_name_the_elapsed_time_and_an_idle_pump_names_nothing() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (harness, ticks) = stand::boot_timer(stand::options(incremental));
        let mut timer = Screen::of(harness);
        assert!(
            timer.node("timer-elapsed").name == "Elapsed: 0.0s",
            "{mode}: no time has elapsed at boot"
        );

        let ticked = timer.step(&mode, |harness| {
            ticks.deliver(3);
            harness.pump();
        });
        assert!(ticked.flag, "{mode}: delivered ticks read as a change");
        assert!(
            ticked.changed() == ["timer-progress", "timer-elapsed"],
            "{mode}: the ticks name timer-elapsed and the progress bar they widen"
        );
        assert!(
            ticked.entry("timer-elapsed").name == "Elapsed: 0.3s",
            "{mode}: timer-elapsed reads its name after the ticks"
        );
        assert!(ticked.keeps_every_node(), "{mode}: no node comes or goes");

        let idle = timer.step(&mode, |harness| harness.pump());
        assert!(
            idle.diff.is_empty(),
            "{mode}: a pump with nothing delivered reads an empty diff"
        );
        assert!(
            !idle.flag,
            "{mode}: a pump with nothing delivered is no change"
        );

        runs.push(timer.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_step_that_changes_nothing_reads_an_empty_diff() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut diffs = Vec::new();
        for task in LeanTask::ALL {
            let mut screen = Screen::boot(task, incremental);
            let idle = screen.step(&mode, |harness| harness.pump());
            assert!(
                idle.diff.is_empty(),
                "{mode}: a pump with nothing pending on {task:?} reads an empty diff"
            );
            assert!(
                !idle.flag,
                "{mode}: a pump with nothing pending on {task:?} is no change"
            );
            diffs.extend(screen.diffs);
        }

        let mut crud = Screen::boot(LeanTask::Crud, incremental);
        assert!(
            crud.node("crud-delete").state.enabled == Some(false),
            "{mode}: crud-delete is disabled at boot"
        );
        let click = crud.step(&mode, |harness| harness.click("#crud-delete"));
        assert!(
            click.diff.is_empty(),
            "{mode}: a click on the disabled crud-delete reads an empty diff"
        );
        assert!(
            !click.flag,
            "{mode}: a click on the disabled crud-delete is no change"
        );
        diffs.extend(crud.diffs);

        runs.push(diffs);
    }
    assert_modes_agree(&runs);
}

/// A panel a button hides and shows by its `hidden` attribute.
fn hidden_fixture() -> Element {
    let mut hidden = use_signal(|| false);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-toggle", onclick: move |_| hidden.toggle(), "Toggle" }
            div { id: "fx-panel", hidden: hidden(),
                p { id: "fx-line", "Shown" }
                button { id: "fx-inner", "Inner" }
            }
        }
    }
}

/// The hidden fixture's panel and its descendants, in document order.
const PANEL: [&str; 3] = ["fx-panel", "fx-line", "fx-inner"];

#[test]
fn a_hidden_element_leaves_with_its_descendants_and_returns_with_them() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut screen = Screen::fixture(hidden_fixture, incremental);
        for id in PANEL {
            assert!(
                screen.holds(id),
                "{mode}: {id:?} is a snapshot node at boot"
            );
        }

        let hide = screen.step(&mode, |harness| harness.click("#fx-toggle"));
        assert!(
            screen.harness.attr("#fx-panel", "hidden").is_some(),
            "{mode}: the click writes the hidden attribute"
        );
        assert!(hide.flag, "{mode}: hiding reads as a change");
        assert!(
            hide.removed() == PANEL,
            "{mode}: the hidden panel leaves with its descendants"
        );
        assert!(hide.diff.added.is_empty(), "{mode}: hiding adds none");

        let show = screen.step(&mode, |harness| harness.click("#fx-toggle"));
        assert!(
            screen.harness.attr("#fx-panel", "hidden").is_none(),
            "{mode}: the second click takes the hidden attribute away"
        );
        assert!(show.flag, "{mode}: showing reads as a change");
        assert!(
            show.added() == PANEL,
            "{mode}: the shown panel returns with its descendants"
        );
        for (id, parent) in [
            ("fx-panel", "fx-root"),
            ("fx-line", "fx-panel"),
            ("fx-inner", "fx-panel"),
        ] {
            assert!(
                show.entry(id).parent.as_deref() == Some(parent),
                "{mode}: {id:?} returns under {parent:?}"
            );
        }
        assert!(show.diff.removed.is_empty(), "{mode}: showing removes none");

        runs.push(screen.diffs);
    }
    assert_modes_agree(&runs);
}

/// A labelled password input, the control the lean tasks lack.
fn password_fixture() -> Element {
    rsx! {
        div { id: "fx-root",
            label { r#for: "fx-secret", "Passphrase" }
            input { id: "fx-secret", r#type: "password" }
        }
    }
}

#[test]
fn a_typed_password_is_named_once_and_only_by_its_mask() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut screen = Screen::fixture(password_fixture, incremental);
        screen.step(&mode, |harness| harness.click("#fx-secret"));
        assert!(
            screen.node("fx-secret").state.value.as_deref() == Some(""),
            "{mode}: an empty password reads empty"
        );

        let first = screen.step(&mode, |harness| harness.type_text(&SECRET[..1]));
        assert!(first.flag, "{mode}: the first character reads as a change");
        assert!(
            first.changed() == ["fx-secret"] && first.keeps_every_node(),
            "{mode}: the first character names fx-secret alone"
        );
        assert!(
            first.entry("fx-secret").state.value.as_deref() == Some(MASKED_VALUE),
            "{mode}: fx-secret reads the mask as its value"
        );

        let second = screen.step(&mode, |harness| harness.type_text(&SECRET[1..2]));
        assert!(second.flag, "{mode}: the second character is written");
        assert!(
            second.diff.is_empty(),
            "{mode}: the second character changes no reading"
        );

        let rest = screen.step(&mode, |harness| harness.type_text(&SECRET[2..]));
        assert!(rest.flag, "{mode}: the further characters are written");
        assert!(
            rest.diff.is_empty(),
            "{mode}: the further characters change no reading"
        );

        assert!(
            editor_text(&screen.harness, "fx-secret") == SECRET,
            "{mode}: the password input holds the typed text"
        );
        for step_diff in &screen.diffs {
            let carried = step_diff
                .added
                .iter()
                .chain(&step_diff.changed)
                .flat_map(|entry| {
                    [
                        Some(entry.id.as_str()),
                        entry.parent.as_deref(),
                        Some(entry.name.as_str()),
                        entry.state.value.as_deref(),
                    ]
                })
                .flatten()
                .chain(step_diff.removed.iter().map(String::as_str));
            assert!(
                carried.into_iter().all(|field| !field.contains(SECRET)),
                "{mode}: a field of a diff entry holds the typed text"
            );
        }

        runs.push(screen.diffs);
    }
    assert_modes_agree(&runs);
}

/// A text input whose label a button rewrites.
fn label_fixture() -> Element {
    let mut renames = use_signal(|| 0);
    rsx! {
        div { id: "fx-root",
            label { id: "fx-label", r#for: "fx-field", "Field {renames}" }
            input { id: "fx-field" }
            button { id: "fx-rename", onclick: move |_| renames += 1, "Rename" }
        }
    }
}

#[test]
fn a_rewritten_label_names_the_input_it_labels() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut screen = Screen::fixture(label_fixture, incremental);
        assert!(
            screen.node("fx-field").name == "Field 0",
            "{mode}: the input is named by its label at boot"
        );

        let rename = screen.step(&mode, |harness| harness.click("#fx-rename"));
        assert!(rename.flag, "{mode}: the rewrite reads as a change");
        assert!(
            rename.changed() == ["fx-label", "fx-field"],
            "{mode}: the rewrite names the label and the input it labels"
        );
        let field = rename.entry("fx-field");
        assert!(
            field.name == "Field 1" && field.role == Role::TextInput,
            "{mode}: fx-field reads its new name"
        );
        assert!(
            rename
                .diff
                .changed
                .iter()
                .all(|entry| { entry.role != Role::TextRun && screen.holds(&entry.id) }),
            "{mode}: every entry is an element of the screen, none a text run"
        );
        assert!(rename.keeps_every_node(), "{mode}: no node comes or goes");

        runs.push(screen.diffs);
    }
    assert_modes_agree(&runs);
}
