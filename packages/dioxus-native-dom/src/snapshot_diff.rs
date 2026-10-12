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

/// One reading of a node that [`Snapshot::diff`] compares, in the order a node is read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading {
    /// The id of the node's parent.
    Parent,
    /// Its role.
    Role,
    /// Its accessible name.
    Name,
    /// [`NodeState::enabled`].
    Enabled,
    /// [`NodeState::checked`].
    Checked,
    /// [`NodeState::value`].
    Value,
    /// [`NodeState::selected`].
    Selected,
    /// [`NodeState::pressed`].
    Pressed,
    /// [`NodeState::invalid`].
    Invalid,
    /// [`NodeState::focused`].
    Focused,
    /// Its bounds.
    Bounds,
}

impl Reading {
    /// Every reading, in the order a node is read.
    pub const ALL: [Reading; 11] = [
        Reading::Parent,
        Reading::Role,
        Reading::Name,
        Reading::Enabled,
        Reading::Checked,
        Reading::Value,
        Reading::Selected,
        Reading::Pressed,
        Reading::Invalid,
        Reading::Focused,
        Reading::Bounds,
    ];
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
    /// For a changed node, the readings that differ from the earlier snapshot's, in the order of
    /// [`Reading::ALL`]; empty for an added node.
    pub differs: Vec<Reading>,
}

impl Snapshot {
    /// What `after` reads differently from this snapshot, taken before it.
    ///
    /// A node is added when `after` holds its id and this snapshot does not, removed when
    /// this snapshot holds it and `after` does not, and changed when both hold it and its
    /// role, name, state, bounds or parent's id differ; a changed node names which of them
    /// differ ([`DiffNode::differs`]). A node whose own reading is the same is not named because
    /// a descendant's differs. Where two nodes of one snapshot carry one
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
                None => added.push(reading(node, *parent, Vec::new())),
                Some(&at) => {
                    let (was, was_parent) = earlier[at];
                    let differs = differing(was, was_parent, node, *parent);
                    if !differs.is_empty() {
                        changed.push(reading(node, *parent, differs));
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

/// The readings of a node that differ between the two snapshots, in the order of
/// [`Reading::ALL`].
fn differing(
    was: &SnapshotNode,
    was_parent: Option<&str>,
    node: &SnapshotNode,
    parent: Option<&str>,
) -> Vec<Reading> {
    // Destructured whole, so a reading added to the state is a reading this list compares.
    let NodeState {
        enabled,
        checked,
        value,
        selected,
        pressed,
        invalid,
        focused,
    } = &node.state;
    let before = &was.state;
    [
        (Reading::Parent, was_parent != parent),
        (Reading::Role, was.role != node.role),
        (Reading::Name, was.name != node.name),
        (Reading::Enabled, before.enabled != *enabled),
        (Reading::Checked, before.checked != *checked),
        (Reading::Value, before.value != *value),
        (Reading::Selected, before.selected != *selected),
        (Reading::Pressed, before.pressed != *pressed),
        (Reading::Invalid, before.invalid != *invalid),
        (Reading::Focused, before.focused != *focused),
        (Reading::Bounds, was.bounds != node.bounds),
    ]
    .into_iter()
    .filter_map(|(reading, differs)| differs.then_some(reading))
    .collect()
}

fn reading(node: &SnapshotNode, parent: Option<&str>, differs: Vec<Reading>) -> DiffNode {
    DiffNode {
        id: node.id.clone(),
        parent: parent.map(str::to_string),
        role: node.role,
        name: node.name.clone(),
        state: node.state.clone(),
        bounds: node.bounds,
        differs,
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
                differs: vec![Reading::Name],
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
    fn a_selected_pressed_or_invalid_reading_alone_names_that_node() {
        let readings: [fn(&mut NodeState); 3] = [
            |state| state.selected = Some(true),
            |state| state.pressed = Some(false),
            |state| state.invalid = true,
        ];
        for (row, set) in readings.into_iter().enumerate() {
            let mut after = base();
            set(&mut edit(&mut after, "second").state);
            let mut stated = NodeState::default();
            set(&mut stated);
            assert_ne!(stated, NodeState::default(), "row {row}: the reading moved");

            let diff = base().diff(&after);
            assert_eq!(ids(&diff.changed), ["second"], "row {row}");
            assert_eq!(diff.changed[0].state, stated, "row {row}");
            assert!(
                diff.added.is_empty() && diff.removed.is_empty(),
                "row {row}"
            );
        }
    }

    #[test]
    fn changed_bounds_name_that_node_alone() {
        let moved = BoundingRect { y: 24.0, ..BOX };
        let mut after = base();
        edit(&mut after, "side").bounds = moved;

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.changed), ["side"]);
        assert_eq!(diff.changed[0].bounds, moved);
        assert_eq!(diff.changed[0].differs, [Reading::Bounds]);
        assert_eq!(parents(&diff.changed), [Some("root")]);
        assert!(diff.added.is_empty() && diff.removed.is_empty());
    }

    #[test]
    fn a_changed_node_names_exactly_the_readings_that_differ_in_order() {
        let mut after = base();
        let first = edit(&mut after, "first");
        first.state.focused = true;
        first.state.value = Some("1".to_string());
        let side = edit(&mut after, "side");
        side.bounds = BoundingRect { x: 4.0, ..BOX };
        side.role = Role::Button;
        side.state.pressed = Some(true);

        let diff = base().diff(&after);
        assert_eq!(ids(&diff.changed), ["first", "side"]);
        assert_eq!(diff.changed[0].differs, [Reading::Value, Reading::Focused]);
        assert_eq!(
            diff.changed[1].differs,
            [Reading::Role, Reading::Pressed, Reading::Bounds]
        );

        // An added node names nothing, and a node moved under another parent names its parent.
        edit(&mut after, "side").children = vec![node("new", "", vec![])];
        let diff = base().diff(&after);
        assert_eq!(ids(&diff.added), ["new"]);
        assert!(diff.added[0].differs.is_empty());
        let mut moved = base();
        let child = edit(&mut moved, "list").children.remove(0);
        edit(&mut moved, "side").children.push(child);
        assert_eq!(base().diff(&moved).changed[0].differs, [Reading::Parent]);

        // Each reading alone names itself alone.
        let edits: [fn(&mut SnapshotNode); 11] = [
            |_| {},
            |node| node.role = Role::Button,
            |node| node.name = "Zwei".to_string(),
            |node| node.state.enabled = Some(true),
            |node| node.state.checked = Some(false),
            |node| node.state.value = Some(String::new()),
            |node| node.state.selected = Some(true),
            |node| node.state.pressed = Some(false),
            |node| node.state.invalid = true,
            |node| node.state.focused = true,
            |node| node.bounds = BoundingRect { y: 1.0, ..BOX },
        ];
        assert_eq!(edits.len(), Reading::ALL.len());
        for (row, (set, reading)) in edits.into_iter().zip(Reading::ALL).enumerate().skip(1) {
            let mut after = base();
            set(edit(&mut after, "second"));
            let diff = base().diff(&after);
            assert_eq!(ids(&diff.changed), ["second"], "row {row}");
            assert_eq!(diff.changed[0].differs, [reading], "row {row}");
        }
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
