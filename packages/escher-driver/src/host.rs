//! The host side: one process serving one [`Session`] until it is stopped or has been idle for
//! its expiry.

use std::path::Path;
use std::time::Duration;

use crate::{Session, SessionError};

/// How long a host a command line starts serves without a request before it stops by itself:
/// 30 minutes.
pub const IDLE_EXPIRY: Duration = Duration::from_secs(1800);

/// Hosts `session` in `state_dir` until a `stop` request arrives, or until no request has been
/// answered for `idle_expiry`.
///
/// The state directory is created owner-only (mode `0700`) when it is absent — its parent must
/// exist — and refused with [`SessionError::StateDirNotPrivate`] when it is present and open to
/// its group or to others. The session answers on the Unix-domain socket `session.sock` inside
/// it (a `UnixListener`, mode `0600`), one connection at a time, on the calling thread: the
/// thread that owns the instance. A socket file no process answers on is replaced; one a
/// process answers on is [`SessionError::AlreadyRunning`].
///
/// A connection carries one request. `hello` and `stop` are the lifecycle's. A call is run on
/// the session through [`Session::run`] and nothing else, and answered with its outcome or its
/// refusal as one line of JSON; a call whose answer is longer than the socket carries is told
/// so in place of the answer, and what it did stands. A request outside the protocol, or
/// longer than it admits, is refused and runs nothing, and so does a connection that breaks
/// before its request is whole.
///
/// Each answered request starts the idle count again. When it runs out the host ends exactly
/// as on `stop`: the session is dropped and the socket file and the state directory are
/// removed. Nothing else is ever deleted: the directory is removed only when it is empty.
///
/// Unix only: elsewhere this returns [`SessionError::Unsupported`].
pub fn serve(
    state_dir: &Path,
    session: Session,
    idle_expiry: Duration,
) -> Result<(), SessionError> {
    imp::serve(state_dir, session, idle_expiry)
}

#[cfg(unix)]
mod imp {
    use std::fs::{self, DirBuilder, Permissions};
    use std::io::{self, Write};
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::Path;
    use std::thread;
    use std::time::{Duration, Instant};

    use crate::wire::{self, Line, Reply, Request};
    use crate::{Session, SessionError};

    /// How often an idle host looks for a connection.
    const POLL: Duration = Duration::from_millis(25);

    pub(super) fn serve(
        state_dir: &Path,
        mut session: Session,
        idle_expiry: Duration,
    ) -> Result<(), SessionError> {
        let created = prepare_state_dir(state_dir)?;
        let socket = state_dir.join(wire::SOCKET_FILE);
        let listener = match bind(&socket) {
            Ok(listener) => listener,
            Err(error) => {
                if created {
                    let _ = fs::remove_dir(state_dir);
                }
                return Err(error);
            }
        };

        let served = answer_until_done(&listener, &mut session, idle_expiry);

        drop(session);
        drop(listener);
        let removed = fs::remove_file(&socket).map_err(io_error);
        let _ = fs::remove_dir(state_dir);
        served.and(removed)
    }

    fn io_error(error: io::Error) -> SessionError {
        SessionError::Io(error.kind())
    }

    /// Whether this call created the directory.
    fn prepare_state_dir(state_dir: &Path) -> Result<bool, SessionError> {
        match DirBuilder::new().mode(0o700).create(state_dir) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let mode = fs::metadata(state_dir)
                    .map_err(io_error)?
                    .permissions()
                    .mode();
                if mode & 0o077 == 0 {
                    Ok(false)
                } else {
                    Err(SessionError::StateDirNotPrivate)
                }
            }
            Err(error) => Err(io_error(error)),
        }
    }

    fn bind(socket: &Path) -> Result<UnixListener, SessionError> {
        let listener = match UnixListener::bind(socket) {
            Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
                match UnixStream::connect(socket) {
                    Ok(_) => return Err(SessionError::AlreadyRunning),
                    Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                        fs::remove_file(socket).map_err(io_error)?;
                        UnixListener::bind(socket)
                    }
                    Err(error) => return Err(io_error(error)),
                }
            }
            bound => bound,
        }
        .map_err(bind_error)?;
        fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
        Ok(listener)
    }

    /// `std` refuses a path with no room for a socket address as `InvalidInput`.
    fn bind_error(error: io::Error) -> SessionError {
        match error.kind() {
            io::ErrorKind::InvalidInput => SessionError::StateDirTooLong,
            kind => SessionError::Io(kind),
        }
    }

    /// Answers connections until a `stop` arrives or none has been answered for `idle_expiry`.
    fn answer_until_done(
        listener: &UnixListener,
        session: &mut Session,
        idle_expiry: Duration,
    ) -> Result<(), SessionError> {
        listener.set_nonblocking(true).map_err(io_error)?;
        let mut served: u64 = 0;
        let mut last_answer = Instant::now();
        loop {
            let mut stream = match listener.accept() {
                Ok((stream, _)) => stream,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if last_answer.elapsed() >= idle_expiry {
                        return Ok(());
                    }
                    thread::sleep(POLL);
                    continue;
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(io_error(error)),
            };
            // A connection that breaks is not answered: it neither counts nor restarts the
            // expiry.
            if let Ok(stop) = answer(&mut stream, session, served, idle_expiry) {
                served += 1;
                last_answer = Instant::now();
                if stop {
                    return Ok(());
                }
            }
        }
    }

    /// Answers one connection: whether it carried a `stop`.
    fn answer(
        stream: &mut UnixStream,
        session: &mut Session,
        served: u64,
        idle_expiry: Duration,
    ) -> io::Result<bool> {
        // A stream accepted off a non-blocking listener is non-blocking itself on some
        // platforms; its reads and writes wait, each under its bound.
        stream.set_nonblocking(false)?;
        stream.set_read_timeout(Some(wire::IO_BOUND))?;
        stream.set_write_timeout(Some(wire::IO_BOUND))?;
        let parsed = match wire::read_line(&*stream, wire::MAX_REQUEST_BYTES)? {
            Line::Text(line) => Request::parse(&line),
            Line::TooLong => Err(Reply::RefusedMalformed),
            Line::Unterminated => return Err(io::ErrorKind::UnexpectedEof.into()),
        };
        let (reply, stop) = match parsed {
            Ok(Request::Hello) => (
                Reply::Hello {
                    pid: std::process::id(),
                    label: session.label().to_string(),
                    served,
                    idle_expiry_s: idle_expiry.as_secs(),
                },
                false,
            ),
            Ok(Request::Stop) => (Reply::Stopping, true),
            Ok(Request::Call(call)) => {
                let reply = match session.run(&call) {
                    Ok(outcome) => wire::frame(true, outcome.to_json()),
                    Err(refusal) => wire::frame(false, refusal.to_json()),
                };
                (reply, false)
            }
            Err(refusal) => (refusal, false),
        };
        stream.write_all(reply.encode().as_bytes())?;
        Ok(stop)
    }
}

#[cfg(not(unix))]
mod imp {
    use std::path::Path;
    use std::time::Duration;

    use crate::{Session, SessionError};

    pub(super) fn serve(
        _state_dir: &Path,
        _session: Session,
        _idle_expiry: Duration,
    ) -> Result<(), SessionError> {
        Err(SessionError::Unsupported)
    }
}
