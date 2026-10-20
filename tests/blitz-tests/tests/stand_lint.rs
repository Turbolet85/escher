//! `Session::lint` reads the screen of a held instance for six defects, each named by the stable
//! id of the element it is found on with its measured reason, in both layout modes. On a page
//! that plants each defect beside six negatives it names exactly the seven planted findings in
//! document order, with their reasons; on each lean stand task at boot it reads nothing; it
//! changes nothing; after twelve Creates it names the last CRUD row cut by the list's box, the
//! row a `click` is refused `off-screen` on; and an app with no font is refused
//! `fonts-unavailable`. A failure message carries a layout mode, a task and a row index only.

use blitz_test_harness::HarnessOptions;
use dioxus::prelude::*;
use escher_driver::{Bounds, Cause, Defect, Finding, Reason, Session};
use seven_guis::stand::{self, LeanTask};

mod fixtures;
mod session_common;
use session_common::{act, click, focused, hold, refused, slug, type_into};

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
fn the_stand_at_boot_reads_no_finding() {
    for incremental in [false, true] {
        for (index, task) in LeanTask::ALL.into_iter().enumerate() {
            let at = format!("incremental={incremental}: task {index}");
            let (mut session, _ticks) = hold(task, incremental);
            let findings = session.lint();
            assert!(findings.is_ok(), "{at}: the lint reads the task");
            let defects: Vec<&str> = findings
                .as_ref()
                .unwrap()
                .iter()
                .map(|finding| finding.defect.name())
                .collect();
            assert!(defects.is_empty(), "{at}: no finding, read {defects:?}");
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
