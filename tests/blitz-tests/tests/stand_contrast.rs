//! The text/background pair of every stand control, measured and recorded: each row of the
//! shared `controls` table over the four lean tasks, booted headlessly in both layout modes,
//! reads the contrast of its text against the background it is painted on, from computed
//! styles, and its result against the ratio SC 1.4.3 sets for its text's size and weight — or
//! that it is inactive and exempt, or that it paints no text. The reading is this file's stated
//! record, row for row; and the background each measured row resolves is the one painted
//! inside its control's left padding.
//!
//! SC 1.4.3 is asserted on what is measured, not on what is stated: every text-bearing reading
//! that is not exempt meets its ratio — the control rows at boot, the CRUD list rows and the
//! text outside the controls (`TEXT_ROWS`), and the colours the controls take under real input
//! (`STATE_ROWS`: a choice, an open list, an unparseable date, a booking, a selected row, hover and
//! active).
//! The pages the stand serves beside them are held to their required ratios with no recorded hex:
//! the load page's button at rest, hover and active, its shell and its message once its sheet has
//! landed, and Cells' shell, a filled cell, its open editor and its headers.
//! A failure message carries a layout mode, a task and an index, never an id or what a screen
//! reads.

use std::time::Instant;

use blitz_test_harness::{Harness, PixelRect, TextContrast};
use dioxus_native_dom::DioxusDocument;
use escher_driver::LOAD_WAIT;
use keyboard_types::{Key, NamedKey};
use seven_guis::stand::{self, LeanTask};

mod common;
use Reading::{ExemptInactive, Fail, NoText, Pass};
use common::{boot, controls};

/// A measured pair: colours as 8-bit `0xrrggbb`, opaque; the ratio rounded to two decimals; the
/// computed font size in whole px and the computed font weight.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair {
    text: u32,
    background: u32,
    ratio: f32,
    size: u32,
    weight: f32,
}

/// What a control's row records.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Reading {
    /// The control paints no text.
    NoText,
    /// The control reads not enabled: part of an inactive component, which SC 1.4.3 exempts.
    ExemptInactive(Pair),
    /// The ratio meets the one SC 1.4.3 sets for the text.
    Pass(Pair),
    /// The ratio falls short of it.
    Fail(Pair),
}

/// The per-pair record, one row per `controls(task)` row over `LeanTask::ALL`, in that order.
const RECORD: &[(LeanTask, &str, Reading)] = &[
    (
        LeanTask::Counter,
        "back-btn",
        Pass(pair(0x333333, 0xf0f0f0, 11.09, 13, 400.0)),
    ),
    (
        LeanTask::Counter,
        "counter-increment",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 16, 500.0)),
    ),
    (
        LeanTask::FlightBooker,
        "back-btn",
        Pass(pair(0x333333, 0xf0f0f0, 11.09, 13, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-type",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-start",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-return-date",
        ExemptInactive(pair(0x999999, 0xececec, 2.41, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-book",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 15, 500.0)),
    ),
    (
        LeanTask::Timer,
        "back-btn",
        Pass(pair(0x333333, 0xf0f0f0, 11.09, 13, 400.0)),
    ),
    (LeanTask::Timer, "timer-duration", NoText),
    (
        LeanTask::Timer,
        "timer-reset",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 16, 500.0)),
    ),
    (
        LeanTask::Crud,
        "back-btn",
        Pass(pair(0x333333, 0xf0f0f0, 11.09, 13, 400.0)),
    ),
    (
        LeanTask::Crud,
        "crud-filter",
        Pass(pair(0x000000, 0xffffff, 21.00, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "crud-name",
        Pass(pair(0x000000, 0xffffff, 21.00, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "crud-surname",
        Pass(pair(0x000000, 0xffffff, 21.00, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "crud-create",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 14, 500.0)),
    ),
    (
        LeanTask::Crud,
        "crud-update",
        ExemptInactive(pair(0xffffff, 0xb0b8d8, 1.96, 14, 500.0)),
    ),
    (
        LeanTask::Crud,
        "crud-delete",
        ExemptInactive(pair(0xffffff, 0xb0b8d8, 1.96, 14, 500.0)),
    ),
];

const fn pair(text: u32, background: u32, ratio: f32, size: u32, weight: f32) -> Pair {
    Pair {
        text,
        background,
        ratio,
        size,
        weight,
    }
}

fn mode(incremental: bool) -> &'static str {
    if incremental { "incremental" } else { "full" }
}

fn pack(r: u8, g: u8, b: u8) -> u32 {
    (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)
}

fn measured(contrast: TextContrast) -> Pair {
    let (text, background) = (contrast.text.to_rgba8(), contrast.background.to_rgba8());
    Pair {
        text: pack(text.r, text.g, text.b),
        background: pack(background.r, background.g, background.b),
        ratio: (contrast.ratio * 100.0).round() / 100.0,
        size: contrast.font_size_px.round() as u32,
        weight: contrast.font_weight,
    }
}

/// The element `selector` resolves, as `harness` reads it: exempt only where the snapshot reads
/// it not enabled, and passing or failing by its measured ratio against its required one.
#[track_caller]
fn read(harness: &Harness<DioxusDocument>, selector: &str, at: &str) -> Reading {
    let enabled = selector.strip_prefix('#').and_then(|id| {
        harness
            .doc
            .snapshot()
            .get(id)
            .map(|node| node.state.enabled)
    });
    let node = harness.query(selector);
    assert!(node.is_some(), "{at}: the selector resolves the element");
    match harness.text_contrast(node.unwrap()) {
        None => NoText,
        Some(contrast) if enabled == Some(Some(false)) => ExemptInactive(measured(contrast)),
        Some(contrast) if contrast.ratio >= contrast.required_ratio() => Pass(measured(contrast)),
        Some(contrast) => Fail(measured(contrast)),
    }
}

/// Each control of `task` as `harness` reads it, in `controls(task)`'s order.
fn measure(harness: &Harness<DioxusDocument>, task: LeanTask, incremental: bool) -> Vec<Reading> {
    let snapshot = harness.doc.snapshot();
    controls(task)
        .iter()
        .enumerate()
        .map(|(row, (id, _))| {
            let at = format!("{}, {task:?}, control {row}", mode(incremental));
            assert!(
                snapshot.get(id).is_some(),
                "{at}: the snapshot lists the control"
            );
            read(harness, &format!("#{id}"), &at)
        })
        .collect()
}

/// The CRUD list rows at boot and the text outside the controls, each read from the element its
/// selector resolves, in `LeanTask::ALL`'s order.
const TEXT_ROWS: [(LeanTask, &str, Reading); 14] = [
    (
        LeanTask::Counter,
        "#task-title",
        Pass(pair(0x111111, 0xffffff, 18.88, 15, 600.0)),
    ),
    (
        LeanTask::Counter,
        "#counter-value",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 24, 600.0)),
    ),
    (
        LeanTask::FlightBooker,
        "#task-title",
        Pass(pair(0x111111, 0xffffff, 18.88, 15, 600.0)),
    ),
    (
        LeanTask::Timer,
        "#task-title",
        Pass(pair(0x111111, 0xffffff, 18.88, 15, 600.0)),
    ),
    (
        LeanTask::Timer,
        "#timer-elapsed",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 18, 600.0)),
    ),
    (
        LeanTask::Timer,
        "label[for=timer-duration]",
        Pass(pair(0x444444, 0xffffff, 9.74, 15, 400.0)),
    ),
    (
        LeanTask::Timer,
        "#timer-duration-value",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 600.0)),
    ),
    (
        LeanTask::Crud,
        "#task-title",
        Pass(pair(0x111111, 0xffffff, 18.88, 15, 600.0)),
    ),
    (
        LeanTask::Crud,
        "label[for=crud-filter]",
        Pass(pair(0x000000, 0xf5f5f5, 19.26, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "#crud-person-0",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "#crud-person-1",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "#crud-person-2",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        "label[for=crud-name]",
        Pass(pair(0x333333, 0xf5f5f5, 11.59, 14, 500.0)),
    ),
    (
        LeanTask::Crud,
        "label[for=crud-surname]",
        Pass(pair(0x333333, 0xf5f5f5, 11.59, 14, 500.0)),
    ),
];

/// The real input a state row is read after, on a fresh boot of its task.
#[derive(Clone, Copy, Debug)]
enum Reach {
    /// A click on `flight-type`, End and Enter, choosing its return flight.
    ReturnChosen,
    /// A click on `flight-type`, its list left open.
    ListOpen,
    /// A click on `flight-start` and one character typed after its date, which then parses no
    /// more.
    StartUnparseable,
    /// A click on `flight-book`.
    Booked,
    /// Three Tabs to the CRUD list and ArrowDown, selecting its first row.
    RowSelected,
    /// The pointer moved to the centre of the row's own element.
    Hover,
    /// The pointer moved there and its main button pressed.
    Active,
}

/// The colours the controls take under real input, each read after its `Reach` on a fresh boot.
const STATE_ROWS: [(LeanTask, Reach, &str, Reading); 20] = [
    (
        LeanTask::FlightBooker,
        Reach::ListOpen,
        "flight-type-one-way",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::ListOpen,
        "flight-type-return",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::ReturnChosen,
        "flight-type",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::ReturnChosen,
        "flight-return-date",
        Pass(pair(0x1a1a1a, 0xffffff, 17.40, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::StartUnparseable,
        "flight-start",
        Pass(pair(0xc53030, 0xfff5f5, 5.11, 15, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::StartUnparseable,
        "flight-book",
        ExemptInactive(pair(0xffffff, 0xa0aec0, 2.26, 15, 500.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::Booked,
        "flight-booked",
        Pass(pair(0x276749, 0xebf8ee, 6.15, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        Reach::RowSelected,
        "crud-person-0",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 14, 400.0)),
    ),
    (
        LeanTask::Crud,
        Reach::RowSelected,
        "crud-update",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 14, 500.0)),
    ),
    (
        LeanTask::Crud,
        Reach::RowSelected,
        "crud-delete",
        Pass(pair(0xffffff, 0x3a5ce5, 5.47, 14, 500.0)),
    ),
    (
        LeanTask::Counter,
        Reach::Hover,
        "counter-increment",
        Pass(pair(0xffffff, 0x2a4cd3, 6.83, 16, 500.0)),
    ),
    (
        LeanTask::Counter,
        Reach::Active,
        "counter-increment",
        Pass(pair(0xffffff, 0x1f3fc1, 8.27, 16, 500.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::Hover,
        "flight-book",
        Pass(pair(0xffffff, 0x2a4cd3, 6.83, 15, 500.0)),
    ),
    (
        LeanTask::FlightBooker,
        Reach::Active,
        "flight-book",
        Pass(pair(0xffffff, 0x1f3fc1, 8.27, 15, 500.0)),
    ),
    (
        LeanTask::Timer,
        Reach::Hover,
        "timer-reset",
        Pass(pair(0xffffff, 0x2a4cd3, 6.83, 16, 500.0)),
    ),
    (
        LeanTask::Timer,
        Reach::Active,
        "timer-reset",
        Pass(pair(0xffffff, 0x1f3fc1, 8.27, 16, 500.0)),
    ),
    // CRUD's buttons and the shell's back button declare no `:active` colour: pressed, they
    // keep their hover one.
    (
        LeanTask::Crud,
        Reach::Hover,
        "crud-create",
        Pass(pair(0xffffff, 0x2a4cd3, 6.83, 14, 500.0)),
    ),
    (
        LeanTask::Crud,
        Reach::Active,
        "crud-create",
        Pass(pair(0xffffff, 0x2a4cd3, 6.83, 14, 500.0)),
    ),
    (
        LeanTask::Counter,
        Reach::Hover,
        "back-btn",
        Pass(pair(0x333333, 0xe4e4e4, 9.94, 13, 400.0)),
    ),
    (
        LeanTask::Counter,
        Reach::Active,
        "back-btn",
        Pass(pair(0x333333, 0xe4e4e4, 9.94, 13, 400.0)),
    ),
];

/// Drives `harness` through `reach`, asserting the state it leads to holds.
#[track_caller]
fn reach(harness: &mut Harness<DioxusDocument>, reach: Reach, id: &str, at: &str) {
    match reach {
        Reach::ReturnChosen => {
            harness.click("#flight-type");
            harness.press(Key::Named(NamedKey::End));
            harness.press(Key::Named(NamedKey::Enter));
            let snapshot = harness.doc.snapshot();
            assert!(
                common::node(&snapshot, "flight-type")
                    .state
                    .value
                    .as_deref()
                    == Some("return")
                    && common::node(&snapshot, "flight-type-return").state.selected == Some(true),
                "{at}: the flight type reads the return flight chosen"
            );
        }
        Reach::ListOpen => {
            harness.click("#flight-type");
            assert!(
                harness.base().open_select() == Some(harness.node("#flight-type")),
                "{at}: the list stands open"
            );
        }
        Reach::StartUnparseable => {
            harness.click("#flight-start");
            harness.type_text("x");
            let snapshot = harness.doc.snapshot();
            assert!(
                common::node(&snapshot, "flight-start").state.invalid,
                "{at}: the departure date reads invalid"
            );
        }
        Reach::Booked => {
            harness.click("#flight-book");
            assert!(
                harness.query("#flight-booked").is_some(),
                "{at}: the booking message renders"
            );
        }
        Reach::RowSelected => {
            for _ in 0..3 {
                harness.press(Key::Named(NamedKey::Tab));
            }
            harness.press(Key::Named(NamedKey::ArrowDown));
            let snapshot = harness.doc.snapshot();
            assert!(
                common::node(&snapshot, "crud-person-0").state.selected == Some(true),
                "{at}: the first row reads selected"
            );
        }
        Reach::Hover | Reach::Active => {
            let node = harness.node(&format!("#{id}"));
            let (x, y) = harness.layout_rect_of(node).center();
            harness.move_mouse_to(x, y);
            if matches!(reach, Reach::Active) {
                harness.mouse_down_at(x, y);
            }
            assert!(
                harness.hovered() == Some(node),
                "{at}: the pointer is over the control"
            );
        }
    }
}

/// Every row of `controls(task)` over `LeanTask::ALL`, in that order.
fn control_rows() -> Vec<(LeanTask, &'static str)> {
    LeanTask::ALL
        .into_iter()
        .flat_map(|task| controls(task).iter().map(move |(id, _)| (task, *id)))
        .collect()
}

#[test]
fn every_stand_control_reads_its_recorded_pair() {
    let rows = control_rows();
    assert_eq!(rows.len(), 17, "the four lean tasks state 17 controls");
    assert_eq!(RECORD.len(), rows.len(), "the record has a row per control");
    for (index, ((task, id), (stated_task, stated_id, _))) in rows.iter().zip(RECORD).enumerate() {
        assert!(
            task == stated_task && id == stated_id,
            "record row {index} follows the controls table's order"
        );
    }

    let mut records = Vec::new();
    for incremental in [false, true] {
        let mut record = Vec::new();
        for task in LeanTask::ALL {
            record.extend(measure(&boot(task, incremental), task, incremental));
        }
        for (index, (reading, (task, _, stated))) in record.iter().zip(RECORD).enumerate() {
            assert!(
                reading == stated,
                "{}, {task:?}, record row {index}: the reading is the recorded one",
                mode(incremental)
            );
        }
        records.push(record);
    }
    assert!(
        records[0] == records[1],
        "the record is the same in both layout modes"
    );
}

#[test]
fn the_painted_background_is_the_measured_background() {
    for incremental in [false, true] {
        for task in LeanTask::ALL {
            let mut harness = boot(task, incremental);
            let mut probes = Vec::new();
            for (row, (id, _)) in controls(task).iter().enumerate() {
                let at = format!("{}, {task:?}, control {row}", mode(incremental));
                let node = harness.query(&format!("#{id}"));
                assert!(node.is_some(), "{at}: the selector resolves the control");
                let node = node.unwrap();
                let Some(contrast) = harness.text_contrast(node) else {
                    continue;
                };
                let rect = harness.layout_rect_of(node);
                let (border, padding) = {
                    let doc = harness.base();
                    let layout = doc.get_node(node).unwrap().final_layout();
                    (layout.border.left, layout.padding.left)
                };
                assert!(padding >= 2.0, "{at}: the pixel lies in the left padding");
                let pixel = PixelRect {
                    x: (rect.x + border + padding / 2.0) as u32,
                    y: (rect.y + rect.height / 2.0) as u32,
                    width: 1,
                    height: 1,
                };
                probes.push((at, node, pixel, contrast.background.to_rgba8()));
            }
            assert!(
                probes
                    .iter()
                    .all(|(_, node, _, _)| harness.focused() != Some(*node)),
                "{}, {task:?}: no measured control holds focus, so no caret paints",
                mode(incremental)
            );
            for (at, _, pixel, background) in probes {
                let raster = harness.screenshot_rect(pixel);
                assert!(raster.is_ok(), "{at}: the pixel is painted");
                let painted = raster.unwrap().rgba;
                let expected = [background.r, background.g, background.b];
                for (channel, value) in expected.into_iter().enumerate() {
                    assert!(
                        painted[channel].abs_diff(value) <= 1,
                        "{at}, channel {channel}: the painted background is the measured one"
                    );
                }
            }
        }
    }
}

/// How many of `readings` read each way: pass, fail, exempt-inactive, no text.
fn tally(readings: &[Reading]) -> (usize, usize, usize, usize) {
    let count = |class: fn(&Reading) -> bool| readings.iter().filter(|r| class(r)).count();
    (
        count(|r| matches!(r, Pass(_))),
        count(|r| matches!(r, Fail(_))),
        count(|r| matches!(r, ExemptInactive(_))),
        count(|r| matches!(r, NoText)),
    )
}

#[test]
fn every_measured_control_meets_its_required_ratio() {
    let rows = control_rows();
    for incremental in [false, true] {
        let mut readings = Vec::new();
        for task in LeanTask::ALL {
            readings.extend(measure(&boot(task, incremental), task, incremental));
        }
        assert_eq!(
            readings.len(),
            rows.len(),
            "{}: a reading per control",
            mode(incremental)
        );
        for (index, (reading, (task, _))) in readings.iter().zip(&rows).enumerate() {
            assert!(
                !matches!(reading, Fail(_)),
                "{}, {task:?}, control row {index}: the measured ratio meets the required one",
                mode(incremental)
            );
        }
        assert_eq!(
            tally(&readings),
            (13, 0, 3, 1),
            "{}: pass, fail, exempt-inactive and no-text rows",
            mode(incremental)
        );
    }
}

#[test]
fn every_list_row_and_text_outside_the_controls_meets_its_ratio() {
    assert_eq!(TEXT_ROWS.len(), 14, "the stated text rows");
    for incremental in [false, true] {
        let mut readings = Vec::new();
        for task in LeanTask::ALL {
            let harness = boot(task, incremental);
            for (index, (row_task, selector, stated)) in TEXT_ROWS.iter().enumerate() {
                if *row_task != task {
                    continue;
                }
                let at = format!("{}, {task:?}, text row {index}", mode(incremental));
                let reading = read(&harness, selector, &at);
                assert!(
                    matches!(reading, Pass(_)),
                    "{at}: the measured ratio meets the required one"
                );
                assert!(reading == *stated, "{at}: the reading is the recorded one");
                readings.push(reading);
            }
        }
        assert_eq!(
            tally(&readings),
            (14, 0, 0, 0),
            "{}: every text row is read and passes",
            mode(incremental)
        );
    }
}

/// Asserts `selector` reads a ratio that meets its required one.
#[track_caller]
fn assert_meets_its_ratio(harness: &Harness<DioxusDocument>, selector: &str, at: &str) {
    assert!(
        matches!(read(harness, selector, at), Pass(_)),
        "{at}: the measured ratio meets the required one"
    );
}

#[test]
fn the_load_page_meets_its_ratios() {
    const REST: [&str; 3] = ["#load-fetch", "#back-btn", "#task-title"];
    assert_eq!(REST.len(), 3);
    for incremental in [false, true] {
        let boot_load = || stand::boot_load(stand::options(incremental));
        let harness = boot_load();
        for (row, selector) in REST.into_iter().enumerate() {
            let at = format!("{}, load page, row {row}", mode(incremental));
            assert_meets_its_ratio(&harness, selector, &at);
        }
        for (row, how) in [Reach::Hover, Reach::Active].into_iter().enumerate() {
            let at = format!("{}, load page, state {row}", mode(incremental));
            let mut harness = boot_load();
            reach(&mut harness, how, "load-fetch", &at);
            assert_meets_its_ratio(&harness, "#load-fetch", &at);
        }

        // The message shows past `LOADED_CSS` alone, so its row reads the page once the fetch is
        // answered: an answer that comes on a thread of its own, the driver's own bound on it.
        let at = format!("{}, load page, message", mode(incremental));
        let mut harness = boot_load();
        harness.click("#load-fetch");
        let deadline = Instant::now() + LOAD_WAIT;
        while harness.settle().is_err() {
            assert!(Instant::now() < deadline, "{at}: the load is answered");
            std::thread::yield_now();
        }
        assert_meets_its_ratio(&harness, "#load-done", &at);
    }
}

#[test]
fn cells_meets_its_ratios() {
    const ROWS: [&str; 5] = [
        "#back-btn",
        "#task-title",
        "#cell-A0",
        ".col-header",
        ".row-header",
    ];
    assert_eq!(ROWS.len(), 5);
    for incremental in [false, true] {
        let mut harness = stand::boot_cells(stand::options(incremental));
        for _ in 0..2 {
            harness.press(Key::Named(NamedKey::Tab));
        }
        harness.press(Key::Named(NamedKey::Enter));
        harness.type_text("5");
        let at = format!("{}, Cells, its editor", mode(incremental));
        assert_meets_its_ratio(&harness, "#cell-input", &at);
        harness.press(Key::Named(NamedKey::Enter));
        for (row, selector) in ROWS.into_iter().enumerate() {
            let at = format!("{}, Cells, row {row}", mode(incremental));
            assert_meets_its_ratio(&harness, selector, &at);
        }
    }
}

#[test]
fn every_colour_state_meets_its_ratio() {
    assert_eq!(STATE_ROWS.len(), 20, "the stated state rows");
    for incremental in [false, true] {
        let mut readings = Vec::new();
        for (index, (task, how, id, stated)) in STATE_ROWS.into_iter().enumerate() {
            let at = format!("{}, {task:?}, state row {index}", mode(incremental));
            let mut harness = boot(task, incremental);
            reach(&mut harness, how, id, &at);
            let reading = read(&harness, &format!("#{id}"), &at);
            assert!(
                !matches!(reading, Fail(_) | NoText),
                "{at}: the measured ratio meets the required one, or the control is inactive"
            );
            assert!(reading == stated, "{at}: the reading is the recorded one");
            readings.push(reading);
        }
        assert_eq!(
            tally(&readings),
            (19, 0, 1, 0),
            "{}: pass, fail, exempt-inactive and no-text rows",
            mode(incremental)
        );
    }
}
