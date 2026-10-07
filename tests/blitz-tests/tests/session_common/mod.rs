//! What the session checks share. For a session held in process: each lean task's label, a
//! session booted through the stand, and one command that changes what a task shows. For a
//! session held by a process of its own: a state directory under the test target's temp dir,
//! the host command — this test binary re-run on one ignored child — and a guard over the host
//! that bounds every wait and kills and reaps a host a failing check still holds. A check
//! declares `mod session_common;` and reads what it needs. This module holds no test.

#![allow(dead_code)]

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use escher_driver::Session;
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
/// ticks. For any other task nothing reads the handle.
pub fn hold(task: LeanTask, incremental: bool) -> (Session, TimerTicks) {
    let (harness, ticks) = match task {
        LeanTask::Timer => stand::boot_timer(stand::options(incremental)),
        task => (
            stand::boot(task, stand::options(incremental)),
            TimerTicks::default(),
        ),
    };
    let session = Session::start(slug(task), || harness).expect("a lean task's slug is a label");
    (session, ticks)
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
