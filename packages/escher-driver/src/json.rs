//! The written form of what a command returns: one line of JSON per answer, written by hand
//! and keyed by the schema's own words. Output only: nothing here reads JSON.

use std::fmt::Write as _;

use crate::client::Hello;
use crate::coverage::Coverage;
use crate::error::SessionError;
use crate::execute::Outcome;
use crate::image;
use crate::refusal::Refusal;

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

/// A ratio as the shortest decimal its `f32` reads back as; `null` when not finite.
fn ratio(out: &mut String, value: f32) {
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

/// The size fields of what `screenshot` returns, in the schema's order.
fn shot(width: u32, height: u32) -> Object {
    let mut object = Object::new();
    object.count("width", u64::from(width));
    object.count("height", u64::from(height));
    object
}

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

/// What `screenshot` returns without its image: the line an MCP tool result carries beside the
/// image itself.
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
                caused,
                caused_past,
            } => caused::with_caused(diff::acted(*settled, *busy, diff), caused, caused_past),
            Outcome::Advanced {
                settled,
                busy,
                diff,
                advanced_ms,
                caused,
                caused_past,
            } => {
                let mut object = diff::acted(*settled, *busy, diff);
                object.count("advanced_ms", u64::from(*advanced_ms));
                caused::with_caused(object, caused, caused_past)
            }
            Outcome::Scrolled {
                settled,
                busy,
                diff,
                in_view,
                caused,
                caused_past,
            } => {
                let mut object = diff::acted(*settled, *busy, diff);
                object.flag("in_view", *in_view);
                caused::with_caused(object, caused, caused_past)
            }
            Outcome::Shot { width, height, png } => {
                let mut object = shot(*width, *height);
                object.text("png", &image::base64(png));
                object.end()
            }
            Outcome::Linted { findings } => {
                let mut object = Object::new();
                object.json("findings", &list(findings.iter().map(lint::finding)));
                object.end()
            }
            Outcome::Graphed { graph } => {
                let mut object = Object::new();
                object.json("nodes", &list(graph.nodes.iter().map(graph::graph_node)));
                object.json("states", &list(graph.states.iter().map(graph::graph_state)));
                object.end()
            }
            Outcome::Actions { nodes } => {
                let mut object = Object::new();
                object.json("nodes", &list(nodes.iter().map(actions::action_node)));
                object.end()
            }
            Outcome::Layout { explanation } => layout::layout_answer(explanation),
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

mod actions;
mod caused;
mod diff;
mod graph;
mod layout;
mod lint;
mod snapshot;

#[cfg(test)]
#[path = "json_tests.rs"]
mod tests;
