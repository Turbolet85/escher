//! The snapshot model: the screen of a [`DioxusDocument`] read as a tree whose every node is
//! an element carrying its stable element id, role, accessible name, state and bounds.

use std::collections::{HashMap, HashSet};

use accesskit::{Node as TreeNode, NodeId as TreeNodeId, Role, Toggled, TreeUpdate};
use blitz_dom::{BaseDocument, BoundingRect, Document, ElementData, Node, NodeId, local_name};

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
            let node = self.doc.get_node(node_id)?;
            let element = node.element_data()?;
            // A select's option is seen in its select's box, the one it shows or not.
            let boxed_by = match select_parent(self.doc, node) {
                Some(select) => select,
                None => node_id,
            };
            let bounds = self.doc.get_client_bounding_rect(boxed_by)?;
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

/// The parent of an `option` whose parent is a `select`; `None` for any other node.
fn select_parent(doc: &BaseDocument, node: &Node) -> Option<NodeId> {
    let parent = node.parent?;
    (node.data.is_element_with_tag_name(&local_name!("option"))
        && doc
            .get_node(parent)?
            .data
            .is_element_with_tag_name(&local_name!("select")))
    .then_some(parent)
}

/// The form reader's value of a text-entry control: its editor's text or a range input's
/// current value, else its `value` attribute, and a single-choice select's chosen option's
/// value. A masked control ([`is_masked_input`]) reads [`MASKED_VALUE`] in place of a
/// non-empty one.
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

    /// The value `id`'s node reads. Names only the id on failure.
    #[track_caller]
    fn value_of<'s>(snapshot: &'s Snapshot, id: &str) -> Option<&'s str> {
        snapshot
            .get(id)
            .unwrap_or_else(|| panic!("{id:?} is a snapshot node"))
            .state
            .value
            .as_deref()
    }

    #[test]
    fn a_password_input_reads_a_masked_value() {
        const SECRET: &str = "synthetic-pw-7Qz";
        fn app() -> Element {
            rsx! {
                input { id: "filled", r#type: "password", value: SECRET }
                input { id: "empty", r#type: "password" }
                input { id: "upper", r#type: "PASSWORD", value: SECRET }
            }
        }
        let doc = build(app);
        let editor = |id: &str| {
            let base = doc.inner();
            let node_id = base.query_selector(&format!("#{id}")).ok().flatten()?;
            let data = base.get_node(node_id)?.element_data()?.text_input_data()?;
            Some(data.editor.text().to_string())
        };
        assert!(
            editor("filled").as_deref() == Some(SECRET),
            "the fixture's password input holds the text"
        );
        assert!(
            editor("empty").as_deref() == Some(""),
            "the fixture's empty password input holds none"
        );
        let upper_attribute = {
            let base = doc.inner();
            base.query_selector("#upper")
                .ok()
                .flatten()
                .and_then(|node_id| base.get_node(node_id)?.attr(local_name!("value")))
                .map(str::to_string)
        };
        assert!(
            upper_attribute.as_deref() == Some(SECRET),
            "the fixture's upper-case password input holds the text"
        );

        let snapshot = doc.snapshot();
        assert!(
            value_of(&snapshot, "filled") == Some(MASKED_VALUE),
            "a password holding text reads the mask"
        );
        assert!(
            value_of(&snapshot, "empty") == Some(""),
            "an empty password reads empty"
        );
        assert!(
            value_of(&snapshot, "upper") == Some(MASKED_VALUE),
            "the type is compared case-insensitively"
        );
        for node in snapshot.nodes() {
            let carried = [
                Some(node.id.as_str()),
                Some(node.name.as_str()),
                node.state.value.as_deref(),
            ];
            assert!(
                carried
                    .into_iter()
                    .flatten()
                    .all(|field| !field.contains(SECRET)),
                "a field of a snapshot node holds the typed text"
            );
        }
        assert_eq!(MASKED_VALUE.chars().count(), 8);
        assert!(MASKED_VALUE.chars().all(|c| c == '\u{2022}'));
    }

    #[test]
    fn a_file_input_reads_a_masked_value() {
        const PATH: &str = "/synthetic/dir/chosen-report.txt";
        fn app() -> Element {
            rsx! {
                input { id: "filled", r#type: "file", value: PATH }
                input { id: "empty", r#type: "file" }
                input { id: "upper", r#type: "FILE", value: PATH }
            }
        }
        let doc = build(app);
        let attribute = |id: &str| {
            let base = doc.inner();
            base.query_selector(&format!("#{id}"))
                .ok()
                .flatten()
                .and_then(|node_id| base.get_node(node_id)?.attr(local_name!("value")))
                .map(str::to_string)
        };
        assert!(
            attribute("filled").as_deref() == Some(PATH),
            "the fixture's file input holds the path"
        );
        assert!(
            attribute("empty").is_none(),
            "the fixture's empty file input holds no value"
        );
        assert!(
            attribute("upper").as_deref() == Some(PATH),
            "the fixture's upper-case file input holds the path"
        );

        let snapshot = doc.snapshot();
        assert!(
            value_of(&snapshot, "filled") == Some(MASKED_VALUE),
            "a file input holding a value reads the mask"
        );
        assert!(
            value_of(&snapshot, "empty").is_none(),
            "a file input with no value reads none"
        );
        assert!(
            value_of(&snapshot, "upper") == Some(MASKED_VALUE),
            "the type is compared case-insensitively"
        );
        for node in snapshot.nodes() {
            let carried = [
                Some(node.id.as_str()),
                Some(node.name.as_str()),
                node.state.value.as_deref(),
            ];
            assert!(
                carried
                    .into_iter()
                    .flatten()
                    .all(|field| !field.contains(PATH)),
                "a field of a snapshot node holds the path"
            );
        }
    }

    /// The crate's unit tests carry no HTML parser, so the two attributes are written through
    /// the mutator, as parsed markup leaves them: the bridge itself never writes a falsy
    /// `disabled`.
    #[test]
    fn a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value() {
        fn app() -> Element {
            rsx! {
                button { id: "falsy", "A" }
                button { id: "bare", "B" }
            }
        }
        let mut doc = build(app);
        for (id, written) in [("falsy", "false"), ("bare", "")] {
            let mut base = doc.inner_mut();
            let node_id = base
                .query_selector(&format!("#{id}"))
                .ok()
                .flatten()
                .unwrap_or_else(|| panic!("the fixture renders {id:?}"));
            base.mutate()
                .set_attribute(node_id, crate::qual_name("disabled", None), written);
        }
        doc.inner_mut().resolve(0.0);
        for (id, written) in [("falsy", "false"), ("bare", "")] {
            let base = doc.inner();
            let node_id = base
                .query_selector(&format!("#{id}"))
                .ok()
                .flatten()
                .unwrap_or_else(|| panic!("the fixture renders {id:?}"));
            let node = base.get_node(node_id).expect("a queried node resolves");
            assert_eq!(
                node.attr(local_name!("disabled")),
                Some(written),
                "{id:?} carries the disabled attribute"
            );
            assert!(
                !node.is_focussable(),
                "{id:?}: the focusability reader takes it as disabled"
            );
        }

        let snapshot = doc.snapshot();
        for id in ["falsy", "bare"] {
            let enabled = snapshot
                .get(id)
                .unwrap_or_else(|| panic!("{id:?} is a snapshot node"))
                .state
                .enabled;
            assert_eq!(enabled, Some(false), "{id:?}");
        }
    }

    #[test]
    fn a_radio_reads_checked() {
        fn app() -> Element {
            rsx! {
                input { id: "picked", r#type: "radio", name: "plan", checked: true }
                input { id: "other", r#type: "radio", name: "plan" }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(node(&snapshot, "picked").role, Role::RadioButton);
        assert_eq!(node(&snapshot, "picked").state.checked, Some(true));
        assert_eq!(node(&snapshot, "other").state.checked, Some(false));
    }

    #[test]
    fn a_range_input_reads_its_current_value() {
        fn app() -> Element {
            rsx! { input { id: "level", r#type: "range", value: "15" } }
        }
        let doc = build(app);
        let snapshot = doc.snapshot();
        assert_eq!(
            snapshot.get("level").map(|node| node.role),
            Some(Role::Slider),
            "the fixture renders a range input"
        );
        assert!(
            value_of(&snapshot, "level") == Some("15"),
            "a range input reads its current value"
        );
    }

    #[test]
    fn a_stepped_uncontrolled_range_input_reads_its_moved_value() {
        fn app() -> Element {
            rsx! { input { id: "level", r#type: "range", min: "0", max: "30", value: "15" } }
        }
        let mut doc = build(app);
        let level = {
            let base = doc.inner();
            let Some(level) = base.query_selector("#level").ok().flatten() else {
                panic!("the fixture renders the range input");
            };
            assert!(
                base.get_node(level)
                    .and_then(|node| node.element_data())
                    .and_then(ElementData::range_input_data)
                    .is_some(),
                "the range input holds its current value"
            );
            level
        };

        doc.inner_mut().set_focus_to(level);
        doc.handle_ui_event(blitz_traits::events::UiEvent::KeyDown(
            blitz_traits::events::BlitzKeyEvent {
                key: ::keyboard_types::Key::Named(::keyboard_types::NamedKey::ArrowRight),
                code: ::keyboard_types::Code::Unidentified,
                modifiers: ::keyboard_types::Modifiers::empty(),
                location: ::keyboard_types::Location::Standard,
                is_auto_repeating: false,
                is_composing: false,
                state: blitz_traits::events::KeyState::Pressed,
                text: None,
            },
        ));
        doc.inner_mut().resolve(0.0);

        assert!(value_of(&doc.snapshot(), "level") == Some("16"));
        assert_eq!(
            attribute(&doc, "level", local_name!("value")).as_deref(),
            Some("15"),
            "the value attribute keeps its boot value"
        );
    }

    #[test]
    fn a_password_or_file_input_is_masked_and_nothing_else_is() {
        fn app() -> Element {
            rsx! {
                input { id: "password", r#type: "password" }
                input { id: "upper", r#type: "PassWord" }
                input { id: "file", r#type: "file" }
                input { id: "text", r#type: "text" }
                input { id: "untyped" }
                textarea { id: "area" }
                button { id: "button", r#type: "password", "Go" }
            }
        }
        let rows = [
            ("password", true),
            ("upper", true),
            ("file", true),
            ("text", false),
            ("untyped", false),
            ("area", false),
            ("button", false),
        ];
        assert_eq!(rows.len(), 7);
        let doc = build(app);
        let base = doc.inner();
        for (row, (id, masked)) in rows.into_iter().enumerate() {
            let element = base
                .query_selector(&format!("#{id}"))
                .ok()
                .flatten()
                .and_then(|node_id| base.get_node(node_id))
                .and_then(|node| node.element_data());
            assert!(
                element.is_some_and(|element| is_masked_input(element) == masked),
                "row {row}"
            );
        }
    }

    /// What `id`'s element holds as the attribute `name`.
    fn attribute(doc: &DioxusDocument, id: &str, name: blitz_dom::LocalName) -> Option<String> {
        let base = doc.inner();
        let node_id = base.query_selector(&format!("#{id}")).ok().flatten()?;
        base.get_node(node_id)?.attr(name).map(str::to_string)
    }

    /// Each value an `aria-selected` is written with — `None` writes no attribute — and what
    /// its element reads as selected.
    const SELECTED: [(Option<&str>, Option<bool>); 7] = [
        (Some("true"), Some(true)),
        (Some("false"), Some(false)),
        (None, None),
        (Some(""), None),
        (Some("undefined"), None),
        (Some("TRUE"), None),
        (Some("no-such-value"), None),
    ];

    #[test]
    fn an_aria_selected_value_reads_selected() {
        fn app() -> Element {
            rsx! {
                for (row, (value, _)) in SELECTED.into_iter().enumerate() {
                    div { id: "row-{row}", role: "option", "aria-selected": value }
                }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(SELECTED.len(), 7);
        for (row, (_, stated)) in SELECTED.into_iter().enumerate() {
            let read = node(&snapshot, &format!("row-{row}"));
            assert_eq!(read.role, Role::ListBoxOption, "row {row}");
            assert_eq!(read.state.selected, stated, "row {row}");
            assert_eq!(
                (read.state.pressed, read.state.invalid),
                (None, false),
                "row {row}"
            );
        }
    }

    /// Each value an `aria-pressed` is written with and what its element reads as pressed: a
    /// mixed toggle reads not pressed.
    const PRESSED: [(Option<&str>, Option<bool>); 7] = [
        (Some("true"), Some(true)),
        (Some("false"), Some(false)),
        (Some("mixed"), Some(false)),
        (None, None),
        (Some(""), None),
        (Some("undefined"), None),
        (Some("no-such-value"), None),
    ];

    #[test]
    fn an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not() {
        fn app() -> Element {
            rsx! {
                for (row, (value, _)) in PRESSED.into_iter().enumerate() {
                    button { id: "row-{row}", "aria-pressed": value }
                }
                button { id: "bridged", aria_pressed: false }
                input { id: "native", r#type: "checkbox", checked: true }
                input { id: "aria-box", r#type: "checkbox", "aria-checked": "true" }
                div { id: "aria-plain", "aria-checked": "true" }
            }
        }
        let doc = build(app);
        let snapshot = doc.snapshot();
        assert_eq!(PRESSED.len(), 7);
        for (row, (_, stated)) in PRESSED.into_iter().enumerate() {
            let state = &node(&snapshot, &format!("row-{row}")).state;
            assert_eq!(state.pressed, stated, "row {row}");
            assert_eq!(
                (state.selected, state.invalid, state.checked),
                (None, false, None),
                "row {row}"
            );
        }

        assert_eq!(
            attribute(&doc, "bridged", local_name!("aria-pressed")).as_deref(),
            Some("false"),
            "the bridge writes a false `bool` of this attribute as its text"
        );
        assert_eq!(node(&snapshot, "bridged").state.pressed, Some(false));

        let native = &node(&snapshot, "native").state;
        assert_eq!((native.checked, native.pressed), (Some(true), None));
        for id in ["aria-box", "aria-plain"] {
            assert_eq!(
                attribute(&doc, id, local_name!("aria-checked")).as_deref(),
                Some("true"),
                "{id:?} carries the attribute"
            );
        }
        let aria_box = &node(&snapshot, "aria-box").state;
        assert_eq!((aria_box.checked, aria_box.pressed), (Some(false), None));
        let aria_plain = &node(&snapshot, "aria-plain").state;
        assert_eq!((aria_plain.checked, aria_plain.pressed), (None, None));
    }

    #[test]
    fn an_aria_disabled_button_still_reads_enabled() {
        fn app() -> Element {
            rsx! { button { id: "aria-off", "aria-disabled": "true", "Off" } }
        }
        let doc = build(app);
        assert_eq!(
            attribute(&doc, "aria-off", local_name!("aria-disabled")).as_deref(),
            Some("true"),
            "the fixture's button carries the attribute"
        );
        let tree = doc.accessibility_tree();
        let node_id = doc.inner().query_selector("#aria-off").ok().flatten();
        let disabled_in_tree = node_id.is_some_and(|node_id| {
            tree.nodes
                .iter()
                .any(|(id, node)| id.0 == node_id.as_u64() && node.is_disabled())
        });
        assert!(disabled_in_tree, "the tree reads the button disabled");
        assert_eq!(
            node(&doc.snapshot(), "aria-off").state.enabled,
            Some(true),
            "the snapshot reads the disabled attribute only"
        );
    }

    #[test]
    fn a_switch_reads_its_aria_checked_as_checked_and_not_pressed() {
        fn app() -> Element {
            rsx! { div { id: "toggle", role: "switch", "aria-checked": "true" } }
        }
        let snapshot = build(app).snapshot();
        let toggle = node(&snapshot, "toggle");
        assert_eq!(toggle.role, Role::Switch, "the fixture renders a switch");
        assert_eq!(
            (toggle.state.checked, toggle.state.pressed),
            (Some(true), None)
        );
    }

    #[test]
    fn a_toggle_button_still_reads_its_aria_pressed_as_pressed() {
        fn app() -> Element {
            rsx! { div { id: "toggle", role: "button", "aria-pressed": "true" } }
        }
        let snapshot = build(app).snapshot();
        let toggle = node(&snapshot, "toggle");
        assert_eq!(toggle.role, Role::Button, "the fixture renders a button");
        assert_eq!(
            (toggle.state.checked, toggle.state.pressed),
            (None, Some(true))
        );
    }

    /// Each value an `aria-invalid` is written with and whether its element reads invalid: a
    /// grammar or a spelling error does, and so does a value outside the attribute's list.
    const INVALID: [(Option<&str>, bool); 7] = [
        (Some("true"), true),
        (Some("grammar"), true),
        (Some("spelling"), true),
        (Some("no-such-value"), true),
        (None, false),
        (Some(""), false),
        (Some("false"), false),
    ];

    #[test]
    fn an_aria_invalid_value_reads_invalid() {
        fn app() -> Element {
            rsx! {
                for (row, (value, _)) in INVALID.into_iter().enumerate() {
                    input { id: "row-{row}", "aria-invalid": value }
                }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(INVALID.len(), 7);
        for (row, (_, stated)) in INVALID.into_iter().enumerate() {
            let state = &node(&snapshot, &format!("row-{row}")).state;
            assert_eq!(state.invalid, stated, "row {row}");
            assert_eq!((state.selected, state.pressed), (None, None), "row {row}");
        }
    }

    /// One option's `(value attribute, text, selected)`.
    type SelectOption = (Option<&'static str>, &'static str, bool);

    /// Each select of the value table: its options as `(value attribute, text, selected)` and
    /// the value the select reads — a chosen option's `value` attribute, else its text stripped
    /// and collapsed, and with none written `selected` its first option.
    const SELECT_VALUES: [(&[SelectOption], Option<&str>); 4] = [
        (
            &[(Some("a"), "Apple", false), (Some("b"), "Banana", true)],
            Some("b"),
        ),
        (
            &[(None, "Apple", false), (None, "  Big\n Banana ", true)],
            Some("Big Banana"),
        ),
        (
            &[(Some("a"), "Apple", false), (Some("b"), "Banana", false)],
            Some("a"),
        ),
        (&[(Some(""), "Empty", true)], Some("")),
    ];

    #[test]
    fn a_select_reads_its_chosen_options_value() {
        fn app() -> Element {
            rsx! {
                for (row, (options, _)) in SELECT_VALUES.into_iter().enumerate() {
                    select { id: "row-{row}",
                        for (index, (value, text, selected)) in options.iter().copied().enumerate() {
                            option { id: "row-{row}-{index}", value, selected, "{text}" }
                        }
                    }
                }
            }
        }
        let snapshot = build(app).snapshot();
        assert_eq!(SELECT_VALUES.len(), 4);
        for (row, (_, expected)) in SELECT_VALUES.into_iter().enumerate() {
            let select = node(&snapshot, &format!("row-{row}"));
            assert_eq!(select.role, Role::ComboBox, "row {row}");
            assert_eq!(select.state.value.as_deref(), expected, "row {row}");
        }
    }

    #[test]
    fn a_selects_options_are_its_children_named_by_their_labels() {
        fn app() -> Element {
            rsx! {
                select { id: "pick",
                    option { id: "apple", value: "a", "Apple" }
                    option { id: "banana", value: "b", selected: true, "Banana" }
                    option { id: "cherry", value: "c", label: "Cherries", "Cherry" }
                }
            }
        }
        let snapshot = build(app).snapshot();
        let pick = node(&snapshot, "pick");
        let children: Vec<_> = pick
            .children
            .iter()
            .map(|child| {
                (
                    child.id.as_str(),
                    child.role,
                    child.name.as_str(),
                    child.state.selected,
                )
            })
            .collect();
        assert_eq!(
            children,
            [
                ("apple", Role::ListBoxOption, "Apple", Some(false)),
                ("banana", Role::ListBoxOption, "Banana", Some(true)),
                ("cherry", Role::ListBoxOption, "Cherries", Some(false)),
            ]
        );
        for child in &pick.children {
            assert_eq!(child.bounds, pick.bounds, "{}", child.id);
            assert_eq!(child.state.value, None, "{}", child.id);
        }
    }
}
