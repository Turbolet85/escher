//! The `escher-session` binary keeps a session as a replayable check when its launcher names a
//! recording directory, through the command line and through its `mcp` role alike. *The command
//! line:* `start counter --session <dir> --record <dir>`, then a `click`, a `snapshot` and `stop`,
//! leaves exactly one `check-1.json`, `0600` in a `0700` directory, and the state directory gone
//! after `stop` as without a recording. *The `mcp` role:* `mcp --record <dir>` driven with
//! `start` counter, the same `click` and `snapshot`, then `stop`, leaves a check byte-identical to
//! the command line's. *Refusals:* a recording directory open to its group refuses `start` with
//! `record-dir-not-private`, exit 3 and no host spawned, and refuses `mcp --record` with exit 3.
//! *Unchanged surfaces:* the `mcp` role's `tools/list` and `help` answer the same bytes with and
//! without a recording directory, and every stdout line is one uncoloured line. Every host is
//! held behind a guard that stops or kills it, and every wait is bounded. A failure message
//! names a step and a count — never a captured line. These checks are unix only: the hosted
//! session crosses a Unix-domain socket, and a check's modes are unix modes. *Off unix,*
//! recording is unsupported: `mcp --record` ends with exit 3 and the `unsupported` error's
//! message on stderr, `start … --record` answers that error, and neither creates the directory.

#![cfg(not(target_arch = "wasm32"))]
// The helpers serve the unix checks; off unix only the refusal check reads them.
#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

mod common;

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

#[cfg(unix)]
use common::Started;
use common::{Speaker, clear, client, number, state_dir, string_at, text};
use escher_driver::Check;
#[cfg(not(unix))]
use escher_driver::SessionError;

const BINARY: &str = env!("CARGO_BIN_EXE_escher-session");

/// The permission bits of `path`.
#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    fs::metadata(path)
        .expect("the path is there")
        .permissions()
        .mode()
        & 0o777
}

/// A fresh recording directory under the test target's temp dir: nothing is there.
fn fresh(name: &str) -> std::path::PathBuf {
    let dir = state_dir(name);
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// The one check a recording directory holds, read after its modes are checked.
#[track_caller]
fn the_check(dir: &Path, surface: &str) -> Vec<u8> {
    let names: Vec<_> = fs::read_dir(dir)
        .expect("the recording directory is listed")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert!(
        names == ["check-1.json"],
        "{surface}: the directory holds {} entries, not the one check",
        names.len()
    );
    assert!(mode(dir) == 0o700, "{surface}: the directory is owner-only");
    let file = dir.join("check-1.json");
    assert!(mode(&file) == 0o600, "{surface}: the check is owner-only");
    fs::read(file).expect("the check is read")
}

fn request(id: u32, method: &str, params: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{params}}}"#)
}

fn tool_call(id: u32, name: &str, arguments: &str) -> String {
    request(
        id,
        "tools/call",
        &format!(r#"{{"name":"{name}","arguments":{arguments}}}"#),
    )
}

fn mcp(record: Option<&Path>) -> Speaker {
    let mut command = Command::new(BINARY);
    command.arg("mcp").env_remove("RUST_LOG");
    if let Some(dir) = record {
        command.arg("--record").arg(dir);
    }
    Speaker::spawn(command)
}

/// Sends `line` and returns the reply, which must not be marked as an error.
#[track_caller]
fn answered(speaker: &mut Speaker, line: &str, step: &str) -> String {
    speaker.send(line.as_bytes());
    let reply = speaker.reply();
    assert!(
        !reply.contains(r#""isError":true"#),
        "mcp: {step} is answered as an error"
    );
    reply
}

#[cfg(unix)]
#[test]
fn the_command_line_and_the_mcp_role_keep_the_same_check() {
    let dir = state_dir("rc-cli");
    clear(&dir);
    let kept = fresh("rc-cli-kept");
    let started = client(
        BINARY,
        &[
            "start",
            "counter",
            "--session",
            text(&dir),
            "--record",
            text(&kept),
        ],
    );
    assert!(started.status == Some(0), "cli: start is accepted");
    let _session = Started {
        dir: dir.clone(),
        pid: number(started.line(), "pid"),
    };
    for (step, args) in [
        ("click", &["click", "--id", "counter-increment"][..]),
        ("snapshot", &["snapshot"][..]),
    ] {
        let mut words = args.to_vec();
        words.extend(["--session", text(&dir)]);
        let ran = client(BINARY, &words);
        assert!(ran.status == Some(0), "cli: {step} is accepted");
        ran.line();
    }
    let stopped = client(BINARY, &["stop", "--session", text(&dir)]);
    assert!(stopped.status == Some(0), "cli: stop is accepted");
    stopped.line();
    assert!(!dir.exists(), "cli: the state directory is gone after stop");
    let by_command_line = the_check(&kept, "cli");
    let check = Check::from_bytes(&by_command_line).expect("the check reads back");
    assert!(check.app() == "counter", "cli: the check names its app");
    assert!(check.steps().len() == 2, "cli: one step per call");

    let kept_by_mcp = fresh("rc-mcp-kept");
    let mut speaker = mcp(Some(&kept_by_mcp));
    for (id, (step, name, arguments)) in [
        ("start", "start", r#"{"app":"counter"}"#),
        ("click", "click", r#"{"id":"counter-increment"}"#),
        ("snapshot", "snapshot", "{}"),
        ("stop", "stop", "{}"),
    ]
    .into_iter()
    .enumerate()
    {
        answered(
            &mut speaker,
            &tool_call(id as u32 + 1, name, arguments),
            step,
        );
    }
    let spoke = speaker.close();
    assert!(spoke.status == Some(0), "mcp: the role ends 0");
    assert!(spoke.stdout.is_empty(), "mcp: no line is left unread");
    let by_mcp = the_check(&kept_by_mcp, "mcp");
    assert!(
        by_mcp == by_command_line,
        "the two surfaces keep checks of {} and {} bytes, not the same bytes",
        by_mcp.len(),
        by_command_line.len()
    );
    for dir in [kept, kept_by_mcp] {
        let _ = fs::remove_dir_all(&dir);
    }
}

#[cfg(unix)]
#[test]
fn a_recording_directory_open_to_its_group_is_refused_before_anything_runs() {
    let open = fresh("rc-open");
    fs::create_dir(&open).expect("the directory is created");
    fs::set_permissions(&open, fs::Permissions::from_mode(0o750)).expect("opened to the group");

    let dir = state_dir("rc-open-s");
    clear(&dir);
    let refused = client(
        BINARY,
        &[
            "start",
            "counter",
            "--session",
            text(&dir),
            "--record",
            text(&open),
        ],
    );
    assert!(refused.status == Some(3), "start: a session error");
    let line = refused.line();
    assert!(
        string_at(line, "kind").as_deref() == Some("record-dir-not-private"),
        "start: the error names the open recording directory"
    );
    assert!(!dir.exists(), "start: no host was spawned");

    let spoken = client(BINARY, &["mcp", "--record", text(&open)]);
    assert!(spoken.status == Some(3), "mcp: a session error");
    assert!(spoken.stdout.is_empty(), "mcp: stdout is the protocol's");
    assert!(
        !spoken.stderr.is_empty(),
        "mcp: the error's message is on stderr"
    );
    assert!(
        fs::read_dir(&open).expect("listed").count() == 0,
        "nothing is written to the open directory"
    );
    let _ = fs::remove_dir_all(&open);
}

#[cfg(unix)]
#[test]
fn the_mcp_roles_listing_and_help_are_the_same_with_a_recording_directory() {
    let kept = fresh("rc-same");
    let lines = [request(1, "tools/list", "{}"), tool_call(2, "help", "{}")];
    let mut replies = Vec::new();
    for record in [None, Some(kept.as_path())] {
        let mut speaker = mcp(record);
        let read: Vec<String> = lines
            .iter()
            .map(|line| answered(&mut speaker, line, "a listing or help"))
            .collect();
        let spoke = speaker.close();
        assert!(spoke.status == Some(0), "mcp: the role ends 0");
        replies.push(read);
    }
    assert!(
        replies[0] == replies[1],
        "mcp: the replies differ with a recording"
    );
    assert!(
        fs::read_dir(&kept).expect("listed").count() == 0,
        "no session was started, so no check is kept"
    );
    let _ = fs::remove_dir_all(&kept);
}

#[cfg(not(unix))]
#[test]
fn off_unix_a_recording_directory_is_refused_as_unsupported() {
    let kept = fresh("rc-off-unix");
    let unsupported = SessionError::Unsupported;

    let spoken = client(BINARY, &["mcp", "--record", text(&kept)]);
    assert!(spoken.status == Some(3), "mcp: a session error");
    assert!(spoken.stdout.is_empty(), "mcp: stdout is the protocol's");
    assert!(
        spoken.stderr == format!("{unsupported}\n").as_bytes(),
        "mcp: stderr is the unsupported error's fixed message, {} bytes",
        spoken.stderr.len()
    );
    assert!(!kept.exists(), "mcp: no recording directory is created");

    let dir = state_dir("rc-off-unix-s");
    let started = client(
        BINARY,
        &[
            "start",
            "counter",
            "--session",
            text(&dir),
            "--record",
            text(&kept),
        ],
    );
    assert!(started.status == Some(3), "start: a session error");
    assert!(
        string_at(started.line(), "kind").as_deref() == Some(unsupported.kind()),
        "start: the error is the unsupported one"
    );
    assert!(!kept.exists(), "start: no recording directory is created");
    assert!(!dir.exists(), "start: no state directory is created");
}
