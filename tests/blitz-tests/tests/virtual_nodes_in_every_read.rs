//! A canvas's virtual nodes in every driver read that holds elements: the UI graph holds one
//! node per snapshot node, a virtual node owned by its canvas with its canvas's component and no
//! handler or read; `actions` lists each with its six target verbs and the cause each call
//! returns — `click` by the one target check, `layout` none, `type`, `scroll`, `screenshot` and
//! `canvas` `not-found`; `layout` explains a virtual id as placed by the app inside its canvas;
//! exploration acts on each as on every listed node; a replayed click on one reaches its
//! canvas's own listener, counted under the canvas's id; and the graph, actions and layout
//! answers carry the id and no declared name, kept whole in a kept check that replays. Each case
//! runs in both layout modes on a fixture canvas, its condition asserted first. A failure message
//! names a layout mode and a row only.

use dioxus::prelude::Element;
use dioxus_native_dom::{MASKED_VALUE, SizeCause};
use escher_driver::{ArgValue, Call, Cause, PathEnding, Placer, Session, StepBudget, explore};

mod fixtures;
mod session_common;

use fixtures::{
    VIRTUAL_IDS, VIRTUAL_NAMES, declares_five, virtual_canvas_listened_page, virtual_canvas_page,
};
#[cfg(unix)]
use session_common::snapshot;
use session_common::{
    act, actioned, actions, canvas, click, explained, focused, graph, graphed, layout,
    refused_unchanged, screenshot_of, scroll, type_into,
};

/// The label the fixture's sessions are started under.
const APP: &str = "virtual-nodes";

/// The declaring canvas's stable id.
const CANVAS: &str = "vn-canvas";

/// The stated tolerance of a bounds reading, read inclusively.
const TOLERANCE: f64 = 1.0 / 64.0;

fn hold_on(page: fn() -> Element, incremental: bool) -> Session {
    fixtures::hold(APP, page, incremental)
}

fn hold(incremental: bool) -> Session {
    hold_on(virtual_canvas_page, incremental)
}

fn mode(incremental: bool) -> String {
    format!("incremental={incremental}")
}

fn near(read: f64, expected: f64) -> bool {
    (read - expected).abs() <= TOLERANCE
}

#[test]
fn the_graph_holds_each_virtual_node_owned_by_its_canvas() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let screen = session.harness().doc.snapshot();
        let graph_read = session.harness().doc.graph();
        let snapshot_ids: Vec<&str> = screen.nodes().map(|node| node.id.as_str()).collect();
        let graph_ids: Vec<&str> = graph_read
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect();
        assert!(
            graph_ids == snapshot_ids,
            "{at}: one graph node per snapshot node, in pre-order"
        );
        let Some(canvas_node) = graph_read.nodes.iter().find(|node| node.id == CANVAS) else {
            panic!("{at}: the graph holds the canvas");
        };
        for (row, id) in VIRTUAL_IDS.iter().enumerate() {
            let Some(node) = graph_read.nodes.iter().find(|node| node.id == *id) else {
                panic!("{at}: row {row}: the graph holds the node");
            };
            assert!(
                node.owner.as_deref() == Some(CANVAS),
                "{at}: row {row}: owned by its canvas"
            );
            assert!(
                node.component == canvas_node.component,
                "{at}: row {row}: its canvas's component"
            );
            assert!(
                node.handlers.is_empty() && node.reads.is_empty(),
                "{at}: row {row}: no handler or read"
            );
        }
        let Some(answered) = graphed(&mut session, &graph()) else {
            panic!("{at}: the graph is accepted");
        };
        assert!(
            answered == graph_read,
            "{at}: the verb answers the in-process graph"
        );
    }
}

/// The causes `actions` lists for each virtual node, in [`VIRTUAL_IDS`]'s order, per target
/// verb in the table's order: `click`, `type`, `scroll`, `screenshot`, `layout`, `canvas`.
const VIRTUAL_CAUSES: [[Option<Cause>; 6]; 5] = {
    const N: Option<Cause> = Some(Cause::NotFound);
    [
        [None, N, N, N, None, N],
        [None, N, N, N, None, N],
        [Some(Cause::Disabled), N, N, N, None, N],
        [Some(Cause::Covered), N, N, N, None, N],
        [Some(Cause::OffScreen), N, N, N, None, N],
    ]
};

/// The call of each target verb naming `id`, in the table's order.
fn target_calls(id: &str) -> [Call; 6] {
    [
        click(id),
        type_into(id, ""),
        scroll(id),
        screenshot_of(id),
        layout(id),
        canvas(id),
    ]
}

#[test]
fn actions_lists_each_virtual_node_with_the_cause_its_call_returns() {
    assert_eq!(VIRTUAL_CAUSES.len(), 5, "five rows");
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let Some(nodes) = actioned(&mut session, &actions()) else {
            panic!("{at}: actions is accepted");
        };
        for (row, id) in VIRTUAL_IDS.iter().enumerate() {
            let Some(node) = nodes.iter().find(|node| node.id == *id) else {
                panic!("{at}: row {row}: actions lists the node");
            };
            assert!(
                node.owner.as_deref() == Some(CANVAS) && !node.keyed,
                "{at}: row {row}: owned by its canvas, not keyed"
            );
            let verbs: Vec<&str> = node.verbs.iter().map(|reading| reading.verb).collect();
            assert!(
                verbs == ["click", "type", "scroll", "screenshot", "layout", "canvas"],
                "{at}: row {row}: all six target verbs"
            );
            let causes: Vec<Option<Cause>> =
                node.verbs.iter().map(|reading| reading.cause).collect();
            assert!(
                causes == VIRTUAL_CAUSES[row],
                "{at}: row {row}: the listed causes"
            );
            for (column, call) in target_calls(id).iter().enumerate() {
                let mut fresh = hold(incremental);
                let returned = fresh.run(call).err().map(|refusal| refusal.cause());
                assert!(
                    returned == causes[column],
                    "{at}: row {row}: verb {column}: the call returns the listed cause"
                );
            }
        }
        assert!(
            act(&mut session, &click("vn-drop")).is_some(),
            "{at}: the drop button's click runs"
        );
        let Some(nodes) = actioned(&mut session, &actions()) else {
            panic!("{at}: actions is accepted after the drop");
        };
        let listed: Vec<&str> = nodes
            .iter()
            .map(|node| node.id.as_str())
            .filter(|id| id.contains("///"))
            .collect();
        assert!(
            listed == [VIRTUAL_IDS[0]],
            "{at}: after the drop, a alone is listed"
        );
    }
}

/// Each page, with `a`'s snapshot bounds (x, y, width, height) on it, as measured: the body's
/// 8 px margin, plus the holder's (40, 30) on the second page, plus the canvas's 7 px inset.
const LAYOUT_PAGES: [(fn() -> Element, (f64, f64, f64, f64)); 2] = [
    (virtual_canvas_page, (15.0, 15.0, 60.0, 30.0)),
    (virtual_canvas_listened_page, (55.0, 45.0, 60.0, 30.0)),
];

/// `a`'s and `e`'s offsets from the canvas's border box: `VIRTUAL_CANVAS_INSET` 7 plus the
/// declared region origins (0, 0) and (40, 20).
const AT: [(&str, (f64, f64)); 2] = [
    ("vn-canvas///a", (7.0, 7.0)),
    ("vn-canvas///e", (47.0, 27.0)),
];

#[test]
fn layout_explains_a_virtual_node_placed_by_its_app() {
    assert_eq!(LAYOUT_PAGES.len(), 2, "two pages");
    assert_eq!(AT.len(), 2, "two nodes");
    for incremental in [false, true] {
        for (page_row, (page, a_bounds)) in LAYOUT_PAGES.into_iter().enumerate() {
            let at = format!("{} page {page_row}", mode(incremental));
            let mut session = hold_on(page, incremental);
            declares_five(session.harness(), &at);
            let Some(canvas_reading) = explained(&mut session, &layout(CANVAS)) else {
                panic!("{at}: the canvas is explained");
            };
            let screen = session.harness().doc.snapshot();
            for (row, (id, offset)) in AT.into_iter().enumerate() {
                let Some(reading) = explained(&mut session, &layout(id)) else {
                    panic!("{at}: row {row}: the layout is accepted");
                };
                let Placer::App { canvas } = &reading.placed.placer else {
                    panic!("{at}: row {row}: placed by the app");
                };
                assert!(
                    canvas == CANVAS && reading.placed.container.as_deref() == Some(CANVAS),
                    "{at}: row {row}: in its canvas"
                );
                assert!(
                    near(reading.placed.at.0, offset.0) && near(reading.placed.at.1, offset.1),
                    "{at}: row {row}: at the region's offset from the canvas's border box"
                );
                let Some(listed) = screen.get(id) else {
                    panic!("{at}: row {row}: the snapshot lists the node");
                };
                assert!(
                    near(reading.bounds.x, listed.bounds.x)
                        && near(reading.bounds.y, listed.bounds.y)
                        && near(reading.bounds.width, listed.bounds.width)
                        && near(reading.bounds.height, listed.bounds.height),
                    "{at}: row {row}: the snapshot's bounds"
                );
                assert!(
                    reading.shifts == canvas_reading.shifts,
                    "{at}: row {row}: the canvas's shifts"
                );
                assert!(
                    near(reading.width.px, listed.bounds.width)
                        && near(reading.height.px, listed.bounds.height)
                        && reading.width.by == SizeCause::Layout
                        && reading.height.by == SizeCause::Layout,
                    "{at}: row {row}: each size its region's, by layout"
                );
            }
            let Some(a) = explained(&mut session, &layout(VIRTUAL_IDS[0])) else {
                panic!("{at}: a is explained");
            };
            let (x, y, width, height) = a_bounds;
            assert!(
                near(a.bounds.x, x)
                    && near(a.bounds.y, y)
                    && near(a.bounds.width, width)
                    && near(a.bounds.height, height),
                "{at}: a stands where the page puts it"
            );
            let Ok(answer) = session.run(&layout(VIRTUAL_IDS[0])) else {
                panic!("{at}: the layout is accepted");
            };
            let line = answer.to_json();
            assert!(
                line.contains(r#""by":"app""#) && line.contains(r#""canvas":"vn-canvas""#),
                "{at}: the written line places it by the app in its canvas"
            );

            session.harness_mut().base_mut().take_changed_nodes();
            let before = (session.harness().doc.snapshot(), focused(&session));
            assert!(
                session.layout(VIRTUAL_IDS[0]).is_ok(),
                "{at}: the in-process read answers"
            );
            let after = (session.harness().doc.snapshot(), focused(&session));
            assert!(
                !session.harness().base().has_changes(),
                "{at}: the read marks no change"
            );
            assert!(
                before == after && before.0.diff(&after.0).is_empty(),
                "{at}: the snapshot and diff are unchanged"
            );

            let (cause, unchanged) = refused_unchanged(&mut session, &layout("vn-canvas///zz"));
            assert!(
                cause == Some(Cause::NotFound) && unchanged,
                "{at}: an unknown key reads not found"
            );
            assert!(
                act(&mut session, &click("vn-drop")).is_some(),
                "{at}: the drop button's click runs"
            );
            let (cause, unchanged) = refused_unchanged(&mut session, &layout(VIRTUAL_IDS[1]));
            assert!(
                cause == Some(Cause::Stale) && unchanged,
                "{at}: a dropped node reads stale"
            );
        }
    }
}

#[test]
fn the_new_answers_carry_the_id_and_no_declared_name() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let calls = [graph(), actions(), layout(VIRTUAL_IDS[0])];
        assert_eq!(calls.len(), 3, "{at}: three answers");
        for (row, call) in calls.iter().enumerate() {
            let Ok(answer) = session.run(call) else {
                panic!("{at}: row {row}: the call is accepted");
            };
            let line = answer.to_json();
            assert!(
                line.contains(VIRTUAL_IDS[0]),
                "{at}: row {row}: the answer holds the virtual id"
            );
            for name in VIRTUAL_NAMES {
                assert!(
                    !line.contains(name),
                    "{at}: row {row}: the answer holds no declared name"
                );
            }
            assert!(
                !line.contains(MASKED_VALUE),
                "{at}: row {row}: the answer holds no mask"
            );
        }
    }
}

/// The id argument of `call`.
fn id_of(call: &Call) -> Option<&str> {
    call.args.iter().find_map(|(name, value)| match value {
        ArgValue::Text(text) if name == "id" => Some(text.as_str()),
        _ => None,
    })
}

/// Each one-call path the exploration must hold: its verb, the node it names and its ending.
const EXPLORED: [(&str, &str, PathEnding); 6] = [
    (
        "click",
        "vn-canvas///a",
        PathEnding::Accepted { settled: true },
    ),
    (
        "type",
        "vn-canvas///a",
        PathEnding::Refused(Cause::NotFound),
    ),
    (
        "scroll",
        "vn-canvas///a",
        PathEnding::Refused(Cause::NotFound),
    ),
    (
        "click",
        "vn-canvas///b",
        PathEnding::Refused(Cause::Disabled),
    ),
    (
        "click",
        "vn-canvas///d",
        PathEnding::Refused(Cause::Covered),
    ),
    (
        "click",
        "vn-canvas///c",
        PathEnding::Refused(Cause::OffScreen),
    ),
];

#[test]
fn exploration_acts_on_each_virtual_node_as_a_listed_node() {
    assert_eq!(EXPLORED.len(), 6, "six paths");
    let budget = StepBudget::new(256).expect("256 is within the bound");
    for incremental in [false, true] {
        let at = mode(incremental);
        declares_five(hold(incremental).harness(), &at);
        let Ok(exploration) = explore(|| Ok(hold(incremental)), budget, None) else {
            panic!("{at}: the exploration runs");
        };
        for (row, (verb, id, ending)) in EXPLORED.into_iter().enumerate() {
            let Some(path) = exploration.paths.iter().find(|path| {
                path.calls.len() == 1
                    && path.calls[0].verb == verb
                    && id_of(&path.calls[0]) == Some(id)
            }) else {
                panic!("{at}: row {row}: the path is explored");
            };
            assert!(
                path.ending == ending,
                "{at}: row {row}: ends as its call does"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn a_replayed_virtual_click_reaches_its_canvas_listener() {
    use escher_driver::{Check, handler_coverage};
    use session_common::{record_check, state_dir};

    let listened = |incremental| hold_on(virtual_canvas_listened_page, incremental);
    for incremental in [false, true] {
        let at = mode(incremental);
        declares_five(listened(incremental).harness(), &at);
        // The step, then whether the canvas's click and the first node's click read unexercised.
        let rows = [
            (click(VIRTUAL_IDS[0]), false, false),
            (click("vn-drop"), true, true),
        ];
        assert_eq!(rows.len(), 2, "{at}: two checks");
        for (row, (step, canvas_unexercised, first_unexercised)) in rows.into_iter().enumerate() {
            let at = format!("{at} row {row}");
            let dir = state_dir(&format!("vnr-cov-{incremental}-{row}"));
            let bytes = record_check(&dir, || listened(incremental), &[snapshot(), step], &at);
            let check =
                Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
            let Ok(coverage) = handler_coverage(&[check], || Ok(listened(incremental))) else {
                panic!("{at}: the boots start");
            };
            assert!(coverage.held == 1, "{at}: the replay holds");
            let listed = |id: &str| {
                coverage
                    .unexercised
                    .iter()
                    .find(|handler| handler.id == id && handler.event == "click")
            };
            assert!(
                listed(CANVAS).is_some() == canvas_unexercised,
                "{at}: the canvas's click reads unexercised as stated"
            );
            assert!(
                listed(VIRTUAL_IDS[0]).is_some() == first_unexercised,
                "{at}: the first node's click reads unexercised as stated"
            );
            let second = listed(VIRTUAL_IDS[1])
                .unwrap_or_else(|| panic!("{at}: the unclicked node's click reads unexercised"));
            assert!(
                second.owner.as_deref() == Some(CANVAS),
                "{at}: owned by its canvas"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_of_the_new_reads_replays() {
    use escher_driver::Check;
    use session_common::{record_check, state_dir};
    use std::fs;

    let calls = [graph(), actions(), layout(VIRTUAL_IDS[0])];
    for incremental in [false, true] {
        let at = mode(incremental);
        declares_five(hold(incremental).harness(), &at);
        let dir = state_dir(&format!("vnr-kept-{incremental}"));
        let bytes = record_check(&dir, || hold(incremental), &calls, &at);
        let text = std::str::from_utf8(&bytes).expect("a check is text");
        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        assert!(
            check.steps().len() == calls.len()
                && text.matches(VIRTUAL_IDS[0]).count() >= calls.len(),
            "{at}: each answer is kept with the virtual id"
        );
        let replay = check.replay(&mut hold(incremental));
        assert!(
            replay.holds() && replay.ran() == calls.len(),
            "{at}: the replay holds"
        );

        let dir = state_dir(&format!("vnr-explore-{incremental}"));
        let _ = fs::remove_dir_all(&dir);
        let budget = StepBudget::new(256).expect("256 is within the bound");
        let Ok(exploration) = explore(|| Ok(hold(incremental)), budget, Some(&dir)) else {
            panic!("{at}: the exploration runs");
        };
        let Some(index) = exploration.paths.iter().position(|path| {
            path.calls.len() == 1
                && path.calls[0].verb == "click"
                && id_of(&path.calls[0]) == Some(VIRTUAL_IDS[0])
        }) else {
            panic!("{at}: the virtual click's path is explored");
        };
        let bytes = fs::read(dir.join(format!("check-{}.json", index + 1)))
            .unwrap_or_else(|_| panic!("{at}: the path's check is kept"));
        let _ = fs::remove_dir_all(&dir);
        let text = std::str::from_utf8(&bytes).expect("a check is text");
        assert!(
            text.contains(VIRTUAL_IDS[0]),
            "{at}: the path's check holds the virtual id"
        );
        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        let replay = check.replay(&mut hold(incremental));
        assert!(replay.holds(), "{at}: the path's check replays");
    }
}
