//! `BaseDocument::layout_reason` reads why an element sits where it does at its size, on fixture
//! pages in both layout modes: every placing word (`PlacedBy`) on a node of the placing table,
//! with its container and offset, and every size cause (`SizeCause`) on a node of the size
//! table. A scrolled holder reads a `Scroll` shift and leaves the holder's own bounds unmoved; a
//! translated holder reads a `Transform` shift with the bounds the client rect reads, and a
//! `scale(0)` holder reads without a panic; a `position: relative` box reads its inset inside its
//! offset; an inline-block held by an anonymous block reads the element as its container; and a
//! stale id, a `display: none` element and a text node read none. Every number asserted is on a
//! box whose size the page states in px, so none depends on a font. A failure message names a
//! layout mode and a row index only.

use blitz_dom::{LayoutReason, PlacedBy, ShiftBy, SizeCause};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::node_id::NodeId;

fn boot(page: &str, incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(
        page,
        HarnessOptions {
            incremental: Some(incremental),
            ..HarnessOptions::default()
        },
    )
}

#[track_caller]
fn node(harness: &Harness<HtmlDocument>, selector: &str, at: &str) -> NodeId {
    let node = harness.query(selector);
    assert!(node.is_some(), "{at}: the selector resolves");
    node.unwrap()
}

#[track_caller]
fn reason(harness: &Harness<HtmlDocument>, selector: &str, at: &str) -> LayoutReason {
    let reason = harness.base().layout_reason(node(harness, selector, at));
    assert!(reason.is_some(), "{at}: the element reads a reason");
    reason.unwrap()
}

/// One box per placing word, each in a container whose offsets the page states.
const PLACING: &str = r#"
<style>
  body { margin: 0; }
  div { box-sizing: border-box; }
</style>
<div id="block-parent" style="width: 400px; padding: 10px">
  <div id="block" style="height: 30px"></div>
</div>
<div id="flex-parent" style="display: flex; width: 300px; height: 50px; padding-left: 5px">
  <div id="flex-item" style="width: 40px; height: 20px"></div>
</div>
<div id="grid-parent" style="display: grid; grid-template-columns: 100px 100px; width: 200px">
  <div style="height: 20px"></div>
  <div id="grid-item" style="height: 20px"></div>
</div>
<table id="table" style="border-spacing: 0; border: 0">
  <tr><td id="cell" style="padding: 0"><div style="width: 40px; height: 20px"></div></td></tr>
</table>
<div id="line" style="width: 300px; font-size: 0; line-height: 0"><span id="atom" style="display: inline-block; width: 40px; height: 20px; vertical-align: top"></span></div>
<p id="text-line" style="margin: 0; width: 300px">Words <span id="span">spanned</span></p>
<div id="float-parent" style="width: 300px; height: 20px">
  <div id="float" style="float: right; width: 50px; height: 20px"></div>
</div>
<div id="abs-parent" style="position: relative; width: 200px; height: 100px">
  <div id="abs" style="position: absolute; left: 10px; top: 5px; width: 20px; height: 20px"></div>
</div>
<div id="fixed" style="position: fixed; left: 7px; top: 9px; width: 10px; height: 10px"></div>
<select id="select">
  <option id="chosen">A</option>
  <option id="other">B</option>
</select>
"#;

/// A row of the placing table: the node, its word, its container and its offset there; the
/// list opened first for a row of an open select list; an offset of `None` where the box's
/// place depends on a font (a non-atomic inline's line fragments).
struct PlacingRow {
    node: &'static str,
    by: PlacedBy,
    container: Option<&'static str>,
    at: Option<(f64, f64)>,
    open_list: bool,
}

const fn row(
    node: &'static str,
    by: PlacedBy,
    container: Option<&'static str>,
    at: Option<(f64, f64)>,
) -> PlacingRow {
    PlacingRow {
        node,
        by,
        container,
        at,
        open_list: false,
    }
}

const PLACING_ROWS: [PlacingRow; 13] = [
    row(
        "#block",
        PlacedBy::Block,
        Some("#block-parent"),
        Some((10.0, 10.0)),
    ),
    row(
        "#flex-item",
        PlacedBy::Flex,
        Some("#flex-parent"),
        Some((5.0, 0.0)),
    ),
    row(
        "#grid-item",
        PlacedBy::Grid,
        Some("#grid-parent"),
        Some((100.0, 0.0)),
    ),
    row("#cell", PlacedBy::Table, Some("#table"), Some((0.0, 0.0))),
    row("#atom", PlacedBy::Inline, Some("#line"), Some((0.0, 0.0))),
    row("#span", PlacedBy::Inline, Some("#text-line"), None),
    row(
        "#float",
        PlacedBy::Float,
        Some("#float-parent"),
        Some((250.0, 0.0)),
    ),
    row(
        "#abs",
        PlacedBy::Absolute,
        Some("#abs-parent"),
        Some((10.0, 5.0)),
    ),
    row("#fixed", PlacedBy::Fixed, Some("html"), Some((7.0, 9.0))),
    row("html", PlacedBy::Root, None, Some((0.0, 0.0))),
    row(
        "#other",
        PlacedBy::Select,
        Some("#select"),
        Some((0.0, 0.0)),
    ),
    PlacingRow {
        node: "#other",
        by: PlacedBy::SelectList,
        container: Some("#select"),
        at: None,
        open_list: true,
    },
    row("body", PlacedBy::Block, Some("html"), Some((0.0, 0.0))),
];

#[test]
fn every_placing_word_reads_on_its_fixture_node() {
    let words: std::collections::HashSet<String> = PLACING_ROWS
        .iter()
        .map(|row| format!("{:?}", row.by))
        .collect();
    assert_eq!(PLACING_ROWS.len(), 13);
    assert_eq!(words.len(), 11, "every placing word has a row");
    for incremental in [false, true] {
        for (index, row) in PLACING_ROWS.iter().enumerate() {
            let at = format!("incremental={incremental} row {index}");
            let mut harness = boot(PLACING, incremental);
            if row.open_list {
                harness.click("#select");
                assert!(
                    harness.base().open_select().is_some(),
                    "{at}: the list is open"
                );
            }
            let reason = reason(&harness, row.node, &at);
            assert!(reason.placed.by == row.by, "{at}: the placing word");
            let container = row.container.map(|selector| node(&harness, selector, &at));
            assert!(reason.placed.container == container, "{at}: the container");
            if let Some(offset) = row.at {
                assert!(reason.placed.at == offset, "{at}: the offset");
            }
            let shown = harness
                .base()
                .select_list_bounds(node(&harness, row.node, &at));
            if shown.is_none() && row.by != PlacedBy::Select {
                assert!(
                    Some(reason.bounds)
                        == harness
                            .base()
                            .get_client_bounding_rect(node(&harness, row.node, &at)),
                    "{at}: the bounds are the client rect"
                );
            }
        }
    }
}

/// One box per size cause, each sized by what the page states in px.
const SIZING: &str = r#"
<style>
  body { margin: 0; }
  div { box-sizing: border-box; }
</style>
<div style="width: 400px">
  <div id="styled" style="height: 30px"></div>
  <div id="min" style="width: 20px; min-width: 60px; height: 10px"></div>
  <div id="max" style="max-width: 70px; height: 10px"></div>
</div>
<div style="display: flex; width: 300px; height: 50px">
  <div id="grow" style="flex: 1 1 0px; height: 20px"></div>
</div>
<div style="display: flex; width: 300px; height: 50px">
  <div id="shrink" style="flex: 0 1 400px"></div>
</div>
<div style="display: grid; grid-template-columns: 100px; width: 200px">
  <div id="grid-cell" style="height: 20px"></div>
</div>
<table style="border-spacing: 0">
  <tr><td id="cell" style="padding: 0"><div style="width: 40px; height: 20px"></div></td></tr>
</table>
<input id="text-input">
<div style="width: 300px">
  <div id="float" style="float: left; width: 50px"><div style="height: 20px"></div></div>
</div>
<div style="display: flex; width: 100px; height: 10px">
  <div id="overflowing" style="width: 200px"></div>
</div>
"#;

const SIZE_ROWS: [(&str, bool, SizeCause); 11] = [
    ("#styled", false, SizeCause::Style),
    ("#min", true, SizeCause::Min),
    ("#max", true, SizeCause::Max),
    ("#grow", true, SizeCause::FlexGrow),
    ("#shrink", true, SizeCause::FlexShrink),
    ("#shrink", false, SizeCause::Stretch),
    ("#grid-cell", true, SizeCause::Grid),
    ("#cell", true, SizeCause::Table),
    ("#text-input", true, SizeCause::Replaced),
    ("#float", false, SizeCause::Content),
    ("#overflowing", true, SizeCause::Layout),
];

#[test]
fn every_size_cause_reads_on_its_fixture_node() {
    let causes: std::collections::HashSet<String> = SIZE_ROWS
        .iter()
        .map(|(_, _, cause)| format!("{cause:?}"))
        .collect();
    assert_eq!(SIZE_ROWS.len(), 11);
    assert_eq!(causes.len(), 11, "every size cause has a row");
    for incremental in [false, true] {
        let harness = boot(SIZING, incremental);
        for (index, (selector, width, cause)) in SIZE_ROWS.iter().enumerate() {
            let at = format!("incremental={incremental} row {index}");
            let reason = reason(&harness, selector, &at);
            let axis = if *width { reason.width } else { reason.height };
            assert!(axis.by == *cause, "{at}: the size cause");
        }
    }
}

#[test]
fn the_stated_px_and_inputs_read_beside_each_cause() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(SIZING, incremental);
        let min = reason(&harness, "#min", &at).width;
        assert!(min.px == 60.0, "{at}: the minimum wins");
        let grow = reason(&harness, "#grow", &at).width;
        assert!(grow.px == 300.0, "{at}: the item grows to its line");
        let flex = grow.flex;
        assert!(
            flex.is_some_and(|flex| flex.grow == 1.0 && flex.shrink == 1.0),
            "{at}: the flex inputs"
        );
        let stretched = reason(&harness, "#shrink", &at).height;
        assert!(stretched.px == 50.0, "{at}: the item stretches across");
        assert!(
            stretched.flex.is_none(),
            "{at}: no flex inputs off the main axis"
        );
    }
}

#[test]
fn a_scrolled_holder_reads_a_scroll_shift_and_its_own_bounds_unmoved() {
    let page = r#"
<style> body { margin: 0; } </style>
<div id="scroller" style="width: 100px; height: 50px; overflow: auto">
  <div id="content" style="height: 200px"><div id="deep" style="height: 10px; margin-top: 150px"></div></div>
</div>
"#;
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut harness = boot(page, incremental);
        let before = reason(&harness, "#scroller", &at).bounds;
        let deep = node(&harness, "#deep", &at);
        harness.scroll_into_view(deep);
        let scroller = node(&harness, "#scroller", &at);
        let offset = harness
            .base()
            .get_node(scroller)
            .map(|n| n.scroll_offset().y);
        assert!(
            offset.is_some_and(|y| y > 0.0),
            "{at}: the holder is scrolled"
        );
        let content = reason(&harness, "#content", &at);
        let shift = content
            .shifts
            .iter()
            .find(|shift| shift.by == ShiftBy::Scroll && shift.holder == Some(scroller));
        assert!(
            shift.is_some_and(|shift| Some(-shift.dy) == offset),
            "{at}: the holder's scroll shift"
        );
        assert!(
            content.bounds.y == content.placed.at.1 + shift.map_or(0.0, |s| s.dy),
            "{at}: the scroll moves the content from its offset"
        );
        let after = reason(&harness, "#scroller", &at);
        assert!(
            after.bounds == before,
            "{at}: the holder's own bounds stand"
        );
        assert!(
            after
                .shifts
                .iter()
                .all(|shift| shift.holder != Some(scroller)),
            "{at}: the holder's own scroll moves no shift of its own"
        );
    }
}

#[test]
fn a_transformed_holder_reads_a_transform_shift() {
    let page = r#"
<style> body { margin: 0; } div { box-sizing: border-box; } </style>
<div id="moved" style="transform: translateX(30px); width: 100px; height: 40px">
  <div id="inner" style="height: 10px"></div>
</div>
<div id="flat" style="transform: scale(0); width: 100px; height: 40px">
  <div id="flat-inner" style="height: 10px"></div>
</div>
"#;
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(page, incremental);
        let moved = node(&harness, "#moved", &at);
        for (index, selector) in ["#inner", "#flat-inner"].into_iter().enumerate() {
            let at = format!("{at} case {index}");
            let reason = reason(&harness, selector, &at);
            let id = node(&harness, selector, &at);
            assert!(
                Some(reason.bounds) == harness.base().get_client_bounding_rect(id),
                "{at}: the bounds are the client rect"
            );
            assert!(
                reason
                    .shifts
                    .iter()
                    .any(|shift| shift.by == ShiftBy::Transform),
                "{at}: a transform shift"
            );
        }
        let inner = reason(&harness, "#inner", &at);
        assert!(
            inner
                .shifts
                .iter()
                .any(|shift| shift.by == ShiftBy::Transform && shift.holder == Some(moved)),
            "{at}: the translated holder moves it"
        );
        assert!(inner.bounds.x == 30.0, "{at}: the bounds are moved");
        assert!(inner.laid_out.x == 0.0, "{at}: layout put it at the left");
    }
}

#[test]
fn a_relative_box_reads_its_inset_inside_its_offset() {
    let page = r#"
<style> body { margin: 0; } </style>
<div id="holder" style="width: 200px; padding-top: 20px">
  <div id="rel" style="position: relative; left: 15px; top: -5px; width: 10px; height: 10px"></div>
</div>
"#;
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(page, incremental);
        let rel = reason(&harness, "#rel", &at);
        assert!(rel.placed.relative == Some((15.0, -5.0)), "{at}: the inset");
        assert!(
            rel.placed.at == (15.0, 15.0),
            "{at}: the offset includes it"
        );
        let still = reason(&harness, "#holder", &at);
        assert!(
            still.placed.relative.is_none(),
            "{at}: a static box reads none"
        );
    }
}

#[test]
fn a_box_held_by_an_anonymous_block_reads_the_element_as_its_container() {
    let page = r#"
<style> body { margin: 0; } </style>
<div id="parent" style="width: 200px">
  <span id="atom" style="display: inline-block; width: 10px; height: 10px"></span>text
  <div style="height: 5px"></div>
</div>
"#;
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = boot(page, incremental);
        let atom = node(&harness, "#atom", &at);
        let held_by = harness
            .base()
            .get_node(atom)
            .and_then(|n| n.containing_block())
            .and_then(|id| harness.base().get_node(id).map(|n| n.is_anonymous()));
        assert!(held_by == Some(true), "{at}: an anonymous block holds it");
        let reason = reason(&harness, "#atom", &at);
        assert!(
            reason.placed.container == Some(node(&harness, "#parent", &at)),
            "{at}: the element is its container"
        );
    }
}

#[test]
fn a_stale_id_a_hidden_element_and_a_text_node_read_none() {
    let page = r#"
<div id="hidden" style="display: none"><div id="hidden-child"></div></div>
<p id="text">Words</p>
<div id="gone"></div>
"#;
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut harness = boot(page, incremental);
        let gone = node(&harness, "#gone", &at);
        harness.base_mut().mutate().remove_node(gone);
        harness.pump();
        let text = harness
            .base()
            .get_node(node(&harness, "#text", &at))
            .and_then(|p| p.children.first().copied());
        assert!(text.is_some(), "{at}: the paragraph holds a text node");
        let ids = [
            gone,
            node(&harness, "#hidden", &at),
            node(&harness, "#hidden-child", &at),
            text.unwrap(),
        ];
        for (index, id) in ids.into_iter().enumerate() {
            assert!(
                harness.base().layout_reason(id).is_none(),
                "{at} case {index}: none"
            );
        }
    }
}
