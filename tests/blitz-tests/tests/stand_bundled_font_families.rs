//! The generic families a run's styles can name read the bundled set: on a page booted with the
//! stand's options (DejaVu Sans, system fonts off), in both layout modes, one line of text set in
//! `serif`, `monospace`, `system-ui` and a family no font provides reads the text extent the
//! same line set in `sans-serif` reads, its width and its height. `cursive` and `fantasy`, which
//! the bundled set registers no font for, are measured and read the same extent too. A failure
//! message names a layout mode and a family only.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

mod fixtures;

/// Each element of the page with the family it sets its text in.
const FAMILIES: [(&str, &str); 7] = [
    ("#fam-sans", "sans-serif"),
    ("#fam-serif", "serif"),
    ("#fam-mono", "monospace"),
    ("#fam-system", "system-ui"),
    ("#fam-missing", "an unavailable family"),
    ("#fam-cursive", "cursive"),
    ("#fam-fantasy", "fantasy"),
];

/// One line of the same text in each family, at one size.
fn families() -> Element {
    rsx! {
        div { style: "font-size: 20px; margin: 0;",
            p { id: "fam-sans", style: "margin: 0; white-space: nowrap; font-family: sans-serif;", "Hamburgefonstiv 0123" }
            p { id: "fam-serif", style: "margin: 0; white-space: nowrap; font-family: serif;", "Hamburgefonstiv 0123" }
            p { id: "fam-mono", style: "margin: 0; white-space: nowrap; font-family: monospace;", "Hamburgefonstiv 0123" }
            p { id: "fam-system", style: "margin: 0; white-space: nowrap; font-family: system-ui;", "Hamburgefonstiv 0123" }
            p { id: "fam-missing", style: "margin: 0; white-space: nowrap; font-family: 'No Such Family';", "Hamburgefonstiv 0123" }
            p { id: "fam-cursive", style: "margin: 0; white-space: nowrap; font-family: cursive;", "Hamburgefonstiv 0123" }
            p { id: "fam-fantasy", style: "margin: 0; white-space: nowrap; font-family: fantasy;", "Hamburgefonstiv 0123" }
        }
    }
}

/// The width and height of the text the element `selector` names lays out.
#[track_caller]
fn extent(harness: &Harness<DioxusDocument>, selector: &str, at: &str) -> (f64, f64) {
    let node = harness
        .query(selector)
        .unwrap_or_else(|| panic!("{at}: the element is on the page"));
    let extent = harness
        .base()
        .text_extent(node)
        .unwrap_or_else(|| panic!("{at}: the element reads a text extent"));
    (extent.text.width, extent.text.height)
}

/// Asserts that the family at `index` of [`FAMILIES`] reads the `sans-serif` line's extent.
#[track_caller]
fn reads_sans_serif(index: usize) {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let harness = fixtures::boot(families, incremental);
        let (sans_selector, sans) = FAMILIES[0];
        let sans_extent = extent(&harness, sans_selector, &format!("{mode} {sans}"));
        assert!(
            sans_extent.0 > 0.0 && sans_extent.1 > 0.0,
            "{mode} {sans}: the bundled font lays the line out"
        );
        let (selector, family) = FAMILIES[index];
        let at = format!("{mode} {family}");
        assert!(
            extent(&harness, selector, &at) == sans_extent,
            "{at}: the line reads the sans-serif line's extent"
        );
    }
}

#[test]
fn serif_reads_the_bundled_font() {
    reads_sans_serif(1);
}

#[test]
fn monospace_reads_the_bundled_font() {
    reads_sans_serif(2);
}

#[test]
fn system_ui_reads_the_bundled_font() {
    reads_sans_serif(3);
}

#[test]
fn an_unavailable_family_reads_the_bundled_font() {
    reads_sans_serif(4);
}

#[test]
fn cursive_reads_the_bundled_font() {
    reads_sans_serif(5);
}

#[test]
fn fantasy_reads_the_bundled_font() {
    reads_sans_serif(6);
}
