//! An inline element that holds only block children paints no box of its own, read off headless
//! frames in both layout modes: across the box its background and border would paint, the page
//! shows, while an inline element holding text with the same decorations paints its background.
//! Before this check only `layers.rs`' unit tests reached the `paints_own_box` gate.

use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, Raster};
use seven_guis::stand;

const BACKGROUND: [u8; 3] = [255, 0, 0];
const BORDER: [u8; 3] = [0, 0, 255];

const PAGE: &str = r#"<html><body style="margin:0; background:#ffffff;">
    <div style="position:absolute; left:20px; top:20px; width:300px;">
        <span id="blocks" style="background:rgb(255,0,0); border:4px solid rgb(0,0,255); padding:10px;"><div id="inner" style="width:120px; height:60px;"></div></span>
    </div>
    <div style="position:absolute; left:20px; top:200px; width:300px; font-size:20px;">
        <span id="text" style="background:rgb(255,0,0); border:4px solid rgb(0,0,255); padding:10px;">HH</span>
    </div>
</body></html>"#;

/// The pixels of `x0..x1` × `y0..y1` reading exactly `colour`.
fn count(raster: &Raster, (x0, y0): (u32, u32), (x1, y1): (u32, u32), colour: [u8; 3]) -> usize {
    (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let index = (y as usize * raster.width as usize + x as usize) * 4;
            raster.rgba[index..index + 3] == colour
        })
        .count()
}

#[test]
fn an_inline_box_of_blocks_paints_no_decorations_of_its_own() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let mut harness: Harness<HtmlDocument> =
            Harness::from_html_with(PAGE, stand::options(incremental));

        // The fixture: the first span holds only a block, the second text.
        let only_blocks = |selector: &str| {
            harness
                .base()
                .get_node(harness.node(selector))
                .expect("the span stands")
                .is_inline_with_only_block_children()
        };
        assert!(
            only_blocks("#blocks"),
            "{mode}: #blocks is an inline box of blocks"
        );
        assert!(!only_blocks("#text"), "{mode}: #text holds text");
        let rect = harness.layout_rect("#blocks");
        assert_eq!(
            (rect.x, rect.y, rect.width, rect.height),
            (20.0, 20.0, 300.0, 88.0),
            "{mode}: #blocks' box spans its block, padding and border"
        );

        let raster = harness
            .screenshot()
            .unwrap_or_else(|_| panic!("{mode}: the frame paints"));

        let (from, to) = ((20, 20), (320, 108));
        assert_eq!(
            count(&raster, from, to, BACKGROUND),
            0,
            "{mode}: #blocks paints no background"
        );
        assert_eq!(
            count(&raster, from, to, BORDER),
            0,
            "{mode}: #blocks paints no border"
        );

        let painted = count(&raster, (0, 180), (120, 250), BACKGROUND);
        assert!(
            painted >= 200,
            "{mode}: #text paints its background, {painted} pixels"
        );
    }
}
