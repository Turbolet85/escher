//! Handler coverage on the stand: `escher_driver::handler_coverage` replays kept checks, each on a
//! fresh session of its task, and lists the handlers the UI graph showed that no replay reached.
//! In both layout modes, each check recorded in process through `Session::recording_into` into an
//! owner-only directory under the target's temporary directory, read back by `Check::from_bytes`
//! and removed. *A check of one `snapshot`* lists exactly `HANDLERS_AT_BOOT` on each of the six
//! served tasks — the handlers `DioxusDocument::graph` reads on a fresh boot, `mounted` left out.
//! *A check of a task's `GRAPH_DRIVES`* lists exactly its `UNEXERCISED_AFTER_DRIVES` row. *Two
//! Counter checks* list only what neither exercised. *The load page*, which registers no state,
//! reads its button's click exercised. *A `click` refused `disabled`* exercises nothing, and
//! *`enter` on a Tab-focused button* exercises the click it dispatches. *Cells' editor*, left by
//! Enter, reads its `keydown` exercised though the step removes it. *A typed sentinel* never
//! reaches the line, and *a check of another app* contributes nothing and does not hold. A
//! failure message names the layout mode, the task and a row index, never an id or a reading.

#[cfg(unix)]
use std::fs;

#[cfg(unix)]
use blitz_test_harness::Harness;
#[cfg(unix)]
use dioxus::prelude::*;
#[cfg(unix)]
use dioxus_native_dom::DioxusDocument;
#[cfg(unix)]
use escher_driver::{Call, Check, Coverage, Session, handler_coverage};
#[cfg(unix)]
use seven_guis::stand::{self, LeanTask};
#[cfg(unix)]
use seven_guis::tasks::timer::TimerTicks;

mod common;
mod session_common;
#[cfg(unix)]
use common::{
    CELLS_CLICKED_BY_DRIVES, CELLS_TAB_ORDER, Drive, GRAPH_DRIVES, HANDLERS_AT_BOOT,
    UNEXERCISED_AFTER_DRIVES, grid_cells, tab_order,
};
#[cfg(unix)]
use session_common::{advance, click, hold, press, slug, snapshot, state_dir, type_into};

/// The text typed into CRUD's name field: synthetic, and no part of any id or name.
#[cfg(unix)]
const SENTINEL: &str = "coverage-sentinel-Kp3";

#[cfg(unix)]
fn mode_name(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

#[cfg(unix)]
fn app_root() -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    seven_guis::app::app()
}

/// Home booted, then the task behind `card` opened by a click on its card.
#[cfg(unix)]
fn open(card: &str, incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = Harness::from_vdom(VirtualDom::new(app_root), stand::options(incremental));
    let card_node = harness.node(&format!("#{card}"));
    harness.scroll_into_view(card_node);
    harness.click(&format!("#{card}"));
    harness
}

/// A fresh session on the task `task` names, labelled `task`: the lean four through the stand's
/// own hold, Cells and the load page as the binary serves them, the other two opened from Home.
#[cfg(unix)]
fn boot(task: &str, incremental: bool) -> Session {
    if let Some(lean) = LeanTask::ALL.into_iter().find(|lean| slug(*lean) == task) {
        return hold(lean, incremental).0;
    }
    let harness = match task {
        "cells" => stand::boot_cells(stand::options(incremental)),
        "load" => stand::boot_load(stand::options(incremental)),
        "temp-converter" => open("task-card-temp-converter", incremental),
        "circle-drawer" => open("task-card-circle-drawer", incremental),
        _ => panic!("a stand task"),
    };
    Session::start(task, || harness).expect("a stand task's name is a label")
}

/// The session label of the stand task titled `title`.
#[cfg(unix)]
fn title_slug(title: &str) -> &'static str {
    match title {
        "Counter" => "counter",
        "Flight Booker" => "flight-booker",
        "Timer" => "timer",
        "CRUD" => "crud",
        "Temperature Converter" => "temp-converter",
        "Circle Drawer" => "circle-drawer",
        "Cells" => "cells",
        _ => panic!("a stand task's title"),
    }
}

#[cfg(unix)]
fn call_of(drive: Drive) -> Call {
    match drive {
        Drive::Click(id) => click(id),
        Drive::Type(id, text) => type_into(id, text),
        Drive::Press(key) => press(key, false),
        Drive::Advance(ms) => advance(ms),
    }
}

/// `calls` run on a fresh session of `task`, recorded into `name`'s directory and read back as
/// the check it keeps; the directory is removed.
#[cfg(unix)]
#[track_caller]
fn record(task: &str, incremental: bool, calls: &[Call], name: &str) -> Check {
    let at = format!("{} {task} {name}", mode_name(incremental));
    let dir = state_dir(&format!("cov-{name}-{}", mode_name(incremental)));
    let _ = fs::remove_dir_all(&dir);
    {
        let mut session = boot(task, incremental)
            .recording_into(&dir)
            .unwrap_or_else(|_| panic!("{at}: the recording starts"));
        for call in calls {
            let _ = session.run(call);
        }
        assert!(!session.recording_failed(), "{at}: the recording stands");
    }
    let bytes = fs::read(dir.join("check-1.json")).unwrap_or_else(|_| panic!("{at}: one check"));
    let _ = fs::remove_dir_all(&dir);
    Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"))
}

/// What `checks` leave unexercised on fresh sessions of `task`.
#[cfg(unix)]
#[track_caller]
fn coverage(task: &str, incremental: bool, checks: &[Check]) -> Coverage {
    handler_coverage(checks, || Ok(boot(task, incremental)))
        .unwrap_or_else(|_| panic!("{} {task}: the boots start", mode_name(incremental)))
}

/// A coverage's unexercised handlers as (id, event).
#[cfg(unix)]
fn pairs(coverage: &Coverage) -> Vec<(String, String)> {
    coverage
        .unexercised
        .iter()
        .map(|handler| (handler.id.clone(), handler.event.clone()))
        .collect()
}

#[cfg(unix)]
fn owned(rows: &[(&str, &str)]) -> Vec<(String, String)> {
    rows.iter()
        .map(|(id, event)| (id.to_string(), event.to_string()))
        .collect()
}

/// `HANDLERS_AT_BOOT`'s row for `task`, Cells' grid cells after its stated rows.
#[cfg(unix)]
fn handlers_at_boot(task: &str) -> Vec<(String, String)> {
    let (_, rows) = HANDLERS_AT_BOOT
        .iter()
        .find(|(served, _)| *served == task)
        .expect("a served task's row");
    let mut handlers = owned(rows);
    if task == "cells" {
        handlers.extend(grid_cells().into_iter().map(|id| (id, "click".to_string())));
    }
    handlers
}

/// `UNEXERCISED_AFTER_DRIVES`'s row for `title`, Cells' unclicked grid cells after its rows.
#[cfg(unix)]
fn unexercised_after_drives(title: &str) -> Vec<(String, String)> {
    let (_, rows) = UNEXERCISED_AFTER_DRIVES
        .iter()
        .find(|(task, _)| *task == title)
        .expect("a driven task's row");
    let mut handlers = owned(rows);
    if title == "Cells" {
        handlers.extend(
            grid_cells()
                .into_iter()
                .filter(|id| !CELLS_CLICKED_BY_DRIVES.contains(&id.as_str()))
                .map(|id| (id, "click".to_string())),
        );
    }
    handlers
}

/// The handlers the graph of a fresh boot of `task` shows, `mounted` left out, as (id, event).
#[cfg(unix)]
fn graph_at_boot(task: &str, incremental: bool) -> Vec<(String, String)> {
    boot(task, incremental)
        .harness()
        .doc
        .graph()
        .nodes
        .into_iter()
        .flat_map(|node| {
            node.handlers
                .into_iter()
                .filter(|handler| handler.event != "mounted")
                .map(move |handler| (node.id.clone(), handler.event))
        })
        .collect()
}

/// `n` Tab presses, bringing focus to the `n`-th stop of a task's sequence.
#[cfg(unix)]
fn tabs(n: usize) -> Vec<Call> {
    (0..n).map(|_| press("tab", false)).collect()
}

#[cfg(unix)]
#[test]
fn a_snapshot_check_lists_every_handler_the_boot_screen_shows() {
    for incremental in [false, true] {
        for (row, (task, _)) in HANDLERS_AT_BOOT.iter().enumerate() {
            let at = format!("{} {task} row {row}", mode_name(incremental));
            let expected = handlers_at_boot(task);
            assert!(
                graph_at_boot(task, incremental) == expected,
                "{at}: the table is the graph's reading at boot"
            );
            let check = record(task, incremental, &[snapshot()], "boot");
            let read = coverage(task, incremental, &[check]);
            assert!(pairs(&read) == expected, "{at}: every handler unexercised");
            assert!((read.checks, read.held) == (1, 1), "{at}: one check, held");
        }
    }
}

#[cfg(unix)]
#[test]
fn a_check_of_each_tasks_drives_lists_its_unexercised_row() {
    for incremental in [false, true] {
        for (row, (title, drives)) in GRAPH_DRIVES.iter().enumerate() {
            let task = title_slug(title);
            let at = format!("{} {task} row {row}", mode_name(incremental));
            let calls: Vec<Call> = drives.iter().map(|drive| call_of(*drive)).collect();
            let check = record(task, incremental, &calls, "drives");
            for (index, step) in check.steps().iter().enumerate() {
                assert!(step.accepted(), "{at}: step {index} accepted");
            }
            let read = coverage(task, incremental, &[check]);
            assert!(
                pairs(&read) == unexercised_after_drives(title),
                "{at}: the stated row"
            );
            assert!((read.checks, read.held) == (1, 1), "{at}: one check, held");
        }
    }
}

#[cfg(unix)]
#[test]
fn two_checks_list_only_what_neither_exercised() {
    for incremental in [false, true] {
        let at = format!("{} counter", mode_name(incremental));
        let looked = record("counter", incremental, &[snapshot()], "looked");
        let clicked = record(
            "counter",
            incremental,
            &[click("counter-increment")],
            "clicked",
        );
        let read = coverage("counter", incremental, &[looked, clicked]);
        assert!(
            pairs(&read) == owned(&[("back-btn", "click")]),
            "{at}: the back button alone"
        );
        assert!((read.checks, read.held) == (2, 2), "{at}: two checks, held");
    }
}

#[cfg(unix)]
#[test]
fn the_load_page_registers_no_state_and_reads_its_click_exercised() {
    for incremental in [false, true] {
        let at = format!("{} load", mode_name(incremental));
        assert!(
            boot("load", incremental)
                .harness()
                .doc
                .graph()
                .states
                .is_empty(),
            "{at}: no registered state"
        );
        let check = record("load", incremental, &[click("load-fetch")], "fetched");
        assert!(check.steps()[0].accepted(), "{at}: the click is accepted");
        let read = coverage("load", incremental, &[check]);
        assert!(
            pairs(&read) == owned(&[("back-btn", "click")]),
            "{at}: the button's click exercised"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_refused_step_exercises_nothing_and_a_keyboard_activation_its_click() {
    for incremental in [false, true] {
        let at = format!("{} flight-booker", mode_name(incremental));
        let check = record(
            "flight-booker",
            incremental,
            &[click("flight-return-date")],
            "refused",
        );
        assert!(!check.steps()[0].accepted(), "{at}: the click is refused");
        let read = coverage("flight-booker", incremental, &[check]);
        assert!(
            pairs(&read) == handlers_at_boot("flight-booker"),
            "{at}: nothing exercised"
        );

        let at = format!("{} counter", mode_name(incremental));
        let stop = tab_order(LeanTask::Counter)
            .iter()
            .position(|id| *id == "counter-increment")
            .expect("the button is a Tab stop");
        let mut calls = tabs(stop + 1);
        calls.push(press("enter", false));
        let check = record("counter", incremental, &calls, "entered");
        let read = coverage("counter", incremental, &[check]);
        assert!(
            pairs(&read) == owned(&[("back-btn", "click")]),
            "{at}: enter exercised the button's click"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_handler_whose_step_removes_its_element_reads_exercised() {
    for incremental in [false, true] {
        let at = format!("{} cells", mode_name(incremental));
        let grid = CELLS_TAB_ORDER
            .iter()
            .position(|id| *id == "cells-grid")
            .expect("the grid is a Tab stop");
        let mut calls = tabs(grid + 1);
        calls.push(press("enter", false));
        calls.push(press("enter", false));

        let shows_editor = |session: &Session| {
            session
                .harness()
                .doc
                .graph()
                .nodes
                .iter()
                .any(|node| node.id == "cell-input")
        };
        let mut session = boot("cells", incremental);
        for (index, call) in calls.iter().enumerate() {
            assert!(session.run(call).is_ok(), "{at}: step {index} accepted");
            let open = index == calls.len() - 2;
            assert!(
                shows_editor(&session) == open,
                "{at}: step {index}'s editor"
            );
        }

        let check = record("cells", incremental, &calls, "editor");
        let read = coverage("cells", incremental, &[check]);
        let unexercised = pairs(&read);
        let editor = |event: &str| ("cell-input".to_string(), event.to_string());
        assert!(
            unexercised.contains(&editor("input")),
            "{at}: the editor was seen"
        );
        assert!(
            !unexercised.contains(&editor("keydown")),
            "{at}: its keydown exercised"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_typed_sentinel_never_reaches_the_coverage_line() {
    for incremental in [false, true] {
        let at = format!("{} crud", mode_name(incremental));
        let check = record(
            "crud",
            incremental,
            &[type_into("crud-name", SENTINEL), snapshot()],
            "typed",
        );
        let line = coverage("crud", incremental, &[check]).to_json();
        assert!(
            line.matches(SENTINEL).count() == 0,
            "{at}: no typed text in the line"
        );
        assert!(line.contains("\"unexercised\":["), "{at}: the line reads");
    }
}

#[cfg(unix)]
#[test]
fn a_check_of_another_app_contributes_nothing_and_does_not_hold() {
    for incremental in [false, true] {
        let at = format!("{} counter", mode_name(incremental));
        let other = record("crud", incremental, &[click("crud-person-0")], "other");
        let read = coverage("counter", incremental, std::slice::from_ref(&other));
        assert!(pairs(&read).is_empty(), "{at}: nothing contributed");
        assert!(
            (read.checks, read.held) == (1, 0),
            "{at}: one check, not held"
        );

        let own = record("counter", incremental, &[snapshot()], "own");
        let read = coverage("counter", incremental, &[other, own]);
        assert!(
            pairs(&read) == handlers_at_boot("counter"),
            "{at}: the Counter's check alone"
        );
        assert!(
            (read.checks, read.held) == (2, 1),
            "{at}: two checks, one held"
        );
    }
}
