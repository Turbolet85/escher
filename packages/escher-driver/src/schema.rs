//! What can be asked of the driver: the verb set, each verb with its argument shapes and its
//! result shape, stated as data.

mod spec;

pub use spec::{ArgKind, ArgSpec, FieldKind, FieldSpec, Level, VerbSpec};

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
        help: "for a checkbox or a radio input, whether it is checked; else, for an element whose \
               role is checkbox, switch, radio, menuitemcheckbox or menuitemradio, whether it \
               states itself checked by `aria-checked`; left out for every other element and \
               where such a role states neither; a mixed one reads false",
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
               states neither and for the five roles whose toggle reads as `checked`; a mixed \
               toggle reads false",
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

const SETTLED: FieldSpec = FieldSpec {
    name: "settled",
    kind: FieldKind::Flag,
    always: true,
    help: "whether the app went quiet after the step, a load in flight waited for up to 5 s",
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
    help: "the time the session's clock moved, in milliseconds: exactly `ms`",
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
    help: "clicks the element an id names, then settles; a click on the same point within 500 ms \
           of the session's time after the last click there, `type`'s own click included, is a \
           double click, and an `advance` of at least 500 ms separates two single clicks",
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
           settles; an empty text clears it; on a select, the text chooses the first option \
           whose label starts with it",
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
    help: "moves the session's clock forward by `ms` milliseconds, then settles; on every session \
           it moves the engine's animations, the overlay scrollbar fade and the app's own time \
           together, and `advanced_ms` is that move",
    args: &[ArgSpec {
        name: "ms",
        kind: ArgKind::Milliseconds,
        required: true,
        help: "how far to move the session's clock, in milliseconds",
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
           the part of its bounds that can be seen",
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
#[path = "schema_tests.rs"]
mod tests;
