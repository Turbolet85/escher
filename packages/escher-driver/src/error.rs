//! The one outcome type of the session's edges: its lifecycle, and a call's way to it and back.

use std::fmt;
use std::io;

use blitz_test_harness::Busy;

/// Why a session step did not do what was asked.
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
    /// The instance did not go quiet after a step: work of this class was still outstanding.
    NotSettled(Busy),
    /// A call's answer is longer than the session's socket carries. The call ran, and what it
    /// did is not rolled back.
    AnswerTooLarge,
}

/// One row of the session errors' table: an error's kind, what it means and what the caller
/// does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ErrorSpec {
    /// The error's kind.
    pub(crate) kind: &'static str,
    /// What it means: the error's fixed message, without the detail a variant adds.
    pub(crate) meaning: &'static str,
    /// What the caller does next.
    pub(crate) remedy: &'static str,
    /// The earlier kind whose remedy this row repeats word for word; not served.
    pub(crate) repeats: Option<&'static str>,
}

/// The state directory's bound, as the message of [`SessionError::StateDirTooLong`] states it:
/// a socket address holds a path shorter than 108 bytes on Linux and 104 elsewhere, and the
/// session's socket adds 13 bytes after the directory.
#[cfg(target_os = "linux")]
const STATE_DIR_TOO_LONG: &str =
    "the session state directory's path is too long for a socket: as given, it is at most 94 bytes";
#[cfg(not(target_os = "linux"))]
const STATE_DIR_TOO_LONG: &str =
    "the session state directory's path is too long for a socket: as given, it is at most 90 bytes";

/// The remedy two kinds share where the session's other end is missing.
const START_HERE: &str =
    "start a session in this state directory, or name the directory a running session uses";

/// The remedy two kinds share where a call ran and its answer does not say how the screen stands.
const READ_THE_SCREEN: &str =
    "the call ran and is not rolled back: take a snapshot to read the screen as it stands";

/// Every session error's row, in the variants' order.
pub(crate) const SESSION_ERRORS: [ErrorSpec; 13] = [
    ErrorSpec {
        kind: "already-running",
        meaning: "a session is already running",
        remedy: "stop the running session, or name another state directory",
        repeats: None,
    },
    ErrorSpec {
        kind: "no-session",
        meaning: "no session is running",
        remedy: START_HERE,
        repeats: None,
    },
    ErrorSpec {
        kind: "dead",
        meaning: "the session's process is gone",
        remedy: START_HERE,
        repeats: Some("no-session"),
    },
    ErrorSpec {
        kind: "host-exited",
        meaning: "the session host exited before it answered",
        remedy: "start the session again, and if its host exits again, name another state \
                 directory",
        repeats: None,
    },
    ErrorSpec {
        kind: "timeout",
        meaning: "the session did not answer in time",
        remedy: "the command may have run: ask `status`, then take a snapshot before acting again",
        repeats: None,
    },
    ErrorSpec {
        kind: "protocol",
        meaning: "the session answered outside its protocol",
        remedy: "stop the session with the binary that started it, or name another state \
                 directory",
        repeats: None,
    },
    ErrorSpec {
        kind: "invalid-label",
        meaning: "a session label is 1 to 32 bytes of a-z, 0-9 and -",
        remedy: "name an app of 1 to 32 bytes of a-z, 0-9 and -, as the apps list names it",
        repeats: None,
    },
    ErrorSpec {
        kind: "state-dir-too-long",
        meaning: STATE_DIR_TOO_LONG,
        remedy: "name a shorter path for the state directory, or a relative one, which counts \
                 only its own bytes",
        repeats: None,
    },
    ErrorSpec {
        kind: "state-dir-not-private",
        meaning: "the session state directory is open to its group or to others",
        remedy: "name a new state directory, or make this one open to its owner only",
        repeats: None,
    },
    ErrorSpec {
        kind: "unsupported",
        meaning: "the session lifecycle is not supported on this platform",
        remedy: "hold the session in process instead: the `mcp` role holds it in its own process",
        repeats: None,
    },
    ErrorSpec {
        kind: "io",
        meaning: "a session input or output step failed",
        remedy: "check that the state directory exists and its owner can write it, then retry",
        repeats: None,
    },
    ErrorSpec {
        kind: "not-settled",
        meaning: "the instance did not go quiet after a step",
        remedy: READ_THE_SCREEN,
        repeats: None,
    },
    ErrorSpec {
        kind: "answer-too-large",
        meaning: "the answer is larger than the session carries; the call ran and is not rolled \
                  back",
        remedy: READ_THE_SCREEN,
        repeats: Some("not-settled"),
    },
];

impl SessionError {
    /// The error's row of the session errors' table.
    const fn spec(&self) -> &'static ErrorSpec {
        let index = match self {
            SessionError::AlreadyRunning => 0,
            SessionError::NoSession => 1,
            SessionError::Dead => 2,
            SessionError::HostExited(_) => 3,
            SessionError::Timeout => 4,
            SessionError::Protocol => 5,
            SessionError::InvalidLabel => 6,
            SessionError::StateDirTooLong => 7,
            SessionError::StateDirNotPrivate => 8,
            SessionError::Unsupported => 9,
            SessionError::Io(_) => 10,
            SessionError::NotSettled(_) => 11,
            SessionError::AnswerTooLarge => 12,
        };
        &SESSION_ERRORS[index]
    }

    /// The error's kind: lowercase words joined by hyphens, one per variant.
    pub const fn kind(&self) -> &'static str {
        self.spec().kind
    }

    /// What the caller does next: a fixed string, one per variant.
    pub const fn remedy(&self) -> &'static str {
        self.spec().remedy
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meaning = self.spec().meaning;
        match self {
            SessionError::Io(kind) => write!(f, "{meaning}: {kind:?}"),
            SessionError::NotSettled(busy) => write!(f, "{meaning}: {busy:?}"),
            _ => f.write_str(meaning),
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
            SessionError::NotSettled(Busy::Render),
            SessionError::NotSettled(Busy::Layout),
            SessionError::NotSettled(Busy::Loads),
            SessionError::AnswerTooLarge,
        ];
        assert_eq!(variants.len(), 18);
        for variant in variants {
            let message = variant.to_string();
            assert!(!message.is_empty(), "{variant:?}");
            assert!(!message.contains('/'), "{variant:?}");
            let kind = variant.kind();
            assert!(!kind.is_empty(), "{variant:?}");
            assert!(
                kind.bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte == b'-'),
                "{variant:?}"
            );
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
    fn the_thirteen_errors_each_state_a_kind_a_meaning_and_a_remedy() {
        let variants = [
            SessionError::AlreadyRunning,
            SessionError::NoSession,
            SessionError::Dead,
            SessionError::HostExited(None),
            SessionError::Timeout,
            SessionError::Protocol,
            SessionError::InvalidLabel,
            SessionError::StateDirTooLong,
            SessionError::StateDirNotPrivate,
            SessionError::Unsupported,
            SessionError::Io(io::ErrorKind::NotFound),
            SessionError::NotSettled(Busy::Render),
            SessionError::AnswerTooLarge,
        ];
        assert_eq!(SESSION_ERRORS.len(), 13);
        assert_eq!(variants.len(), SESSION_ERRORS.len());
        for (row, (variant, spec)) in variants.iter().zip(&SESSION_ERRORS).enumerate() {
            assert!(variant.kind() == spec.kind, "row {row}");
            assert!(variant.remedy() == spec.remedy, "row {row}");
            assert!(variant.to_string().starts_with(spec.meaning), "row {row}");
            assert!(
                !spec.meaning.is_empty() && !spec.remedy.is_empty(),
                "row {row}"
            );
            assert!(
                !spec.meaning.contains('/') && !spec.remedy.contains('/'),
                "row {row}"
            );
        }
        // Two rows share a remedy exactly where the later names the earlier.
        for (row, spec) in SESSION_ERRORS.iter().enumerate() {
            for (earlier, other) in SESSION_ERRORS.iter().enumerate().take(row) {
                let shared = spec.remedy == other.remedy;
                assert!(
                    shared == (spec.repeats == Some(other.kind)),
                    "rows {earlier} and {row}"
                );
            }
            if let Some(kind) = spec.repeats {
                let named = SESSION_ERRORS[..row].iter().any(|other| other.kind == kind);
                assert!(named, "row {row}");
            }
        }
        let shared = SESSION_ERRORS
            .iter()
            .filter(|spec| spec.repeats.is_some())
            .count();
        assert_eq!(shared, 2);
    }

    #[test]
    fn the_state_directorys_message_states_its_bound() {
        let message = SessionError::StateDirTooLong.to_string();
        let bound = if cfg!(target_os = "linux") { 94 } else { 90 };
        assert!(message.ends_with(&format!("at most {bound} bytes")));
        assert!(
            SessionError::StateDirTooLong
                .remedy()
                .contains("a relative one")
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
