//! The actionable-key check: every element an agent can act on should read its author key, so
//! that its stable element id survives any edit of the code around it. An element that reads a
//! positional path instead is returned by [`DioxusDocument::unkeyed_actionable`]. The same rule
//! lists the screen's actionable nodes, [`DioxusDocument::actionable_nodes`].

use std::collections::HashMap;

use accesskit::Role;
use blitz_dom::{Document, Node, NodeId};
use rustc_hash::FxHashMap;

use crate::DioxusDocument;
use crate::element_id::{ElementFacts, element_facts};

/// An actionable element that reads a positional path where the rule asks for an author key.
/// Built by [`DioxusDocument::unkeyed_actionable`]. It names structure only: no field holds the
/// element's text, its accessible name or the value of one of its attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct UnkeyedActionable {
    /// The element's node.
    pub node: NodeId,
    /// The positional id the element reads (see [`DioxusDocument::element_id`]): an anchored,
    /// component or document path, so it holds a `/`.
    pub id: String,
    /// The element's tag name.
    pub tag: String,
    /// The role of the element's accessibility node, or `None` when the accessibility tree
    /// leaves the element out (a hidden element).
    pub role: Option<Role>,
    /// Whether the element is focusable.
    pub focusable: bool,
    /// Whether [`role`](Self::role) is an interactive one: a button, a link, a text or other
    /// input, a slider, a list or menu option and the like.
    pub interactive_role: bool,
    /// Whether the element's current listeners hold an event other than `mounted`.
    pub listener: bool,
}

impl UnkeyedActionable {
    /// What to change so the element reads an author key.
    pub fn remedy(&self) -> String {
        format!(
            "give the `{}` element reading `{}` an HTML `id` attribute that is non-empty, holds no `/` and is unique in the document",
            self.tag, self.id
        )
    }
}

/// One actionable node of the screen. Built by [`DioxusDocument::actionable_nodes`]. It holds
/// ids and a bool only: no `NodeId`, text, name or value.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionableNode {
    /// The node's stable element id, as its snapshot node reads it.
    pub id: String,
    /// The stable id of the element whose handler serves this node, as the UI graph reads it:
    /// a select for its options and groups. `None` where the node is its own.
    pub owner: Option<String>,
    /// Whether the id is an author key, which holds no `/`; every path id holds one.
    pub keyed: bool,
}

/// The three readings of the actionable rule for one element.
struct Readings {
    focusable: bool,
    interactive_role: bool,
    listener: bool,
}

impl Readings {
    fn of(node: &Node, role: Option<Role>, listeners: &[&'static str]) -> Self {
        Self {
            focusable: node.is_focussable(),
            interactive_role: role.is_some_and(is_interactive),
            listener: listeners.iter().any(|event| *event != "mounted"),
        }
    }

    fn actionable(&self) -> bool {
        self.focusable || self.interactive_role || self.listener
    }
}

impl DioxusDocument {
    /// Every actionable element whose stable element id is a positional path, in document
    /// pre-order; empty when every actionable element reads its author key.
    ///
    /// An element is actionable when it is focusable, or its accessibility node has an
    /// interactive role, or its current listeners hold an event other than `mounted` — the
    /// vdom's own reading, so a listener a re-render dropped is not counted. Ids are those of
    /// [`element_ids`](Self::element_ids) and roles those of the document's accessibility tree,
    /// so a disabled control is still held by its role and a hidden one by the other two
    /// readers.
    pub fn unkeyed_actionable(&self) -> Vec<UnkeyedActionable> {
        let roles = self.roles();
        let facts = element_facts(&self.vdom, &self.vdom_state, &self.inner.borrow());
        let doc = self.inner();
        facts
            .into_iter()
            .filter(|fact| fact.id.contains('/'))
            .filter_map(|fact| {
                let node = doc.get_node(fact.node)?;
                let element = node.element_data()?;
                let role = roles.get(&fact.node.as_u64()).copied();
                let readings = Readings::of(node, role, &fact.listeners);
                readings.actionable().then(|| UnkeyedActionable {
                    node: fact.node,
                    id: fact.id,
                    tag: element.name.local.to_string(),
                    role,
                    focusable: readings.focusable,
                    interactive_role: readings.interactive_role,
                    listener: readings.listener,
                })
            })
            .collect()
    }

    /// Every actionable node of the screen, in snapshot pre-order: each node of the UI graph
    /// ([`graph`](Self::graph)) that the rule of [`unkeyed_actionable`](Self::unkeyed_actionable)
    /// holds for, or whose graph `owner` is another element. Read on demand; it writes nothing.
    pub fn actionable_nodes(&self) -> Vec<ActionableNode> {
        let graph = self.graph();
        let roles = self.roles();
        let facts = element_facts(&self.vdom, &self.vdom_state, &self.inner.borrow());
        let by_id: FxHashMap<&str, &ElementFacts> =
            facts.iter().map(|fact| (fact.id.as_str(), fact)).collect();
        let doc = self.inner();
        graph
            .nodes
            .into_iter()
            .filter(|node| {
                node.owner.is_some()
                    || by_id.get(node.id.as_str()).is_some_and(|fact| {
                        doc.get_node(fact.node).is_some_and(|element| {
                            let role = roles.get(&fact.node.as_u64()).copied();
                            Readings::of(element, role, &fact.listeners).actionable()
                        })
                    })
            })
            .map(|node| ActionableNode {
                keyed: !node.id.contains('/'),
                id: node.id,
                owner: node.owner,
            })
            .collect()
    }

    /// The role of each node the accessibility tree keeps.
    fn roles(&self) -> HashMap<u64, Role> {
        self.accessibility_tree()
            .nodes
            .iter()
            .map(|(id, node)| (id.0, node.role()))
            .collect()
    }
}

/// The roles of the widgets an agent presses, toggles, selects or types into.
fn is_interactive(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::Link
            | Role::CheckBox
            | Role::RadioButton
            | Role::Slider
            | Role::SpinButton
            | Role::ScrollBar
            | Role::ComboBox
            | Role::ListBox
            | Role::ListBoxOption
            | Role::MenuItem
            | Role::MenuItemCheckBox
            | Role::MenuItemRadio
            | Role::Tab
            | Role::TreeItem
            | Role::DisclosureTriangle
            | Role::MultilineTextInput
            | Role::TextInput
            | Role::ColorWell
            | Role::DateInput
            | Role::DateTimeInput
            | Role::EmailInput
            | Role::NumberInput
            | Role::PasswordInput
            | Role::SearchInput
            | Role::PhoneNumberInput
            | Role::TimeInput
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_registry::tests::{click, node};
    use blitz_dom::DocumentConfig;
    use dioxus::prelude::*;

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc.inner_mut().resolve(0.0);
        doc
    }

    fn ids(found: &[UnkeyedActionable]) -> Vec<&str> {
        found.iter().map(|entry| entry.id.as_str()).collect()
    }

    #[test]
    fn an_unkeyed_actionable_element_is_reported_with_its_remedy() {
        fn app() -> Element {
            rsx! {
                button { "One" }
                button { disabled: true, "Two" }
                div { onclick: |_| {}, "Three" }
                button { id: "go", "Four" }
                button { id: "go", "Five" }
            }
        }
        let doc = build(app);
        let found = doc.unkeyed_actionable();
        assert_eq!(
            ids(&found),
            ["/button:0", "/button:1", "/div:0", "/button:3"]
        );

        let readers: Vec<(bool, bool, bool)> = found
            .iter()
            .map(|entry| (entry.focusable, entry.interactive_role, entry.listener))
            .collect();
        assert_eq!(
            readers,
            [
                (true, true, false),
                (false, true, false),
                (false, false, true),
                (true, true, false),
            ]
        );
        let roles: Vec<Option<Role>> = found.iter().map(|entry| entry.role).collect();
        assert_eq!(
            roles,
            [
                Some(Role::Button),
                Some(Role::Button),
                Some(Role::GenericContainer),
                Some(Role::Button),
            ]
        );

        for entry in &found {
            assert_eq!(doc.element_id(entry.node).as_ref(), Some(&entry.id));
            let remedy = entry.remedy();
            assert!(remedy.contains(&entry.id), "{remedy:?} names the element");
            assert!(remedy.contains(&entry.tag), "{remedy:?} names the tag");
            assert!(remedy.contains("`id`"), "{remedy:?} names the fix");
        }
    }

    #[test]
    fn a_keyed_or_inert_element_is_not_reported() {
        fn app() -> Element {
            rsx! {
                button { id: "press", onclick: |_| {}, "Press" }
                input { id: "field" }
                div { id: "row", onclick: |_| {}, "Row" }
                div {
                    p { "Words" }
                    label { r#for: "field", "Field" }
                }
            }
        }
        let doc = build(app);
        let all: Vec<String> = doc.element_ids().into_iter().map(|(_, id)| id).collect();
        for expected in [
            "press",
            "field",
            "row",
            "/div:1",
            "/div:1/p:0",
            "/div:1/label:0",
        ] {
            assert!(
                all.iter().any(|id| id == expected),
                "{expected:?} in {all:?}"
            );
        }
        assert_eq!(doc.unkeyed_actionable(), []);
    }

    #[test]
    fn a_hidden_or_disabled_control_is_still_held() {
        fn app() -> Element {
            rsx! {
                button { style: "display: none", onclick: |_| {}, "Hidden" }
                input { disabled: true }
            }
        }
        let found = build(app).unkeyed_actionable();
        assert_eq!(ids(&found), ["/button:0", "/input:0"]);

        let hidden = &found[0];
        assert_eq!(
            hidden.role, None,
            "the tree leaves a display: none element out"
        );
        assert!(!hidden.interactive_role);
        assert!(hidden.focusable && hidden.listener);

        let disabled = &found[1];
        assert_eq!(disabled.role, Some(Role::TextInput));
        assert!(disabled.interactive_role);
        assert!(!disabled.focusable && !disabled.listener);
    }

    fn listed(doc: &DioxusDocument) -> Vec<(String, Option<String>, bool)> {
        doc.actionable_nodes()
            .into_iter()
            .map(|node| (node.id, node.owner, node.keyed))
            .collect()
    }

    #[test]
    fn a_listener_removed_on_a_re_render_is_read_by_neither_reader() {
        fn page() -> Element {
            let mut armed = use_signal(|| true);
            let spread: Vec<Attribute> = if armed() {
                vec![dioxus::html::onclick(move |_: MouseEvent| {})]
            } else {
                Vec::new()
            };
            rsx! {
                div { ..spread, "Toggled" }
                button { id: "disarm", onclick: move |_| armed.set(false), "Disarm" }
                div { id: "held", onmounted: move |_| {}, "Held" }
            }
        }
        let mut doc = build(page);
        assert_eq!(ids(&doc.unkeyed_actionable()), ["/div:0"]);
        assert_eq!(
            listed(&doc),
            [
                ("/div:0".to_string(), None, false),
                ("disarm".to_string(), None, true),
            ],
            "a mounted listener alone makes no node actionable"
        );

        let disarm = node(&doc, "disarm");
        click(&mut doc, disarm);
        doc.inner_mut().resolve(0.0);
        let (toggled, _) = doc
            .element_ids()
            .into_iter()
            .find(|(_, id)| id == "/div:0")
            .expect("the toggled div stands");
        let marked = doc
            .inner()
            .get_node(toggled)
            .and_then(|node| node.element_data())
            .is_some_and(|element| {
                element
                    .attrs
                    .iter()
                    .any(|attr| *attr.name.local == *"data-dioxus-id")
            });
        assert!(
            marked,
            "the listener mark outlives the listener, so a mark reading would still count it"
        );
        assert_eq!(doc.unkeyed_actionable(), []);
        assert_eq!(listed(&doc), [("disarm".to_string(), None, true)]);
    }

    #[test]
    fn a_selects_options_and_group_are_listed_with_the_select_as_owner() {
        fn page() -> Element {
            rsx! {
                select { id: "pick", oninput: move |_| {},
                    option { id: "one", value: "1", "One" }
                    optgroup { id: "group", label: "More",
                        option { id: "two", value: "2", "Two" }
                    }
                }
            }
        }
        let pick = Some("pick".to_string());
        assert_eq!(
            listed(&build(page)),
            [
                ("pick".to_string(), None, true),
                ("one".to_string(), pick.clone(), true),
                ("group".to_string(), pick.clone(), true),
                ("two".to_string(), pick, true),
            ]
        );
    }

    #[test]
    fn a_positional_id_reads_unkeyed_and_an_author_key_keyed() {
        fn page() -> Element {
            rsx! {
                button { "Positional" }
                button { id: "named", "Named" }
                p { "Words" }
            }
        }
        let doc = build(page);
        assert_eq!(
            listed(&doc),
            [
                ("/button:0".to_string(), None, false),
                ("named".to_string(), None, true),
            ]
        );
        assert_eq!(ids(&doc.unkeyed_actionable()), ["/button:0"]);
    }

    #[test]
    fn an_inert_or_hidden_element_is_not_listed() {
        fn page() -> Element {
            rsx! {
                button { id: "shown", "Shown" }
                button { id: "hidden", style: "display: none", onclick: |_| {}, "Hidden" }
                div { inert: true,
                    button { id: "inert", onclick: |_| {}, "Inert" }
                }
            }
        }
        assert_eq!(listed(&build(page)), [("shown".to_string(), None, true)]);
    }

    #[test]
    fn the_report_names_structure_never_content() {
        const CONTENT: [&str; 5] = ["Secret", "Whisper", "hunter2", "loud", "Hint"];
        fn app() -> Element {
            rsx! {
                button { class: "loud", "aria-label": "Whisper", "Secret" }
                input { value: "hunter2", placeholder: "Hint" }
            }
        }
        let found = build(app).unkeyed_actionable();
        assert_eq!(ids(&found), ["/button:0", "/input:0"]);
        assert_eq!(found[0].tag, "button");
        assert_eq!(found[0].role, Some(Role::Button));
        assert_eq!(found[1].tag, "input");
        assert_eq!(found[1].role, Some(Role::TextInput));
        for entry in &found {
            let printed = format!("{entry:?} {}", entry.remedy());
            for content in CONTENT {
                assert!(!printed.contains(content), "{content:?} in {printed:?}");
            }
        }
    }
}
