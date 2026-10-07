//! The diff of two [`Snapshot`]s of one screen: the nodes the later one adds, the ids it lacks
//! and the nodes whose reading differs, each named by its stable element id.

use std::collections::HashMap;

use accesskit::Role;
use blitz_dom::BoundingRect;

use crate::snapshot::{NodeState, Snapshot, SnapshotNode};

/// What differs between two [`Snapshot`]s. Built by [`Snapshot::diff`].
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotDiff {
    /// The nodes of the later snapshot whose id the earlier one lacks, in the later snapshot's
    /// pre-order.
    pub added: Vec<DiffNode>,
    /// The ids of the earlier snapshot's nodes the later one lacks, in the earlier snapshot's
    /// pre-order.
    pub removed: Vec<String>,
    /// The nodes both snapshots hold whose role, name, state, bounds or parent differ, as the
    /// later snapshot reads them, in its pre-order.
    pub changed: Vec<DiffNode>,
}

impl SnapshotDiff {
    /// Whether the two snapshots read the same: nothing added, removed or changed.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

/// One node of a [`SnapshotDiff`]: what the later snapshot reads for it, without its children.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffNode {
    /// The node's stable element id (see [`SnapshotNode::id`]).
    pub id: String,
    /// The id of the node's parent in the later snapshot; `None` for a root.
    pub parent: Option<String>,
    /// The node's role.
    pub role: Role,
    /// The node's accessible name.
    pub name: String,
    /// The node's state.
    pub state: NodeState,
    /// The node's bounds.
    pub bounds: BoundingRect,
}

impl Snapshot {
    /// What `after` reads differently from this snapshot, taken before it.
    ///
    /// A node is added when `after` holds its id and this snapshot does not, removed when
    /// this snapshot holds it and `after` does not, and changed when both hold it and its
    /// role, name, state, bounds or parent's id differ. A node whose own reading is the same
    /// is not named because a descendant's differs. Where two nodes of one snapshot carry one
    /// id, the first in pre-order stands for it, as [`Snapshot::get`] reads it.
    ///
    /// The diff reads the two snapshots and nothing else, so every value in it is one a
    /// snapshot holds: a password or file `input`'s value is the snapshot's mask.
    pub fn diff(&self, after: &Snapshot) -> SnapshotDiff {
        let earlier = with_parents(self);
        let later = with_parents(after);
        let earlier_at = first_by_id(&earlier);
        let later_at = first_by_id(&later);

        let mut added = Vec::new();
        let mut changed = Vec::new();
        for (index, (node, parent)) in later.iter().enumerate() {
            if later_at[node.id.as_str()] != index {
                continue;
            }
            match earlier_at.get(node.id.as_str()) {
                None => added.push(reading(node, *parent)),
                Some(&at) => {
                    let (was, was_parent) = earlier[at];
                    if was.role != node.role
                        || was.name != node.name
                        || was.state != node.state
                        || was.bounds != node.bounds
                        || was_parent != *parent
                    {
                        changed.push(reading(node, *parent));
                    }
                }
            }
        }
        let removed = earlier
            .iter()
            .enumerate()
            .filter(|(index, (node, _))| {
                earlier_at[node.id.as_str()] == *index && !later_at.contains_key(node.id.as_str())
            })
            .map(|(_, (node, _))| node.id.clone())
            .collect();

        SnapshotDiff {
            added,
            removed,
            changed,
        }
    }
}

/// The snapshot's nodes in pre-order, each with the id of its parent node.
fn with_parents(snapshot: &Snapshot) -> Vec<(&SnapshotNode, Option<&str>)> {
    let mut out = Vec::new();
    let mut stack: Vec<(&SnapshotNode, Option<&str>)> = snapshot
        .roots
        .iter()
        .rev()
        .map(|root| (root, None))
        .collect();
    while let Some((node, parent)) = stack.pop() {
        out.push((node, parent));
        let id = Some(node.id.as_str());
        stack.extend(node.children.iter().rev().map(|child| (child, id)));
    }
    out
}

/// Where each id first occurs in `nodes`.
fn first_by_id<'s>(nodes: &[(&'s SnapshotNode, Option<&'s str>)]) -> HashMap<&'s str, usize> {
    let mut at = HashMap::new();
    for (index, (node, _)) in nodes.iter().enumerate() {
        at.entry(node.id.as_str()).or_insert(index);
    }
    at
}

fn reading(node: &SnapshotNode, parent: Option<&str>) -> DiffNode {
    DiffNode {
        id: node.id.clone(),
        parent: parent.map(str::to_string),
        role: node.role,
        name: node.name.clone(),
        state: node.state.clone(),
        bounds: node.bounds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOX: BoundingRect = BoundingRect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    fn node(id: &str, name: &str, children: Vec<SnapshotNode>) -> SnapshotNode {
        SnapshotNode {
            id: id.to_string(),
            role: Role::GenericContainer,
            name: name.to_string(),
            state: NodeState::default(),
            bounds: BOX,
            children,
        }
    }

    fn screen(roots: Vec<SnapshotNode>) -> Snapshot {
        Snapshot { roots }
    }

    /// `root` holding `list` holding `first` and `second`, and `side` beside the list.
    fn base() -> Snapshot {
        screen(vec![node(
            "root",
            "",
            vec![
                node(
                    "list",
                    "",
                    vec![node("first", "One", vec![]), node("second", "Two", vec![])],
                ),
                node("side", "", vec![]),
            ],
        )])
    }

    /// The node `id` names in `snapshot`, to edit in place.
    fn edit<'s>(snapshot: &'s mut Snapshot, id: &str) -> &'s mut SnapshotNode {
        fn find<'n>(nodes: &'n mut [SnapshotNode], id: &str) -> Option<&'n mut SnapshotNode> {
            for node in nodes {
                if node.id == id {
                    return Some(node);
                }
                if let Some(found) = find(&mut node.children, id) {
                    return Some(found);
                }
            }
            None
        }
        find(&mut snapshot.roots, id).unwrap_or_else(|| panic!("{id:?} is a node"))
    }

    fn ids(nodes: &[DiffNode]) -> Vec<&str> {
        nodes.iter().map(|node| node.id.as_str()).collect()
    }

    fn parents(nodes: &[DiffNode]) -> Vec<Option<&str>> {
        nodes.iter().map(|node| node.parent.as_deref()).collect()
    }

    #[test]
    fn two_equal_snapshots_give_an_empty_diff() {
        let diff = base().diff(&base());
        assert!(diff.is_empty());
        assert_eq!(
            diff,
            SnapshotDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            }
        );
        assert!(screen(vec![]).diff(&screen(vec![])).is_empty());
    }

    #[test]
    fn a_changed_name_names_that_node_alone() {
        let mut after = base();
        edit(&mut after, "second").name = "Deux".to_string();

        let diff = base().diff(&after);
        assert!(!diff.is_empty());
        assert_eq!(
            diff.changed,
            [DiffNode {
                id: "second".to_string(),
                parent: Some("list".to_string()),
                role: Role::GenericContainer,
                name: "Deux".to_string(),
                state: NodeState::default(),
                bounds: BOX,
            }]
        );
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn a_changed_state_names_that_node_alone() {
        let mut after = base();
        edit(&mut after, "first").state.focused = true;
        edit(&mut after, "side").role = Role::Button;

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.changed), ["first", "side"]);
        assert!(diff.changed[0].state.focused, "the reading after the step");
        assert_eq!(diff.changed[1].role, Role::Button);
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn changed_bounds_name_that_node_alone() {
        let moved = BoundingRect { y: 24.0, ..BOX };
        let mut after = base();
        edit(&mut after, "side").bounds = moved;

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.changed), ["side"]);
        assert_eq!(diff.changed[0].bounds, moved);
        assert_eq!(parents(&diff.changed), [Some("root")]);
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn an_added_subtree_names_every_node_with_its_parent() {
        let mut after = base();
        edit(&mut after, "side").children = vec![node(
            "panel",
            "",
            vec![node("ok", "OK", vec![]), node("cancel", "Cancel", vec![])],
        )];

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.added), ["panel", "ok", "cancel"]);
        assert_eq!(
            parents(&diff.added),
            [Some("side"), Some("panel"), Some("panel")]
        );
        assert_eq!(diff.added[1].name, "OK");
        assert!(diff.changed.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn a_removed_subtree_names_every_id_in_the_before_order() {
        let mut after = base();
        edit(&mut after, "root").children.remove(0);

        let diff = base().diff(&after);
        assert_eq!(diff.removed, ["list", "first", "second"]);
        assert!(diff.added.is_empty() && diff.changed.is_empty());
    }

    #[test]
    fn a_node_moved_under_another_parent_is_changed() {
        let mut after = base();
        let moved = edit(&mut after, "list").children.remove(0);
        edit(&mut after, "side").children.push(moved);

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.changed), ["first"]);
        assert_eq!(parents(&diff.changed), [Some("side")]);
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn added_and_changed_follow_the_after_pre_order() {
        let before = screen(vec![
            node("a", "A", vec![]),
            node("b", "B", vec![]),
            node("c", "C", vec![]),
        ]);
        let after = screen(vec![
            node("new-1", "", vec![]),
            node("c", "C2", vec![node("new-2", "", vec![])]),
            node("b", "B", vec![]),
            node("a", "A2", vec![]),
            node("new-3", "", vec![]),
        ]);

        let diff = before.diff(&after);
        assert_eq!(ids(&diff.added), ["new-1", "new-2", "new-3"]);
        assert_eq!(parents(&diff.added), [None, Some("c"), None]);
        assert_eq!(ids(&diff.changed), ["c", "a"]);
        assert!(diff.removed.is_empty());

        // Two nodes carrying one id: the first in pre-order stands for it.
        let twins = |first: &str, second: &str| {
            screen(vec![node(
                "root",
                "",
                vec![node("twin", first, vec![]), node("twin", second, vec![])],
            )])
        };
        let alone = screen(vec![node("root", "", vec![])]);
        assert!(
            twins("A", "B").diff(&twins("A", "C")).is_empty(),
            "a change of the second twin is not a change of the id"
        );
        let diff = twins("A", "B").diff(&twins("A2", "B"));
        assert_eq!(ids(&diff.changed), ["twin"]);
        assert_eq!(diff.changed[0].name, "A2");
        let diff = alone.diff(&twins("A", "B"));
        assert_eq!(ids(&diff.added), ["twin"]);
        assert_eq!(diff.added[0].name, "A");
        assert_eq!(twins("A", "B").diff(&alone).removed, ["twin"]);
    }
}
