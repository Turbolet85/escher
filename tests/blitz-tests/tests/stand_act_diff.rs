//! What a driver action returns is the diff of the screen before and after it. On each lean
//! stand task, in both layout modes, the diff an acting call returns equals `Snapshot::diff`
//! over this file's own snapshots taken either side of the call, and names exactly the nodes
//! stated per step: a click names the value it rewrote, a typed text its input and the control
//! that lost focus, Create the new row added and Delete that row removed, an advance the
//! elapsed time. A click on an element that reacts to nothing returns settled with an empty
//! diff, and the two layout modes return equal diffs. The case the lean tasks lack is proven on
//! a fixture booted with the stand's options: a text typed by the driver into a password input
//! is nowhere in what the call returns, and the entry reads the mask. A diff is content: this
//! file prints none, and a failure message carries the layout mode, the task and a step index,
//! never an id or what a screen reads.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DiffNode, MASKED_VALUE, SnapshotDiff};
use escher_driver::{Call, Outcome, Session};
use seven_guis::stand::{self, LeanTask};

mod common;
mod session_common;
use common::editor_text;
use session_common::{Acted, act, advance, click, hold, type_into};

/// The text the driver types into the fixture's password input: synthetic, and no part of any
/// id or name.
const SECRET: &str = "synthetic-pw-7Qz";

/// What a step's diff names: the ids added, removed and changed, each in the diff's order.
type Named = [&'static [&'static str]; 3];

/// One held session in one layout mode, and the diffs its steps returned.
struct Held {
    mode: String,
    session: Session,
    diffs: Vec<SnapshotDiff>,
}

impl Held {
    fn task(task: LeanTask, incremental: bool) -> Self {
        Held {
            mode: format!("incremental={incremental}: {task:?}"),
            session: hold(task, incremental).0,
            diffs: Vec::new(),
        }
    }

    /// Runs `call` through the driver between this file's own two snapshots: asserts it
    /// returns settled, that its diff is the diff of those snapshots and that the diff names
    /// exactly `named`.
    #[track_caller]
    fn step(&mut self, call: &Call, named: Named) -> Acted {
        let mode = &self.mode;
        let step = self.diffs.len();
        let before = self.session.harness().doc.snapshot();
        let Some(acted) = act(&mut self.session, call) else {
            panic!("{mode}: step {step} runs");
        };
        let after = self.session.harness().doc.snapshot();
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: step {step} returns settled"
        );
        assert!(
            acted.diff == before.diff(&after),
            "{mode}: step {step} returns the diff of the snapshots before and after it"
        );
        assert!(
            [acted.added(), acted.removed(), acted.changed()] == named,
            "{mode}: step {step} names exactly the nodes stated for it"
        );
        self.diffs.push(acted.diff.clone());
        acted
    }
}

/// The entry a diff names `id` with, added or changed.
fn entry<'d>(acted: &'d Acted, id: &str) -> Option<&'d DiffNode> {
    acted
        .diff
        .added
        .iter()
        .chain(&acted.diff.changed)
        .find(|entry| entry.id == id)
}

/// Asserts the two layout modes returned the same diffs, step by step.
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
fn a_click_on_the_counter_returns_the_value_it_rewrote() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut counter = Held::task(LeanTask::Counter, incremental);
        let first = counter.step(&click("counter-increment"), [&[], &[], &["counter-value"]]);
        assert!(
            entry(&first, "counter-value").is_some_and(|value| value.name == "1"),
            "{}: the first click returns the count it wrote",
            counter.mode
        );
        let second = counter.step(&click("counter-increment"), [&[], &[], &["counter-value"]]);
        assert!(
            entry(&second, "counter-value").is_some_and(|value| value.name == "2"),
            "{}: the second click returns the count it wrote",
            counter.mode
        );
        runs.push(counter.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_choice_on_the_flight_booker_returns_the_control_it_enabled() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut flight = Held::task(LeanTask::FlightBooker, incremental);
        let chosen = flight.step(&click("flight-return"), [&[], &[], &["flight-return-date"]]);
        assert!(
            entry(&chosen, "flight-return-date")
                .is_some_and(|date| date.state.enabled == Some(true)),
            "{}: a return flight returns the return date enabled",
            flight.mode
        );
        let back = flight.step(
            &click("flight-one-way"),
            [&[], &[], &["flight-return-date"]],
        );
        assert!(
            entry(&back, "flight-return-date")
                .is_some_and(|date| date.state.enabled == Some(false)),
            "{}: a one-way flight returns the return date disabled",
            flight.mode
        );
        runs.push(flight.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn an_advance_on_the_timer_returns_the_elapsed_time() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut timer = Held::task(LeanTask::Timer, incremental);
        let moved = timer.step(
            &advance(300),
            [&[], &[], &["timer-progress", "timer-elapsed"]],
        );
        assert!(
            entry(&moved, "timer-elapsed").is_some_and(|elapsed| elapsed.name == "Elapsed: 0.3s"),
            "{}: the advance returns the elapsed time it reached",
            timer.mode
        );
        runs.push(timer.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn create_returns_the_new_row_added_and_delete_returns_it_removed() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut crud = Held::task(LeanTask::Crud, incremental);
        let mode = crud.mode.clone();

        let name = crud.step(&type_into("crud-name", "Ada"), [&[], &[], &["crud-name"]]);
        assert!(
            entry(&name, "crud-name").is_some_and(|input| {
                input.state.focused && input.state.value.as_deref() == Some("Ada")
            }),
            "{mode}: the typed name is returned as its input's value"
        );
        crud.step(
            &type_into("crud-surname", "Lovelace"),
            [&[], &[], &["crud-name", "crud-surname"]],
        );

        let create = crud.step(
            &click("crud-create"),
            [&["crud-person-3"], &[], &["crud-surname"]],
        );
        assert!(
            entry(&create, "crud-person-3").is_some_and(|row| {
                row.parent.as_deref() == Some("crud-list")
                    && row.name.contains("Ada")
                    && row.name.contains("Lovelace")
            }),
            "{mode}: the new row is returned under the list, named from the typed fields"
        );

        let select = crud.step(
            &click("crud-person-3"),
            [&[], &[], &["crud-update", "crud-delete"]],
        );
        assert!(
            entry(&select, "crud-delete").is_some_and(|delete| delete.state.enabled == Some(true)),
            "{mode}: the selection returns Delete enabled"
        );

        crud.step(
            &click("crud-delete"),
            [&[], &["crud-person-3"], &["crud-update", "crud-delete"]],
        );
        assert!(crud.diffs.len() == 5, "{mode}: five steps");
        runs.push(crud.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_click_on_an_element_that_reacts_to_nothing_returns_settled_with_an_empty_diff() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut diffs = Vec::new();
        for task in LeanTask::ALL {
            let mut held = Held::task(task, incremental);
            let idle = held.step(&click("task-title"), [&[], &[], &[]]);
            assert!(
                idle.diff.is_empty(),
                "{}: a click on the heading returns an empty diff",
                held.mode
            );
            diffs.extend(held.diffs);
        }
        assert!(
            diffs.len() == 4,
            "incremental={incremental}: one step per lean task"
        );
        runs.push(diffs);
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
fn a_password_typed_by_the_driver_is_returned_only_as_its_mask() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: fixture");
        let mut session = Session::start("password", || {
            Harness::from_vdom(
                VirtualDom::new(password_fixture),
                stand::options(incremental),
            )
        })
        .expect("the label is one of the closed set");
        assert!(
            session
                .harness()
                .doc
                .snapshot()
                .get("fx-secret")
                .is_some_and(|input| input.state.value.as_deref() == Some("")),
            "{mode}: the password input is empty at boot"
        );

        let outcome = session.run(&type_into("fx-secret", SECRET));
        assert!(
            editor_text(session.harness(), "fx-secret") == SECRET,
            "{mode}: the password input holds the text the driver typed"
        );
        let returned = format!("{outcome:?}");
        assert!(
            returned.contains(MASKED_VALUE),
            "{mode}: what the call returned, written out whole, holds the mask"
        );
        assert!(
            returned.matches(SECRET).count() == 0,
            "{mode}: what the call returned, written out whole, holds none of the typed text"
        );

        let Ok(Outcome::Acted { settled, diff, .. }) = outcome else {
            panic!("{mode}: the typing call ran");
        };
        assert!(settled, "{mode}: the typing call returns settled");
        let changed: Vec<&str> = diff.changed.iter().map(|entry| &*entry.id).collect();
        assert!(
            changed == ["fx-secret"] && diff.added.is_empty() && diff.removed.is_empty(),
            "{mode}: the typing call names the password input alone"
        );
        assert!(
            diff.changed[0].state.value.as_deref() == Some(MASKED_VALUE),
            "{mode}: the entry's value reads the mask"
        );
        runs.push(vec![diff]);
    }
    assert_modes_agree(&runs);
}
