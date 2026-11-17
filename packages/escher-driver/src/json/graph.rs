//! The UI graph: a `graph` answer's nodes and states.

use dioxus_native_dom::{GraphNode, GraphState, WriteFlag};

use super::{Object, list, string};

/// A list of texts.
fn texts<'a>(items: impl Iterator<Item = &'a str>) -> String {
    list(items.map(|item| {
        let mut text = String::new();
        string(&mut text, item);
        text
    }))
}

/// The two readings of a write and its flag, where there is one.
fn write_readings(object: &mut Object, declared: bool, confirmed: bool, flag: Option<WriteFlag>) {
    object.flag("declared", declared);
    object.flag("confirmed", confirmed);
    if let Some(flag) = flag {
        object.text("flag", flag.word());
    }
}

/// One node of a `graph`'s `nodes`, keyed as the field's help names.
pub(super) fn graph_node(node: &GraphNode) -> String {
    let mut object = Object::new();
    object.text("id", &node.id);
    if let Some(owner) = &node.owner {
        object.text("owner", owner);
    }
    if let Some(component) = &node.component {
        object.text("component", component);
    }
    object.json(
        "handlers",
        &list(node.handlers.iter().map(|handler| {
            let mut entry = Object::new();
            entry.text("event", &handler.event);
            entry.json(
                "writes",
                &list(handler.writes.iter().map(|write| {
                    let mut entry = Object::new();
                    entry.text("state", &write.state);
                    write_readings(&mut entry, write.declared, write.confirmed, write.flag);
                    entry.end()
                })),
            );
            entry.end()
        })),
    );
    object.json("reads", &texts(node.reads.iter().map(String::as_str)));
    object.end()
}

/// One state of a `graph`'s `states`, keyed as the field's help names.
pub(super) fn graph_state(state: &GraphState) -> String {
    let mut object = Object::new();
    object.text("name", &state.name);
    if let Some(component) = &state.component {
        object.text("component", component);
    }
    object.json(
        "declared",
        &list(state.declared.iter().map(|declared| {
            let mut entry = Object::new();
            entry.text("id", declared.id);
            entry.text("event", declared.event);
            entry.end()
        })),
    );
    object.json(
        "writers",
        &list(state.writers.iter().map(|writer| {
            let mut entry = Object::new();
            entry.text("id", &writer.id);
            entry.text("event", &writer.event);
            write_readings(&mut entry, writer.declared, writer.confirmed, writer.flag);
            entry.end()
        })),
    );
    object.json("readers", &texts(state.readers.iter().map(String::as_str)));
    object.end()
}
