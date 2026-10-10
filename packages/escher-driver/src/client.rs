//! The client side: the lifecycle another process drives, and the calls it sends.

use std::path::Path;
use std::process::{Child, Command};
use std::time::Duration;

use crate::SessionError;
use crate::command::{Call, validate};
use crate::wire::Reply;

/// What a session answers to `attach`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    /// The process id of the session's host.
    pub pid: u32,
    /// The label the session was started with.
    pub label: String,
    /// How many requests the host had answered before this one.
    pub served: u64,
    /// The seconds without a request after which the host stops the session by itself.
    pub idle_expiry_s: u64,
}

/// What a session answers to a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// Whether the call ran: false when it was refused.
    pub accepted: bool,
    /// The outcome of a call that ran, or the refusal of one that did not, as one line of
    /// JSON: the form [`Outcome::to_json`](crate::Outcome::to_json) and
    /// [`Refusal::to_json`](crate::Refusal::to_json) write. It holds what the screen reads —
    /// ids, names and values — and is an answer to the caller: never a log line.
    pub json: String,
}

/// A session [`start`] brought up.
#[derive(Debug)]
pub struct Started {
    /// The host process `start` spawned. The session outlives this handle: dropping it neither
    /// stops nor reaps the host.
    pub child: Child,
    /// The session's first answer.
    pub hello: Hello,
}

/// Reaches the session in `state_dir`.
///
/// [`SessionError::NoSession`] when the directory holds no socket, [`SessionError::Dead`] when
/// it holds one no process answers on, [`SessionError::Protocol`] when the answer is not the
/// session's.
///
/// Unix only: elsewhere this returns [`SessionError::Unsupported`].
pub fn attach(state_dir: &Path) -> Result<Hello, SessionError> {
    imp::attach(state_dir)
}

/// Stops the session in `state_dir` and waits, for at most 5 s, until the state directory is
/// gone ([`SessionError::Timeout`] past that).
///
/// With no session to stop it reads as [`attach`] does: [`SessionError::NoSession`] or
/// [`SessionError::Dead`].
///
/// Unix only: elsewhere this returns [`SessionError::Unsupported`].
pub fn stop(state_dir: &Path) -> Result<(), SessionError> {
    imp::stop(state_dir)
}

/// Sends `call` to the session in `state_dir` and returns its answer.
///
/// The call is checked first ([`validate`]): one the schema refuses is answered here with its
/// refusal and reaches no session, so a refused call changes nothing and every call that is
/// sent fits the socket's request bound. A call the schema admits is run by the session's host
/// through [`Session::run`](crate::Session::run), which may still refuse it — the id names
/// nothing on the screen, or the element cannot take the action.
///
/// The answer is waited for under a 30 s bound on each read ([`SessionError::Timeout`] past
/// it). An answer longer than the socket carries is [`SessionError::AnswerTooLarge`]: the call
/// ran, and what it did is not rolled back. With no session to reach this reads as [`attach`]
/// does.
///
/// Unix only: elsewhere a call the schema admits returns [`SessionError::Unsupported`].
pub fn call(state_dir: &Path, call: &Call) -> Result<Answer, SessionError> {
    if let Err(refusal) = validate(call) {
        return Ok(Answer {
            accepted: false,
            json: refusal.to_json(),
        });
    }
    imp::call(state_dir, call)
}

/// What a call's reply reads as to the caller.
#[cfg_attr(not(unix), allow(dead_code))]
fn read_answer(reply: Reply) -> Result<Answer, SessionError> {
    match reply {
        Reply::Accepted(json) => Ok(Answer {
            accepted: true,
            json,
        }),
        Reply::Refused(json) => Ok(Answer {
            accepted: false,
            json,
        }),
        Reply::Oversize => Err(SessionError::AnswerTooLarge),
        _ => Err(SessionError::Protocol),
    }
}

/// Starts a session in `state_dir` by spawning `host`, and waits until it answers.
///
/// `host` is a command that serves a session in `state_dir`; it is spawned exactly as the
/// caller configured it. When a session already answers there this returns
/// [`SessionError::AlreadyRunning`] and spawns nothing; a dead session's leftover socket does
/// not block a start. The wait ends when the session answers, when the host exits
/// ([`SessionError::HostExited`]) or when `ready` has passed ([`SessionError::Timeout`], the
/// spawned host killed and reaped).
///
/// Unix only: elsewhere this returns [`SessionError::Unsupported`].
pub fn start(state_dir: &Path, host: Command, ready: Duration) -> Result<Started, SessionError> {
    imp::start(state_dir, host, ready)
}

#[cfg(unix)]
mod imp {
    use std::io::{self, Write};
    use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::process::{Child, Command};
    use std::thread;
    use std::time::{Duration, Instant};

    use super::{Answer, Hello, Started, read_answer};
    use crate::SessionError;
    use crate::command::Call;
    use crate::wire::{self, Line, Reply, Request};

    const POLL: Duration = Duration::from_millis(25);
    const STOP_BOUND: Duration = Duration::from_secs(5);

    /// What a reply is read under: the bound on each read, and the longest line.
    type ReplyBounds = (Duration, usize);

    const LIFECYCLE: ReplyBounds = (wire::IO_BOUND, wire::MAX_REPLY_BYTES);
    const ANSWER: ReplyBounds = (wire::ANSWER_BOUND, wire::MAX_CALL_REPLY_BYTES);

    pub(super) fn attach(state_dir: &Path) -> Result<Hello, SessionError> {
        match exchange(state_dir, &Request::Hello, LIFECYCLE)? {
            Reply::Hello {
                pid,
                label,
                served,
                idle_expiry_s,
            } => Ok(Hello {
                pid,
                label,
                served,
                idle_expiry_s,
            }),
            _ => Err(SessionError::Protocol),
        }
    }

    pub(super) fn call(state_dir: &Path, call: &Call) -> Result<Answer, SessionError> {
        read_answer(exchange(state_dir, &Request::Call(call.clone()), ANSWER)?)
    }

    pub(super) fn stop(state_dir: &Path) -> Result<(), SessionError> {
        if exchange(state_dir, &Request::Stop, LIFECYCLE)? != Reply::Stopping {
            return Err(SessionError::Protocol);
        }
        let deadline = Instant::now() + STOP_BOUND;
        while state_dir.exists() {
            if Instant::now() >= deadline {
                return Err(SessionError::Timeout);
            }
            thread::sleep(POLL);
        }
        Ok(())
    }

    pub(super) fn start(
        state_dir: &Path,
        mut host: Command,
        ready: Duration,
    ) -> Result<Started, SessionError> {
        match attach(state_dir) {
            Ok(_) => return Err(SessionError::AlreadyRunning),
            Err(SessionError::NoSession | SessionError::Dead) => {}
            Err(error) => return Err(error),
        }
        let mut child = host.spawn().map_err(io_error)?;
        let deadline = Instant::now() + ready;
        loop {
            match attach(state_dir) {
                Ok(hello) => return Ok(Started { child, hello }),
                // Not up yet: no socket, or the dead session's one the host is about to replace.
                Err(SessionError::NoSession | SessionError::Dead) => {}
                Err(error) => {
                    kill_and_reap(&mut child);
                    return Err(error);
                }
            }
            match child.try_wait() {
                Ok(Some(status)) => return Err(SessionError::HostExited(status.code())),
                Ok(None) => {}
                Err(error) => {
                    kill_and_reap(&mut child);
                    return Err(io_error(error));
                }
            }
            if Instant::now() >= deadline {
                kill_and_reap(&mut child);
                return Err(SessionError::Timeout);
            }
            thread::sleep(POLL);
        }
    }

    fn kill_and_reap(child: &mut Child) {
        let _ = child.kill();
        let _ = child.wait();
    }

    fn io_error(error: io::Error) -> SessionError {
        match error.kind() {
            io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => SessionError::Timeout,
            kind => SessionError::Io(kind),
        }
    }

    fn exchange(
        state_dir: &Path,
        request: &Request,
        (read_bound, longest): ReplyBounds,
    ) -> Result<Reply, SessionError> {
        let mut stream =
            UnixStream::connect(state_dir.join(wire::SOCKET_FILE)).map_err(|error| match error
                .kind()
            {
                io::ErrorKind::NotFound => SessionError::NoSession,
                io::ErrorKind::ConnectionRefused => SessionError::Dead,
                // `std` refuses a path with no room for a socket address as `InvalidInput`.
                io::ErrorKind::InvalidInput => SessionError::StateDirTooLong,
                kind => SessionError::Io(kind),
            })?;
        stream
            .set_read_timeout(Some(read_bound))
            .map_err(io_error)?;
        stream
            .set_write_timeout(Some(wire::IO_BOUND))
            .map_err(io_error)?;
        stream
            .write_all(request.encode().as_bytes())
            .map_err(io_error)?;
        match wire::read_line(&stream, longest).map_err(io_error)? {
            Line::Text(line) => Reply::parse(&line).ok_or(SessionError::Protocol),
            Line::TooLong | Line::Unterminated => Err(SessionError::Protocol),
        }
    }
}

#[cfg(not(unix))]
mod imp {
    use std::path::Path;
    use std::process::Command;
    use std::time::Duration;

    use super::{Answer, Hello, Started};
    use crate::SessionError;
    use crate::command::Call;

    pub(super) fn attach(_state_dir: &Path) -> Result<Hello, SessionError> {
        Err(SessionError::Unsupported)
    }

    pub(super) fn call(_state_dir: &Path, _call: &Call) -> Result<Answer, SessionError> {
        Err(SessionError::Unsupported)
    }

    pub(super) fn stop(_state_dir: &Path) -> Result<(), SessionError> {
        Err(SessionError::Unsupported)
    }

    pub(super) fn start(
        _state_dir: &Path,
        _host: Command,
        _ready: Duration,
    ) -> Result<Started, SessionError> {
        Err(SessionError::Unsupported)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::ArgValue;
    use crate::refusal::{Cause, Fault, Refusal};
    use crate::wire::{self, MAX_ANSWER_BYTES};

    #[test]
    fn a_calls_reply_reads_as_its_answer_or_as_a_session_error() {
        let answer = |accepted: bool, json: &str| {
            Ok(Answer {
                accepted,
                json: json.to_string(),
            })
        };
        let hello = Reply::Hello {
            pid: 1,
            label: "counter".to_string(),
            served: 0,
            idle_expiry_s: 1800,
        };
        let rows = [
            (Reply::Accepted("{}".to_string()), answer(true, "{}")),
            (Reply::Refused("{}".to_string()), answer(false, "{}")),
            (Reply::Oversize, Err(SessionError::AnswerTooLarge)),
            (hello, Err(SessionError::Protocol)),
            (Reply::Stopping, Err(SessionError::Protocol)),
            (Reply::RefusedVersion, Err(SessionError::Protocol)),
            (Reply::RefusedMalformed, Err(SessionError::Protocol)),
        ];
        assert_eq!(rows.len(), 7);
        for (row, (reply, read)) in rows.into_iter().enumerate() {
            assert!(read_answer(reply) == read, "row {row}");
        }
        // The answer bound, end to end on a made value: one byte over it is framed as the
        // oversize reply, which reads as the error that names it.
        let one_over = "x".repeat(MAX_ANSWER_BYTES + 1);
        assert!(read_answer(wire::frame(true, one_over)) == Err(SessionError::AnswerTooLarge));
    }

    #[test]
    fn a_call_the_schema_refuses_is_answered_without_a_session() {
        let no_session = Path::new("a-state-directory-that-does-not-exist");
        let rows = [
            (
                Call {
                    verb: "screenshot".to_string(),
                    args: Vec::new(),
                },
                Refusal::new(Cause::UnknownVerb),
            ),
            (
                Call {
                    verb: "stop".to_string(),
                    args: Vec::new(),
                },
                Refusal::new(Cause::UnknownVerb),
            ),
            (
                Call {
                    verb: "click".to_string(),
                    args: Vec::new(),
                },
                Refusal::malformed(Fault::Missing("id")),
            ),
            (
                Call {
                    verb: "advance".to_string(),
                    args: vec![("ms".to_string(), ArgValue::Number(0))],
                },
                Refusal::malformed(Fault::OutOfBound("ms")),
            ),
        ];
        assert_eq!(rows.len(), 4);
        for (row, (refused, refusal)) in rows.into_iter().enumerate() {
            let read = call(no_session, &refused);
            let expected = Ok(Answer {
                accepted: false,
                json: refusal.to_json(),
            });
            assert!(read == expected, "row {row}");
        }
    }
}
