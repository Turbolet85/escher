//! The actionable-key check: every element an agent can act on should read its author key, so
//! that its stable element id survives any edit of the code around it. An element that reads a
//! positional path instead is returned by [`DioxusDocument::unkeyed_actionable`].

use std::collections::HashMap;

use accesskit::Role;
use blitz_dom::{Document, NodeId};

use crate::DioxusDocument;

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
    /// Whether the element carries an event listener other than `onmounted`.
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

impl DioxusDocument {
    /// Every actionable element whose stable element id is a positional path, in document
    /// pre-order; empty when every actionable element reads its author key.
    ///
    /// An element is actionable when it is focusable, or its accessibility node has an
    /// interactive role, or it carries an event listener. Ids are those of
    /// [`element_ids`](Self::element_ids) and roles those of the document's accessibility tree,
    /// so a disabled control is still held by its role and a hidden one by the other two
    /// readers.
    pub fn unkeyed_actionable(&self) -> Vec<UnkeyedActionable> {
        let roles: HashMap<u64, Role> = self
            .accessibility_tree()
            .nodes
            .iter()
            .map(|(id, node)| (id.0, node.role()))
            .collect();
        let ids = self.element_ids();
        let doc = self.inner();
        ids.into_iter()
            .filter(|(_, id)| id.contains('/'))
            .filter_map(|(node_id, id)| {
                let node = doc.get_node(node_id)?;
                let element = node.element_data()?;
                let role = roles.get(&node_id.as_u64()).copied();
                let focusable = node.is_focussable();
                let interactive_role = role.is_some_and(is_interactive);
                let listener = element
                    .attrs
                    .iter()
                    .any(|attr| *attr.name.local == *"data-dioxus-id");
                (focusable || interactive_role || listener).then(|| UnkeyedActionable {
                    node: node_id,
                    id,
                    tag: element.name.local.to_string(),
                    role,
                    focusable,
                    interactive_role,
                    listener,
                })
            })
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
