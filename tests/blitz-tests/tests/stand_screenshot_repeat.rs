//! The same stand state, screenshotted in separate runs on the same host, gives the same pixels.
//! This test binary re-runs its own ignored child twice. Each child boots the four lean tasks in
//! both layout modes and takes, through the driver, each task's whole screen, CRUD's list, the
//! counter's value display, and the flight booker with its start date focused by a click, so its
//! caret is painted. No scroll precedes a shot here; a shot after one repeats too, since an
//! overlay scrollbar thumb fades by the session's clock alone (`stand_virtual_clock`).
//! The child prints one line per shot — its layout mode, its task, its index and the PNG's bytes
//! in hex — and nothing else of the screen; the two children's lines are byte-identical, and
//! each equals the same shot taken in the parent's process. A failure message carries a layout
//! mode, a task and an index, or a count.

use std::fmt::Write as _;
use std::process::Command;

use escher_driver::Session;
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, click, hold, screenshot, screenshot_of, shot};

/// The word a child's shot line opens with.
const SHOT_LINE: &str = "shot";

/// The shots one layout mode takes: per task, the calls run before them and the elements shot.
fn plan(task: LeanTask) -> (Option<&'static str>, &'static [Option<&'static str>]) {
    match task {
        LeanTask::Counter => (None, &[None, Some("counter-value")]),
        LeanTask::FlightBooker => (Some("flight-start"), &[None]),
        LeanTask::Timer => (None, &[None]),
        LeanTask::Crud => (None, &[None, Some("crud-list")]),
    }
}

fn shoot(session: &mut Session, element: Option<&str>) -> Vec<u8> {
    let call = match element {
        Some(id) => screenshot_of(id),
        None => screenshot(),
    };
    shot(session, &call)
        .expect("the screenshot is accepted")
        .png
}

/// Every shot of both layout modes, as `(key, PNG bytes)`: the key names the mode, the task and
/// the shot's index.
fn shots() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    for incremental in [false, true] {
        for task in LeanTask::ALL {
            let (focus, elements) = plan(task);
            let (mut session, _ticks) = hold(task, incremental);
            if let Some(id) = focus {
                assert!(
                    act(&mut session, &click(id)).is_some(),
                    "incremental={incremental}: {task:?}: the click runs"
                );
            }
            for (index, element) in elements.iter().enumerate() {
                let key = format!("{incremental} {task:?} {index}");
                out.push((key, shoot(&mut session, *element)));
            }
        }
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(text, "{byte:02x}");
    }
    text
}

/// The shot lines a run of the child printed, its process id, and whether it passed.
fn child_run() -> (Vec<String>, Option<u32>) {
    let output = Command::new(std::env::current_exe().expect("the test binary has a path"))
        .args([
            "--ignored",
            "--exact",
            "child_prints_its_shots",
            "--nocapture",
        ])
        .output()
        .expect("the test binary runs");
    assert!(output.status.success(), "the child passes");
    let stdout = String::from_utf8(output.stdout).expect("the child prints text");
    let mut pid = None;
    let mut lines = Vec::new();
    for line in stdout.lines() {
        if let Some(read) = line.strip_prefix("pid ") {
            pid = read.parse().ok();
        } else if line.starts_with(SHOT_LINE) {
            lines.push(line.to_string());
        }
    }
    (lines, pid)
}

#[test]
fn a_shot_of_one_state_is_the_same_in_two_runs_and_in_this_one() {
    let parent: Vec<String> = shots()
        .into_iter()
        .map(|(key, png)| format!("{SHOT_LINE} {key} {}", hex(&png)))
        .collect();
    assert!(
        parent.len() == 12,
        "{} shots over the two modes",
        parent.len()
    );

    let (first, first_pid) = child_run();
    let (second, second_pid) = child_run();
    assert!(
        first_pid.is_some() && second_pid.is_some() && first_pid != second_pid,
        "two distinct child processes"
    );
    assert!(
        first_pid != Some(std::process::id()),
        "the child is not the parent"
    );
    assert!(
        first.len() == parent.len() && second.len() == parent.len(),
        "{} and {} shot lines for {}",
        first.len(),
        second.len(),
        parent.len()
    );
    for (index, ((one, two), here)) in first.iter().zip(&second).zip(&parent).enumerate() {
        let key: String = here.split(' ').take(4).collect::<Vec<_>>().join(" ");
        assert!(one == two, "{key} (line {index}): the two runs differ");
        assert!(
            one == here,
            "{key} (line {index}): a run differs from this process"
        );
    }
}

#[test]
#[ignore = "spawned as a child process by a_shot_of_one_state_is_the_same_in_two_runs_and_in_this_one"]
fn child_prints_its_shots() {
    println!("pid {}", std::process::id());
    for (key, png) in shots() {
        println!("{SHOT_LINE} {key} {}", hex(&png));
    }
}
