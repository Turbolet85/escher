//! The shapes the verb table is written in: argument and result-field kinds, an argument, a
//! result field, a verb's level and the verb itself.

/// The kind of an argument's value, each with the bound it states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    /// An element id: 1 to [`MAX_ID_BYTES`](super::MAX_ID_BYTES) bytes of text.
    Id,
    /// Text of 0 to [`MAX_TEXT_BYTES`](super::MAX_TEXT_BYTES) bytes.
    Text,
    /// One name of [`KEY_NAMES`](super::KEY_NAMES).
    Key,
    /// True or false.
    Flag,
    /// A whole number of milliseconds, 1 to [`MAX_MILLISECONDS`](super::MAX_MILLISECONDS).
    Milliseconds,
    /// A name: 1 to 32 bytes of `a-z`, `0-9` and `-`.
    Name,
    /// A part of a screen read in parts, counted from 1: a whole number from 1 to
    /// [`MAX_PART`](super::MAX_PART).
    Part,
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
            ArgKind::Part => "part",
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
            ArgKind::Part => "a whole number from 1 to 1000",
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
    /// One name of [`BUSY_CLASSES`](super::BUSY_CLASSES).
    Busy,
    /// A list of element ids.
    Ids,
    /// A list of nodes, each with the fields [`NODE_FIELDS`](super::NODE_FIELDS) names.
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
