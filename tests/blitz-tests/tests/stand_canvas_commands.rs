//! Circle Drawer's canvas read as draw commands through the driver's `canvas` verb, in process
//! only (Circle Drawer is served by no role), in both layout modes. At boot the summary reads a
//! canvas and its content box; one centre click adds a fill, a stroke and its diameter's glyphs,
//! and the full dump reads those lines. A line diff of the dumps before and after the click names
//! exactly the new circle's lines, all added and none removed, and a step that changes nothing
//! reads the same dump. Two reads of one state answer the same bytes, and every answer — the
//! summary, the full answer and every part, at boot, after the click, at 12 and at 100 circles —
//! is held with its byte count to one committed table, read on every CI OS. The 100-circle dump
//! reads in parts, each within the 10,000-byte budget, whose texts in order are the whole dump's.
//! The read changes nothing — the snapshot, both focus readings, the next diff and the clock
//! unmoved, delivered Timer ticks unapplied — refuses an unread id `not-found`, a deleted row's
//! id `stale` and an app with no usable font `fonts-unavailable`, which the `actions` read names
//! too, and answers `canvas` false for an element with no widget. A failure names a layout mode,
//! a state and an index only.

use blitz_test_harness::HarnessOptions;
use dioxus_native_dom::SNAPSHOT_TEXT_BUDGET;
use escher_driver::{Call, Cause, Outcome, Refusal, Session, answer_digest};
use seven_guis::stand::{self, LeanTask};

mod session_common;
use session_common::{
    CIRCLE_CANVAS as CANVAS, act, actioned, actions, advance, canvas, canvas_full, canvas_part,
    canvased, carried, circle_drawer, click, focused, hold, kinds, mcp_result_text, reach,
    refused_unchanged, slug,
};

/// One reading: the state, the level (`summary`, `full` or `part`), the part (0 but for a
/// part), the answer's bytes and its digest. Both layout modes read the same row.
type Row = (&'static str, &'static str, u32, usize, u64);

/// The states the table reads, in order.
const STATES: [&str; 4] = ["boot", "click", "12", "100"];

/// How many readings the table holds: three answers at boot, after the click and at 12
/// circles, and the summary, the full answer and six parts at 100.
const ROWS: usize = 17;

/// Every answer of every state.
const TABLE: &[Row] = &[
    ("boot", "summary", 0, 166, 0x2ef5df6a58e97aff),
    ("boot", "full", 0, 109, 0x07463ca344458c15),
    ("boot", "part", 1, 128, 0xd7c89add4fad0dca),
    ("click", "summary", 0, 166, 0xe0dcd8a94862ec23),
    ("click", "full", 0, 633, 0xa1b5d43dfcbea885),
    ("click", "part", 1, 652, 0xe607d5530f79027a),
    ("12", "summary", 0, 169, 0x573605104c26a6ae),
    ("12", "full", 0, 6294, 0x6b9e124a4ed7910a),
    ("12", "part", 1, 6313, 0xcec687282320e713),
    ("100", "summary", 0, 172, 0xbdc328e2b05b6077),
    ("100", "full", 0, 9810, 0x2447b44705d1d92f),
    ("100", "part", 1, 9810, 0x2447b44705d1d92f),
    ("100", "part", 2, 9842, 0xe0ad1bc28c4cbde7),
    ("100", "part", 3, 9936, 0x2019591dc2d25b86),
    ("100", "part", 4, 9831, 0x52f157b03c65b543),
    ("100", "part", 5, 9965, 0x76635348920f719f),
    ("100", "part", 6, 2994, 0xd618739a20869ac8),
];

/// The text a full answer or a part carries, read from its written line.
fn text_of(outcome: &Outcome) -> String {
    mcp_result_text(&outcome.to_json())
}

/// How many parts a part answer reads the dump as.
fn parts_of(outcome: &Outcome) -> Option<u32> {
    match outcome {
        Outcome::CanvasPart { parts, .. } => Some(*parts),
        _ => None,
    }
}

/// Every answer of `state`: the summary, the full answer and each part, with its row's key.
fn answers(session: &mut Session, at: &str) -> Vec<(&'static str, u32, Result<Outcome, Refusal>)> {
    let mut out = Vec::new();
    for (level, call) in [("summary", canvas(CANVAS)), ("full", canvas_full(CANVAS))] {
        out.push((level, 0, session.run(&call)));
    }
    let first = session.run(&canvas_part(CANVAS, 1));
    let parts = first
        .as_ref()
        .ok()
        .and_then(parts_of)
        .unwrap_or_else(|| panic!("{at}: part 1 answers a part"));
    out.push(("part", 1, first));
    for part in 2..=parts {
        out.push((
            "part",
            part,
            session.run(&canvas_part(CANVAS, i64::from(part))),
        ));
    }
    out
}

/// The lines of the longest common subsequence's complement: the lines of `after` it adds and
/// the lines of `before` it removes.
fn line_diff<'a>(before: &'a str, after: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let a: Vec<&str> = before.lines().collect();
    let b: Vec<&str> = after.lines().collect();
    let mut table = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            table[i][j] = if a[i] == b[j] {
                table[i + 1][j + 1] + 1
            } else {
                table[i + 1][j].max(table[i][j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let (mut added, mut removed) = (Vec::new(), Vec::new());
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            i += 1;
            j += 1;
        } else if j < b.len() && (i == a.len() || table[i][j + 1] >= table[i + 1][j]) {
            added.push(b[j]);
            j += 1;
        } else {
            removed.push(a[i]);
            i += 1;
        }
    }
    (added, removed)
}

/// The whole full dump of the canvas, read unpaged: the reading written as one answer.
fn whole_text(session: &mut Session, at: &str) -> String {
    let reading = session
        .canvas(CANVAS)
        .unwrap_or_else(|_| panic!("{at}: the canvas reads"));
    text_of(&Outcome::CanvasDump { reading })
}

#[test]
fn at_boot_and_after_a_click_the_summary_counts_the_new_circle_and_the_dump_reads_it() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} boot");
        let mut session = circle_drawer(incremental);
        assert!(carried(&session).is_empty(), "{at}: no circle at boot");
        let Some(Outcome::CanvasSummary {
            canvas: read,
            width,
            height,
            ..
        }) = canvased(&mut session, &canvas(CANVAS))
        else {
            panic!("{at}: the summary reads");
        };
        assert!(read, "{at}: the element holds a canvas");
        assert!(
            width > 0.0 && height > 0.0,
            "{at}: its content box has area"
        );
        let before = kinds(&mut session, &at);
        let dump_before = whole_text(&mut session, &at);

        let at = format!("incremental={incremental} click");
        assert!(
            act(&mut session, &click(CANVAS)).is_some(),
            "{at}: the click is accepted"
        );
        assert!(
            carried(&session).split(';').count() == 1,
            "{at}: the canvas carries one circle"
        );
        let after = kinds(&mut session, &at);
        let added: Vec<usize> = after.iter().zip(before).map(|(a, b)| a - b).collect();
        assert!(
            added[..6] == [0, 0, 0, 0, 1, 1] && added[6] >= 1 && added[7] == 0,
            "{at}: one more fill, one more stroke and its label's glyphs"
        );
        let dump_after = whole_text(&mut session, &at);
        let (new, gone) = line_diff(&dump_before, &dump_after);
        assert!(gone.is_empty(), "{at}: no line is removed");
        assert!(
            new.len() == added.iter().sum::<usize>(),
            "{at}: the lines added are the new circle's commands"
        );
        let words: Vec<&str> = new
            .iter()
            .map(|line| line.trim_start().split(' ').next().unwrap_or(""))
            .collect();
        assert!(
            words.iter().filter(|w| **w == "fill").count() == 1
                && words.iter().filter(|w| **w == "stroke").count() == 1
                && words.iter().filter(|w| **w == "glyphs").count() == added[6],
            "{at}: the dump reads the fill, the stroke and the glyph runs"
        );

        let at = format!("incremental={incremental} advance");
        assert!(
            act(&mut session, &advance(100)).is_some(),
            "{at}: the advance is accepted"
        );
        let dump_later = whole_text(&mut session, &at);
        let (new, gone) = line_diff(&dump_after, &dump_later);
        assert!(
            new.is_empty() && gone.is_empty(),
            "{at}: a step that changes nothing reads an empty diff"
        );
    }
}

#[test]
fn every_answer_reads_its_table_and_two_reads_read_the_same_bytes() {
    let mut measured = Vec::new();
    for incremental in [false, true] {
        for state in STATES {
            let at = format!("incremental={incremental} {state}");
            let mut session = reach(state, incremental);
            let first = answers(&mut session, &at);
            let second = answers(&mut session, &at);
            for (index, ((level, part, one), (_, _, two))) in first.iter().zip(&second).enumerate()
            {
                let one = one
                    .as_ref()
                    .unwrap_or_else(|_| panic!("{at}: answer {index} is accepted"))
                    .to_json();
                let two = two
                    .as_ref()
                    .unwrap_or_else(|_| panic!("{at}: answer {index} is accepted"))
                    .to_json();
                assert!(
                    one == two,
                    "{at}: answer {index} reads the same bytes twice"
                );
                let digest = answer_digest(&Ok(first[index]
                    .2
                    .clone()
                    .unwrap_or_else(|_| panic!("{at}: answer {index}"))));
                measured.push((incremental, state, *level, *part, one.len(), digest));
            }
        }
    }
    let rows = TABLE.len();
    assert!(rows == ROWS, "the table holds its stated row count");
    assert!(
        measured.len() == 2 * rows,
        "the table holds every answer of one layout mode"
    );
    for (index, (incremental, state, level, part, bytes, digest)) in measured.iter().enumerate() {
        let at = format!("incremental={incremental} {state} {index}");
        let row = TABLE[index % rows];
        assert!(
            (row.0, row.1, row.2) == (*state, *level, *part),
            "{at}: the table's order"
        );
        assert!(
            (row.3, row.4) == (*bytes, *digest),
            "{at}: the answer reads its table"
        );
    }
}

#[test]
fn a_full_dump_over_the_budget_reads_in_parts_each_within_it() {
    let table_parts = TABLE
        .iter()
        .filter(|row| row.0 == "100" && row.1 == "part")
        .count();
    for incremental in [false, true] {
        let at = format!("incremental={incremental} 100");
        let mut session = reach("100", incremental);
        let whole = whole_text(&mut session, &at);
        assert!(
            whole.len() > SNAPSHOT_TEXT_BUDGET,
            "{at}: the dump is over the budget"
        );
        let first = session.run(&canvas_full(CANVAS));
        let Ok(Outcome::CanvasPart { part: 1, parts, .. }) = first else {
            panic!("{at}: a dump over the budget answers part 1");
        };
        assert!(
            parts as usize == table_parts && parts > 1,
            "{at}: the dump reads in its table's parts"
        );
        let mut joined = String::new();
        for part in 1..=parts {
            let at = format!("{at} part {part}");
            let Some(outcome) = canvased(&mut session, &canvas_part(CANVAS, i64::from(part)))
            else {
                panic!("{at}: the part is accepted");
            };
            assert!(
                outcome.to_json().len() <= SNAPSHOT_TEXT_BUDGET,
                "{at}: the part is within the budget"
            );
            let Outcome::CanvasPart { reading, .. } = &outcome else {
                panic!("{at}: a part answers a part");
            };
            assert!(
                reading.fonts.is_empty() == (part > 1),
                "{at}: fonts stand in part 1 alone"
            );
            joined.push_str(&text_of(&outcome));
        }
        assert!(
            joined == whole,
            "{at}: the parts in order are the whole dump"
        );
        let past = canvased(&mut session, &canvas_part(CANVAS, i64::from(parts) + 1));
        let Some(Outcome::CanvasPart { reading, .. }) = past else {
            panic!("{at}: a part past the last answers a part");
        };
        assert!(
            reading.commands.is_empty() && reading.fonts.is_empty() && reading.images.is_empty(),
            "{at}: a part past the last is empty"
        );
    }
}

#[test]
fn a_canvas_read_changes_nothing_with_timer_ticks_pending() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} timer");
        let (mut session, ticks) = hold(LeanTask::Timer, incremental);
        session.harness_mut().base_mut().take_changed_nodes();
        ticks.deliver(5);
        let before = (session.harness().doc.snapshot(), focused(&session));
        let time = session.harness().time();
        for (index, call) in [canvas("timer-elapsed"), canvas_full("timer-elapsed")]
            .iter()
            .enumerate()
        {
            let answer = canvased(&mut session, call);
            let none = match answer {
                Some(Outcome::CanvasSummary {
                    canvas,
                    width,
                    height,
                    kinds,
                    ..
                }) => !canvas && width == 0.0 && height == 0.0 && kinds == [0; 8],
                Some(Outcome::CanvasDump { reading }) => {
                    !reading.canvas && reading.commands.is_empty()
                }
                _ => false,
            };
            assert!(none, "{at}: answer {index} reads no canvas");
        }
        let after = (session.harness().doc.snapshot(), focused(&session));
        assert!(
            before == after && before.0.diff(&after.0).is_empty(),
            "{at}: the snapshot, the focus readings and the diff are unchanged"
        );
        assert!(
            !session.harness().base().has_changes(),
            "{at}: the read marks no change"
        );
        assert!(
            session.harness().time() == time,
            "{at}: the clock is unmoved"
        );
        session.harness_mut().pump();
        assert!(
            session.harness().doc.snapshot() != after.0,
            "{at}: a pass applies the ticks the read left pending"
        );

        let at = format!("incremental={incremental} click");
        let mut session = reach("click", incremental);
        session.harness_mut().base_mut().take_changed_nodes();
        let screen = session.harness().doc.snapshot();
        let summary = session.run(&canvas(CANVAS)).map(|o| o.to_json());
        let dump = session.run(&canvas_full(CANVAS)).map(|o| o.to_json());
        assert!(
            session.run(&canvas(CANVAS)).map(|o| o.to_json()) == summary
                && session.run(&canvas_full(CANVAS)).map(|o| o.to_json()) == dump,
            "{at}: the summary and the dump read the same before and after"
        );
        assert!(
            session.harness().doc.snapshot() == screen && !session.harness().base().has_changes(),
            "{at}: the read changes nothing"
        );
    }
}

#[test]
fn an_unread_id_a_deleted_rows_id_and_a_fontless_app_are_refused_changing_nothing() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental} crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        let (cause, unchanged) = refused_unchanged(&mut session, &canvas("canvas-no-such-id"));
        assert!(
            cause == Some(Cause::NotFound) && unchanged,
            "{at}: an unread id is not found, nothing changed"
        );
        assert!(
            matches!(
                canvased(&mut session, &canvas("crud-person-1")),
                Some(Outcome::CanvasSummary { canvas: false, .. })
            ),
            "{at}: a row holds no canvas"
        );
        assert!(
            act(&mut session, &click("crud-person-1")).is_some(),
            "{at}: the row is selected"
        );
        assert!(
            act(&mut session, &click("crud-delete")).is_some(),
            "{at}: the row is deleted"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &canvas_full("crud-person-1"));
        assert!(
            cause == Some(Cause::Stale) && unchanged,
            "{at}: the deleted row's id is stale, nothing changed"
        );

        let at = format!("incremental={incremental} no font");
        let harness = stand::boot(
            LeanTask::Crud,
            HarnessOptions {
                font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
                ..stand::options(incremental)
            },
        );
        let mut session =
            Session::start(slug(LeanTask::Crud), || harness).expect("a lean task's slug");
        let calls: [Call; 2] = [canvas("crud-list"), canvas_full("crud-list")];
        for (index, call) in calls.iter().enumerate() {
            assert!(
                refused_unchanged(&mut session, call) == (Some(Cause::FontsUnavailable), true),
                "{at}: read {index} is refused fonts-unavailable, nothing changed"
            );
        }
        let nodes = actioned(&mut session, &actions()).expect("the actions read answers");
        assert!(!nodes.is_empty(), "{at}: nodes are listed");
        for (index, node) in nodes.iter().enumerate() {
            let cause = node
                .verbs
                .iter()
                .find(|verb| verb.verb == "canvas")
                .and_then(|verb| verb.cause);
            assert!(
                cause == Some(Cause::FontsUnavailable),
                "{at}: node {index} reads fonts-unavailable for canvas"
            );
        }
    }
}
