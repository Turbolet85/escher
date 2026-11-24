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
use dioxus_native_dom::{SnapshotDiff, UiGraph};
use escher_driver::{
    ActionNode, ArgValue, Busy, Call, Cause, Check, Finding, LayoutExplanation, Outcome, Session,
    SessionError, serve_mcp,
};
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

/// The driver call that presses `key` with the host's action key held: Meta on macOS, Ctrl
/// elsewhere.
pub fn press_ctrl(key: &str) -> Call {
    call(
        "press",
        &[
            ("key", ArgValue::Text(key.to_string())),
            ("ctrl", ArgValue::Flag(true)),
        ],
    )
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

/// The driver call that reads the screen's UI graph.
pub fn graph() -> Call {
    call("graph", &[])
}

/// The driver call that reads part `n` of the screen's UI graph.
pub fn graph_part(n: i64) -> Call {
    call("graph", &[("part", ArgValue::Number(n))])
}

/// What a `graph` read in parts answered: the part's graph, the part and how many parts the
/// graph reads as. Any other outcome fails the check.
pub fn graph_of_part(outcome: Outcome) -> (UiGraph, u32, u32) {
    match outcome {
        Outcome::GraphPart { graph, part, parts } => (graph, part, parts),
        other => panic!("a graph read in parts answers a part, not {other:?}"),
    }
}

/// The driver call that reads what each actionable element of the screen can take now.
pub fn actions() -> Call {
    call("actions", &[])
}

/// The driver call that reads why the element `id` names sits where it does at its size.
pub fn layout(id: &str) -> Call {
    call("layout", &[("id", ArgValue::Text(id.to_string()))])
}

/// The driver call that reads the summary of the draw commands the canvas `id` names paints.
pub fn canvas(id: &str) -> Call {
    call("canvas", &[("id", ArgValue::Text(id.to_string()))])
}

/// The driver call that reads the draw commands the canvas `id` names paints, whole, or part 1
/// of a dump over the budget.
pub fn canvas_full(id: &str) -> Call {
    call(
        "canvas",
        &[
            ("id", ArgValue::Text(id.to_string())),
            ("full", ArgValue::Flag(true)),
        ],
    )
}

/// The driver call that reads part `part` of the canvas `id` names's full dump.
pub fn canvas_part(id: &str, part: i64) -> Call {
    call(
        "canvas",
        &[
            ("id", ArgValue::Text(id.to_string())),
            ("full", ArgValue::Flag(true)),
            ("part", ArgValue::Number(part)),
        ],
    )
}

/// Runs `call` on `session` through the driver and reads its outcome as a canvas answer — a
/// summary, a whole dump or a part: `None` for a refused call and for any other outcome.
pub fn canvased(session: &mut Session, call: &Call) -> Option<Outcome> {
    match session.run(call).ok()? {
        outcome @ (Outcome::CanvasSummary { .. }
        | Outcome::CanvasDump { .. }
        | Outcome::CanvasPart { .. }) => Some(outcome),
        _ => None,
    }
}

/// Runs `call` on `session` through the driver and reads its outcome as a layout answer: `None`
/// for a refused call and for any other outcome.
pub fn explained(session: &mut Session, call: &Call) -> Option<LayoutExplanation> {
    match session.run(call).ok()? {
        Outcome::Layout { explanation } => Some(explanation),
        _ => None,
    }
}

/// Runs `call` on `session` through the driver and reads its outcome as an actions answer:
/// `None` for a refused call and for any other outcome.
pub fn actioned(session: &mut Session, call: &Call) -> Option<Vec<ActionNode>> {
    match session.run(call).ok()? {
        Outcome::Actions { nodes } => Some(nodes),
        _ => None,
    }
}

/// Runs `call` on `session` through the driver and reads its outcome as a UI graph: `None` for
/// a refused call and for any other outcome. A graph answered in parts is read whole: every
/// part read in turn, its nodes in order and part 1's states.
pub fn graphed(session: &mut Session, call: &Call) -> Option<UiGraph> {
    match session.run(call).ok()? {
        Outcome::Graphed { graph } => Some(graph),
        Outcome::GraphPart { parts, .. } => {
            let mut whole = UiGraph::default();
            for n in 1..=parts {
                let Outcome::GraphPart { graph, .. } =
                    session.run(&graph_part(i64::from(n))).ok()?
                else {
                    return None;
                };
                whole.nodes.extend(graph.nodes);
                if n == 1 {
                    whole.states = graph.states;
                }
            }
            Some(whole)
        }
        _ => None,
    }
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
/// for a refused call, a `snapshot`, a `screenshot`, a `lint`, a `graph`, an `actions`, a
/// `layout` and a `canvas`.
pub fn act(session: &mut Session, call: &Call) -> Option<Acted> {
    match session.run(call).ok()? {
        Outcome::Screen { .. }
        | Outcome::Part { .. }
        | Outcome::Shot { .. }
        | Outcome::Linted { .. }
        | Outcome::Graphed { .. }
        | Outcome::GraphPart { .. }
        | Outcome::Actions { .. }
        | Outcome::Layout { .. }
        | Outcome::CanvasSummary { .. }
        | Outcome::CanvasDump { .. }
        | Outcome::CanvasPart { .. } => None,
        Outcome::Acted {
            settled,
            busy,
            diff,
            ..
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
            ..
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
            ..
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

/// One command that changes what `task` shows: a click, the flight booker's return flight chosen
/// by `type`, or for the timer three delivered ticks.
pub fn change(task: LeanTask, session: &mut Session, ticks: &TimerTicks) {
    match task {
        LeanTask::Counter => session.harness_mut().click("#counter-increment"),
        LeanTask::FlightBooker => {
            act(session, &type_into("flight-type", "return"))
                .expect("the flight type takes the return flight");
        }
        LeanTask::Timer => {
            ticks.deliver(3);
            session.harness_mut().pump();
        }
        LeanTask::Crud => session.harness_mut().click("#crud-person-0"),
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

/// The first text block of an MCP tool result, its JSON string decoded whole: every escape read
/// back, so a `\\` in an answer line (a location as Windows spells it) compares as the line
/// `Outcome::to_json` wrote.
pub fn mcp_result_text(reply: &str) -> String {
    let key = r#""text":""#;
    let Some(start) = reply.find(key) else {
        return String::new();
    };
    let mut out = String::new();
    let mut characters = reply[start + key.len()..].chars();
    while let Some(character) = characters.next() {
        match character {
            '"' => break,
            '\\' => match characters.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('u') => {
                    let hex: String = characters.by_ref().take(4).collect();
                    if let Some(decoded) =
                        u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32)
                    {
                        out.push(decoded);
                    }
                }
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    out
}

/// An MCP `tools/call` request line: request `id`, the tool `name` and its JSON `arguments`.
pub fn tool_call(id: usize, name: &str, arguments: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{"name":"{name}","arguments":{arguments}}}}}"#
    )
}

/// The replies `serve_mcp` writes for `requests`, one per line, serving `apps` booted by `boot`.
pub fn serve_lines(
    requests: &[String],
    apps: &[&str],
    boot: impl FnMut(&str) -> Result<Session, SessionError>,
) -> Vec<String> {
    let mut input = requests.join("\n");
    input.push('\n');
    let mut output = Vec::new();
    serve_mcp(
        input.as_bytes(),
        &mut output,
        apps,
        boot,
        Duration::from_secs(1800),
    )
    .expect("the exchange ends");
    String::from_utf8(output)
        .expect("replies are text")
        .lines()
        .map(str::to_string)
        .collect()
}

/// The text block of a tool result's reply, unescaped over exactly the escapes the driver's
/// writer writes, and whether the result is marked as an error.
#[track_caller]
pub fn mcp_tool_result(reply: &str) -> (bool, String) {
    let (_, rest) = reply
        .split_once(r#""content":[{"type":"text","text":""#)
        .expect("the reply is a tool result");
    let mut text = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next().expect("the text block ends") {
            '"' => break,
            '\\' => match chars.next().expect("an escape is whole") {
                '"' => text.push('"'),
                '\\' => text.push('\\'),
                'n' => text.push('\n'),
                'r' => text.push('\r'),
                't' => text.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    let code = u32::from_str_radix(&hex, 16).expect("four hex digits");
                    text.push(char::from_u32(code).expect("a character"));
                }
                _ => panic!("an escape the writer does not write"),
            },
            other => text.push(other),
        }
    }
    (reply.ends_with(r#""isError":true}}"#), text)
}

/// The lean task a served screen's label names, if it is one.
pub fn lean(label: &str) -> Option<LeanTask> {
    LeanTask::ALL.into_iter().find(|task| slug(*task) == label)
}

/// A session on the served screen `label` names, booted through the stand.
pub fn hold_served(label: &str, incremental: bool) -> Session {
    if let Some(task) = lean(label) {
        return hold(task, incremental).0;
    }
    let harness = match label {
        "cells" => stand::boot_cells(stand::options(incremental)),
        "load" => stand::boot_load(stand::options(incremental)),
        "order" => stand::boot_order(stand::options(incremental)),
        _ => panic!("a served screen's label"),
    };
    Session::start(label, || harness).expect("the label is one of the closed set")
}

/// One held session in one layout mode, and the diffs its steps returned.
pub struct Held {
    pub mode: String,
    pub session: Session,
    pub diffs: Vec<SnapshotDiff>,
}

impl Held {
    /// A session on `task` in the layout mode `incremental` names, with no step taken.
    pub fn task(task: LeanTask, incremental: bool) -> Self {
        Held {
            mode: format!("incremental={incremental}: {task:?}"),
            session: hold(task, incremental).0,
            diffs: Vec::new(),
        }
    }

    /// Runs `call` through the driver between the check's own two snapshots and asserts it
    /// returns settled with the diff of those snapshots.
    #[track_caller]
    pub fn settled_step(&mut self, call: &Call) -> Acted {
        let mode = &self.mode;
        let step = self.diffs.len();
        let before = self.session.harness().doc.snapshot();
        let Some(acted) = act(&mut self.session, call) else {
            panic!("{mode}: step {step} runs");
        };
        let after = self.session.harness().doc.snapshot();
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: step {step} returns settled"
        );
        assert!(
            acted.diff == before.diff(&after),
            "{mode}: step {step} returns the diff of the snapshots before and after it"
        );
        self.diffs.push(acted.diff.clone());
        acted
    }
}

/// Asserts the two layout modes returned the same diffs, step by step.
#[track_caller]
pub fn assert_modes_agree(runs: &[Vec<SnapshotDiff>]) {
    assert!(runs.len() == 2, "one run per layout mode");
    assert!(!runs[0].is_empty(), "a run takes a step");
    assert!(
        runs[0] == runs[1],
        "the steps' diffs differ between incremental false and true"
    );
}

/// The left, top, right and bottom of the bounds the snapshot reads for `id`.
pub fn edges(session: &Session, id: &str) -> (f64, f64, f64, f64) {
    let snapshot = session.harness().doc.snapshot();
    let bounds = snapshot
        .get(id)
        .expect("the fixture's element is a snapshot node")
        .bounds;
    (
        bounds.x,
        bounds.y,
        bounds.x + bounds.width,
        bounds.y + bounds.height,
    )
}

/// The centre of the bounds the snapshot reads for `id`.
pub fn centre(session: &Session, id: &str) -> (f64, f64) {
    let (left, top, right, bottom) = edges(session, id);
    ((left + right) / 2.0, (top + bottom) / 2.0)
}

/// Whether the bounds the snapshot reads for `id` hold any area.
pub fn has_area(session: &Session, id: &str) -> bool {
    let (left, top, right, bottom) = edges(session, id);
    right > left && bottom > top
}

/// Whether `point` lies inside the rect of the given edges, its edges included.
pub fn inside(point: (f64, f64), (left, top, right, bottom): (f64, f64, f64, f64)) -> bool {
    point.0 >= left && point.0 <= right && point.1 >= top && point.1 <= bottom
}

/// The viewport's logical size as the edges of a rect.
pub fn viewport() -> (f64, f64, f64, f64) {
    (
        0.0,
        0.0,
        f64::from(stand::VIEWPORT_WIDTH),
        f64::from(stand::VIEWPORT_HEIGHT),
    )
}

/// Records `calls` into `dir`, cleared first, on the session `boot` gives: each call asserted
/// accepted and the recording asserted standing, then the kept check's bytes read and `dir`
/// removed. A failure message names `at` only.
#[cfg(unix)]
#[track_caller]
pub fn record_check(
    dir: &Path,
    boot: impl FnOnce() -> Session,
    calls: &[Call],
    at: &str,
) -> Vec<u8> {
    let _ = fs::remove_dir_all(dir);
    {
        let mut session = boot()
            .recording_into(dir)
            .unwrap_or_else(|_| panic!("{at}: the recording starts"));
        for call in calls {
            assert!(session.run(call).is_ok(), "{at}: each step is accepted");
        }
        assert!(!session.recording_failed(), "{at}: the recording stands");
    }
    let bytes =
        fs::read(dir.join("check-1.json")).unwrap_or_else(|_| panic!("{at}: the check is kept"));
    let _ = fs::remove_dir_all(dir);
    bytes
}
