//! The node form: a snapshot node, as a `nodes` result field and a diff write it.

use dioxus_native_dom::Reading;

use super::{Object, list, number, string};
use crate::schema::NODE_FIELDS;

/// What a node of a `nodes` result field reads, by the names of the schema's node fields.
pub(super) struct NodeParts<'a> {
    pub(super) id: &'a str,
    pub(super) parent: Option<&'a str>,
    pub(super) role: &'a str,
    pub(super) name: &'a str,
    pub(super) enabled: Option<bool>,
    pub(super) checked: Option<bool>,
    pub(super) value: Option<&'a str>,
    pub(super) selected: Option<bool>,
    pub(super) pressed: Option<bool>,
    pub(super) invalid: bool,
    pub(super) focused: bool,
    pub(super) clickable: bool,
    /// x, y, width and height.
    pub(super) bounds: [f64; 4],
    /// For a changed node, the readings that differ; `None` for any other node.
    pub(super) differs: Option<&'a [Reading]>,
}

/// The node key a reading is written as.
pub(super) fn reading_word(reading: Reading) -> &'static str {
    let index = match reading {
        Reading::Parent => 1,
        Reading::Role => 2,
        Reading::Name => 3,
        Reading::Enabled => 4,
        Reading::Checked => 5,
        Reading::Value => 6,
        Reading::Selected => 7,
        Reading::Pressed => 8,
        Reading::Invalid => 9,
        Reading::Focused => 10,
        Reading::Clickable => 11,
        Reading::Bounds => 12,
    };
    NODE_FIELDS[index].name
}

/// A node as an object with the fields the schema's node fields name, in that order. A field
/// the node has no reading for is left out, and `differs` is written for a changed node only.
pub(super) fn node(parts: &NodeParts<'_>) -> String {
    let mut object = Object::new();
    object.text("id", parts.id);
    if let Some(parent) = parts.parent {
        object.text("parent", parent);
    }
    object.text("role", parts.role);
    object.text("name", parts.name);
    if let Some(enabled) = parts.enabled {
        object.flag("enabled", enabled);
    }
    if let Some(checked) = parts.checked {
        object.flag("checked", checked);
    }
    if let Some(value) = parts.value {
        object.text("value", value);
    }
    if let Some(selected) = parts.selected {
        object.flag("selected", selected);
    }
    if let Some(pressed) = parts.pressed {
        object.flag("pressed", pressed);
    }
    object.flag("invalid", parts.invalid);
    object.flag("focused", parts.focused);
    object.flag("clickable", parts.clickable);
    let bounds = object.member("bounds");
    bounds.push('[');
    for (index, bound) in parts.bounds.into_iter().enumerate() {
        if index > 0 {
            bounds.push(',');
        }
        number(bounds, bound);
    }
    bounds.push(']');
    if let Some(differs) = parts.differs {
        object.json(
            "differs",
            &list(differs.iter().map(|reading| {
                let mut word = String::new();
                string(&mut word, reading_word(*reading));
                word
            })),
        );
    }
    object.end()
}
