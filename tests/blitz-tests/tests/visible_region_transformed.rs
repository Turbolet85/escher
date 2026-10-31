//! `BaseDocument::visible_region` reads a clipping box through its transforms, in every document,
//! on the shape a pan/zoom surface takes: a transformed world of absolutely positioned nodes
//! under an untransformed `overflow: hidden` viewport. On parsed pages booted with the stand's
//! options, in both layout modes. At a translate, at a translate with `scale(2)` and at
//! `scale(0.5)`, a node's centre lies inside its region and a hit there answers it, and a node
//! panned out lies outside and is not hit. Where the box that clips lies under the transform, a
//! node body's `overflow: auto` inside the world, the region follows the box to where it shows.
//! Before the fix (measured 2026-10-31) that inner box read where layout puts it, so a target it
//! showed read outside its own region. A page with no transform reads as it did. A clipping box
//! under `scale(0)` or a matrix of zeroes reads nothing visible, with no panic. Under a rotation
//! the region is the bounding box of the rotated padding box, so a corner it does not show reads
//! in view: pinned as stated. A failure message carries the layout mode and a row, never a
//! coordinate.

use blitz_html::HtmlDocument;
use blitz_test_harness::Harness;
use seven_guis::stand;

type Rect = (f64, f64, f64, f64);

fn page(html: &str, incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(html, stand::options(incremental))
}

/// A 500 × 400 clip at (50, 30) holding a world moved by `transform`, itself holding a node near
/// its corner and a node far from it.
fn world(transform: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div id="viewport" style="position:absolute; left:50px; top:30px; width:500px; height:400px; overflow:hidden">
        <div id="world" style="position:absolute; left:0; top:0; width:1000px; height:1000px; transform-origin:0 0; transform:{transform}">
            <div id="near" style="position:absolute; left:40px; top:40px; width:60px; height:40px"></div>
            <div id="far" style="position:absolute; left:900px; top:900px; width:60px; height:40px"></div>
        </div>
    </div>
</body></html>"#
    )
}

/// The same clip and world, a node body inside the world that scrolls what it holds: a target
/// shown at its top and a node scrolled out below.
fn node_body(transform: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div id="viewport" style="position:absolute; left:50px; top:30px; width:500px; height:400px; overflow:hidden">
        <div id="world" style="position:absolute; left:0; top:0; width:1000px; height:1000px; transform-origin:0 0; transform:{transform}">
            <div id="body" style="position:absolute; left:20px; top:10px; width:200px; height:100px; overflow:auto">
                <div id="target" style="width:100px; height:40px"></div>
                <div style="height:400px"></div>
                <div id="below" style="width:100px; height:40px"></div>
            </div>
        </div>
    </div>
</body></html>"#
    )
}

/// A 200 × 100 box with a 2px border at (100, 100), scrolling what it holds, styled by `style`.
fn scrolling_box(style: &str) -> String {
    format!(
        r#"<html><body style="margin:0">
    <div id="box" style="position:absolute; left:100px; top:100px; width:200px; height:100px; border:2px solid black; overflow:auto; {style}">
        <div id="target" style="width:100px; height:40px"></div>
        <div style="height:400px"></div>
    </div>
</body></html>"#
    )
}

fn rect(harness: &Harness<HtmlDocument>, selector: &str) -> Rect {
    let seen = harness
        .base()
        .get_client_bounding_rect(harness.node(selector))
        .expect("the element has a client rect");
    (seen.x, seen.y, seen.width, seen.height)
}

fn region(harness: &Harness<HtmlDocument>, selector: &str) -> Option<Rect> {
    harness
        .base()
        .visible_region(harness.node(selector))
        .map(|seen| (seen.x, seen.y, seen.width, seen.height))
}

fn centre((x, y, width, height): Rect) -> (f64, f64) {
    (x + width / 2.0, y + height / 2.0)
}

fn inside(point: (f64, f64), (x, y, width, height): Rect) -> bool {
    point.0 >= x && point.0 <= x + width && point.1 >= y && point.1 <= y + height
}

/// Whether a hit at `point` answers `selector`.
fn hits(harness: &Harness<HtmlDocument>, point: (f64, f64), selector: &str) -> bool {
    harness
        .hit(point.0 as f32, point.1 as f32)
        .is_some_and(|hit| hit.node_id == harness.node(selector))
}

/// Asserts that `selector`'s centre lies inside its region and a hit there answers it.
#[track_caller]
fn assert_shown(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) {
    let point = centre(rect(harness, selector));
    assert!(
        region(harness, selector).is_some_and(|seen| inside(point, seen)),
        "{mode}: the node's centre lies inside its region"
    );
    assert!(hits(harness, point, selector), "{mode}: the node is hit");
}

/// Asserts that `selector`'s centre lies outside its region and a hit there does not answer it.
#[track_caller]
fn assert_panned_out(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) {
    let point = centre(rect(harness, selector));
    assert!(
        region(harness, selector).is_none_or(|seen| !inside(point, seen)),
        "{mode}: the node's centre lies outside its region"
    );
    assert!(
        !hits(harness, point, selector),
        "{mode}: the node is not hit"
    );
}

/// The world's transforms a pan and a zoom take.
const WORLD_ROWS: [&str; 3] = [
    "translate(50px, 20px)",
    "translate(50px, 20px) scale(2)",
    "scale(0.5)",
];

#[test]
fn a_world_under_an_untransformed_viewport_shows_a_node_and_pans_one_out() {
    assert_eq!(WORLD_ROWS.len(), 3);
    for incremental in [false, true] {
        for (row, transform) in WORLD_ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let harness = page(&world(transform), incremental);
            assert!(
                harness
                    .base()
                    .get_node(harness.node("#world"))
                    .is_some_and(|world| world.transform().is_some()),
                "{mode}: the world computes a transform"
            );
            assert_shown(&harness, "#near", &mode);
            assert_panned_out(&harness, "#far", &mode);
        }
    }
}

/// The world's transforms under which a node body's clip, where layout puts it, holds no part of
/// the target it shows, and so tell the fix from the reading before it.
const NODE_BODY_ROWS: [&str; 2] = [
    "translate(300px, 150px)",
    "translate(200px, 100px) scale(2)",
];

#[test]
fn a_target_in_a_node_body_under_a_transformed_world_reads_where_it_shows() {
    assert_eq!(NODE_BODY_ROWS.len(), 2);
    for incremental in [false, true] {
        for (row, transform) in NODE_BODY_ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let harness = page(&node_body(transform), incremental);
            let (_, _, width, height) = rect(&harness, "#body");
            assert!(
                width > 0.0 && height > 0.0,
                "{mode}: the node body shows with an area"
            );
            assert_shown(&harness, "#target", &mode);
            assert_panned_out(&harness, "#below", &mode);
        }
    }
}

#[test]
fn a_page_with_no_transform_reads_its_padding_box_in_the_client_rects_frame() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = page(&scrolling_box(""), incremental);
        let (x, y, width, height) = rect(&harness, "#box");
        let bar = harness
            .base()
            .get_node(harness.node("#box"))
            .map(|node| node.final_layout().scrollbar_size)
            .expect("the box is a node");
        assert!(
            region(&harness, "#target")
                == Some((
                    x + 2.0,
                    y + 2.0,
                    width - 4.0 - f64::from(bar.width),
                    height - 4.0 - f64::from(bar.height),
                )),
            "{mode}: the region is the box's padding box less its scrollbar"
        );
    }
}

#[test]
fn a_translate_moves_the_region_by_the_translation() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let plain = page(&scrolling_box(""), incremental);
        let moved = page(
            &scrolling_box("transform:translate(150px, 75px)"),
            incremental,
        );
        let (x, y, width, height) = region(&plain, "#target").expect("the plain page reads one");
        assert!(
            region(&moved, "#target") == Some((x + 150.0, y + 75.0, width, height)),
            "{mode}: the region moves as the box does, its size the same"
        );
    }
}

/// Transforms that map a box to a point.
const DEGENERATE_ROWS: [&str; 2] = ["scale(0)", "matrix(0, 0, 0, 0, 0, 0)"];

#[test]
fn a_box_mapped_to_a_point_leaves_nothing_visible_inside_it() {
    assert_eq!(DEGENERATE_ROWS.len(), 2);
    for incremental in [false, true] {
        for (row, transform) in DEGENERATE_ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let harness = page(
                &scrolling_box(&format!("transform:{transform}")),
                incremental,
            );
            assert!(
                harness
                    .base()
                    .get_node(harness.node("#box"))
                    .is_some_and(|node| node.transform().is_some()),
                "{mode}: the box computes a transform"
            );
            assert!(
                region(&harness, "#target").is_none(),
                "{mode}: nothing inside it can be seen"
            );
        }
    }
}

/// The stated over-reading: the region is the bounding box of the rotated padding box, and so
/// holds a corner the box does not show.
#[test]
fn under_a_rotation_the_region_reads_the_rotated_boxs_bounding_box() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = page(
            &scrolling_box("border:none; transform:rotate(45deg)"),
            incremental,
        );
        let bounds = rect(&harness, "#box");
        let seen = region(&harness, "#target").expect("the rotated box shows");
        let close = |a: f64, b: f64| (a - b).abs() < 0.5;
        assert!(
            close(seen.0, bounds.0)
                && close(seen.1, bounds.1)
                && close(seen.2, bounds.2)
                && close(seen.3, bounds.3),
            "{mode}: the region is the rotated box's bounding box"
        );
        let corner = (seen.0 + 1.0, seen.1 + 1.0);
        assert!(
            inside(corner, seen),
            "{mode}: the region holds its own corner"
        );
        assert!(
            !hits(&harness, corner, "#box") && !hits(&harness, corner, "#target"),
            "{mode}: the box shows nothing at that corner"
        );
    }
}
