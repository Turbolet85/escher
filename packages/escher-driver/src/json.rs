//! The written form of what a command returns: one line of JSON per answer, written by hand
//! and keyed by the schema's own words. Output only: nothing here reads JSON.

use std::fmt::Write as _;

use blitz_test_harness::Busy;
use dioxus_native_dom::{DiffNode, SnapshotDiff};

use crate::client::Hello;
use crate::error::SessionError;
use crate::execute::{Outcome, busy_word};
use crate::refusal::Refusal;

/// Writes `text` as a JSON string: `"` and `\` escaped, a control character as `\n`, `\r`,
/// `\t` or `\u00XX`, everything else as it is.
fn string(out: &mut String, text: &str) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if control < '\u{20}' => {
                let _ = write!(out, "\\u{:04x}", u32::from(control));
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

/// Writes `value` as the shortest decimal that reads back the same; a value that is not
/// finite, which JSON has no number for, as `null`.
fn number(out: &mut String, value: f64) {
    if value.is_finite() {
        let _ = write!(out, "{value}");
    } else {
        out.push_str("null");
    }
}

/// A JSON object being written, its members in the order they are added.
struct Object {
    text: String,
    members: usize,
}

impl Object {
    fn new() -> Object {
        Object {
            text: String::from("{"),
            members: 0,
        }
    }

    /// Writes the member's key and returns the text its value is written to.
    fn member(&mut self, key: &str) -> &mut String {
        if self.members > 0 {
            self.text.push(',');
        }
        self.members += 1;
        string(&mut self.text, key);
        self.text.push(':');
        &mut self.text
    }

    fn text(&mut self, key: &str, value: &str) {
        string(self.member(key), value);
    }

    fn flag(&mut self, key: &str, value: bool) {
        self.member(key)
            .push_str(if value { "true" } else { "false" });
    }

    fn count(&mut self, key: &str, value: u64) {
        let _ = write!(self.member(key), "{value}");
    }

    /// A member whose value is already JSON.
    fn json(&mut self, key: &str, value: &str) {
        self.member(key).push_str(value);
    }

    fn end(mut self) -> String {
        self.text.push('}');
        self.text
    }
}

/// What a node of a `nodes` result field reads, by the names of the schema's node fields.
struct NodeParts<'a> {
    id: &'a str,
    parent: Option<&'a str>,
    role: &'a str,
    name: &'a str,
    enabled: Option<bool>,
    checked: Option<bool>,
    value: Option<&'a str>,
    focused: bool,
    /// x, y, width and height.
    bounds: [f64; 4],
}

/// A node as an object with the fields the schema's node fields name, in that order. A field
/// the node has no reading for is left out.
fn node(parts: &NodeParts<'_>) -> String {
    let mut object = Object::new();
    object.text("id", parts.id);
    if let Some(parent) = parts.parent {
        object.text("parent", parent);
    }
    object.text("role", parts.role);
    object.text("name", parts.name);
    if let Some(enabled) = parts.enabled {
        object.flag("enabled", enabled);
    }
    if let Some(checked) = parts.checked {
        object.flag("checked", checked);
    }
    if let Some(value) = parts.value {
        object.text("value", value);
    }
    object.flag("focused", parts.focused);
    let bounds = object.member("bounds");
    bounds.push('[');
    for (index, bound) in parts.bounds.into_iter().enumerate() {
        if index > 0 {
            bounds.push(',');
        }
        number(bounds, bound);
    }
    bounds.push(']');
    object.end()
}

/// A list of already written values.
fn list(items: impl Iterator<Item = String>) -> String {
    let mut text = String::from("[");
    for (index, item) in items.enumerate() {
        if index > 0 {
            text.push(',');
        }
        text.push_str(&item);
    }
    text.push(']');
    text
}

fn nodes(nodes: &[DiffNode]) -> String {
    list(nodes.iter().map(|read| {
        // The role in the spelling the snapshot's text writes.
        let role = format!("{:?}", read.role);
        node(&NodeParts {
            id: &read.id,
            parent: read.parent.as_deref(),
            role: &role,
            name: &read.name,
            enabled: read.state.enabled,
            checked: read.state.checked,
            value: read.state.value.as_deref(),
            focused: read.state.focused,
            bounds: [
                read.bounds.x,
                read.bounds.y,
                read.bounds.width,
                read.bounds.height,
            ],
        })
    }))
}

/// The fields every acting verb returns, in the schema's order.
fn acted(settled: bool, busy: Option<Busy>, diff: &SnapshotDiff) -> Object {
    let mut object = Object::new();
    object.flag("settled", settled);
    if let Some(busy) = busy {
        object.text("busy", busy_word(busy));
    }
    object.json("added", &nodes(&diff.added));
    object.json(
        "removed",
        &list(diff.removed.iter().map(|id| {
            let mut text = String::new();
            string(&mut text, id);
            text
        })),
    );
    object.json("changed", &nodes(&diff.changed));
    object
}

impl Outcome {
    /// The outcome as one line of JSON: an object keyed by the result fields of its verb, in
    /// the schema's order, each node an object keyed by the schema's node fields. A field that
    /// is not always present is left out when it has no reading.
    ///
    /// The line holds what the screen reads — ids, names and values. It is an answer to the
    /// caller: never a log line.
    pub fn to_json(&self) -> String {
        match self {
            Outcome::Screen { text } => {
                let mut object = Object::new();
                object.text("text", text);
                object.end()
            }
            Outcome::Acted {
                settled,
                busy,
                diff,
            } => acted(*settled, *busy, diff).end(),
            Outcome::Advanced {
                settled,
                busy,
                diff,
                advanced_ms,
            } => {
                let mut object = acted(*settled, *busy, diff);
                object.count("advanced_ms", u64::from(*advanced_ms));
                object.end()
            }
            Outcome::Scrolled {
                settled,
                busy,
                diff,
                in_view,
            } => {
                let mut object = acted(*settled, *busy, diff);
                object.flag("in_view", *in_view);
                object.end()
            }
        }
    }
}

impl Refusal {
    /// The refusal as one line of JSON: `refused`, holding the cause's name, its meaning and
    /// its remedy, and for a malformed call the rule it broke. Every text is a fixed string of
    /// the schema: the line holds nothing of the call.
    pub fn to_json(&self) -> String {
        let cause = self.cause();
        let mut refused = Object::new();
        refused.text("cause", cause.name());
        refused.text("meaning", cause.meaning());
        refused.text("remedy", cause.remedy());
        if let Some(fault) = self.fault() {
            refused.text("fault", &fault.to_string());
        }
        let mut object = Object::new();
        object.json("refused", &refused.end());
        object.end()
    }
}

impl SessionError {
    /// The error as one line of JSON: `error`, holding the error's kind and its fixed message.
    pub fn to_json(&self) -> String {
        let mut error = Object::new();
        error.text("kind", self.kind());
        error.text("message", &self.to_string());
        let mut object = Object::new();
        object.json("error", &error.end());
        object.end()
    }
}

/// What `start` answers: the session's label, its host's process id and the idle expiry.
pub(crate) fn started(hello: &Hello) -> String {
    let mut object = Object::new();
    object.text("label", &hello.label);
    object.count("pid", u64::from(hello.pid));
    object.count("idle_expiry_s", hello.idle_expiry_s);
    object.end()
}

/// What `status` answers: `start`'s fields and the count of requests answered before this one.
pub(crate) fn status(hello: &Hello) -> String {
    let mut object = Object::new();
    object.text("label", &hello.label);
    object.count("pid", u64::from(hello.pid));
    object.count("served", hello.served);
    object.count("idle_expiry_s", hello.idle_expiry_s);
    object.end()
}

/// What `stop` answers.
pub(crate) fn stopped() -> String {
    let mut object = Object::new();
    object.flag("stopped", true);
    object.end()
}

#[cfg(test)]
mod tests {
    use std::io;

    use dioxus_native_dom::MASKED_VALUE;

    use super::*;
    use crate::refusal::{CAUSES, Cause, Fault};
    use crate::schema::{self, NODE_FIELDS, VerbSpec};

    fn written(text: &str) -> String {
        let mut out = String::new();
        string(&mut out, text);
        out
    }

    /// The keys of an object's own members, in order, read off its written form.
    fn keys(json: &str) -> Vec<String> {
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
                '}' | ']' => depth -= 1,
                ':' if depth == 1 => keys.extend(last_string.take()),
                _ => {}
            }
        }
        keys
    }

    fn field_names(verb: &VerbSpec, without: &[&str]) -> Vec<String> {
        verb.fields
            .iter()
            .map(|field| field.name)
            .filter(|name| !without.contains(name))
            .map(str::to_string)
            .collect()
    }

    fn no_diff() -> SnapshotDiff {
        SnapshotDiff {
            added: Vec::new(),
            removed: Vec::new(),
            changed: Vec::new(),
        }
    }

    #[test]
    fn a_string_is_written_with_its_escapes() {
        let rows = [
            ("", r#""""#),
            ("save", r#""save""#),
            ("a\"b", r#""a\"b""#),
            ("a\\b", r#""a\\b""#),
            ("a\nb", r#""a\nb""#),
            ("a\rb", r#""a\rb""#),
            ("a\tb", r#""a\tb""#),
            ("a\u{0}b", r#""a\u0000b""#),
            ("a\u{1b}b", r#""a\u001bb""#),
            ("a\u{1f}b", r#""a\u001fb""#),
            ("a b", r#""a b""#),
            ("form//input[2]:x", r#""form//input[2]:x""#),
            ("é ✓ 日本", "\"é ✓ 日本\""),
            ("\u{7f}", "\"\u{7f}\""),
            ("'", r#""'""#),
        ];
        assert_eq!(rows.len(), 15);
        for (row, (text, json)) in rows.into_iter().enumerate() {
            assert!(written(text) == json, "row {row}");
        }
        for code in 0..0x20u32 {
            let control = char::from_u32(code).expect("a control character");
            let json = written(&control.to_string());
            assert!(json.is_ascii() && !json.chars().any(char::is_control));
        }
    }

    #[test]
    fn a_number_is_the_shortest_decimal_and_null_when_not_finite() {
        let rows = [
            (0.0, "0"),
            (24.0, "24"),
            (112.796875, "112.796875"),
            (-269.0, "-269"),
            (64.5, "64.5"),
            (0.1, "0.1"),
            (1e21, "1000000000000000000000"),
            (f64::NAN, "null"),
            (f64::INFINITY, "null"),
            (f64::NEG_INFINITY, "null"),
        ];
        assert_eq!(rows.len(), 10);
        for (row, (value, json)) in rows.into_iter().enumerate() {
            let mut out = String::new();
            number(&mut out, value);
            assert!(out == json, "row {row}");
        }
    }

    #[test]
    fn a_node_is_written_with_the_schema_fields_it_has() {
        let full = node(&NodeParts {
            id: "who",
            parent: Some("form"),
            role: "TextInput",
            name: "Name:",
            enabled: Some(true),
            checked: Some(false),
            value: Some("Ada"),
            focused: true,
            bounds: [8.0, 8.5, 200.0, 24.0],
        });
        assert_eq!(
            full,
            r#"{"id":"who","parent":"form","role":"TextInput","name":"Name:","enabled":true,"checked":false,"value":"Ada","focused":true,"bounds":[8,8.5,200,24]}"#
        );
        assert_eq!(keys(&full), NODE_FIELDS);

        let bare = node(&NodeParts {
            id: "root",
            parent: None,
            role: "GenericContainer",
            name: "",
            enabled: None,
            checked: None,
            value: None,
            focused: false,
            bounds: [0.0, 0.0, f64::NAN, 600.0],
        });
        assert_eq!(
            bare,
            r#"{"id":"root","role":"GenericContainer","name":"","focused":false,"bounds":[0,0,null,600]}"#
        );
        assert_eq!(keys(&bare), ["id", "role", "name", "focused", "bounds"]);

        // A masked control's value is the mask: the writer writes what the snapshot reads.
        let masked = node(&NodeParts {
            id: "secret",
            parent: None,
            role: "PasswordInput",
            name: "",
            enabled: Some(true),
            checked: None,
            value: Some(MASKED_VALUE),
            focused: true,
            bounds: [0.0, 0.0, 1.0, 1.0],
        });
        assert!(masked.contains(&format!(r#""value":"{MASKED_VALUE}""#)));
        assert_eq!(
            list([full.clone(), bare.clone()].into_iter()),
            format!("[{full},{bare}]")
        );
        assert_eq!(list(std::iter::empty()), "[]");
    }

    #[test]
    fn each_outcome_is_written_with_its_verbs_fields_in_order() {
        let removed = SnapshotDiff {
            removed: vec!["row-1".to_string(), "a\"b".to_string()],
            ..no_diff()
        };
        let rows = [
            (
                Outcome::Screen {
                    text: "Button \"Save\" id=\"save\" @0,0 1x1\n".to_string(),
                },
                &schema::SNAPSHOT,
                &[][..],
                r#"{"text":"Button \"Save\" id=\"save\" @0,0 1x1\n"}"#,
            ),
            (
                Outcome::Acted {
                    settled: true,
                    busy: None,
                    diff: no_diff(),
                },
                &schema::CLICK,
                &["busy"][..],
                r#"{"settled":true,"added":[],"removed":[],"changed":[]}"#,
            ),
            (
                Outcome::Acted {
                    settled: false,
                    busy: Some(Busy::Loads),
                    diff: removed.clone(),
                },
                &schema::TYPE,
                &[][..],
                r#"{"settled":false,"busy":"loads","added":[],"removed":["row-1","a\"b"],"changed":[]}"#,
            ),
            (
                Outcome::Advanced {
                    settled: true,
                    busy: None,
                    diff: no_diff(),
                    advanced_ms: 200,
                },
                &schema::ADVANCE,
                &["busy"][..],
                r#"{"settled":true,"added":[],"removed":[],"changed":[],"advanced_ms":200}"#,
            ),
            (
                Outcome::Scrolled {
                    settled: false,
                    busy: Some(Busy::Render),
                    diff: no_diff(),
                    in_view: false,
                },
                &schema::SCROLL,
                &[][..],
                r#"{"settled":false,"busy":"render","added":[],"removed":[],"changed":[],"in_view":false}"#,
            ),
        ];
        assert_eq!(rows.len(), 5);
        for (row, (outcome, verb, absent, json)) in rows.into_iter().enumerate() {
            let line = outcome.to_json();
            assert!(line == json, "row {row}");
            assert!(keys(&line) == field_names(verb, absent), "row {row}");
            assert!(!line.contains('\n'), "row {row}");
        }
        assert_eq!(
            field_names(&schema::CLICK, &[]),
            field_names(&schema::PRESS, &[])
        );
    }

    #[test]
    fn each_refusal_is_written_from_its_causes_fixed_strings() {
        let faults = [
            Fault::Missing("id"),
            Fault::Unnamed,
            Fault::Repeated("id"),
            Fault::WrongKind("ms"),
            Fault::OutOfBound("text"),
        ];
        let refusals: Vec<Refusal> = CAUSES
            .into_iter()
            .map(Refusal::new)
            .chain(faults.into_iter().map(Refusal::malformed))
            .collect();
        assert_eq!(refusals.len(), 13);
        for (row, refusal) in refusals.iter().enumerate() {
            let cause = refusal.cause();
            let mut expected = format!(
                "{{\"refused\":{{\"cause\":\"{}\",\"meaning\":\"{}\",\"remedy\":\"{}\"",
                cause.name(),
                cause.meaning(),
                cause.remedy()
            );
            if let Some(fault) = refusal.fault() {
                expected.push_str(&format!(",\"fault\":\"{fault}\""));
            }
            expected.push_str("}}");
            let line = refusal.to_json();
            assert!(line == expected, "row {row}");
            assert!(keys(&line) == ["refused"], "row {row}");
            assert!(!line.contains('\n') && !line.contains('/'), "row {row}");
        }
        // The limit measured for `covered` is in every `covered` refusal an agent is handed.
        let covered = Refusal::new(Cause::Covered).to_json();
        assert!(covered.contains(
            "a hit reaches content scrolled out of a scrolling box, so an element lying where \
             such content extends can read `covered` though nothing shows over it"
        ));
    }

    #[test]
    fn each_session_error_is_written_with_its_kind_and_its_message() {
        let rows = [
            (SessionError::AlreadyRunning, "already-running"),
            (SessionError::NoSession, "no-session"),
            (SessionError::Dead, "dead"),
            (SessionError::HostExited(Some(101)), "host-exited"),
            (SessionError::Timeout, "timeout"),
            (SessionError::Protocol, "protocol"),
            (SessionError::InvalidLabel, "invalid-label"),
            (SessionError::StateDirTooLong, "state-dir-too-long"),
            (SessionError::StateDirNotPrivate, "state-dir-not-private"),
            (SessionError::Unsupported, "unsupported"),
            (SessionError::Io(io::ErrorKind::NotFound), "io"),
            (SessionError::NotSettled(Busy::Layout), "not-settled"),
            (SessionError::AnswerTooLarge, "answer-too-large"),
        ];
        assert_eq!(rows.len(), 13);
        for (row, (error, kind)) in rows.into_iter().enumerate() {
            let line = error.to_json();
            let expected = format!("{{\"error\":{{\"kind\":\"{kind}\",\"message\":\"{error}\"}}}}");
            assert!(line == expected, "row {row}");
            assert!(!line.contains('\n') && !line.contains('/'), "row {row}");
            for (other, (_, later)) in rows.iter().enumerate().skip(row + 1) {
                assert!(kind != *later, "rows {row} and {other}");
            }
        }
    }

    #[test]
    fn each_session_answer_is_written_with_its_verbs_fields_in_order() {
        let hello = Hello {
            pid: 4242,
            label: "flight-booker".to_string(),
            served: 7,
            idle_expiry_s: 1800,
        };
        let rows = [
            (
                started(&hello),
                &schema::START,
                r#"{"label":"flight-booker","pid":4242,"idle_expiry_s":1800}"#,
            ),
            (
                status(&hello),
                &schema::STATUS,
                r#"{"label":"flight-booker","pid":4242,"served":7,"idle_expiry_s":1800}"#,
            ),
            (stopped(), &schema::STOP, r#"{"stopped":true}"#),
        ];
        assert_eq!(rows.len(), 3);
        for (row, (line, verb, json)) in rows.into_iter().enumerate() {
            assert!(line == json, "row {row}");
            assert!(keys(&line) == field_names(verb, &[]), "row {row}");
        }
    }
}
