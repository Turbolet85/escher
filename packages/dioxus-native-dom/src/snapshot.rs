//! The snapshot model: the screen of a [`DioxusDocument`] read as a tree whose every node is
//! an element carrying its stable element id, role, accessible name, state and bounds.

use std::collections::{HashMap, HashSet};

use accesskit::{Node as TreeNode, NodeId as TreeNodeId, Role, Toggled, TreeUpdate};
use blitz_dom::{BaseDocument, BoundingRect, Document, ElementData, NodeId, local_name};

use crate::DioxusDocument;

/// What a password or file `input` holding a value reads as its [`NodeState::value`]: one fixed
/// marker, whatever was typed or chosen, so no text, length or file path is in a [`Snapshot`].
pub const MASKED_VALUE: &str = "••••••••";

/// The screen of a document as a tree of the elements its accessibility tree keeps, in
/// document order. Built by [`DioxusDocument::snapshot`].
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    /// The top-level nodes: in a document with an `html` element, that one element.
    pub roots: Vec<SnapshotNode>,
}

/// One element of a [`Snapshot`].
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotNode {
    /// The element's stable element id (see [`DioxusDocument::element_id`]), which its
    /// accessibility node carries as AccessKit `author_id`.
    pub id: String,
    /// The role of the element's accessibility node.
    pub role: Role,
    /// The accessible name of the element's accessibility node, trimmed: its label, else the
    /// joined names of the nodes labelling it. `""` when it has no name source.
    pub name: String,
    /// What the element reads as: enabled, checked, its value, whether it states itself
    /// selected, pressed or invalid, and whether it has focus.
    pub state: NodeState,
    /// The element's bounding client rect in CSS px, relative to the viewport (see
    /// [`BaseDocument::get_client_bounding_rect`]).
    pub bounds: BoundingRect,
    /// The element's kept descendants, each attached to its nearest kept ancestor, in
    /// document order.
    pub children: Vec<SnapshotNode>,
}

/// The state of a [`SnapshotNode`]'s element.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeState {
    /// For an element that can be disabled (`button`, `input`, `select`, `textarea`), whether
    /// it lacks a `disabled` attribute; `None` for every other element. The reading follows
    /// the attribute's presence, which is what decides whether a click acts on the element, so
    /// a `disabled="false"` and a bare `disabled` both read not enabled.
    pub enabled: Option<bool>,
    /// For a checkbox or radio `input`, whether it is checked; else, for an element whose role is
    /// `checkbox`, `switch`, `radio`, `menuitemcheckbox` or `menuitemradio`, whether it states
    /// itself checked, as its accessibility node reads its `aria-checked`; `None` for every
    /// other element and where such a role states neither. A mixed one reads `Some(false)`, not
    /// checked; the accessibility node keeps which of the three it is.
    pub checked: Option<bool>,
    /// For a text-entry `input` or a `textarea`, its current text, and for a range `input` its
    /// current value, else its `value` attribute; `None` for every other element. A password or file `input` whose value is not empty reads
    /// [`MASKED_VALUE`], never the text or the path; an empty one reads as any other `input`.
    pub value: Option<String>,
    /// Whether the element states itself selected, as its accessibility node reads its
    /// `aria-selected`: `None` where the element states neither.
    pub selected: Option<bool>,
    /// Whether the element states itself pressed, as its accessibility node reads its
    /// `aria-pressed`: `None` where the element states neither and for the five roles whose
    /// toggle is their `aria-checked`, read as [`NodeState::checked`]. A mixed toggle reads
    /// `Some(false)`, not pressed; the accessibility node keeps which of the three it is.
    pub pressed: Option<bool>,
    /// Whether the element states itself invalid, as its accessibility node reads its
    /// `aria-invalid`. A grammar or a spelling error reads `true`; the accessibility node keeps
    /// which kind it is.
    pub invalid: bool,
    /// Whether the element is the document's focused node.
    pub focused: bool,
    /// Whether the element reacts to a click: it carries a `click` listener of the app's own now
    /// and is no disabled form control. A native activation with no app handler (a bare button,
    /// a checkbox, a `summary`) and a listener on an ancestor read false.
    pub clickable: bool,
}

impl Snapshot {
    /// Every node of the snapshot in pre-order.
    pub fn nodes(&self) -> impl Iterator<Item = &SnapshotNode> + '_ {
        let mut stack: Vec<&SnapshotNode> = self.roots.iter().rev().collect();
        std::iter::from_fn(move || {
            let node = stack.pop()?;
            stack.extend(node.children.iter().rev());
            Some(node)
        })
    }

    /// The node whose stable element id is `id`.
    pub fn get(&self, id: &str) -> Option<&SnapshotNode> {
        self.nodes().find(|node| node.id == id)
    }
}

impl DioxusDocument {
    /// The screen as a [`Snapshot`]: one node per element the accessibility tree keeps, with
    /// its id and role from that tree, its name resolved over that tree, its bounds from
    /// [`BaseDocument::get_client_bounding_rect`] and its state from the element — but for
    /// selected, pressed, invalid and an ARIA checked, which that tree's node holds.
    pub fn snapshot(&self) -> Snapshot {
        let tree = self.accessibility_tree();
        let doc = self.inner();
        let builder = Builder {
            doc: &doc,
            nodes: tree.nodes.iter().map(|(id, node)| (*id, node)).collect(),
            focus: tree.focus,
            clicked: crate::element_id::element_listeners(&self.vdom, &self.vdom_state)
                .into_iter()
                .filter(|(_, events)| events.contains(&"click"))
                .map(|(node, _)| node)
                .collect(),
        };
        let mut roots = Vec::new();
        if let Some(root) = tree_root(&tree) {
            builder.visit(root, &mut HashSet::new(), &mut roots);
        }
        Snapshot { roots }
    }
}

fn tree_root(tree: &TreeUpdate) -> Option<TreeNodeId> {
    tree.tree.as_ref().map(|info| info.root)
}

struct Builder<'a> {
    doc: &'a BaseDocument,
    nodes: HashMap<TreeNodeId, &'a TreeNode>,
    focus: TreeNodeId,
    /// The elements carrying an app `click` listener now.
    clicked: HashSet<NodeId>,
}

impl Builder<'_> {
    /// Append the snapshot nodes of `id`'s subtree to `out`: `id` itself when it is kept,
    /// else its kept descendants.
    fn visit(
        &self,
        id: TreeNodeId,
        visited: &mut HashSet<TreeNodeId>,
        out: &mut Vec<SnapshotNode>,
    ) {
        if !visited.insert(id) {
            return;
        }
        let Some(tree_node) = self.nodes.get(&id) else {
            return;
        };
        let element = tree_node.author_id().and_then(|author_id| {
            let node_id = NodeId::from_u64(id.0);
            let node = self.doc.get_node(node_id)?;
            let element = node.element_data()?;
            // A closed select's option or group is seen in its select's box, the one it shows or
            // not; an open list's rows and a `multiple` select's have boxes of their own.
            let boxed_by = match closed_select_of(self.doc, node_id) {
                Some(select) => select,
                None => node_id,
            };
            let bounds = match self.doc.select_list_bounds(node_id) {
                Some(bounds) => bounds,
                None => self.doc.get_client_bounding_rect(boxed_by)?,
            };
            Some((author_id, node_id, element, bounds))
        });
        match element {
            Some((author_id, node_id, element, bounds)) => {
                let checkable = is_checkable(tree_node.role());
                let mut children = Vec::new();
                for child in tree_node.children() {
                    self.visit(*child, visited, &mut children);
                }
                out.push(SnapshotNode {
                    id: author_id.to_string(),
                    role: tree_node.role(),
                    name: self.name(tree_node, &mut HashSet::new()).trim().to_string(),
                    state: NodeState {
                        enabled: element
                            .can_be_disabled()
                            .then(|| !element.has_attr(local_name!("disabled"))),
                        checked: element.checkbox_input_checked().or_else(|| {
                            checkable
                                .then(|| {
                                    tree_node.toggled().map(|toggled| toggled == Toggled::True)
                                })
                                .flatten()
                        }),
                        value: value(self.doc, node_id, element),
                        selected: tree_node.is_selected(),
                        pressed: tree_node
                            .toggled()
                            .filter(|_| !checkable)
                            .map(|toggled| toggled == Toggled::True),
                        invalid: tree_node.invalid().is_some(),
                        focused: id == self.focus,
                        clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
                    },
                    bounds,
                    children,
                });
            }
            None => {
                for child in tree_node.children() {
                    self.visit(*child, visited, out);
                }
            }
        }
    }

    /// AccessKit's name model: the node's label, else the joined names of the nodes
    /// labelling it, where a `TextRun`'s name is its value.
    fn name(&self, node: &TreeNode, visited: &mut HashSet<TreeNodeId>) -> String {
        if let Some(label) = node.label().filter(|label| !label.is_empty()) {
            return label.to_string();
        }
        if node.role() == Role::TextRun {
            return node.value().unwrap_or_default().to_string();
        }
        let mut name = String::new();
        for id in node.labelled_by() {
            if !visited.insert(*id) {
                continue;
            }
            if let Some(labelling) = self.nodes.get(id) {
                name.push_str(&self.name(labelling, visited));
            }
        }
        name
    }
}

/// Whether `role` states its toggle by `aria-checked`, which its accessibility node holds as
/// `toggled`, as every other role holds `aria-pressed` there.
fn is_checkable(role: Role) -> bool {
    matches!(
        role,
        Role::CheckBox
            | Role::Switch
            | Role::RadioButton
            | Role::MenuItemCheckBox
            | Role::MenuItemRadio
    )
}

/// Whether `element` is a masked control: an `input` whose `type` is `password` or `file`,
/// in any ASCII case. What such a control holds reads as [`MASKED_VALUE`] in a [`Snapshot`].
pub fn is_masked_input(element: &ElementData) -> bool {
    &*element.name.local == "input"
        && element.attr(local_name!("type")).is_some_and(|input_type| {
            input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
        })
}

/// The select with no `multiple` and no list open that an `option` or an `optgroup` belongs to;
/// `None` for any other node.
pub fn closed_select_of(doc: &BaseDocument, node: NodeId) -> Option<NodeId> {
    let select = doc.select_of(node)?;
    let element = doc.get_node(select)?.element_data()?;
    (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
}

/// Whether `element` is a `select`, which the driver's `type` focuses with no click.
pub fn is_select(element: &ElementData) -> bool {
    &*element.name.local == "select"
}

/// Whether `element` is a `select` with `multiple`, whose rows have boxes of their own.
pub fn is_multiple_select(element: &ElementData) -> bool {
    &*element.name.local == "select" && element.has_attr(local_name!("multiple"))
}

/// The form reader's value of a text-entry control: its editor's text or a range input's
/// current value, else its `value` attribute, and a select's [`BaseDocument::select_value`]. A
/// masked control ([`is_masked_input`]) reads [`MASKED_VALUE`] in place of a non-empty one.
fn value(doc: &BaseDocument, node_id: NodeId, element: &ElementData) -> Option<String> {
    if &*element.name.local == "select" {
        return doc.select_value(node_id);
    }
    let input_type = element.attr(local_name!("type"));
    let entry = match &*element.name.local {
        "textarea" => true,
        "input" => !matches!(
            input_type,
            Some("checkbox" | "radio" | "button" | "submit" | "reset" | "hidden")
        ),
        _ => false,
    };
    if !entry {
        return None;
    }
    let masked = is_masked_input(element);
    let value = match element.text_input_data() {
        Some(text) => Some(text.editor.text().to_string()),
        None => element
            .range_input_value()
            .or_else(|| element.attr(local_name!("value")).map(str::to_string)),
    };
    match value {
        Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
        value => value,
    }
}

#[cfg(test)]
#[path = "snapshot_tests.rs"]
mod tests;
