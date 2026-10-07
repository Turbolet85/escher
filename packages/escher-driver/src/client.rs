//! The client side: the lifecycle another process drives.

use std::path::Path;
use std::process::{Child, Command};
use std::time::Duration;

use crate::SessionError;

/// What a session answers to `attach`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hello {
    /// The process id of the session's host.
    pub pid: u32,
    /// The label the session was started with.
    pub label: String,
    /// How many requests the host had answered before this one.
    pub served: u64,
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

    use super::{Hello, Started};
    use crate::SessionError;
    use crate::wire::{self, Line, Reply, Request};

    const POLL: Duration = Duration::from_millis(25);
    const STOP_BOUND: Duration = Duration::from_secs(5);

    pub(super) fn attach(state_dir: &Path) -> Result<Hello, SessionError> {
        match exchange(state_dir, Request::Hello)? {
            Reply::Hello { pid, label, served } => Ok(Hello { pid, label, served }),
            _ => Err(SessionError::Protocol),
        }
    }

    pub(super) fn stop(state_dir: &Path) -> Result<(), SessionError> {
        if exchange(state_dir, Request::Stop)? != Reply::Stopping {
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

    fn exchange(state_dir: &Path, request: Request) -> Result<Reply, SessionError> {
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
            .set_read_timeout(Some(wire::IO_BOUND))
            .map_err(io_error)?;
        stream
            .set_write_timeout(Some(wire::IO_BOUND))
            .map_err(io_error)?;
        stream
            .write_all(request.encode().as_bytes())
            .map_err(io_error)?;
        match wire::read_line(&stream, wire::MAX_REPLY_BYTES).map_err(io_error)? {
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

    use super::{Hello, Started};
    use crate::SessionError;

    pub(super) fn attach(_state_dir: &Path) -> Result<Hello, SessionError> {
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
