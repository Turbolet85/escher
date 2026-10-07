//! Every driver command leaves one line in escher's sink, and nothing a command handled is on
//! it. With the sink installed over a capture and `RUST_LOG=info`, in both layout modes, each
//! call run through `Session::run` — the six verbs on the stand's CRUD task and its Timer, and
//! a refused call for each of the eight causes, the `covered` one on a fixture booted with the
//! stand's options — leaves exactly one line under `escher_driver`: at INFO, stamped with the
//! service identity, its `span` reading `command`, its `verb` the table's word and absent for a
//! verb the table lacks. Any other line a call leaves is an engine record, which a build with
//! the engine's own call sites compiled in logs beside it. An acting call's line carries its
//! settle reading and the three list lengths of the diff the call returned, which this file
//! first holds equal to the diff of its own snapshots; a refused call's line carries the name
//! of the cause the call returned and none of the settle or diff fields; a `snapshot`'s carries
//! neither. Over the whole capture no field name of a driver line is outside the stated set,
//! none of its values is redacted, and no stable id, row text, label text or typed text of the
//! driven screens occurs on any line. With `RUST_LOG` removed a command writes no driver line
//! at all. The level filter is read from the process's environment when the sink is installed,
//! so each reading is made in a child: this test binary re-run on one ignored test. No check
//! here sleeps, reads a clock or makes a pass of its own. A failure message carries a layout
//! mode, a task and a call index, or a kind and a count — never an id, a name, a value, typed
//! text or a captured line.

use std::io;
use std::process::Command;
use std::sync::{Arc, Mutex};

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::SnapshotDiff;
use escher_driver::{ArgValue, Busy, CAUSES, Call, Cause, Outcome, Refusal, Session, VERBS};
use seven_guis::stand::{self, LeanTask};

mod common;
mod session_common;
use session_common::{advance, click, hold, press, scroll, type_into};

/// The text the driver types into the CRUD name field: synthetic, and no part of any id.
const SECRET: &str = "synthetic-sp-4Kx";

/// How many driver clicks on Create put a row's centre past the CRUD list's box at the stand's
/// viewport, as measured on the stand.
const CREATES: usize = 12;

/// An id no element of the stand or of the fixture carries.
const NOBODY: &str = "act-spans-names-no-element";

/// A verb the table lacks.
const UNKNOWN_VERB: &str = "capture-zq7";

/// The target the driver's span is logged under.
const TARGET: &str = "escher_driver";

/// The fields a driver line may carry after the service identity: the span's name, the span's
/// own eight, and the three the sink adds when a span closes.
const LINE_FIELDS: [&str; 12] = [
    "span",
    "verb",
    "cause",
    "settled",
    "busy",
    "passes",
    "added",
    "removed",
    "changed",
    "message",
    "time.busy",
    "time.idle",
];

/// The fields only a call that ran as an acting verb carries.
const ACTING_FIELDS: [&str; 6] = ["settled", "busy", "passes", "added", "removed", "changed"];

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl io::Write for Capture {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Capture {
    /// Installs escher's sink over a capture: once per process, so once per child.
    fn install() -> Self {
        let capture = Capture::default();
        let sink = capture.clone();
        let installed =
            escher_telemetry::init_with_writer(escher_telemetry::service_identity!(), move || {
                sink.clone()
            });
        assert!(
            installed == Ok(escher_telemetry::InitOutcome::Installed),
            "the sink installs over the capture"
        );
        capture
    }

    fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }

    /// What the sink has written from byte `from` on.
    fn since(&self, from: usize) -> String {
        String::from_utf8_lossy(&self.0.lock().unwrap()[from..]).into_owned()
    }
}

fn is_driver_line(line: &str) -> bool {
    line.split(' ').nth(2) == Some(TARGET)
}

/// Whether `line` is an engine record. A build that compiles the engine's own `tracing` call
/// sites in — the workspace's does — has them log beside a command, each printed by the sink
/// with all but its safe fields redacted.
fn is_engine_line(line: &str) -> bool {
    line.split(' ').nth(2).is_some_and(|target| {
        escher_telemetry::ENGINE_TARGET_PREFIXES
            .iter()
            .any(|prefix| target.starts_with(prefix))
    })
}

/// One line of the sink: its level, and the `name=value` pairs after its target.
struct Record<'l> {
    level: &'l str,
    pairs: Vec<(&'l str, &'l str)>,
}

impl<'l> Record<'l> {
    /// Reads `line`: `None` when a part after its target is no pair. No value on a driver line
    /// holds a space.
    fn read(line: &'l str) -> Option<Self> {
        let mut parts = line.split(' ');
        let (_time, level, _target) = (parts.next()?, parts.next()?, parts.next()?);
        let pairs = parts
            .map(|part| part.split_once('='))
            .collect::<Option<Vec<_>>>()?;
        Some(Record { level, pairs })
    }

    fn has(&self, name: &str) -> bool {
        self.pairs.iter().any(|(field, _)| *field == name)
    }

    /// The value of the one pair named `name`: `None` when no pair or more than one is.
    fn get(&self, name: &str) -> Option<&'l str> {
        let mut values = self
            .pairs
            .iter()
            .filter(|(field, _)| *field == name)
            .map(|(_, value)| *value);
        let value = values.next()?;
        values.next().is_none().then_some(value)
    }

    /// The word a pair holds: the sink writes a word in quotes.
    fn word(&self, name: &str) -> Option<&'l str> {
        self.get(name).map(|value| value.trim_matches('"'))
    }

    fn count(&self, name: &str) -> Option<usize> {
        self.get(name).and_then(|value| value.parse().ok())
    }
}

/// What a call is stated to return.
#[derive(Clone, Copy)]
enum Expect {
    Screen,
    Acts,
    Refused(Cause),
}

/// The settle reading and the diff of an outcome an acting verb returned.
fn acting(outcome: &Outcome) -> Option<(bool, Option<Busy>, &SnapshotDiff)> {
    match outcome {
        Outcome::Screen { .. } => None,
        Outcome::Acted {
            settled,
            busy,
            diff,
        }
        | Outcome::Advanced {
            settled,
            busy,
            diff,
            ..
        }
        | Outcome::Scrolled {
            settled,
            busy,
            diff,
            ..
        } => Some((*settled, *busy, diff)),
    }
}

/// What the driven screens hold and the calls supplied: none of it may be on a line.
#[derive(Default)]
struct Needles {
    ids: Vec<String>,
    names: Vec<String>,
}

impl Needles {
    /// Adds the screen `session` holds now: every author key that holds a `-` (a key that is an
    /// ordinary word could not be attributed), every row's text and every label's text.
    fn read(&mut self, session: &Session) {
        let harness = session.harness();
        let ids = harness
            .doc
            .element_ids()
            .into_iter()
            .map(|(_, id)| id)
            .filter(|id| !id.contains('/') && id.contains('-'));
        for id in ids {
            if !self.ids.contains(&id) {
                self.ids.push(id);
            }
        }
        for selector in ["#crud-list > div", "label"] {
            for node in harness.query_all(selector) {
                let text = harness
                    .base()
                    .get_node(node)
                    .expect("a queried node exists")
                    .text_content();
                if !self.names.contains(&text) {
                    self.names.push(text);
                }
            }
        }
    }
}

/// One layout mode's calls in the records child, and what their lines read.
struct Run<'c> {
    capture: &'c Capture,
    mode: String,
    start: usize,
    read: usize,
    calls: usize,
    verbs: Vec<String>,
    causes: Vec<String>,
}

impl<'c> Run<'c> {
    fn new(capture: &'c Capture, incremental: bool) -> Self {
        let start = capture.len();
        Run {
            capture,
            mode: format!("incremental={incremental}"),
            start,
            read: start,
            calls: 0,
            verbs: Vec::new(),
            causes: Vec::new(),
        }
    }

    /// Runs `call` between this file's own two snapshots and holds the one line it left
    /// against what it returned.
    fn drive(
        &mut self,
        task: &str,
        session: &mut Session,
        call: &Call,
        expect: Expect,
    ) -> Result<Outcome, Refusal> {
        let at = format!("{}: {task}: call {}", self.mode, self.calls);
        self.calls += 1;

        let before = session.harness().doc.snapshot();
        let result = session.run(call);
        let after = session.harness().doc.snapshot();

        let written = self.capture.since(self.read);
        self.read += written.len();
        let (driver, others): (Vec<&str>, Vec<&str>) =
            written.lines().partition(|line| is_driver_line(line));
        let unadmitted = others.iter().filter(|line| !is_engine_line(line)).count();
        assert!(
            driver.len() == 1 && unadmitted == 0,
            "{at}: the call left {} driver lines and {unadmitted} lines under no engine target",
            driver.len()
        );
        let Some(record) = Record::read(driver[0]) else {
            panic!("{at}: the line is pairs after its target");
        };
        assert!(record.level == "INFO", "{at}: the line is at INFO");
        assert!(
            record.get("service.name") == Some(env!("CARGO_PKG_NAME"))
                && record.has("service.version"),
            "{at}: the line carries the service identity"
        );
        assert!(
            record.word("span") == Some("command"),
            "{at}: the line names the command span"
        );
        let verb = escher_driver::verb(&call.verb).map(|row| row.name);
        assert!(
            record.word("verb") == verb,
            "{at}: the line's verb is the table's word, and absent for a verb the table lacks"
        );
        assert!(
            record.has("time.busy") && record.has("time.idle"),
            "{at}: the line carries both timings"
        );

        match (&result, expect) {
            (Err(refusal), Expect::Refused(cause)) => {
                assert!(
                    refusal.cause() == cause,
                    "{at}: the call is refused with the cause stated for it"
                );
                assert!(
                    record.word("cause") == Some(cause.name()),
                    "{at}: the line's cause is the name of the cause the call returned"
                );
                assert!(
                    ACTING_FIELDS.iter().all(|field| !record.has(field)),
                    "{at}: a refused call's line carries no settle or diff field"
                );
                self.causes.extend(record.word("cause").map(str::to_string));
            }
            (Ok(Outcome::Screen { .. }), Expect::Screen) => {
                assert!(
                    !record.has("cause") && ACTING_FIELDS.iter().all(|field| !record.has(field)),
                    "{at}: a snapshot's line carries its verb alone"
                );
                self.verbs.extend(verb.map(str::to_string));
            }
            (Ok(outcome), Expect::Acts) => {
                let Some((settled, busy, diff)) = acting(outcome) else {
                    panic!("{at}: the call returns an acting verb's outcome");
                };
                assert!(
                    *diff == before.diff(&after),
                    "{at}: the call returns the diff of the snapshots before and after it"
                );
                assert!(settled && busy.is_none(), "{at}: the call returns settled");
                assert!(
                    record.get("settled") == Some("true")
                        && record.count("passes").is_some_and(|passes| passes >= 1)
                        && !record.has("busy"),
                    "{at}: the line carries the settle reading the call returned, with its passes"
                );
                assert!(
                    record.count("added") == Some(diff.added.len())
                        && record.count("removed") == Some(diff.removed.len())
                        && record.count("changed") == Some(diff.changed.len()),
                    "{at}: the line's three counts are the lengths of the returned diff's lists"
                );
                assert!(
                    !record.has("cause"),
                    "{at}: a call that ran carries no cause"
                );
                self.verbs.extend(verb.map(str::to_string));
            }
            _ => panic!("{at}: the call returns the shape stated for it"),
        }
        result
    }

    /// Holds the whole of what this run left in the capture: one driver line per call, the six
    /// verbs and the eight causes read back, no field name outside the stated set, no redacted
    /// value, and none of `needles` nor of what the calls supplied.
    fn finish(self, needles: &Needles) {
        let mode = &self.mode;
        let written = self.capture.since(self.start);
        let driver: Vec<&str> = written
            .lines()
            .filter(|line| is_driver_line(line))
            .collect();
        assert!(
            !written.is_empty() && driver.len() == self.calls,
            "{mode}: {} driver lines for {} calls",
            driver.len(),
            self.calls
        );

        let read_back = |words: &[String], table: Vec<&str>| {
            table
                .iter()
                .all(|word| words.iter().any(|read| read == word))
        };
        assert!(
            read_back(&self.verbs, VERBS.iter().map(|verb| verb.name).collect()),
            "{mode}: each of the six verbs ran and was read back from its line"
        );
        assert!(
            CAUSES.len() == 8 && read_back(&self.causes, CAUSES.map(Cause::name).to_vec()),
            "{mode}: each of the eight causes was read back from its own refused call"
        );

        let supplied = [SECRET, NOBODY, UNKNOWN_VERB];
        assert!(
            needles.ids.len() >= 20 && needles.names.len() >= 6,
            "{mode}: {} id needles and {} name needles were read",
            needles.ids.len(),
            needles.names.len()
        );
        assert!(
            common::rendered(LeanTask::Crud)
                .iter()
                .filter(|id| id.contains('-'))
                .all(|id| needles.ids.iter().any(|needle| needle == id))
                && needles.ids.iter().any(|id| id == "fx-under"),
            "{mode}: the driven screens' author keys are among the id needles"
        );
        assert!(
            needles
                .ids
                .iter()
                .chain(&needles.names)
                .all(|needle| !needle.trim().is_empty()),
            "{mode}: no needle is empty"
        );

        let pairs: Vec<(&str, &str)> = driver
            .iter()
            .filter_map(|line| Record::read(line))
            .flat_map(|record| record.pairs.into_iter().skip(2))
            .collect();
        let outside = pairs
            .iter()
            .filter(|(field, _)| !LINE_FIELDS.contains(field))
            .count();
        let redacted = pairs
            .iter()
            .filter(|(_, value)| *value == escher_telemetry::REDACTED)
            .count();
        let found = |needles: &[String]| {
            needles
                .iter()
                .filter(|needle| written.contains(needle.as_str()))
                .count()
        };
        let (ids_found, names_found) = (found(&needles.ids), found(&needles.names));
        let supplied_found: usize = supplied
            .iter()
            .map(|text| written.matches(text).count())
            .sum();
        assert!(
            outside == 0
                && redacted == 0
                && ids_found == 0
                && names_found == 0
                && supplied_found == 0,
            "{mode}: {outside} field names outside the stated set, {redacted} redacted values, \
             {ids_found} of {} id needles, {names_found} of {} name needles and {supplied_found} \
             occurrences of a text a call supplied, in {} lines",
            needles.ids.len(),
            needles.names.len(),
            written.lines().count()
        );
    }
}

/// One button under one positioned overlay: the `covered` case the stand lacks.
fn covered_fixture() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-under",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px;",
                "Under"
            }
            div {
                id: "fx-cover",
                style: "position: absolute; left: 0; top: 0; width: 200px; height: 140px; z-index: 5; background: #cccccc;",
            }
        }
    }
}

/// Re-runs this test binary on its one ignored test `child`, with `RUST_LOG` set to `level` or
/// removed, and asserts the child ran and passed. What a failing child wrote to stderr is its
/// own failure message, which holds nothing of a screen.
fn run_child(child: &str, level: Option<&str>) {
    let mut command = Command::new(std::env::current_exe().expect("the test binary has a path"));
    command.args(["--ignored", "--exact", child, "--nocapture"]);
    match level {
        Some(level) => command.env("RUST_LOG", level),
        None => command.env_remove("RUST_LOG"),
    };
    let output = command.output().expect("the test binary runs");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stdout.contains("1 passed; 0 failed"),
        "the child {child} did not pass\nstderr:\n{stderr}"
    );
}

#[test]
fn each_driver_command_leaves_one_line_holding_nothing_it_handled() {
    run_child("child_records_each_command_at_info", Some("info"));
}

#[test]
fn a_driver_command_writes_no_line_at_the_default_level() {
    run_child("child_drives_at_the_default_level", None);
}

#[test]
#[ignore = "spawned as a child process by each_driver_command_leaves_one_line_holding_nothing_it_handled"]
fn child_records_each_command_at_info() {
    let capture = Capture::install();
    for incremental in [false, true] {
        let mut run = Run::new(&capture, incremental);
        let mut needles = Needles::default();

        let (mut crud, _ticks) = hold(LeanTask::Crud, incremental);
        let snapshot = Call {
            verb: "snapshot".to_string(),
            args: Vec::new(),
        };
        let _ = run.drive("Crud", &mut crud, &snapshot, Expect::Screen);
        let _ = run.drive(
            "Crud",
            &mut crud,
            &type_into("crud-name", SECRET),
            Expect::Acts,
        );
        let _ = run.drive("Crud", &mut crud, &press("tab", false), Expect::Acts);
        let mut last_row = None;
        for _ in 0..CREATES {
            let created = run.drive("Crud", &mut crud, &click("crud-create"), Expect::Acts);
            last_row = created
                .ok()
                .as_ref()
                .and_then(acting)
                .and_then(|(_, _, diff)| diff.added.first())
                .map(|row| row.id.clone());
        }
        let Some(last_row) = last_row else {
            panic!("{}: Crud: the last click on Create adds a row", run.mode);
        };
        needles.read(&crud);

        for (call, expect) in [
            (click("crud-delete"), Expect::Refused(Cause::Disabled)),
            (click(&last_row), Expect::Refused(Cause::OffScreen)),
            (scroll(&last_row), Expect::Acts),
            (click(&last_row), Expect::Acts),
            (click("crud-delete"), Expect::Acts),
            (click(&last_row), Expect::Refused(Cause::Stale)),
            (click(NOBODY), Expect::Refused(Cause::NotFound)),
            (
                Call {
                    verb: UNKNOWN_VERB.to_string(),
                    args: vec![("id".to_string(), ArgValue::Text(NOBODY.to_string()))],
                },
                Expect::Refused(Cause::UnknownVerb),
            ),
            (
                Call {
                    verb: "click".to_string(),
                    args: Vec::new(),
                },
                Expect::Refused(Cause::Malformed),
            ),
            (advance(300), Expect::Refused(Cause::TimeUnavailable)),
        ] {
            let _ = run.drive("Crud", &mut crud, &call, expect);
        }
        needles.read(&crud);

        let (mut timer, _ticks) = hold(LeanTask::Timer, incremental);
        let moved = run.drive("Timer", &mut timer, &advance(300), Expect::Acts);
        assert!(
            matches!(moved, Ok(Outcome::Advanced { advanced_ms, .. }) if advanced_ms > 0),
            "{}: Timer: the advance moves time",
            run.mode
        );
        needles.read(&timer);

        let mut fixture = Session::start("spans", || {
            Harness::from_vdom(
                VirtualDom::new(covered_fixture),
                stand::options(incremental),
            )
        })
        .expect("the label is one of the closed set");
        let _ = run.drive(
            "fixture",
            &mut fixture,
            &click("fx-under"),
            Expect::Refused(Cause::Covered),
        );
        needles.read(&fixture);

        assert!(run.calls == 27, "{}: {} calls", run.mode, run.calls);
        run.finish(&needles);
    }
}

#[test]
#[ignore = "spawned as a child process by a_driver_command_writes_no_line_at_the_default_level"]
fn child_drives_at_the_default_level() {
    let capture = Capture::install();
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Counter");
        let (mut counter, _ticks) = hold(LeanTask::Counter, incremental);
        let clicked = counter.run(&click("counter-increment"));
        assert!(
            clicked
                .as_ref()
                .ok()
                .and_then(acting)
                .is_some_and(|(settled, _, diff)| settled && diff.changed.len() == 1),
            "{mode}: the click returns settled with the value it rewrote"
        );
        assert!(
            counter
                .run(&click(NOBODY))
                .is_err_and(|refusal| refusal.cause() == Cause::NotFound),
            "{mode}: the call naming no element is refused not-found"
        );
    }

    // The sink is installed and prints an escher record at this level: the zero below is the
    // level's, not a missing sink's or the target allowlist's.
    tracing::warn!(target: "escher_stand_probe", "probe");
    let written = capture.since(0);
    let probes = written
        .lines()
        .filter(|line| line.contains(" WARN escher_stand_probe "))
        .count();
    let driver = written.lines().filter(|line| is_driver_line(line)).count();
    assert!(
        probes == 1 && driver == 0,
        "{probes} probe lines and {driver} driver lines at the default level"
    );
}
