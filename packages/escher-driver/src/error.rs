//! The one outcome type of the session lifecycle.

use std::fmt;
use std::io;

/// Why a session lifecycle step did not do what was asked.
///
/// Every message is a fixed string: none carries a path, a label, an id or a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    /// A session is already answering in the state directory.
    AlreadyRunning,
    /// No session exists in the state directory.
    NoSession,
    /// The state directory holds a session's socket, and no process answers on it.
    Dead,
    /// The host process exited before its session answered, with this exit code when it had one.
    HostExited(Option<i32>),
    /// A bounded wait ran out.
    Timeout,
    /// The other end answered outside the lifecycle protocol.
    Protocol,
    /// The label is not 1 to 32 bytes of `a-z`, `0-9` and `-`.
    InvalidLabel,
    /// The state directory's path leaves no room for a socket address.
    StateDirTooLong,
    /// The state directory grants access to its group or to others.
    StateDirNotPrivate,
    /// The platform has no local socket in `std`.
    Unsupported,
    /// An I/O step failed with this kind.
    Io(io::ErrorKind),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionError::AlreadyRunning => f.write_str("a session is already running"),
            SessionError::NoSession => f.write_str("no session is running"),
            SessionError::Dead => f.write_str("the session's process is gone"),
            SessionError::HostExited(_) => {
                f.write_str("the session host exited before it answered")
            }
            SessionError::Timeout => f.write_str("the session did not answer in time"),
            SessionError::Protocol => f.write_str("the session answered outside its protocol"),
            SessionError::InvalidLabel => {
                f.write_str("a session label is 1 to 32 bytes of a-z, 0-9 and -")
            }
            SessionError::StateDirTooLong => {
                f.write_str("the session state directory's path is too long for a socket")
            }
            SessionError::StateDirNotPrivate => {
                f.write_str("the session state directory is open to its group or to others")
            }
            SessionError::Unsupported => {
                f.write_str("the session lifecycle is not supported on this platform")
            }
            SessionError::Io(kind) => write!(f, "a session input or output step failed: {kind:?}"),
        }
    }
}

impl std::error::Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_message_is_non_empty_and_holds_no_path() {
        let variants = [
            SessionError::AlreadyRunning,
            SessionError::NoSession,
            SessionError::Dead,
            SessionError::HostExited(None),
            SessionError::HostExited(Some(101)),
            SessionError::Timeout,
            SessionError::Protocol,
            SessionError::InvalidLabel,
            SessionError::StateDirTooLong,
            SessionError::StateDirNotPrivate,
            SessionError::Unsupported,
            SessionError::Io(io::ErrorKind::NotFound),
            SessionError::Io(io::ErrorKind::PermissionDenied),
            SessionError::Io(io::ErrorKind::Other),
        ];
        for variant in variants {
            let message = variant.to_string();
            assert!(!message.is_empty(), "{variant:?}");
            assert!(!message.contains('/'), "{variant:?}");
        }
    }

    #[test]
    fn an_exit_code_is_not_in_the_message() {
        assert_eq!(
            SessionError::HostExited(Some(101)).to_string(),
            SessionError::HostExited(None).to_string()
        );
    }

    #[test]
    fn the_io_message_adds_the_kind() {
        assert!(
            SessionError::Io(io::ErrorKind::NotFound)
                .to_string()
                .ends_with("NotFound")
        );
    }
}
