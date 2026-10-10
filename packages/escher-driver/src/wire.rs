//! The session's line protocol: one request line and one reply line per connection, each
//! `\n`-terminated. A request is `hello`, `stop` or a call; a reply carries the lifecycle's
//! facts, or a call's answer as one line of JSON.

use std::io::{self, BufRead, BufReader, Read};
use std::time::Duration;

use crate::command::{ArgValue, Call};
use crate::session::valid_label;

/// The socket's file name inside the state directory.
pub(crate) const SOCKET_FILE: &str = "session.sock";

/// The bound on each read and each write of a connection.
pub(crate) const IO_BOUND: Duration = Duration::from_secs(2);

/// The bound on each read of a call's answer: the call runs before its answer is written.
pub(crate) const ANSWER_BOUND: Duration = Duration::from_secs(30);

/// The longest request line, without its terminator. The widest call the schema admits — a
/// `type` whose 1024-byte id and 4096-byte text are escaped byte for byte — is 15,360 bytes of
/// values.
pub(crate) const MAX_REQUEST_BYTES: usize = 16_384;

/// The longest lifecycle reply line, without its terminator: a `hello` reply with every field
/// at its widest is 122 bytes.
pub(crate) const MAX_REPLY_BYTES: usize = 128;

/// The longest answer a call's reply carries: the JSON line alone, without the words in front
/// of it.
pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;

/// The longest reply line to a call, without its terminator: the widest words in front of an
/// answer are `ok v2 accepted `.
pub(crate) const MAX_CALL_REPLY_BYTES: usize = MAX_ANSWER_BYTES + 16;

const VERSION: &str = "v2";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Request {
    Hello,
    Stop,
    Call(Call),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Reply {
    Hello {
        pid: u32,
        label: String,
        served: u64,
        idle_expiry_s: u64,
    },
    Stopping,
    /// A call that ran, with its outcome as one line of JSON.
    Accepted(String),
    /// A call the session refused, with its refusal as one line of JSON.
    Refused(String),
    /// A call whose answer is over [`MAX_ANSWER_BYTES`]. It was not refused: what it did stands.
    Oversize,
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

/// Whether `byte` crosses the wire as itself: an ASCII letter, a digit or one of `-`, `_`,
/// `.`, `~`.
fn plain(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~')
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// `text` with every byte that is not [`plain`] written as `%` and two uppercase hex digits.
fn escape(text: &str, out: &mut String) {
    for byte in text.bytes() {
        if plain(byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
}

/// The text `word` escapes, or `None` when `word` is not what [`escape`] writes for any text.
fn unescape(word: &str) -> Option<String> {
    let hex = |digit: u8| HEX.iter().position(|known| *known == digit);
    let mut bytes = Vec::with_capacity(word.len());
    let mut rest = word.bytes();
    while let Some(byte) = rest.next() {
        if plain(byte) {
            bytes.push(byte);
        } else if byte == b'%' {
            let (high, low) = (hex(rest.next()?)?, hex(rest.next()?)?);
            let escaped = u8::try_from(high * 16 + low).ok()?;
            if plain(escaped) {
                return None;
            }
            bytes.push(escaped);
        } else {
            return None;
        }
    }
    String::from_utf8(bytes).ok()
}

fn encode_call(call: &Call) -> String {
    let mut line = format!("call {VERSION} ");
    escape(&call.verb, &mut line);
    for (name, value) in &call.args {
        line.push(' ');
        escape(name, &mut line);
        line.push('=');
        match value {
            ArgValue::Text(text) => {
                line.push_str("t:");
                escape(text, &mut line);
            }
            ArgValue::Number(number) => line.push_str(&format!("n:{number}")),
            ArgValue::Flag(flag) => line.push_str(if *flag { "f:true" } else { "f:false" }),
        }
    }
    line.push('\n');
    line
}

/// The call `words` carry — the verb, then one word per argument — or `None` when they are
/// not what [`encode_call`] writes for any call.
fn decode_call(words: &[&str]) -> Option<Call> {
    let (verb, arguments) = words.split_first()?;
    let mut args = Vec::with_capacity(arguments.len());
    for argument in arguments {
        let (name, kinded) = argument.split_once('=')?;
        let (kind, value) = kinded.split_once(':')?;
        let value = match kind {
            "t" => ArgValue::Text(unescape(value)?),
            "n" => {
                let number: i64 = value.parse().ok()?;
                // One spelling per number: `+7` and `007` are not what the encoder writes.
                (number.to_string() == value).then_some(ArgValue::Number(number))?
            }
            "f" => match value {
                "true" => ArgValue::Flag(true),
                "false" => ArgValue::Flag(false),
                _ => return None,
            },
            _ => return None,
        };
        args.push((unescape(name)?, value));
    }
    Some(Call {
        verb: unescape(verb)?,
        args,
    })
}

impl Request {
    pub(crate) fn encode(&self) -> String {
        match self {
            Request::Hello => format!("hello {VERSION}\n"),
            Request::Stop => format!("stop {VERSION}\n"),
            Request::Call(call) => encode_call(call),
        }
    }

    /// The request `line` names, or the refusal it is answered with.
    pub(crate) fn parse(line: &str) -> Result<Request, Reply> {
        if line.len() > MAX_REQUEST_BYTES {
            return Err(Reply::RefusedMalformed);
        }
        let mut words = line.split(' ');
        let (Some(word), Some(version)) = (words.next(), words.next()) else {
            return Err(Reply::RefusedMalformed);
        };
        let rest: Vec<&str> = words.collect();
        let shaped = match word {
            "hello" | "stop" => rest.is_empty(),
            "call" => !rest.is_empty(),
            _ => false,
        };
        if !shaped {
            return Err(Reply::RefusedMalformed);
        }
        if version != VERSION {
            return Err(if is_version(version) {
                Reply::RefusedVersion
            } else {
                Reply::RefusedMalformed
            });
        }
        match word {
            "hello" => Ok(Request::Hello),
            "stop" => Ok(Request::Stop),
            _ => decode_call(&rest)
                .map(Request::Call)
                .ok_or(Reply::RefusedMalformed),
        }
    }
}

fn is_version(word: &str) -> bool {
    word.strip_prefix('v')
        .is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
}

/// The reply that carries a call's answer: `json` as an accepted or a refused answer, or
/// [`Reply::Oversize`] when it is over [`MAX_ANSWER_BYTES`].
pub(crate) fn frame(accepted: bool, json: String) -> Reply {
    if json.len() > MAX_ANSWER_BYTES {
        Reply::Oversize
    } else if accepted {
        Reply::Accepted(json)
    } else {
        Reply::Refused(json)
    }
}

impl Reply {
    pub(crate) fn encode(&self) -> String {
        match self {
            Reply::Hello {
                pid,
                label,
                served,
                idle_expiry_s,
            } => format!(
                "ok {VERSION} pid={pid} label={label} served={served} \
                 idle_expiry_s={idle_expiry_s}\n"
            ),
            Reply::Stopping => format!("ok {VERSION} stopping\n"),
            Reply::Accepted(json) => format!("ok {VERSION} accepted {json}\n"),
            Reply::Refused(json) => format!("ok {VERSION} refused {json}\n"),
            Reply::Oversize => format!("ok {VERSION} oversize\n"),
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
        let rest = line.strip_prefix("ok ")?.strip_prefix(VERSION)?;
        let rest = rest.strip_prefix(' ')?;
        // An answer is the rest of its line, whatever it holds: it is never split into words.
        let answer = |json: &str| {
            (!json.is_empty() && json.len() <= MAX_ANSWER_BYTES).then(|| json.to_string())
        };
        if let Some(json) = rest.strip_prefix("accepted ") {
            return answer(json).map(Reply::Accepted);
        }
        if let Some(json) = rest.strip_prefix("refused ") {
            return answer(json).map(Reply::Refused);
        }
        let fields: Vec<&str> = rest.split(' ').collect();
        match fields.as_slice() {
            ["stopping"] => Some(Reply::Stopping),
            ["oversize"] => Some(Reply::Oversize),
            [pid, label, served, idle_expiry_s] => {
                let label = label.strip_prefix("label=")?;
                valid_label(label).then_some(())?;
                Some(Reply::Hello {
                    pid: pid.strip_prefix("pid=")?.parse().ok()?,
                    label: label.to_string(),
                    served: served.strip_prefix("served=")?.parse().ok()?,
                    idle_expiry_s: idle_expiry_s.strip_prefix("idle_expiry_s=")?.parse().ok()?,
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
    use crate::command::validate;
    use crate::schema::{MAX_ID_BYTES, MAX_TEXT_BYTES};

    fn line_of(encoded: &str) -> &str {
        encoded.strip_suffix('\n').expect("a terminated line")
    }

    fn text(value: &str) -> ArgValue {
        ArgValue::Text(value.to_string())
    }

    fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {
        Call {
            verb: verb.to_string(),
            args: args
                .iter()
                .map(|(name, value)| (name.to_string(), value.clone()))
                .collect(),
        }
    }

    fn escaped(text: &str) -> String {
        let mut out = String::new();
        escape(text, &mut out);
        out
    }

    #[test]
    fn each_request_round_trips() {
        let requests = [
            Request::Hello,
            Request::Stop,
            Request::Call(call("snapshot", &[])),
            Request::Call(call("click", &[("id", text("save"))])),
        ];
        assert_eq!(requests.len(), 4);
        for (row, request) in requests.iter().enumerate() {
            let encoded = request.encode();
            assert!(encoded.is_ascii(), "row {row}");
            assert!(
                Request::parse(line_of(&encoded)).as_ref() == Ok(request),
                "row {row}"
            );
        }
        assert_eq!(Request::Hello.encode(), "hello v2\n");
        assert_eq!(Request::Stop.encode(), "stop v2\n");
        assert_eq!(requests[2].encode(), "call v2 snapshot\n");
        assert_eq!(requests[3].encode(), "call v2 click id=t:save\n");
    }

    #[test]
    fn each_reply_round_trips() {
        let lifecycle = [
            Reply::Hello {
                pid: 4242,
                label: "flight-booker".to_string(),
                served: 7,
                idle_expiry_s: 1800,
            },
            Reply::Hello {
                pid: u32::MAX,
                label: "x".repeat(32),
                served: u64::MAX,
                idle_expiry_s: u64::MAX,
            },
            Reply::Stopping,
            Reply::Oversize,
            Reply::RefusedVersion,
            Reply::RefusedMalformed,
        ];
        assert_eq!(lifecycle.len(), 6);
        for (row, reply) in lifecycle.iter().enumerate() {
            let encoded = reply.encode();
            assert!(encoded.is_ascii(), "row {row}");
            assert!(line_of(&encoded).len() <= MAX_REPLY_BYTES, "row {row}");
            assert!(
                Reply::parse(line_of(&encoded)).as_ref() == Some(reply),
                "row {row}"
            );
        }
        assert_eq!(line_of(&lifecycle[1].encode()).len(), 122);

        let answers = [
            Reply::Accepted(r#"{"text":"Button \"Save\" id=\"save\" @0,0 1x1\n"}"#.to_string()),
            Reply::Accepted(r#"{"settled":true,"added":[],"removed":[],"changed":[]}"#.to_string()),
            Reply::Accepted("{\"text\":\"é ✓ two  spaces\"}".to_string()),
            Reply::Refused(r#"{"refused":{"cause":"not-found"}}"#.to_string()),
        ];
        assert_eq!(answers.len(), 4);
        for (row, reply) in answers.iter().enumerate() {
            let encoded = reply.encode();
            assert!(
                line_of(&encoded).len() <= MAX_CALL_REPLY_BYTES,
                "answer {row}"
            );
            assert!(
                Reply::parse(line_of(&encoded)).as_ref() == Some(reply),
                "answer {row}"
            );
        }

        assert_eq!(
            Reply::Hello {
                pid: 4242,
                label: "counter".to_string(),
                served: 0,
                idle_expiry_s: 1800,
            }
            .encode(),
            "ok v2 pid=4242 label=counter served=0 idle_expiry_s=1800\n"
        );
        assert_eq!(Reply::Stopping.encode(), "ok v2 stopping\n");
        assert_eq!(Reply::Oversize.encode(), "ok v2 oversize\n");
        assert_eq!(
            Reply::Accepted("{}".to_string()).encode(),
            "ok v2 accepted {}\n"
        );
        assert_eq!(
            Reply::Refused("{}".to_string()).encode(),
            "ok v2 refused {}\n"
        );
    }

    #[test]
    fn a_known_word_with_another_version_is_refused_by_version() {
        let lines = [
            "hello v1",
            "stop v0",
            "hello v10",
            "stop v1",
            "call v1 snapshot",
            "call v3 click id=t:save",
        ];
        for line in lines {
            assert_eq!(Request::parse(line), Err(Reply::RefusedVersion), "{line:?}");
        }
    }

    #[test]
    fn anything_else_is_refused_as_malformed() {
        let over_long = format!("hello v2{}", " ".repeat(MAX_REQUEST_BYTES));
        let over_long_call = format!("call v2 type text=t:{}", "t".repeat(MAX_REQUEST_BYTES));
        let lines = [
            "",
            " ",
            "hello",
            "hello ",
            "hello  v2",
            "hello v2 ",
            "hello v2 now",
            "hello 2",
            "hello vx",
            "HELLO v2",
            "snapshot v2",
            "snapshot v1",
            "hello v2\r",
            "call",
            "call v2",
            "call vx snapshot",
            "CALL v2 snapshot",
            // A double space is an empty argument word.
            "call v2 click  id=t:save",
            "call v2 click ",
            // An argument with no name separator, or no kind separator.
            "call v2 click id",
            "call v2 click id=tsave",
            // A kind letter outside the three.
            "call v2 click id=x:save",
            "call v2 click id=T:save",
            "call v2 click id=:save",
            // A number or a flag that does not parse, or is not in its one spelling.
            "call v2 advance ms=n:",
            "call v2 advance ms=n:ten",
            "call v2 advance ms=n:+7",
            "call v2 advance ms=n:007",
            "call v2 advance ms=n:1.5",
            "call v2 advance ms=n:9223372036854775808",
            "call v2 press shift=f:",
            "call v2 press shift=f:yes",
            "call v2 press shift=f:TRUE",
            // A bad escape: cut short, not hex, lowercase hex, or of a byte that needs none.
            "call v2 click id=t:a%",
            "call v2 click id=t:a%2",
            "call v2 click id=t:a%2G",
            "call v2 click id=t:a%2f",
            "call v2 click id=t:%41",
            // A byte that is neither plain nor an escape.
            "call v2 click id=t:a/b",
            "call v2 click id=t:a:b",
            "call v2 click id=t:a=b",
            "call v2 click id=t:é",
            "call v2 cli/ck",
            "call v2 click i/d=t:save",
            // Escaped bytes that are not UTF-8.
            "call v2 click id=t:%FF",
            "call v2 click id=t:%C3",
            "call v2 %FF",
            "call v2 click %FF=t:save",
            "call v2 snapshot\r",
            &over_long,
            &over_long_call,
        ];
        assert_eq!(lines.len(), 51);
        for (row, line) in lines.into_iter().enumerate() {
            assert!(
                Request::parse(line) == Err(Reply::RefusedMalformed),
                "row {row}"
            );
        }
    }

    #[test]
    fn a_reply_outside_the_protocol_does_not_parse() {
        let over_long_answer = format!("ok v2 accepted {}", "x".repeat(MAX_ANSWER_BYTES + 1));
        let lines = [
            "",
            "ok",
            "ok v2",
            "ok v2 ",
            "ok v1 stopping",
            "ok v3 stopping",
            "ok v2 stopped",
            "ok v2 pid=1 label=counter served=0",
            "ok v2 pid=1 label=counter served=0 idle_expiry_s=1 more=1",
            "ok v2 pid=x label=counter served=0 idle_expiry_s=1",
            "ok v2 pid=-1 label=counter served=0 idle_expiry_s=1",
            "ok v2 pid=1 label=Counter served=0 idle_expiry_s=1",
            "ok v2 pid=1 label= served=0 idle_expiry_s=1",
            "ok v2 pid=1 label=counter served= idle_expiry_s=1",
            "ok v2 pid=1 label=counter served=0 idle_expiry_s=",
            "ok v2 pid=1 label=counter served=0 idle=1",
            "ok v2 label=counter pid=1 served=0 idle_expiry_s=1",
            "ok v2 accepted",
            "ok v2 accepted ",
            "ok v2 refused",
            "ok v2 refused ",
            "ok v2 oversize now",
            "ok v1 accepted {}",
            "refused",
            "refused other",
            &over_long_answer,
        ];
        assert_eq!(lines.len(), 26);
        for (row, line) in lines.into_iter().enumerate() {
            assert!(Reply::parse(line).is_none(), "row {row}");
        }
    }

    #[test]
    fn a_line_is_read_up_to_its_terminator_and_no_further() {
        assert_eq!(
            read_line(&b"hello v2\nstop v2\n"[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Text("hello v2".to_string())
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
            read_line(&b"hello v2"[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Unterminated
        );
        assert_eq!(
            read_line(&b""[..], MAX_REQUEST_BYTES).unwrap(),
            Line::Unterminated
        );
    }

    #[test]
    fn bytes_that_are_not_text_are_refused_as_malformed() {
        for raw in [&b"hello \xff2\n"[..], &b"call v2 click id=t:\xff\n"[..]] {
            let Line::Text(text) = read_line(raw, MAX_REQUEST_BYTES).unwrap() else {
                panic!("a terminated line is text");
            };
            assert_eq!(Request::parse(&text), Err(Reply::RefusedMalformed));
        }
    }

    #[test]
    fn a_byte_that_is_not_plain_is_written_as_its_escape() {
        let rows = [
            ("", ""),
            ("save", "save"),
            ("A-z_0.9~", "A-z_0.9~"),
            (" ", "%20"),
            ("\n", "%0A"),
            ("\0", "%00"),
            ("%", "%25"),
            ("=", "%3D"),
            (":", "%3A"),
            ("/", "%2F"),
            ("[2]", "%5B2%5D"),
            ("é", "%C3%A9"),
            ("\u{7f}", "%7F"),
            (
                "form//input[2]: a b\n",
                "form%2F%2Finput%5B2%5D%3A%20a%20b%0A",
            ),
        ];
        assert_eq!(rows.len(), 14);
        for (row, (text, word)) in rows.into_iter().enumerate() {
            assert!(escaped(text) == word, "row {row}");
            assert!(unescape(word).as_deref() == Some(text), "row {row}");
        }
        for byte in 0..=u8::MAX {
            let word = escaped(&char::from(byte).to_string());
            assert!(word.bytes().all(|byte| plain(byte) || byte == b'%'));
        }
    }

    #[test]
    fn a_decoded_call_is_the_call_that_was_encoded() {
        let every_byte: String = (0..=0x7fu8).map(char::from).collect();
        let rows = [
            call("snapshot", &[]),
            call("click", &[("id", text("save"))]),
            call("click", &[("id", text("form//input[2]: a b\nc"))]),
            call(
                "type",
                &[("id", text("crud-name")), ("text", text("Ada Lovelace\n"))],
            ),
            call("type", &[("id", text("who")), ("text", text(""))]),
            call(
                "type",
                &[("text", text(&every_byte)), ("id", text("é ✓ 日本"))],
            ),
            call(
                "press",
                &[("key", text("tab")), ("shift", ArgValue::Flag(true))],
            ),
            call(
                "press",
                &[("shift", ArgValue::Flag(false)), ("key", text("enter"))],
            ),
            call("advance", &[("ms", ArgValue::Number(250))]),
            call("advance", &[("ms", ArgValue::Number(0))]),
            call("advance", &[("ms", ArgValue::Number(-1))]),
            call("advance", &[("ms", ArgValue::Number(i64::MAX))]),
            call("advance", &[("ms", ArgValue::Number(i64::MIN))]),
            call("scroll", &[("id", text("crud-person-14"))]),
            // What the schema refuses still crosses whole: the session answers it.
            call("", &[]),
            call("no such verb", &[("", text("")), ("", text(""))]),
            call("click", &[("a=b:c d", text("=t:"))]),
            call("start", &[("app", text("counter"))]),
        ];
        assert_eq!(rows.len(), 18);
        for (row, call) in rows.iter().enumerate() {
            let encoded = Request::Call(call.clone()).encode();
            assert!(encoded.is_ascii(), "row {row}");
            assert!(
                encoded.matches('\n').count() == 1 && encoded.ends_with('\n'),
                "row {row}"
            );
            assert!(
                Request::parse(line_of(&encoded)) == Ok(Request::Call(call.clone())),
                "row {row}"
            );
        }
    }

    #[test]
    fn the_widest_call_the_schema_admits_fits_the_request_bound() {
        // Every byte of both values needs its three-byte escape.
        let widest = call(
            "type",
            &[
                ("id", text(&" ".repeat(MAX_ID_BYTES))),
                ("text", text(&"\n".repeat(MAX_TEXT_BYTES))),
            ],
        );
        assert!(validate(&widest).is_ok());
        let encoded = Request::Call(widest.clone()).encode();
        let line = line_of(&encoded);
        assert_eq!((MAX_ID_BYTES + MAX_TEXT_BYTES) * 3, 15_360);
        assert!(line.len() > 15_360 && line.len() <= MAX_REQUEST_BYTES);
        assert!(Request::parse(line) == Ok(Request::Call(widest)));
        assert_eq!(MAX_REQUEST_BYTES, 16_384);
    }

    #[test]
    fn an_answer_at_its_bound_is_carried_and_one_byte_over_is_not() {
        assert_eq!(MAX_ANSWER_BYTES, 1_048_576);
        for accepted in [true, false] {
            let at_the_bound = "x".repeat(MAX_ANSWER_BYTES);
            let framed = frame(accepted, at_the_bound.clone());
            let carried = if accepted {
                Reply::Accepted(at_the_bound)
            } else {
                Reply::Refused(at_the_bound)
            };
            assert!(framed == carried, "accepted={accepted}: at the bound");
            let encoded = framed.encode();
            assert!(line_of(&encoded).len() <= MAX_CALL_REPLY_BYTES);
            assert!(
                Reply::parse(line_of(&encoded)) == Some(carried),
                "accepted={accepted}: an answer at the bound round-trips"
            );

            let one_over = "x".repeat(MAX_ANSWER_BYTES + 1);
            assert!(
                frame(accepted, one_over) == Reply::Oversize,
                "accepted={accepted}: one byte over"
            );
        }
        assert_eq!(Reply::parse("ok v2 oversize"), Some(Reply::Oversize));
        assert!(frame(true, "{}".to_string()) == Reply::Accepted("{}".to_string()));
        assert!(frame(false, "{}".to_string()) == Reply::Refused("{}".to_string()));
    }
}
