//! A driver `press` acts on the element an id-addressed action focused. On the stand's CRUD
//! task, in both layout modes: after a driver `click` on a text input, a `press` of Tab and of
//! Tab with shift each return a diff naming exactly the two controls whose `focused` reading
//! moved, and the one node reading focused is the accessibility tree's focus; and after a driver
//! `type` of three characters, a `press` of Backspace leaves two. That last step is the one
//! check driven by a deleting key: the key's platform behaviour is what it proves, and on macOS,
//! where the editor leaves the backward delete to the standard key bindings, its reading is the
//! CI run's. A failure message carries the layout mode and a step index, never an id or what a
//! screen reads.

use escher_driver::{Call, Session};
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use common::editor_text;
use session_common::{Acted, act, click, focused, hold, press, type_into};

/// Runs `call` through the driver between this file's own two snapshots: what it returned, and
/// the ids of the nodes in both snapshots whose `focused` reading differs, in the later one's
/// order.
#[track_caller]
fn step(mode: &str, index: usize, session: &mut Session, call: &Call) -> (Acted, Vec<String>) {
    let before = session.harness().doc.snapshot();
    let Some(acted) = act(session, call) else {
        panic!("{mode}: step {index} runs");
    };
    assert!(
        acted.settled && acted.busy.is_none(),
        "{mode}: step {index} returns settled"
    );
    let after = session.harness().doc.snapshot();
    let focus_moved = after
        .nodes()
        .filter(|now| {
            before
                .get(&now.id)
                .is_some_and(|was| was.state.focused != now.state.focused)
        })
        .map(|now| now.id.clone())
        .collect();
    (acted, focus_moved)
}

#[test]
fn tab_and_shift_tab_return_exactly_the_controls_whose_focus_moved() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);
        let (in_snapshot, in_tree) = focused(&crud);
        assert!(
            in_snapshot.is_empty() && in_tree.is_empty(),
            "{mode}: nothing reads focused on a started session"
        );

        // The call, the controls its diff names changed, and the control reading focused after.
        let steps: [(Call, &[&str], &str); 3] = [
            (click("crud-name"), &["crud-name"], "crud-name"),
            (
                press("tab", false),
                &["crud-name", "crud-surname"],
                "crud-surname",
            ),
            (
                press("tab", true),
                &["crud-name", "crud-surname"],
                "crud-name",
            ),
        ];
        assert_eq!(steps.len(), 3);
        for (index, (call, named, holder)) in steps.iter().enumerate() {
            let (acted, focus_moved) = step(&mode, index, &mut crud, call);
            assert!(
                acted.changed() == *named && acted.added().is_empty() && acted.removed().is_empty(),
                "{mode}: step {index} names exactly the controls stated for it"
            );
            assert!(
                acted.changed() == focus_moved,
                "{mode}: step {index} names exactly the controls whose focused reading moved"
            );
            let (in_snapshot, in_tree) = focused(&crud);
            assert!(
                in_snapshot == [*holder],
                "{mode}: step {index} leaves one node reading focused, the control stated"
            );
            assert!(
                in_tree == in_snapshot,
                "{mode}: step {index}: that node is the accessibility tree's focus"
            );
        }
    }
}

#[test]
fn backspace_deletes_one_character_of_what_the_driver_typed() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);

        let (typed, _) = step(&mode, 0, &mut crud, &type_into("crud-name", "Ada"));
        assert!(
            typed.changed() == ["crud-name"]
                && typed.diff.changed[0].state.value.as_deref() == Some("Ada"),
            "{mode}: the three typed characters are returned as the input's value"
        );
        assert!(
            editor_text(crud.harness(), "crud-name") == "Ada",
            "{mode}: the input's editor holds the three characters"
        );

        let (deleted, _) = step(&mode, 1, &mut crud, &press("backspace", false));
        assert!(
            deleted.changed() == ["crud-name"]
                && deleted.added().is_empty()
                && deleted.removed().is_empty(),
            "{mode}: the key names the input alone"
        );
        assert!(
            deleted.diff.changed[0].state.value.as_deref() == Some("Ad"),
            "{mode}: the key leaves two characters"
        );
        assert!(
            editor_text(crud.harness(), "crud-name") == "Ad",
            "{mode}: the input's editor holds the two characters"
        );
    }
}
