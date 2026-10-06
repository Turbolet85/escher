//! The text form of a [`Snapshot`]: one line per node, nested by indent, each line carrying
//! the node's role, name, stable element id, state and bounds — and the size budget one
//! screen's text stays under.

use std::fmt::Write;

use crate::snapshot::{Snapshot, SnapshotNode};

/// The ceiling for one screen's text, in bytes of [`Snapshot::to_text`]'s result.
///
/// The agent client warns when a tool result exceeds 10,000 tokens, caps one at 25,000
/// tokens and saves one longer than 50,000 characters to a file. A text of N bytes holds at
/// most N characters and at most N tokens, so a text inside this budget is inside all three
/// whatever the tokenizer. [`Snapshot::to_text`] never truncates or pages: the budget is a
/// ceiling a caller compares against, not a behaviour.
pub const SNAPSHOT_TEXT_BUDGET: usize = 10_000;

impl Snapshot {
    /// The snapshot as one text: a line per node in pre-order (the order of
    /// [`Snapshot::nodes`]), each ended by a line break, a child indented two spaces deeper
    /// than its parent. A snapshot with no roots is the empty text.
    ///
    /// A line reads, left to right: the role in its `Debug` spelling; the name as a quoted
    /// string; `id=` and the id quoted; the state tokens the node holds, in this order —
    /// `enabled` or `disabled`, `checked` or `unchecked`, `value=` and the value quoted,
    /// `focused`; then `@x,y widthxheight`, each number the shortest decimal that reads back
    /// as the same bound. A quoted string is written in `str`'s `Debug` form, so it never
    /// holds a line break or a bare `"`, and an apostrophe stays as it is.
    ///
    /// ```text
    /// GenericContainer "" id="form" @0,0 800x64.5
    ///   TextInput "Name:" id="who" enabled value="Ada" focused @8,8 200x24
    ///   CheckBox "Agree" id="agree" enabled unchecked @8,40 16x16
    ///   Button "Save" id="save" disabled @216,8 64x24
    /// ```
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        let mut stack: Vec<(&SnapshotNode, usize)> =
            self.roots.iter().rev().map(|root| (root, 0)).collect();
        while let Some((node, depth)) = stack.pop() {
            write_line(&mut text, node, depth);
            stack.extend(node.children.iter().rev().map(|child| (child, depth + 1)));
        }
        text
    }
}

fn write_line(text: &mut String, node: &SnapshotNode, depth: usize) {
    for _ in 0..depth {
        text.push_str("  ");
    }
    write!(text, "{:?} {:?} id={:?}", node.role, node.name, node.id).unwrap();
    match node.state.enabled {
        Some(true) => text.push_str(" enabled"),
        Some(false) => text.push_str(" disabled"),
        None => {}
    }
    match node.state.checked {
        Some(true) => text.push_str(" checked"),
        Some(false) => text.push_str(" unchecked"),
        None => {}
    }
    if let Some(value) = &node.state.value {
        write!(text, " value={value:?}").unwrap();
    }
    if node.state.focused {
        text.push_str(" focused");
    }
    let bounds = node.bounds;
    writeln!(
        text,
        " @{},{} {}x{}",
        bounds.x, bounds.y, bounds.width, bounds.height
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DioxusDocument, MASKED_VALUE, NodeState};
    use accesskit::Role;
    use blitz_dom::{BoundingRect, Document, DocumentConfig};
    use dioxus::prelude::*;

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc.inner_mut().resolve(0.0);
        doc
    }

    fn rect(x: f64, y: f64, width: f64, height: f64) -> BoundingRect {
        BoundingRect {
            x,
            y,
            width,
            height,
        }
    }

    fn leaf(role: Role, name: &str, id: &str) -> SnapshotNode {
        SnapshotNode {
            id: id.to_string(),
            role,
            name: name.to_string(),
            state: NodeState::default(),
            bounds: rect(0.0, 0.0, 10.0, 10.0),
            children: Vec::new(),
        }
    }

    fn with_children(mut node: SnapshotNode, children: Vec<SnapshotNode>) -> SnapshotNode {
        node.children = children;
        node
    }

    fn with_state(mut node: SnapshotNode, state: NodeState) -> SnapshotNode {
        node.state = state;
        node
    }

    fn text_of(roots: Vec<SnapshotNode>) -> String {
        Snapshot { roots }.to_text()
    }

    #[test]
    fn a_node_writes_role_name_id_and_bounds() {
        let mut node = leaf(Role::Button, "Save", "save");
        node.bounds = rect(216.0, 8.0, 64.0, 24.0);
        assert_eq!(
            text_of(vec![node]),
            "Button \"Save\" id=\"save\" @216,8 64x24\n"
        );
        assert_eq!(
            text_of(vec![leaf(Role::GenericContainer, "", "/html:0/body:0")]),
            "GenericContainer \"\" id=\"/html:0/body:0\" @0,0 10x10\n"
        );
    }

    #[test]
    fn state_tokens_follow_the_model() {
        let line =
            |state: NodeState| text_of(vec![with_state(leaf(Role::Unknown, "", "n"), state)]);
        let state = NodeState::default;
        assert_eq!(line(state()), "Unknown \"\" id=\"n\" @0,0 10x10\n");
        assert_eq!(
            line(NodeState {
                enabled: Some(true),
                ..state()
            }),
            "Unknown \"\" id=\"n\" enabled @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                enabled: Some(false),
                ..state()
            }),
            "Unknown \"\" id=\"n\" disabled @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                checked: Some(true),
                ..state()
            }),
            "Unknown \"\" id=\"n\" checked @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                checked: Some(false),
                ..state()
            }),
            "Unknown \"\" id=\"n\" unchecked @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                value: Some("typed".to_string()),
                ..state()
            }),
            "Unknown \"\" id=\"n\" value=\"typed\" @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                focused: true,
                ..state()
            }),
            "Unknown \"\" id=\"n\" focused @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                enabled: Some(false),
                focused: true,
                ..state()
            }),
            "Unknown \"\" id=\"n\" disabled focused @0,0 10x10\n"
        );
        assert_eq!(
            line(NodeState {
                enabled: Some(true),
                checked: Some(false),
                value: Some("on".to_string()),
                focused: true,
            }),
            "Unknown \"\" id=\"n\" enabled unchecked value=\"on\" focused @0,0 10x10\n"
        );
    }

    #[test]
    fn children_are_indented_in_pre_order() {
        let first = with_children(
            leaf(Role::GenericContainer, "", "a"),
            vec![
                with_children(
                    leaf(Role::List, "", "a1"),
                    vec![
                        leaf(Role::ListItem, "One", "a1x"),
                        leaf(Role::ListItem, "Two", "a1y"),
                    ],
                ),
                leaf(Role::Button, "Go", "a2"),
            ],
        );
        let second = with_children(
            leaf(Role::GenericContainer, "", "b"),
            vec![leaf(Role::Paragraph, "End", "b1")],
        );
        let snapshot = Snapshot {
            roots: vec![first, second],
        };
        let expected = concat!(
            "GenericContainer \"\" id=\"a\" @0,0 10x10\n",
            "  List \"\" id=\"a1\" @0,0 10x10\n",
            "    ListItem \"One\" id=\"a1x\" @0,0 10x10\n",
            "    ListItem \"Two\" id=\"a1y\" @0,0 10x10\n",
            "  Button \"Go\" id=\"a2\" @0,0 10x10\n",
            "GenericContainer \"\" id=\"b\" @0,0 10x10\n",
            "  Paragraph \"End\" id=\"b1\" @0,0 10x10\n",
        );
        assert_eq!(snapshot.to_text(), expected);
        let ids: Vec<&str> = snapshot.nodes().map(|node| node.id.as_str()).collect();
        assert_eq!(ids, ["a", "a1", "a1x", "a1y", "a2", "b", "b1"]);
    }

    #[test]
    fn strings_are_quoted_and_escaped() {
        const AWKWARD: &str = "a\"b\\c\nd\te";
        let node = with_state(
            leaf(Role::TextInput, AWKWARD, AWKWARD),
            NodeState {
                value: Some(AWKWARD.to_string()),
                ..NodeState::default()
            },
        );
        assert_eq!(
            text_of(vec![node]),
            concat!(
                r#"TextInput "a\"b\\c\nd\te" id="a\"b\\c\nd\te" value="a\"b\\c\nd\te""#,
                " @0,0 10x10\n"
            )
        );

        assert_eq!(
            text_of(vec![leaf(Role::Button, "Don't save", "discard")]),
            "Button \"Don't save\" id=\"discard\" @0,0 10x10\n"
        );

        assert_eq!(
            text_of(vec![leaf(Role::Button, "", "list//html:0/div:2 [x: y]")]),
            "Button \"\" id=\"list//html:0/div:2 [x: y]\" @0,0 10x10\n"
        );

        let masked = with_state(
            leaf(Role::PasswordInput, "", "pw"),
            NodeState {
                value: Some(MASKED_VALUE.to_string()),
                ..NodeState::default()
            },
        );
        assert_eq!(
            text_of(vec![masked]),
            "PasswordInput \"\" id=\"pw\" value=\"••••••••\" @0,0 10x10\n"
        );

        let empty = with_state(
            leaf(Role::TextInput, "", "blank"),
            NodeState {
                value: Some(String::new()),
                ..NodeState::default()
            },
        );
        assert_eq!(
            text_of(vec![empty]),
            "TextInput \"\" id=\"blank\" value=\"\" @0,0 10x10\n"
        );
    }

    #[test]
    fn bounds_print_the_shortest_decimal() {
        let mut node = leaf(Role::Image, "", "pic");
        node.bounds = rect(8.0, 12.5, 801.0 / 64.0, 0.0);
        assert_eq!(
            text_of(vec![node.clone()]),
            "Image \"\" id=\"pic\" @8,12.5 12.515625x0\n"
        );
        node.bounds = rect(-4.0, -0.5, 100.0, 1.0 / 64.0);
        assert_eq!(
            text_of(vec![node]),
            "Image \"\" id=\"pic\" @-4,-0.5 100x0.015625\n"
        );
    }

    #[test]
    fn an_empty_snapshot_is_the_empty_text() {
        assert_eq!(text_of(Vec::new()), "");
    }

    #[test]
    fn the_text_of_a_document_has_one_line_per_node() {
        fn app() -> Element {
            rsx! {
                div { id: "box",
                    label { r#for: "who", "Name:" }
                    input { id: "who", value: "Ada" }
                    button { id: "go", disabled: true, "Go" }
                }
            }
        }
        let doc = build(app);
        let snapshot = doc.snapshot();
        let text = snapshot.to_text();
        assert!(snapshot.get("go").is_some(), "the fixture renders nodes");
        assert_eq!(text.lines().count(), snapshot.nodes().count());
        assert!(text.ends_with('\n'));
        assert_eq!(text, doc.snapshot().to_text(), "a second call");
        let who = text
            .lines()
            .find(|line| line.contains("id=\"who\""))
            .expect("the input has a line");
        assert!(
            who.trim_start()
                .starts_with("TextInput \"Name:\" id=\"who\" enabled value=\"Ada\" @"),
            "{who:?}"
        );
        let go = text
            .lines()
            .find(|line| line.contains("id=\"go\""))
            .expect("the button has a line");
        assert!(
            go.trim_start()
                .starts_with("Button \"Go\" id=\"go\" disabled @"),
            "{go:?}"
        );
    }
}
