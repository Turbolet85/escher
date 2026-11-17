//! The `caused` items: what an acting step caused, written last on its answer.

use super::{Object, list};
use crate::caused::{Caused, CausedKind, CausedPast, boundary_word};

/// Writes `caused` and then `caused_past` last on an acting answer, each only when it holds
/// something.
pub(super) fn with_caused(mut object: Object, caused: &[Caused], past: &[CausedPast]) -> String {
    if !caused.is_empty() {
        object.json("caused", &list(caused.iter().map(caused_item)));
    }
    if !past.is_empty() {
        object.json("caused_past", &list(past.iter().map(caused_past_item)));
    }
    object.end()
}

/// A kept text as `key`, then its `cut` only where it was cut.
fn cut_text(object: &mut Object, key: &str, text: &escher_telemetry::Text) {
    object.text(key, &text.text);
    if text.cut > 0 {
        object.count("cut", u64::try_from(text.cut).unwrap_or(u64::MAX));
    }
}

fn caused_item(item: &Caused) -> String {
    let mut object = Object::new();
    object.text("kind", item.kind.name());
    object.text("level", item.level.name());
    if let Some(source) = item.source {
        object.text("source", source.name());
    }
    if let Some(target) = &item.target {
        object.text("target", target);
    }
    cut_text(&mut object, "message", &item.message);
    if let Some(file) = &item.file {
        object.text("file", file);
    }
    if let Some(line) = item.line {
        object.count("line", u64::from(line));
    }
    if let Some(column) = item.column {
        object.count("column", u64::from(column));
    }
    if let Some(boundary) = item.boundary {
        object.text("boundary", boundary_word(boundary));
    }
    if item.kind == CausedKind::Error {
        object.json(
            "chain",
            &list(item.chain.iter().map(|link| {
                let mut entry = Object::new();
                cut_text(&mut entry, "message", link);
                entry.end()
            })),
        );
        object.count("chain_past", u64::from(item.chain_past));
    }
    if item.kind == CausedKind::Log {
        object.json(
            "fields",
            &list(item.fields.iter().map(|field| {
                let mut entry = Object::new();
                entry.text("name", &field.name);
                cut_text(&mut entry, "value", &field.value);
                entry.end()
            })),
        );
        object.count("fields_past", u64::from(item.fields_past));
    }
    if let Some(id) = &item.id {
        object.text("id", id);
    }
    if let Some(owner) = &item.owner {
        object.text("owner", owner);
    }
    if let Some(event) = &item.event {
        object.text("event", event);
    }
    object.end()
}

fn caused_past_item(past: &CausedPast) -> String {
    let mut object = Object::new();
    object.text("kind", past.kind.name());
    if let Some(source) = past.source {
        object.text("source", source.name());
    }
    object.count("count", u64::from(past.count));
    object.end()
}
