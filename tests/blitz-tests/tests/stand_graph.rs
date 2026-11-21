//! The UI graph on the stand, in both layout modes: the `graph` verb through `Session::run`
//! answers what `DioxusDocument::graph` reads in process. On Counter at boot `counter-increment`
//! reads the component `Counter` and one `click` handler whose write of `count` is declared,
//! unconfirmed and unflagged, and `counter-value` reads `count`; after a `click` the same write
//! reads confirmed, and the button reads `clickable` while the value does not. On every stand task, once the steps of the shared table have run each of
//! its handlers that writes a registered state, no write reads a flag: the stand's declarations
//! agree with its code. CRUD's list carries `keydown`; the flight booker's type select owns its
//! two options; Cells' 676 data cells carry `click` and read `clickable` while its grid carries
//! `keydown` and does not. Every graph node is a snapshot node and the other way, and no id on a
//! served screen holds the reserved `///`. A graph read changes nothing, its answer holds no text
//! typed into a control, a `graph` with an argument it does not take is refused `malformed` with
//! nothing run, and Cells' graph, over the snapshot budget, is read in parts: each within the
//! budget, part 1 alone holding the states, the parts' nodes in order the whole graph's, a part
//! past the last empty and a part outside 1 to 1000 refused, while a small screen answers whole.
//! A failure message names a layout mode, a task and an index only.

use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, SNAPSHOT_TEXT_BUDGET, UiGraph, VIRTUAL_KEY_SEPARATOR};
use escher_driver::{ArgValue, Call, Cause, Outcome, Session};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::timer::TimerTicks;

use blitz_test_harness::Harness;

mod common;
mod session_common;
use common::{Drive, GRAPH_DRIVES};
use session_common::{
    act, advance, click, graph, graph_of_part, graph_part, graphed, hold, press, refused_unchanged,
    type_into,
};

/// The answer bound of every carrier, in bytes.
const ANSWER_BOUND: usize = 1_048_576;

/// The text typed into CRUD's name field: synthetic, and no part of any id or name.
const SENTINEL: &str = "graph-sentinel-Qv8";

fn app_root() -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    seven_guis::app::app()
}

/// Home booted, then the task behind `card` opened by a click on its card.
fn open(card: &str, incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = Harness::from_vdom(VirtualDom::new(app_root), stand::options(incremental));
    let card_node = harness.node(&format!("#{card}"));
    harness.scroll_into_view(card_node);
    harness.click(&format!("#{card}"));
    harness
}

/// A session on the stand task titled `title`: the lean four through the stand's own hold, the
/// other three opened as the stand serves them.
fn hold_task(title: &str, incremental: bool) -> Session {
    let lean = LeanTask::ALL
        .into_iter()
        .find(|task| task_title(*task) == title);
    if let Some(task) = lean {
        return hold(task, incremental).0;
    }
    let harness = match title {
        "Temperature Converter" => open("task-card-temp-converter", incremental),
        "Circle Drawer" => open("task-card-circle-drawer", incremental),
        "Cells" => stand::boot_cells(stand::options(incremental)),
        "Order" => stand::boot_order(stand::options(incremental)),
        _ => panic!("a stand task's title"),
    };
    Session::start("stand-task", || harness).expect("a label")
}

fn task_title(task: LeanTask) -> &'static str {
    match task {
        LeanTask::Counter => "Counter",
        LeanTask::FlightBooker => "Flight Booker",
        LeanTask::Timer => "Timer",
        LeanTask::Crud => "CRUD",
    }
}

fn call_of(drive: Drive) -> Call {
    match drive {
        Drive::Click(id) => click(id),
        Drive::Type(id, text) => type_into(id, text),
        Drive::Press(key) => press(key, false),
        Drive::Advance(ms) => advance(ms),
    }
}

/// The graph the session answers, asserted equal to the one its document reads in process.
#[track_caller]
fn read(session: &mut Session, at: &str) -> UiGraph {
    let answered = graphed(session, &graph()).unwrap_or_else(|| panic!("{at}: graph answers"));
    assert!(
        answered == session.harness().doc.graph(),
        "{at}: the verb answers the in-process graph"
    );
    answered
}

#[track_caller]
fn node<'g>(graph: &'g UiGraph, id: &str, at: &str) -> &'g dioxus_native_dom::GraphNode {
    graph
        .nodes
        .iter()
        .find(|node| node.id == id)
        .unwrap_or_else(|| panic!("{at}: a named node is in the graph"))
}

#[test]
fn counter_reads_its_declared_write_at_boot_and_confirmed_after_a_click() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} Counter");
        let (mut session, _) = hold(LeanTask::Counter, incremental);
        let boot = read(&mut session, &at);
        let increment = node(&boot, "counter-increment", &at);
        assert!(
            increment.component.as_deref() == Some("Counter") && increment.owner.is_none(),
            "{at}: the button's component"
        );
        assert!(
            increment.handlers.len() == 1 && increment.handlers[0].event == "click",
            "{at}: one click handler"
        );
        let write = &increment.handlers[0].writes;
        assert!(
            write.len() == 1
                && write[0].state == "count"
                && write[0].declared
                && !write[0].confirmed
                && write[0].flag.is_none(),
            "{at}: the declared write at boot"
        );
        assert!(
            node(&boot, "counter-value", &at).reads == ["count"],
            "{at}: the value reads the count"
        );
        let snapshot = session.harness().doc.snapshot();
        assert!(
            snapshot
                .get("counter-increment")
                .is_some_and(|read| read.state.clickable)
                && snapshot
                    .get("counter-value")
                    .is_some_and(|read| !read.state.clickable),
            "{at}: the button reacts to a click and the value does not"
        );
        let count = boot.states.iter().find(|state| state.name == "count");
        assert!(
            count.is_some_and(|count| count.component.as_deref() == Some("Counter")
                && count.readers == ["Counter"]
                && count.writers.is_empty()),
            "{at}: the state's component, readers and no writer before a run"
        );

        assert!(
            act(&mut session, &click("counter-increment")).is_some(),
            "{at}: the click runs"
        );
        let after = read(&mut session, &at);
        let write = &node(&after, "counter-increment", &at).handlers[0].writes[0];
        assert!(
            write.declared && write.confirmed && write.flag.is_none(),
            "{at}: the write confirmed after the click"
        );
        let count = after.states.iter().find(|state| state.name == "count");
        assert!(
            count.is_some_and(|count| count.declared.len() == 1
                && count.writers.len() == 1
                && count.writers[0].id == "counter-increment"
                && count.writers[0].confirmed),
            "{at}: the state's one writer once a run confirms it, its declaration kept"
        );
    }
}

#[test]
fn every_stand_tasks_declarations_agree_with_its_code_once_its_handlers_have_run() {
    for incremental in [false, true] {
        for (title, drives) in GRAPH_DRIVES {
            let mut session = hold_task(title, incremental);
            for (index, drive) in drives.iter().enumerate() {
                let ran = session.run(&call_of(*drive));
                assert!(
                    ran.is_ok(),
                    "incremental={incremental} {title}: step {index} runs"
                );
            }
            let at = format!("incremental={incremental} {title}");
            let graph = read(&mut session, &at);
            let writes: Vec<_> = graph
                .nodes
                .iter()
                .flat_map(|node| &node.handlers)
                .flat_map(|handler| &handler.writes)
                .collect();
            assert_no_flag(&graph, &at);
            assert!(
                writes.iter().any(|write| write.confirmed),
                "{at}: the steps confirmed a write"
            );
            assert!(!graph.states.is_empty(), "{at}: the task registers states");
        }
    }
}

#[test]
fn the_served_screens_read_their_handlers_owners_and_clicks() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} CRUD");
        let (mut crud, _) = hold(LeanTask::Crud, incremental);
        let graph = read(&mut crud, &at);
        let list = node(&graph, "crud-list", &at);
        assert!(
            list.handlers
                .iter()
                .any(|handler| handler.event == "keydown"),
            "{at}: the list carries its key handler"
        );

        let at = format!("incremental={incremental} Flight Booker");
        let (mut flight, _) = hold(LeanTask::FlightBooker, incremental);
        let graph = read(&mut flight, &at);
        for (index, option) in ["flight-type-one-way", "flight-type-return"]
            .into_iter()
            .enumerate()
        {
            assert!(
                node(&graph, option, &at).owner.as_deref() == Some("flight-type"),
                "{at}: option {index} reads its select as owner"
            );
        }
        assert!(
            node(&graph, "flight-type", &at).owner.is_none(),
            "{at}: the select is its own owner"
        );

        let at = format!("incremental={incremental} Cells");
        let mut cells = hold_task("Cells", incremental);
        let graph = read(&mut cells, &at);
        let snapshot = cells.harness().doc.snapshot();
        let data_cells: Vec<_> = graph
            .nodes
            .iter()
            .filter(|node| node.id.starts_with("cell-") && node.id != "cell-input")
            .collect();
        assert!(data_cells.len() == 676, "{at}: the 676 data cells");
        for (index, cell) in data_cells.iter().enumerate() {
            assert!(
                cell.handlers.iter().any(|handler| handler.event == "click"),
                "{at}: cell {index} carries a click handler"
            );
            assert!(
                snapshot
                    .get(&cell.id)
                    .is_some_and(|read| read.state.clickable),
                "{at}: cell {index} reads clickable"
            );
        }
        let grid = node(&graph, "cells-grid", &at);
        assert!(
            grid.handlers
                .iter()
                .any(|handler| handler.event == "keydown"),
            "{at}: the grid carries its key handler"
        );
        assert!(
            snapshot
                .get("cells-grid")
                .is_some_and(|read| !read.state.clickable),
            "{at}: the grid reads not clickable"
        );
    }
}

#[test]
fn every_graph_node_is_a_snapshot_node_and_no_served_id_holds_the_reserved_form() {
    for incremental in [false, true] {
        let titles = GRAPH_DRIVES.map(|(title, _)| title);
        let mut sessions: Vec<(String, Session)> = titles
            .iter()
            .map(|title| (title.to_string(), hold_task(title, incremental)))
            .collect();
        let load = stand::boot_load(stand::options(incremental));
        sessions.push((
            "load".to_string(),
            Session::start("load", || load).expect("a label"),
        ));
        let order = stand::boot_order(stand::options(incremental));
        sessions.push((
            "order".to_string(),
            Session::start("order", || order).expect("a label"),
        ));
        for (title, session) in &mut sessions {
            let at = format!("incremental={incremental} {title}");
            let graph = read(session, &at);
            let graph_ids: Vec<&str> = graph.nodes.iter().map(|node| node.id.as_str()).collect();
            let snapshot = session.harness().doc.snapshot();
            let snapshot_ids: Vec<&str> = snapshot.nodes().map(|node| node.id.as_str()).collect();
            assert!(
                graph_ids == snapshot_ids,
                "{at}: the graph's nodes are the snapshot's, in its order"
            );
            assert_readers_agree(&graph, &at);
            assert_no_reserved_separator(session, &at);
        }
    }
}

/// No write of a handler and no writer of a state reads a flag.
#[track_caller]
fn assert_no_flag(graph: &UiGraph, at: &str) {
    let writes = graph
        .nodes
        .iter()
        .flat_map(|node| &node.handlers)
        .flat_map(|handler| &handler.writes);
    for (index, write) in writes.enumerate() {
        assert!(write.flag.is_none(), "{at}: write {index} reads no flag");
    }
    for (index, state) in graph.states.iter().enumerate() {
        for (writer, read) in state.writers.iter().enumerate() {
            assert!(
                read.flag.is_none(),
                "{at}: state {index} writer {writer} reads no flag"
            );
        }
    }
}

/// A state names its reading components: a node reading it is a node of one, and each one
/// named has a node reading it.
#[track_caller]
fn assert_readers_agree(graph: &UiGraph, at: &str) {
    for (index, state) in graph.states.iter().enumerate() {
        let reading = graph
            .nodes
            .iter()
            .filter(|node| node.reads.contains(&state.name));
        for node in reading {
            if let Some(component) = &node.component {
                assert!(
                    state.readers.contains(component),
                    "{at}: state {index} names each reading node's component"
                );
            }
        }
        for (reader, component) in state.readers.iter().enumerate() {
            assert!(
                graph.nodes.iter().any(|node| {
                    node.component.as_ref() == Some(component) && node.reads.contains(&state.name)
                }),
                "{at}: state {index} reader {reader} has a node reading it"
            );
        }
    }
}

/// No id on the session's screen holds the reserved separator.
#[track_caller]
fn assert_no_reserved_separator(session: &Session, at: &str) {
    let ids = session.harness().doc.element_ids();
    for (index, (_, id)) in ids.iter().enumerate() {
        assert!(
            !id.contains(VIRTUAL_KEY_SEPARATOR),
            "{at}: id {index} holds no reserved separator"
        );
    }
}

#[test]
fn a_graph_read_changes_nothing_and_holds_no_typed_text() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} CRUD");
        let (mut crud, _) = hold(LeanTask::Crud, incremental);
        assert!(
            act(&mut crud, &type_into("crud-name", SENTINEL)).is_some(),
            "{at}: the text is typed"
        );
        crud.harness_mut().base_mut().take_changed_nodes();
        let before = crud.harness().doc.snapshot();
        let answer = crud.run(&graph()).map(|outcome| outcome.to_json());
        let Ok(answer) = answer else {
            panic!("{at}: graph answers");
        };
        assert!(
            crud.harness().doc.snapshot() == before && !crud.harness().base().has_changes(),
            "{at}: the read changed nothing"
        );
        assert!(
            answer.matches(SENTINEL).count() == 0,
            "{at}: the answer holds no typed text"
        );
        assert!(
            answer.contains("\"first_field\"") && answer.contains("\"Crud\""),
            "{at}: the answer names the state and the component"
        );

        let with_argument = Call {
            verb: "graph".to_string(),
            args: vec![("id".to_string(), ArgValue::Text("crud-name".to_string()))],
        };
        let (cause, unchanged) = refused_unchanged(&mut crud, &with_argument);
        assert!(
            cause == Some(Cause::Malformed) && unchanged,
            "{at}: a graph with an argument it does not take is refused malformed with nothing run"
        );
    }
}

#[test]
fn cells_reads_in_parts_each_within_the_budget_and_together_the_whole_graph() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} Cells");
        let mut cells = hold_task("Cells", incremental);
        let whole = cells.harness().doc.graph();
        let whole_size = Outcome::Graphed {
            graph: whole.clone(),
        }
        .to_json()
        .len();
        assert!(
            whole_size > SNAPSHOT_TEXT_BUDGET && whole.states.len() == 6,
            "{at}: the whole graph is over the budget and registers six states"
        );

        let first = cells
            .run(&graph())
            .unwrap_or_else(|_| panic!("{at}: graph answers"));
        let (first_graph, part, parts) = graph_of_part(first.clone());
        assert!(
            part == 1 && parts > 1,
            "{at}: with no part asked the answer is part 1 of more than one"
        );
        assert!(
            first_graph.states == whole.states,
            "{at}: part 1 holds every state"
        );

        let mut nodes = Vec::new();
        for n in 1..=parts {
            let outcome = cells
                .run(&graph_part(i64::from(n)))
                .unwrap_or_else(|_| panic!("{at}: part {n} answers"));
            let written = outcome.to_json();
            assert!(
                written.len() <= SNAPSHOT_TEXT_BUDGET && written.len() <= ANSWER_BOUND,
                "{at}: part {n} is within the budget"
            );
            let (graph, read_part, read_parts) = graph_of_part(outcome);
            assert!(
                (read_part, read_parts) == (n, parts),
                "{at}: part {n} reads its number and the count"
            );
            assert!(
                (n == 1) == !graph.states.is_empty(),
                "{at}: part {n} holds states only as part 1"
            );
            assert!(!graph.nodes.is_empty(), "{at}: part {n} holds nodes");
            nodes.extend(graph.nodes);
        }
        assert!(
            nodes == whole.nodes,
            "{at}: the parts' nodes in order are the whole graph's"
        );

        let past = cells
            .run(&graph_part(i64::from(parts) + 1))
            .unwrap_or_else(|_| panic!("{at}: a part past the last answers"));
        let (past_graph, past_part, past_parts) = graph_of_part(past);
        assert!(
            past_graph.nodes.is_empty()
                && past_graph.states.is_empty()
                && (past_part, past_parts) == (parts + 1, parts),
            "{at}: a part past the last holds nothing"
        );

        for (row, n) in [0, 1001].into_iter().enumerate() {
            let (cause, unchanged) = refused_unchanged(&mut cells, &graph_part(n));
            assert!(
                cause == Some(Cause::Malformed) && unchanged,
                "{at}: row {row}: a part outside its bound is refused malformed with nothing run"
            );
        }

        let at = format!("incremental={incremental} Counter");
        let (mut counter, _) = hold(LeanTask::Counter, incremental);
        let answer = counter
            .run(&graph())
            .unwrap_or_else(|_| panic!("{at}: graph answers"));
        assert!(
            matches!(answer, Outcome::Graphed { .. }),
            "{at}: a graph within the budget with no part asked answers whole"
        );
        let written = answer.to_json();
        assert!(
            written.starts_with("{\"nodes\":[") && !written.contains("\"part\""),
            "{at}: the whole answer holds no part or count"
        );
    }
}
