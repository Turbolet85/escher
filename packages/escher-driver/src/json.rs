//! The written form of what a command returns: one line of JSON per answer, written by hand
//! and keyed by the schema's own words. Output only: nothing here reads JSON.

use std::fmt::Write as _;

use blitz_test_harness::Busy;
use dioxus_native_dom::{DiffNode, GraphNode, GraphState, Reading, SnapshotDiff, WriteFlag};

use crate::actions::ActionNode;
use crate::client::Hello;
use crate::coverage::Coverage;
use crate::error::SessionError;
use crate::execute::{Outcome, busy_word};
use crate::image;
use crate::lint::{Bounds, Finding, Reason};
use crate::refusal::Refusal;
use crate::schema::NODE_FIELDS;

/// Writes `text` as a JSON string: `"` and `\` escaped, a control character as `\n`, `\r`,
/// `\t` or `\u00XX`, everything else as it is.
pub(crate) fn string(out: &mut String, text: &str) {
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
pub(crate) struct Object {
    text: String,
    members: usize,
}

impl Object {
    pub(crate) fn new() -> Object {
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

    pub(crate) fn text(&mut self, key: &str, value: &str) {
        string(self.member(key), value);
    }

    pub(crate) fn flag(&mut self, key: &str, value: bool) {
        self.member(key)
            .push_str(if value { "true" } else { "false" });
    }

    pub(crate) fn count(&mut self, key: &str, value: u64) {
        let _ = write!(self.member(key), "{value}");
    }

    pub(crate) fn integer(&mut self, key: &str, value: i64) {
        let _ = write!(self.member(key), "{value}");
    }

    /// A member whose value is already JSON.
    pub(crate) fn json(&mut self, key: &str, value: &str) {
        self.member(key).push_str(value);
    }

    pub(crate) fn end(mut self) -> String {
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
    selected: Option<bool>,
    pressed: Option<bool>,
    invalid: bool,
    focused: bool,
    clickable: bool,
    /// x, y, width and height.
    bounds: [f64; 4],
    /// For a changed node, the readings that differ; `None` for any other node.
    differs: Option<&'a [Reading]>,
}

/// The node key a reading is written as.
fn reading_word(reading: Reading) -> &'static str {
    let index = match reading {
        Reading::Parent => 1,
        Reading::Role => 2,
        Reading::Name => 3,
        Reading::Enabled => 4,
        Reading::Checked => 5,
        Reading::Value => 6,
        Reading::Selected => 7,
        Reading::Pressed => 8,
        Reading::Invalid => 9,
        Reading::Focused => 10,
        Reading::Clickable => 11,
        Reading::Bounds => 12,
    };
    NODE_FIELDS[index].name
}

/// A node as an object with the fields the schema's node fields name, in that order. A field
/// the node has no reading for is left out, and `differs` is written for a changed node only.
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
    if let Some(selected) = parts.selected {
        object.flag("selected", selected);
    }
    if let Some(pressed) = parts.pressed {
        object.flag("pressed", pressed);
    }
    object.flag("invalid", parts.invalid);
    object.flag("focused", parts.focused);
    object.flag("clickable", parts.clickable);
    let bounds = object.member("bounds");
    bounds.push('[');
    for (index, bound) in parts.bounds.into_iter().enumerate() {
        if index > 0 {
            bounds.push(',');
        }
        number(bounds, bound);
    }
    bounds.push(']');
    if let Some(differs) = parts.differs {
        object.json(
            "differs",
            &list(differs.iter().map(|reading| {
                let mut word = String::new();
                string(&mut word, reading_word(*reading));
                word
            })),
        );
    }
    object.end()
}

/// A list of already written values.
pub(crate) fn list(items: impl Iterator<Item = String>) -> String {
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

/// The nodes of a `nodes` result field; `changed` writes each node's `differs`.
fn nodes(nodes: &[DiffNode], changed: bool) -> String {
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
            selected: read.state.selected,
            pressed: read.state.pressed,
            invalid: read.state.invalid,
            focused: read.state.focused,
            clickable: read.state.clickable,
            bounds: [
                read.bounds.x,
                read.bounds.y,
                read.bounds.width,
                read.bounds.height,
            ],
            differs: changed.then_some(read.differs.as_slice()),
        })
    }))
}

/// A list of texts.
fn texts<'a>(items: impl Iterator<Item = &'a str>) -> String {
    list(items.map(|item| {
        let mut text = String::new();
        string(&mut text, item);
        text
    }))
}

/// The two readings of a write and its flag, where there is one.
fn write_readings(object: &mut Object, declared: bool, confirmed: bool, flag: Option<WriteFlag>) {
    object.flag("declared", declared);
    object.flag("confirmed", confirmed);
    if let Some(flag) = flag {
        object.text("flag", flag.word());
    }
}

/// One node of a `graph`'s `nodes`, keyed as the field's help names.
/// One node of an `actions` answer, keyed as the field's help names.
fn action_node(node: &ActionNode) -> String {
    let mut object = Object::new();
    object.text("id", &node.id);
    if let Some(owner) = &node.owner {
        object.text("owner", owner);
    }
    object.flag("keyed", node.keyed);
    object.json(
        "verbs",
        &list(node.verbs.iter().map(|reading| {
            let mut entry = Object::new();
            entry.text("verb", reading.verb);
            if let Some(cause) = reading.cause {
                entry.text("cause", cause.name());
            }
            entry.end()
        })),
    );
    object.end()
}

fn graph_node(node: &GraphNode) -> String {
    let mut object = Object::new();
    object.text("id", &node.id);
    if let Some(owner) = &node.owner {
        object.text("owner", owner);
    }
    if let Some(component) = &node.component {
        object.text("component", component);
    }
    object.json(
        "handlers",
        &list(node.handlers.iter().map(|handler| {
            let mut entry = Object::new();
            entry.text("event", &handler.event);
            entry.json(
                "writes",
                &list(handler.writes.iter().map(|write| {
                    let mut entry = Object::new();
                    entry.text("state", &write.state);
                    write_readings(&mut entry, write.declared, write.confirmed, write.flag);
                    entry.end()
                })),
            );
            entry.end()
        })),
    );
    object.json("reads", &texts(node.reads.iter().map(String::as_str)));
    object.end()
}

/// One state of a `graph`'s `states`, keyed as the field's help names.
fn graph_state(state: &GraphState) -> String {
    let mut object = Object::new();
    object.text("name", &state.name);
    if let Some(component) = &state.component {
        object.text("component", component);
    }
    object.json(
        "declared",
        &list(state.declared.iter().map(|declared| {
            let mut entry = Object::new();
            entry.text("id", declared.id);
            entry.text("event", declared.event);
            entry.end()
        })),
    );
    object.json(
        "writers",
        &list(state.writers.iter().map(|writer| {
            let mut entry = Object::new();
            entry.text("id", &writer.id);
            entry.text("event", &writer.event);
            write_readings(&mut entry, writer.declared, writer.confirmed, writer.flag);
            entry.end()
        })),
    );
    object.json("readers", &texts(state.readers.iter().map(String::as_str)));
    object.end()
}

/// The fields every acting verb returns, in the schema's order.
fn acted(settled: bool, busy: Option<Busy>, diff: &SnapshotDiff) -> Object {
    let mut object = Object::new();
    object.flag("settled", settled);
    if let Some(busy) = busy {
        object.text("busy", busy_word(busy));
    }
    object.json("added", &nodes(&diff.added, false));
    object.json(
        "removed",
        &list(diff.removed.iter().map(|id| {
            let mut text = String::new();
            string(&mut text, id);
            text
        })),
    );
    object.json("changed", &nodes(&diff.changed, true));
    object
}

/// The size fields of what `screenshot` returns, in the schema's order.
fn shot(width: u32, height: u32) -> Object {
    let mut object = Object::new();
    object.count("width", u64::from(width));
    object.count("height", u64::from(height));
    object
}

/// A box as a JSON list of x, y, width and height, as a node's bounds are written.
fn bounds(rect: Bounds) -> String {
    let mut text = String::from("[");
    for (index, bound) in [rect.x, rect.y, rect.width, rect.height]
        .into_iter()
        .enumerate()
    {
        if index > 0 {
            text.push(',');
        }
        number(&mut text, bound);
    }
    text.push(']');
    text
}

/// A ratio as the shortest decimal its `f32` reads back as; `null` when not finite.
fn ratio(out: &mut String, value: f32) {
    if value.is_finite() {
        let _ = write!(out, "{value}");
    } else {
        out.push_str("null");
    }
}

/// One finding as an object keyed `id`, `defect`, `bounds`, then its reason's keys.
fn finding(finding: &Finding) -> String {
    let mut object = Object::new();
    object.text("id", &finding.id);
    object.text("defect", finding.defect.name());
    object.json("bounds", &bounds(finding.bounds));
    match &finding.reason {
        Reason::Overlap { other, area } => {
            object.text("other", other);
            object.json("area", &bounds(*area));
        }
        Reason::Clipped { visible } => object.json("visible", &bounds(*visible)),
        Reason::Truncated { text, visible } => {
            object.json("text", &bounds(*text));
            if let Some(visible) = visible {
                object.json("visible", &bounds(*visible));
            }
        }
        Reason::OffScreen | Reason::ZeroSize => {}
        Reason::Contrast {
            ratio: measured,
            required,
        } => {
            ratio(object.member("ratio"), *measured);
            ratio(object.member("required"), *required);
        }
    }
    object.end()
}

/// What `screenshot` returns without its image: the line an MCP tool result carries beside the
/// image itself.
impl Coverage {
    /// The reading as one line of JSON: `unexercised`, each handler keyed `id`, `owner` where
    /// there is one, and `event`, then the counts `checks` and `held`. It holds ids, owner ids
    /// and event names only: an answer to the caller, never a log line.
    pub fn to_json(&self) -> String {
        let mut object = Object::new();
        object.json(
            "unexercised",
            &list(self.unexercised.iter().map(|handler| {
                let mut entry = Object::new();
                entry.text("id", &handler.id);
                if let Some(owner) = &handler.owner {
                    entry.text("owner", owner);
                }
                entry.text("event", &handler.event);
                entry.end()
            })),
        );
        object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX));
        object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
        object.end()
    }
}

pub(crate) fn shot_without_image(width: u32, height: u32) -> String {
    shot(width, height).end()
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
            Outcome::Part { text, part, parts } => {
                let mut object = Object::new();
                object.text("text", text);
                object.count("part", u64::from(*part));
                object.count("parts", u64::from(*parts));
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
            Outcome::Shot { width, height, png } => {
                let mut object = shot(*width, *height);
                object.text("png", &image::base64(png));
                object.end()
            }
            Outcome::Linted { findings } => {
                let mut object = Object::new();
                object.json("findings", &list(findings.iter().map(finding)));
                object.end()
            }
            Outcome::Graphed { graph } => {
                let mut object = Object::new();
                object.json("nodes", &list(graph.nodes.iter().map(graph_node)));
                object.json("states", &list(graph.states.iter().map(graph_state)));
                object.end()
            }
            Outcome::Actions { nodes } => {
                let mut object = Object::new();
                object.json("nodes", &list(nodes.iter().map(action_node)));
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
    /// The error as one line of JSON: `error`, holding the error's kind, its fixed message and
    /// its remedy.
    pub fn to_json(&self) -> String {
        let mut error = Object::new();
        error.text("kind", self.kind());
        error.text("message", &self.to_string());
        error.text("remedy", self.remedy());
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

    use dioxus_native_dom::{
        DeclaredWrite, GraphHandler, GraphWrite, GraphWriter, MASKED_VALUE, UiGraph,
    };

    use super::*;
    use crate::lint::Defect;
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
            selected: Some(false),
            pressed: Some(true),
            invalid: true,
            focused: true,
            clickable: true,
            bounds: [8.0, 8.5, 200.0, 24.0],
            differs: Some(&[Reading::Value, Reading::Focused]),
        });
        assert_eq!(
            full,
            r#"{"id":"who","parent":"form","role":"TextInput","name":"Name:","enabled":true,"checked":false,"value":"Ada","selected":false,"pressed":true,"invalid":true,"focused":true,"clickable":true,"bounds":[8,8.5,200,24],"differs":["value","focused"]}"#
        );
        assert_eq!(keys(&full), NODE_FIELDS.map(|key| key.name));

        let bare = node(&NodeParts {
            id: "root",
            parent: None,
            role: "GenericContainer",
            name: "",
            enabled: None,
            checked: None,
            value: None,
            selected: None,
            pressed: None,
            invalid: false,
            focused: false,
            clickable: false,
            bounds: [0.0, 0.0, f64::NAN, 600.0],
            differs: None,
        });
        assert_eq!(
            bare,
            r#"{"id":"root","role":"GenericContainer","name":"","invalid":false,"focused":false,"clickable":false,"bounds":[0,0,null,600]}"#
        );
        assert_eq!(
            keys(&bare),
            [
                "id",
                "role",
                "name",
                "invalid",
                "focused",
                "clickable",
                "bounds"
            ]
        );

        // A masked control's value is the mask: the writer writes what the snapshot reads.
        let masked = node(&NodeParts {
            id: "secret",
            parent: None,
            role: "PasswordInput",
            name: "",
            enabled: Some(true),
            checked: None,
            value: Some(MASKED_VALUE),
            selected: None,
            pressed: None,
            invalid: false,
            focused: true,
            clickable: false,
            bounds: [0.0, 0.0, 1.0, 1.0],
            differs: None,
        });
        assert!(masked.contains(&format!(r#""value":"{MASKED_VALUE}""#)));
        assert_eq!(
            list([full.clone(), bare.clone()].into_iter()),
            format!("[{full},{bare}]")
        );
        assert_eq!(list(std::iter::empty()), "[]");

        // Each reading is written as the node key it reads.
        assert_eq!(Reading::ALL.len(), 12);
        for (row, reading) in Reading::ALL.into_iter().enumerate() {
            let word = reading_word(reading);
            assert!(word == NODE_FIELDS[row + 1].name, "row {row}");
            assert!(word == format!("{reading:?}").to_lowercase(), "row {row}");
        }
    }

    fn rect(x: f64, y: f64, width: f64, height: f64) -> Bounds {
        Bounds {
            x,
            y,
            width,
            height,
        }
    }

    /// One finding of every reason, a truncation with no visible part among them.
    fn every_reason() -> Vec<Finding> {
        let found = |id: &str, defect, bounds, reason| Finding {
            id: id.to_string(),
            defect,
            bounds,
            reason,
        };
        let (cut, gone) = (rect(0.0, 40.0, 80.0, 18.0), rect(900.0, 40.0, 80.0, 18.0));
        vec![
            found(
                "a",
                Defect::Overlap,
                rect(0.0, 0.0, 10.0, 10.0),
                Reason::Overlap {
                    other: "b".to_string(),
                    area: rect(5.0, 5.0, 5.0, 5.0),
                },
            ),
            found(
                "c",
                Defect::Clipped,
                rect(0.0, 20.0, 200.0, 18.0),
                Reason::Clipped {
                    visible: rect(0.0, 20.0, 100.0, 18.0),
                },
            ),
            found(
                "d",
                Defect::Truncated,
                cut,
                Reason::Truncated {
                    text: rect(0.0, 40.0, 120.5, 18.0),
                    visible: Some(cut),
                },
            ),
            found(
                "e",
                Defect::Truncated,
                gone,
                Reason::Truncated {
                    text: rect(900.0, 40.0, 120.5, 18.0),
                    visible: None,
                },
            ),
            found("e", Defect::OffScreen, gone, Reason::OffScreen),
            found(
                "f",
                Defect::ZeroSize,
                rect(0.0, 60.0, 0.0, 0.0),
                Reason::ZeroSize,
            ),
            found(
                "g",
                Defect::Contrast,
                rect(0.0, 80.0, 50.0, 18.0),
                Reason::Contrast {
                    ratio: 2.85,
                    required: 4.5,
                },
            ),
        ]
    }

    /// A graph holding every optional key and both flags: a select owning its option, a node
    /// outside every component, and a state the app root registered.
    /// Three actionable nodes: one with a verb accepted and one refused, one served by an
    /// owner, one whose id is a path.
    fn action_nodes() -> Vec<ActionNode> {
        use crate::actions::VerbReading;
        let reading = |verb, cause| VerbReading { verb, cause };
        vec![
            ActionNode {
                id: "pick".to_string(),
                owner: None,
                keyed: true,
                verbs: vec![
                    reading("click", None),
                    reading("type", Some(Cause::Disabled)),
                ],
            },
            ActionNode {
                id: "one".to_string(),
                owner: Some("pick".to_string()),
                keyed: true,
                verbs: vec![reading("click", Some(Cause::Covered))],
            },
            ActionNode {
                id: "/div:0".to_string(),
                owner: None,
                keyed: false,
                verbs: Vec::new(),
            },
        ]
    }

    fn a_graph() -> UiGraph {
        let write = |state: &str, declared, confirmed, flag| GraphWrite {
            state: state.to_string(),
            declared,
            confirmed,
            flag,
        };
        let writer = |id: &str, event: &str, declared, confirmed, flag| GraphWriter {
            id: id.to_string(),
            event: event.to_string(),
            declared,
            confirmed,
            flag,
        };
        let handler = |event: &str, writes| GraphHandler {
            event: event.to_string(),
            writes,
        };
        let mode = || vec!["mode".to_string()];
        let form = || Some("Form".to_string());
        UiGraph {
            nodes: vec![
                GraphNode {
                    id: "pick".to_string(),
                    owner: None,
                    component: form(),
                    handlers: vec![handler("input", vec![write("mode", true, true, None)])],
                    reads: mode(),
                },
                GraphNode {
                    id: "one".to_string(),
                    owner: Some("pick".to_string()),
                    component: form(),
                    handlers: Vec::new(),
                    reads: mode(),
                },
                GraphNode {
                    id: "go".to_string(),
                    owner: None,
                    component: None,
                    handlers: vec![handler(
                        "click",
                        vec![
                            write("mode", false, true, Some(WriteFlag::Undeclared)),
                            write("a\"b", true, false, Some(WriteFlag::Unconfirmed)),
                        ],
                    )],
                    reads: Vec::new(),
                },
            ],
            states: vec![
                GraphState {
                    name: "mode".to_string(),
                    component: form(),
                    declared: vec![DeclaredWrite::new("pick", "input")],
                    writers: vec![
                        writer("pick", "input", true, true, None),
                        writer("go", "click", false, true, Some(WriteFlag::Undeclared)),
                    ],
                    readers: vec!["Form".to_string()],
                },
                GraphState {
                    name: "a\"b".to_string(),
                    component: None,
                    declared: vec![DeclaredWrite::new("g*", "click")],
                    writers: vec![writer(
                        "go",
                        "click",
                        true,
                        false,
                        Some(WriteFlag::Unconfirmed),
                    )],
                    readers: Vec::new(),
                },
            ],
        }
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
                &["part", "parts"][..],
                r#"{"text":"Button \"Save\" id=\"save\" @0,0 1x1\n"}"#,
            ),
            (
                Outcome::Part {
                    text: "  Button \"Save\" id=\"save\" @0,0 1x1\n".to_string(),
                    part: 2,
                    parts: 3,
                },
                &schema::SNAPSHOT,
                &[][..],
                r#"{"text":"  Button \"Save\" id=\"save\" @0,0 1x1\n","part":2,"parts":3}"#,
            ),
            (
                Outcome::Part {
                    text: String::new(),
                    part: 4,
                    parts: 3,
                },
                &schema::SNAPSHOT,
                &[][..],
                r#"{"text":"","part":4,"parts":3}"#,
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
            (
                Outcome::Shot {
                    width: 800,
                    height: 600,
                    png: b"foobar".to_vec(),
                },
                &schema::SCREENSHOT,
                &[][..],
                r#"{"width":800,"height":600,"png":"Zm9vYmFy"}"#,
            ),
            (
                Outcome::Linted {
                    findings: Vec::new(),
                },
                &schema::LINT,
                &[][..],
                r#"{"findings":[]}"#,
            ),
            (
                Outcome::Linted {
                    findings: every_reason(),
                },
                &schema::LINT,
                &[][..],
                concat!(
                    r#"{"findings":[{"id":"a","defect":"overlap","bounds":[0,0,10,10],"other":"b","area":[5,5,5,5]},"#,
                    r#"{"id":"c","defect":"clipped","bounds":[0,20,200,18],"visible":[0,20,100,18]},"#,
                    r#"{"id":"d","defect":"truncated","bounds":[0,40,80,18],"text":[0,40,120.5,18],"visible":[0,40,80,18]},"#,
                    r#"{"id":"e","defect":"truncated","bounds":[900,40,80,18],"text":[900,40,120.5,18]},"#,
                    r#"{"id":"e","defect":"off-screen","bounds":[900,40,80,18]},"#,
                    r#"{"id":"f","defect":"zero-size","bounds":[0,60,0,0]},"#,
                    r#"{"id":"g","defect":"contrast","bounds":[0,80,50,18],"ratio":2.85,"required":4.5}]}"#,
                ),
            ),
            (
                Outcome::Graphed {
                    graph: UiGraph::default(),
                },
                &schema::GRAPH,
                &[][..],
                r#"{"nodes":[],"states":[]}"#,
            ),
            (
                Outcome::Graphed { graph: a_graph() },
                &schema::GRAPH,
                &[][..],
                concat!(
                    r#"{"nodes":[{"id":"pick","component":"Form","handlers":[{"event":"input","writes":[{"state":"mode","declared":true,"confirmed":true}]}],"reads":["mode"]},"#,
                    r#"{"id":"one","owner":"pick","component":"Form","handlers":[],"reads":["mode"]},"#,
                    r#"{"id":"go","handlers":[{"event":"click","writes":[{"state":"mode","declared":false,"confirmed":true,"flag":"undeclared"},"#,
                    r#"{"state":"a\"b","declared":true,"confirmed":false,"flag":"unconfirmed"}]}],"reads":[]}],"#,
                    r#""states":[{"name":"mode","component":"Form","declared":[{"id":"pick","event":"input"}],"#,
                    r#""writers":[{"id":"pick","event":"input","declared":true,"confirmed":true},"#,
                    r#"{"id":"go","event":"click","declared":false,"confirmed":true,"flag":"undeclared"}],"readers":["Form"]},"#,
                    r#"{"name":"a\"b","declared":[{"id":"g*","event":"click"}],"#,
                    r#""writers":[{"id":"go","event":"click","declared":true,"confirmed":false,"flag":"unconfirmed"}],"readers":[]}]}"#,
                ),
            ),
            (
                Outcome::Actions { nodes: Vec::new() },
                &schema::ACTIONS,
                &[][..],
                r#"{"nodes":[]}"#,
            ),
            (
                Outcome::Actions {
                    nodes: action_nodes(),
                },
                &schema::ACTIONS,
                &[][..],
                concat!(
                    r#"{"nodes":[{"id":"pick","keyed":true,"verbs":[{"verb":"click"},{"verb":"type","cause":"disabled"}]},"#,
                    r#"{"id":"one","owner":"pick","keyed":true,"verbs":[{"verb":"click","cause":"covered"}]},"#,
                    r#"{"id":"/div:0","keyed":false,"verbs":[]}]}"#,
                ),
            ),
        ];
        assert_eq!(rows.len(), 14);
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
        let without = shot_without_image(369, 420);
        assert_eq!(without, r#"{"width":369,"height":420}"#);
        assert_eq!(keys(&without), field_names(&schema::SCREENSHOT, &["png"]));
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
        // The limits measured for `covered` are in every `covered` refusal an agent is handed.
        let covered = Refusal::new(Cause::Covered).to_json();
        assert!(covered.contains(
            "an option of a closed select lies under it: click the select first, then the option; \
             a box clipped on one axis stops a hit at its whole padding box, so an element lying \
             outside it along the axis it does not clip can read `covered` though it shows; a box \
             with no width or no height is hit at its centre on the element that holds it, so it \
             can read `covered` though nothing covers it"
        ));
    }

    #[test]
    fn each_session_error_is_written_with_its_kind_its_message_and_its_remedy() {
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
            (SessionError::RecordDirNotPrivate, "record-dir-not-private"),
            (SessionError::RecordingFailed, "recording-failed"),
        ];
        assert_eq!(rows.len(), 15);
        for (row, (error, kind)) in rows.into_iter().enumerate() {
            let line = error.to_json();
            let expected = format!(
                "{{\"error\":{{\"kind\":\"{kind}\",\"message\":\"{error}\",\"remedy\":\"{}\"}}}}",
                error.remedy()
            );
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

    #[test]
    fn a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner() {
        use crate::coverage::{Coverage, HandlerRef};

        let handler = |id: &str, owner: Option<&str>, event: &str| HandlerRef {
            id: id.to_string(),
            owner: owner.map(str::to_string),
            event: event.to_string(),
        };
        let coverage = Coverage {
            unexercised: vec![
                handler("back-btn", None, "click"),
                handler("kind-a", Some("kind"), "input"),
                handler("say \"hi\"", None, "keydown"),
            ],
            checks: 3,
            held: 2,
        };
        assert_eq!(
            coverage.to_json(),
            r#"{"unexercised":[{"id":"back-btn","event":"click"},{"id":"kind-a","owner":"kind","event":"input"},{"id":"say \"hi\"","event":"keydown"}],"checks":3,"held":2}"#
        );
        assert_eq!(keys(&coverage.to_json()), ["unexercised", "checks", "held"]);
        let none = Coverage {
            unexercised: Vec::new(),
            checks: 0,
            held: 0,
        };
        assert_eq!(none.to_json(), r#"{"unexercised":[],"checks":0,"held":0}"#);
    }
}
