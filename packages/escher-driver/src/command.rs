//! A call checked against the schema: a typed command, or a refusal, before anything runs.

use serde_json::{Map, Value};

use crate::refusal::{Cause, Fault, Refusal};
use crate::schema::{
    self, ArgKind, ArgSpec, Level, MAX_ID_BYTES, MAX_MILLISECONDS, MAX_PART, MAX_TEXT_BYTES,
    VerbSpec,
};
use crate::session::valid_label;

/// The value a call passes for one argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgValue {
    /// Text.
    Text(String),
    /// A whole number.
    Number(i64),
    /// True or false.
    Flag(bool),
}

/// What a caller asks of the driver, as it arrived: a verb's name and its named arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The verb's name.
    pub verb: String,
    /// The arguments, each a name and a value, in the order they were passed.
    pub args: Vec<(String, ArgValue)>,
}

/// The call JSON arguments spell for `verb`, as an MCP tool call or a kept check carries them:
/// one argument per member. A value that is neither text, a whole number nor a flag refuses the
/// call as malformed.
pub(crate) fn call_of(
    verb: &'static VerbSpec,
    arguments: &Map<String, Value>,
) -> Result<Call, Refusal> {
    let mut args = Vec::with_capacity(arguments.len());
    for (name, value) in arguments {
        let value = match value {
            Value::String(text) => Some(ArgValue::Text(text.clone())),
            Value::Bool(flag) => Some(ArgValue::Flag(*flag)),
            Value::Number(number) => number.as_i64().map(ArgValue::Number),
            Value::Null | Value::Array(_) | Value::Object(_) => None,
        };
        let Some(value) = value else {
            let fault = match verb.args.iter().find(|arg| arg.name == name) {
                Some(arg) => Fault::WrongKind(arg.name),
                None => Fault::Unnamed,
            };
            return Err(Refusal::malformed(fault));
        };
        args.push((name.clone(), value));
    }
    Ok(Call {
        verb: verb.name.to_string(),
        args,
    })
}

mod key;

pub use key::Key;

/// A call the schema admits, asked of the held instance: one variant per instance-level verb,
/// holding its checked arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Read the screen, or one part of a screen read in parts.
    Snapshot {
        /// The part, counted from 1; `None` for the whole screen, or its first part when it is
        /// read in parts.
        part: Option<u32>,
    },
    /// Click the element `id` names.
    Click {
        /// The element's id.
        id: String,
    },
    /// Type `text` into the element `id` names.
    Type {
        /// The element's id.
        id: String,
        /// The text to type.
        text: String,
    },
    /// Press `key`.
    Press {
        /// The key.
        key: Key,
        /// Whether shift is held.
        shift: bool,
        /// Whether the host's action key is held: Meta on macOS, Ctrl elsewhere.
        ctrl: bool,
    },
    /// Move the app's time forward by `ms` milliseconds.
    Advance {
        /// How far, in milliseconds.
        ms: u32,
    },
    /// Bring the element `id` names into view.
    Scroll {
        /// The element's id.
        id: String,
    },
    /// Take an image of the screen, or of the element `id` names.
    Screenshot {
        /// The element's id; `None` for the whole screen.
        id: Option<String>,
    },
    /// Read the screen for its visual defects.
    Lint,
    /// Read the screen's UI graph, or one part of a graph read in parts.
    Graph {
        /// The part, counted from 1; `None` for the whole graph, or its first part when it is
        /// read in parts.
        part: Option<u32>,
    },
    /// Read what each actionable element of the screen can take now.
    Actions,
    /// Read why the element `id` names sits where it does at its size.
    Layout {
        /// The element's id.
        id: String,
    },
    /// Read the draw commands the widget of the element `id` names paints: a summary, the full
    /// dump, or one part of a dump read in parts.
    Canvas {
        /// The element's id.
        id: String,
        /// Whether the full dump is asked rather than the summary.
        full: bool,
        /// The part of the full dump, counted from 1; `None` for the summary, or the whole dump —
        /// its first part when it is read in parts — with `full`.
        part: Option<u32>,
    },
}

impl Command {
    /// The schema of the command's verb.
    pub const fn spec(&self) -> &'static VerbSpec {
        match self {
            Command::Snapshot { .. } => &schema::SNAPSHOT,
            Command::Click { .. } => &schema::CLICK,
            Command::Type { .. } => &schema::TYPE,
            Command::Press { .. } => &schema::PRESS,
            Command::Advance { .. } => &schema::ADVANCE,
            Command::Scroll { .. } => &schema::SCROLL,
            Command::Screenshot { .. } => &schema::SCREENSHOT,
            Command::Lint => &schema::LINT,
            Command::Graph { .. } => &schema::GRAPH,
            Command::Actions => &schema::ACTIONS,
            Command::Layout { .. } => &schema::LAYOUT,
            Command::Canvas { .. } => &schema::CANVAS,
        }
    }
}

/// A call the schema admits, asked of the session itself or of the tool: one variant per
/// session-level verb, holding its checked arguments, and one for `help`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionCommand {
    /// Start a session on the app `app` names.
    Start {
        /// The app's name.
        app: String,
    },
    /// Reach the session.
    Status,
    /// Stop the session.
    Stop,
    /// Describe the tool itself.
    Help,
}

impl SessionCommand {
    /// The schema of the command's verb.
    pub const fn spec(&self) -> &'static VerbSpec {
        match self {
            SessionCommand::Start { .. } => &schema::START,
            SessionCommand::Status => &schema::STATUS,
            SessionCommand::Stop => &schema::STOP,
            SessionCommand::Help => &schema::HELP,
        }
    }
}

enum Admitted<'a> {
    Id(&'a str),
    Text(&'a str),
    Key(Key),
    Flag(bool),
    Milliseconds(u32),
    Name(&'a str),
    Part(u32),
}

/// Checks `call` against the schema: the [`Command`] it asks of the held instance, or why it is
/// refused.
///
/// It reads the call and the schema and nothing else, so a refused call has changed nothing.
/// Every call has one answer, the first failure in this order: the verb is looked up by its
/// exact name among the instance-level verbs — a session-level verb is not one a session runs,
/// and is refused as an unknown verb whatever its arguments; then the call's arguments, in the
/// order passed, each for a name the verb does not have and for a name passed before; then the
/// verb's arguments, in the schema's order, each for being required and absent, for a value of
/// another kind and for a value outside its bound.
pub fn validate(call: &Call) -> Result<Command, Refusal> {
    let spec = verb_at(call, &[Level::Instance])?;
    let admitted = admitted(spec, call)?;
    match (spec.name, admitted.as_slice()) {
        ("snapshot", [None]) => Ok(Command::Snapshot { part: None }),
        ("snapshot", [Some(Admitted::Part(part))]) => Ok(Command::Snapshot { part: Some(*part) }),
        ("click", [Some(Admitted::Id(id))]) => Ok(Command::Click {
            id: (*id).to_owned(),
        }),
        ("type", [Some(Admitted::Id(id)), Some(Admitted::Text(text))]) => Ok(Command::Type {
            id: (*id).to_owned(),
            text: (*text).to_owned(),
        }),
        ("press", [Some(Admitted::Key(key)), shift, ctrl]) => Ok(Command::Press {
            key: *key,
            shift: matches!(shift, Some(Admitted::Flag(true))),
            ctrl: matches!(ctrl, Some(Admitted::Flag(true))),
        }),
        ("advance", [Some(Admitted::Milliseconds(ms))]) => Ok(Command::Advance { ms: *ms }),
        ("scroll", [Some(Admitted::Id(id))]) => Ok(Command::Scroll {
            id: (*id).to_owned(),
        }),
        ("screenshot", [None]) => Ok(Command::Screenshot { id: None }),
        ("screenshot", [Some(Admitted::Id(id))]) => Ok(Command::Screenshot {
            id: Some((*id).to_owned()),
        }),
        ("lint", []) => Ok(Command::Lint),
        ("graph", [None]) => Ok(Command::Graph { part: None }),
        ("graph", [Some(Admitted::Part(part))]) => Ok(Command::Graph { part: Some(*part) }),
        ("actions", []) => Ok(Command::Actions),
        ("layout", [Some(Admitted::Id(id))]) => Ok(Command::Layout {
            id: (*id).to_owned(),
        }),
        ("canvas", [Some(Admitted::Id(id)), full, part]) => Ok(Command::Canvas {
            id: (*id).to_owned(),
            full: matches!(full, Some(Admitted::Flag(true))),
            part: match part {
                Some(Admitted::Part(part)) => Some(*part),
                _ => None,
            },
        }),
        // A verb the table lists in a shape no command has is not one this function knows.
        _ => Err(Refusal::new(Cause::UnknownVerb)),
    }
}

/// Checks `call` against the schema: the [`SessionCommand`] it asks of the session, or why it
/// is refused.
///
/// The rules and their order are [`validate`]'s, over the session-level verbs and `help`, the
/// verb asked of the tool itself: an instance-level verb handed to it is refused as an unknown
/// verb.
pub fn validate_session(call: &Call) -> Result<SessionCommand, Refusal> {
    let spec = verb_at(call, &[Level::Session, Level::Tool])?;
    let admitted = admitted(spec, call)?;
    match (spec.name, admitted.as_slice()) {
        ("start", [Some(Admitted::Name(app))]) => Ok(SessionCommand::Start {
            app: (*app).to_owned(),
        }),
        ("status", []) => Ok(SessionCommand::Status),
        ("stop", []) => Ok(SessionCommand::Stop),
        ("help", []) => Ok(SessionCommand::Help),
        // A verb the table lists in a shape no command has is not one this function knows.
        _ => Err(Refusal::new(Cause::UnknownVerb)),
    }
}

/// The row of the verb `call` names, when the table lists it at one of `levels`.
fn verb_at(call: &Call, levels: &[Level]) -> Result<&'static VerbSpec, Refusal> {
    schema::verb(&call.verb)
        .filter(|spec| levels.contains(&spec.level))
        .ok_or(Refusal::new(Cause::UnknownVerb))
}

/// The call's arguments checked against the row's, in the schema's order: `None` for an
/// optional argument the call does not pass.
fn admitted<'a>(
    spec: &'static VerbSpec,
    call: &'a Call,
) -> Result<Vec<Option<Admitted<'a>>>, Refusal> {
    let mut passed: Vec<(&'static ArgSpec, Option<&ArgValue>)> =
        spec.args.iter().map(|arg| (arg, None)).collect();
    for (name, value) in &call.args {
        let Some((arg, slot)) = passed.iter_mut().find(|(arg, _)| arg.name == name) else {
            return Err(Refusal::malformed(Fault::Unnamed));
        };
        if slot.is_some() {
            return Err(Refusal::malformed(Fault::Repeated(arg.name)));
        }
        *slot = Some(value);
    }

    let mut admitted = Vec::with_capacity(passed.len());
    for (arg, value) in passed {
        admitted.push(match value {
            Some(value) => Some(admit(arg, value)?),
            None if arg.required => return Err(Refusal::malformed(Fault::Missing(arg.name))),
            None => None,
        });
    }
    Ok(admitted)
}

fn admit<'a>(arg: &'static ArgSpec, value: &'a ArgValue) -> Result<Admitted<'a>, Refusal> {
    let out_of_bound = Refusal::malformed(Fault::OutOfBound(arg.name));
    match (arg.kind, value) {
        (ArgKind::Id, ArgValue::Text(text)) => {
            if !text.is_empty() && text.len() <= MAX_ID_BYTES {
                Ok(Admitted::Id(text))
            } else {
                Err(out_of_bound)
            }
        }
        (ArgKind::Text, ArgValue::Text(text)) => {
            if text.len() <= MAX_TEXT_BYTES {
                Ok(Admitted::Text(text))
            } else {
                Err(out_of_bound)
            }
        }
        (ArgKind::Key, ArgValue::Text(name)) => {
            Key::from_name(name).map(Admitted::Key).ok_or(out_of_bound)
        }
        (ArgKind::Flag, ArgValue::Flag(flag)) => Ok(Admitted::Flag(*flag)),
        (ArgKind::Milliseconds, ArgValue::Number(number)) => u32::try_from(*number)
            .ok()
            .filter(|ms| (1..=MAX_MILLISECONDS).contains(ms))
            .map(Admitted::Milliseconds)
            .ok_or(out_of_bound),
        (ArgKind::Part, ArgValue::Number(number)) => u32::try_from(*number)
            .ok()
            .filter(|part| (1..=MAX_PART).contains(part))
            .map(Admitted::Part)
            .ok_or(out_of_bound),
        (ArgKind::Name, ArgValue::Text(name)) => {
            if valid_label(name) {
                Ok(Admitted::Name(name))
            } else {
                Err(out_of_bound)
            }
        }
        _ => Err(Refusal::malformed(Fault::WrongKind(arg.name))),
    }
}

#[cfg(test)]
#[path = "command_tests.rs"]
mod tests;
