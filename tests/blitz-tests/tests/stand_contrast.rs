//! The text/background pair of every stand control, measured and recorded: each row of the
//! shared `controls` table over the four lean tasks, booted headlessly in both layout modes,
//! reads the contrast of its text against the background it is painted on, from computed
//! styles, and its result against the ratio SC 1.4.3 sets for its text's size and weight — or
//! that it is inactive and exempt, or that it paints no text. The reading is this file's stated
//! record, row for row; and the background each measured row resolves is the one painted
//! inside its control's left padding.

use blitz_test_harness::{Harness, PixelRect, TextContrast};
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::LeanTask;

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
        Fail(pair(0xffffff, 0x4a6cf7, 4.39, 16, 500.0)),
    ),
    (
        LeanTask::FlightBooker,
        "back-btn",
        Pass(pair(0x333333, 0xf0f0f0, 11.09, 13, 400.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-one-way",
        Fail(pair(0xffffff, 0x4a6cf7, 4.39, 14, 600.0)),
    ),
    (
        LeanTask::FlightBooker,
        "flight-return",
        Pass(pair(0x444444, 0xfafafa, 9.33, 14, 400.0)),
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
        Fail(pair(0xffffff, 0x4a6cf7, 4.39, 15, 500.0)),
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
        Fail(pair(0xffffff, 0x4a6cf7, 4.39, 16, 500.0)),
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
        Fail(pair(0xffffff, 0x4a6cf7, 4.39, 14, 500.0)),
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

/// Each control of `task` as `harness` reads it, in `controls(task)`'s order.
fn measure(harness: &Harness<DioxusDocument>, task: LeanTask, incremental: bool) -> Vec<Reading> {
    let snapshot = harness.doc.snapshot();
    controls(task)
        .iter()
        .enumerate()
        .map(|(row, (id, _))| {
            let at = format!("{}, {task:?}, control {row}", mode(incremental));
            let enabled = snapshot.get(id).map(|node| node.state.enabled);
            assert!(enabled.is_some(), "{at}: the snapshot lists the control");
            let node = harness.query(&format!("#{id}"));
            assert!(node.is_some(), "{at}: the selector resolves the control");
            match harness.text_contrast(node.unwrap()) {
                None => Reading::NoText,
                Some(contrast) if enabled == Some(Some(false)) => {
                    Reading::ExemptInactive(measured(contrast))
                }
                Some(contrast) if contrast.ratio >= contrast.required_ratio() => {
                    Reading::Pass(measured(contrast))
                }
                Some(contrast) => Reading::Fail(measured(contrast)),
            }
        })
        .collect()
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
    assert_eq!(rows.len(), 18, "the four lean tasks state 18 controls");
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
