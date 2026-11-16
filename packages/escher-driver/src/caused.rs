//! What an acting step caused: the handler panics caught on the run path, an error the app's
//! root error boundary took, and the records the step logged, each tied to the first handler
//! the step's dispatches reached. Closed words, ids and counts alone: no payload, message, field
//! value or target name.

use dioxus_native_dom::{DioxusDocument, ReachedHandler, RootErrorMark};
use escher_telemetry::Captured;
use tracing::Level;

/// What an item of an acting answer's `caused` is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausedKind {
    /// A handler panicked during the step, and the run path caught it.
    Panic,
    /// A handler returned an error that reached the app's root error boundary.
    Error,
    /// The app, or the UI framework at `warn` or `error`, logged a record during the step.
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

/// One thing an acting step caused, tied to the handler it ran under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caused {
    /// What it is.
    pub kind: CausedKind,
    /// Its level: `error` for a panic and an error.
    pub level: CausedLevel,
    /// The stable id of the element carrying the first handler the step reached; `None` when
    /// the step reached none.
    pub id: Option<String>,
    /// That element's owner id, as the UI graph reads it; `None` where it has none.
    pub owner: Option<String>,
    /// That handler's event name, without its `on`.
    pub event: Option<String>,
    /// How many: the panics caught, 1 for an error, the records logged at its level.
    pub count: u32,
}

/// What the run path saw of one step.
pub(crate) struct StepReading {
    /// The panics caught around the step and its settle.
    pub(crate) panics: u32,
    /// The root error boundary's mark before the step.
    pub(crate) mark_before: Option<RootErrorMark>,
    /// The records the step and its settle logged.
    pub(crate) captured: Captured,
    /// The handlers the step's dispatches reached, in first-reached order.
    pub(crate) reached: Vec<ReachedHandler>,
}

/// The items of `reading`: a `panic`, an `error`, then a `log` per level counted, each tied to
/// the first handler reached, its owner read from `doc`'s UI graph.
pub(crate) fn caused(doc: &DioxusDocument, reading: StepReading) -> Vec<Caused> {
    let mark_after = doc.root_error_mark();
    let errored = mark_after.is_some() && mark_after != reading.mark_before;
    let mut items: Vec<(CausedKind, CausedLevel, u32)> = Vec::new();
    if reading.panics > 0 {
        items.push((CausedKind::Panic, CausedLevel::Error, reading.panics));
    }
    if errored {
        items.push((CausedKind::Error, CausedLevel::Error, 1));
    }
    for (level, count) in reading.captured.levels() {
        if count > 0 {
            items.push((CausedKind::Log, CausedLevel::of(level), count));
        }
    }
    if items.is_empty() {
        return Vec::new();
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
    items
        .into_iter()
        .map(|(kind, level, count)| Caused {
            kind,
            level,
            id: id.clone(),
            owner: owner.clone(),
            event: event.clone(),
            count,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_kind_and_level_reads_as_its_closed_word() {
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
    }
}
