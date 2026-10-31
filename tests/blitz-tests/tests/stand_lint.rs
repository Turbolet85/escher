//! `Session::lint` reads the screen of a held instance for six defects, each named by the stable
//! id of the element it is found on with its measured reason, in both layout modes. On a page
//! that plants each defect beside six negatives it names exactly the seven planted findings in
//! document order, with their reasons; on a page of three paragraphs holding text, an off-screen
//! one reads `truncated` beside `off-screen`, a clipped one whose line leaves the box that clips
//! it reads `clipped` and `truncated`, and a clipped one whose line stays inside reads `clipped`
//! alone, the verb answering the same; on each lean stand task at boot it reads what is
//! measured there — nothing, but on the flight booker one `overlap` between its type select's
//! two options, which the snapshot gives their select's client rect; it changes nothing; after twelve Creates it names the last CRUD row cut by the list's box, the
//! row a `click` is refused `off-screen` on; and an app with no font is refused
//! `fonts-unavailable`. The verb `lint`, run through `Session::run`, answers exactly the findings
//! the reader reads, on the planted page and on each lean task. A failure message carries a
//! layout mode, a task and a row index only.

use blitz_test_harness::HarnessOptions;
use dioxus::prelude::*;
use escher_driver::{Bounds, Cause, Defect, Finding, Reason, Session};
use seven_guis::stand::{self, LeanTask};

mod fixtures;
mod session_common;
use session_common::{act, click, focused, hold, lint, linted, refused, slug, type_into};

/// How many Creates put the last CRUD row's centre past the list's box.
const CREATES: usize = 12;

/// Each planted defect and the six negatives beside them, at separate absolute positions on a
/// root that fills the viewport inside the body's default margin.
fn lint_page() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 784px; height: 584px;",
            div {
                id: "fx-overlap-a",
                style: "position: absolute; left: 20px; top: 20px; width: 100px; height: 40px; background: #dddddd;",
            }
            div {
                id: "fx-overlap-b",
                style: "position: absolute; left: 80px; top: 40px; width: 100px; height: 40px; background: #bbbbbb;",
            }
            div {
                id: "fx-inert",
                "inert": "true",
                style: "position: absolute; left: 80px; top: 40px; width: 100px; height: 40px;",
                "Inert"
            }
            div {
                id: "fx-hidden",
                hidden: true,
                style: "position: absolute; left: 80px; top: 40px; width: 100px; height: 40px;",
                "Hidden"
            }
            div {
                id: "fx-clip-box",
                style: "position: absolute; left: 220px; top: 20px; width: 100px; height: 40px; overflow: hidden;",
                div { id: "fx-clipped", style: "width: 200px; height: 20px;" }
            }
            div {
                id: "fx-paint-box",
                style: "position: absolute; left: 440px; top: 20px; width: 100px; height: 40px; contain: paint;",
                div { id: "fx-paint-clipped", style: "width: 200px; height: 20px;" }
            }
            p {
                id: "fx-truncated",
                style: "position: absolute; left: 20px; top: 100px; width: 80px; margin: 0; white-space: nowrap; overflow: hidden;",
                "A line too long for its box"
            }
            div {
                id: "fx-offscreen",
                style: "position: absolute; left: 820px; top: 100px; width: 50px; height: 20px;",
            }
            button {
                id: "fx-zero",
                style: "position: absolute; left: 140px; top: 100px; width: 0; height: 0; padding: 0; border: 0;",
                "Go"
            }
            p {
                id: "fx-low-contrast",
                style: "position: absolute; left: 220px; top: 100px; margin: 0; color: #999999; background: #ffffff;",
                "Faint text"
            }
            div {
                id: "fx-spacer",
                style: "position: absolute; left: 20px; top: 160px; width: 100px; height: 0;",
            }
            button {
                id: "fx-disabled",
                disabled: true,
                style: "position: absolute; left: 220px; top: 160px; color: #999999; background: #ffffff;",
                "Off"
            }
            p {
                id: "fx-faded",
                style: "position: absolute; left: 340px; top: 160px; margin: 0; opacity: 0.5; color: #999999;",
                "Faded"
            }
            input {
                id: "fx-long-input",
                r#type: "text",
                value: "A value much wider than its box",
                style: "position: absolute; left: 20px; top: 220px; width: 60px;",
            }
            p {
                id: "fx-wrapped",
                style: "position: absolute; left: 220px; top: 220px; width: 80px; margin: 0;",
                "A line too long for its box"
            }
        }
    }
}

/// Three paragraphs holding text: one whose line runs past the box that clips it, one whose
/// padding alone passes that box while its line stays inside, and one wholly right of the
/// viewport.
fn reading_page() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 784px; height: 584px;",
            div {
                id: "fx-cut-box",
                style: "position: absolute; left: 20px; top: 20px; width: 100px; height: 40px; overflow: hidden;",
                p {
                    id: "fx-cut-text",
                    style: "width: 200px; margin: 0; white-space: nowrap;",
                    "A line past its box"
                }
            }
            div {
                id: "fx-padded-box",
                style: "position: absolute; left: 220px; top: 20px; width: 100px; height: 40px; overflow: hidden;",
                p {
                    id: "fx-padded-text",
                    style: "width: 60px; margin: 0; padding-right: 100px;",
                    "Go"
                }
            }
            p {
                id: "fx-off-text",
                style: "position: absolute; left: 820px; top: 20px; margin: 0; white-space: nowrap;",
                "Off"
            }
        }
    }
}

/// The findings, each as its element's id and its defect's word.
fn named(findings: &[Finding]) -> Vec<(&str, &str)> {
    findings
        .iter()
        .map(|finding| (&*finding.id, finding.defect.name()))
        .collect()
}

/// The bounds a snapshot of `session` reads for `id`.
#[track_caller]
fn bounds(session: &Session, id: &str, at: &str) -> Bounds {
    let screen = session.harness().doc.snapshot();
    let node = screen.get(id);
    assert!(node.is_some(), "{at}: the element is on the screen");
    let rect = node.unwrap().bounds;
    Bounds {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

fn within(inner: Bounds, outer: Bounds) -> bool {
    inner.x >= outer.x
        && inner.y >= outer.y
        && inner.x + inner.width <= outer.x + outer.width
        && inner.y + inner.height <= outer.y + outer.height
}

#[test]
fn the_planted_defects_are_named_each_by_its_id_and_reason() {
    let rows = [
        ("fx-overlap-a", "overlap"),
        ("fx-clipped", "clipped"),
        ("fx-paint-clipped", "clipped"),
        ("fx-truncated", "truncated"),
        ("fx-offscreen", "off-screen"),
        ("fx-zero", "zero-size"),
        ("fx-low-contrast", "contrast"),
    ];
    assert_eq!(rows.len(), 7);
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = fixtures::hold("lint", lint_page, incremental);
        let findings = session.lint();
        assert!(findings.is_ok(), "{at}: the lint reads the page");
        let findings = findings.unwrap();
        assert!(
            named(&findings) == rows,
            "{at}: the findings are the seven planted, in document order"
        );

        assert!(
            matches!(&findings[0].reason, Reason::Overlap { other, .. } if other == "fx-overlap-b"),
            "{at}: row 0 overlaps the later box"
        );
        for (row, holder) in [(1, "fx-clip-box"), (2, "fx-paint-box")] {
            let holder = bounds(&session, holder, &at);
            let finding = &findings[row];
            assert!(
                matches!(finding.reason, Reason::Clipped { visible }
                    if within(visible, holder) && visible.width < finding.bounds.width),
                "{at}: row {row} is seen inside its clipping box, narrower than its bounds"
            );
        }
        assert!(
            matches!(findings[3].reason, Reason::Truncated { text, .. }
                if text.width > findings[3].bounds.width),
            "{at}: row 3's text is wider than its box"
        );
        let faint = session.harness().query("#fx-low-contrast");
        assert!(faint.is_some(), "{at}: row 6 resolves");
        let measured = session.harness().text_contrast(faint.unwrap());
        assert!(measured.is_some(), "{at}: row 6's contrast is measured");
        assert!(
            matches!(findings[6].reason, Reason::Contrast { ratio, required }
                if ratio == measured.unwrap().ratio && ratio < required && required == 4.5),
            "{at}: row 6 reads the harness's ratio, below the 4.5 it needs"
        );
    }
}

#[test]
fn a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it() {
    let rows = [
        ("fx-cut-text", "clipped"),
        ("fx-cut-text", "truncated"),
        ("fx-padded-text", "clipped"),
        ("fx-off-text", "truncated"),
        ("fx-off-text", "off-screen"),
    ];
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = fixtures::hold("lint", reading_page, incremental);

        // The condition first: each paragraph lays out a line with the bundled font.
        for (index, selector) in ["#fx-cut-text", "#fx-padded-text", "#fx-off-text"]
            .into_iter()
            .enumerate()
        {
            let node = session.harness().query(selector);
            assert!(node.is_some(), "{at}: row {index} resolves");
            let extent = session.harness().base().text_extent(node.unwrap());
            assert!(
                extent.is_some_and(|extent| extent.text.width > 0.0 && extent.text.height > 0.0),
                "{at}: row {index} lays out a line"
            );
        }

        let findings = session.lint();
        assert!(findings.is_ok(), "{at}: the lint reads the page");
        let findings = findings.unwrap();
        assert!(
            named(&findings) == rows,
            "{at}: the findings in document order, then defect order"
        );
        assert!(
            matches!(findings[1].reason, Reason::Truncated { text, visible: Some(visible) }
                if visible.width < text.width),
            "{at}: row 1's line is seen in part"
        );
        assert!(
            matches!(findings[3].reason, Reason::Truncated { visible: None, .. }),
            "{at}: row 3's line is not seen"
        );
        assert!(
            linted(&mut session, &lint()) == Some(findings),
            "{at}: the verb answers the reader's findings"
        );
    }
}

/// What the lint reads on `task` at boot, measured: each finding's id, defect and the other
/// element of its overlap. The flight booker's two type options read their closed select's client
/// rect as their bounds, which the lint does not read as an overlap.
fn at_boot(task: LeanTask) -> &'static [(&'static str, Defect, &'static str)] {
    match task {
        LeanTask::FlightBooker | LeanTask::Counter | LeanTask::Timer | LeanTask::Crud => &[],
    }
}

/// A closed select's options, a closed select's options under a group, and a `multiple` select
/// whose second row is planted over its first.
fn selects_page() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 784px; height: 584px;",
            select {
                id: "fx-closed",
                aria_label: "Closed",
                style: "position: absolute; left: 20px; top: 20px; width: 120px;",
                option { id: "fx-closed-a", value: "a", "A" }
                option { id: "fx-closed-b", value: "b", "B" }
            }
            select {
                id: "fx-grouped",
                aria_label: "Grouped",
                style: "position: absolute; left: 200px; top: 20px; width: 120px;",
                optgroup { id: "fx-group", label: "Group",
                    option { id: "fx-grouped-a", value: "a", "A" }
                    option { id: "fx-grouped-b", value: "b", "B" }
                }
            }
            select {
                id: "fx-rows",
                aria_label: "Rows",
                multiple: true,
                style: "position: absolute; left: 400px; top: 20px; width: 120px;",
                option { id: "fx-row-a", value: "a", "A" }
                option { id: "fx-row-b", value: "b", style: "margin-top: -6px;", "B" }
            }
        }
    }
}

#[test]
fn a_closed_selects_options_overlap_nothing_and_a_multiple_selects_rows_are_linted() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = fixtures::hold("lint", selects_page, incremental);
        let findings = linted(&mut session, &lint()).expect("the lint is answered");
        assert!(
            named(&findings) == [("fx-row-a", "overlap")],
            "{at}: the planted rows are the one finding"
        );
        assert!(
            matches!(&findings[0].reason, Reason::Overlap { other, .. } if other == "fx-row-b"),
            "{at}: the finding names the row over it"
        );
    }
}

#[test]
fn the_stand_at_boot_reads_its_measured_findings() {
    assert_eq!(LeanTask::ALL.map(|task| at_boot(task).len()), [0, 0, 0, 0]);
    for incremental in [false, true] {
        for (index, task) in LeanTask::ALL.into_iter().enumerate() {
            let at = format!("incremental={incremental}: task {index}");
            let (mut session, _ticks) = hold(task, incremental);
            let findings = session.lint();
            assert!(findings.is_ok(), "{at}: the lint reads the task");
            let read: Vec<(&str, Defect, &str)> = findings
                .as_ref()
                .unwrap()
                .iter()
                .map(|finding| {
                    let other = match &finding.reason {
                        Reason::Overlap { other, .. } => other.as_str(),
                        _ => "",
                    };
                    (finding.id.as_str(), finding.defect, other)
                })
                .collect();
            assert!(
                read == at_boot(task),
                "{at}: the lint reads the measured findings"
            );
        }
    }
}

#[test]
fn the_lint_reads_and_changes_nothing() {
    for incremental in [false, true] {
        for case in 0..2 {
            let at = format!("incremental={incremental}: case {case}");
            let mut session = match case {
                0 => fixtures::hold("lint", lint_page, incremental),
                _ => hold(LeanTask::Crud, incremental).0,
            };
            let _ = session.harness_mut().base_mut().take_changed_nodes();
            let before = (
                session.harness().doc.snapshot(),
                focused(&session),
                session.harness().time(),
            );
            let mut read = Vec::new();
            for round in 0..2 {
                let findings = session.lint();
                assert!(findings.is_ok(), "{at}: round {round} reads");
                read.push(findings.unwrap());
                assert!(
                    !session.harness().base().has_changes(),
                    "{at}: round {round} marks no change"
                );
                let after = (
                    session.harness().doc.snapshot(),
                    focused(&session),
                    session.harness().time(),
                );
                assert!(after.1 == before.1, "{at}: round {round} keeps the focus");
                assert!(after.2 == before.2, "{at}: round {round} moves no time");
                assert!(after.0 == before.0, "{at}: round {round} keeps the screen");
            }
            assert!(read[0] == read[1], "{at}: the two rounds read alike");
        }
    }
}

#[test]
fn the_rows_cut_by_the_list_agree_with_off_screen() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        for index in 0..CREATES {
            let created = act(&mut session, &type_into("crud-name", "Ada")).is_some()
                && act(&mut session, &type_into("crud-surname", "Lovelace")).is_some()
                && act(&mut session, &click("crud-create")).is_some();
            assert!(created, "{at}: create {index} runs");
        }
        let last = format!("crud-person-{}", 2 + CREATES);
        let list = bounds(&session, "crud-list", &at);
        let findings = session.lint();
        assert!(findings.is_ok(), "{at}: the lint reads the screen");
        let findings = findings.unwrap();
        let cut: Vec<&Finding> = findings
            .iter()
            .filter(|finding| finding.id == last)
            .filter(|finding| matches!(finding.defect, Defect::Clipped | Defect::OffScreen))
            .collect();
        assert!(cut.len() == 1, "{at}: the last row reads cut by the list");
        assert!(
            match cut[0].reason {
                Reason::Clipped { visible } => within(visible, list),
                Reason::OffScreen => !within(cut[0].bounds, list),
                _ => false,
            },
            "{at}: the cut is measured against the list's box"
        );
        assert!(
            refused(&mut session, &click(&last)) == Some(Cause::OffScreen),
            "{at}: a click on the last row is refused off-screen"
        );
    }
}

#[test]
fn an_app_with_no_font_is_refused_fonts_unavailable() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}: Crud");
        let harness = stand::boot(
            LeanTask::Crud,
            HarnessOptions {
                font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
                ..stand::options(incremental)
            },
        );
        let mut session = Session::start(slug(LeanTask::Crud), || harness)
            .expect("a lean task's slug is a label");
        let before = (session.harness().doc.snapshot(), focused(&session));
        let refusal = session.lint().err().map(|refusal| refusal.cause());
        assert!(
            refusal == Some(Cause::FontsUnavailable),
            "{at}: the lint is refused fonts-unavailable"
        );
        let after = (session.harness().doc.snapshot(), focused(&session));
        assert!(before == after, "{at}: the screen is unchanged");
    }
}

#[test]
fn the_lint_verb_answers_what_the_reader_reads() {
    for incremental in [false, true] {
        for case in 0..=LeanTask::ALL.len() {
            let at = format!("incremental={incremental}: case {case}");
            let mut session = match case {
                0 => fixtures::hold("lint", lint_page, incremental),
                task => hold(LeanTask::ALL[task - 1], incremental).0,
            };
            let read = session.lint();
            assert!(read.is_ok(), "{at}: the reader reads the screen");
            let read = read.unwrap();
            assert!(
                read.len()
                    == if case == 0 {
                        7
                    } else {
                        at_boot(LeanTask::ALL[case - 1]).len()
                    },
                "{at}: the reader reads the planted findings, and a task's measured ones at boot"
            );
            assert!(
                linted(&mut session, &lint()) == Some(read),
                "{at}: the verb answers the reader's findings"
            );
        }
    }
}
