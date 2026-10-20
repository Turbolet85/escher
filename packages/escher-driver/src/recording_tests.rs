//! Drives a session recording and its kept check: the answer's digest, the records kept and read
//! back, a masked `type`'s stand-in, the owner-only check and directory, the bound and the failed
//! write that end a recording, and the strict reader's grammar and bounds. A child module of
//! `recording`, so it reads the module's private items and its tests keep their
//! `recording::tests` paths.

use std::fs;
use std::os::unix::fs::PermissionsExt;

use dioxus_native_dom::SnapshotDiff;

use super::*;
use crate::lint::{Bounds, Defect, Finding, Reason};

/// A fresh path under the workspace's `target/tmp`, named for the test: nothing is there.
fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp")).join(format!(
        "escher-driver-recording-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.parent().expect("a parent")).expect("target/tmp is created");
    dir
}

fn text(value: &str) -> ArgValue {
    ArgValue::Text(value.to_string())
}

fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {
    Call {
        verb: verb.to_string(),
        args: args
            .iter()
            .map(|(name, value)| (name.to_string(), value.clone()))
            .collect(),
    }
}

fn acted() -> Result<Outcome, Refusal> {
    Ok(Outcome::Acted {
        settled: true,
        busy: None,
        diff: SnapshotDiff {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
        },
    })
}

fn mode(path: &Path) -> u32 {
    fs::metadata(path)
        .expect("the path is there")
        .permissions()
        .mode()
        & 0o777
}

#[test]
fn the_digest_is_fnv_1a_64_of_the_written_answer() {
    let rows: [(&[u8], u64); 3] = [
        (b"", 0xcbf2_9ce4_8422_2325),
        (b"a", 0xaf63_dc4c_8601_ec8c),
        (b"foobar", 0x8594_4171_f739_67e8),
    ];
    assert_eq!(rows.len(), 3);
    for (row, (bytes, digest)) in rows.into_iter().enumerate() {
        assert!(fnv1a(bytes) == digest, "row {row}");
    }
    assert_eq!(hex(fnv1a(b"")), "cbf29ce484222325");
    assert_eq!(hex(0), "0000000000000000");
    let refused = Err(Refusal::new(Cause::NotFound));
    assert_eq!(
        answer_digest(&refused),
        fnv1a(Refusal::new(Cause::NotFound).to_json().as_bytes())
    );
    let shown = Ok(Outcome::Screen {
        text: "Button \"Go\" id=\"go\"\n".to_string(),
    });
    let written = Outcome::Screen {
        text: "Button \"Go\" id=\"go\"\n".to_string(),
    }
    .to_json();
    assert_eq!(answer_digest(&shown), fnv1a(written.as_bytes()));
    assert!(answer_digest(&shown) != answer_digest(&refused));
}

#[test]
fn a_recording_reads_back_as_the_calls_it_kept() {
    let dir = scratch("round-trip");
    let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
    let not_found = Err(Refusal::new(Cause::NotFound));
    let calls = [
        (call("snapshot", &[]), false, acted()),
        (call("click", &[("id", text("increment"))]), false, acted()),
        (
            call(
                "type",
                &[("text", text("Ada \"L\"\n")), ("id", text("who"))],
            ),
            false,
            acted(),
        ),
        (
            call(
                "press",
                &[("shift", ArgValue::Flag(true)), ("key", text("tab"))],
            ),
            false,
            acted(),
        ),
        (
            call("advance", &[("ms", ArgValue::Number(250))]),
            false,
            acted(),
        ),
        (
            call("advance", &[("ms", ArgValue::Number(-1))]),
            false,
            acted(),
        ),
        (call("scroll", &[("id", text("row-14"))]), false, acted()),
        (
            call("screenshot", &[]),
            false,
            Ok(Outcome::Shot {
                width: 2,
                height: 1,
                png: b"foobar".to_vec(),
            }),
        ),
        (
            call("lint", &[]),
            false,
            Ok(Outcome::Linted {
                findings: vec![Finding {
                    id: "go".to_string(),
                    defect: Defect::ZeroSize,
                    bounds: Bounds {
                        x: 8.0,
                        y: 8.0,
                        width: 0.0,
                        height: 0.0,
                    },
                    reason: Reason::ZeroSize,
                }],
            }),
        ),
        (
            call("click", &[("id", ArgValue::Number(7))]),
            false,
            not_found,
        ),
    ];
    assert_eq!(calls.len(), 10);
    for (row, (call, masked, answer)) in calls.iter().enumerate() {
        assert!(recording.record(call, *masked, answer), "row {row}");
    }
    let file = dir.join("check-1.json");
    let bytes = fs::read(&file).expect("the check is written");
    let check = Check::from_bytes(&bytes).expect("the check reads back");
    assert_eq!(check.app(), "counter");
    assert_eq!(check.steps().len(), calls.len());
    for (row, (step, (call, _, answer))) in check.steps().iter().zip(&calls).enumerate() {
        // A call's arguments are kept in the schema's order.
        let verb = schema::verb(&call.verb).expect("a verb of the table");
        let mut ordered = call.args.clone();
        ordered.sort_by_key(|(name, _)| verb.args.iter().position(|arg| arg.name == name));
        assert!(step.call().verb == call.verb, "row {row}");
        assert!(step.call().args == ordered, "row {row}");
        assert!(step.accepted() == answer.is_ok(), "row {row}");
        assert!(step.digest() == answer_digest(answer), "row {row}");
        assert!(!step.masked(), "row {row}");
        let line: Value = serde_json::from_str(&kept_line(answer)).expect("a line of JSON");
        assert!(step.answer() == &line, "row {row}");
    }
    let shot = check.steps()[7].answer();
    assert_eq!(check.steps()[7].call().verb, "screenshot");
    assert!(shot.get("png").is_none());
    assert_eq!(shot.as_object().map(Map::len), Some(2));
    assert!(shot["width"] == 2 && shot["height"] == 1);
    assert!(check.steps()[7].digest() == answer_digest(&calls[7].2));
    // A lint's findings are kept whole.
    assert_eq!(check.steps()[8].call().verb, "lint");
    assert!(check.steps()[8].answer()["findings"][0]["defect"] == "zero-size");
    let text = String::from_utf8(bytes).expect("the check is text");
    assert!(text.starts_with("[\n{\"tool\":\"start\",\"arguments\":{\"app\":\"counter\"}},\n"));
    assert!(text.contains(
        r#"{"tool":"press","arguments":{"key":"tab","shift":true},"accepted":true,"digest":""#
    ));
    assert!(text.ends_with("}\n]\n"));
    assert!(!text.contains(r#""png""#) && !text.contains("Zm9vYmFy"));
    assert_eq!(text.lines().count(), calls.len() + 3);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_masked_type_keeps_the_stand_in_and_nothing_of_the_text() {
    let dir = scratch("masked");
    let secret = "synthetic-pw-9Kx";
    let mut recording = Recording::create(&dir, "login").expect("the recording starts");
    let typed = call("type", &[("id", text("password")), ("text", text(secret))]);
    assert!(recording.record(&typed, true, &acted()));
    // Only a `type` is masked.
    assert!(recording.record(&call("click", &[("id", text("go"))]), true, &acted()));
    let bytes = fs::read(dir.join("check-1.json")).expect("the check is written");
    let written = String::from_utf8(bytes.clone()).expect("the check is text");
    assert!(!written.contains(secret));
    assert!(written.contains(&format!(
        r#"{{"tool":"type","arguments":{{"id":"password","text":"{MASKED_STAND_IN}"}},"masked":true,"accepted":true,"digest":""#
    )));
    // A secret of another length leaves the same bytes: no length is kept.
    let other = scratch("masked-other");
    let mut recording = Recording::create(&other, "login").expect("the recording starts");
    let longer = call(
        "type",
        &[("id", text("password")), ("text", text(&secret.repeat(3)))],
    );
    assert!(recording.record(&longer, true, &acted()));
    assert!(recording.record(&call("click", &[("id", text("go"))]), true, &acted()));
    assert_eq!(
        fs::read(other.join("check-1.json")).ok(),
        Some(bytes.clone())
    );
    let _ = fs::remove_dir_all(&other);
    let check = Check::from_bytes(&bytes).expect("the check reads back");
    let [masked, clicked] = check.steps() else {
        panic!("two steps are kept");
    };
    assert!(masked.masked() && !clicked.masked());
    assert_eq!(
        masked.call().args,
        vec![
            ("id".to_string(), text("password")),
            ("text".to_string(), text(MASKED_STAND_IN))
        ]
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_call_no_record_can_spell_is_not_kept() {
    let dir = scratch("unkept");
    let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
    let rows = [
        call("capture", &[]),
        call("start", &[("app", text("counter"))]),
        call("stop", &[]),
        call("help", &[]),
        call("click", &[("target", text("save"))]),
        call("click", &[("id", text("a")), ("id", text("b"))]),
    ];
    assert_eq!(rows.len(), 6);
    let refused = Err(Refusal::new(Cause::UnknownVerb));
    for (row, unkept) in rows.iter().enumerate() {
        assert!(recording.record(unkept, false, &refused), "row {row}");
    }
    let bytes = fs::read(dir.join("check-1.json")).expect("the check is written");
    let check = Check::from_bytes(&bytes).expect("the check reads back");
    assert!(check.steps().is_empty());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_check_and_its_directory_are_owner_only_and_no_file_is_replaced() {
    let dir = scratch("modes");
    fs::create_dir(&dir).expect("the directory is created");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("made private");
    fs::write(dir.join("check-1.json"), "kept").expect("a file of another session");
    fs::write(dir.join("check-3.json"), "kept").expect("a file of another session");
    let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
    let file = dir.join("check-2.json");
    assert_eq!(mode(&file), 0o600);
    assert!(recording.record(&call("snapshot", &[]), false, &acted()));
    // The rewrite went through its temporary file, which is gone, and kept the mode.
    assert_eq!(mode(&file), 0o600);
    assert!(!dir.join(".check-2.json.tmp").exists());
    assert_eq!(
        fs::read_to_string(dir.join("check-1.json")).ok().as_deref(),
        Some("kept")
    );
    assert_eq!(
        fs::read_to_string(dir.join("check-3.json")).ok().as_deref(),
        Some("kept")
    );
    let next = Recording::create(&dir, "counter").expect("a second recording starts");
    assert!(dir.join("check-4.json").exists());
    drop((recording, next));

    let created = scratch("created");
    let _recording = Recording::create(&created, "counter").expect("the recording starts");
    assert_eq!(mode(&created), 0o700);
    assert_eq!(mode(&created.join("check-1.json")), 0o600);

    let open = scratch("open");
    fs::create_dir(&open).expect("the directory is created");
    for open_mode in [0o750, 0o705, 0o777] {
        fs::set_permissions(&open, fs::Permissions::from_mode(open_mode)).expect("opened");
        assert!(matches!(
            Recording::create(&open, "counter"),
            Err(SessionError::RecordDirNotPrivate)
        ));
        assert_eq!(fs::read_dir(&open).expect("listed").count(), 0);
    }
    for dir in [dir, created, open] {
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn a_recording_that_would_pass_its_bound_or_cannot_write_ends_and_removes_its_check() {
    let dir = scratch("bound");
    let mut recording = Recording::create(&dir, "counter").expect("the recording starts");
    let snapshot = call("snapshot", &[]);
    for row in 1..MAX_CHECK_STEPS {
        assert!(recording.record(&snapshot, false, &acted()), "row {row}");
    }
    let file = dir.join("check-1.json");
    let bytes = fs::read(&file).expect("the check is written");
    assert_eq!(
        Check::from_bytes(&bytes).map(|check| check.steps().len()),
        Ok(MAX_CHECK_STEPS - 1)
    );
    assert!(!recording.record(&snapshot, false, &acted()));
    assert!(!file.exists());
    assert_eq!(fs::read_dir(&dir).expect("listed").count(), 0);

    let blocked = scratch("blocked");
    let mut recording = Recording::create(&blocked, "counter").expect("the recording starts");
    fs::write(blocked.join(".check-1.json.tmp"), "kept").expect("the temporary path is taken");
    assert!(!recording.record(&snapshot, false, &acted()));
    // The check is gone, and the file the recording did not create is left as it was.
    assert!(!blocked.join("check-1.json").exists());
    assert_eq!(
        fs::read_to_string(blocked.join(".check-1.json.tmp"))
            .ok()
            .as_deref(),
        Some("kept")
    );
    assert_eq!(fs::read_dir(&blocked).expect("listed").count(), 1);

    let wide = scratch("wide");
    let mut recording = Recording::create(&wide, "counter").expect("the recording starts");
    let widest = call(
        "type",
        &[
            ("id", text(&"i".repeat(1024))),
            ("text", text(&"\n".repeat(4096))),
        ],
    );
    let mut kept = 0;
    while recording.record(&widest, false, &acted()) {
        kept += 1;
    }
    assert!(kept > 0 && kept < MAX_CHECK_STEPS);
    assert_eq!(fs::read_dir(&wide).expect("listed").count(), 0);
    for dir in [dir, blocked, wide] {
        let _ = fs::remove_dir_all(&dir);
    }
}

const OPENING: &str = r#"{"tool":"start","arguments":{"app":"counter"}}"#;
const CLICK: &str = r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":{"settled":true,"added":[],"removed":[],"changed":[]}}"#;
/// [`CLICK`] without its answer.
const UNANSWERED: &str =
    r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef"}"#;

fn check(records: &[&str]) -> Vec<u8> {
    format!("[{}]", records.join(",")).into_bytes()
}

#[test]
fn a_check_in_the_grammar_is_read() {
    let rows = [
        check(&[OPENING]),
        check(&[OPENING, CLICK]),
        check(&[
            OPENING,
            r#"{"digest":"ffffffffffffffff","accepted":false,"arguments":{"text":"masked","id":"p"},"masked":true,"tool":"type","answer":{}}"#,
        ]),
        check(&[
            OPENING,
            r#"{"tool":"advance","arguments":{"ms":0},"accepted":false,"digest":"0000000000000000","answer":{}}"#,
        ]),
        check(&[
            OPENING,
            r#"{"tool":"screenshot","arguments":{},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ]),
        format!(" [ {OPENING} ,\n {CLICK} ] \n").into_bytes(),
        check(&[
            OPENING,
            r#"{"answer":{"text":"Button \"Go\" id=\"go\"\n"},"digest":"0123456789abcdef","accepted":true,"arguments":{},"tool":"snapshot"}"#,
        ]),
        check(&[
            OPENING,
            r#"{"tool":"click","arguments":{"id":"x"},"accepted":false,"digest":"0123456789abcdef","answer":{"refused":{"cause":"not-found"}}}"#,
        ]),
    ];
    assert_eq!(rows.len(), 8);
    for (row, bytes) in rows.iter().enumerate() {
        assert!(Check::from_bytes(bytes).is_ok(), "row {row}");
    }
    let read = Check::from_bytes(&rows[2]).expect("read");
    let [step] = read.steps() else {
        panic!("one step");
    };
    assert!(step.masked() && !step.accepted() && step.digest() == u64::MAX);
}

#[test]
fn a_check_outside_the_grammar_is_refused_as_malformed() {
    let step = |record: &str| check(&[OPENING, record]);
    let rows = [
        b"".to_vec(),
        b"nope".to_vec(),
        b"{}".to_vec(),
        b"[]".to_vec(),
        format!("[{OPENING}").into_bytes(),
        check(&[CLICK]),
        check(&[r#"{"tool":"start","arguments":{"app":7}}"#]),
        check(&[r#"{"tool":"start","arguments":{"app":"Counter"}}"#]),
        check(&[r#"{"tool":"start","arguments":{"app":"counter","id":"x"}}"#]),
        check(&[r#"{"tool":"start","arguments":{"app":"counter"},"accepted":true}"#]),
        check(&[r#"{"tool":"start"}"#]),
        check(&[r#""start""#]),
        step(
            r#"{"tool":"stop","arguments":{},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"capture","arguments":{},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":7,"arguments":{},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(r#"{"arguments":{},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#),
        step(UNANSWERED),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":"x"}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":7}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":[]}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":null}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef","answer":{},"answers":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"target":"save"},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":null},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":["save"]},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"advance","arguments":{"ms":1.5},"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":[],"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":"true","digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"answer":{}}"#),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcde","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdef0","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789ABCDEF","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":"0123456789abcdeg","answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"accepted":true,"digest":7,"answer":{}}"#,
        ),
        step(
            r#"{"tool":"click","arguments":{"id":"save"},"masked":true,"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"type","arguments":{"id":"p","text":"masked"},"masked":false,"accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(
            r#"{"tool":"type","arguments":{"id":"p","text":"masked"},"masked":"true","accepted":true,"digest":"0123456789abcdef","answer":{}}"#,
        ),
        step(r#""click""#),
        check(&[OPENING, CLICK, "null"]),
    ];
    assert_eq!(rows.len(), 40);
    for (row, bytes) in rows.iter().enumerate() {
        assert!(
            Check::from_bytes(bytes) == Err(Refusal::new(Cause::Malformed)),
            "row {row}"
        );
    }
}

#[test]
fn a_check_at_its_bounds_is_read_and_one_past_them_is_refused() {
    let mut records = vec![OPENING; 1];
    records.extend(std::iter::repeat_n(CLICK, MAX_CHECK_STEPS - 1));
    assert_eq!(records.len(), 400);
    let at_steps = check(&records);
    assert!(at_steps.len() <= MAX_CHECK_BYTES);
    assert_eq!(
        Check::from_bytes(&at_steps).map(|read| read.steps().len()),
        Ok(399)
    );
    records.push(CLICK);
    assert_eq!(
        Check::from_bytes(&check(&records)),
        Err(Refusal::new(Cause::Malformed))
    );

    let mut at_bytes = check(&[OPENING, CLICK]);
    at_bytes.resize(MAX_CHECK_BYTES, b' ');
    assert_eq!(at_bytes.len(), 1_048_576);
    assert!(Check::from_bytes(&at_bytes).is_ok());
    at_bytes.push(b' ');
    assert_eq!(
        Check::from_bytes(&at_bytes),
        Err(Refusal::new(Cause::Malformed))
    );
    assert_eq!((MAX_CHECK_STEPS, MAX_CHECK_BYTES), (400, 1_048_576));
}
