//! A breadth-first exploration of a held app: `escher_driver::explore` drives every action the
//! `actions` reader offers, each path on a fresh session through `Session::run`, within a step
//! budget. In both layout modes, on the stand's bundled font. *The six served tasks* are each
//! explored within a budget of 40, every path's calls a `click`, a `type` or a `scroll`, the same
//! paths on a second run. *The Counter*, recorded into an owner-only directory, leaves one kept
//! check per path, each of format 1, holding no `screenshot` step and replaying holding. *A
//! refusing control* is met and its path reported by its cause and not explored further, and the
//! page's finite screens are explored whole. *A panicking handler* is caught, its path reported
//! `Panicked`, and the exploration goes on; recorded, its check holds the steps before it and
//! replays holding. *A group-open recording directory* is refused before anything boots, with
//! nothing written. *Cells' budget* bounds every acting call. A failure message names the layout
//! mode, a task or page and an index only.

use std::cell::Cell;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::path::Path;

use dioxus::prelude::*;
use escher_driver::{ArgValue, Call, Cause, Exploration, PathEnding, Session, StepBudget, explore};
#[cfg(unix)]
use escher_driver::{CHECK_FORMAT, Check, SessionError};
use seven_guis::stand::{self, LeanTask};

mod fixtures;
mod session_common;
#[cfg(unix)]
use session_common::state_dir;
use session_common::{hold, slug, snapshot};

fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

/// The six tasks the stand's binary serves.
const SERVED: [&str; 6] = ["counter", "flight-booker", "timer", "crud", "cells", "load"];

/// A fresh session on the served task `task`, labelled `task`, booted as the binary boots it.
fn boot(task: &str, incremental: bool) -> Session {
    if let Some(lean) = LeanTask::ALL.into_iter().find(|lean| slug(*lean) == task) {
        return hold(lean, incremental).0;
    }
    let harness = match task {
        "cells" => stand::boot_cells(stand::options(incremental)),
        "load" => stand::boot_load(stand::options(incremental)),
        _ => panic!("a served task"),
    };
    Session::start(task, || harness).expect("a served task's name is a label")
}

fn budget(steps: u32) -> StepBudget {
    StepBudget::new(steps).expect("a budget within its bounds")
}

/// The id a path's call names.
fn target(call: &Call) -> Option<&str> {
    call.args.iter().find_map(|(name, value)| match value {
        ArgValue::Text(id) if name == "id" => Some(id.as_str()),
        _ => None,
    })
}

/// The index of the first path of one call, `verb`, naming `id`.
fn first_path(exploration: &Exploration, verb: &str, id: &str) -> Option<usize> {
    exploration.paths.iter().position(|path| {
        matches!(path.calls.as_slice(), [call] if call.verb == verb && target(call) == Some(id))
    })
}

/// Whether some path runs the calls of `prefix`, then more.
fn extended(exploration: &Exploration, prefix: &[Call]) -> bool {
    exploration
        .paths
        .iter()
        .any(|path| path.calls.len() > prefix.len() && path.calls.starts_with(prefix))
}

/// A disabled button beside one that switches a flag: a refusal to meet and a page of few
/// screens.
fn refusing() -> Element {
    let mut on = use_signal(|| false);
    rsx! {
        div { id: "fx-root",
            button { id: "fx-disabled", disabled: true, "Locked" }
            button { id: "fx-switch", onclick: move |_| on.toggle(), "Switch" }
            output { id: "fx-state", if on() { "On" } else { "Off" } }
        }
    }
}

/// The panicking button's handler: a unit function, so the handler is no diverging closure.
fn fail() {
    panic!("a planted handler panic");
}

/// A button whose click handler panics, then a button that counts its clicks.
fn panicking() -> Element {
    let mut clicks = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            button {
                id: "fx-panic",
                onclick: move |_| fail(),
                "Fail"
            }
            button { id: "fx-count", onclick: move |_| clicks += 1, "Count" }
            output { id: "fx-clicks", "{clicks}" }
        }
    }
}

#[test]
fn every_served_task_is_explored_within_its_budget() {
    assert_eq!(SERVED.len(), 6);
    for incremental in [false, true] {
        let mode = mode_name(incremental);
        for (index, task) in SERVED.into_iter().enumerate() {
            let run = || {
                explore(|| Ok(boot(task, incremental)), budget(40), None)
                    .unwrap_or_else(|_| panic!("{mode}: {task}: row {index} explores"))
            };
            let first = run();
            assert!(
                first.steps <= 40,
                "{mode}: {task}: row {index} within budget"
            );
            assert!(
                !first.paths.is_empty(),
                "{mode}: {task}: row {index} has paths"
            );
            for (path, explored) in first.paths.iter().enumerate() {
                assert!(
                    explored
                        .calls
                        .iter()
                        .all(|call| ["click", "type", "scroll"].contains(&call.verb.as_str())),
                    "{mode}: {task}: row {index}, path {path} names an explored verb"
                );
            }
            assert!(
                run() == first,
                "{mode}: {task}: row {index} explores the same paths again"
            );
        }
    }
}

#[cfg(unix)]
#[track_caller]
fn record_dir(name: &str) -> std::path::PathBuf {
    let dir = state_dir(name);
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The kept check `check-{n}.json` of `dir`, read.
#[cfg(unix)]
fn kept(dir: &Path, n: usize) -> Option<Check> {
    let bytes = fs::read(dir.join(format!("check-{n}.json"))).ok()?;
    Check::from_bytes(&bytes).ok()
}

/// Each path's kept check holds its answered calls, opens with format 1, holds no `screenshot`
/// and replays holding on a fresh session; the directory holds one check per path.
#[cfg(unix)]
#[track_caller]
fn checks_hold(
    exploration: &Exploration,
    dir: &Path,
    at: &str,
    mut fresh: impl FnMut() -> Session,
) {
    let files = fs::read_dir(dir).map(Iterator::count).unwrap_or(0);
    assert!(files == exploration.paths.len(), "{at}: one check per path");
    for (index, path) in exploration.paths.iter().enumerate() {
        let check = kept(dir, index + 1).unwrap_or_else(|| panic!("{at}: path {index} reads"));
        let opening =
            fs::read_to_string(dir.join(format!("check-{}.json", index + 1))).unwrap_or_default();
        assert!(
            opening.contains(&format!("\"format\":{CHECK_FORMAT}}}")),
            "{at}: path {index} states format 1"
        );
        let answered = match path.ending {
            PathEnding::Panicked => path.calls.len() - 1,
            _ => path.calls.len(),
        };
        let calls: Vec<&Call> = check.steps().iter().map(|step| step.call()).collect();
        assert!(
            calls == path.calls[..answered].iter().collect::<Vec<_>>(),
            "{at}: path {index} keeps its answered calls"
        );
        assert!(
            calls.iter().all(|call| call.verb != "screenshot"),
            "{at}: path {index} holds no screenshot"
        );
        assert!(
            check.replay(&mut fresh()).holds(),
            "{at}: path {index} replays holding"
        );
    }
}

#[cfg(unix)]
#[test]
fn the_counter_is_explored_and_each_path_replays_green() {
    for incremental in [false, true] {
        let at = format!("{}: counter", mode_name(incremental));
        let dir = record_dir(&format!("explore-counter-{}", mode_name(incremental)));
        let exploration = explore(|| Ok(boot("counter", incremental)), budget(200), Some(&dir))
            .unwrap_or_else(|_| panic!("{at}: explores"));
        // Every click on the button shows a count no screen showed before: the Counter has no
        // last screen, so its budget ends the run.
        assert!(!exploration.exhausted, "{at}: the budget ends the run");
        assert!(exploration.steps <= 200, "{at}: within budget");
        let increment = first_path(&exploration, "click", "counter-increment")
            .unwrap_or_else(|| panic!("{at}: the increment is clicked"));
        assert!(
            exploration.paths[increment].ending == PathEnding::Accepted { settled: true },
            "{at}: path {increment} is accepted"
        );
        assert!(
            fs::metadata(&dir)
                .map(|meta| meta.permissions().mode() & 0o777)
                .ok()
                == Some(0o700),
            "{at}: the directory is owner-only"
        );
        checks_hold(&exploration, &dir, &at, || boot("counter", incremental));
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn a_refusing_control_is_met_and_reported_by_its_cause() {
    for incremental in [false, true] {
        let at = format!("{}: refusing", mode_name(incremental));
        let exploration = explore(
            || Ok(fixtures::hold("refusing", refusing, incremental)),
            budget(2000),
            None,
        )
        .unwrap_or_else(|_| panic!("{at}: explores"));
        let locked = first_path(&exploration, "click", "fx-disabled")
            .unwrap_or_else(|| panic!("{at}: the disabled button is clicked"));
        let path = &exploration.paths[locked];
        assert!(
            path.ending == PathEnding::Refused(Cause::Disabled),
            "{at}: path {locked} is refused disabled"
        );
        assert!(
            !extended(&exploration, &path.calls),
            "{at}: path {locked} is not explored further"
        );
        let switch = first_path(&exploration, "click", "fx-switch")
            .unwrap_or_else(|| panic!("{at}: the switch is clicked"));
        assert!(
            matches!(
                exploration.paths[switch].ending,
                PathEnding::Accepted { .. }
            ),
            "{at}: path {switch} is accepted"
        );
        assert!(
            extended(&exploration, &exploration.paths[switch].calls),
            "{at}: path {switch} is explored further"
        );
        // The flag and the focus give the page few screens: the run explores them whole.
        assert!(exploration.exhausted, "{at}: every path is run");
        assert!(exploration.steps <= 2000, "{at}: within budget");
    }
}

#[test]
fn a_panicking_handler_is_caught_and_the_exploration_goes_on() {
    for incremental in [false, true] {
        let at = format!("{}: panicking", mode_name(incremental));
        let mut listed = fixtures::hold("panicking", panicking, incremental);
        let click_accepted = listed.actions().iter().any(|node| {
            node.id == "fx-panic"
                && node
                    .verbs
                    .iter()
                    .any(|reading| reading.verb == "click" && reading.cause.is_none())
        });
        assert!(click_accepted, "{at}: the panicking button takes a click");
        drop(listed);
        let exploration = explore(
            || Ok(fixtures::hold("panicking", panicking, incremental)),
            budget(30),
            None,
        )
        .unwrap_or_else(|_| panic!("{at}: explores"));
        let failed = first_path(&exploration, "click", "fx-panic")
            .unwrap_or_else(|| panic!("{at}: the panicking button is clicked"));
        assert!(
            exploration.paths[failed].ending == PathEnding::Panicked,
            "{at}: path {failed} panicked"
        );
        assert!(
            !extended(&exploration, &exploration.paths[failed].calls),
            "{at}: path {failed} is not explored further"
        );
        let counted = first_path(&exploration, "click", "fx-count")
            .unwrap_or_else(|| panic!("{at}: the counting button is clicked"));
        assert!(
            counted > failed,
            "{at}: path {counted} runs after the panic"
        );
        assert!(
            exploration.paths[counted].ending == PathEnding::Accepted { settled: true },
            "{at}: path {counted} is accepted"
        );
        let mut after = fixtures::hold("panicking", panicking, incremental);
        assert!(
            after.run(&snapshot()).is_ok(),
            "{at}: a session booted after the panic answers"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_panicking_paths_kept_check_holds_the_steps_before_it_and_replays_green() {
    for incremental in [false, true] {
        let at = format!("{}: panicking", mode_name(incremental));
        let dir = record_dir(&format!("explore-panic-{}", mode_name(incremental)));
        let exploration = explore(
            || Ok(fixtures::hold("panicking", panicking, incremental)),
            budget(30),
            Some(&dir),
        )
        .unwrap_or_else(|_| panic!("{at}: explores"));
        let failed = first_path(&exploration, "click", "fx-panic")
            .unwrap_or_else(|| panic!("{at}: the panicking button is clicked"));
        let check = kept(&dir, failed + 1).unwrap_or_else(|| panic!("{at}: path {failed} reads"));
        assert!(
            check.steps().is_empty(),
            "{at}: path {failed} keeps no step of the panicking call"
        );
        let panicked = exploration
            .paths
            .iter()
            .filter(|path| path.ending == PathEnding::Panicked)
            .count();
        assert!(panicked > 0, "{at}: a path panicked");
        checks_hold(&exploration, &dir, &at, || {
            fixtures::hold("panicking", panicking, incremental)
        });
        let _ = fs::remove_dir_all(&dir);
    }
}

#[cfg(unix)]
#[test]
fn a_group_open_recording_directory_is_refused_with_nothing_written() {
    for incremental in [false, true] {
        let at = format!("{}: counter", mode_name(incremental));
        let dir = record_dir(&format!("explore-open-{}", mode_name(incremental)));
        fs::create_dir(&dir).unwrap_or_else(|_| panic!("{at}: the directory is made"));
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o750))
            .unwrap_or_else(|_| panic!("{at}: the directory is opened"));
        let boots = Cell::new(0);
        let explored = explore(
            || {
                boots.set(boots.get() + 1);
                Ok(boot("counter", incremental))
            },
            budget(10),
            Some(&dir),
        );
        assert!(
            matches!(explored, Err(SessionError::RecordDirNotPrivate)),
            "{at}: the directory is refused"
        );
        assert!(boots.get() == 0, "{at}: nothing boots");
        assert!(
            fs::read_dir(&dir).map(Iterator::count).ok() == Some(0),
            "{at}: nothing is written"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn the_budget_bounds_every_acting_call() {
    for incremental in [false, true] {
        let at = format!("{}: cells", mode_name(incremental));
        let boots = Cell::new(0);
        let exploration = explore(
            || {
                boots.set(boots.get() + 1);
                Ok(boot("cells", incremental))
            },
            budget(25),
            None,
        )
        .unwrap_or_else(|_| panic!("{at}: explores"));
        let run: usize = exploration.paths.iter().map(|path| path.calls.len()).sum();
        assert!(
            usize::try_from(exploration.steps).ok() == Some(run),
            "{at}: the steps are the calls run"
        );
        assert!(exploration.steps <= 25, "{at}: within budget");
        assert!(!exploration.exhausted, "{at}: the budget ends the run");
        assert!(
            boots.get() == exploration.paths.len() + 1,
            "{at}: one boot per path and one for the first screen"
        );
    }
}
