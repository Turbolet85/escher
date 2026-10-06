//! The snapshot model: the screen of a [`DioxusDocument`] read as a tree whose every node is
//! an element carrying its stable element id, role, accessible name, state and bounds.

use std::collections::{HashMap, HashSet};

use accesskit::{Node as TreeNode, NodeId as TreeNodeId, Role, TreeUpdate};
use blitz_dom::{BaseDocument, BoundingRect, Document, ElementData, NodeId, local_name};

use crate::DioxusDocument;

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
    /// What the element reads as: enabled, checked, its value and whether it has focus.
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
    /// it lacks a `disabled` attribute; `None` for every other element.
    pub enabled: Option<bool>,
    /// For a checkbox or radio `input`, whether it is checked; `None` for every other element.
    pub checked: Option<bool>,
    /// For a text-entry `input` or a `textarea`, its current text, else its `value` attribute;
    /// `None` for every other element.
    pub value: Option<String>,
    /// Whether the element is the document's focused node.
    pub focused: bool,
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
    /// [`BaseDocument::get_client_bounding_rect`] and its state from the element.
    pub fn snapshot(&self) -> Snapshot {
        let tree = self.accessibility_tree();
        let doc = self.inner();
        let builder = Builder {
            doc: &doc,
            nodes: tree.nodes.iter().map(|(id, node)| (*id, node)).collect(),
            focus: tree.focus,
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
            let element = self.doc.get_node(node_id)?.element_data()?;
            let bounds = self.doc.get_client_bounding_rect(node_id)?;
            Some((author_id, element, bounds))
        });
        match element {
            Some((author_id, element, bounds)) => {
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
                        checked: element.checkbox_input_checked(),
                        value: value(element),
                        focused: id == self.focus,
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

/// The form reader's value of a text-entry control: its editor's text, else its `value`
/// attribute.
fn value(element: &ElementData) -> Option<String> {
    let entry = match &*element.name.local {
        "textarea" => true,
        "input" => !matches!(
            element.attr(local_name!("type")),
            Some("checkbox" | "radio" | "button" | "submit" | "reset" | "hidden")
        ),
        _ => false,
    };
    if !entry {
        return None;
    }
    match element.text_input_data() {
        Some(text) => Some(text.editor.text().to_string()),
        None => element.attr(local_name!("value")).map(str::to_string),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blitz_dom::DocumentConfig;
    use dioxus::prelude::*;

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc.inner_mut().resolve(0.0);
        doc
    }

    #[track_caller]
    fn node<'s>(snapshot: &'s Snapshot, id: &str) -> &'s SnapshotNode {
        snapshot
            .get(id)
            .unwrap_or_else(|| panic!("{id:?} is a node of {snapshot:#?}"))
    }

    #[test]
    fn an_aria_labelled_button_reads_its_label() {
        fn app() -> Element {
            rsx! { button { id: "go", "aria-label": "Start", "Go" } }
        }
        let snapshot = build(app).snapshot();
        let go = node(&snapshot, "go");
        assert_eq!(go.role, Role::Button);
        assert_eq!(go.name, "Start");
    }

    #[test]
    fn a_label_bound_input_reads_the_label_text() {
        fn app() -> Element {
            rsx! {
                label { r#for: "who", " Name: " }
                input { id: "who" }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(node(&snapshot, "who").name, "Name:");
    }

    #[test]
    fn a_text_only_paragraph_reads_its_text() {
        fn app() -> Element {
            rsx! { p { id: "line", "Hello there" } }
        }
        let snapshot = build(app).snapshot();
        let line = node(&snapshot, "line");
        assert_eq!(line.role, Role::Paragraph);
        assert_eq!(line.name, "Hello there");
    }

    #[test]
    fn a_display_none_subtree_is_absent() {
        fn app() -> Element {
            rsx! {
                div { id: "shown" }
                div { id: "gone", style: "display: none",
                    button { id: "inner", "Hidden" }
                }
            }
        }
        let snapshot = build(app).snapshot();
        assert!(
            snapshot.get("shown").is_some(),
            "the fixture renders a node"
        );
        assert!(snapshot.get("gone").is_none());
        assert!(snapshot.get("inner").is_none());
        assert!(snapshot.nodes().all(|node| !node.id.contains("head")));
    }

    #[test]
    fn a_disabled_button_reads_not_enabled() {
        fn app() -> Element {
            rsx! {
                button { id: "off", disabled: true, "Off" }
                button { id: "on", "On" }
                div { id: "plain" }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(node(&snapshot, "off").state.enabled, Some(false));
        assert_eq!(node(&snapshot, "on").state.enabled, Some(true));
        assert_eq!(node(&snapshot, "plain").state.enabled, None);
    }

    #[test]
    fn a_checkbox_reads_checked() {
        fn app() -> Element {
            rsx! {
                input { id: "yes", r#type: "checkbox", checked: true }
                input { id: "no", r#type: "checkbox" }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(node(&snapshot, "yes").state.checked, Some(true));
        assert_eq!(node(&snapshot, "no").state.checked, Some(false));
        assert_eq!(node(&snapshot, "no").state.value, None);
    }

    #[test]
    fn a_text_input_reads_its_value() {
        fn app() -> Element {
            rsx! {
                input { id: "field", value: "typed" }
                button { id: "press", "Press" }
            }
        }
        let snapshot = build(app).snapshot();
        let field = node(&snapshot, "field");
        assert_eq!(field.state.value.as_deref(), Some("typed"));
        assert_eq!(field.state.checked, None);
        assert_eq!(node(&snapshot, "press").state.value, None);
    }

    #[test]
    fn text_runs_are_never_nodes() {
        fn app() -> Element {
            rsx! { p { id: "line", "words", span { "more" } } }
        }
        let snapshot = build(app).snapshot();
        assert!(snapshot.nodes().count() > 0, "the fixture renders nodes");
        assert!(snapshot.nodes().all(|node| node.role != Role::TextRun));
        assert_eq!(node(&snapshot, "line").children.len(), 1, "only the span");
    }

    #[test]
    fn every_id_is_the_element_id_of_its_element() {
        fn app() -> Element {
            rsx! {
                div { id: "box",
                    button { "One" }
                    button { "Two" }
                }
            }
        }
        let doc = build(app);
        let snapshot = doc.snapshot();
        let ids: Vec<String> = snapshot.nodes().map(|node| node.id.clone()).collect();
        let expected: Vec<String> = doc
            .element_ids()
            .into_iter()
            .map(|(_, id)| id)
            .filter(|id| !id.contains("head"))
            .collect();
        assert_eq!(ids, expected);
        assert_eq!(snapshot.roots.len(), 1);
        assert_eq!(snapshot.roots[0].id, "/html:0");
    }
}
