//! A driver session starts in a named fixture state of its app, and its screen reads as that
//! state does when reached by hand from boot. In both layout modes. *For each of the stand's
//! three states* — CRUD's `fourteen-people` and `person-selected`, the flight booker's
//! `return-flight` — a session started in it by `stand::start_in` reads otherwise than a fresh
//! boot, then answers `snapshot`, `lint`, `actions` and `graph` with the lines a session of the
//! same task answers once this check's own by-hand calls, stated here apart from the stand's
//! table, have driven it there from boot. *The next call* on CRUD's `fourteen-people` is taken as
//! the by-hand session takes it. *A name no state holds* is refused `UnknownState` with nothing
//! run, the session reading as a fresh boot. *A step the screen refuses* stops the reach, naming
//! its index and cause, the steps before it standing. *A fixture page* held in a state through the
//! shared fixtures' `hold_in` reads as the same page clicked by hand. *A recorded session* that
//! reaches a state keeps the reach's steps as ordinary steps, and its kept check replays from boot
//! holding. A failure message names the layout mode, a task or page and an index only.

use escher_driver::{Call, Cause, Fixture, FixtureStep, ReachError, Session};
use seven_guis::stand::{self, LeanTask, StartInError};

mod fixtures;
mod session_common;
use session_common::{actions, click, graph, hold, lint, scroll, slug, snapshot, type_into};

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// One call of a by-hand path from boot.
#[derive(Clone, Copy)]
enum Hand {
    Click(&'static str),
    Type(&'static str, &'static str),
}

impl Hand {
    fn call(self) -> Call {
        match self {
            Hand::Click(id) => click(id),
            Hand::Type(id, text) => type_into(id, text),
        }
    }
}

/// Each stand state: its task, its name and the calls that reach it by hand from boot, as the
/// stand checks reach it — twelve Creates (`cli_commands`), a row selected and the return flight
/// chosen (`ENABLED_TAB_ORDER`'s two enabled states).
const BY_HAND: [(LeanTask, &str, &[Hand]); 3] = [
    (
        LeanTask::Crud,
        "fourteen-people",
        &[Hand::Click("crud-create"); 12],
    ),
    (
        LeanTask::Crud,
        "person-selected",
        &[Hand::Click("crud-person-0")],
    ),
    (
        LeanTask::FlightBooker,
        "return-flight",
        &[Hand::Type("flight-type", "r")],
    ),
];

/// The answer line `call` reads on `session`, as the driver writes it.
fn line(session: &mut Session, call: &Call) -> String {
    match session.run(call) {
        Ok(outcome) => outcome.to_json(),
        Err(refusal) => refusal.to_json(),
    }
}

/// A session of `task` at boot, driven by `calls`, each accepted.
fn by_hand(task: LeanTask, calls: &[Hand], incremental: bool) -> Session {
    let (mut session, _) = hold(task, incremental);
    for (index, step) in calls.iter().enumerate() {
        assert!(
            session.run(&step.call()).is_ok(),
            "{}: {}: by-hand call {index} is accepted",
            mode_name(incremental),
            slug(task)
        );
    }
    session
}

fn started_in(task: LeanTask, name: &str, incremental: bool) -> Session {
    match stand::start_in(slug(task), task, name, incremental) {
        Ok(session) => session,
        Err(_) => panic!(
            "{}: {}: the state is reached",
            mode_name(incremental),
            slug(task)
        ),
    }
}

#[test]
fn every_named_state_reads_as_its_by_hand_path_from_boot() {
    assert_eq!(BY_HAND.len(), 3);
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (index, (task, name, calls)) in BY_HAND.into_iter().enumerate() {
            let task_name = slug(task);
            let mut started = started_in(task, name, incremental);
            let (mut boot, _) = hold(task, incremental);
            assert!(
                line(&mut started, &snapshot()) != line(&mut boot, &snapshot()),
                "{mode}: {task_name}: row {index} reads otherwise than boot"
            );
            let mut reached = by_hand(task, calls, incremental);
            for (verb, call) in [snapshot(), lint(), actions(), graph()]
                .into_iter()
                .enumerate()
            {
                let (ours, theirs) = (started.run(&call), reached.run(&call));
                assert!(
                    escher_driver::answer_digest(&ours) == escher_driver::answer_digest(&theirs),
                    "{mode}: {task_name}: row {index}, verb {verb} answers the by-hand digest"
                );
                assert!(
                    ours == theirs,
                    "{mode}: {task_name}: row {index}, verb {verb} answers the by-hand line"
                );
            }
        }
    }
}

#[test]
fn a_session_started_in_a_state_takes_the_next_call_as_the_by_hand_one_does() {
    let (task, name, calls) = BY_HAND[0];
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut started = started_in(task, name, incremental);
        let mut reached = by_hand(task, calls, incremental);
        for (index, session) in [&mut started, &mut reached].into_iter().enumerate() {
            assert!(
                session
                    .run(&click("crud-person-14"))
                    .err()
                    .map(|r| r.cause())
                    == Some(Cause::OffScreen),
                "{mode}: crud: session {index}'s last row is off-screen"
            );
        }
        for (index, call) in [scroll("crud-person-14"), click("crud-person-14")]
            .into_iter()
            .enumerate()
        {
            assert!(
                line(&mut started, &call) == line(&mut reached, &call),
                "{mode}: crud: next call {index} answers the by-hand line"
            );
        }
        assert!(
            line(&mut started, &snapshot()) == line(&mut reached, &snapshot()),
            "{mode}: crud: the screens read the same after the next calls"
        );
    }
}

#[test]
fn a_name_no_state_holds_is_refused_and_the_session_reads_as_boot() {
    let over_long = "x".repeat(33);
    let names = ["", "absent", over_long.as_str(), "FOURTEEN-PEOPLE"];
    assert_eq!(names.len(), 4);
    for (index, name) in names.into_iter().enumerate() {
        assert!(
            Fixture::named(stand::fixtures(LeanTask::Crud), name) == Err(ReachError::UnknownState),
            "crud: name {index} is refused"
        );
    }
    assert!(stand::fixtures(LeanTask::Counter).is_empty());
    for (index, (_, name, _)) in BY_HAND.into_iter().enumerate() {
        assert!(
            Fixture::named(stand::fixtures(LeanTask::Counter), name)
                == Err(ReachError::UnknownState),
            "counter: name {index} is refused"
        );
    }
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (index, name) in names.into_iter().enumerate() {
            assert!(
                matches!(
                    stand::start_in("crud", LeanTask::Crud, name, incremental),
                    Err(StartInError::Reach(ReachError::UnknownState))
                ),
                "{mode}: crud: start in name {index} is refused"
            );
        }
        assert!(
            matches!(
                stand::start_in("counter", LeanTask::Counter, "fourteen-people", incremental),
                Err(StartInError::Reach(ReachError::UnknownState))
            ),
            "{mode}: counter: a start in a state is refused"
        );
        let (mut asked, _) = hold(LeanTask::Crud, incremental);
        let refused = Fixture::named(stand::fixtures(LeanTask::Crud), "absent");
        assert!(refused.is_err(), "{mode}: crud: the name is refused");
        let (mut boot, _) = hold(LeanTask::Crud, incremental);
        assert!(
            line(&mut asked, &snapshot()) == line(&mut boot, &snapshot()),
            "{mode}: crud: the session reads as boot"
        );
    }
}

/// A fixture whose second step clicks CRUD's Delete, disabled until a row is selected.
const DELETE_AT_BOOT: [FixtureStep; 3] = [
    FixtureStep::Click("crud-create"),
    FixtureStep::Click("crud-delete"),
    FixtureStep::Click("crud-create"),
];

#[test]
fn a_step_the_screen_refuses_stops_the_reach_naming_its_index_and_cause() {
    let fixture = Fixture::new("delete-at-boot", &DELETE_AT_BOOT);
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let (mut session, _) = hold(LeanTask::Crud, incremental);
        assert!(
            session.run(&click("crud-delete")).err().map(|r| r.cause()) == Some(Cause::Disabled),
            "{mode}: crud: Delete is disabled at boot"
        );
        let (mut session, _) = hold(LeanTask::Crud, incremental);
        assert!(
            session.reach(&fixture)
                == Err(ReachError::Refused {
                    step: 1,
                    cause: Cause::Disabled,
                }),
            "{mode}: crud: the reach stops at step 1, disabled"
        );
        let mut one_create = by_hand(LeanTask::Crud, &[Hand::Click("crud-create")], incremental);
        assert!(
            line(&mut session, &snapshot()) == line(&mut one_create, &snapshot()),
            "{mode}: crud: step 0's effect stands and step 2 never ran"
        );
    }
}

/// Three clicks of the shared page's first item.
const THREE_CLICKS: [FixtureStep; 3] = [FixtureStep::Click("item-001"); 3];

#[test]
fn a_fixture_page_held_in_a_state_reads_as_its_clicks_from_boot() {
    let fixture = Fixture::new("three-clicks", &THREE_CLICKS);
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let mut held = fixtures::hold_in("items", fixtures::many_items, &fixture, incremental);
        assert!(
            held.harness().text_content("#clicks") == "3",
            "{mode}: items: the state counts three clicks"
        );
        let mut by_hand = fixtures::hold("items", fixtures::many_items, incremental);
        assert!(
            by_hand.harness().text_content("#clicks") == "0",
            "{mode}: items: the page counts none at boot"
        );
        for index in 0..3 {
            assert!(
                by_hand.run(&click("item-001")).is_ok(),
                "{mode}: items: by-hand click {index} is accepted"
            );
        }
        assert!(
            line(&mut held, &snapshot()) == line(&mut by_hand, &snapshot()),
            "{mode}: items: the held state reads as the clicks by hand"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_recorded_session_started_in_a_state_replays_from_boot() {
    use escher_driver::Check;
    use std::fs;

    let fixture = Fixture::named(stand::fixtures(LeanTask::Crud), "fourteen-people")
        .expect("the stand names the state");
    let reach_steps = fixture.steps().len();
    assert_eq!(reach_steps, 12);
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        let dir = session_common::state_dir(&format!("start-in-rec-{mode}"));
        let _ = fs::remove_dir_all(&dir);
        let (session, _) = hold(LeanTask::Crud, incremental);
        let mut session = session.recording_into(&dir).expect("the recording starts");
        assert!(
            session.reach(fixture).is_ok(),
            "{mode}: crud: the state is reached"
        );
        assert!(
            session.run(&snapshot()).is_ok(),
            "{mode}: crud: the snapshot is accepted"
        );
        drop(session);
        let bytes = fs::read(dir.join("check-1.json")).expect("the check is kept");
        let check = Check::from_bytes(&bytes).expect("the check reads");
        assert!(check.app() == "crud", "{mode}: crud: opens on the task");
        assert!(
            check.steps().len() == reach_steps + 1,
            "{mode}: crud: the reach's steps and the snapshot are kept"
        );
        for (index, step) in check.steps().iter().enumerate() {
            let verb = if index < reach_steps {
                "click"
            } else {
                "snapshot"
            };
            assert!(
                step.call().verb == verb && step.accepted(),
                "{mode}: crud: step {index} is kept as an ordinary step"
            );
        }
        let (mut fresh, _) = hold(LeanTask::Crud, incremental);
        let replay = check.replay(&mut fresh);
        assert!(replay.holds(), "{mode}: crud: the check holds from boot");
        assert!(
            replay.ran() == reach_steps + 1,
            "{mode}: crud: every step ran"
        );
        fs::remove_dir_all(&dir).expect("the recording directory is removed");
    }
}
