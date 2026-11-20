//! The diff form: what every acting verb returns, the nodes added, removed and changed.

use blitz_test_harness::Busy;
use dioxus_native_dom::{DiffNode, SnapshotDiff};

use super::snapshot::{NodeParts, node};
use super::{Object, list, string};
use crate::execute::busy_word;

/// The nodes of a `nodes` result field; `changed` writes each node's `differs`.
fn nodes(nodes: &[DiffNode], changed: bool) -> String {
    list(nodes.iter().map(|read| {
        // The role in the spelling the snapshot's text writes.
        let role = format!("{:?}", read.role);
        node(&NodeParts {
            id: &read.id,
            parent: read.parent.as_deref(),
            role: &role,
            name: &read.name,
            enabled: read.state.enabled,
            checked: read.state.checked,
            value: read.state.value.as_deref(),
            selected: read.state.selected,
            pressed: read.state.pressed,
            invalid: read.state.invalid,
            focused: read.state.focused,
            clickable: read.state.clickable,
            highlighted: read.state.highlighted,
            bounds: [
                read.bounds.x,
                read.bounds.y,
                read.bounds.width,
                read.bounds.height,
            ],
            differs: changed.then_some(read.differs.as_slice()),
        })
    }))
}

/// The fields every acting verb returns, in the schema's order.
pub(super) fn acted(settled: bool, busy: Option<Busy>, diff: &SnapshotDiff) -> Object {
    let mut object = Object::new();
    object.flag("settled", settled);
    if let Some(busy) = busy {
        object.text("busy", busy_word(busy));
    }
    object.json("added", &nodes(&diff.added, false));
    object.json(
        "removed",
        &list(diff.removed.iter().map(|id| {
            let mut text = String::new();
            string(&mut text, id);
            text
        })),
    );
    object.json("changed", &nodes(&diff.changed, true));
    object
}
