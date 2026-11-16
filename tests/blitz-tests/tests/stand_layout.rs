//! Why an element sits where it does at its size, on the stand: `Session::layout`, in process
//! only, read in both layout modes. On each of the six served screens at boot every snapshot node
//! answers, its `bounds` the snapshot's own, and every node with no transform shift sits at its
//! container's bounds plus its offset plus the container's scroll shift, to 1/64 px. Pinned
//! readings: the Cells cell `cell-A0` placed by `Grid` in `cells-grid`, its height `Style` and its
//! width `Grid`; a CRUD row placed by `Flex` in `crud-list`, its width `Stretch`; and after twelve
//! Creates and a `scroll` of the last row, that row moved by a `Scroll` shift held by `crud-list`.
//! The read changes nothing — no change pending, the snapshot, both focus readings and the clock
//! unmoved, an open select list still open, a delivered Timer tick still unapplied — refuses an
//! id no screen read `not-found` and a deleted row's id `stale`, and writes no line under an
//! `escher_` target at TRACE. A failure message names a layout mode, a task and an index only.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use dioxus_native_dom::{NodeId, PlacedBy, ShiftBy, SizeCause, SnapshotNode};
use escher_driver::{Cause, LayoutExplanation, Placer, Session};
use seven_guis::stand::{self, LeanTask};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

mod session_common;
use session_common::{act, click, focused, hold, scroll, slug};

/// The six screens the stand serves, by the label each session is started with.
const SERVED: [&str; 6] = ["counter", "flight-booker", "timer", "crud", "cells", "load"];

/// How many Creates put the last CRUD row past the list's box.
const CREATES: usize = 12;

/// The readings this check pins, each an id, the placing word, the container's id and the
/// causes of width and height (`None` where not pinned).
struct Pinned {
    task: &'static str,
    id: &'static str,
    by: PlacedBy,
    container: &'static str,
    width: Option<SizeCause>,
    height: Option<SizeCause>,
}

const PINNED: [Pinned; 2] = [
    Pinned {
        task: "cells",
        id: "cell-A0",
        by: PlacedBy::Grid,
        container: "cells-grid",
        width: Some(SizeCause::Grid),
        height: Some(SizeCause::Style),
    },
    Pinned {
        task: "crud",
        id: "crud-person-0",
        by: PlacedBy::Flex,
        container: "crud-list",
        width: Some(SizeCause::Stretch),
        height: None,
    },
];

/// The last CRUD row after [`CREATES`] Creates.
const LAST_ROW: &str = "crud-person-14";

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

#[track_caller]
fn read(session: &mut Session, id: &str, at: &str) -> LayoutExplanation {
    session
        .layout(id)
        .unwrap_or_else(|_| panic!("{at}: the element answers"))
}

/// Equal to 1/64 px: the offset and both rects are each snapped to 1/64, so their sum may sit
/// one unit from the rect.
fn same(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1.0 / 64.0 + f64::EPSILON
}

/// The scroll shift `explained` names held by `holder`: the viewport's for `None`.
fn scroll_by(explained: &LayoutExplanation, holder: Option<&str>) -> (f64, f64) {
    explained
        .shifts
        .iter()
        .find(|shift| shift.by == ShiftBy::Scroll && shift.holder.as_deref() == holder)
        .map_or((0.0, 0.0), |shift| (shift.dx, shift.dy))
}

/// Checks one snapshot node's answer against the node and its container, and returns its
/// placing word.
#[track_caller]
fn check_node(
    session: &mut Session,
    elements: &HashMap<String, NodeId>,
    node: &SnapshotNode,
    at: &str,
) -> String {
    let explained = read(session, &node.id, at);
    let bounds = node.bounds;
    assert!(
        (explained.bounds.x, explained.bounds.y) == (bounds.x, bounds.y)
            && (explained.bounds.width, explained.bounds.height) == (bounds.width, bounds.height),
        "{at}: the bounds are the snapshot's"
    );
    let Placer::Engine(by) = explained.placed.placer.clone() else {
        panic!("{at}: the engine placed it");
    };
    let transformed = explained
        .shifts
        .iter()
        .any(|shift| shift.by == ShiftBy::Transform);
    if !transformed {
        let (base, scroll) = match explained.placed.container.as_deref() {
            Some(container) => {
                let element = elements.get(container).copied();
                let reason =
                    element.and_then(|element| session.harness().base().layout_reason(element));
                assert!(reason.is_some(), "{at}: the container reads a reason");
                let container_bounds = reason.unwrap().bounds;
                (
                    (container_bounds.x, container_bounds.y),
                    scroll_by(&explained, Some(container)),
                )
            }
            None => ((0.0, 0.0), scroll_by(&explained, None)),
        };
        assert!(
            same(
                explained.bounds.x,
                base.0 + explained.placed.at.0 + scroll.0
            ) && same(
                explained.bounds.y,
                base.1 + explained.placed.at.1 + scroll.1
            ),
            "{at}: it sits at its container plus its offset"
        );
    }
    format!("{by:?}")
}

#[test]
fn every_snapshot_node_answers_and_sits_at_its_container_plus_its_offset() {
    for incremental in [false, true] {
        let mut words = HashSet::new();
        for (row, label) in SERVED.into_iter().enumerate() {
            let at = format!("incremental={incremental} task {row}");
            let mut session = hold_served(label, incremental);
            let snapshot = session.harness().doc.snapshot();
            let nodes: Vec<SnapshotNode> = snapshot.nodes().cloned().collect();
            assert!(!nodes.is_empty(), "{at}: the screen lists nodes");
            let elements: HashMap<String, NodeId> = session
                .harness()
                .doc
                .element_ids()
                .into_iter()
                .map(|(node, id)| (id, node))
                .collect();
            for (index, node) in nodes.iter().enumerate() {
                let at = format!("{at} node {index}");
                words.insert(check_node(&mut session, &elements, node, &at));
            }
        }
        for word in ["Flex", "Grid", "Block"] {
            assert!(
                words.contains(word),
                "incremental={incremental}: a node reads {word}"
            );
        }
    }
}

#[test]
fn the_pinned_cells_and_crud_readings() {
    for incremental in [false, true] {
        for (index, pinned) in PINNED.iter().enumerate() {
            let at = format!("incremental={incremental} pinned {index}");
            let mut session = hold_served(pinned.task, incremental);
            let explained = read(&mut session, pinned.id, &at);
            assert!(
                explained.placed.placer == Placer::Engine(pinned.by),
                "{at}: the placing word"
            );
            assert!(
                explained.placed.container.as_deref() == Some(pinned.container),
                "{at}: the container"
            );
            if let Some(width) = pinned.width {
                assert!(explained.width.by == width, "{at}: the width cause");
            }
            if let Some(height) = pinned.height {
                assert!(explained.height.by == height, "{at}: the height cause");
            }
        }
    }
}

#[test]
fn the_scrolled_last_crud_row_reads_the_lists_scroll_shift() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        for _ in 0..CREATES {
            assert!(
                act(&mut session, &click("crud-create")).is_some(),
                "{at}: a Create is accepted"
            );
        }
        let before = read(&mut session, LAST_ROW, &at);
        assert!(
            scroll_by(&before, Some("crud-list")) == (0.0, 0.0),
            "{at}: the list is not scrolled yet"
        );
        assert!(
            act(&mut session, &scroll(LAST_ROW)).is_some_and(|acted| acted.in_view == Some(true)),
            "{at}: the scroll brings the row into view"
        );
        let after = read(&mut session, LAST_ROW, &at);
        let (_, dy) = scroll_by(&after, Some("crud-list"));
        assert!(dy < 0.0, "{at}: the list's scroll moves the row up");
        assert!(
            after.placed.container.as_deref() == Some("crud-list")
                && after.placed.at == before.placed.at,
            "{at}: the row keeps its place in the list"
        );
        assert!(
            same(after.bounds.y, before.bounds.y + dy),
            "{at}: the bounds move by the shift"
        );
    }
}

#[test]
fn the_read_changes_nothing() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} flight-booker");
        let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
        assert!(
            act(&mut session, &click("flight-type")).is_some(),
            "{at}: the click opens the list"
        );
        assert!(
            session.harness().base().open_select().is_some(),
            "{at}: the list is open"
        );
        session.harness_mut().base_mut().take_changed_nodes();
        let ids: Vec<String> = session
            .harness()
            .doc
            .snapshot()
            .nodes()
            .map(|node| node.id.clone())
            .collect();
        let before = (session.harness().doc.snapshot(), focused(&session));
        let time = session.harness().time();
        for (index, id) in ids.iter().enumerate() {
            assert!(session.layout(id).is_ok(), "{at}: node {index} answers");
        }
        let after = (session.harness().doc.snapshot(), focused(&session));
        assert!(
            !session.harness().base().has_changes(),
            "{at}: the read marks no change"
        );
        assert!(
            before == after && before.0.diff(&after.0).is_empty(),
            "{at}: the snapshot, the focus readings and the diff are unchanged"
        );
        assert!(
            session.harness().base().open_select().is_some(),
            "{at}: the list stays open"
        );
        assert!(
            session.harness().time() == time,
            "{at}: the clock is unmoved"
        );
    }
}

#[test]
fn the_read_runs_no_pass_that_would_apply_pending_ticks() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} timer");
        let (mut session, ticks) = hold(LeanTask::Timer, incremental);
        ticks.deliver(5);
        let before = session.harness().doc.snapshot();
        let ids: Vec<String> = before.nodes().map(|node| node.id.clone()).collect();
        for (index, id) in ids.iter().enumerate() {
            assert!(session.layout(id).is_ok(), "{at}: node {index} answers");
        }
        let after = session.harness().doc.snapshot();
        assert!(
            before == after,
            "{at}: the pending ticks stay unapplied through the read"
        );
        session.harness_mut().pump();
        assert!(
            session.harness().doc.snapshot() != after,
            "{at}: a pass applies the ticks the read left pending"
        );
    }
}

#[test]
fn an_unread_id_is_not_found_and_a_deleted_rows_id_stale() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            session.layout("layout-no-such-id").err().map(|r| r.cause()) == Some(Cause::NotFound),
            "{at}: an unread id is not found"
        );
        read(&mut session, "crud-person-1", &at);
        assert!(
            act(&mut session, &click("crud-person-1")).is_some(),
            "{at}: the row is selected"
        );
        assert!(
            act(&mut session, &click("crud-delete")).is_some(),
            "{at}: the row is deleted"
        );
        assert!(
            session.layout("crud-person-1").err().map(|r| r.cause()) == Some(Cause::Stale),
            "{at}: the deleted row's id is stale"
        );
    }
}

/// The targets of every span and event made while it is the default subscriber.
#[derive(Clone, Default)]
struct Targets(Arc<Mutex<Vec<String>>>);

impl Subscriber for Targets {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attributes: &Attributes<'_>) -> Id {
        let mut targets = self.0.lock().unwrap();
        targets.push(attributes.metadata().target().to_string());
        Id::from_u64(targets.len() as u64)
    }

    fn record(&self, _: &Id, _: &Record<'_>) {}

    fn record_follows_from(&self, _: &Id, _: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let target = event.metadata().target().to_string();
        self.0.lock().unwrap().push(target);
    }

    fn enter(&self, _: &Id) {}

    fn exit(&self, _: &Id) {}
}

#[test]
fn the_read_writes_no_escher_line_at_trace() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        let targets = Targets::default();
        let answered = tracing::subscriber::with_default(targets.clone(), || {
            session.layout("crud-list").is_ok() && session.layout("crud-person-0").is_ok()
        });
        assert!(answered, "{at}: the read answers");
        let escher = targets
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|target| target.starts_with("escher_"))
            .count();
        assert!(escher == 0, "{at}: no escher_ line");
    }
}
