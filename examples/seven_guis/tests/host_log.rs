//! The `escher-session` host, its telemetry sink installed, writes nothing a command handled to
//! stderr at `RUST_LOG=trace`: no stable element id, no accessible name and no typed text —
//! while a hosted `snapshot`, `click` and `type` run on it — and each call it runs leaves
//! exactly one command-span line there. Guards a measured leak: the sink printed records from
//! third-party targets as written, which put every author key of the screen on stderr at
//! `debug` and each row's name and each label's text at `trace`. The check reads the host as
//! its own build makes it; the workspace build compiles the engine's tracing call sites in, and
//! the same check is run on that host too.

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
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    use common::{Host, clear, drain, state_dir};
    use escher_driver::{ArgValue, Call, attach, call, stop};
    use seven_guis::stand::{self, LeanTask};

    const READY: Duration = Duration::from_secs(60);
    const EXIT: Duration = Duration::from_secs(10);

    // The needles are read from the screen the host serves, booted here in process. An id is a
    // needle when it is an author key (it holds no `/`) and holds a `-`: that leaves out `main`,
    // an author key that is also an ordinary word, so a hit on it could not be attributed.
    let harness = stand::boot(LeanTask::Crud, stand::options(true));
    let ids: Vec<String> = harness
        .doc
        .element_ids()
        .into_iter()
        .map(|(_, id)| id)
        .filter(|id| !id.contains('/') && id.contains('-'))
        .collect();
    let text_of = |selector: &str| -> Vec<String> {
        let nodes = harness.query_all(selector);
        let doc = harness.base();
        nodes
            .into_iter()
            .map(|node| doc.get_node(node).expect("a queried node exists"))
            .map(|node| node.text_content())
            .collect()
    };
    let rows = text_of("#crud-list > div");
    let names: Vec<String> = rows.iter().cloned().chain(text_of("label")).collect();

    assert!(
        ids.iter().any(|id| id == "crud-surname"),
        "the screen's author keys are among the id needles"
    );
    assert!(ids.len() >= 15, "{} id needles were read", ids.len());
    assert!(rows.len() >= 3, "{} row names were read", rows.len());
    assert!(
        ids.iter().chain(&names).all(|needle| !needle.is_empty()),
        "no needle is empty"
    );
    assert!(
        ids.iter()
            .chain(&names)
            .all(|needle| !needle.contains(SENTINEL) && !SENTINEL.contains(needle.as_str())),
        "the sentinel is no part of a needle"
    );

    let dir = state_dir("hl-trace");
    clear(&dir);
    let mut host = Host(
        Command::new(BINARY)
            .args(["serve", "crud", "--session"])
            .arg(&dir)
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

    // Three calls run on the host's own instance. Each answer is checked for what it must
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

    let lines = stderr.lines().count();
    let unstamped = stderr
        .lines()
        .filter(|line| !line.contains("service.name=seven_guis"))
        .count();
    assert!(lines >= 1, "the sink's install line reaches stderr");
    assert_eq!(
        unstamped, 0,
        "{unstamped} of {lines} stderr lines carry no service name"
    );

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

    let found = |needles: &[String]| {
        needles
            .iter()
            .filter(|needle| stderr.contains(needle.as_str()))
            .count()
    };
    let (ids_found, names_found) = (found(&ids), found(&names));
    let sentinels = stderr.matches(SENTINEL).count();
    assert!(
        ids_found == 0 && names_found == 0 && sentinels == 0,
        "{ids_found} of {} id needles, {names_found} of {} name needles and {sentinels} \
         occurrences of the typed text found in {lines} stderr lines",
        ids.len(),
        names.len()
    );
}
