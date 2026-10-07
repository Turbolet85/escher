//! The lifecycle line protocol: one request line and one reply line per connection, ASCII,
//! `\n`-terminated. It carries the lifecycle only — nothing of the screen.

use std::io::{self, BufRead, BufReader, Read};
use std::time::Duration;

use crate::session::valid_label;

/// The socket's file name inside the state directory.
pub(crate) const SOCKET_FILE: &str = "session.sock";

/// The bound on each read and each write of a connection.
pub(crate) const IO_BOUND: Duration = Duration::from_secs(2);

/// The longest request line, without its terminator.
pub(crate) const MAX_REQUEST_BYTES: usize = 64;

/// The longest reply line, without its terminator: a `hello` reply with every field at its
/// widest is 87 bytes.
pub(crate) const MAX_REPLY_BYTES: usize = 128;

const VERSION: &str = "v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Request {
    Hello,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Reply {
    Hello {
        pid: u32,
        label: String,
        served: u64,
    },
    Stopping,
    RefusedVersion,
    RefusedMalformed,
}

/// One line read off a connection.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Line {
    Text(String),
    TooLong,
    /// The stream ended before a terminator.
    Unterminated,
}

impl Request {
    pub(crate) fn encode(self) -> String {
        let word = match self {
            Request::Hello => "hello",
            Request::Stop => "stop",
        };
        format!("{word} {VERSION}\n")
    }

    /// The request `line` names, or the refusal it is answered with.
    pub(crate) fn parse(line: &str) -> Result<Request, Reply> {
        if line.len() > MAX_REQUEST_BYTES {
            return Err(Reply::RefusedMalformed);
        }
        let mut words = line.split(' ');
        let (Some(word), Some(version), None) = (words.next(), words.next(), words.next()) else {
            return Err(Reply::RefusedMalformed);
        };
        let request = match word {
            "hello" => Request::Hello,
            "stop" => Request::Stop,
            _ => return Err(Reply::RefusedMalformed),
        };
        if version == VERSION {
            Ok(request)
        } else if is_version(version) {
            Err(Reply::RefusedVersion)
        } else {
            Err(Reply::RefusedMalformed)
        }
    }
}

fn is_version(word: &str) -> bool {
    word.strip_prefix('v')
        .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
}

impl Reply {
    pub(crate) fn encode(&self) -> String {
        match self {
            Reply::Hello { pid, label, served } => {
                format!("ok {VERSION} pid={pid} label={label} served={served}\n")
            }
            Reply::Stopping => format!("ok {VERSION} stopping\n"),
            Reply::RefusedVersion => "refused version\n".to_string(),
            Reply::RefusedMalformed => "refused malformed\n".to_string(),
        }
    }

    pub(crate) fn parse(line: &str) -> Option<Reply> {
        match line {
            "refused version" => return Some(Reply::RefusedVersion),
            "refused malformed" => return Some(Reply::RefusedMalformed),
            _ => {}
        }
        let mut words = line.split(' ');
        if (words.next(), words.next()) != (Some("ok"), Some(VERSION)) {
            return None;
        }
        let fields: Vec<&str> = words.collect();
        match fields.as_slice() {
            ["stopping"] => Some(Reply::Stopping),
            [pid, label, served] => {
                let label = label.strip_prefix("label=")?;
                valid_label(label).then_some(())?;
                Some(Reply::Hello {
                    pid: pid.strip_prefix("pid=")?.parse().ok()?,
                    label: label.to_string(),
                    served: served.strip_prefix("served=")?.parse().ok()?,
                })
            }
            _ => None,
        }
    }
}

/// Reads one line of at most `max` bytes, without its terminator.
pub(crate) fn read_line(reader: impl Read, max: usize) -> io::Result<Line> {
    let mut bytes = Vec::new();
    BufReader::new(reader.take(max as u64 + 1)).read_until(b'\n', &mut bytes)?;
    if bytes.last() != Some(&b'\n') {
        return Ok(if bytes.len() > max {
            Line::TooLong
        } else {
            Line::Unterminated
        });
    }
    bytes.pop();
    Ok(Line::Text(String::from_utf8_lossy(&bytes).into_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_of(encoded: &str) -> &str {
        encoded.strip_suffix('\n').expect("a terminated line")
    }

    #[test]
    fn each_request_round_trips() {
        for request in [Request::Hello, Request::Stop] {
            let encoded = request.encode();
            assert!(encoded.is_ascii());
            assert_eq!(Request::parse(line_of(&encoded)), Ok(request));
        }
        assert_eq!(Request::Hello.encode(), "hello v1\n");
        assert_eq!(Request::Stop.encode(), "stop v1\n");
    }

    #[test]
    fn each_reply_round_trips() {
        let replies = [
            Reply::Hello {
                pid: 4242,
                label: "flight-booker".to_string(),
                served: 7,
            },
            Reply::Hello {
                pid: u32::MAX,
                label: "x".repeat(32),
                served: u64::MAX,
            },
            Reply::Stopping,
            Reply::RefusedVersion,
            Reply::RefusedMalformed,
        ];
        for reply in replies {
            let encoded = reply.encode();
            assert!(encoded.is_ascii());
            assert!(line_of(&encoded).len() <= MAX_REPLY_BYTES);
            assert_eq!(Reply::parse(line_of(&encoded)), Some(reply));
        }
        assert_eq!(
            Reply::Hello {
                pid: 4242,
                label: "counter".to_string(),
                served: 0,
            }
            .encode(),
            "ok v1 pid=4242 label=counter served=0\n"
        );
        assert_eq!(Reply::Stopping.encode(), "ok v1 stopping\n");
    }

    #[test]
    fn a_known_word_with_another_version_is_refused_by_version() {
        for line in ["hello v2", "stop v0", "hello v10"] {
            assert_eq!(Request::parse(line), Err(Reply::RefusedVersion), "{line:?}");
        }
    }

    #[test]
    fn anything_else_is_refused_as_malformed() {
        let over_long = format!("hello v1{}", " ".repeat(MAX_REQUEST_BYTES));
        let lines = [
            "",
            " ",
            "hello",
            "hello ",
            "hello  v1",
            "hello v1 ",
            "hello v1 now",
            "hello 1",
            "hello vx",
            "HELLO v1",
            "snapshot v1",
            "snapshot v2",
            "hello v1\r",
            &over_long,
        ];
        for line in lines {
            assert_eq!(
                Request::parse(line),
                Err(Reply::RefusedMalformed),
                "{line:?}"
            );
        }
    }

    #[test]
    fn a_reply_outside_the_protocol_does_not_parse() {
        let lines = [
            "",
            "ok",
            "ok v1",
            "ok v2 stopping",
            "ok v1 stopped",
            "ok v1 pid=1 label=counter",
            "ok v1 pid=1 label=counter served=0 more=1",
            "ok v1 pid=x label=counter served=0",
            "ok v1 pid=-1 label=counter served=0",
            "ok v1 pid=1 label=Counter served=0",
            "ok v1 pid=1 label= served=0",
            "ok v1 pid=1 label=counter served=",
            "ok v1 label=counter pid=1 served=0",
            "refused",
            "refused other",
        ];
        for line in lines {
            assert_eq!(Reply::parse(line), None, "{line:?}");
        }
    }

    #[test]
    fn a_line_is_read_up_to_its_terminator_and_no_further() {
        assert_eq!(
            read_line(&b"hello v1\nstop v1\n"[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Text("hello v1".to_string())
        );
        assert_eq!(
            read_line(&b"\n"[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Text(String::new())
        );
        let widest = format!("{}\n", "x".repeat(MAX_REQUEST_BYTES));
        assert_eq!(
            read_line(widest.as_bytes(), MAX_REQUEST_BYTES).unwrap(),
            Line::Text("x".repeat(MAX_REQUEST_BYTES))
        );
    }

    #[test]
    fn an_over_long_line_and_an_unterminated_one_are_named() {
        let over_long = format!("{}\n", "x".repeat(MAX_REQUEST_BYTES + 1));
        assert_eq!(
            read_line(over_long.as_bytes(), MAX_REQUEST_BYTES).unwrap(),
            Line::TooLong
        );
        assert_eq!(
            read_line(&b"hello v1"[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Unterminated
        );
        assert_eq!(
            read_line(&b""[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Unterminated
        );
    }

    #[test]
    fn bytes_that_are_not_text_are_refused_as_malformed() {
        let Line::Text(text) = read_line(&b"hello \xff1\n"[..], MAX_REQUEST_BYTES).unwrap() else {
            panic!("a terminated line is text");
        };
        assert_eq!(Request::parse(&text), Err(Reply::RefusedMalformed));
    }
}
