//! What can be asked of the driver: the verb set, each verb with its argument shapes and its
//! result shape, stated as data.

/// The longest element id an argument admits, in bytes.
pub const MAX_ID_BYTES: usize = 1024;

/// The longest text an argument admits, in bytes.
pub const MAX_TEXT_BYTES: usize = 4096;

/// The most time one call may ask for, in milliseconds.
pub const MAX_MILLISECONDS: u32 = 60_000;

/// The names a `key` argument admits.
pub const KEY_NAMES: [&str; 12] = [
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
];

/// The classes of work a `busy` result field names.
pub const BUSY_CLASSES: [&str; 3] = ["render", "layout", "loads"];

/// The fields of each node in a `nodes` result field.
pub const NODE_FIELDS: [&str; 9] = [
    "id", "parent", "role", "name", "enabled", "checked", "value", "focused", "bounds",
];

/// The kind of an argument's value, each with the bound it states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    /// An element id: 1 to [`MAX_ID_BYTES`] bytes of text.
    Id,
    /// Text of 0 to [`MAX_TEXT_BYTES`] bytes.
    Text,
    /// One name of [`KEY_NAMES`].
    Key,
    /// True or false.
    Flag,
    /// A whole number of milliseconds, 1 to [`MAX_MILLISECONDS`].
    Milliseconds,
    /// A name: 1 to 32 bytes of `a-z`, `0-9` and `-`.
    Name,
}

impl ArgKind {
    /// The kind's name.
    pub const fn name(self) -> &'static str {
        match self {
            ArgKind::Id => "id",
            ArgKind::Text => "text",
            ArgKind::Key => "key",
            ArgKind::Flag => "flag",
            ArgKind::Milliseconds => "milliseconds",
            ArgKind::Name => "name",
        }
    }

    /// The bound a value of the kind keeps, in words.
    pub const fn bound(self) -> &'static str {
        match self {
            ArgKind::Id => "1 to 1024 bytes of text",
            ArgKind::Text => "0 to 4096 bytes of text",
            ArgKind::Key => "one name of the key list",
            ArgKind::Flag => "true or false",
            ArgKind::Milliseconds => "a whole number from 1 to 60000",
            ArgKind::Name => "1 to 32 bytes of a-z, 0-9 and -",
        }
    }
}

/// One argument of a verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgSpec {
    /// The argument's name.
    pub name: &'static str,
    /// The kind of its value.
    pub kind: ArgKind,
    /// Whether a call must pass it.
    pub required: bool,
    /// What it is for.
    pub help: &'static str,
}

/// The kind of a result field's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    /// Text.
    Text,
    /// True or false.
    Flag,
    /// A whole number of milliseconds.
    Milliseconds,
    /// A whole number.
    Count,
    /// One name of [`BUSY_CLASSES`].
    Busy,
    /// A list of element ids.
    Ids,
    /// A list of nodes, each with the fields [`NODE_FIELDS`] names.
    Nodes,
}

impl FieldKind {
    /// The kind's name.
    pub const fn name(self) -> &'static str {
        match self {
            FieldKind::Text => "text",
            FieldKind::Flag => "flag",
            FieldKind::Milliseconds => "milliseconds",
            FieldKind::Count => "count",
            FieldKind::Busy => "busy",
            FieldKind::Ids => "ids",
            FieldKind::Nodes => "nodes",
        }
    }
}

/// One field of a verb's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSpec {
    /// The field's name.
    pub name: &'static str,
    /// The kind of its value.
    pub kind: FieldKind,
    /// Whether every result carries it; a field that is not always present says when in its
    /// help.
    pub always: bool,
    /// What it reads.
    pub help: &'static str,
}

/// What a verb is asked of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// The held instance: the verb reads its screen or acts on it.
    Instance,
    /// The session itself: the verb starts it, reaches it or stops it.
    Session,
}

/// One verb: its name, what it does, the arguments it takes and the fields of its result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbSpec {
    /// The verb's name.
    pub name: &'static str,
    /// What the verb is asked of.
    pub level: Level,
    /// What the verb does.
    pub help: &'static str,
    /// Its arguments, in the schema's order.
    pub args: &'static [ArgSpec],
    /// The fields of its result, in the schema's order.
    pub fields: &'static [FieldSpec],
}

const SETTLED: FieldSpec = FieldSpec {
    name: "settled",
    kind: FieldKind::Flag,
    always: true,
    help: "whether the app went quiet after the step",
};

const BUSY: FieldSpec = FieldSpec {
    name: "busy",
    kind: FieldKind::Busy,
    always: false,
    help: "the class of work still outstanding, present only when `settled` is false; the step \
           ran and is not rolled back",
};

const ADDED: FieldSpec = FieldSpec {
    name: "added",
    kind: FieldKind::Nodes,
    always: true,
    help: "the elements on the screen after the step that were not on it before",
};

const REMOVED: FieldSpec = FieldSpec {
    name: "removed",
    kind: FieldKind::Ids,
    always: true,
    help: "the ids of the elements that were on the screen before the step and are not on it \
           after",
};

const CHANGED: FieldSpec = FieldSpec {
    name: "changed",
    kind: FieldKind::Nodes,
    always: true,
    help: "the elements on the screen before and after the step whose role, name, state, bounds \
           or parent differ, as they read after it; a box that is itself scrolled reads its bounds \
           shifted by its own scroll offset, so a step that scrolls it names it here though it has \
           not moved",
};

const ADVANCED_MS: FieldSpec = FieldSpec {
    name: "advanced_ms",
    kind: FieldKind::Milliseconds,
    always: true,
    help: "the time the app actually moved, in milliseconds: never more than `ms`",
};

const IN_VIEW: FieldSpec = FieldSpec {
    name: "in_view",
    kind: FieldKind::Flag,
    always: true,
    help: "whether the element is in view after the step; false when scrolling could not bring \
           it there, and an action on it would still be refused `off-screen`",
};

pub(crate) const SNAPSHOT: VerbSpec = VerbSpec {
    name: "snapshot",
    level: Level::Instance,
    help: "reads the screen: every element with its id, role, name, state and bounds",
    args: &[],
    fields: &[FieldSpec {
        name: "text",
        kind: FieldKind::Text,
        always: true,
        help: "the screen as one text: one line per element, nested by indent; a box that is \
               itself scrolled reads its bounds shifted by its own scroll offset, and a `click` or \
               a `type` naming it lands that far from its centre, or is refused `off-screen` while \
               it is in view",
    }],
};

pub(crate) const CLICK: VerbSpec = VerbSpec {
    name: "click",
    level: Level::Instance,
    help: "clicks the element an id names, then settles",
    args: &[ArgSpec {
        name: "id",
        kind: ArgKind::Id,
        required: true,
        help: "the id of the element to click, as a snapshot lists it",
    }],
    fields: &[SETTLED, BUSY, ADDED, REMOVED, CHANGED],
};

pub(crate) const TYPE: VerbSpec = VerbSpec {
    name: "type",
    level: Level::Instance,
    help: "types text into the element an id names, replacing what the element holds, then \
           settles; an empty text clears it",
    args: &[
        ArgSpec {
            name: "id",
            kind: ArgKind::Id,
            required: true,
            help: "the id of the element to type into, as a snapshot lists it",
        },
        ArgSpec {
            name: "text",
            kind: ArgKind::Text,
            required: true,
            help: "the text to type",
        },
    ],
    fields: &[SETTLED, BUSY, ADDED, REMOVED, CHANGED],
};

pub(crate) const PRESS: VerbSpec = VerbSpec {
    name: "press",
    level: Level::Instance,
    help: "presses one key on the focused element, then settles",
    args: &[
        ArgSpec {
            name: "key",
            kind: ArgKind::Key,
            required: true,
            help: "the key to press: one name of the key list",
        },
        ArgSpec {
            name: "shift",
            kind: ArgKind::Flag,
            required: false,
            help: "whether shift is held while the key is pressed; false when absent",
        },
    ],
    fields: &[SETTLED, BUSY, ADDED, REMOVED, CHANGED],
};

pub(crate) const ADVANCE: VerbSpec = VerbSpec {
    name: "advance",
    level: Level::Instance,
    help: "moves the app's time forward by `ms` milliseconds, then settles; an app moves time in \
           its own units, and `advanced_ms` is the time it actually moved, never more than asked",
    args: &[ArgSpec {
        name: "ms",
        kind: ArgKind::Milliseconds,
        required: true,
        help: "how far to move the app's time, in milliseconds",
    }],
    fields: &[SETTLED, BUSY, ADDED, REMOVED, CHANGED, ADVANCED_MS],
};

pub(crate) const SCROLL: VerbSpec = VerbSpec {
    name: "scroll",
    level: Level::Instance,
    help: "brings the element an id names into view, then settles",
    args: &[ArgSpec {
        name: "id",
        kind: ArgKind::Id,
        required: true,
        help: "the id of the element to bring into view, as a snapshot lists it",
    }],
    fields: &[SETTLED, BUSY, ADDED, REMOVED, CHANGED, IN_VIEW],
};

const LABEL: FieldSpec = FieldSpec {
    name: "label",
    kind: FieldKind::Text,
    always: true,
    help: "the name of the app the session holds",
};

const PID: FieldSpec = FieldSpec {
    name: "pid",
    kind: FieldKind::Count,
    always: true,
    help: "the process id of the session's host",
};

const SERVED: FieldSpec = FieldSpec {
    name: "served",
    kind: FieldKind::Count,
    always: true,
    help: "how many requests the host had answered before this one",
};

const IDLE_EXPIRY_S: FieldSpec = FieldSpec {
    name: "idle_expiry_s",
    kind: FieldKind::Count,
    always: true,
    help: "the seconds without a request after which the host stops the session by itself; \
           each answered request starts the count again",
};

pub(crate) const START: VerbSpec = VerbSpec {
    name: "start",
    level: Level::Session,
    help: "starts a session on the app a name names and answers once it is up",
    args: &[ArgSpec {
        name: "app",
        kind: ArgKind::Name,
        required: true,
        help: "the app to hold: one name of the apps the binary boots",
    }],
    fields: &[LABEL, PID, IDLE_EXPIRY_S],
};

pub(crate) const STATUS: VerbSpec = VerbSpec {
    name: "status",
    level: Level::Session,
    help: "reaches the session and says what holds it",
    args: &[],
    fields: &[LABEL, PID, SERVED, IDLE_EXPIRY_S],
};

pub(crate) const STOP: VerbSpec = VerbSpec {
    name: "stop",
    level: Level::Session,
    help: "stops the session and returns once nothing of it is left",
    args: &[],
    fields: &[FieldSpec {
        name: "stopped",
        kind: FieldKind::Flag,
        always: true,
        help: "true once the session is gone",
    }],
};

/// The verb set, in the schema's order: the six verbs asked of the held instance, then the
/// three asked of the session.
pub const VERBS: &[VerbSpec] = &[
    SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, START, STATUS, STOP,
];

/// The verb `name` names exactly, or `None`.
pub fn verb(name: &str) -> Option<&'static VerbSpec> {
    VERBS.iter().find(|verb| verb.name == name)
}

#[cfg(test)]
mod tests {
    use blitz_test_harness::Busy;

    use super::*;

    const VERB_NAMES: [&str; 9] = [
        "snapshot", "click", "type", "press", "advance", "scroll", "start", "status", "stop",
    ];

    #[test]
    fn the_nine_verbs_are_named_in_order_each_at_its_level() {
        assert_eq!(VERBS.len(), 9);
        let names: Vec<&str> = VERBS.iter().map(|verb| verb.name).collect();
        assert_eq!(names, VERB_NAMES);
        let levels: Vec<Level> = VERBS.iter().map(|verb| verb.level).collect();
        let stated = [[Level::Instance; 6].as_slice(), &[Level::Session; 3]].concat();
        assert_eq!(levels, stated);
    }

    #[test]
    fn every_verb_states_its_help_its_arguments_and_a_result() {
        assert_eq!(VERBS.len(), 9);
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
                "the screen as one text: one line per element, nested by indent; a box that is \
                 itself scrolled reads its bounds shifted by its own scroll offset, and a `click` \
                 or a `type` naming it lands that far from its centre, or is refused `off-screen` \
                 while it is in view",
            ),
            (
                field_help(&CLICK, "changed"),
                "the elements on the screen before and after the step whose role, name, state, \
                 bounds or parent differ, as they read after it; a box that is itself scrolled \
                 reads its bounds shifted by its own scroll offset, so a step that scrolls it \
                 names it here though it has not moved",
            ),
            (
                Some(TYPE.help),
                "types text into the element an id names, replacing what the element holds, then \
                 settles; an empty text clears it",
            ),
        ];
        assert_eq!(amended.len(), 3);
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
        let rows: [(&str, Args, Vec<&str>); 9] = [
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
        ];
        assert_eq!(field_kinds.len(), 13);
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
        assert_eq!(
            NODE_FIELDS,
            [
                "id", "parent", "role", "name", "enabled", "checked", "value", "focused", "bounds",
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
        ];
        assert_eq!(field_kind_names.len(), 7);
        for (kind, name) in field_kind_names {
            assert_eq!(kind.name(), name);
        }
    }
}
