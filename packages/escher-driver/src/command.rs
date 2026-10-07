//! A call checked against the schema: a typed command, or a refusal, before anything runs.

use crate::refusal::{Cause, Fault, Refusal};
use crate::schema::{
    self, ArgKind, ArgSpec, KEY_NAMES, MAX_ID_BYTES, MAX_MILLISECONDS, MAX_TEXT_BYTES, VerbSpec,
};

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

/// A key a `press` names: one per name of [`KEY_NAMES`], in its order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// `tab`
    Tab,
    /// `enter`
    Enter,
    /// `escape`
    Escape,
    /// `backspace`
    Backspace,
    /// `delete`
    Delete,
    /// `space`
    Space,
    /// `arrow-up`
    ArrowUp,
    /// `arrow-down`
    ArrowDown,
    /// `arrow-left`
    ArrowLeft,
    /// `arrow-right`
    ArrowRight,
    /// `home`
    Home,
    /// `end`
    End,
}

impl Key {
    /// Every key, in the order of [`KEY_NAMES`].
    pub const ALL: [Key; 12] = [
        Key::Tab,
        Key::Enter,
        Key::Escape,
        Key::Backspace,
        Key::Delete,
        Key::Space,
        Key::ArrowUp,
        Key::ArrowDown,
        Key::ArrowLeft,
        Key::ArrowRight,
        Key::Home,
        Key::End,
    ];

    /// The key's name in [`KEY_NAMES`].
    pub const fn name(self) -> &'static str {
        match self {
            Key::Tab => "tab",
            Key::Enter => "enter",
            Key::Escape => "escape",
            Key::Backspace => "backspace",
            Key::Delete => "delete",
            Key::Space => "space",
            Key::ArrowUp => "arrow-up",
            Key::ArrowDown => "arrow-down",
            Key::ArrowLeft => "arrow-left",
            Key::ArrowRight => "arrow-right",
            Key::Home => "home",
            Key::End => "end",
        }
    }

    /// The key `name` names exactly, or `None`.
    pub fn from_name(name: &str) -> Option<Key> {
        KEY_NAMES
            .into_iter()
            .zip(Key::ALL)
            .find_map(|(listed, key)| (listed == name).then_some(key))
    }
}

/// A call the schema admits: one variant per verb, holding its checked arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Read the screen.
    Snapshot,
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
}

impl Command {
    /// The schema of the command's verb.
    pub const fn spec(&self) -> &'static VerbSpec {
        match self {
            Command::Snapshot => &schema::SNAPSHOT,
            Command::Click { .. } => &schema::CLICK,
            Command::Type { .. } => &schema::TYPE,
            Command::Press { .. } => &schema::PRESS,
            Command::Advance { .. } => &schema::ADVANCE,
            Command::Scroll { .. } => &schema::SCROLL,
        }
    }
}

enum Admitted<'a> {
    Id(&'a str),
    Text(&'a str),
    Key(Key),
    Flag(bool),
    Milliseconds(u32),
}

/// Checks `call` against the schema: the [`Command`] it asks for, or why it is refused.
///
/// It reads the call and the schema and nothing else, so a refused call has changed nothing.
/// Every call has one answer, the first failure in this order: the verb is looked up by its
/// exact name; then the call's arguments, in the order passed, each for a name the verb does
/// not have and for a name passed before; then the verb's arguments, in the schema's order,
/// each for being required and absent, for a value of another kind and for a value outside
/// its bound.
pub fn validate(call: &Call) -> Result<Command, Refusal> {
    let spec = schema::verb(&call.verb).ok_or(Refusal::new(Cause::UnknownVerb))?;

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

    match (spec.name, admitted.as_slice()) {
        ("snapshot", []) => Ok(Command::Snapshot),
        ("click", [Some(Admitted::Id(id))]) => Ok(Command::Click {
            id: (*id).to_owned(),
        }),
        ("type", [Some(Admitted::Id(id)), Some(Admitted::Text(text))]) => Ok(Command::Type {
            id: (*id).to_owned(),
            text: (*text).to_owned(),
        }),
        ("press", [Some(Admitted::Key(key)), shift]) => Ok(Command::Press {
            key: *key,
            shift: matches!(shift, Some(Admitted::Flag(true))),
        }),
        ("advance", [Some(Admitted::Milliseconds(ms))]) => Ok(Command::Advance { ms: *ms }),
        ("scroll", [Some(Admitted::Id(id))]) => Ok(Command::Scroll {
            id: (*id).to_owned(),
        }),
        // A verb the table lists in a shape no command has is not one this function knows.
        _ => Err(Refusal::new(Cause::UnknownVerb)),
    }
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
        _ => Err(Refusal::malformed(Fault::WrongKind(arg.name))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn malformed(fault: Fault) -> Result<Command, Refusal> {
        Err(Refusal::malformed(fault))
    }

    fn admitted_rows() -> Vec<(Call, Command)> {
        let widest_id = "i".repeat(MAX_ID_BYTES);
        let widest_text = "t".repeat(MAX_TEXT_BYTES);
        let mut rows = vec![
            (call("snapshot", &[]), Command::Snapshot),
            (
                call("click", &[("id", text("save"))]),
                Command::Click {
                    id: "save".to_string(),
                },
            ),
            (
                call("click", &[("id", text("i"))]),
                Command::Click {
                    id: "i".to_string(),
                },
            ),
            (
                call("click", &[("id", text(&widest_id))]),
                Command::Click {
                    id: widest_id.clone(),
                },
            ),
            (
                call("click", &[("id", text("form//input[2] é"))]),
                Command::Click {
                    id: "form//input[2] é".to_string(),
                },
            ),
            (
                call("type", &[("id", text("name")), ("text", text("Ada"))]),
                Command::Type {
                    id: "name".to_string(),
                    text: "Ada".to_string(),
                },
            ),
            (
                call("type", &[("text", text("Ada")), ("id", text("name"))]),
                Command::Type {
                    id: "name".to_string(),
                    text: "Ada".to_string(),
                },
            ),
            (
                call("type", &[("id", text("name")), ("text", text(""))]),
                Command::Type {
                    id: "name".to_string(),
                    text: String::new(),
                },
            ),
            (
                call(
                    "type",
                    &[("id", text("name")), ("text", text(&widest_text))],
                ),
                Command::Type {
                    id: "name".to_string(),
                    text: widest_text.clone(),
                },
            ),
            (
                call(
                    "press",
                    &[("key", text("tab")), ("shift", ArgValue::Flag(true))],
                ),
                Command::Press {
                    key: Key::Tab,
                    shift: true,
                },
            ),
            (
                call(
                    "press",
                    &[("shift", ArgValue::Flag(false)), ("key", text("enter"))],
                ),
                Command::Press {
                    key: Key::Enter,
                    shift: false,
                },
            ),
            (
                call("advance", &[("ms", ArgValue::Number(1))]),
                Command::Advance { ms: 1 },
            ),
            (
                call("advance", &[("ms", ArgValue::Number(100))]),
                Command::Advance { ms: 100 },
            ),
            (
                call("advance", &[("ms", ArgValue::Number(60_000))]),
                Command::Advance { ms: 60_000 },
            ),
            (
                call("scroll", &[("id", text("i"))]),
                Command::Scroll {
                    id: "i".to_string(),
                },
            ),
            (
                call("scroll", &[("id", text(&widest_id))]),
                Command::Scroll {
                    id: widest_id.clone(),
                },
            ),
        ];
        let keys = [
            ("tab", Key::Tab),
            ("enter", Key::Enter),
            ("escape", Key::Escape),
            ("backspace", Key::Backspace),
            ("delete", Key::Delete),
            ("space", Key::Space),
            ("arrow-up", Key::ArrowUp),
            ("arrow-down", Key::ArrowDown),
            ("arrow-left", Key::ArrowLeft),
            ("arrow-right", Key::ArrowRight),
            ("home", Key::Home),
            ("end", Key::End),
        ];
        for (name, key) in keys {
            rows.push((
                call("press", &[("key", text(name))]),
                Command::Press { key, shift: false },
            ));
        }
        rows
    }

    #[test]
    fn an_admitted_call_becomes_its_command() {
        let rows = admitted_rows();
        assert_eq!(rows.len(), 28);
        for (row, (call, command)) in rows.iter().enumerate() {
            assert!(validate(call).as_ref() == Ok(command), "row {row}");
        }
        assert_eq!(Key::ALL.len(), 12);
        assert_eq!(Key::ALL.map(Key::name), KEY_NAMES);
        for key in Key::ALL {
            assert_eq!(Key::from_name(key.name()), Some(key));
        }
    }

    #[test]
    fn an_unknown_verb_is_refused_by_that_name() {
        let verbs = [
            "",
            "CLICK",
            "Snapshot",
            " click",
            "click ",
            "stop",
            "hello",
            "screenshot",
        ];
        let unknown = Err(Refusal::new(Cause::UnknownVerb));
        for (row, verb) in verbs.into_iter().enumerate() {
            assert!(validate(&call(verb, &[])) == unknown, "row {row}");
            let with_an_argument = call(verb, &[("id", text("save"))]);
            assert!(validate(&with_an_argument) == unknown, "row {row}");
        }
    }

    #[test]
    fn a_malformed_call_is_refused_with_the_rule_it_broke() {
        let id = || ("id", text("save"));
        let flag = ArgValue::Flag(true);
        let number = ArgValue::Number(7);
        let ms = |number| call("advance", &[("ms", ArgValue::Number(number))]);
        let rows = [
            (call("click", &[]), Fault::Missing("id")),
            (call("type", &[("text", text("Ada"))]), Fault::Missing("id")),
            (call("type", &[id()]), Fault::Missing("text")),
            (call("press", &[]), Fault::Missing("key")),
            (
                call("press", &[("shift", flag.clone())]),
                Fault::Missing("key"),
            ),
            (call("advance", &[]), Fault::Missing("ms")),
            (call("scroll", &[]), Fault::Missing("id")),
            (call("snapshot", &[id()]), Fault::Unnamed),
            (
                call("click", &[id(), ("text", text("Ada"))]),
                Fault::Unnamed,
            ),
            (call("click", &[("ID", text("save"))]), Fault::Unnamed),
            (call("click", &[("", text("save"))]), Fault::Unnamed),
            (call("advance", &[("id", text("save"))]), Fault::Unnamed),
            (call("click", &[id(), id()]), Fault::Repeated("id")),
            (
                call(
                    "press",
                    &[
                        ("key", text("tab")),
                        ("shift", flag.clone()),
                        ("shift", flag.clone()),
                    ],
                ),
                Fault::Repeated("shift"),
            ),
            (
                call("click", &[("id", number.clone())]),
                Fault::WrongKind("id"),
            ),
            (
                call("click", &[("id", flag.clone())]),
                Fault::WrongKind("id"),
            ),
            (
                call("type", &[id(), ("text", number.clone())]),
                Fault::WrongKind("text"),
            ),
            (
                call("type", &[id(), ("text", flag.clone())]),
                Fault::WrongKind("text"),
            ),
            (
                call("press", &[("key", number.clone())]),
                Fault::WrongKind("key"),
            ),
            (
                call("press", &[("key", flag.clone())]),
                Fault::WrongKind("key"),
            ),
            (
                call("press", &[("key", text("tab")), ("shift", text("true"))]),
                Fault::WrongKind("shift"),
            ),
            (
                call("press", &[("key", text("tab")), ("shift", number.clone())]),
                Fault::WrongKind("shift"),
            ),
            (
                call("advance", &[("ms", text("100"))]),
                Fault::WrongKind("ms"),
            ),
            (
                call("advance", &[("ms", flag.clone())]),
                Fault::WrongKind("ms"),
            ),
            (call("click", &[("id", text(""))]), Fault::OutOfBound("id")),
            (
                call("click", &[("id", text(&"i".repeat(MAX_ID_BYTES + 1)))]),
                Fault::OutOfBound("id"),
            ),
            (
                call(
                    "type",
                    &[id(), ("text", text(&"t".repeat(MAX_TEXT_BYTES + 1)))],
                ),
                Fault::OutOfBound("text"),
            ),
            (ms(0), Fault::OutOfBound("ms")),
            (ms(60_001), Fault::OutOfBound("ms")),
            (ms(-1), Fault::OutOfBound("ms")),
            (ms(i64::MAX), Fault::OutOfBound("ms")),
            (ms(i64::MIN), Fault::OutOfBound("ms")),
            (
                call("press", &[("key", text("f1"))]),
                Fault::OutOfBound("key"),
            ),
            (
                call("press", &[("key", text("Tab"))]),
                Fault::OutOfBound("key"),
            ),
            (
                call("press", &[("key", text(""))]),
                Fault::OutOfBound("key"),
            ),
        ];
        assert_eq!(rows.len(), 35);
        for (row, (call, fault)) in rows.into_iter().enumerate() {
            assert!(validate(&call) == malformed(fault), "row {row}");
        }
    }

    #[test]
    fn a_call_that_breaks_two_rules_gets_the_first_in_order() {
        let rows = [
            // An unnamed argument and a missing one: the call's arguments are read first.
            (call("click", &[("target", text("save"))]), Fault::Unnamed),
            // A value outside its bound, passed twice: the call's arguments are read first.
            (
                call("click", &[("id", text("")), ("id", text("save"))]),
                Fault::Repeated("id"),
            ),
            // Two bad values: the schema's order decides, not the call's.
            (
                call("type", &[("text", ArgValue::Number(7)), ("id", text(""))]),
                Fault::OutOfBound("id"),
            ),
            // A repeat and an unnamed argument: the call's order decides.
            (
                call(
                    "press",
                    &[
                        ("key", text("tab")),
                        ("key", text("tab")),
                        ("target", text("save")),
                    ],
                ),
                Fault::Repeated("key"),
            ),
        ];
        assert_eq!(rows.len(), 4);
        for (row, (call, fault)) in rows.into_iter().enumerate() {
            assert!(validate(&call) == malformed(fault), "row {row}");
        }
        let unknown_and_malformed = call("screenshot", &[("id", text("")), ("id", text(""))]);
        assert!(validate(&unknown_and_malformed) == Err(Refusal::new(Cause::UnknownVerb)));
    }

    #[test]
    fn every_verb_of_the_table_is_reached_and_names_its_own_spec() {
        let mut reached: Vec<&str> = Vec::new();
        for (row, (call, _)) in admitted_rows().iter().enumerate() {
            let Ok(command) = validate(call) else {
                panic!("row {row} is admitted");
            };
            let spec = command.spec();
            assert!(spec.name == call.verb, "row {row}");
            assert!(schema::verb(&call.verb) == Some(spec), "row {row}");
            if !reached.contains(&spec.name) {
                reached.push(spec.name);
            }
        }
        let table: Vec<&str> = schema::VERBS.iter().map(|verb| verb.name).collect();
        assert_eq!(table.len(), 6);
        assert_eq!(reached, table);
    }
}
