//! What an open `select` list paints, read off frames on one host, in both layout modes: its plain
//! pair through `Harness::text_contrast` at the default sheet's colours and at the stand's, the
//! highlight and a disabled row as painted within 1 per channel against the engine's constants,
//! each ratio through the one contrast reader, no second formula; its border; a list under an
//! `opacity` ancestor and past an `overflow: hidden` box painted whole; a closed select's frame
//! byte-equal before opening and after a close with no choice; a group's header darker than a row
//! by weight; a scroll thumb that paints and takes no hit; the highlight's pair the default sheet's
//! `multiple` chosen-row pair, read by `Harness::highlight_contrast` on the highlighted row alone;
//! and a larger authored font's glyphs painted wholly inside their row. A failure message carries
//! the layout mode and a row index.

use ::keyboard_types::{Key, NamedKey};
use blitz_dom::{BoundingRect, SelectListView};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions, Raster};
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::{self, LeanTask};

mod fixtures;

type Page = Harness<DioxusDocument>;

/// A large font, so that a glyph's stems hold pixels of its whole colour; a disabled row; a page
/// colour nothing in the list paints; and the far box `fx-beside`.
fn page() -> Element {
    rsx! {
        div { id: "fx-root", style: "background: #204060; width: 760px; height: 560px;",
            select { id: "fx-pick", style: "font-size: 28px;",
                option { id: "fx-apple", value: "a", "HHHH" }
                option { id: "fx-banana", value: "b", selected: true, "HHHH" }
                option { id: "fx-off", value: "k", disabled: true, "HHHH" }
            }
        }
    }
}

fn enter() -> Key {
    Key::Named(NamedKey::Enter)
}

/// `selector`'s select focused by no click, then opened by Enter, so that no pointer state paints.
fn open_by_key(harness: &mut Page, selector: &str) {
    let select = harness.node(selector);
    harness.base_mut().set_focus_to(select);
    harness.pump();
    harness.press(enter());
}

fn frame(harness: &mut Page, at: &str) -> Raster {
    harness
        .screenshot()
        .unwrap_or_else(|_| panic!("{at}: the frame paints"))
}

fn pixel(raster: &Raster, x: f64, y: f64) -> [u8; 3] {
    let index = (y as usize * raster.width as usize + x as usize) * 4;
    [
        raster.rgba[index],
        raster.rgba[index + 1],
        raster.rgba[index + 2],
    ]
}

fn row_rect(harness: &Page, selector: &str, at: &str) -> BoundingRect {
    harness
        .base()
        .select_list_bounds(harness.node(selector))
        .unwrap_or_else(|| panic!("{at}: the row reads a box"))
}

/// The pixel of `rect` furthest from `from`, summing the channels' distance: the row's text at its
/// full colour where a glyph's stem covers a pixel whole.
fn furthest_from(raster: &Raster, rect: BoundingRect, from: [u8; 3]) -> [u8; 3] {
    let mut furthest = from;
    let mut distance = 0;
    let (x0, y0) = (rect.x.ceil() as u32, rect.y.ceil() as u32);
    let (x1, y1) = (
        (rect.x + rect.width).floor() as u32,
        (rect.y + rect.height).floor() as u32,
    );
    for y in y0..y1.min(raster.height) {
        for x in x0..x1.min(raster.width) {
            let read = pixel(raster, f64::from(x), f64::from(y));
            let apart: u32 = (0..3)
                .map(|channel| u32::from(read[channel].abs_diff(from[channel])))
                .sum();
            if apart > distance {
                distance = apart;
                furthest = read;
            }
        }
    }
    furthest
}

#[track_caller]
fn assert_within_one(read: [u8; 3], expected: [u8; 3], at: &str) {
    for channel in 0..3 {
        assert!(
            read[channel].abs_diff(expected[channel]) <= 1,
            "{at}: channel {channel} reads {read:?}, the constant is {expected:?}"
        );
    }
}

/// The WCAG ratio of a pair, through the one reader: a page painting the pair in computed styles.
fn ratio_of(text: [u8; 3], background: [u8; 3]) -> f32 {
    let css = |[r, g, b]: [u8; 3]| format!("rgb({r}, {g}, {b})");
    let html = format!(
        r#"<html><body><p id="pair" style="color: {}; background: {}">x</p></body></html>"#,
        css(text),
        css(background)
    );
    let harness = Harness::<HtmlDocument>::from_html_with(&html, HarnessOptions::default());
    harness
        .text_contrast(harness.node("#pair"))
        .map_or(0.0, |contrast| contrast.ratio)
}

#[test]
fn the_highlight_and_a_disabled_row_read_as_painted_against_their_constants() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        open_by_key(&mut harness, "#fx-pick");
        let raster = frame(&mut harness, &at);

        let highlighted = row_rect(&harness, "#fx-banana", &at);
        let back = pixel(
            &raster,
            highlighted.x + highlighted.width - 2.0,
            highlighted.y + 1.0,
        );
        assert_within_one(back, SelectListView::HIGHLIGHT_BACKGROUND, &at);
        let text = furthest_from(&raster, highlighted, back);
        assert_within_one(text, SelectListView::HIGHLIGHT_TEXT, &at);
        let ratio = ratio_of(text, back);
        assert!(
            (ratio - 12.56).abs() < 0.2,
            "{at}: the highlight reads {ratio}"
        );

        let disabled = row_rect(&harness, "#fx-off", &at);
        let back = pixel(&raster, disabled.x + disabled.width - 2.0, disabled.y + 1.0);
        assert_within_one(back, [255, 255, 255], &at);
        let text = furthest_from(&raster, disabled, back);
        assert_within_one(text, SelectListView::DISABLED_TEXT, &at);
        let ratio = ratio_of(text, back);
        assert!(ratio >= 4.5, "{at}: the disabled row reads {ratio}");

        let view = harness.base().select_list_view();
        let view = view.unwrap_or_else(|| panic!("{at}: paint reads the list"));
        let border = pixel(
            &raster,
            view.bounds.x + 0.5,
            view.bounds.y + view.bounds.height / 2.0,
        );
        assert_within_one(border, SelectListView::BORDER, &at);
    }
}

#[test]
fn the_plain_pair_is_measured_at_the_default_sheet_and_at_the_stand() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        open_by_key(&mut harness, "#fx-pick");
        let plain = harness.text_contrast(harness.node("#fx-apple"));
        let plain = plain.unwrap_or_else(|| panic!("{at}: the default sheet's pair is measured"));
        assert!(plain.ratio >= plain.required_ratio(), "{at}: default sheet");

        let mut flight = stand::boot(LeanTask::FlightBooker, stand::options(incremental));
        flight.click("#flight-type");
        let doc_open = flight.base().open_select();
        assert_eq!(
            doc_open,
            Some(flight.node("#flight-type")),
            "{at}: it opens"
        );
        let raster = frame(&mut flight, &at);
        let rect = row_rect(&flight, "#flight-type-return", &at);
        let plain = flight.text_contrast(flight.node("#flight-type-return"));
        let plain = plain.unwrap_or_else(|| panic!("{at}: the stand's pair is measured"));
        assert!(plain.ratio >= plain.required_ratio(), "{at}: the stand");
        let back = pixel(&raster, rect.x + rect.width - 2.0, rect.y + 1.0);
        assert_within_one(back, [255, 255, 255], &at);
    }
}

/// A select under an `opacity` ancestor, another past an `overflow: hidden` box, on a page colour
/// nothing in a list paints.
fn clipped_page() -> Element {
    rsx! {
        div { id: "fx-root", style: "background: #204060; width: 760px; height: 560px;",
            div { style: "opacity: 0.4; width: 300px;",
                select { id: "fx-faded",
                    option { id: "fx-faded-a", value: "a", "Alpha" }
                    option { id: "fx-faded-b", value: "b", "Beta" }
                }
            }
            div { style: "overflow: hidden; width: 300px; height: 40px; margin-top: 100px;",
                select { id: "fx-cut",
                    option { id: "fx-cut-a", value: "a", "Alpha" }
                    option { id: "fx-cut-b", value: "b", "Beta" }
                    option { id: "fx-cut-c", value: "c", "Gamma" }
                }
            }
        }
    }
}

#[test]
fn a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select() {
    let rows = [("#fx-faded", "#fx-faded-b"), ("#fx-cut", "#fx-cut-c")];
    assert_eq!(rows.len(), 2);
    for incremental in [false, true] {
        for (row, (select, option)) in rows.into_iter().enumerate() {
            let at = format!("incremental {incremental}: row {row}");
            let mut harness = fixtures::boot(clipped_page, incremental);
            open_by_key(&mut harness, select);
            let raster = frame(&mut harness, &at);
            let rect = row_rect(&harness, option, &at);
            let back = pixel(
                &raster,
                rect.x + rect.width - 2.0,
                rect.y + rect.height / 2.0,
            );
            assert_within_one(back, [255, 255, 255], &at);
            let text = furthest_from(&raster, rect, back);
            assert!(
                text.iter().all(|channel| *channel < 128),
                "{at}: the row's text paints at its own strength, reading {text:?}"
            );
        }
    }
}

#[test]
fn a_closed_selects_frame_is_byte_equal_before_opening_and_after_a_close_with_no_choice() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        let pick = harness.node("#fx-pick");
        harness.base_mut().set_focus_to(pick);
        harness.pump();
        let before = frame(&mut harness, &at);
        harness.press(enter());
        let open = frame(&mut harness, &at);
        assert!(before.rgba != open.rgba, "{at}: the list paints");
        harness.press(Key::Named(NamedKey::Escape));
        let after = frame(&mut harness, &at);
        assert!(before.rgba == after.rgba, "{at}: nothing of it stays");
    }
}

/// One label as a group's header and as an option's text.
fn grouped_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-grouped", style: "font-size: 20px;",
                option { id: "fx-first", value: "f", "Other" }
                optgroup { id: "fx-group", label: "HHHH",
                    option { id: "fx-same", value: "s", "HHHH" }
                }
            }
        }
    }
}

/// How much darker than white a row's pixels read, summed over its box.
fn ink(raster: &Raster, rect: BoundingRect) -> u64 {
    let mut total = 0;
    let (y0, y1) = (rect.y.ceil() as u32, (rect.y + rect.height).floor() as u32);
    let (x0, x1) = (rect.x.ceil() as u32, (rect.x + rect.width).floor() as u32);
    for y in y0..y1 {
        for x in x0..x1 {
            total += u64::from(255 - pixel(raster, f64::from(x), f64::from(y))[1]);
        }
    }
    total
}

#[test]
fn a_groups_header_is_darker_than_a_row_by_weight() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(grouped_page, incremental);
        open_by_key(&mut harness, "#fx-grouped");
        let raster = frame(&mut harness, &at);
        let header = ink(&raster, row_rect(&harness, "#fx-group", &at));
        let row = ink(&raster, row_rect(&harness, "#fx-same", &at));
        assert!(row > 0, "{at}: the row's text paints");
        assert!(
            header > row,
            "{at}: the header reads {header}, the row {row}"
        );
    }
}

/// A select of 40 rows on a page colour nothing in a list paints.
fn long_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-long",
                for row in 0..40 {
                    option { key: "{row}", id: "fx-row-{row}", value: "{row}", "Row {row}" }
                }
            }
        }
    }
}

#[test]
fn a_scrolled_list_paints_its_thumb_which_takes_no_hit() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(long_page, incremental);
        open_by_key(&mut harness, "#fx-long");
        let view = harness.base().select_list_view();
        let view = view.unwrap_or_else(|| panic!("{at}: paint reads the list"));
        let thumb = view
            .thumb
            .unwrap_or_else(|| panic!("{at}: forty rows read a thumb"));
        let (x, y) = (
            view.scrollport.x + view.scrollport.width / 2.0,
            view.scrollport.y + view.scrollport.height / 2.0,
        );
        let at_thumb = (thumb.x + thumb.width / 2.0, thumb.y + thumb.height / 2.0);
        let before = pixel(&frame(&mut harness, &at), at_thumb.0, at_thumb.1);
        harness.wheel_at(x as f32, y as f32, 0.0, -20.0);
        let moved = harness
            .base()
            .select_list_view()
            .and_then(|view| view.thumb);
        let moved = moved.unwrap_or_else(|| panic!("{at}: the thumb stands"));
        let raster = frame(&mut harness, &at);
        let shown = pixel(
            &raster,
            moved.x + moved.width / 2.0,
            moved.y + moved.height / 2.0,
        );
        assert_ne!(shown, before, "{at}: the wheel shows the thumb");
        let hit = harness.hit(
            (moved.x + moved.width / 2.0) as f32,
            (moved.y + moved.height / 2.0) as f32,
        );
        assert!(
            hit.is_some_and(|hit| hit.node_id != harness.node("#fx-long")),
            "{at}: a hit at the thumb reads a row"
        );
    }
}

/// A `multiple` select whose second option is chosen, at the default sheet.
fn chosen_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-many", multiple: true,
                option { id: "fx-many-a", value: "a", "HHHH" }
                option { id: "fx-many-b", value: "b", selected: true, "HHHH" }
            }
        }
    }
}

fn rgb8(reading: blitz_test_harness::TextContrast) -> ([u8; 3], [u8; 3]) {
    let (text, background) = (reading.text.to_rgba8(), reading.background.to_rgba8());
    (
        [text.r, text.g, text.b],
        [background.r, background.g, background.b],
    )
}

#[test]
fn the_highlight_shares_the_default_sheets_chosen_row_pair() {
    assert_eq!(SelectListView::HIGHLIGHT_TEXT, [0x00, 0x00, 0x00]);
    assert_eq!(SelectListView::HIGHLIGHT_BACKGROUND, [0xc8, 0xc8, 0xc8]);
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let harness = fixtures::boot(chosen_page, incremental);
        assert_eq!(
            harness
                .base()
                .select_value(harness.node("#fx-many"))
                .as_deref(),
            Some("b"),
            "{at}: the fixture's second row is chosen"
        );
        let chosen = harness.text_contrast(harness.node("#fx-many-b"));
        let chosen = chosen.unwrap_or_else(|| panic!("{at}: the chosen row is measured"));
        assert_eq!(
            rgb8(chosen),
            ([0x00, 0x00, 0x00], [0xc8, 0xc8, 0xc8]),
            "{at}: the default sheet's chosen-row pair"
        );
        assert_eq!(
            rgb8(chosen),
            (
                SelectListView::HIGHLIGHT_TEXT,
                SelectListView::HIGHLIGHT_BACKGROUND
            ),
            "{at}: the open list's highlight is the same pair"
        );
    }
}

#[test]
fn the_highlight_reader_answers_the_painted_pair_on_the_highlighted_row_alone() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(page, incremental);
        let banana = harness.node("#fx-banana");
        assert_eq!(
            harness.highlight_contrast(banana),
            None,
            "{at}: a closed select's option reads none"
        );
        open_by_key(&mut harness, "#fx-pick");
        let reading = harness.highlight_contrast(banana);
        let reading = reading.unwrap_or_else(|| panic!("{at}: the highlighted row is read"));
        assert!(
            (reading.ratio - 12.56).abs() < 0.2,
            "{at}: the highlight reads {}",
            reading.ratio
        );
        assert_eq!(reading.font_size_px, 28.0, "{at}: the row's own font size");
        assert_eq!(
            harness.highlight_contrast(harness.node("#fx-apple")),
            None,
            "{at}: a plain row reads none"
        );
        harness.press(Key::Named(NamedKey::Escape));
        assert_eq!(harness.base().open_select(), None, "{at}: Escape closes");
        assert_eq!(
            harness.highlight_contrast(banana),
            None,
            "{at}: with the list closed it reads none"
        );
    }
}

/// A 13px select on the default white whose second option is at 32px.
fn large_font_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-sized", style: "font-size: 13px;",
                option { id: "fx-small", value: "s", "Small" }
                option { id: "fx-large", value: "l", style: "font-size: 32px;", "HHHH" }
            }
        }
    }
}

/// The first and last pixel rows of `rect` that hold ink, relative to its top.
fn inked_rows(raster: &Raster, rect: BoundingRect) -> Option<(u32, u32)> {
    let (y0, y1) = (rect.y.ceil() as u32, (rect.y + rect.height).floor() as u32);
    let (x0, x1) = (rect.x.ceil() as u32, (rect.x + rect.width).floor() as u32);
    let inked: Vec<u32> = (y0..y1)
        .filter(|&y| (x0..x1).any(|x| pixel(raster, f64::from(x), f64::from(y))[1] < 128))
        .collect();
    Some((*inked.first()? - y0, *inked.last()? - y0))
}

#[test]
fn a_larger_fonts_glyphs_paint_wholly_inside_their_row() {
    for incremental in [false, true] {
        let at = format!("incremental {incremental}");
        let mut harness = fixtures::boot(large_font_page, incremental);
        open_by_key(&mut harness, "#fx-sized");
        let raster = frame(&mut harness, &at);
        let rect = row_rect(&harness, "#fx-large", &at);
        let (top, bottom) = inked_rows(&raster, rect)
            .unwrap_or_else(|| panic!("{at}: the 32px row's glyphs paint"));
        let last = (rect.y + rect.height).floor() as u32 - rect.y.ceil() as u32 - 1;
        assert!(top > 0, "{at}: no glyph reaches the row's top edge");
        assert!(
            bottom < last,
            "{at}: no glyph reaches the row's bottom edge"
        );
        // Measured: the top inked row reads 8 on a plain build and 7 under blitz-paint's
        // `font-embolden`, which thickens the glyph by a pixel. This crate cannot read a
        // dependency's feature, and macOS emboldens by its own gate, so the top takes the band.
        assert!(
            (7..=8).contains(&top),
            "{at}: the H's top inked row, measured 8, emboldened 7, reads {top}"
        );
        assert_eq!(bottom, 30, "{at}: the H's bottom inked row, measured");
    }
}
