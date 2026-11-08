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

/// The highest part of a screen a `snapshot` may ask for.
pub const MAX_PART: u32 = 1000;

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
pub const NODE_FIELDS: [NodeFieldSpec; 14] = [
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
        name: "clickable",
        help: "whether the element reacts to a click: it carries a click handler of the app's \
               own and is not a disabled form control; a native activation with no app handler, \
               and a handler on an ancestor, read false",
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
           or parent differ, as they read after it, `clickable` among its state; `selected` and \
           `pressed` are left out where the element states neither, `pressed` is false for a mixed toggle, and `invalid` is \
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
    help: "reads the screen: every element with its id, role, name, state and bounds; a screen \
           whose text is over 10,000 bytes is read in parts, each a run of whole lines of at most \
           10,000 bytes, and the parts in order are the whole text; without `part`, a screen over \
           the budget answers part 1, and one inside it answers whole with no `part` or `parts`; \
           a part past the last is the empty text; one element's line longer than 10,000 bytes is \
           a part alone, longer than the budget; each part is read from the screen as it stands, \
           so a step between two reads gives parts of the new screen",
    args: &[ArgSpec {
        name: "part",
        kind: ArgKind::Part,
        required: false,
        help: "the part of the screen to read, counted from 1; a screen over the budget is read \
               in parts without it from part 1",
    }],
    fields: &[
        FieldSpec {
            name: "text",
            kind: FieldKind::Text,
            always: true,
            help: "the screen as one text, or the part of it asked for: one line per element, \
                   nested by indent, a part's lines as they read in the whole text; a line's state \
                   words, each written only where the element states it, are enabled or disabled, \
                   checked or unchecked, value=, selected or unselected, pressed or unpressed, \
                   invalid, focused and clickable; a mixed toggle reads unpressed, and a grammar \
                   or spelling error reads invalid",
        },
        FieldSpec {
            name: "part",
            kind: FieldKind::Count,
            always: false,
            help: "the part this answer holds, present when the screen is read in parts",
        },
        FieldSpec {
            name: "parts",
            kind: FieldKind::Count,
            always: false,
            help: "how many parts the screen reads as, present with `part`",
        },
    ],
};

pub(crate) const CLICK: VerbSpec = VerbSpec {
    name: "click",
    level: Level::Instance,
    help: "clicks the element an id names, then settles; a click on a select opens its list or \
           closes the one standing, and a click on a row of an open list chooses it; a click on \
           the same point within 500 ms of the session's time after the last click there, \
           `type`'s own click included, is a double click, and an `advance` of at least 500 ms \
           separates two single clicks",
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
           settles; an empty text clears it; on a select, focused with no click and leaving no \
           list open, the text chooses the first option whose label starts with it, and on a \
           `multiple` select that option alone, with no click on a row; a text no label starts, \
           or an empty one, leaves a select's choice",
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
    help: "presses one key on the focused element, then settles; on a closed select `enter` and \
           `space` open its list, and on an open one the arrows, `home` and `end` move its \
           highlight, which only a screenshot shows, `enter` or `space` chooses the highlighted \
           row and `escape` closes with no choice; on a `multiple` select the arrows, `home` and \
           `end` choose a row alone and with `shift` span to it, while a set of rows that are \
           not adjacent needs Ctrl, which this verb does not carry",
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

pub(crate) const LINT: VerbSpec = VerbSpec {
    name: "lint",
    level: Level::Instance,
    help: "reads the screen for six defects, each found on an element by its id with the \
           measured reason: overlap, an element's visible part intersecting another's, neither \
           holding the other; clipped, part of its box cut from view; truncated, part of its \
           laid-out text cut from view, a text control's value never measured; off-screen, no \
           part of its box in view; zero-size, content in a box with no width or no height; \
           contrast, its text below the contrast ratio WCAG 2.x SC 1.4.3 sets, an element that \
           reads not enabled exempt; geometry only, no hit tested, and right-to-left and \
           vertical text read as left-to-right lines",
    args: &[],
    fields: &[FieldSpec {
        name: "findings",
        kind: FieldKind::Entries,
        always: true,
        help: "every defect found, in document order, one element's in the order the help \
               names them; each keyed id, defect (overlap, clipped, truncated, off-screen, \
               zero-size or contrast) and bounds as a node's, then the reason: other, the id of \
               the element later in document order, and area, where the two intersect, for an \
               overlap; visible, the part of the box that can be seen, for a clip; text, the box \
               the laid-out lines take, and visible, the part of it that can be seen, left out \
               when none can, for a truncation; ratio, the measured ratio, and required, 3 for \
               large text, else 4.5, for contrast; each box x, y, width and height, in pixels",
    }],
};

pub(crate) const GRAPH: VerbSpec = VerbSpec {
    name: "graph",
    level: Level::Instance,
    help: "reads the screen's UI graph: every element a snapshot lists, by its id, with the \
           component that rendered it, the handlers it carries and, for each handler, the \
           registered states it writes, and every registered state with the elements whose \
           component reads it; a handler's writes are what the app declares when it registers \
           the state, shown before the handler runs, each confirmed once a run of it writes the \
           state; only states the app registers are graphed, a write no handler made (a timer, \
           a task) names no handler, and a reader is a component, so every element it rendered \
           reads the state; no state's value is read",
    args: &[],
    fields: &[
        FieldSpec {
            name: "nodes",
            kind: FieldKind::GraphNodes,
            always: true,
            help: "one entry per element a snapshot lists, in its order, keyed id; owner, the id \
                   of the element whose handler serves it — a select's for its options and \
                   groups — left out where it is its own; component, the name of the component \
                   that rendered it, left out outside every component the app wrote; handlers, \
                   one per handler it carries now, keyed event and writes, each write keyed \
                   state, declared, whether the state's declaration names the handler, \
                   confirmed, whether a run of it wrote the state, and flag, left out where the \
                   two agree: undeclared for a write no declaration names, unconfirmed for a \
                   declared write once the handler has run and no run of it wrote the state; and \
                   reads, the states its component reads",
        },
        FieldSpec {
            name: "states",
            kind: FieldKind::GraphStates,
            always: true,
            help: "one entry per registered state, in the order the app registered them, keyed \
                   name, a later registration of a name its component already registered \
                   reading name:k; component, left out for the app root; declared, the \
                   handlers the app declares write it, each keyed id, an element's id or a \
                   prefix ending in *, and event; writers, one per element whose handler a run \
                   confirmed writing it or whose write is flagged, keyed id, event, declared, \
                   confirmed and flag as a node's write, a declared write no run confirmed being \
                   in declared and on its node alone; and readers, the names of the components that read \
                   it, each once: a state's reading elements are the nodes whose component it \
                   names, each of which names the state among its reads; a state only the app \
                   root reads is named in its nodes' reads alone",
        },
    ],
};

pub(crate) const ACTIONS: VerbSpec = VerbSpec {
    name: "actions",
    level: Level::Instance,
    help: "reads what can be done on the screen now: every actionable element — focusable, of \
           an interactive role, carrying a handler now, or served by an owner's handler — by \
           its id, with its owner, whether its id is an author key, and each verb a call can \
           name it by, the table's instance verbs that take an id; a verb with no cause would \
           be accepted now, and one with a cause would be refused with it; accepted is not \
           acting: a type on a button is accepted, a click in the scrollbar band of a box just \
           scrolled is accepted and dispatches nothing, and press and advance act on focus and \
           the clock, so no element lists them; it runs no step and changes nothing",
    args: &[],
    fields: &[FieldSpec {
        name: "nodes",
        kind: FieldKind::ActionNodes,
        always: true,
        help: "one entry per actionable element, in the snapshot's order, keyed id; owner, the \
               id of the element whose handler serves it — a select's for its options and \
               groups — left out where it is its own; keyed, whether its id is an author key \
               rather than a path; and verbs, one per verb a call can name it by, in the \
               table's order, keyed verb and cause, the cause a call would be refused with now, \
               left out where the call would be accepted",
    }],
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

/// The verb set, in the schema's order: the ten verbs asked of the held instance, the three
/// asked of the session, then `help`, asked of the tool itself.
pub const VERBS: &[VerbSpec] = &[
    SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS,
    STOP, HELP,
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
