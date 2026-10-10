//! What the checks of the `escher-session` binary share: a state directory under the test
//! target's temp dir, the clear of what a killed earlier run left there, a guard that kills
//! and reaps a host a failing check still holds, one bounded run of the binary as a client with
//! both its streams drained, and the readers of an answer's one line. A check declares
//! `mod common;` and reads what it needs. This module holds no test.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// A state directory of the check's own, its name short enough for a socket address.
pub fn state_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(name)
}

/// Removes what a killed earlier run may have left: the socket file and the empty directory.
pub fn clear(state_dir: &Path) {
    let _ = fs::remove_file(state_dir.join("session.sock"));
    let _ = fs::remove_dir(state_dir);
}

/// A state directory's path as the text a command line takes.
pub fn text(state_dir: &Path) -> &str {
    state_dir
        .to_str()
        .expect("the test target's temp dir is text")
}

/// Kills and reaps the host when a failing check still holds it.
#[cfg(unix)]
pub struct Host(pub std::process::Child);

#[cfg(unix)]
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Reads a process's stream to its end on a thread of its own, from the spawn on: a process
/// that writes more than a pipe holds would otherwise block before it ends.
pub fn drain(
    mut pipe: impl std::io::Read + Send + 'static,
) -> std::thread::JoinHandle<std::io::Result<Vec<u8>>> {
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        pipe.read_to_end(&mut bytes).map(|_| bytes)
    })
}

/// What one run of a process wrote and how it ended: `status` is `None` when the run was
/// still going at its bound and was killed, or was ended by a signal.
pub struct Ran {
    pub status: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Ran {
    /// The one line an answering command writes to stdout, without its line break. A failure
    /// names a count only: a command's output is ids, names and values.
    #[track_caller]
    pub fn line(&self) -> &str {
        let breaks = self.stdout.iter().filter(|byte| **byte == b'\n').count();
        let escapes = self.stdout.iter().filter(|byte| **byte == 0x1b).count();
        assert!(
            breaks == 1 && self.stdout.last() == Some(&b'\n') && escapes == 0,
            "stdout holds {breaks} line breaks and {escapes} ESC bytes in {} bytes",
            self.stdout.len()
        );
        std::str::from_utf8(&self.stdout[..self.stdout.len() - 1]).expect("an answer is text")
    }
}

/// How long one run of a command may take: `start` itself waits up to 30 s for its host.
pub const RUN_BOUND: std::time::Duration = std::time::Duration::from_secs(90);

/// Runs `command` to its end with its stdin closed and both streams drained, for at most
/// [`RUN_BOUND`]: a run still going then is killed and reaped.
pub fn run(mut command: std::process::Command) -> Ran {
    use std::process::Stdio;
    use std::time::{Duration, Instant};

    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the command spawns");
    let stdout = drain(child.stdout.take().expect("stdout is piped"));
    let stderr = drain(child.stderr.take().expect("stderr is piped"));
    let deadline = Instant::now() + RUN_BOUND;
    let status = loop {
        if let Some(status) = child.try_wait().expect("the command can be waited on") {
            break status.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    Ran {
        status,
        stdout: stdout
            .join()
            .expect("the stdout reader ends")
            .expect("stdout is read to its end"),
        stderr: stderr
            .join()
            .expect("the stderr reader ends")
            .expect("stderr is read to its end"),
    }
}

/// Runs the binary at `binary` as a client: `args` as given, `RUST_LOG` unset.
pub fn client(binary: &str, args: &[&str]) -> Ran {
    let mut command = std::process::Command::new(binary);
    command.args(args).env_remove("RUST_LOG");
    run(command)
}

/// The keys of a written JSON object's own members, in order.
pub fn keys(json: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    let mut current = String::new();
    let mut last_string = None;
    for character in json.chars() {
        if in_string {
            match (escaped, character) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => {
                    in_string = false;
                    last_string = Some(std::mem::take(&mut current));
                    continue;
                }
                _ => {}
            }
            current.push(character);
            continue;
        }
        match character {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth = depth.saturating_sub(1),
            ':' if depth == 1 => keys.extend(last_string.take()),
            _ => {}
        }
    }
    keys
}

/// The whole number a written JSON object holds under `key`, read off its first occurrence.
pub fn number(json: &str, key: &str) -> Option<u64> {
    let (_, rest) = json.split_once(&format!("\"{key}\":"))?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// The cause a refusal's line names.
pub fn cause(json: &str) -> Option<&str> {
    let rest = json.strip_prefix("{\"refused\":{\"cause\":\"")?;
    rest.split_once('"').map(|(cause, _)| cause)
}

/// A session a check started through the binary's own `start`. The host is the binary's
/// child, not the check's: dropping this stops the session when a failing check left it up,
/// and kills the host by its process id when it does not stop.
#[cfg(unix)]
pub struct Started {
    pub dir: PathBuf,
    pub pid: Option<u64>,
}

#[cfg(unix)]
impl Drop for Started {
    fn drop(&mut self) {
        if !self.dir.exists() {
            return;
        }
        if escher_driver::stop(&self.dir).is_ok() {
            return;
        }
        if let Some(pid) = self.pid {
            let _ = std::process::Command::new("kill")
                .args(["-9", &pid.to_string()])
                .status();
        }
        clear(&self.dir);
    }
}
