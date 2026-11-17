//! The actions answer: a node, its owner and the verbs it takes.

use super::{Object, list};
use crate::actions::ActionNode;

/// One node of an `actions` answer, keyed as the field's help names.
pub(super) fn action_node(node: &ActionNode) -> String {
    let mut object = Object::new();
    object.text("id", &node.id);
    if let Some(owner) = &node.owner {
        object.text("owner", owner);
    }
    object.flag("keyed", node.keyed);
    object.json(
        "verbs",
        &list(node.verbs.iter().map(|reading| {
            let mut entry = Object::new();
            entry.text("verb", reading.verb);
            if let Some(cause) = reading.cause {
                entry.text("cause", cause.name());
            }
            entry.end()
        })),
    );
    object.end()
}
