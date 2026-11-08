//! What can be done on the screen now: the `actions` verb, run through `Session::run`, answers
//! what `Session::actions` reads, in both layout modes. On each of the six served screens at
//! boot it lists every node a snapshot reads `clickable` or that is focusable, and every node an
//! owner serves, each a node of the UI graph, no id holding the reserved `///`, the count per
//! task the stated one. For every listed node and verb on the four lean tasks at boot, and on a
//! stated subset of Cells and of the load page with its fetch in flight, the listed acceptance or
//! cause is what the real call returns: a refused call on the one held session, its instance
//! unchanged, an accepted `screenshot` there too, and every other accepted call on a fresh boot.
//! The stated cases: the flight booker's two options refused `covered` with owner `flight-type`
//! until its list opens, its return date refused `disabled` while one-way, the last CRUD row
//! after twelve Creates refused `off-screen` until a `scroll` and the scrollbar's fade; the
//! covered fixture's button refused `covered`; an unkeyed button read `keyed` false and named by
//! `unkeyed_actionable`; every `screenshot` refused `fonts-unavailable` on a boot with no font.
//! The read changes nothing, holds no typed text and no mask, is refused `malformed` with an
//! argument, and the whole Cells answer is within the answer bound. A failure message names a
//! layout mode, a task, an index and a verb word only.

use std::collections::HashMap;

use blitz_test_harness::HarnessOptions;
use dioxus::prelude::*;
use dioxus_native_dom::{MASKED_VALUE, VIRTUAL_KEY_SEPARATOR};
use escher_driver::{ActionNode, ArgValue, Busy, Call, Cause, Outcome, Session};
use seven_guis::stand::{self, LeanTask};

mod common;
mod fixtures;
mod session_common;
use common::ACTIONABLE_AT_BOOT;
use session_common::{
    act, actioned, actions, advance, click, focused, hold, refused_unchanged, screenshot_of,
    scroll, slug, type_into,
};

/// The answer bound of every carrier, in bytes.
const ANSWER_BOUND: usize = 1_048_576;

/// The text typed into a control: synthetic, and no part of any id or name.
const SENTINEL: &str = "actions-sentinel-Rk4";

/// How many Creates put the last CRUD row's centre past the list's box.
const CREATES: usize = 12;

/// The last CRUD row after [`CREATES`] Creates.
const LAST_ROW: &str = "crud-person-14";

/// Time enough for the overlay scrollbars a scroll shows to fade, in milliseconds.
const FADE_MS: i64 = 5000;

/// The six screens the stand serves, by the label each session is started with.
const SERVED: [&str; 6] = ["counter", "flight-booker", "timer", "crud", "cells", "load"];

fn lean(label: &str) -> Option<LeanTask> {
    LeanTask::ALL.into_iter().find(|task| slug(*task) == label)
}

/// A session on the served screen `label` names, booted through the stand.
fn hold_served(label: &str, incremental: bool) -> Session {
    if let Some(task) = lean(label) {
        return hold(task, incremental).0;
    }
    let harness = match label {
        "cells" => stand::boot_cells(stand::options(incremental)),
        "load" => stand::boot_load(stand::options(incremental)),
        _ => panic!("a served screen's label"),
    };
    Session::start(label, || harness).expect("the label is one of the closed set")
}

/// The load page with its fetch in flight: its button clicked through the harness, which waits
/// on nothing.
fn load_in_flight(incremental: bool) -> Session {
    let mut session = hold_served("load", incremental);
    session.harness_mut().click("#load-fetch");
    session
}

/// The call of `verb` naming `id`, as the read builds it: `type` with the empty text.
fn call_for(verb: &str, id: &str) -> Call {
    match verb {
        "click" => click(id),
        "type" => type_into(id, ""),
        "scroll" => scroll(id),
        "screenshot" => screenshot_of(id),
        _ => panic!("a verb that names an element"),
    }
}

/// The nodes the `actions` verb answers, asserted equal to what `Session::actions` reads.
#[track_caller]
fn read(session: &mut Session, at: &str) -> Vec<ActionNode> {
    let answered = actioned(session, &actions()).unwrap_or_else(|| panic!("{at}: actions answers"));
    assert!(
        answered == session.actions(),
        "{at}: the verb answers what the reader reads"
    );
    answered
}

/// Each reading of `nodes` whose id `pick` takes, compared with the real call: a refused one on
/// `held`, which it leaves unchanged; an accepted `screenshot` on `held`; any other accepted one
/// on a fresh session from `fresh`. Returns how many pairs were compared.
#[track_caller]
fn agree(
    at: &str,
    held: &mut Session,
    fresh: &dyn Fn() -> Session,
    nodes: &[ActionNode],
    pick: &dyn Fn(&str) -> bool,
) -> usize {
    let mut compared = 0;
    for (index, node) in nodes.iter().enumerate() {
        if !pick(&node.id) {
            continue;
        }
        for reading in &node.verbs {
            let verb = reading.verb;
            let call = call_for(verb, &node.id);
            match reading.cause {
                Some(cause) => {
                    let (refused, unchanged) = refused_unchanged(held, &call);
                    assert!(
                        refused == Some(cause) && unchanged,
                        "{at}: node {index} {verb}: the call is refused with the listed cause, nothing run"
                    );
                }
                None if verb == "screenshot" => {
                    assert!(
                        held.run(&call).is_ok(),
                        "{at}: node {index} {verb}: the call is accepted"
                    );
                }
                None => {
                    let mut session = fresh();
                    assert!(
                        session.run(&call).is_ok(),
                        "{at}: node {index} {verb}: the call is accepted"
                    );
                }
            }
            compared += 1;
        }
    }
    compared
}

#[track_caller]
fn listed<'n>(nodes: &'n [ActionNode], id: &str, at: &str) -> &'n ActionNode {
    nodes
        .iter()
        .find(|node| node.id == id)
        .unwrap_or_else(|| panic!("{at}: a stated node is listed"))
}

#[track_caller]
fn cause(node: &ActionNode, verb: &str, at: &str) -> Option<Cause> {
    node.verbs
        .iter()
        .find(|reading| reading.verb == verb)
        .unwrap_or_else(|| panic!("{at}: {verb} is listed"))
        .cause
}

#[test]
fn each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs() {
    assert_eq!(ACTIONABLE_AT_BOOT.len(), SERVED.len());
    for incremental in [false, true] {
        let mut counts = Vec::new();
        for (row, label) in SERVED.into_iter().enumerate() {
            let at = format!("incremental={incremental} task {row}");
            let mut session = hold_served(label, incremental);
            let nodes = read(&mut session, &at);
            counts.push((label, nodes.len()));

            let doc = &session.harness().doc;
            let graph = doc.graph();
            let snapshot = doc.snapshot();
            let elements: HashMap<String, _> = doc
                .element_ids()
                .into_iter()
                .map(|(node, id)| (id, node))
                .collect();
            let is_listed = |id: &str| nodes.iter().any(|node| node.id == id);
            for (index, node) in nodes.iter().enumerate() {
                let in_graph = graph.nodes.iter().find(|graphed| graphed.id == node.id);
                assert!(
                    in_graph.is_some_and(|graphed| graphed.owner == node.owner),
                    "{at}: node {index} is a graph node with its owner"
                );
                assert!(
                    !node.id.contains(VIRTUAL_KEY_SEPARATOR),
                    "{at}: node {index} holds no virtual separator"
                );
                assert!(
                    node.keyed == !node.id.contains('/'),
                    "{at}: node {index} reads keyed by its id"
                );
                let verbs: Vec<&str> = node.verbs.iter().map(|reading| reading.verb).collect();
                assert!(
                    verbs == ["click", "type", "scroll", "screenshot"],
                    "{at}: node {index} lists the table's verbs that name an element"
                );
            }
            let base = session.harness().base();
            for (index, node) in snapshot.nodes().enumerate() {
                let focusable = elements
                    .get(&node.id)
                    .and_then(|element| base.get_node(*element))
                    .is_some_and(|element| element.is_focussable());
                if node.state.clickable || focusable {
                    assert!(is_listed(&node.id), "{at}: snapshot node {index} is listed");
                }
            }
            for (index, node) in graph.nodes.iter().enumerate() {
                if node.owner.is_some() {
                    assert!(is_listed(&node.id), "{at}: graph node {index} is listed");
                }
            }
        }
        let measured: Vec<usize> = counts.iter().map(|(_, count)| *count).collect();
        assert!(
            counts == ACTIONABLE_AT_BOOT,
            "incremental={incremental}: the listed counts are the stated ones, measured {measured:?}"
        );
    }
}

#[test]
fn every_listed_cause_on_the_lean_tasks_is_what_the_call_returns() {
    for incremental in [false, true] {
        let mut disabled = 0;
        for task in LeanTask::ALL {
            let at = format!("incremental={incremental} {}", slug(task));
            let (mut held, _ticks) = hold(task, incremental);
            let nodes = read(&mut held, &at);
            disabled += nodes
                .iter()
                .filter(|node| cause(node, "click", &at) == Some(Cause::Disabled))
                .count();
            let fresh = || hold(task, incremental).0;
            let compared = agree(&at, &mut held, &fresh, &nodes, &|_| true);
            assert!(
                compared == nodes.len() * 4,
                "{at}: every listed pair is compared"
            );
        }
        assert!(
            disabled == 3,
            "incremental={incremental}: {disabled} controls read disabled at boot"
        );
    }
}

#[test]
fn the_stated_cells_and_load_page_causes_are_what_the_call_returns() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} cells");
        let mut cells = hold_served("cells", incremental);
        let nodes = read(&mut cells, &at);
        let column: Vec<&ActionNode> = nodes
            .iter()
            .filter(|node| {
                node.id
                    .strip_prefix("cell-A")
                    .is_some_and(|row| row.parse::<u32>().is_ok())
            })
            .collect();
        let past = column
            .iter()
            .position(|node| cause(node, "click", &at) == Some(Cause::OffScreen))
            .unwrap_or_else(|| panic!("{at}: a column-A cell lies past the viewport"));
        assert!(past >= 2, "{at}: the cells before it are in view");
        let stated = [
            "cells-grid".to_string(),
            column[0].id.clone(),
            column[past - 1].id.clone(),
            column[past].id.clone(),
        ];
        assert!(
            column[0].id == "cell-A0",
            "{at}: the column starts at its first row"
        );
        assert!(
            cause(column[past - 1], "click", &at).is_none(),
            "{at}: the last cell in view takes a click"
        );
        let fresh = || hold_served("cells", incremental);
        let compared = agree(&at, &mut cells, &fresh, &nodes, &|id| {
            stated.iter().any(|stated| stated == id)
        });
        assert!(compared == stated.len() * 4, "{at}: every stated pair");

        let at = format!("incremental={incremental} load");
        let mut load = load_in_flight(incremental);
        assert!(
            load.harness_mut().settle().err().map(|not| not.busy) == Some(Busy::Loads),
            "{at}: the fetch is in flight"
        );
        let nodes = read(&mut load, &at);
        let fresh = || load_in_flight(incremental);
        let compared = agree(&at, &mut load, &fresh, &nodes, &|id| id == "load-fetch");
        assert!(compared == 4, "{at}: every pair of the button");
    }
}

#[test]
fn the_flight_bookers_options_read_covered_until_the_list_opens() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} flight-booker");
        let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
        let nodes = read(&mut session, &at);
        for id in ["flight-type-one-way", "flight-type-return"] {
            let option = listed(&nodes, id, &at);
            assert!(
                option.owner.as_deref() == Some("flight-type"),
                "{at}: an option is served by its select"
            );
            assert!(
                cause(option, "click", &at) == Some(Cause::Covered),
                "{at}: an option of the closed select reads covered"
            );
            let (refused, unchanged) = refused_unchanged(&mut session, &click(id));
            assert!(
                refused == Some(Cause::Covered) && unchanged,
                "{at}: the call is refused covered"
            );
        }
        let date = listed(&nodes, "flight-return-date", &at);
        for verb in ["click", "type"] {
            assert!(
                cause(date, verb, &at) == Some(Cause::Disabled),
                "{at}: the return date reads disabled while one-way for {verb}"
            );
        }

        assert!(
            act(&mut session, &click("flight-type")).is_some(),
            "{at}: the click opens the list"
        );
        assert!(
            session.harness().base().open_select().is_some(),
            "{at}: the list is open"
        );
        let nodes = read(&mut session, &at);
        for id in ["flight-type-one-way", "flight-type-return"] {
            assert!(
                cause(listed(&nodes, id, &at), "click", &at).is_none(),
                "{at}: an option of the open list takes a click"
            );
        }
        assert!(
            session.harness().base().open_select().is_some(),
            "{at}: the read leaves the list open"
        );
        assert!(
            act(&mut session, &click("flight-type-return")).is_some(),
            "{at}: the call is accepted"
        );
    }
}

#[test]
fn the_last_crud_row_reads_off_screen_until_a_scroll() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        for _ in 0..CREATES {
            assert!(
                act(&mut session, &click("crud-create")).is_some(),
                "{at}: a Create is accepted"
            );
        }
        let nodes = read(&mut session, &at);
        let row = listed(&nodes, LAST_ROW, &at);
        assert!(
            cause(row, "click", &at) == Some(Cause::OffScreen),
            "{at}: the last row reads off-screen"
        );
        assert!(
            cause(row, "scroll", &at).is_none(),
            "{at}: the last row takes a scroll"
        );
        let (refused, unchanged) = refused_unchanged(&mut session, &click(LAST_ROW));
        assert!(
            refused == Some(Cause::OffScreen) && unchanged,
            "{at}: the call is refused off-screen"
        );

        assert!(
            act(&mut session, &scroll(LAST_ROW))
                .is_some_and(|scrolled| scrolled.in_view == Some(true)),
            "{at}: the scroll brings the row into view"
        );
        assert!(
            act(&mut session, &advance(FADE_MS)).is_some(),
            "{at}: the scrollbar fades"
        );
        let nodes = read(&mut session, &at);
        assert!(
            cause(listed(&nodes, LAST_ROW, &at), "click", &at).is_none(),
            "{at}: the row in view takes a click"
        );
        assert!(
            act(&mut session, &click(LAST_ROW)).is_some(),
            "{at}: the call is accepted"
        );
    }
}

/// One unkeyed button beside a keyed one.
fn unkeyed_page() -> Element {
    rsx! {
        div { id: "fx-root",
            button { "Unkeyed" }
            button { id: "fx-keyed", "Keyed" }
        }
    }
}

#[test]
fn the_fixtures_read_covered_unkeyed_and_fonts_unavailable() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} covered");
        let mut session = fixtures::hold("covered", fixtures::covered, incremental);
        let nodes = read(&mut session, &at);
        let under = listed(&nodes, "fx-under", &at);
        assert!(
            cause(under, "click", &at) == Some(Cause::Covered),
            "{at}: the button under the overlay reads covered"
        );
        let fresh = || fixtures::hold("covered", fixtures::covered, incremental);
        assert!(
            agree(&at, &mut session, &fresh, &nodes, &|id| id == "fx-under") == 4,
            "{at}: every pair of the button"
        );

        let at = format!("incremental={incremental} unkeyed");
        let mut session = fixtures::hold("unkeyed", unkeyed_page, incremental);
        let nodes = read(&mut session, &at);
        let unkeyed: Vec<&str> = nodes
            .iter()
            .filter(|node| !node.keyed)
            .map(|node| node.id.as_str())
            .collect();
        let reported: Vec<String> = session
            .harness()
            .doc
            .unkeyed_actionable()
            .into_iter()
            .map(|entry| entry.id)
            .collect();
        assert!(
            unkeyed.len() == 1 && reported == unkeyed,
            "{at}: the one unkeyed button reads keyed false and is the one reported"
        );
        assert!(
            listed(&nodes, "fx-keyed", &at).keyed,
            "{at}: the keyed button reads keyed"
        );

        let at = format!("incremental={incremental} no font");
        let boot = || {
            let harness = stand::boot(
                LeanTask::Crud,
                HarnessOptions {
                    font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
                    ..stand::options(incremental)
                },
            );
            Session::start(slug(LeanTask::Crud), || harness).expect("a lean task's slug")
        };
        let mut session = boot();
        let nodes = read(&mut session, &at);
        assert!(!nodes.is_empty(), "{at}: nodes are listed");
        for (index, node) in nodes.iter().enumerate() {
            assert!(
                cause(node, "screenshot", &at) == Some(Cause::FontsUnavailable),
                "{at}: node {index} reads fonts-unavailable for a screenshot"
            );
        }
        let pairs = agree(&at, &mut session, &boot, &nodes, &|_| true);
        assert!(pairs == nodes.len() * 4, "{at}: every pair");
    }
}

#[test]
fn the_read_changes_nothing_and_holds_no_typed_text_or_mask() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            act(&mut crud, &type_into("crud-name", SENTINEL)).is_some(),
            "{at}: the text is typed"
        );
        crud.harness_mut().base_mut().take_changed_nodes();
        let before = (crud.harness().doc.snapshot(), focused(&crud));
        let answer = crud
            .run(&actions())
            .map(|outcome| outcome.to_json())
            .unwrap_or_else(|_| panic!("{at}: actions answers"));
        let after = (crud.harness().doc.snapshot(), focused(&crud));
        assert!(
            !crud.harness().base().has_changes(),
            "{at}: the read marks no change"
        );
        assert!(
            before == after && before.0.diff(&after.0).is_empty(),
            "{at}: the snapshot, the focus readings and the diff are unchanged"
        );
        assert!(
            answer.matches(SENTINEL).count() == 0,
            "{at}: the answer holds no typed text"
        );

        let with_argument = Call {
            verb: "actions".to_string(),
            args: vec![("id".to_string(), ArgValue::Text("crud-name".to_string()))],
        };
        let (refused, unchanged) = refused_unchanged(&mut crud, &with_argument);
        assert!(
            refused == Some(Cause::Malformed) && unchanged,
            "{at}: an argument is refused malformed with nothing run"
        );

        let at = format!("incremental={incremental} password");
        let mut session = fixtures::hold("password", fixtures::password_labelled, incremental);
        assert!(
            act(&mut session, &type_into("fx-secret", SENTINEL)).is_some(),
            "{at}: the secret is typed"
        );
        let answer = session
            .run(&actions())
            .map(|outcome| outcome.to_json())
            .unwrap_or_else(|_| panic!("{at}: actions answers"));
        assert!(
            answer.contains("\"fx-secret\""),
            "{at}: the password input is listed"
        );
        assert!(
            answer.matches(SENTINEL).count() == 0 && answer.matches(MASKED_VALUE).count() == 0,
            "{at}: the answer holds neither the secret nor the mask"
        );
    }
}

#[test]
fn the_whole_cells_answer_is_within_the_answer_bound() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} cells");
        let mut cells = hold_served("cells", incremental);
        let Ok(outcome @ Outcome::Actions { .. }) = cells.run(&actions()) else {
            panic!("{at}: actions answers");
        };
        let size = outcome.to_json().len();
        assert!(
            size > 100_000 && size <= ANSWER_BOUND,
            "{at}: the answer of {size} bytes is within the bound"
        );
    }
}
