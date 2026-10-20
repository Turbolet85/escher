//! What the session checks share. For a session held in process: each lean task's label, a
//! session booted through the stand, one command that changes what a task shows, and the
//! driver's calls with what an acting one returned or why a refused one was. For a session
//! held by a process of its own: a state directory under the test target's temp dir, the host
//! command — this test binary re-run on one ignored child — and a guard over the host that
//! bounds every wait and kills and reaps a host a failing check still holds. A check declares
//! `mod session_common;` and reads what it needs. This module holds no test.

#![allow(dead_code)]

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use blitz_dom::Document;
use dioxus_native_dom::SnapshotDiff;
use escher_driver::{ArgValue, Busy, Call, Cause, Check, Finding, Outcome, Session};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::timer::TimerTicks;

/// The stand's slug for a lean task: the label its session is started with.
pub fn slug(task: LeanTask) -> &'static str {
    match task {
        LeanTask::Counter => "counter",
        LeanTask::FlightBooker => "flight-booker",
        LeanTask::Timer => "timer",
        LeanTask::Crud => "crud",
    }
}

/// A session on `task`, booted through the stand, with the handle that delivers the timer's
/// ticks. The timer's session carries the stand's time step over that handle; for any other
/// task nothing reads the handle and the session has no step.
pub fn hold(task: LeanTask, incremental: bool) -> (Session, TimerTicks) {
    let (harness, ticks) = match task {
        LeanTask::Timer => stand::boot_timer(stand::options(incremental)),
        task => (
            stand::boot(task, stand::options(incremental)),
            TimerTicks::default(),
        ),
    };
    let session = Session::start(slug(task), || harness).expect("a lean task's slug is a label");
    let session = match task {
        LeanTask::Timer => session.with_time(stand::timer_step(ticks.clone())),
        _ => session,
    };
    (session, ticks)
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

/// The driver call that reads the screen.
pub fn snapshot() -> Call {
    call("snapshot", &[])
}

/// The driver call that reads part `n` of a screen read in parts.
pub fn snapshot_part(n: i64) -> Call {
    call("snapshot", &[("part", ArgValue::Number(n))])
}

/// What a `snapshot` read in parts answered: the part's text, the part and how many parts the
/// screen reads as. Any other outcome fails the check.
pub fn part(outcome: Outcome) -> (String, u32, u32) {
    match outcome {
        Outcome::Part { text, part, parts } => (text, part, parts),
        _ => panic!("the outcome is a part"),
    }
}

/// The driver call that clicks the element `id` names.
pub fn click(id: &str) -> Call {
    call("click", &[("id", ArgValue::Text(id.to_string()))])
}

/// The driver call that types `text` into the element `id` names.
pub fn type_into(id: &str, text: &str) -> Call {
    call(
        "type",
        &[
            ("id", ArgValue::Text(id.to_string())),
            ("text", ArgValue::Text(text.to_string())),
        ],
    )
}

/// The driver call that presses the key `key` names, with shift held when `shift` is true.
pub fn press(key: &str, shift: bool) -> Call {
    let key = ("key", ArgValue::Text(key.to_string()));
    if shift {
        call("press", &[key, ("shift", ArgValue::Flag(true))])
    } else {
        call("press", &[key])
    }
}

/// The driver call that moves the session's clock forward by `ms` milliseconds, then hands
/// `ms` to the app's time step where the session carries one.
pub fn advance(ms: i64) -> Call {
    call("advance", &[("ms", ArgValue::Number(ms))])
}

/// The driver call that brings the element `id` names into view.
pub fn scroll(id: &str) -> Call {
    call("scroll", &[("id", ArgValue::Text(id.to_string()))])
}

/// The driver call that takes an image of the whole screen.
pub fn screenshot() -> Call {
    call("screenshot", &[])
}

/// The driver call that takes an image of the element `id` names.
pub fn screenshot_of(id: &str) -> Call {
    call("screenshot", &[("id", ArgValue::Text(id.to_string()))])
}

/// The driver call that reads the screen for its visual defects.
pub fn lint() -> Call {
    call("lint", &[])
}

/// Runs `call` on `session` through the driver and reads its outcome as a lint's findings:
/// `None` for a refused call and for any other outcome.
pub fn linted(session: &mut Session, call: &Call) -> Option<Vec<Finding>> {
    match session.run(call).ok()? {
        Outcome::Linted { findings } => Some(findings),
        _ => None,
    }
}

/// What a `screenshot` returned: the image's width, its height and its PNG bytes.
pub struct Shot {
    pub width: u32,
    pub height: u32,
    pub png: Vec<u8>,
}

/// Runs `call` on `session` through the driver and reads its outcome as a shot: `None` for a
/// refused call and for any other outcome.
pub fn shot(session: &mut Session, call: &Call) -> Option<Shot> {
    match session.run(call).ok()? {
        Outcome::Shot { width, height, png } => Some(Shot { width, height, png }),
        _ => None,
    }
}

/// What an acting call returned, whichever of the three acting shapes it has.
pub struct Acted {
    pub settled: bool,
    pub busy: Option<Busy>,
    pub diff: SnapshotDiff,
    /// What an `advance` reports it moved; `None` for every other verb.
    pub advanced_ms: Option<u32>,
    /// Whether a `scroll` reports its element in view after the step; `None` for every other
    /// verb.
    pub in_view: Option<bool>,
}

impl Acted {
    /// The ids the diff names added, in its order.
    pub fn added(&self) -> Vec<&str> {
        self.diff.added.iter().map(|entry| &*entry.id).collect()
    }

    /// The ids the diff names removed, in its order.
    pub fn removed(&self) -> Vec<&str> {
        self.diff.removed.iter().map(String::as_str).collect()
    }

    /// The ids the diff names changed, in its order.
    pub fn changed(&self) -> Vec<&str> {
        self.diff.changed.iter().map(|entry| &*entry.id).collect()
    }
}

/// The ids reading focused in a snapshot of the held instance, and the id the accessibility
/// tree's focus carries: none when that is the `Window`.
pub fn focused(session: &Session) -> (Vec<String>, Vec<String>) {
    let doc = &session.harness().doc;
    let in_snapshot = doc
        .snapshot()
        .nodes()
        .filter(|node| node.state.focused)
        .map(|node| node.id.clone())
        .collect();
    let tree = doc.accessibility_tree();
    let in_tree = tree
        .nodes
        .iter()
        .filter(|(id, _)| *id == tree.focus)
        .filter_map(|(_, node)| node.author_id())
        .map(str::to_string)
        .collect();
    (in_snapshot, in_tree)
}

/// Runs `call` on `session` through the driver and reads its outcome as an acting one: `None`
/// for a refused call, a `snapshot`, a `screenshot` and a `lint`.
pub fn act(session: &mut Session, call: &Call) -> Option<Acted> {
    match session.run(call).ok()? {
        Outcome::Screen { .. }
        | Outcome::Part { .. }
        | Outcome::Shot { .. }
        | Outcome::Linted { .. } => None,
        Outcome::Acted {
            settled,
            busy,
            diff,
        } => Some(Acted {
            settled,
            busy,
            diff,
            advanced_ms: None,
            in_view: None,
        }),
        Outcome::Advanced {
            settled,
            busy,
            diff,
            advanced_ms,
        } => Some(Acted {
            settled,
            busy,
            diff,
            advanced_ms: Some(advanced_ms),
            in_view: None,
        }),
        Outcome::Scrolled {
            settled,
            busy,
            diff,
            in_view,
        } => Some(Acted {
            settled,
            busy,
            diff,
            advanced_ms: None,
            in_view: Some(in_view),
        }),
    }
}

/// Runs `call` on `session` through the driver and reads why it was refused: `None` for a
/// call that ran.
pub fn refused(session: &mut Session, call: &Call) -> Option<Cause> {
    session.run(call).err().map(|refusal| refusal.cause())
}

/// Runs `call` on `session` and reads why it was refused, with whether the call left the
/// instance as it was: the snapshot, the ids reading focused and the accessibility tree's focus
/// equal before and after.
pub fn refused_unchanged(session: &mut Session, call: &Call) -> (Option<Cause>, bool) {
    let before = (session.harness().doc.snapshot(), focused(session));
    let cause = refused(session, call);
    let after = (session.harness().doc.snapshot(), focused(session));
    (cause, before == after)
}

/// The first 64 base64 characters of the image a `screenshot` of the whole screen answers, as
/// the driver writes it: what a kept check would hold if it kept a shot's pixels.
pub fn shot_prefix(session: &mut Session) -> String {
    let line = session
        .run(&screenshot())
        .expect("the screenshot is accepted")
        .to_json();
    let (_, png) = line
        .split_once(r#""png":""#)
        .expect("a shot's line holds its image");
    png.chars().take(64).collect()
}

/// What a kept check reads for each step: whether it was accepted and its digest.
pub fn kept(check: &Check) -> Vec<(bool, u64)> {
    check
        .steps()
        .iter()
        .map(|step| (step.accepted(), step.digest()))
        .collect()
}

/// The permission bits of `path`.
#[cfg(unix)]
pub fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .expect("the path is there")
        .permissions()
        .mode()
        & 0o777
}

/// One command that changes what `task` shows: a click, or for the timer three delivered ticks.
pub fn change(task: LeanTask, session: &mut Session, ticks: &TimerTicks) {
    let harness = session.harness_mut();
    match task {
        LeanTask::Counter => harness.click("#counter-increment"),
        LeanTask::FlightBooker => harness.click("#flight-return"),
        LeanTask::Timer => {
            ticks.deliver(3);
            harness.pump();
        }
        LeanTask::Crud => harness.click("#crud-person-0"),
    }
}

/// How long a check waits for a host to answer.
pub const READY: Duration = Duration::from_secs(60);

/// How long a check waits for a stopped or killed host to exit.
pub const EXIT: Duration = Duration::from_secs(10);

/// The socket's file name inside a state directory.
pub const SOCKET_FILE: &str = "session.sock";

/// A check's own state directory. `name` is short: a socket address has about a hundred bytes
/// for the whole path. A parent and the child it re-runs derive the same directory from it.
pub fn state_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(name)
}

/// Removes what a killed earlier run may have left: the socket file and the empty directory.
pub fn clear(state_dir: &Path) {
    let _ = fs::remove_file(state_dir.join(SOCKET_FILE));
    let _ = fs::remove_dir(state_dir);
}

/// This test binary re-run on the one ignored test `child`, which hosts a session.
pub fn host_command(child: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().expect("the test binary has a path"));
    command
        .args(["--ignored", "--exact", child, "--nocapture"])
        .stdin(Stdio::null());
    command
}

/// A host process a check started. Dropping it kills and reaps the host if it still runs.
pub struct Host(Child);

impl Host {
    pub fn new(child: Child) -> Self {
        Self(child)
    }

    pub fn pid(&self) -> u32 {
        self.0.id()
    }

    /// The host's exit code, waited for at most [`EXIT`]: `None` when it is still running then,
    /// or was ended by a signal.
    pub fn exit_code(&mut self) -> Option<i32> {
        self.exit().and_then(|status| status.code())
    }

    /// Kills the host and reaps it.
    pub fn kill(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }

    /// What an exited host wrote to its piped stdout and stderr.
    pub fn output(&mut self) -> (String, String) {
        let (mut stdout, mut stderr) = (String::new(), String::new());
        if let Some(mut pipe) = self.0.stdout.take() {
            pipe.read_to_string(&mut stdout)
                .expect("the host's stdout is text");
        }
        if let Some(mut pipe) = self.0.stderr.take() {
            pipe.read_to_string(&mut stderr)
                .expect("the host's stderr is text");
        }
        (stdout, stderr)
    }

    fn exit(&mut self) -> Option<ExitStatus> {
        let deadline = Instant::now() + EXIT;
        loop {
            if let Some(status) = self.0.try_wait().expect("the host can be waited on") {
                return Some(status);
            }
            if Instant::now() >= deadline {
                return None;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        self.kill();
    }
}
