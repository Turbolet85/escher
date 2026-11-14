//! Each of the six served tasks reads one state the same way on every OS the tests run on. Held
//! in process as the session host holds it, in both layout modes, each task is read through the
//! driver — `snapshot` (every part of a screen over the budget), `lint` and `actions` — at boot
//! and again after one acting step reached by `click`, `type` or `advance` alone, never `press`.
//! The digest the driver keeps of each answer, `answer_digest`, is held equal to one committed
//! table, so a run on linux, windows and macOS reading the one table is the cross-OS reading;
//! the comparison is by readings, never by a shot's bytes. A failure names the task, the moment,
//! the verb, the part and the layout mode only.

use escher_driver::{Call, Cause, Outcome, Session, answer_digest};
use seven_guis::stand::{self, LeanTask};

mod session_common;
use session_common::{actions, advance, click, hold, lint, snapshot, snapshot_part, type_into};

/// One reading: the task, the moment (`boot` or `step`), the verb, the part (0 for an answer
/// that is not one part of a paged screen) and the digest of the answer.
type Row = (&'static str, &'static str, &'static str, u32, u64);

/// How many readings the table holds.
const ROWS: usize = 48;

/// Every reading of the six tasks, in the order the check takes them.
const TABLE: [Row; ROWS] = [
    ("counter", "boot", "snapshot", 0, 0x5802032aa3564184),
    ("counter", "boot", "lint", 0, 0x7da3ac622b6348c1),
    ("counter", "boot", "actions", 0, 0x33919762299c84de),
    ("counter", "step", "snapshot", 0, 0x5ee0a345954c80de),
    ("counter", "step", "lint", 0, 0x7da3ac622b6348c1),
    ("counter", "step", "actions", 0, 0x33919762299c84de),
    ("flight-booker", "boot", "snapshot", 0, 0x12332f45f99802fc),
    ("flight-booker", "boot", "lint", 0, 0x7da3ac622b6348c1),
    ("flight-booker", "boot", "actions", 0, 0x0583b4b0112201fe),
    ("flight-booker", "step", "snapshot", 0, 0xc53198db16cc080d),
    ("flight-booker", "step", "lint", 0, 0x7da3ac622b6348c1),
    ("flight-booker", "step", "actions", 0, 0xe11399e4cc08294a),
    ("timer", "boot", "snapshot", 0, 0x8ca23163734f31c4),
    ("timer", "boot", "lint", 0, 0x7da3ac622b6348c1),
    ("timer", "boot", "actions", 0, 0xeaed8e5ec6393447),
    ("timer", "step", "snapshot", 0, 0x734eb174e18bac7b),
    ("timer", "step", "lint", 0, 0x7da3ac622b6348c1),
    ("timer", "step", "actions", 0, 0xeaed8e5ec6393447),
    ("crud", "boot", "snapshot", 0, 0x9a0efb90da84612d),
    ("crud", "boot", "lint", 0, 0x7da3ac622b6348c1),
    ("crud", "boot", "actions", 0, 0xb15f0c4eb7faa0cf),
    ("crud", "step", "snapshot", 0, 0x2c9623923ba52de6),
    ("crud", "step", "lint", 0, 0x7da3ac622b6348c1),
    ("crud", "step", "actions", 0, 0xc121806d5e841dcf),
    ("cells", "boot", "snapshot", 0, 0x1a88b0b2a0752027),
    ("cells", "boot", "snapshot", 1, 0x1a88b0b2a0752027),
    ("cells", "boot", "snapshot", 2, 0x72aeb5590935890f),
    ("cells", "boot", "snapshot", 3, 0x2a8238c1074d77cd),
    ("cells", "boot", "snapshot", 4, 0x0d18453d8dc71ef2),
    ("cells", "boot", "snapshot", 5, 0xb6880a8a027ba694),
    ("cells", "boot", "snapshot", 6, 0x4fc8c84fd863b649),
    ("cells", "boot", "lint", 0, 0xd2449727e07a4eb0),
    ("cells", "boot", "actions", 0, 0xcb5c4cde60367d2b),
    ("cells", "step", "snapshot", 0, 0x8d0c39bd1eaa6138),
    ("cells", "step", "snapshot", 1, 0x8d0c39bd1eaa6138),
    ("cells", "step", "snapshot", 2, 0xd51df52941206cbc),
    ("cells", "step", "snapshot", 3, 0x2d53ea33452b3ce7),
    ("cells", "step", "snapshot", 4, 0xfebd502c8071fdba),
    ("cells", "step", "snapshot", 5, 0x62d44dc65047081e),
    ("cells", "step", "snapshot", 6, 0xd657d46148e54bbc),
    ("cells", "step", "lint", 0, 0xd2449727e07a4eb0),
    ("cells", "step", "actions", 0, 0x18f689d9261f8986),
    ("load", "boot", "snapshot", 0, 0x863ea86223b43ea4),
    ("load", "boot", "lint", 0, 0x7da3ac622b6348c1),
    ("load", "boot", "actions", 0, 0x67e8066e8fb7b56f),
    ("load", "step", "snapshot", 0, 0x03296567ce0abc73),
    ("load", "step", "lint", 0, 0x7da3ac622b6348c1),
    ("load", "step", "actions", 0, 0x67e8066e8fb7b56f),
];

/// A session on the served task `task`, booted through the stand as the session host boots it.
fn hold_served(task: &str, incremental: bool) -> Session {
    let lean = LeanTask::ALL
        .into_iter()
        .find(|lean| session_common::slug(*lean) == task);
    if let Some(lean) = lean {
        return hold(lean, incremental).0;
    }
    let harness = match task {
        "cells" => stand::boot_cells(stand::options(incremental)),
        "load" => stand::boot_load(stand::options(incremental)),
        _ => panic!("a served task's label"),
    };
    Session::start(task, || harness).expect("the label is one of the closed set")
}

/// The one acting step that moves `task` to a second state, call by call.
fn step(task: &str) -> Vec<Call> {
    match task {
        "counter" => vec![click("counter-increment")],
        "flight-booker" => vec![
            type_into("flight-type", "return"),
            type_into("flight-return-date", "02.01.2026"),
        ],
        "timer" => vec![advance(1000)],
        "crud" => vec![
            type_into("crud-name", "Ada"),
            type_into("crud-surname", "Lovelace"),
            click("crud-create"),
        ],
        "cells" => vec![click("cell-A0"), type_into("cell-input", "42")],
        "load" => vec![click("load-fetch")],
        _ => panic!("a served task's label"),
    }
}

/// Runs `call` and returns the digest of its answer; a refused call fails the check.
#[track_caller]
fn digest(session: &mut Session, call: &Call, at: &str) -> (u64, Outcome) {
    let answer = session.run(call);
    let digest = answer_digest(&answer);
    let outcome = answer.unwrap_or_else(|_| panic!("{at}: the call is accepted"));
    (digest, outcome)
}

/// The readings of the screen `session` holds now, at `moment`.
#[track_caller]
fn read(session: &mut Session, task: &'static str, moment: &'static str, mode: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    let at = format!("{mode} {task} {moment} snapshot part 0");
    let (whole, outcome) = digest(session, &snapshot(), &at);
    rows.push((task, moment, "snapshot", 0, whole));
    if let Outcome::Part { parts, .. } = outcome {
        for n in 1..=parts {
            let at = format!("{mode} {task} {moment} snapshot part {n}");
            let (part, _) = digest(session, &snapshot_part(i64::from(n)), &at);
            rows.push((task, moment, "snapshot", n, part));
        }
    }
    let at = format!("{mode} {task} {moment} lint part 0");
    rows.push((task, moment, "lint", 0, digest(session, &lint(), &at).0));
    let at = format!("{mode} {task} {moment} actions part 0");
    rows.push((
        task,
        moment,
        "actions",
        0,
        digest(session, &actions(), &at).0,
    ));
    rows
}

/// Reads `task` at boot and after its step, in both layout modes, and holds every reading to
/// the table's rows for that task.
fn holds(task: &'static str) {
    let expected: Vec<Row> = TABLE.into_iter().filter(|row| row.0 == task).collect();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_served(task, incremental);
        assert!(
            session.run(&lint()).err().map(|refusal| refusal.cause())
                != Some(Cause::FontsUnavailable),
            "{mode} {task}: the held app can paint text"
        );
        let mut read_rows = read(&mut session, task, "boot", &mode);
        for (index, call) in step(task).iter().enumerate() {
            assert!(
                session.run(call).is_ok(),
                "{mode} {task}: step call {index} is accepted"
            );
        }
        read_rows.extend(read(&mut session, task, "step", &mode));
        assert!(
            read_rows.len() == expected.len(),
            "{mode} {task}: the task reads as many readings as the table holds for it"
        );
        for (read_row, row) in read_rows.iter().zip(&expected) {
            let (_, moment, verb, part, _) = *row;
            assert!(
                read_row == row,
                "{mode} {task} {moment} {verb} part {part}: the reading is the table's"
            );
        }
    }
}

#[test]
fn the_table_counts_its_rows_and_covers_the_six_tasks() {
    assert!(TABLE.len() == ROWS, "the table holds its stated row count");
    for task in ["counter", "flight-booker", "timer", "crud", "cells", "load"] {
        for moment in ["boot", "step"] {
            for verb in ["snapshot", "lint", "actions"] {
                assert!(
                    TABLE
                        .iter()
                        .any(|row| row.0 == task && row.1 == moment && row.2 == verb),
                    "{task} {moment} {verb}: the table holds the reading"
                );
            }
        }
    }
}

#[test]
fn the_counter_reads_its_table() {
    holds("counter");
}

#[test]
fn the_flight_booker_reads_its_table() {
    holds("flight-booker");
}

#[test]
fn the_timer_reads_its_table() {
    holds("timer");
}

#[test]
fn crud_reads_its_table() {
    holds("crud");
}

#[test]
fn cells_reads_its_table() {
    holds("cells");
}

#[test]
fn the_load_page_reads_its_table() {
    holds("load");
}
