//! The `escher-session` host, its telemetry sink installed, writes nothing a command handled to
//! stderr at `RUST_LOG=trace`: no stable element id, no accessible name, no typed text and no
//! image byte — while a hosted `snapshot`, `click`, `type`, two `screenshot`s, the whole
//! screen and one element, a `lint` and a `snapshot` of part 1 run on it — and each call it runs leaves
//! exactly one command-span line there. Guards a measured leak: the sink printed records from
//! third-party targets as written, which put every author key of the screen on stderr at
//! `debug` and each row's name and each label's text at `trace`. The check reads the host as
//! its own build makes it; the workspace build compiles the engine's tracing call sites in, and
//! the same check is run on that host too. The flow runs once more on a host keeping its session
//! as a check (`--record`): its stderr then holds neither the recording directory's name nor
//! any digest the check keeps, and its command-span lines are as many as without. The clients
//! of a session `start --record` launched, each run at `RUST_LOG=trace` — `start`, a `type`, a
//! `snapshot`, a `snapshot` of part 1, a `lint` and `stop` — write to their own stderr no id, no
//! name, no typed text, no recording directory's path and nothing the kept check holds; the
//! host `start` spawns has its streams closed, so this reads the clients alone.

#![cfg(not(target_arch = "wasm32"))]

mod common;

#[cfg(unix)]
const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// The text a hosted `type` types: synthetic, and no part of any id or name.
#[cfg(unix)]
const SENTINEL: &str = "Typed-Sentinel-71kWzq";

#[cfg(unix)]
#[test]
fn the_host_logs_no_id_no_name_and_no_typed_text_at_trace() {
    host_flow("hl-trace", false);
}

#[cfg(unix)]
#[test]
fn a_recording_host_logs_nothing_of_its_check_at_trace() {
    host_flow("hl-rec", true);
}

#[cfg(unix)]
#[test]
fn a_recording_sessions_clients_log_nothing_of_it_at_trace() {
    use std::process::Command;

    use common::{
        Needles, Ran, Started, clear, found, number, occurrences, recorded_needles, run,
        stand_needles, state_dir, text,
    };

    let Needles { ids, names } = stand_needles(SENTINEL);
    let dir = state_dir("hl-client");
    clear(&dir);
    let kept = state_dir("hl-client-rec");
    let _ = std::fs::remove_dir_all(&kept);

    // One client run of the binary at `RUST_LOG=trace`, its arguments as given.
    let client = |args: &[&str]| -> Ran {
        let mut command = Command::new(BINARY);
        command.args(args).env("RUST_LOG", "trace");
        run(command)
    };
    let mut stderr = Vec::new();
    let mut ended = |step: usize, ran: Ran| -> String {
        assert_eq!(ran.status, Some(0), "client {step} ends accepted");
        let line = ran.line().to_string();
        stderr.extend_from_slice(&ran.stderr);
        line
    };

    let started = ended(
        0,
        client(&[
            "start",
            "crud",
            "--record",
            text(&kept),
            "--session",
            text(&dir),
        ]),
    );
    let pid = number(&started, "pid").expect("start answers its host's process id");
    let _guard = Started {
        dir: dir.clone(),
        pid: Some(pid),
    };
    let calls: [&[&str]; 5] = [
        &["type", "--id", "crud-name", "--text", SENTINEL],
        &["snapshot"],
        &["snapshot", "--part", "1"],
        &["lint"],
        &["stop"],
    ];
    let answers: Vec<String> = calls
        .iter()
        .enumerate()
        .map(|(index, args)| {
            let mut line = args.to_vec();
            line.extend(["--session", text(&dir)]);
            ended(index + 1, client(&line))
        })
        .collect();
    assert!(
        answers[0].contains(SENTINEL) && ids.iter().all(|id| answers[1].contains(id.as_str())),
        "the answers hold the typed text and the screen's ids"
    );
    assert!(
        answers[3].starts_with(r#"{"findings":["#),
        "the lint's answer holds its findings"
    );
    assert!(!dir.exists(), "stop leaves no state directory");

    // A failure names a kind and a count only: what the session handled is ids and names.
    let stderr = String::from_utf8_lossy(&stderr);
    let lines = stderr.lines().count();
    let (ids_found, names_found) = (found(&stderr, &ids), found(&stderr, &names));
    let sentinels = stderr.matches(SENTINEL).count();
    let paths = occurrences(&stderr, &[text(&kept).to_string()]);
    let recorded = recorded_needles(&kept);
    let recorded_found = occurrences(&stderr, &recorded);
    assert!(
        ids_found == 0 && names_found == 0 && sentinels == 0 && paths == 0 && recorded_found == 0,
        "{ids_found} of {} id needles, {names_found} of {} name needles, {sentinels} \
         occurrences of the typed text, {paths} of the recording directory's path and \
         {recorded_found} of {} recording needles found in {lines} client stderr lines",
        ids.len(),
        names.len(),
        recorded.len()
    );
    let _ = std::fs::remove_dir_all(&kept);
}

/// The host's flow at `RUST_LOG=trace` in the state directory `name`, keeping its session as a
/// check beside it when `record` is set.
#[cfg(unix)]
fn host_flow(name: &str, record: bool) {
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    use common::{
        Host, Needles, clear, drain, found, occurrences, recorded_needles, stamped_lines,
        stand_needles, state_dir,
    };
    use escher_driver::{ArgValue, Call, attach, call, stop};

    const READY: Duration = Duration::from_secs(60);
    const EXIT: Duration = Duration::from_secs(10);

    // The needles are read from the screen the host serves.
    let Needles { ids, names } = stand_needles(SENTINEL);
    assert!(
        ids.iter().any(|id| id == "crud-surname"),
        "the screen's author keys are among the id needles"
    );

    let dir = state_dir(name);
    clear(&dir);
    let kept = state_dir(&format!("{name}-kept"));
    let _ = std::fs::remove_dir_all(&kept);
    let mut serve = Command::new(BINARY);
    serve.args(["serve", "crud", "--session"]).arg(&dir);
    if record {
        serve.arg("--record").arg(&kept);
    }
    let mut host = Host(
        serve
            .env("RUST_LOG", "trace")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the binary spawns"),
    );
    let stdout = drain(host.0.stdout.take().expect("stdout is piped"));
    let stderr = drain(host.0.stderr.take().expect("stderr is piped"));

    let deadline = Instant::now() + READY;
    let hello = loop {
        if let Ok(hello) = attach(&dir) {
            break hello;
        }
        let exited = host.0.try_wait().expect("the host can be waited on");
        assert!(exited.is_none(), "the host exited before it answered");
        assert!(
            Instant::now() < deadline,
            "the host answers within the bound"
        );
        thread::sleep(Duration::from_millis(10));
    };
    assert_eq!(hello.pid, host.0.id(), "the binary itself answers");
    assert_eq!(hello.label, "crud");

    // Seven calls run on the host's own instance. Each answer is checked for what it must
    // hold, so the absences read below are of things the host did handle.
    let text = |value: &str| ArgValue::Text(value.to_string());
    let calls = [
        Call {
            verb: "snapshot".to_string(),
            args: Vec::new(),
        },
        Call {
            verb: "click".to_string(),
            args: vec![("id".to_string(), text("crud-create"))],
        },
        Call {
            verb: "type".to_string(),
            args: vec![
                ("id".to_string(), text("crud-name")),
                ("text".to_string(), text(SENTINEL)),
            ],
        },
        Call {
            verb: "screenshot".to_string(),
            args: Vec::new(),
        },
        Call {
            verb: "screenshot".to_string(),
            args: vec![("id".to_string(), text("crud-list"))],
        },
        Call {
            verb: "lint".to_string(),
            args: Vec::new(),
        },
        Call {
            verb: "snapshot".to_string(),
            args: vec![("part".to_string(), ArgValue::Number(1))],
        },
    ];
    let answers: Vec<String> = calls
        .iter()
        .enumerate()
        .map(|(sent, call_sent)| {
            let answer = call(&dir, call_sent).expect("the host answers the call");
            assert!(answer.accepted, "call {sent} ran");
            answer.json
        })
        .collect();
    let ids_answered = ids
        .iter()
        .filter(|id| answers[0].contains(id.as_str()))
        .count();
    assert!(
        ids_answered == ids.len(),
        "the snapshot's answer holds {ids_answered} of {} id needles",
        ids.len()
    );
    assert!(
        answers[2].contains(SENTINEL),
        "the answer to the type holds the typed text as the control's value"
    );
    // The first 64 characters of each shot's base64: the image as it crosses.
    let images: Vec<String> = answers[3..5]
        .iter()
        .map(|answer| {
            let (_, png) = answer
                .split_once("\"png\":\"")
                .expect("a shot's answer holds its png");
            png.chars().take(64).collect()
        })
        .collect();
    assert!(
        images.iter().all(|image| image.len() == 64),
        "each shot's answer holds an image"
    );
    assert!(
        answers[5].starts_with(r#"{"findings":["#),
        "the lint's answer holds its findings"
    );
    assert!(
        answers[6].ends_with(r#","part":1,"parts":1}"#)
            && ids.iter().all(|id| answers[6].contains(id.as_str())),
        "the part's answer holds the screen as its one part"
    );

    stop(&dir).expect("the session stops");

    let deadline = Instant::now() + EXIT;
    let status = loop {
        if let Some(status) = host.0.try_wait().expect("the host can be waited on") {
            break status;
        }
        assert!(Instant::now() < deadline, "the host exits after stop");
        thread::sleep(Duration::from_millis(10));
    };
    let stdout = stdout
        .join()
        .expect("the stdout reader ends")
        .expect("stdout is read to its end");
    let stderr = stderr
        .join()
        .expect("the stderr reader ends")
        .expect("stderr is read to its end");
    let stderr = String::from_utf8_lossy(&stderr);

    // A failure names a kind and a count only: what the host wrote is ids and names, and a
    // failing run's output is kept in logs that outlive it.
    assert_eq!(status.code(), Some(0), "the host exits 0 after stop");
    assert!(
        stdout.is_empty(),
        "the host wrote {} bytes to stdout",
        stdout.len()
    );
    assert!(!dir.exists(), "stop leaves no state directory");

    let lines = stamped_lines(&stderr);

    // One line per call the host ran: the command span, closed, under the driver's target.
    let command_spans = stderr
        .lines()
        .filter(|line| line.contains(" escher_driver ") && line.contains(" span=\"command\""))
        .count();
    assert_eq!(
        command_spans,
        calls.len(),
        "{command_spans} command-span lines for {} calls sent",
        calls.len()
    );

    let (ids_found, names_found) = (found(&stderr, &ids), found(&stderr, &names));
    let sentinels = stderr.matches(SENTINEL).count();
    let image_bytes = occurrences(&stderr, &images);
    assert!(
        ids_found == 0 && names_found == 0 && sentinels == 0 && image_bytes == 0,
        "{ids_found} of {} id needles, {names_found} of {} name needles, {sentinels} \
         occurrences of the typed text and {image_bytes} of an image found in {lines} stderr \
         lines",
        ids.len(),
        names.len()
    );

    assert!(
        kept.exists() == record,
        "a check is kept exactly when recording"
    );
    if record {
        let recorded = recorded_needles(&kept);
        let recorded_found = found(&stderr, &recorded);
        assert!(
            recorded_found == 0,
            "{recorded_found} of {} recording needles found in {lines} stderr lines",
            recorded.len()
        );
        let _ = std::fs::remove_dir_all(&kept);
    }
}
