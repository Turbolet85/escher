//! One string names an element to the driver. For every node of each lean stand task's
//! snapshot, in both layout modes, the id the snapshot lists is the `author_id` of the element's
//! accessibility node, the element that id resolves to is that node's, and a driver `click`
//! naming it is accepted — but for the seven nodes that cannot take a click at boot, stated
//! here from the measured stand, each refused with its cause: the three disabled controls, and
//! the header's spacer, a box with no height, at whose centre the header is hit. And an
//! id-addressed click lands on the element it names: for each of the stand's 15 controls a
//! click is accepted on, the diff the driver returns is the diff a twin instance reads when
//! the same control is clicked by selector and settled — not empty for the controls a click
//! changes at boot, empty for the rest; for a control a click is refused on, the twin's
//! selector click changes nothing. The Timer has no changing control at boot, so its Reset is
//! read after time has moved. The mount is still the stand's after every action. A failure
//! message carries the layout mode, the task and a row index, never an id or what a screen
//! reads.

use std::collections::HashSet;

use blitz_dom::Document;
use escher_driver::Cause;
use seven_guis::stand::{self, LeanTask};

mod common;
mod session_common;
use common::{boot, controls};
use session_common::{act, advance, click, hold, refused};

/// The stand's mount: the shell under the app's root.
const MOUNT: &str = "main#main > #task-shell";

/// An id no stand element carries.
const NOBODY: &str = "act-ids-names-no-element";

/// The controls of `task` whose click changes what a snapshot reads at boot. The others are
/// the shell's `back-btn`, which has no Home behind it on a held instance, the choice already
/// made, the disabled controls, the range input and a Reset with nothing to reset.
fn changes_at_boot(task: LeanTask) -> &'static [&'static str] {
    match task {
        LeanTask::Counter => &["counter-increment"],
        LeanTask::FlightBooker => &["flight-return", "flight-start", "flight-book"],
        LeanTask::Timer => &[],
        LeanTask::Crud => &["crud-filter", "crud-name", "crud-surname", "crud-create"],
    }
}

/// The snapshot nodes of `task` a driver click is refused on at boot, each with its cause.
fn refused_at_boot(task: LeanTask) -> &'static [(&'static str, Cause)] {
    match task {
        LeanTask::Counter => &[("task-header-spacer", Cause::Covered)],
        LeanTask::FlightBooker => &[
            ("task-header-spacer", Cause::Covered),
            ("flight-return-date", Cause::Disabled),
        ],
        LeanTask::Timer => &[("task-header-spacer", Cause::Covered)],
        LeanTask::Crud => &[
            ("task-header-spacer", Cause::Covered),
            ("crud-update", Cause::Disabled),
            ("crud-delete", Cause::Disabled),
        ],
    }
}

/// The cause the table states for `id` on `task`: `None` for a node a click is accepted on.
fn stated_refusal(task: LeanTask, id: &str) -> Option<Cause> {
    refused_at_boot(task)
        .iter()
        .find(|(refused, _)| *refused == id)
        .map(|(_, cause)| *cause)
}

#[test]
fn the_snapshot_the_accessibility_tree_and_the_driver_read_one_id() {
    assert_eq!(
        LeanTask::ALL
            .map(|task| refused_at_boot(task).len())
            .iter()
            .sum::<usize>(),
        7
    );
    for incremental in [false, true] {
        let mut named_controls = HashSet::new();
        let (mut accepted, mut refusals) = (0, 0);
        for task in LeanTask::ALL {
            let mode = format!("incremental={incremental}: {task:?}");
            let (listing, _ticks) = hold(task, incremental);
            let ids: Vec<String> = listing
                .harness()
                .doc
                .snapshot()
                .nodes()
                .map(|node| node.id.clone())
                .collect();
            assert!(!ids.is_empty(), "{mode}: the snapshot has nodes");
            for (row, (control, _)) in controls(task).iter().enumerate() {
                assert!(
                    ids.iter().any(|id| id == control),
                    "{mode}: control {row} is a snapshot node"
                );
                named_controls.insert(*control);
            }

            for (row, id) in ids.iter().enumerate() {
                let (mut session, _ticks) = hold(task, incremental);
                {
                    let doc = &session.harness().doc;
                    assert!(
                        doc.snapshot().get(id).is_some(),
                        "{mode}: node {row} is in a fresh instance's snapshot"
                    );
                    let tree = doc.accessibility_tree();
                    let carrying: Vec<u64> = tree
                        .nodes
                        .iter()
                        .filter(|(_, node)| node.author_id() == Some(id.as_str()))
                        .map(|(tree_id, _)| tree_id.0)
                        .collect();
                    assert!(
                        carrying.len() == 1,
                        "{mode}: node {row}: one accessibility node carries the id as its author_id"
                    );
                    let resolved: Vec<u64> = doc
                        .element_ids()
                        .into_iter()
                        .filter(|(_, listed)| listed == id)
                        .map(|(node, _)| node.as_u64())
                        .collect();
                    assert!(
                        resolved == carrying,
                        "{mode}: node {row}: the element the id resolves to is that node's"
                    );
                }

                let stated = stated_refusal(task, id);
                assert!(
                    refused(&mut session, &click(id)) == stated,
                    "{mode}: node {row}: a driver click naming the id is accepted, or refused \
                     with the cause the table states"
                );
                accepted += usize::from(stated.is_none());
                refusals += usize::from(stated.is_some());
                assert!(
                    session.harness().query(MOUNT).is_some(),
                    "{mode}: node {row}: the mount is the stand's after the click"
                );
            }

            let (mut session, _ticks) = hold(task, incremental);
            assert!(
                session
                    .run(&click(NOBODY))
                    .is_err_and(|refusal| refusal.cause() == Cause::NotFound),
                "{mode}: an id no element carries is refused not-found"
            );
        }
        assert!(
            named_controls.len() == 15,
            "incremental={incremental}: the stand's 15 controls"
        );
        assert!(
            (accepted, refusals) == (70, 7),
            "incremental={incremental}: of the stand's 77 nodes a click is accepted on 70 and \
             refused on the table's 7"
        );
    }
}

#[test]
fn an_id_addressed_click_has_the_effect_of_a_selector_click_on_a_twin() {
    for incremental in [false, true] {
        let (mut changing, mut refused_controls) = (0, 0);
        let mut clicked = HashSet::new();
        for task in LeanTask::ALL {
            let mode = format!("incremental={incremental}: {task:?}");
            for (row, (control, _)) in controls(task).iter().enumerate() {
                let mut twin = boot(task, incremental);
                let before = twin.doc.snapshot();
                twin.click(&format!("#{control}"));
                assert!(
                    twin.settle().is_ok(),
                    "{mode}: control {row}: the twin settles"
                );
                let by_selector = before.diff(&twin.doc.snapshot());

                let changes = changes_at_boot(task).contains(control);
                assert!(
                    by_selector.is_empty() != changes,
                    "{mode}: control {row}: the selector click changes the twin's screen exactly \
                     when the table lists the control"
                );

                let (mut session, _ticks) = hold(task, incremental);
                if let Some(cause) = stated_refusal(task, control) {
                    assert!(
                        by_selector.is_empty(),
                        "{mode}: control {row}: a selector click on a control the driver \
                         refuses changes nothing on the twin"
                    );
                    assert!(
                        refused(&mut session, &click(control)) == Some(cause),
                        "{mode}: control {row}: the driver click is refused with the stated cause"
                    );
                    clicked.insert(*control);
                    refused_controls += 1;
                    continue;
                }
                let Some(by_id) = act(&mut session, &click(control)) else {
                    panic!("{mode}: control {row}: the driver click runs");
                };
                assert!(
                    by_id.settled,
                    "{mode}: control {row}: the driver click returns settled"
                );
                assert!(
                    by_id.diff == by_selector,
                    "{mode}: control {row}: the driver's diff is the selector click's"
                );
                assert!(
                    session.harness().query(MOUNT).is_some(),
                    "{mode}: control {row}: the mount is the stand's after the click"
                );
                changing += usize::from(changes);
                clicked.insert(*control);
            }
        }
        assert!(
            clicked.len() == 15,
            "incremental={incremental}: the stand's 15 controls"
        );
        assert!(
            changing == 8,
            "incremental={incremental}: eight controls change the screen at boot"
        );
        assert!(
            refused_controls == 3,
            "incremental={incremental}: three controls are refused at boot"
        );

        let mode = format!("incremental={incremental}: Timer");
        let (mut twin, ticks) = stand::boot_timer(stand::options(incremental));
        ticks.deliver(10);
        assert!(
            twin.settle().is_ok(),
            "{mode}: the twin settles its ten ticks"
        );
        let before = twin.doc.snapshot();
        twin.click("#timer-reset");
        assert!(twin.settle().is_ok(), "{mode}: the twin settles its Reset");
        let by_selector = before.diff(&twin.doc.snapshot());
        assert!(
            !by_selector.is_empty(),
            "{mode}: a selector click on Reset changes the twin's screen once time has moved"
        );

        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        assert!(
            act(&mut timer, &advance(1000)).is_some_and(|moved| moved.advanced_ms == Some(1000)),
            "{mode}: the driver moves the held Timer by the twin's ten ticks"
        );
        let Some(by_id) = act(&mut timer, &click("timer-reset")) else {
            panic!("{mode}: the driver click on Reset runs");
        };
        assert!(
            by_id.settled && by_id.diff == by_selector,
            "{mode}: the driver's diff for Reset is the selector click's"
        );
        assert!(
            timer.harness().query(MOUNT).is_some(),
            "{mode}: the mount is the stand's after the actions"
        );
    }
}
