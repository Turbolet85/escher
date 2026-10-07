//! The host side: one process serving one [`Session`] until it is stopped.

use std::path::Path;

use crate::{Session, SessionError};

/// Hosts `session` in `state_dir` until a `stop` request arrives.
///
/// The state directory is created owner-only (mode `0700`) when it is absent — its parent must
/// exist — and refused with [`SessionError::StateDirNotPrivate`] when it is present and open to
/// its group or to others. The session answers on the Unix-domain socket `session.sock` inside
/// it (a `UnixListener`, mode `0600`), one connection at a time, on the calling thread: the
/// thread that owns the instance. A socket file no process answers on is replaced; one a
/// process answers on is [`SessionError::AlreadyRunning`].
///
/// On `stop` the session is dropped and the socket file and the state directory are removed.
/// Nothing else is ever deleted: the directory is removed only when it is empty.
///
/// Unix only: elsewhere this returns [`SessionError::Unsupported`].
pub fn serve(state_dir: &Path, session: Session) -> Result<(), SessionError> {
    imp::serve(state_dir, session)
}

#[cfg(unix)]
mod imp {
    use std::fs::{self, DirBuilder, Permissions};
    use std::io::{self, Write};
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::Path;

    use crate::wire::{self, Line, Reply, Request};
    use crate::{Session, SessionError};

    pub(super) fn serve(state_dir: &Path, session: Session) -> Result<(), SessionError> {
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

        let served = answer_until_stop(&listener, session.label());

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

    fn answer_until_stop(listener: &UnixListener, label: &str) -> Result<(), SessionError> {
        let mut served: u64 = 0;
        loop {
            let (mut stream, _) = listener.accept().map_err(io_error)?;
            match answer(&mut stream, label, served) {
                Ok(Some(request)) => {
                    served += 1;
                    if request == Request::Stop {
                        return Ok(());
                    }
                }
                Ok(None) => served += 1,
                // A broken connection changes nothing.
                Err(_) => {}
            }
        }
    }

    /// Answers one connection: the request it carried when that was one, `None` when it was
    /// refused.
    fn answer(stream: &mut UnixStream, label: &str, served: u64) -> io::Result<Option<Request>> {
        stream.set_read_timeout(Some(wire::IO_BOUND))?;
        stream.set_write_timeout(Some(wire::IO_BOUND))?;
        let parsed = match wire::read_line(&*stream, wire::MAX_REQUEST_BYTES)? {
            Line::Text(line) => Request::parse(&line),
            Line::TooLong => Err(Reply::RefusedMalformed),
            Line::Unterminated => return Err(io::ErrorKind::UnexpectedEof.into()),
        };
        let reply = match parsed {
            Ok(Request::Hello) => Reply::Hello {
                pid: std::process::id(),
                label: label.to_string(),
                served,
            },
            Ok(Request::Stop) => Reply::Stopping,
            Err(ref refusal) => refusal.clone(),
        };
        stream.write_all(reply.encode().as_bytes())?;
        Ok(parsed.ok())
    }
}

#[cfg(not(unix))]
mod imp {
    use std::path::Path;

    use crate::{Session, SessionError};

    pub(super) fn serve(_state_dir: &Path, _session: Session) -> Result<(), SessionError> {
        Err(SessionError::Unsupported)
    }
}
