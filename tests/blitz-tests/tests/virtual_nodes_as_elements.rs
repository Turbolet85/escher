//! A canvas's virtual nodes as elements of the screen: each node a canvas declares and its read
//! takes enters the accessibility tree as a child of its canvas's node, with its reserved id
//! `{owner}///{key}`, its declared role, name and disabled state and a tree id equal to no DOM
//! node's; the snapshot reads it from there with its region moved by its canvas's content-box
//! origin; the driver's `click` takes its id and acts at its region's centre, and refuses it
//! `disabled`, `off-screen`, `covered`, `not-found` or `stale` in the run path's one order, with
//! nothing run. `type`, `scroll`, `screenshot` and `canvas` read a virtual id `not-found`, and the
//! lint answer holds none; the graph, actions and layout reads of a virtual node are
//! `virtual_nodes_in_every_read`'s. The id and name cross MCP and a kept check, which replays. Each case
//! runs in both layout modes, its fixture's condition asserted first. A failure message names a
//! layout mode and a case or row only.

use accesskit::Role;
use blitz_dom::Document;
use dioxus_native_dom::{Reading, Snapshot, SnapshotNode};
use escher_driver::{Cause, Session, SessionError};

mod fixtures;
mod session_common;

use fixtures::{
    VIRTUAL_CANVAS_INSET, VIRTUAL_IDS, VIRTUAL_NAMES, declares_five, virtual_canvas_page,
};
use session_common::{
    act, canvas, click, lint, mcp_result_text, mcp_tool_result, refused_unchanged, screenshot_of,
    scroll, serve_lines, snapshot, tool_call, type_into,
};

/// The label the fixture's session is started under.
const APP: &str = "virtual-nodes";

/// The declaring canvas's stable id.
const CANVAS: &str = "vn-canvas";

/// Each declared node's role and region (x, y, width, height) in the canvas's content box, in
/// [`VIRTUAL_IDS`]'s order.
const DECLARED: [(Role, (f64, f64, f64, f64)); 5] = [
    (Role::Button, (0.0, 0.0, 60.0, 30.0)),
    (Role::Button, (40.0, 20.0, 40.0, 30.0)),
    (Role::CheckBox, (100.0, 0.0, 40.0, 30.0)),
    (Role::Button, (0.0, 60.0, 40.0, 30.0)),
    (Role::Button, (150.0, 90.0, 40.0, 30.0)),
];

/// The one node declared disabled, by its index in [`VIRTUAL_IDS`].
const DISABLED: usize = 2;

/// The stated tolerance of a bounds reading, read inclusively.
const TOLERANCE: f64 = 1.0 / 64.0;

fn hold(incremental: bool) -> Session {
    fixtures::hold(APP, virtual_canvas_page, incremental)
}

fn mode(incremental: bool) -> String {
    format!("incremental={incremental}")
}

/// The snapshot node of the canvas.
#[track_caller]
fn canvas_node<'a>(screen: &'a Snapshot, at: &str) -> &'a SnapshotNode {
    screen
        .get(CANVAS)
        .unwrap_or_else(|| panic!("{at}: the snapshot lists the canvas"))
}

#[test]
fn the_tree_holds_each_virtual_node_under_its_canvas() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let session = hold(incremental);
        let harness = session.harness();
        declares_five(harness, &at);
        let tree = harness.doc.accessibility_tree();
        let node_of = |id: accesskit::NodeId| {
            tree.nodes
                .iter()
                .find(|(tree_id, _)| *tree_id == id)
                .map(|(_, node)| node)
        };
        let Some((_, canvas)) = tree
            .nodes
            .iter()
            .find(|(_, node)| node.author_id() == Some(CANVAS))
        else {
            panic!("{at}: the tree holds the canvas");
        };
        let children = canvas.children();
        assert!(children.len() >= 5, "{at}: the canvas holds five children");
        let (own, virtual_ids) = children.split_at(children.len() - 5);
        assert!(
            own.iter().all(|id| node_of(*id)
                .is_none_or(|node| node.author_id().is_none_or(|a| !a.contains("///")))),
            "{at}: the canvas's own children come first"
        );
        let dom_ids: Vec<u64> = harness
            .doc
            .inner
            .borrow()
            .tree()
            .iter()
            .map(|(id, _)| id.as_u64())
            .collect();
        for (row, id) in virtual_ids.iter().enumerate() {
            let Some(node) = node_of(*id) else {
                panic!("{at}: row {row}: the child is in the tree");
            };
            let (role, _) = DECLARED[row];
            assert!(
                node.author_id() == Some(VIRTUAL_IDS[row])
                    && node.role() == role
                    && node.label() == Some(VIRTUAL_NAMES[row]),
                "{at}: row {row}: the child carries its id, role and name as declared"
            );
            assert!(
                node.is_disabled() == (row == DISABLED),
                "{at}: row {row}: the child is disabled as declared"
            );
            assert!(
                (id.0 >> 32) % 2 == 0,
                "{at}: row {row}: the tree id's high half is even"
            );
            assert!(
                !dom_ids.contains(&id.0),
                "{at}: row {row}: the tree id is no DOM node's"
            );
        }
    }
}

#[test]
fn a_hidden_canvas_holds_no_virtual_node() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let Some(hidden) = act(&mut session, &click("vn-hide")) else {
            panic!("{at}: the hide button's click runs");
        };
        assert!(hidden.settled, "{at}: the click settles");
        let doc = &session.harness().doc;
        let tree = doc.accessibility_tree();
        assert!(
            tree.nodes
                .iter()
                .all(|(_, node)| node.author_id().is_none_or(|id| !id.contains("///"))),
            "{at}: the tree holds no virtual node"
        );
        assert!(
            doc.snapshot().nodes().all(|node| !node.id.contains("///")),
            "{at}: the snapshot holds no virtual node"
        );
    }
}

#[test]
fn the_snapshot_reads_each_virtual_node() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let screen = session.harness().doc.snapshot();
        let canvas = canvas_node(&screen, &at);
        let virtual_nodes: Vec<&SnapshotNode> = canvas
            .children
            .iter()
            .filter(|node| node.id.contains("///"))
            .collect();
        let ids: Vec<&str> = virtual_nodes.iter().map(|node| node.id.as_str()).collect();
        assert!(
            ids == VIRTUAL_IDS,
            "{at}: the canvas's children are its five nodes"
        );
        let origin = (
            canvas.bounds.x + VIRTUAL_CANVAS_INSET,
            canvas.bounds.y + VIRTUAL_CANVAS_INSET,
        );
        for (row, node) in virtual_nodes.iter().enumerate() {
            let (role, (x, y, width, height)) = DECLARED[row];
            assert!(
                node.role == role && node.name == VIRTUAL_NAMES[row],
                "{at}: row {row}: role and name as declared"
            );
            assert!(
                node.state.enabled == Some(row != DISABLED),
                "{at}: row {row}: enabled as declared"
            );
            assert!(!node.state.clickable, "{at}: row {row}: not clickable");
            assert!(node.children.is_empty(), "{at}: row {row}: no children");
            let near = |read: f64, expected: f64| (read - expected).abs() <= TOLERANCE;
            assert!(
                near(node.bounds.x, origin.0 + x)
                    && near(node.bounds.y, origin.1 + y)
                    && near(node.bounds.width, width)
                    && near(node.bounds.height, height),
                "{at}: row {row}: bounds are the region moved by the content-box origin"
            );
        }
        let Ok(answer) = session.run(&snapshot()) else {
            panic!("{at}: the snapshot is accepted");
        };
        let line = answer.to_json();
        for (row, (id, name)) in VIRTUAL_IDS.iter().zip(VIRTUAL_NAMES).enumerate() {
            assert!(
                line.contains(id) && line.contains(name),
                "{at}: row {row}: the answer holds the id and the name"
            );
        }
    }
}

#[test]
fn a_click_on_a_virtual_node_acts_on_its_region() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let Some(acted) = act(&mut session, &click(VIRTUAL_IDS[0])) else {
            panic!("{at}: the click is accepted");
        };
        assert!(acted.settled, "{at}: the click returns settled");
        let Some(changed) = acted.diff.changed.iter().find(|n| n.id == VIRTUAL_IDS[0]) else {
            panic!("{at}: the diff names the clicked node changed");
        };
        assert!(
            changed.differs.contains(&Reading::Name) && changed.name == VIRTUAL_NAMES[5],
            "{at}: the clicked node is renamed"
        );
        assert!(
            acted.diff.changed.iter().all(|n| n.id != VIRTUAL_IDS[1]),
            "{at}: the overlapping node is unchanged"
        );
        let screen = session.harness().doc.snapshot();
        assert!(
            canvas_node(&screen, &at).state.focused,
            "{at}: the canvas holds focus"
        );
    }
}

#[test]
fn a_virtual_node_is_refused_in_the_one_order() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        assert!(
            session.run(&snapshot()).is_ok(),
            "{at}: the snapshot is accepted"
        );
        let rows = [
            (VIRTUAL_IDS[2], Cause::Disabled),
            (VIRTUAL_IDS[4], Cause::OffScreen),
            (VIRTUAL_IDS[3], Cause::Covered),
            ("vn-canvas///zz", Cause::NotFound),
        ];
        for (row, (id, cause)) in rows.into_iter().enumerate() {
            let (read, unchanged) = refused_unchanged(&mut session, &click(id));
            assert!(read == Some(cause), "{at}: row {row}: refused as stated");
            assert!(unchanged, "{at}: row {row}: nothing ran");
        }
        let Some(dropped) = act(&mut session, &click("vn-drop")) else {
            panic!("{at}: the drop button's click runs");
        };
        assert!(dropped.settled, "{at}: the drop settles");
        let (read, unchanged) = refused_unchanged(&mut session, &click(VIRTUAL_IDS[1]));
        assert!(
            read == Some(Cause::Stale),
            "{at}: the dropped node reads stale"
        );
        assert!(unchanged, "{at}: nothing ran on the stale node");
    }
}

#[test]
fn the_other_id_verbs_read_a_virtual_id_not_found() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        assert!(
            session.run(&snapshot()).is_ok(),
            "{at}: the snapshot is accepted"
        );
        let id = VIRTUAL_IDS[0];
        let calls = [
            type_into(id, "x"),
            scroll(id),
            screenshot_of(id),
            canvas(id),
        ];
        assert_eq!(
            calls.len(),
            4,
            "{at}: four verbs read a virtual id not found"
        );
        for (row, call) in calls.iter().enumerate() {
            let (read, unchanged) = refused_unchanged(&mut session, call);
            assert!(
                read == Some(Cause::NotFound),
                "{at}: row {row}: refused not found"
            );
            assert!(unchanged, "{at}: row {row}: nothing ran");
        }
    }
}

/// On this page `a` and `e` overlap and `c` lies past its holder's clip, so a lint that read
/// virtual nodes would find them.
#[test]
fn the_lint_holds_no_virtual_node() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = hold(incremental);
        declares_five(session.harness(), &at);
        let Ok(answer) = session.run(&lint()) else {
            panic!("{at}: the lint is accepted");
        };
        assert!(
            !answer.to_json().contains("///"),
            "{at}: the lint answer holds no virtual node"
        );
    }
}

fn boot(incremental: bool) -> impl FnMut(&str) -> Result<Session, SessionError> {
    move |app| Session::start(app, || fixtures::boot(virtual_canvas_page, incremental))
}

#[test]
fn the_mcp_role_carries_the_virtual_node() {
    for incremental in [false, true] {
        let at = mode(incremental);
        declares_five(hold(incremental).harness(), &at);
        let requests = [
            tool_call(1, "start", &format!(r#"{{"app":"{APP}"}}"#)),
            tool_call(2, "snapshot", "{}"),
            tool_call(3, "click", &format!(r#"{{"id":"{}"}}"#, VIRTUAL_IDS[0])),
        ];
        let replies = serve_lines(&requests, &[APP], boot(incremental));
        assert!(replies.len() == 3, "{at}: one reply per call");
        let screen = mcp_result_text(&replies[1]);
        assert!(
            screen.contains(VIRTUAL_IDS[0]) && screen.contains(VIRTUAL_NAMES[0]),
            "{at}: the snapshot result holds the id and the name"
        );
        let (is_error, _) = mcp_tool_result(&replies[2]);
        assert!(!is_error, "{at}: the click is accepted");
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_holds_and_replays_the_virtual_click() {
    use escher_driver::Check;
    use session_common::{record_check, state_dir};

    let calls = [snapshot(), click(VIRTUAL_IDS[0])];
    for incremental in [false, true] {
        let at = mode(incremental);
        declares_five(hold(incremental).harness(), &at);
        let dir = state_dir(&format!("vne-check-{incremental}"));
        let bytes = record_check(&dir, || hold(incremental), &calls, &at);
        let text = std::str::from_utf8(&bytes).expect("a check is text");
        assert!(
            text.contains(VIRTUAL_IDS[0]) && text.contains(VIRTUAL_NAMES[5]),
            "{at}: the check holds the id and the renamed name"
        );
        let check = Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
        let replay = check.replay(&mut hold(incremental));
        assert!(
            replay.holds() && replay.ran() == calls.len(),
            "{at}: the replay holds"
        );
    }
}
