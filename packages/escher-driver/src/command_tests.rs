//! Checks calls against the verb table: an admitted call becomes its command, an unknown verb and
//! a verb asked at another level are refused, each malformed call is refused with the rule it
//! broke and the first in order where it breaks two, and every verb of the table is reached. A
//! child module of `command`, so it reads the checker's private items and its tests keep their
//! `command::tests` paths.

use super::*;
use crate::schema::KEY_NAMES;

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
        (call("snapshot", &[]), Command::Snapshot { part: None }),
        (
            call("snapshot", &[("part", ArgValue::Number(1))]),
            Command::Snapshot { part: Some(1) },
        ),
        (
            call("snapshot", &[("part", ArgValue::Number(1000))]),
            Command::Snapshot { part: Some(1000) },
        ),
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
                ctrl: false,
            },
        ),
        (
            call(
                "press",
                &[("key", text("arrow-down")), ("ctrl", ArgValue::Flag(true))],
            ),
            Command::Press {
                key: Key::ArrowDown,
                shift: false,
                ctrl: true,
            },
        ),
        (
            call(
                "press",
                &[
                    ("ctrl", ArgValue::Flag(true)),
                    ("key", text("home")),
                    ("shift", ArgValue::Flag(true)),
                ],
            ),
            Command::Press {
                key: Key::Home,
                shift: true,
                ctrl: true,
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
                ctrl: false,
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
        (call("screenshot", &[]), Command::Screenshot { id: None }),
        (
            call("screenshot", &[("id", text(&widest_id))]),
            Command::Screenshot {
                id: Some(widest_id.clone()),
            },
        ),
        (call("lint", &[]), Command::Lint),
        (call("graph", &[]), Command::Graph { part: None }),
        (
            call("graph", &[("part", ArgValue::Number(1))]),
            Command::Graph { part: Some(1) },
        ),
        (
            call("graph", &[("part", ArgValue::Number(1000))]),
            Command::Graph { part: Some(1000) },
        ),
        (call("actions", &[]), Command::Actions),
        (
            call("layout", &[("id", text("counter-increment"))]),
            Command::Layout {
                id: "counter-increment".to_string(),
            },
        ),
        (
            call("layout", &[("id", text(&widest_id))]),
            Command::Layout {
                id: widest_id.clone(),
            },
        ),
        (
            call("canvas", &[("id", text("circle-canvas"))]),
            Command::Canvas {
                id: "circle-canvas".to_string(),
                full: false,
                part: None,
            },
        ),
        (
            call(
                "canvas",
                &[
                    ("id", text("circle-canvas")),
                    ("full", ArgValue::Flag(true)),
                ],
            ),
            Command::Canvas {
                id: "circle-canvas".to_string(),
                full: true,
                part: None,
            },
        ),
        (
            call(
                "canvas",
                &[("id", text("circle-canvas")), ("part", ArgValue::Number(2))],
            ),
            Command::Canvas {
                id: "circle-canvas".to_string(),
                full: false,
                part: Some(2),
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
            Command::Press {
                key,
                shift: false,
                ctrl: false,
            },
        ));
    }
    rows
}

#[test]
fn an_admitted_call_becomes_its_command() {
    let rows = admitted_rows();
    assert_eq!(rows.len(), 44);
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
        "", "CLICK", "Snapshot", " click", "click ", "hello", "capture", "serve",
    ];
    assert_eq!(verbs.len(), 8);
    let unknown = Err(Refusal::new(Cause::UnknownVerb));
    let unknown_to_the_session = Err(Refusal::new(Cause::UnknownVerb));
    for (row, verb) in verbs.into_iter().enumerate() {
        assert!(validate(&call(verb, &[])) == unknown, "row {row}");
        assert!(
            validate_session(&call(verb, &[])) == unknown_to_the_session,
            "row {row}"
        );
        let with_an_argument = call(verb, &[("id", text("save"))]);
        assert!(validate(&with_an_argument) == unknown, "row {row}");
        assert!(
            validate_session(&with_an_argument) == unknown_to_the_session,
            "row {row}"
        );
    }
}

fn session_rows() -> Vec<(Call, SessionCommand)> {
    let widest_app = "a".repeat(32);
    vec![
        (
            call("start", &[("app", text("counter"))]),
            SessionCommand::Start {
                app: "counter".to_string(),
            },
        ),
        (
            call("start", &[("app", text("flight-booker"))]),
            SessionCommand::Start {
                app: "flight-booker".to_string(),
            },
        ),
        (
            call("start", &[("app", text("7"))]),
            SessionCommand::Start {
                app: "7".to_string(),
            },
        ),
        (
            call("start", &[("app", text(&widest_app))]),
            SessionCommand::Start {
                app: widest_app.clone(),
            },
        ),
        (call("status", &[]), SessionCommand::Status),
        (call("stop", &[]), SessionCommand::Stop),
        (call("help", &[]), SessionCommand::Help),
    ]
}

#[test]
fn a_verb_is_checked_at_its_own_level_only() {
    let unknown = Refusal::new(Cause::UnknownVerb);

    let session = session_rows();
    assert_eq!(session.len(), 7);
    for (row, (call, command)) in session.iter().enumerate() {
        assert!(validate_session(call).as_ref() == Ok(command), "row {row}");
        assert!(validate(call) == Err(unknown), "row {row}");
    }
    // A session-level verb is refused by `validate` before its arguments are read: the
    // answer is the same whatever they are.
    let whatever = [
        call("start", &[]),
        call("start", &[("app", text("Counter"))]),
        call(
            "start",
            &[("app", text("counter")), ("app", text("counter"))],
        ),
        call("status", &[("id", text("save"))]),
        call("stop", &[("ms", ArgValue::Number(0))]),
        call("help", &[("id", text("save"))]),
    ];
    assert_eq!(whatever.len(), 6);
    for (row, call) in whatever.iter().enumerate() {
        assert!(validate(call) == Err(unknown), "row {row}");
    }

    let instance = admitted_rows();
    assert_eq!(instance.len(), 44);
    for (row, (call, command)) in instance.iter().enumerate() {
        assert!(validate(call).as_ref() == Ok(command), "row {row}");
        assert!(validate_session(call) == Err(unknown), "row {row}");
    }
}

#[test]
fn a_malformed_session_call_is_refused_with_the_rule_it_broke() {
    let app = |value: &str| call("start", &[("app", text(value))]);
    let over_long = "a".repeat(33);
    let rows = [
        (call("start", &[]), Fault::Missing("app")),
        (call("start", &[("id", text("save"))]), Fault::Unnamed),
        (call("status", &[("app", text("counter"))]), Fault::Unnamed),
        (call("stop", &[("id", text("save"))]), Fault::Unnamed),
        (call("help", &[("id", text("save"))]), Fault::Unnamed),
        (call("help", &[("", text("x"))]), Fault::Unnamed),
        (
            call("start", &[("app", text("counter")), ("app", text("timer"))]),
            Fault::Repeated("app"),
        ),
        (
            call("start", &[("app", ArgValue::Number(7))]),
            Fault::WrongKind("app"),
        ),
        (
            call("start", &[("app", ArgValue::Flag(true))]),
            Fault::WrongKind("app"),
        ),
        (app(""), Fault::OutOfBound("app")),
        (app(&over_long), Fault::OutOfBound("app")),
        (app("Counter"), Fault::OutOfBound("app")),
        (app("flight booker"), Fault::OutOfBound("app")),
        (app("a/b"), Fault::OutOfBound("app")),
        (app("é"), Fault::OutOfBound("app")),
    ];
    assert_eq!(rows.len(), 15);
    for (row, (call, fault)) in rows.into_iter().enumerate() {
        assert!(
            validate_session(&call) == Err(Refusal::malformed(fault)),
            "row {row}"
        );
    }
}

#[test]
fn a_malformed_call_is_refused_with_the_rule_it_broke() {
    let id = || ("id", text("save"));
    let flag = ArgValue::Flag(true);
    let number = ArgValue::Number(7);
    let ms = |number| call("advance", &[("ms", ArgValue::Number(number))]);
    let part = |number| call("snapshot", &[("part", ArgValue::Number(number))]);
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
        (call("lint", &[id()]), Fault::Unnamed),
        (call("graph", &[id()]), Fault::Unnamed),
        (call("actions", &[id()]), Fault::Unnamed),
        (call("layout", &[]), Fault::Missing("id")),
        (
            call("layout", &[id(), ("text", text("Ada"))]),
            Fault::Unnamed,
        ),
        (call("layout", &[id(), id()]), Fault::Repeated("id")),
        (call("canvas", &[]), Fault::Missing("id")),
        (
            call("canvas", &[("full", flag.clone())]),
            Fault::Missing("id"),
        ),
        (
            call("canvas", &[("id", text(&"i".repeat(MAX_ID_BYTES + 1)))]),
            Fault::OutOfBound("id"),
        ),
        (
            call("canvas", &[id(), ("part", ArgValue::Number(0))]),
            Fault::OutOfBound("part"),
        ),
        (
            call("canvas", &[id(), ("part", ArgValue::Number(1001))]),
            Fault::OutOfBound("part"),
        ),
        (
            call("canvas", &[id(), ("text", text("Ada"))]),
            Fault::Unnamed,
        ),
        (
            call(
                "canvas",
                &[id(), ("full", flag.clone()), ("full", flag.clone())],
            ),
            Fault::Repeated("full"),
        ),
        (
            call("layout", &[("id", text(&"i".repeat(MAX_ID_BYTES + 1)))]),
            Fault::OutOfBound("id"),
        ),
        (call("screenshot", &[("text", text("Ada"))]), Fault::Unnamed),
        (call("screenshot", &[id(), id()]), Fault::Repeated("id")),
        (
            call("screenshot", &[("id", number.clone())]),
            Fault::WrongKind("id"),
        ),
        (
            call("screenshot", &[("id", text(""))]),
            Fault::OutOfBound("id"),
        ),
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
        (part(0), Fault::OutOfBound("part")),
        (part(1001), Fault::OutOfBound("part")),
        (part(-1), Fault::OutOfBound("part")),
        (part(i64::MAX), Fault::OutOfBound("part")),
        (
            call("graph", &[("part", ArgValue::Number(0))]),
            Fault::OutOfBound("part"),
        ),
        (
            call("graph", &[("part", ArgValue::Number(1001))]),
            Fault::OutOfBound("part"),
        ),
        (
            call("graph", &[("part", text("2"))]),
            Fault::WrongKind("part"),
        ),
        (
            call("press", &[("key", text("tab")), ("ctrl", text("true"))]),
            Fault::WrongKind("ctrl"),
        ),
        (
            call(
                "press",
                &[
                    ("key", text("tab")),
                    ("ctrl", flag.clone()),
                    ("ctrl", flag.clone()),
                ],
            ),
            Fault::Repeated("ctrl"),
        ),
        (
            call("snapshot", &[("part", text("2"))]),
            Fault::WrongKind("part"),
        ),
        (
            call("snapshot", &[("part", flag.clone())]),
            Fault::WrongKind("part"),
        ),
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
    assert_eq!(rows.len(), 64);
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
    let unknown_and_malformed = call("capture", &[("id", text("")), ("id", text(""))]);
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
    for (row, (call, _)) in session_rows().iter().enumerate() {
        let Ok(command) = validate_session(call) else {
            panic!("session row {row} is admitted");
        };
        let spec = command.spec();
        assert!(spec.name == call.verb, "session row {row}");
        assert!(schema::verb(&call.verb) == Some(spec), "session row {row}");
        assert!(
            matches!(spec.level, Level::Session | Level::Tool),
            "session row {row}"
        );
        if !reached.contains(&spec.name) {
            reached.push(spec.name);
        }
    }
    let table: Vec<&str> = schema::VERBS.iter().map(|verb| verb.name).collect();
    assert_eq!(table.len(), 16);
    assert_eq!(reached, table);
    let at = |level: Level| {
        schema::VERBS
            .iter()
            .filter(|verb| verb.level == level)
            .count()
    };
    assert_eq!(
        (at(Level::Instance), at(Level::Session), at(Level::Tool)),
        (12, 3, 1)
    );
}
