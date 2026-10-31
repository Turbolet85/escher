//! `:open` matches a `details` or a `dialog` carrying `open`, in every document, through every path
//! that writes it: at element construction, on `open` written through the mutator and on its
//! removal, and on a `summary` flipping its `details` by a click and by Enter. An author `:open`
//! rule's computed style reads it. On parsed pages, in both layout modes. A failure message carries
//! the layout mode and a row.
//!
//! <https://drafts.csswg.org/selectors-4/#open-state>

use ::keyboard_types::{Key, NamedKey};
use blitz_dom::{QualName, local_name, ns};
use blitz_html::HtmlDocument;
use blitz_test_harness::{Harness, HarnessOptions};

/// A closed and an open `details`, a closed and an open `dialog`, and the `:open` rule that widens
/// whichever matches.
const PAGE: &str = r#"<html><head><style>
    details, dialog { width: 100px; display: block; position: static; margin: 0; padding: 0; border: 0; }
    :open { width: 300px; }
</style></head><body style="margin:0">
    <details id="closed-details"><summary id="closed-summary">A</summary></details>
    <details id="open-details" open><summary id="open-summary">B</summary></details>
    <dialog id="closed-dialog">C</dialog>
    <dialog id="open-dialog" open>D</dialog>
</body></html>"#;

fn boot(incremental: bool) -> Harness<HtmlDocument> {
    Harness::from_html_with(
        PAGE,
        HarnessOptions {
            incremental: Some(incremental),
            ..Default::default()
        },
    )
}

fn open_name() -> QualName {
    QualName::new(None, ns!(), local_name!("open"))
}

/// Whether `selector` matches `:open` as a query reads it, and as the width its rule computes.
#[track_caller]
fn matches_open(harness: &Harness<HtmlDocument>, selector: &str, mode: &str) -> bool {
    let node = harness.node(selector);
    let by_query = harness.query_all(":open").contains(&node);
    let width = harness.base().resolved_style_value(node, "width");
    let by_style = width == "300px";
    assert!(
        by_query == by_style,
        "{mode}: the selector and the computed style agree"
    );
    by_query
}

#[test]
fn at_construction_open_is_read_from_the_attribute() {
    const ROWS: [(&str, bool); 4] = [
        ("#closed-details", false),
        ("#open-details", true),
        ("#closed-dialog", false),
        ("#open-dialog", true),
    ];
    assert_eq!(ROWS.len(), 4);
    for incremental in [false, true] {
        let harness = boot(incremental);
        for (row, (selector, expected)) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            assert!(
                matches_open(&harness, selector, &mode) == expected,
                "{mode}: boot reads the attribute"
            );
        }
    }
}

#[test]
fn a_write_and_a_removal_through_the_mutator_move_it() {
    const ROWS: [&str; 2] = ["#closed-details", "#closed-dialog"];
    assert_eq!(ROWS.len(), 2);
    for incremental in [false, true] {
        let mut harness = boot(incremental);
        for (row, selector) in ROWS.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {row}");
            let node = harness.node(selector);
            harness
                .base_mut()
                .mutate()
                .set_attribute(node, open_name(), "");
            harness.pump();
            assert!(
                matches_open(&harness, selector, &mode),
                "{mode}: the write matches"
            );
            harness
                .base_mut()
                .mutate()
                .clear_attribute(node, open_name());
            harness.pump();
            assert!(
                !matches_open(&harness, selector, &mode),
                "{mode}: the removal stops matching"
            );
        }
    }
}

#[test]
fn a_summary_click_flips_its_details() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot(incremental);
        harness.click("#closed-summary");
        assert!(
            matches_open(&harness, "#closed-details", &mode),
            "{mode}: a click opens it"
        );
        harness.advance_time(std::time::Duration::from_millis(500));
        harness.click("#closed-summary");
        assert!(
            !matches_open(&harness, "#closed-details", &mode),
            "{mode}: a second click closes it"
        );
    }
}

#[test]
fn a_summary_enter_flips_its_details() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut harness = boot(incremental);
        let summary = harness.node("#open-summary");
        harness.base_mut().set_focus_to(summary);
        harness.press(Key::Named(NamedKey::Enter));
        assert!(
            !matches_open(&harness, "#open-details", &mode),
            "{mode}: Enter closes the open one"
        );
        harness.press(Key::Named(NamedKey::Enter));
        assert!(
            matches_open(&harness, "#open-details", &mode),
            "{mode}: Enter opens it again"
        );
    }
}
