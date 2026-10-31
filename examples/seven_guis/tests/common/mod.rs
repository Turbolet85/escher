//! What the checks of the `escher-session` binary share: a state directory under the test
//! target's temp dir, the clear of what a killed earlier run left there, a guard that kills
//! and reaps a host a failing check still holds, one bounded run of the binary as a client with
//! both its streams drained, a child held with a piped stdin that is spoken to one line at a
//! time and answers one line at a time, the readers of an answer's one line, and what the two
//! log checks read stderr with: the CRUD screen's id and name needles, a Cells part's id
//! needles and the cell a name is committed to, the service-name stamp,
//! the needle and occurrence counts and a recording's needles. A check declares `mod common;`
//! and reads what it needs. This module holds no test.

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

/// The first string a written JSON object holds under `key`, unescaped over exactly the escapes
/// the driver's writer writes — `\"`, `\\`, `\n`, `\r`, `\t` and `\u00XX` — and `None` for
/// any other. Read off the outer layer only: a JSON line held as a string comes back as that
/// line.
pub fn string_at(json: &str, key: &str) -> Option<String> {
    let (_, rest) = json.split_once(&format!("\"{key}\":\""))?;
    let mut text = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next()? {
            '"' => return Some(text),
            '\\' => match chars.next()? {
                '"' => text.push('"'),
                '\\' => text.push('\\'),
                'n' => text.push('\n'),
                'r' => text.push('\r'),
                't' => text.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    let code = hex
                        .strip_prefix("00")
                        .and_then(|low| u32::from_str_radix(low, 16).ok())?;
                    text.push(char::from_u32(code)?);
                }
                _ => return None,
            },
            other => text.push(other),
        }
    }
}

/// How long a [`Speaker`] waits for one reply, and for its child to end once its stdin closes.
pub const REPLY_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

/// A spawned child spoken to over a piped stdin, one line at a time: its stdout read line by
/// line on a thread of its own into a channel, its stderr drained from the spawn on, and the
/// child killed and reaped when a failing check drops it.
pub struct Speaker {
    child: std::process::Child,
    stdin: Option<std::process::ChildStdin>,
    lines: std::sync::mpsc::Receiver<Vec<u8>>,
    stderr: Option<std::thread::JoinHandle<std::io::Result<Vec<u8>>>>,
}

/// How a [`Speaker`]'s child ended: `status` is `None` when it was still going at the bound and
/// was killed, or was ended by a signal; `stdout` holds the lines it wrote that no
/// [`Speaker::reply`] took.
pub struct Spoke {
    pub status: Option<i32>,
    pub stdout: Vec<Vec<u8>>,
    pub stderr: Vec<u8>,
}

impl Speaker {
    /// Spawns `command` with its three streams piped.
    pub fn spawn(mut command: std::process::Command) -> Speaker {
        use std::io::BufRead;
        use std::process::Stdio;

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the command spawns");
        let stdin = child.stdin.take();
        let stderr = drain(child.stderr.take().expect("stderr is piped"));
        let stdout = child.stdout.take().expect("stdout is piped");
        let (sender, lines) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut stdout = std::io::BufReader::new(stdout);
            loop {
                let mut line = Vec::new();
                match stdout.read_until(b'\n', &mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if sender.send(line).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        Speaker {
            child,
            stdin,
            lines,
            stderr: Some(stderr),
        }
    }

    /// Writes `line` and a line break to the child's stdin, and flushes.
    pub fn send(&mut self, line: &[u8]) {
        use std::io::Write;

        let stdin = self.stdin.as_mut().expect("stdin is open");
        stdin
            .write_all(line)
            .and_then(|()| stdin.write_all(b"\n"))
            .and_then(|()| stdin.flush())
            .expect("the child reads its stdin");
    }

    /// The next line the child writes to stdout, as it wrote it, its line break included,
    /// waited for under [`REPLY_BOUND`]. A failure names a count only: a reply holds ids, names
    /// and values.
    #[track_caller]
    pub fn reply_bytes(&self) -> Vec<u8> {
        match self.lines.recv_timeout(REPLY_BOUND) {
            Ok(line) => line,
            Err(_) => panic!("no reply line within {} s", REPLY_BOUND.as_secs()),
        }
    }

    /// The next line the child writes to stdout, without its line break: one line of text
    /// with no ESC byte.
    #[track_caller]
    pub fn reply(&self) -> String {
        let line = self.reply_bytes();
        let escapes = line.iter().filter(|byte| **byte == 0x1b).count();
        assert!(
            line.last() == Some(&b'\n') && escapes == 0,
            "a reply of {} bytes holds {escapes} ESC bytes",
            line.len()
        );
        String::from_utf8(line[..line.len() - 1].to_vec()).expect("a reply is text")
    }

    /// Closes the child's stdin and waits for it to end, for at most [`REPLY_BOUND`]: a child
    /// still going then is killed and reaped.
    pub fn close(mut self) -> Spoke {
        use std::time::{Duration, Instant};

        drop(self.stdin.take());
        let deadline = Instant::now() + REPLY_BOUND;
        let status = loop {
            if let Some(status) = self.child.try_wait().expect("the child can be waited on") {
                break status.code();
            }
            if Instant::now() >= deadline {
                let _ = self.child.kill();
                let _ = self.child.wait();
                break None;
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        let stderr = self
            .stderr
            .take()
            .expect("stderr is drained once")
            .join()
            .expect("the stderr reader ends")
            .expect("stderr is read to its end");
        let stdout =
            std::iter::from_fn(|| self.lines.recv_timeout(Duration::from_secs(1)).ok()).collect();
        Spoke {
            status,
            stdout,
            stderr,
        }
    }
}

impl Drop for Speaker {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
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

/// What a log check looks for in what the binary wrote to stderr, read from the CRUD screen the
/// binary serves, booted here in process.
pub struct Needles {
    pub ids: Vec<String>,
    pub names: Vec<String>,
}

/// Reads the CRUD screen's needles. An id is a needle when it is an author key (it holds no `/`)
/// and holds a `-`: that leaves out `main`, an author key that is also an ordinary word, so a hit
/// on it could not be attributed. A name is a needle when it is a row's, a label's or the list's
/// `aria-label`. Asserts that enough of each were read and that none is empty or a part of
/// `sentinel`.
#[track_caller]
pub fn stand_needles(sentinel: &str) -> Needles {
    use seven_guis::stand::{self, LeanTask};

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
    let list_name = harness.attr("#crud-list", "aria-label");
    assert!(list_name.is_some(), "the list's name was read");
    let names: Vec<String> = rows
        .iter()
        .cloned()
        .chain(text_of("label"))
        .chain(list_name)
        .collect();
    assert!(ids.len() >= 15, "{} id needles were read", ids.len());
    assert!(rows.len() >= 3, "{} row names were read", rows.len());
    assert!(
        ids.iter().chain(&names).all(|needle| !needle.is_empty()
            && !needle.contains(sentinel)
            && !sentinel.contains(needle.as_str())),
        "no needle is empty or a part of the sentinel"
    );
    Needles { ids, names }
}

/// Reads the needles of the flight booker's `flight-type` options, booted here in process: each
/// option's id, its name and its value. Asserts that the two options were read.
#[track_caller]
pub fn option_needles() -> Vec<String> {
    use seven_guis::stand::{self, LeanTask};

    let harness = stand::boot(LeanTask::FlightBooker, stand::options(true));
    let snapshot = harness.doc.snapshot();
    let select = snapshot
        .get("flight-type")
        .expect("the flight booker reads its select");
    assert_eq!(
        select.children.len(),
        2,
        "the select's two options are read"
    );
    let mut needles = Vec::new();
    for option in &select.children {
        let value = harness.attr(&format!("#{}", option.id), "value");
        needles.extend([option.id.clone(), option.name.clone()]);
        needles.extend(value);
    }
    assert_eq!(needles.len(), 6, "each option gives three needles");
    assert!(
        needles.iter().all(|needle| !needle.is_empty()),
        "no needle is empty"
    );
    needles
}

/// Reads part `part` of the Cells screen the binary serves, booted here in process: its author
/// keys as id needles, by `stand_needles`' rule, and the middle one of its column-A cells, where
/// a check commits the name it then looks for. Cells' own names at boot — empty cells and row
/// numbers — are no needle a hit could be attributed by. Asserts that enough ids were read.
#[track_caller]
pub fn cells_needles(part: usize) -> (Vec<String>, String) {
    use seven_guis::stand;

    let harness = stand::boot_cells(stand::options(true));
    let parts = harness
        .doc
        .snapshot()
        .text_parts(dioxus_native::SNAPSHOT_TEXT_BUDGET);
    assert!(parts.len() > part, "the screen reads past part {part}");
    let ids: Vec<String> = parts[part - 1]
        .ids
        .iter()
        .filter(|id| !id.contains('/') && id.contains('-'))
        .cloned()
        .collect();
    let column_a: Vec<&String> = ids.iter().filter(|id| id.starts_with("cell-A")).collect();
    assert!(ids.len() >= 100, "{} id needles were read", ids.len());
    assert!(
        column_a.len() >= 3,
        "{} column-A cells were read",
        column_a.len()
    );
    let cell = column_a[column_a.len() / 2].clone();
    (ids, cell)
}

/// Asserts that the sink's install line reached stderr and that every line carries the service
/// name, and returns how many lines stderr holds.
#[track_caller]
pub fn stamped_lines(stderr: &str) -> usize {
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
    lines
}

/// How many of `needles` a captured stderr holds.
pub fn found(stderr: &str, needles: &[String]) -> usize {
    needles
        .iter()
        .filter(|needle| stderr.contains(needle.as_str()))
        .count()
}

/// How many times a captured stderr holds any of `needles`, each occurrence counted.
pub fn occurrences(stderr: &str, needles: &[String]) -> usize {
    needles
        .iter()
        .map(|needle| stderr.matches(needle.as_str()).count())
        .sum()
}

/// What a recording session leaves for the log checks to look for: the recording directory's
/// name and each digest its one check holds, as the check writes them.
#[cfg(unix)]
pub fn recorded_needles(kept: &Path) -> Vec<String> {
    let names: Vec<_> = fs::read_dir(kept)
        .expect("the recording directory is listed")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert!(names == ["check-1.json"], "the session kept one check");
    let bytes = fs::read(kept.join("check-1.json")).expect("the check is read");
    let check = escher_driver::Check::from_bytes(&bytes).expect("the check reads back");
    assert!(!check.steps().is_empty(), "the check holds the calls");
    let dir_name = kept
        .file_name()
        .and_then(|name| name.to_str())
        .expect("the directory's name is text")
        .to_string();
    std::iter::once(dir_name)
        .chain(
            check
                .steps()
                .iter()
                .map(|step| format!("{:016x}", step.digest())),
        )
        .collect()
}
