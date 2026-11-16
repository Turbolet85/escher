//! What an acting step caused, one item per record: each handler panic with its payload and
//! location, each error a boundary of the app took with its message and cause chain, and each
//! record the step logged — the app's at every level, the UI framework's and the engine's at
//! `warn` and above — with its source, target, message, location and fields, each tied to the
//! first handler the step's dispatches reached.
//!
//! The text is what escher-telemetry's capture kept: a field named in the sink's content set
//! reads `[redacted]`, and each text is cut at [`escher_telemetry::TEXT_CUT`] bytes with the bytes cut said. An
//! answer keeps at most [`CAUSED_KEPT`] items and counts the rest by kind and source.

use dioxus_native_dom::{Boundary, BoundaryMark, DioxusDocument, ReachedHandler};
use escher_telemetry::{Captured, KeptField, Source, Text};
use tracing::Level;

/// The items an acting answer keeps at most; the rest are counted in its `caused_past`. At most
/// 2 records crossed one measured step.
pub const CAUSED_KEPT: usize = 16;

/// The links of an error's cause chain an item keeps at most; the rest are counted. The longest
/// measured chain held 3.
pub const CHAIN_KEPT: usize = 8;

/// What an item of an acting answer's `caused` is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausedKind {
    /// A handler panicked during the step.
    Panic,
    /// A handler returned an error that reached one of the app's error boundaries.
    Error,
    /// The app, or the UI framework or the engine at `warn` or `error`, logged a record.
    Log,
}

impl CausedKind {
    /// Every kind, in the order an answer lists them.
    pub const ALL: [CausedKind; 3] = [CausedKind::Panic, CausedKind::Error, CausedKind::Log];

    /// The kind's closed word.
    pub fn name(self) -> &'static str {
        match self {
            CausedKind::Panic => "panic",
            CausedKind::Error => "error",
            CausedKind::Log => "log",
        }
    }
}

/// The level of an item of an acting answer's `caused`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausedLevel {
    /// `error`.
    Error,
    /// `warn`.
    Warn,
    /// `info`.
    Info,
    /// `debug`.
    Debug,
    /// `trace`.
    Trace,
}

impl CausedLevel {
    /// Every level, in the order an answer lists them.
    pub const ALL: [CausedLevel; 5] = [
        CausedLevel::Error,
        CausedLevel::Warn,
        CausedLevel::Info,
        CausedLevel::Debug,
        CausedLevel::Trace,
    ];

    /// The level's closed word.
    pub fn name(self) -> &'static str {
        match self {
            CausedLevel::Error => "error",
            CausedLevel::Warn => "warn",
            CausedLevel::Info => "info",
            CausedLevel::Debug => "debug",
            CausedLevel::Trace => "trace",
        }
    }

    fn of(level: Level) -> CausedLevel {
        match level {
            Level::ERROR => CausedLevel::Error,
            Level::WARN => CausedLevel::Warn,
            Level::INFO => CausedLevel::Info,
            Level::DEBUG => CausedLevel::Debug,
            Level::TRACE => CausedLevel::Trace,
        }
    }
}

/// The closed word of a boundary.
pub fn boundary_word(boundary: Boundary) -> &'static str {
    match boundary {
        Boundary::Root => "root",
        Boundary::Own => "own",
    }
}

/// One thing an acting step caused, tied to the handler it ran under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caused {
    /// What it is.
    pub kind: CausedKind,
    /// Its level: `error` for a panic and an error.
    pub level: CausedLevel,
    /// A log record's source; `None` for a panic and an error.
    pub source: Option<Source>,
    /// A log record's target; `None` for a panic and an error.
    pub target: Option<String>,
    /// A panic's payload, an error's top line or a record's message.
    pub message: Text,
    /// The file it happened in, as the compiler wrote it, where known.
    pub file: Option<String>,
    /// Its line, where known.
    pub line: Option<u32>,
    /// Its column, where known.
    pub column: Option<u32>,
    /// The boundary that took an error; `None` for any other kind.
    pub boundary: Option<Boundary>,
    /// An error's causes after its top line, at most [`CHAIN_KEPT`].
    pub chain: Vec<Text>,
    /// The causes past [`CHAIN_KEPT`].
    pub chain_past: u32,
    /// A log record's fields, a content-named one's value `[redacted]`.
    pub fields: Vec<KeptField>,
    /// A log record's fields past the bound.
    pub fields_past: u32,
    /// The stable id of the element carrying the first handler the step reached; `None` when
    /// the step reached none.
    pub id: Option<String>,
    /// That element's owner id, as the UI graph reads it; `None` where it has none.
    pub owner: Option<String>,
    /// That handler's event name, without its `on`.
    pub event: Option<String>,
}

/// The items of one kind and source past [`CAUSED_KEPT`], or past the capture's own bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CausedPast {
    /// Their kind.
    pub kind: CausedKind,
    /// Their source, for log records; `None` for panics and errors.
    pub source: Option<Source>,
    /// How many.
    pub count: u32,
}

/// What the run path saw of one step.
pub(crate) struct StepReading {
    /// The panics caught around the step and its settle.
    pub(crate) panics: u32,
    /// The marks of every error boundary before the step.
    pub(crate) marks_before: Vec<BoundaryMark>,
    /// The records and panics the step and its settle caused.
    pub(crate) captured: Captured,
    /// The handlers the step's dispatches reached, in first-reached order.
    pub(crate) reached: Vec<ReachedHandler>,
}

fn item(kind: CausedKind, level: CausedLevel, message: Text) -> Caused {
    Caused {
        kind,
        level,
        source: None,
        target: None,
        message,
        file: None,
        line: None,
        column: None,
        boundary: None,
        chain: Vec::new(),
        chain_past: 0,
        fields: Vec::new(),
        fields_past: 0,
        id: None,
        owner: None,
        event: None,
    }
}

fn add_past(past: &mut Vec<CausedPast>, kind: CausedKind, source: Option<Source>, count: u32) {
    if count == 0 {
        return;
    }
    match past
        .iter_mut()
        .find(|entry| entry.kind == kind && entry.source == source)
    {
        Some(entry) => entry.count = entry.count.saturating_add(count),
        None => past.push(CausedPast {
            kind,
            source,
            count,
        }),
    }
}

/// The items of `reading` — its panics, the errors new in a boundary since its marks, then its
/// records in record order — the first [`CAUSED_KEPT`] kept and the rest counted, each tied to
/// the first handler reached, its owner read from `doc`'s UI graph.
pub(crate) fn caused(doc: &DioxusDocument, reading: StepReading) -> (Vec<Caused>, Vec<CausedPast>) {
    let captured = &reading.captured;
    let mut items: Vec<Caused> = Vec::new();
    let mut past: Vec<CausedPast> = Vec::new();

    for panic in captured.panics() {
        let mut panicked = item(CausedKind::Panic, CausedLevel::Error, panic.payload.clone());
        panicked.file.clone_from(&panic.file);
        panicked.line = panic.line;
        panicked.column = panic.column;
        items.push(panicked);
    }
    let seen = u32::try_from(captured.panics().len())
        .unwrap_or(u32::MAX)
        .saturating_add(captured.panics_past());
    for _ in seen..reading.panics {
        items.push(item(
            CausedKind::Panic,
            CausedLevel::Error,
            Text::cut("<payload unread>".to_string()),
        ));
    }
    add_past(&mut past, CausedKind::Panic, None, captured.panics_past());

    for mark in doc.boundary_marks() {
        if reading.marks_before.contains(&mark) {
            continue;
        }
        let error = mark.read();
        let mut errored = item(
            CausedKind::Error,
            CausedLevel::Error,
            Text::cut(error.message),
        );
        errored.boundary = Some(error.boundary);
        let links = error.chain.len();
        errored.chain = error
            .chain
            .into_iter()
            .take(CHAIN_KEPT)
            .map(Text::cut)
            .collect();
        errored.chain_past = u32::try_from(links.saturating_sub(CHAIN_KEPT)).unwrap_or(u32::MAX);
        items.push(errored);
    }

    for record in captured.records() {
        let mut logged = item(
            CausedKind::Log,
            CausedLevel::of(record.level),
            record.message.clone(),
        );
        logged.source = Some(record.source);
        logged.target = Some(record.target.clone());
        logged.file.clone_from(&record.file);
        logged.line = record.line;
        logged.fields.clone_from(&record.fields);
        logged.fields_past = record.fields_past;
        items.push(logged);
    }

    for dropped in items.split_off(CAUSED_KEPT.min(items.len())) {
        add_past(&mut past, dropped.kind, dropped.source, 1);
    }
    for source in Source::ALL {
        add_past(
            &mut past,
            CausedKind::Log,
            Some(source),
            captured.records_past(source),
        );
    }

    if items.is_empty() && past.is_empty() {
        return (items, past);
    }
    let handler = reading.reached.into_iter().next();
    let owner = handler.as_ref().and_then(|handler| {
        doc.graph()
            .nodes
            .into_iter()
            .find(|node| node.id == handler.id)
            .and_then(|node| node.owner)
    });
    let (id, event) = match handler {
        Some(ReachedHandler { id, event }) => (Some(id), Some(event)),
        None => (None, None),
    };
    for entry in &mut items {
        entry.id.clone_from(&id);
        entry.owner.clone_from(&owner);
        entry.event.clone_from(&event);
    }
    (items, past)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_level_and_boundary_reads_as_its_closed_word() {
        let kinds = [
            (CausedKind::Panic, "panic"),
            (CausedKind::Error, "error"),
            (CausedKind::Log, "log"),
        ];
        assert_eq!(kinds.len(), 3);
        for (row, (kind, word)) in kinds.iter().enumerate() {
            assert!(CausedKind::ALL[row] == *kind, "row {row}");
            assert!(kind.name() == *word, "row {row}");
        }
        let levels = [
            (CausedLevel::Error, Level::ERROR, "error"),
            (CausedLevel::Warn, Level::WARN, "warn"),
            (CausedLevel::Info, Level::INFO, "info"),
            (CausedLevel::Debug, Level::DEBUG, "debug"),
            (CausedLevel::Trace, Level::TRACE, "trace"),
        ];
        assert_eq!(levels.len(), 5);
        for (row, (level, tracing_level, word)) in levels.iter().enumerate() {
            assert!(CausedLevel::ALL[row] == *level, "row {row}");
            assert!(CausedLevel::of(*tracing_level) == *level, "row {row}");
            assert!(level.name() == *word, "row {row}");
            assert!(
                escher_telemetry::CAPTURED_LEVELS[row] == *tracing_level,
                "row {row}"
            );
        }
        assert_eq!(boundary_word(Boundary::Root), "root");
        assert_eq!(boundary_word(Boundary::Own), "own");
    }

    #[test]
    fn past_counts_add_up_by_kind_and_source() {
        let mut past = Vec::new();
        add_past(&mut past, CausedKind::Log, Some(Source::App), 2);
        add_past(&mut past, CausedKind::Panic, None, 0);
        add_past(&mut past, CausedKind::Log, Some(Source::Engine), 1);
        add_past(&mut past, CausedKind::Log, Some(Source::App), 3);
        add_past(&mut past, CausedKind::Panic, None, 1);
        let expected = [
            (CausedKind::Log, Some(Source::App), 5),
            (CausedKind::Log, Some(Source::Engine), 1),
            (CausedKind::Panic, None, 1),
        ];
        assert_eq!(expected.len(), 3);
        let read: Vec<_> = past
            .iter()
            .map(|entry| (entry.kind, entry.source, entry.count))
            .collect();
        assert_eq!(read, expected);
    }
}
