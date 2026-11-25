//! Circle Drawer's circles as virtual nodes: the canvas declares each circle it paints as a node
//! `circle-canvas///c{id}`, keyed by the id the app gives the circle, role `Image`, named
//! `Circle {id}, {d} px`, its region its bounding square cut to the content box. Each reads in
//! the accessibility tree, the snapshot and its diff, the UI graph, `actions`, `layout` and an
//! exploration; an undone circle's id reads `stale` and its redo keeps it. A click on a circle's
//! node adds a circle at its centre, another circle never covers one, the lint reads no circle
//! and a label wider than its circle reads no `truncated` finding — the stated limits, each
//! witnessed. The full canvas answer groups each command under the circle that covers it, its
//! line closed by the node's 1-based index, positional. Handler coverage lists a circle's `click`
//! under its own id. Booted in process through `stand::boot_circle_drawer`, in both layout modes,
//! each case's fixture condition asserted first. A failure names a layout mode and a case or row
//! only.

use accesskit::Role;
use blitz_dom::{Document, LocalName, Namespace, QualName};
use dioxus_native_dom::Reading;
use escher_driver::{Call, Cause, Outcome, PathEnding, Placer, Session, StepBudget, explore};

mod common;
mod session_common;

use common::{CIRCLE_NODE_CAUSES, CIRCLE_SPILLS};
use session_common::{
    CIRCLE_CANVAS as CANVAS, act, actioned, actions, advance, canvas, canvas_full, canvas_part,
    canvased, carried, circle_drawer, click, draw, explained, graph, graphed, layout, lint, linted,
    mcp_result_text, refused, screenshot_of, scroll, type_into,
};

/// The stated tolerance of a bounds reading, read inclusively.
const TOLERANCE: f64 = 1.0 / 64.0;

/// The width of `.cd-canvas`'s border: the content box's offset from the border box.
const BORDER: f64 = 2.0;

/// The first circle's node, its name and its region in the content box after one centre click,
/// as measured: a 40 px circle centred at (348, 198).
const FIRST: &str = "circle-canvas///c1";
const FIRST_NAME: &str = "Circle 1, 40 px";
const FIRST_REGION: (f64, f64, f64, f64) = (328.0, 178.0, 40.0, 40.0);

fn mode(incremental: bool) -> String {
    format!("incremental={incremental}")
}

fn near(read: f64, expected: f64) -> bool {
    (read - expected).abs() <= TOLERANCE
}

/// The id of circle `k`'s node.
fn node(k: u64) -> String {
    format!("{CANVAS}///c{k}")
}

/// The circle nodes the snapshot lists, in its order.
fn circle_nodes(session: &Session) -> Vec<String> {
    session
        .harness()
        .doc
        .snapshot()
        .nodes()
        .filter(|n| n.id.starts_with("circle-canvas///"))
        .map(|n| n.id.clone())
        .collect()
}

/// Runs `call`, which must be an accepted acting call.
#[track_caller]
fn acts(session: &mut Session, call: &Call, at: &str) -> session_common::Acted {
    act(session, call).unwrap_or_else(|| panic!("{at}: the step is accepted"))
}

/// A centre click on the canvas, then an `advance` so the next one is no double click.
#[track_caller]
fn add_circle(session: &mut Session, at: &str) {
    acts(session, &click(CANVAS), at);
    acts(session, &advance(1000), at);
}

/// The content box's origin on the page: the canvas's snapshot bounds moved by its border.
#[track_caller]
fn content_origin(session: &Session, at: &str) -> (f64, f64) {
    let screen = session.harness().doc.snapshot();
    let canvas = screen
        .get(CANVAS)
        .unwrap_or_else(|| panic!("{at}: the snapshot lists the canvas"));
    (canvas.bounds.x + BORDER, canvas.bounds.y + BORDER)
}

/// The text of the full canvas answer, read in process.
#[track_caller]
fn full_text(session: &mut Session, at: &str) -> String {
    let reading = session
        .canvas(CANVAS)
        .unwrap_or_else(|_| panic!("{at}: the canvas reads"));
    mcp_result_text(&Outcome::CanvasDump { reading }.to_json())
}

/// The node index each fill of the full reading carries, in paint order.
#[track_caller]
fn fill_nodes(session: &mut Session, at: &str) -> Vec<Option<usize>> {
    let reading = session
        .canvas(CANVAS)
        .unwrap_or_else(|_| panic!("{at}: the canvas reads"));
    reading
        .commands
        .iter()
        .zip(&reading.nodes)
        .filter(|(command, _)| escher_driver::COMMAND_KINDS[command.kind()] == "fill")
        .map(|(_, grouped)| *grouped)
        .collect()
}

/// Writes `text` to the canvas's `data-circles` through the held harness's mutator, then
/// settles.
#[track_caller]
fn write_circles(session: &mut Session, text: &str, at: &str) {
    let harness = session.harness_mut();
    let node = harness.query(&format!("#{CANVAS}")).expect("the canvas");
    {
        let mut doc = harness.base_mut();
        let mut mutator = doc.mutate();
        mutator.set_attribute(
            node,
            QualName::new(None, Namespace::from(""), LocalName::from("data-circles")),
            text,
        );
    }
    assert!(harness.settle().is_ok(), "{at}: the write settles");
}

/// The lines `after` holds that `before` does not, and those `before` holds that `after` does
/// not, each counted as many times as it stands.
fn line_diff(before: &str, after: &str) -> (Vec<String>, Vec<String>) {
    let mut gone: Vec<&str> = before.lines().collect();
    let mut new = Vec::new();
    for line in after.lines() {
        match gone.iter().position(|was| *was == line) {
            Some(index) => {
                gone.remove(index);
            }
            None => new.push(line.to_string()),
        }
    }
    (new, gone.into_iter().map(str::to_string).collect())
}

#[test]
fn one_click_reads_a_circle_node_in_the_tree_and_the_snapshot() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        let Some(Outcome::CanvasSummary { width, height, .. }) =
            canvased(&mut session, &canvas(CANVAS))
        else {
            panic!("{at}: the summary reads");
        };
        assert!(
            width == 696.0 && height == 396.0,
            "{at}: the content box is the widget's stated 696 × 396"
        );
        assert!(circle_nodes(&session).is_empty(), "{at}: no circle at boot");
        acts(&mut session, &click(CANVAS), &at);
        assert!(
            carried(&session) == "348,198,40,1",
            "{at}: the click adds circle 1"
        );

        let tree = session.harness().doc.accessibility_tree();
        let Some((canvas_id, canvas_node)) = tree
            .nodes
            .iter()
            .find(|(_, n)| n.author_id() == Some(CANVAS))
        else {
            panic!("{at}: the tree holds the canvas");
        };
        let children: Vec<_> = canvas_node
            .children()
            .iter()
            .filter_map(|child| tree.nodes.iter().find(|(id, _)| id == child))
            .filter(|(_, n)| n.author_id().is_some_and(|id| id.contains("///")))
            .collect();
        assert!(children.len() == 1, "{at}: the canvas's one virtual child");
        let (_, circle) = children[0];
        assert!(
            circle.author_id() == Some(FIRST)
                && circle.role() == Role::Image
                && circle.label() == Some(FIRST_NAME),
            "{at}: its id, role and name"
        );
        assert!(
            *canvas_id != accesskit::NodeId(u64::MAX),
            "{at}: a DOM node"
        );

        let screen = session.harness().doc.snapshot();
        let Some(listed) = screen.get(FIRST) else {
            panic!("{at}: the snapshot lists the circle");
        };
        let origin = content_origin(&session, &at);
        let (x, y, w, h) = FIRST_REGION;
        assert!(
            listed.role == Role::Image && listed.name == FIRST_NAME,
            "{at}: the snapshot's role and name"
        );
        assert!(
            near(listed.bounds.x, origin.0 + x)
                && near(listed.bounds.y, origin.1 + y)
                && near(listed.bounds.width, w)
                && near(listed.bounds.height, h),
            "{at}: its bounds are its region moved by the content origin"
        );
    }
}

#[test]
fn at_twelve_and_a_hundred_circles_every_circle_reads_a_node() {
    assert_eq!(CIRCLE_SPILLS.len(), 7, "seven spills");
    for incremental in [false, true] {
        for n in [12_usize, 100] {
            let at = format!("{} n={n}", mode(incremental));
            let mut session = circle_drawer(incremental);
            draw(&mut session, n, &at);
            let expected: Vec<String> = (1..=n as u64).map(node).collect();
            assert!(
                circle_nodes(&session) == expected,
                "{at}: one node per circle, in paint order"
            );
            let harness = session.harness();
            let canvas_node = harness.query(&format!("#{CANVAS}")).expect("the canvas");
            let Ok(read) = harness.doc.virtual_nodes(canvas_node) else {
                panic!("{at}: the declarations read");
            };
            assert!(
                read.nodes.len() == n && read.refused.is_empty(),
                "{at}: none is refused"
            );
            if n == 100 {
                for (row, (k, (x, y, w, h))) in CIRCLE_SPILLS.into_iter().enumerate() {
                    let Some(taken) = read.nodes.iter().find(|t| t.id == node(k)) else {
                        panic!("{at}: row {row}: the spilled circle is taken");
                    };
                    let r = taken.declaration.region;
                    assert!(
                        near(r.x, x) && near(r.y, y) && near(r.width, w) && near(r.height, h),
                        "{at}: row {row}: its region is cut to the content box"
                    );
                }
            }
        }
    }
}

#[test]
fn an_undone_circle_reads_stale_and_its_redo_keeps_its_id() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        add_circle(&mut session, &at);
        add_circle(&mut session, &at);
        assert!(
            circle_nodes(&session) == [node(1), node(2)],
            "{at}: two clicks read c1 and c2"
        );
        acts(&mut session, &click("circle-undo"), &at);
        assert!(
            circle_nodes(&session) == [node(1)],
            "{at}: the undo takes c2 away"
        );
        assert!(
            refused(&mut session, &click(&node(2))) == Some(Cause::Stale),
            "{at}: an undone circle reads stale"
        );
        acts(&mut session, &click("circle-redo"), &at);
        assert!(
            circle_nodes(&session) == [node(1), node(2)],
            "{at}: the redo brings c2 back under its id"
        );
        acts(&mut session, &click("circle-undo"), &at);
        acts(&mut session, &click(CANVAS), &at);
        assert!(
            circle_nodes(&session) == [node(1), node(3)],
            "{at}: a click after the undo reads c3, never c2 again"
        );
        assert!(
            refused(&mut session, &click(&node(99))) == Some(Cause::NotFound),
            "{at}: an id no screen read is not found"
        );
    }
}

#[test]
fn a_click_and_a_resize_read_in_the_diff() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        let first = acts(&mut session, &click(CANVAS), &at);
        assert!(
            first.added().contains(&FIRST),
            "{at}: the click's diff adds the new node"
        );
        acts(&mut session, &click(CANVAS), &at);
        assert!(
            session
                .harness()
                .doc
                .snapshot()
                .get("circle-diameter")
                .is_some(),
            "{at}: a double click opens the dialog"
        );
        let resize = acts(&mut session, &click("circle-diameter"), &at);
        let Some(changed) = resize.diff.changed.iter().find(|n| n.id == FIRST) else {
            panic!("{at}: the resize changes c1");
        };
        assert!(
            changed.differs.contains(&Reading::Name)
                && changed.differs.contains(&Reading::Bounds)
                && changed.name == "Circle 1, 53 px",
            "{at}: its diff names name and bounds"
        );
    }
}

#[test]
fn a_click_on_a_circle_node_adds_a_circle_at_its_centre() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        add_circle(&mut session, &at);
        let acted = acts(&mut session, &click(FIRST), &at);
        assert!(acted.settled, "{at}: the click settles");
        assert!(
            acted.added() == [node(2).as_str()],
            "{at}: its diff adds c2"
        );
        assert!(
            acted.changed().iter().all(|id| *id != FIRST),
            "{at}: c1 is unchanged"
        );
        assert!(
            carried(&session) == "348,198,40,1;348,198,40,2",
            "{at}: the new circle stands at c1's centre"
        );
    }
}

#[test]
fn a_circle_under_another_is_not_refused_covered() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        add_circle(&mut session, &at);
        add_circle(&mut session, &at);
        assert!(
            carried(&session) == "348,198,40,1;348,198,40,2",
            "{at}: c2 is painted over c1's centre"
        );
        assert!(
            refused(&mut session, &click(FIRST)).is_none(),
            "{at}: a click on c1 is accepted, not covered"
        );
    }
}

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

/// The id argument of `call`.
fn id_of(call: &Call) -> Option<&str> {
    call.args.iter().find_map(|(name, value)| match value {
        escher_driver::ArgValue::Text(text) if name == "id" => Some(text.as_str()),
        _ => None,
    })
}

#[test]
fn the_graph_actions_layout_and_exploration_read_each_circle_by_id() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        draw(&mut session, 12, &at);
        let ids: Vec<String> = (1..=12).map(node).collect();
        let Some(graph_read) = graphed(&mut session, &graph()) else {
            panic!("{at}: the graph is accepted");
        };
        let Some(nodes) = actioned(&mut session, &actions()) else {
            panic!("{at}: actions is accepted");
        };
        for (row, id) in ids.iter().enumerate() {
            let Some(g) = graph_read.nodes.iter().find(|n| n.id == *id) else {
                panic!("{at}: row {row}: the graph holds the circle");
            };
            assert!(
                g.owner.as_deref() == Some(CANVAS) && g.handlers.is_empty() && g.reads.is_empty(),
                "{at}: row {row}: owned by the canvas, no handler or read"
            );
            let Some(a) = nodes.iter().find(|n| n.id == *id) else {
                panic!("{at}: row {row}: actions lists the circle");
            };
            let causes: Vec<Option<Cause>> = a.verbs.iter().map(|v| v.cause).collect();
            assert!(
                causes == CIRCLE_NODE_CAUSES,
                "{at}: row {row}: click and layout free, the rest not found"
            );
            let Some(explanation) = explained(&mut session, &layout(id)) else {
                panic!("{at}: row {row}: the layout is accepted");
            };
            assert!(
                matches!(&explanation.placed.placer, Placer::App { canvas } if canvas == CANVAS),
                "{at}: row {row}: placed by the app in its canvas"
            );
        }
        for (column, call) in target_calls(&ids[0]).iter().enumerate() {
            assert!(
                refused(&mut session, call) == CIRCLE_NODE_CAUSES[column],
                "{at}: verb {column}: the call returns the listed cause"
            );
        }
        for (row, call) in [graph(), actions(), layout(&ids[0])].iter().enumerate() {
            let Ok(answer) = session.run(call) else {
                panic!("{at}: answer {row} is accepted");
            };
            let line = answer.to_json();
            assert!(
                line.contains(&ids[0]) && !line.contains("Circle 1, "),
                "{at}: answer {row} holds the id and never the name"
            );
        }

        // 80 steps, as measured: the first circle click is the 37th path, past the back
        // button's branch.
        let budget = StepBudget::new(80).expect("80 is within the bound");
        let Ok(exploration) = explore(|| Ok(circle_drawer(incremental)), budget, None) else {
            panic!("{at}: the exploration runs");
        };
        assert!(
            exploration.paths.iter().any(|path| {
                matches!(path.ending, PathEnding::Accepted { .. })
                    && path.calls.last().is_some_and(|call| {
                        call.verb == "click"
                            && id_of(call).is_some_and(|id| id.starts_with("circle-canvas///c"))
                    })
            }),
            "{at}: an explored path clicks a circle's node"
        );
    }
}

#[test]
fn the_lint_holds_no_circle_node() {
    for incremental in [false, true] {
        for n in [12_usize, 100] {
            let at = format!("{} n={n}", mode(incremental));
            let mut session = circle_drawer(incremental);
            draw(&mut session, n, &at);
            assert!(
                circle_nodes(&session).len() == n,
                "{at}: every circle reads a node"
            );
            let Some(findings) = linted(&mut session, &lint()) else {
                panic!("{at}: the lint is accepted");
            };
            assert!(
                findings.is_empty() && findings.iter().all(|f| !f.id.contains("///")),
                "{at}: no finding, none on a circle"
            );
        }
    }
}

#[test]
fn a_label_wider_than_its_circle_reads_no_truncated_finding() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        draw(&mut session, 100, &at);
        let smallest = carried(&session)
            .split(';')
            .filter_map(|entry| entry.split(',').nth(2)?.parse::<f64>().ok())
            .fold(f64::INFINITY, f64::min);
        assert!(
            smallest < 14.0,
            "{at}: the fixture holds a circle under 14 px, narrower than its two-digit label"
        );
        let Some(findings) = linted(&mut session, &lint()) else {
            panic!("{at}: the lint is accepted");
        };
        assert!(
            findings.iter().all(|f| f.id != CANVAS),
            "{at}: the label's region is the content box, so no truncated finding"
        );
    }
}

#[test]
fn each_circle_s_commands_carry_its_own_index() {
    for incremental in [false, true] {
        let at = mode(incremental);

        let mut session = circle_drawer(incremental);
        acts(&mut session, &click(CANVAS), &at);
        let text = full_text(&mut session, &at);
        let mut kinds = 0;
        for (row, line) in text.lines().enumerate() {
            let word = line.trim_start().split(' ').next().unwrap_or("");
            let expected = match word {
                "fill" | "stroke" | "glyphs" => " node=1",
                _ => " node=-",
            };
            kinds += usize::from(expected == " node=1");
            assert!(line.ends_with(expected), "{at}: line {row} reads its node");
        }
        assert!(kinds >= 3, "{at}: a fill, a stroke and the label's glyphs");

        for n in [12_usize, 100] {
            let at = format!("{at} n={n}");
            let mut session = circle_drawer(incremental);
            draw(&mut session, n, &at);
            let reading = session
                .canvas(CANVAS)
                .unwrap_or_else(|_| panic!("{at}: the canvas reads"));
            let mut fills = Vec::new();
            let mut strokes = Vec::new();
            let mut glyphs = Vec::new();
            for (command, grouped) in reading.commands.iter().zip(&reading.nodes) {
                match escher_driver::COMMAND_KINDS[command.kind()] {
                    "fill" => fills.push(*grouped),
                    "stroke" => strokes.push(*grouped),
                    "glyphs" => glyphs.push(*grouped),
                    _ => assert!(grouped.is_none(), "{at}: a layer line reads none"),
                }
            }
            assert!(
                fills.len() == n && strokes.len() == n && glyphs.len() == n,
                "{at}: one fill, stroke and glyph run per circle"
            );
            let own = (0..n)
                .filter(|&i| {
                    fills[i] == Some(i + 1) && strokes[i] == Some(i + 1) && glyphs[i] == Some(i + 1)
                })
                .count();
            assert!(own == n, "{at}: {own} of {n} circles read their own index");
        }

        let mut session = circle_drawer(incremental);
        acts(&mut session, &click(CANVAS), &at);
        acts(&mut session, &click(CANVAS), &at);
        let before = full_text(&mut session, &at);
        acts(&mut session, &click("circle-diameter"), &at);
        let after = full_text(&mut session, &at);
        let (new, gone) = line_diff(&before, &after);
        assert!(
            !new.is_empty() && !gone.is_empty(),
            "{at}: the resize changes lines"
        );
        assert!(
            new.iter()
                .chain(&gone)
                .all(|line| line.ends_with(" node=1")),
            "{at}: every changed line names circle 1"
        );

        // Every driver click lands at the canvas's centre, so circles it adds share one region and
        // the tie reads them all under the first: after an undo of c2 and a click, c3 reads 1.
        let mut session = circle_drawer(incremental);
        add_circle(&mut session, &at);
        add_circle(&mut session, &at);
        acts(&mut session, &click("circle-undo"), &at);
        acts(&mut session, &click(CANVAS), &at);
        assert!(
            circle_nodes(&session) == [node(1), node(3)],
            "{at}: c3 takes the place c2 held"
        );
        assert!(
            fill_nodes(&mut session, &at) == [Some(1), Some(1)],
            "{at}: one region, the tie to the first declared"
        );
        // Apart, the second circle reads index 2 whatever its id: c3 where c2 stood reads the
        // index c2 held, and its lines read as c2's did.
        write_circles(&mut session, "348,198,40,1;100,100,40,2", &at);
        let held = full_text(&mut session, &at);
        assert!(
            fill_nodes(&mut session, &at) == [Some(1), Some(2)],
            "{at}: c2 reads index 2"
        );
        write_circles(&mut session, "348,198,40,1;100,100,40,3", &at);
        assert!(
            circle_nodes(&session) == [node(1), node(3)],
            "{at}: c3 stands where c2 stood"
        );
        assert!(
            fill_nodes(&mut session, &at) == [Some(1), Some(2)],
            "{at}: c3 reads index 2, the index c2 held"
        );
        assert!(
            full_text(&mut session, &at) == held,
            "{at}: its lines read as c2's did"
        );
    }
}

#[test]
fn the_full_answer_and_its_parts_carry_numbers_only_in_the_tag() {
    for incremental in [false, true] {
        let at = mode(incremental);
        let mut session = circle_drawer(incremental);
        draw(&mut session, 100, &at);
        for (row, call) in [canvas_full(CANVAS), canvas_part(CANVAS, 2)]
            .iter()
            .enumerate()
        {
            let Ok(answer) = session.run(call) else {
                panic!("{at}: answer {row} is accepted");
            };
            let line = answer.to_json();
            assert!(
                !line.contains("///") && !line.contains("Circle "),
                "{at}: answer {row} holds no node id or name"
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn a_kept_check_lists_the_unclicked_circle_s_click() {
    use escher_driver::{Check, handler_coverage};
    use session_common::{record_check, state_dir};

    for incremental in [false, true] {
        let at = mode(incremental);
        // The calls of each check, then the circle clicks it reads unexercised and exercised.
        let rows: [(Vec<Call>, Vec<String>, Vec<String>); 2] = [
            (
                vec![
                    click(CANVAS),
                    advance(1000),
                    click(CANVAS),
                    advance(1000),
                    click(FIRST),
                ],
                vec![node(2)],
                vec![node(1)],
            ),
            (
                vec![click(CANVAS), advance(1000), click(CANVAS)],
                vec![node(1), node(2)],
                Vec::new(),
            ),
        ];
        assert_eq!(rows.len(), 2, "{at}: two checks");
        for (row, (calls, unexercised, exercised)) in rows.into_iter().enumerate() {
            let at = format!("{at} row {row}");
            let dir = state_dir(&format!("circle-nodes-cov-{incremental}-{row}"));
            let bytes = record_check(&dir, || circle_drawer(incremental), &calls, &at);
            let check =
                Check::from_bytes(&bytes).unwrap_or_else(|_| panic!("{at}: the check reads"));
            let Ok(coverage) = handler_coverage(&[check], || Ok(circle_drawer(incremental))) else {
                panic!("{at}: the boots start");
            };
            assert!(coverage.held == 1, "{at}: the replay holds");
            let listed = |id: &str| {
                coverage
                    .unexercised
                    .iter()
                    .find(|handler| handler.id == id && handler.event == "click")
            };
            for id in &unexercised {
                let Some(handler) = listed(id) else {
                    panic!("{at}: an unclicked circle's click reads unexercised");
                };
                assert!(
                    handler.owner.as_deref() == Some(CANVAS),
                    "{at}: owned by the canvas"
                );
            }
            for id in &exercised {
                assert!(
                    listed(id).is_none(),
                    "{at}: a clicked circle's click reads exercised"
                );
            }
            assert!(
                listed(CANVAS).is_none(),
                "{at}: the canvas's click reads exercised"
            );
        }
    }
}
