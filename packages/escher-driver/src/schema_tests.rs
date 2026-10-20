//! Reads the verb table against itself: the verb names and their order, each verb's arguments
//! and result fields with their kinds and help, the node keys, and the exit statuses. A child
//! module of `schema`, so it reads the table through the module's own names and its tests keep
//! their `schema::tests` paths.

use blitz_test_harness::Busy;

use super::*;

const VERB_NAMES: [&str; 11] = [
    "snapshot",
    "click",
    "type",
    "press",
    "advance",
    "scroll",
    "screenshot",
    "start",
    "status",
    "stop",
    "help",
];

#[test]
fn the_eleven_verbs_are_named_in_order_each_at_its_level() {
    assert_eq!(VERBS.len(), 11);
    let names: Vec<&str> = VERBS.iter().map(|verb| verb.name).collect();
    assert_eq!(names, VERB_NAMES);
    let levels: Vec<Level> = VERBS.iter().map(|verb| verb.level).collect();
    let stated = [
        [Level::Instance; 7].as_slice(),
        &[Level::Session; 3],
        &[Level::Tool; 1],
    ]
    .concat();
    assert_eq!(levels, stated);
    let level_names = [
        (Level::Instance, "instance"),
        (Level::Session, "session"),
        (Level::Tool, "tool"),
    ];
    assert_eq!(level_names.len(), 3);
    for (row, (level, name)) in level_names.into_iter().enumerate() {
        assert!(level.name() == name, "row {row}");
    }
}

#[test]
fn every_verb_states_its_help_its_arguments_and_a_result() {
    assert_eq!(VERBS.len(), 11);
    let field_help = |verb: &VerbSpec, name: &str| {
        verb.fields
            .iter()
            .find(|field| field.name == name)
            .map(|field| field.help)
    };
    // The help strings that state a measured limit or a decided behaviour, word for word.
    let amended = [
        (
            field_help(&SNAPSHOT, "text"),
            "the screen as one text: one line per element, nested by indent; a line's state \
             words, each written only where the element states it, are enabled or disabled, \
             checked or unchecked, value=, selected or unselected, pressed or unpressed, \
             invalid and focused; a mixed toggle reads unpressed, and a grammar or spelling \
             error reads invalid",
        ),
        (
            field_help(&CLICK, "changed"),
            "the elements on the screen before and after the step whose role, name, state, \
             bounds or parent differ, as they read after it; `selected` and `pressed` are \
             left out where the element states neither, `pressed` is false for a mixed \
             toggle, and `invalid` is true for a grammar or spelling error",
        ),
        (
            Some(TYPE.help),
            "types text into the element an id names, replacing what the element holds, then \
             settles; an empty text clears it; on a select, the text chooses the first option \
             whose label starts with it",
        ),
        (
            Some(SCREENSHOT.help),
            "the screen, or the element an id names, as a PNG image; an element shot is \
             cropped to the part of its bounds that can be seen",
        ),
        (
            Some(CLICK.help),
            "clicks the element an id names, then settles; a click on the same point within \
             500 ms of the session's time after the last click there, `type`'s own click \
             included, is a double click, and an `advance` of at least 500 ms separates two \
             single clicks",
        ),
        (
            Some(ADVANCE.help),
            "moves the session's clock forward by `ms` milliseconds, then settles; on every \
             session it moves the engine's animations, the overlay scrollbar fade and the \
             app's own time together, and `advanced_ms` is that move",
        ),
        (
            field_help(&ADVANCE, "advanced_ms"),
            "the time the session's clock moved, in milliseconds: exactly `ms`",
        ),
    ];
    assert_eq!(amended.len(), 7);
    for (row, (stated, text)) in amended.into_iter().enumerate() {
        assert!(stated == Some(text), "row {row}");
    }
    for acting in [&CLICK, &TYPE, &PRESS, &ADVANCE, &SCROLL] {
        assert_eq!(field_help(acting, "changed"), field_help(&CLICK, "changed"));
    }
    for verb in VERBS {
        let name = verb.name;
        assert!(!verb.help.is_empty(), "{name}");
        assert!(!verb.fields.is_empty(), "{name}");
        for (index, arg) in verb.args.iter().enumerate() {
            assert!(!arg.name.is_empty(), "{name}");
            assert!(!arg.help.is_empty(), "{name} {}", arg.name);
            for other in verb.args.iter().skip(index + 1) {
                assert_ne!(arg.name, other.name, "{name}");
            }
        }
        for (index, field) in verb.fields.iter().enumerate() {
            assert!(!field.name.is_empty(), "{name}");
            assert!(!field.help.is_empty(), "{name} {}", field.name);
            for other in verb.fields.iter().skip(index + 1) {
                assert_ne!(field.name, other.name, "{name}");
            }
        }
    }
}

#[test]
fn the_settled_help_states_the_load_wait_and_the_wait_ends_before_an_answer_read() {
    let stated = format!(
        "whether the app went quiet after the step, a load in flight waited for up to {} s",
        crate::LOAD_WAIT.as_secs()
    );
    assert_eq!(SETTLED.help, stated);
    assert_eq!(crate::LOAD_WAIT.subsec_nanos(), 0);
    for acting in [&CLICK, &TYPE, &PRESS, &ADVANCE, &SCROLL] {
        assert!(acting.fields.contains(&SETTLED), "{}", acting.name);
    }
    assert!(crate::LOAD_WAIT < crate::wire::ANSWER_BOUND);
}

#[test]
fn a_verb_is_found_by_its_exact_name_only() {
    for name in VERB_NAMES {
        assert_eq!(verb(name).map(|verb| verb.name), Some(name));
    }
    let near = [
        "", "Click", "SNAPSHOT", " click", "click ", "click\n", "clic", "clicks",
    ];
    for name in near {
        assert_eq!(verb(name), None, "{name:?}");
    }
}

#[test]
fn each_verb_has_its_stated_shapes_and_the_key_list_its_twelve_names() {
    const ACTING: [&str; 5] = ["settled", "busy", "added", "removed", "changed"];
    type Args = &'static [(&'static str, ArgKind, bool)];
    let rows: [(&str, Args, Vec<&str>); 11] = [
        ("snapshot", &[], vec!["text"]),
        ("click", &[("id", ArgKind::Id, true)], ACTING.to_vec()),
        (
            "type",
            &[("id", ArgKind::Id, true), ("text", ArgKind::Text, true)],
            ACTING.to_vec(),
        ),
        (
            "press",
            &[("key", ArgKind::Key, true), ("shift", ArgKind::Flag, false)],
            ACTING.to_vec(),
        ),
        (
            "advance",
            &[("ms", ArgKind::Milliseconds, true)],
            [&ACTING[..], &["advanced_ms"]].concat(),
        ),
        (
            "scroll",
            &[("id", ArgKind::Id, true)],
            [&ACTING[..], &["in_view"]].concat(),
        ),
        (
            "screenshot",
            &[("id", ArgKind::Id, false)],
            vec!["width", "height", "png"],
        ),
        (
            "start",
            &[("app", ArgKind::Name, true)],
            vec!["label", "pid", "idle_expiry_s"],
        ),
        (
            "status",
            &[],
            vec!["label", "pid", "served", "idle_expiry_s"],
        ),
        ("stop", &[], vec!["stopped"]),
        (
            "help",
            &[],
            vec![
                "verbs",
                "node_keys",
                "key_names",
                "busy_classes",
                "causes",
                "exit_statuses",
                "session_errors",
            ],
        ),
    ];
    assert_eq!(VERBS.len(), rows.len());
    for (verb, (name, args, fields)) in VERBS.iter().zip(rows) {
        assert_eq!(verb.name, name);
        let stated: Vec<_> = verb
            .args
            .iter()
            .map(|arg| (arg.name, arg.kind, arg.required))
            .collect();
        assert_eq!(stated, args, "{name}");
        let stated: Vec<&str> = verb.fields.iter().map(|field| field.name).collect();
        assert_eq!(stated, fields, "{name}");
        for field in verb.fields {
            assert_eq!(field.always, field.name != "busy", "{name} {}", field.name);
        }
    }

    let field_kinds = [
        ("text", FieldKind::Text),
        ("settled", FieldKind::Flag),
        ("busy", FieldKind::Busy),
        ("added", FieldKind::Nodes),
        ("removed", FieldKind::Ids),
        ("changed", FieldKind::Nodes),
        ("advanced_ms", FieldKind::Milliseconds),
        ("in_view", FieldKind::Flag),
        ("label", FieldKind::Text),
        ("pid", FieldKind::Count),
        ("served", FieldKind::Count),
        ("idle_expiry_s", FieldKind::Count),
        ("stopped", FieldKind::Flag),
        ("width", FieldKind::Count),
        ("height", FieldKind::Count),
        ("png", FieldKind::Png),
        ("verbs", FieldKind::Entries),
        ("node_keys", FieldKind::Entries),
        ("key_names", FieldKind::Words),
        ("busy_classes", FieldKind::Words),
        ("causes", FieldKind::Entries),
        ("exit_statuses", FieldKind::Entries),
        ("session_errors", FieldKind::Entries),
    ];
    assert_eq!(field_kinds.len(), 23);
    for verb in VERBS {
        for field in verb.fields {
            let stated = field_kinds.iter().find(|(name, _)| *name == field.name);
            assert_eq!(stated.map(|(_, kind)| *kind), Some(field.kind));
        }
    }

    assert_eq!(
        KEY_NAMES,
        [
            "tab",
            "enter",
            "escape",
            "backspace",
            "delete",
            "space",
            "arrow-up",
            "arrow-down",
            "arrow-left",
            "arrow-right",
            "home",
            "end",
        ]
    );
    assert_eq!(BUSY_CLASSES, ["render", "layout", "loads"]);
    let classes = [Busy::Render, Busy::Layout, Busy::Loads];
    assert_eq!(
        classes.map(|busy| format!("{busy:?}").to_lowercase()),
        BUSY_CLASSES
    );

    assert_eq!(MAX_ID_BYTES, 1024);
    assert_eq!(MAX_TEXT_BYTES, 4096);
    assert_eq!(MAX_MILLISECONDS, 60_000);
    let arg_kinds = [
        (ArgKind::Id, "id", "1 to 1024 bytes of text"),
        (ArgKind::Text, "text", "0 to 4096 bytes of text"),
        (ArgKind::Key, "key", "one name of the key list"),
        (ArgKind::Flag, "flag", "true or false"),
        (
            ArgKind::Milliseconds,
            "milliseconds",
            "a whole number from 1 to 60000",
        ),
        (ArgKind::Name, "name", "1 to 32 bytes of a-z, 0-9 and -"),
    ];
    assert_eq!(arg_kinds.len(), 6);
    for (kind, name, bound) in arg_kinds {
        assert_eq!(kind.name(), name);
        assert_eq!(kind.bound(), bound, "{name}");
    }
    let field_kind_names = [
        (FieldKind::Text, "text"),
        (FieldKind::Flag, "flag"),
        (FieldKind::Milliseconds, "milliseconds"),
        (FieldKind::Count, "count"),
        (FieldKind::Busy, "busy"),
        (FieldKind::Ids, "ids"),
        (FieldKind::Nodes, "nodes"),
        (FieldKind::Words, "words"),
        (FieldKind::Entries, "entries"),
        (FieldKind::Png, "png"),
    ];
    assert_eq!(field_kind_names.len(), 10);
    for (kind, name) in field_kind_names {
        assert_eq!(kind.name(), name);
    }
}

#[test]
fn the_thirteen_node_keys_are_named_in_order_each_with_its_help() {
    let names = [
        "id", "parent", "role", "name", "enabled", "checked", "value", "selected", "pressed",
        "invalid", "focused", "bounds", "differs",
    ];
    assert_eq!(NODE_FIELDS.len(), 13);
    assert_eq!(NODE_FIELDS.map(|key| key.name), names);
    for (row, key) in NODE_FIELDS.iter().enumerate() {
        assert!(!key.help.is_empty(), "row {row}");
        assert!(!key.help.contains('/'), "row {row}");
    }
    let help = |name: &str| {
        NODE_FIELDS
            .iter()
            .find(|key| key.name == name)
            .map_or("", |key| key.help)
    };
    // The boolean model's two limits, each in its key's help; no role set and no kind.
    assert!(help("pressed").contains("a mixed toggle reads false"));
    assert!(help("invalid").contains("a grammar or spelling error reads true"));
    assert!(!help("pressed").contains("aria-checked"));
    for name in ["role", "name"] {
        assert!(
            help(name).contains("as the accessibility tree reads it"),
            "{name}"
        );
    }
    assert!(help("enabled").contains("the reading follows the attribute's presence"));
}

#[test]
fn the_four_exit_statuses_are_stated_in_status_order() {
    let rows = [
        (0, "accepted"),
        (1, "refused"),
        (2, "usage"),
        (3, "session-error"),
    ];
    assert_eq!(EXIT_STATUSES.len(), 4);
    assert_eq!(rows.len(), EXIT_STATUSES.len());
    for (row, (ending, (status, name))) in EXIT_STATUSES.iter().zip(rows).enumerate() {
        assert!(ending.status == status, "row {row}");
        assert!(ending.name == name, "row {row}");
        assert!(!ending.meaning.is_empty(), "row {row}");
    }
}
