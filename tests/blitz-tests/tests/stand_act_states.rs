//! The step that changes what a control states of itself returns that control in its diff, as
//! it reads after the step. On the stand's flight booker and CRUD, through the driver, in both
//! layout modes: a `type` choosing the other flight type returns the select with the value it
//! chose and both of its options with their selected readings swapped, a click on a row returns
//! that row selected and the row it leaves not selected, and a text typed into the start date
//! returns that field invalid when the text is no date and not invalid when it is one. Each returned diff equals `Snapshot::diff` over this
//! file's own snapshots taken either side of the call, and the two layout modes return equal
//! diffs. A diff is content: this file prints none, and a failure message carries the layout
//! mode, the task and a step index, never an id or what a screen reads.

use dioxus_native_dom::{NodeState, SnapshotDiff};
use escher_driver::{Call, Session};
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{Acted, act, click, hold, type_into};

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

    /// Runs `call` through the driver between this file's own two snapshots and asserts it
    /// returns settled with the diff of those snapshots.
    #[track_caller]
    fn step(&mut self, call: &Call) -> Acted {
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
        self.diffs.push(acted.diff.clone());
        acted
    }
}

/// The state a diff's changed nodes read for `id`.
fn changed<'d>(acted: &'d Acted, id: &str) -> Option<&'d NodeState> {
    acted
        .diff
        .changed
        .iter()
        .find(|entry| entry.id == id)
        .map(|entry| &entry.state)
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
fn a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut flight = Held::task(LeanTask::FlightBooker, incremental);
        let selected = |acted: &Acted, id: &str| changed(acted, id).map(|state| state.selected);
        let value =
            |acted: &Acted| changed(acted, "flight-type").and_then(|state| state.value.clone());

        let chosen = flight.step(&type_into("flight-type", "return"));
        assert!(
            value(&chosen).as_deref() == Some("return"),
            "{}: step 0 returns the select reading the type chosen",
            flight.mode
        );
        assert!(
            selected(&chosen, "flight-type-one-way") == Some(Some(false))
                && selected(&chosen, "flight-type-return") == Some(Some(true)),
            "{}: step 0 returns the option left not selected and the option chosen selected",
            flight.mode
        );
        let back = flight.step(&type_into("flight-type", "one"));
        assert!(
            value(&back).as_deref() == Some("one-way"),
            "{}: step 1 returns the select reading the type chosen back",
            flight.mode
        );
        assert!(
            selected(&back, "flight-type-one-way") == Some(Some(true))
                && selected(&back, "flight-type-return") == Some(Some(false)),
            "{}: step 1 returns the two readings swapped back",
            flight.mode
        );
        runs.push(flight.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut crud = Held::task(LeanTask::Crud, incremental);
        let selected = |acted: &Acted, id: &str| changed(acted, id).map(|state| state.selected);

        let first = crud.step(&click("crud-person-0"));
        assert!(
            selected(&first, "crud-person-0") == Some(Some(true)),
            "{}: step 0 returns the clicked row selected",
            crud.mode
        );
        assert!(
            selected(&first, "crud-person-1").is_none()
                && selected(&first, "crud-person-2").is_none(),
            "{}: step 0 returns no row whose reading stayed",
            crud.mode
        );
        let second = crud.step(&click("crud-person-2"));
        assert!(
            selected(&second, "crud-person-0") == Some(Some(false))
                && selected(&second, "crud-person-2") == Some(Some(true)),
            "{}: step 1 returns the row left not selected and the row clicked selected",
            crud.mode
        );
        runs.push(crud.diffs);
    }
    assert_modes_agree(&runs);
}

#[test]
fn a_typed_start_date_returns_the_field_invalid_or_not() {
    let mut runs = Vec::new();
    for incremental in [false, true] {
        let mut flight = Held::task(LeanTask::FlightBooker, incremental);
        let invalid = |acted: &Acted| changed(acted, "flight-start").map(|state| state.invalid);

        let broken = flight.step(&type_into("flight-start", "next week"));
        assert!(
            invalid(&broken) == Some(true),
            "{}: step 0 returns the field invalid",
            flight.mode
        );
        let mended = flight.step(&type_into("flight-start", "24.12.2026"));
        assert!(
            invalid(&mended) == Some(false),
            "{}: step 1 returns the field not invalid",
            flight.mode
        );
        runs.push(flight.diffs);
    }
    assert_modes_agree(&runs);
}
