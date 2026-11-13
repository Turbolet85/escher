//! `mix-blend-mode` and `isolation` as painted, read off headless frames in both layout modes: a box
//! under `multiply` over a coloured backdrop paints the product of the two colours, the same pair
//! at `normal` paints the box's own colour, and a `multiply` box inside an `isolation: isolate`
//! group blends with the group's empty backdrop, not with the page behind it. Before this check no
//! test reached `convert_blend_mode`'s result at the blend layer.

use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, Raster, Rect};
use seven_guis::stand;

const BACKDROP: [u8; 3] = [200, 100, 50];
const SOURCE: [u8; 3] = [100, 200, 250];
/// One unit per channel, the rounding of an 8-bit multiply and of the isolated group's alpha.
const TOLERANCE: u8 = 1;

const PAGE: &str = r#"<html><body style="margin:0; background:#ffffff;">
    <div id="under-multiply" style="position:absolute; left:0; top:0; width:200px; height:200px; background:rgb(200,100,50);">
        <div id="multiply" style="position:absolute; left:50px; top:50px; width:100px; height:100px; background:rgb(100,200,250); mix-blend-mode:multiply;"></div>
    </div>
    <div id="under-normal" style="position:absolute; left:300px; top:0; width:200px; height:200px; background:rgb(200,100,50);">
        <div id="normal" style="position:absolute; left:50px; top:50px; width:100px; height:100px; background:rgb(100,200,250); mix-blend-mode:normal;"></div>
    </div>
    <div id="under-group" style="position:absolute; left:0; top:300px; width:200px; height:200px; background:rgb(200,100,50);">
        <div id="group" style="position:absolute; left:50px; top:50px; width:100px; height:100px; isolation:isolate;">
            <div id="isolated" style="width:100px; height:100px; background:rgb(100,200,250); mix-blend-mode:multiply;"></div>
        </div>
    </div>
</body></html>"#;

fn multiply(a: [u8; 3], b: [u8; 3]) -> [u8; 3] {
    [0, 1, 2].map(|c| ((u32::from(a[c]) * u32::from(b[c]) + 127) / 255) as u8)
}

fn pixel(raster: &Raster, (x, y): (f32, f32)) -> [u8; 3] {
    let index = (y as usize * raster.width as usize + x as usize) * 4;
    [
        raster.rgba[index],
        raster.rgba[index + 1],
        raster.rgba[index + 2],
    ]
}

#[track_caller]
fn assert_near(read: [u8; 3], expected: [u8; 3], at: &str) {
    let apart = (0..3)
        .map(|c| read[c].abs_diff(expected[c]))
        .max()
        .unwrap_or(0);
    assert!(
        apart <= TOLERANCE,
        "{at}: read {read:?}, expected {expected:?} within {TOLERANCE}"
    );
}

#[track_caller]
fn assert_rect(rect: Rect, x: f32, y: f32, at: &str) {
    assert_eq!(
        (rect.x, rect.y, rect.width, rect.height),
        (x, y, 100.0, 100.0),
        "{at}: the box stands where the fixture places it"
    );
}

#[test]
fn blend_modes_and_isolation_paint_as_composited() {
    for incremental in [false, true] {
        let mode = if incremental { "incremental" } else { "full" };
        let mut harness: Harness<HtmlDocument> =
            Harness::from_html_with(PAGE, stand::options(incremental));

        // The fixture: each blended box sits wholly over its backdrop.
        assert_rect(harness.layout_rect("#multiply"), 50.0, 50.0, mode);
        assert_rect(harness.layout_rect("#normal"), 350.0, 50.0, mode);
        assert_rect(harness.layout_rect("#isolated"), 50.0, 350.0, mode);

        let raster = harness
            .screenshot()
            .unwrap_or_else(|_| panic!("{mode}: the frame paints"));

        // The backdrop paints its own colour beside each box.
        assert_near(
            pixel(&raster, (20.0, 20.0)),
            BACKDROP,
            &format!("{mode}: backdrop"),
        );
        assert_near(
            pixel(&raster, (320.0, 20.0)),
            BACKDROP,
            &format!("{mode}: normal backdrop"),
        );
        assert_near(
            pixel(&raster, (20.0, 320.0)),
            BACKDROP,
            &format!("{mode}: group backdrop"),
        );

        assert_near(
            pixel(&raster, (100.0, 100.0)),
            multiply(BACKDROP, SOURCE),
            &format!("{mode}: multiply"),
        );
        assert_near(
            pixel(&raster, (400.0, 100.0)),
            SOURCE,
            &format!("{mode}: normal"),
        );
        assert_near(
            pixel(&raster, (100.0, 400.0)),
            SOURCE,
            &format!("{mode}: isolated multiply"),
        );
    }
}
