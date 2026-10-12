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

/// One key of a node in a `nodes` result field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeFieldSpec {
    /// The key's name.
    pub name: &'static str,
    /// What it reads.
    pub help: &'static str,
}

/// The keys of each node in a `nodes` result field, in the order a node is written with them.
/// `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` are left out where the
/// node has no reading for them, and `differs` is written on a changed node only.
pub const NODE_FIELDS: [NodeFieldSpec; 13] = [
    NodeFieldSpec {
        name: "id",
        help: "the element's stable id, the one a call names it by",
    },
    NodeFieldSpec {
        name: "parent",
        help: "the id of the element's parent; left out for a root",
    },
    NodeFieldSpec {
        name: "role",
        help: "the element's role, as the accessibility tree reads it",
    },
    NodeFieldSpec {
        name: "name",
        help: "the element's accessible name, as the accessibility tree reads it; empty where it \
               has none",
    },
    NodeFieldSpec {
        name: "enabled",
        help: "for an element that can be disabled, whether it lacks a `disabled` attribute: the \
               reading follows the attribute's presence, whatever its value; left out for every \
               other element",
    },
    NodeFieldSpec {
        name: "checked",
        help: "for a checkbox or a radio input, whether it is checked; left out for every other \
               element",
    },
    NodeFieldSpec {
        name: "value",
        help: "for a text-entry input or a textarea, the text it holds, else its `value` \
               attribute; a password or a file input that holds a value reads a fixed mask; left \
               out for every other element",
    },
    NodeFieldSpec {
        name: "selected",
        help: "whether the element states itself selected by `aria-selected`; left out where it \
               states neither",
    },
    NodeFieldSpec {
        name: "pressed",
        help: "whether the element states itself pressed by `aria-pressed`; left out where it \
               states neither; a mixed toggle reads false",
    },
    NodeFieldSpec {
        name: "invalid",
        help: "whether the element states itself invalid by `aria-invalid`; a grammar or spelling \
               error reads true",
    },
    NodeFieldSpec {
        name: "focused",
        help: "whether the element holds the document's focus",
    },
    NodeFieldSpec {
        name: "bounds",
        help: "where the element stands on the screen: x, y, width and height, in pixels; a box \
               reads the same bounds however far its content is scrolled",
    },
    NodeFieldSpec {
        name: "differs",
        help: "on a changed node only: the keys whose reading differs from before the step, in \
               this order; a node that only moved reads bounds alone",
    },
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
    /// A list of fixed words.
    Words,
    /// A list of entries, each keyed as its field's help names.
    Entries,
    /// A PNG image, written as base64 text (RFC 4648 §4, padded).
    Png,
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
            FieldKind::Words => "words",
            FieldKind::Entries => "entries",
            FieldKind::Png => "png",
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
    /// The tool itself: the verb describes the tool and asks nothing of a session.
    Tool,
}

impl Level {
    /// The level's name.
    pub const fn name(self) -> &'static str {
        match self {
            Level::Instance => "instance",
            Level::Session => "session",
            Level::Tool => "tool",
        }
    }
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
           or parent differ, as they read after it; `selected` and `pressed` are left out where \
           the element states neither, `pressed` is false for a mixed toggle, and `invalid` is \
           true for a grammar or spelling error",
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
        help: "the screen as one text: one line per element, nested by indent; a line's state \
               words, each written only where the element states it, are enabled or disabled, \
               checked or unchecked, value=, selected or unselected, pressed or unpressed, \
               invalid and focused; a mixed toggle reads unpressed, and a grammar or spelling \
               error reads invalid",
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

pub(crate) const SCREENSHOT: VerbSpec = VerbSpec {
    name: "screenshot",
    level: Level::Instance,
    help: "the screen, or the element an id names, as a PNG image; an element shot is cropped to \
           the part of its bounds that can be seen; in a build that paints overlay scrollbars, a \
           scrollbar thumb painted within 0.7 s of a scroll fades by wall time",
    args: &[ArgSpec {
        name: "id",
        kind: ArgKind::Id,
        required: false,
        help: "the id of the element to crop to, as a snapshot lists it; the whole screen is \
               taken without it",
    }],
    fields: &[
        FieldSpec {
            name: "width",
            kind: FieldKind::Count,
            always: true,
            help: "the image's width, in pixels",
        },
        FieldSpec {
            name: "height",
            kind: FieldKind::Count,
            always: true,
            help: "the image's height, in pixels",
        },
        FieldSpec {
            name: "png",
            kind: FieldKind::Png,
            always: true,
            help: "the image: a PNG of 8-bit RGBA pixels as painted, written as base64 text",
        },
    ],
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

pub(crate) const HELP: VerbSpec = VerbSpec {
    name: "help",
    level: Level::Tool,
    help: "describes the tool itself, as one document: every verb with its arguments and its \
           result fields, the keys of a node, the key names, the busy classes, the refusal \
           causes with their remedies, the command line's exit statuses and the session errors \
           with their remedies",
    args: &[],
    fields: &[
        FieldSpec {
            name: "verbs",
            kind: FieldKind::Entries,
            always: true,
            help: "every verb, in the table's order, keyed name, level, help, args and fields; \
                   each argument keyed name, kind, bound, required and help, each result field \
                   keyed name, kind, always and help",
        },
        FieldSpec {
            name: "node_keys",
            kind: FieldKind::Entries,
            always: true,
            help: "the keys of a node in a nodes field, in the order a node is written with \
                   them, keyed name and help",
        },
        FieldSpec {
            name: "key_names",
            kind: FieldKind::Words,
            always: true,
            help: "the names a key argument admits",
        },
        FieldSpec {
            name: "busy_classes",
            kind: FieldKind::Words,
            always: true,
            help: "the classes of work a busy field names",
        },
        FieldSpec {
            name: "causes",
            kind: FieldKind::Entries,
            always: true,
            help: "every cause a refusal names, keyed name, meaning and remedy",
        },
        FieldSpec {
            name: "exit_statuses",
            kind: FieldKind::Entries,
            always: true,
            help: "the statuses a run of the command line ends with, keyed status, name and \
                   meaning",
        },
        FieldSpec {
            name: "session_errors",
            kind: FieldKind::Entries,
            always: true,
            help: "every error a session edge ends a command with, keyed kind, meaning and remedy",
        },
    ],
};

/// The verb set, in the schema's order: the seven verbs asked of the held instance, the three
/// asked of the session, then `help`, asked of the tool itself.
pub const VERBS: &[VerbSpec] = &[
    SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, START, STATUS, STOP, HELP,
];

/// One way a run of the command line ends: its status, its name and what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExitStatus {
    /// The process status.
    pub status: u8,
    /// The ending's name.
    pub name: &'static str,
    /// What the ending means and what it writes.
    pub meaning: &'static str,
}

/// The four endings of a run of the command line, in status order.
pub const EXIT_STATUSES: [ExitStatus; 4] = [
    ExitStatus {
        status: 0,
        name: "accepted",
        meaning: "the command ran: its answer on stdout as one line of JSON; a step that ran and \
                  did not go quiet is accepted, and its `settled` reads false",
    },
    ExitStatus {
        status: 1,
        name: "refused",
        meaning: "the command was refused and ran nothing: the refusal on stdout as one line of \
                  JSON, naming its cause and its remedy; a word that is no verb and an argument \
                  the verb's schema does not admit are refusals",
    },
    ExitStatus {
        status: 2,
        name: "usage",
        meaning: "the line is no command: one fixed line on stderr and nothing on stdout; no \
                  command, an argument that is not text, a `--session` that is absent where the \
                  verb needs one or is given twice, an app the binary does not boot",
    },
    ExitStatus {
        status: 3,
        name: "session-error",
        meaning: "a session edge stopped the command: the error on stdout as one line of JSON, \
                  naming its kind and its remedy, and its fixed message on stderr",
    },
];

/// The verb `name` names exactly, or `None`.
pub fn verb(name: &str) -> Option<&'static VerbSpec> {
    VERBS.iter().find(|verb| verb.name == name)
}

#[cfg(test)]
mod tests {
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
                 settles; an empty text clears it",
            ),
            (
                Some(SCREENSHOT.help),
                "the screen, or the element an id names, as a PNG image; an element shot is \
                 cropped to the part of its bounds that can be seen; in a build that paints \
                 overlay scrollbars, a scrollbar thumb painted within 0.7 s of a scroll fades by \
                 wall time",
            ),
        ];
        assert_eq!(amended.len(), 4);
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
}
