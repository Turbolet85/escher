//! A box under a transform that cannot be inverted takes no hit, nor does anything it holds, in
//! every document: paint shows nothing there, so a hit answers what lies beneath. Before the fix
//! (measured 2026-10-31) `scale(0)` and a matrix of zeroes with no clip of their own answered a
//! hit at points far from where they show nothing, the inverse's NaN coordinates passing the
//! bounds test. An invertible twin is hit as it was. On parsed pages booted with the stand's
//! options, in both layout modes. A failure message carries the layout mode and a row, never a
//! coordinate.

use blitz_html::HtmlDocument;
use blitz_test_harness::Harness;
use seven_guis::stand;

/// A 600 × 400 clip at the page's corner holding a box styled by `transform`, with a child.
fn page(transform: &str, incremental: bool) -> Harness<HtmlDocument> {
    let html = format!(
        r#"<html><body style="margin:0">
    <div id="clip" style="width:600px; height:400px; overflow:hidden">
        <div id="box" style="width:400px; height:300px; transform:{transform}">
            <div id="child" style="width:100px; height:100px"></div>
        </div>
    </div>
</body></html>"#
    );
    Harness::from_html_with(&html, stand::options(incremental))
}

/// Points of the clip that lie far from where a box mapped to a point shows nothing.
const POINTS: [(f32, f32); 2] = [(300.0, 200.0), (590.0, 390.0)];

/// Transforms with no inverse.
const NON_INVERTIBLE: [&str; 2] = ["scale(0)", "matrix(0, 0, 0, 0, 0, 0)"];

#[test]
fn a_box_under_a_non_invertible_transform_takes_no_hit() {
    assert_eq!(NON_INVERTIBLE.len(), 2);
    for incremental in [false, true] {
        for (row, transform) in NON_INVERTIBLE.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let harness = page(transform, incremental);
            let [clip, boxed] = ["#clip", "#box"].map(|selector| harness.node(selector));
            assert!(
                harness
                    .base()
                    .get_node(boxed)
                    .is_some_and(|node| node.transform().is_some()),
                "{mode}: the box computes a transform"
            );
            for (index, (x, y)) in POINTS.into_iter().enumerate() {
                assert!(
                    harness.hit(x, y).map(|hit| hit.node_id) == Some(clip),
                    "{mode}: point {index}: the hit answers the clip and not the box"
                );
            }
        }
    }
}

#[test]
fn an_invertible_twin_is_hit_where_it_shows() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = page("scale(2)", incremental);
        let boxed = harness.node("#box");
        for (index, (x, y)) in POINTS.into_iter().enumerate() {
            assert!(
                harness.hit(x, y).map(|hit| hit.node_id) == Some(boxed),
                "{mode}: point {index}: the hit answers the box"
            );
        }
    }
}
