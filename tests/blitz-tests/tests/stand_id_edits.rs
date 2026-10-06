//! An element left in place keeps its stable element id when the app's code around it changes.
//! The stand is compiled, so an edit is a before/after pair: two components of the same name in
//! two modules, booted with the stand's options in both layout modes. A lean task keeps every
//! id when its shell is edited; a keyed element keeps its id under every edit; an anchored
//! element keeps its id under an edit outside its anchor; a behaviour edit changes no id; and
//! the edits the grammar lets change an id are measured, on positional ids only.

use std::collections::HashSet;

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::{DioxusDocument, NodeId};
use seven_guis::stand::{self, LeanTask};
use seven_guis::tasks::counter::Counter;
use seven_guis::tasks::crud::Crud;
use seven_guis::tasks::flight_booker::FlightBooker;
use seven_guis::tasks::timer::{Timer, TimerTicks};

/// The shell a task is mounted in, as first written: the stand's `main#task-body`.
mod plain {
    use dioxus::prelude::*;

    #[component]
    pub fn TaskShell(children: Element) -> Element {
        rsx! {
            main { id: "task-body", {children} }
        }
    }
}

/// The same shell after an edit around the task: an earlier sibling of the task root's tag, a
/// wrapper around the task, and the body one level deeper.
mod edited {
    use dioxus::prelude::*;

    #[component]
    pub fn TaskShell(children: Element) -> Element {
        rsx! {
            div { class: "banner" }
            section {
                main { id: "task-body",
                    div { class: "earlier" }
                    div { class: "wrapper", {children} }
                }
            }
        }
    }
}

#[derive(Clone)]
struct ShellRoot {
    task: LeanTask,
    edited: bool,
}

fn shell_root(props: ShellRoot) -> Element {
    use_hook(|| provide_context(TimerTicks::default()));
    let task = match props.task {
        LeanTask::Counter => rsx! { Counter {} },
        LeanTask::FlightBooker => rsx! { FlightBooker {} },
        LeanTask::Timer => rsx! { Timer {} },
        LeanTask::Crud => rsx! { Crud {} },
    };
    if props.edited {
        rsx! { edited::TaskShell { {task} } }
    } else {
        rsx! { plain::TaskShell { {task} } }
    }
}

fn boot_in_shell(task: LeanTask, edited: bool, incremental: bool) -> Harness<DioxusDocument> {
    let vdom = VirtualDom::new_with_props(shell_root, ShellRoot { task, edited });
    Harness::from_vdom(vdom, stand::options(incremental))
}

fn task_root(task: LeanTask) -> &'static str {
    match task {
        LeanTask::Counter => ".counter-root",
        LeanTask::FlightBooker => ".flight-root",
        LeanTask::Timer => ".timer-root",
        LeanTask::Crud => ".crud-root",
    }
}

fn all_ids(harness: &Harness<DioxusDocument>) -> Vec<String> {
    harness
        .doc
        .element_ids()
        .into_iter()
        .map(|(_, id)| id)
        .collect()
}

/// The ids of `selector`'s element and of every element below it, in document order.
fn subtree_ids(harness: &Harness<DioxusDocument>, selector: &str) -> Vec<String> {
    let doc = harness.base();
    let mut inside: HashSet<NodeId> = HashSet::new();
    let mut stack = vec![harness.node(selector)];
    while let Some(id) = stack.pop() {
        inside.insert(id);
        stack.extend(doc.get_node(id).unwrap().children.iter().copied());
    }
    harness
        .doc
        .element_ids()
        .into_iter()
        .filter(|(node, _)| inside.contains(node))
        .map(|(_, id)| id)
        .collect()
}

#[track_caller]
fn id_of(harness: &Harness<DioxusDocument>, selector: &str) -> String {
    harness
        .doc
        .element_id(harness.node(selector))
        .unwrap_or_else(|| panic!("{selector:?} reads an id"))
}

#[test]
fn a_task_keeps_every_id_when_the_code_around_it_changes() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let what = format!("{task:?} incremental={incremental}");
            let root = task_root(task);
            let plain = boot_in_shell(task, false, incremental);
            let edited = boot_in_shell(task, true, incremental);
            assert!(
                plain.query(&format!("#task-body > {root}")).is_some(),
                "{what}: the plain shell mounts the task in its body"
            );
            assert!(
                edited
                    .query(&format!("#task-body > .earlier + .wrapper > {root}"))
                    .is_some(),
                "{what}: the edited shell wraps the task after an earlier sibling"
            );
            assert!(
                edited.query("section > #task-body").is_some(),
                "{what}: the edited shell holds its body one level deeper"
            );
            assert_ne!(
                all_ids(&plain),
                all_ids(&edited),
                "{what}: the shell changed"
            );

            let before = subtree_ids(&plain, root);
            assert!(before.len() > 1, "{what}: the task renders elements");
            assert_eq!(
                subtree_ids(&edited, root),
                before,
                "{what}: same ids, in order"
            );

            let stand = stand::boot(task, stand::options(incremental));
            assert_eq!(subtree_ids(&stand, root), before, "{what}: the stand's ids");
        }
    }
}

const KEYS: [&str; 3] = ["form-name", "form-actions", "form-send"];
const LEAD: &str = ".lead";
const HINT: &str = ".hint";

/// The form fixture as first written. Every edit below is a component of the same name, so an
/// edit never reads as a rename.
mod before {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            div { class: "form",
                p { class: "lead", "Sign in" }
                label { r#for: "form-name", "Name" }
                input { id: "form-name" }
                div { id: "form-actions",
                    span { class: "hint", "then" }
                    button { id: "form-send", onclick: |_| {}, "Send" }
                }
            }
        }
    }
}

/// An earlier same-tag sibling before `#form-name` and before `#form-actions`.
mod sibling_added {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            div { class: "form",
                p { class: "lead", "Sign in" }
                label { r#for: "form-name", "Name" }
                input { class: "added" }
                input { id: "form-name" }
                div { class: "added" }
                div { id: "form-actions",
                    span { class: "hint", "then" }
                    button { id: "form-send", onclick: |_| {}, "Send" }
                }
            }
        }
    }
}

/// The paragraph and the label before `#form-name` removed.
mod sibling_removed {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            div { class: "form",
                input { id: "form-name" }
                div { id: "form-actions",
                    span { class: "hint", "then" }
                    button { id: "form-send", onclick: |_| {}, "Send" }
                }
            }
        }
    }
}

/// The form, `#form-name` and `#form-actions` each wrapped in a new ancestor.
mod wrapped {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            section {
                div { class: "form",
                    p { class: "lead", "Sign in" }
                    label { r#for: "form-name", "Name" }
                    fieldset {
                        input { id: "form-name" }
                    }
                    div { class: "row",
                        div { id: "form-actions",
                            span { class: "hint", "then" }
                            button { id: "form-send", onclick: |_| {}, "Send" }
                        }
                    }
                }
            }
        }
    }
}

/// Text, an attribute, a class and the handlers changed; no element added, removed or moved.
mod behaviour {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        let mut sent = use_signal(|| 0);
        rsx! {
            div { class: "form compact",
                p { class: "lead", "Log in" }
                label { r#for: "form-name", "Your name" }
                input { id: "form-name", placeholder: "you", oninput: |_| {} }
                div { id: "form-actions",
                    span { class: "hint", "and then" }
                    button { id: "form-send", onclick: move |_| sent += 1, "Submit" }
                }
            }
        }
    }
}

/// An earlier same-tag sibling before the lead paragraph, before the hint and before
/// `#form-send`: each inside the subtree the later element's path starts at.
mod inside_sibling {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            div { class: "form",
                p { "Welcome" }
                p { class: "lead", "Sign in" }
                label { r#for: "form-name", "Name" }
                input { id: "form-name" }
                div { id: "form-actions",
                    span { "first" }
                    span { class: "hint", "then" }
                    button { "Cancel" }
                    button { id: "form-send", onclick: |_| {}, "Send" }
                }
            }
        }
    }
}

/// A wrapper between the lead paragraph and its component's root, and between the hint and its
/// anchor.
mod inside_wrapper {
    use dioxus::prelude::*;

    #[component]
    pub fn Form() -> Element {
        rsx! {
            div { class: "form",
                header {
                    p { class: "lead", "Sign in" }
                }
                label { r#for: "form-name", "Name" }
                input { id: "form-name" }
                div { id: "form-actions",
                    div {
                        span { class: "hint", "then" }
                    }
                    button { id: "form-send", onclick: |_| {}, "Send" }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Edit {
    None,
    SiblingAdded,
    SiblingRemoved,
    Wrapped,
    Behaviour,
    InsideSibling,
    InsideWrapper,
}

fn form_root(edit: Edit) -> Element {
    match edit {
        Edit::None => rsx! { before::Form {} },
        Edit::SiblingAdded => rsx! { sibling_added::Form {} },
        Edit::SiblingRemoved => rsx! { sibling_removed::Form {} },
        Edit::Wrapped => rsx! { wrapped::Form {} },
        Edit::Behaviour => rsx! { behaviour::Form {} },
        Edit::InsideSibling => rsx! { inside_sibling::Form {} },
        Edit::InsideWrapper => rsx! { inside_wrapper::Form {} },
    }
}

fn boot_form(edit: Edit, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(
        VirtualDom::new_with_props(form_root, edit),
        stand::options(incremental),
    )
}

/// Boot the form as first written and after `edit`, asserting the edit changed the document.
#[track_caller]
fn boot_pair(edit: Edit, incremental: bool) -> (Harness<DioxusDocument>, Harness<DioxusDocument>) {
    let before = boot_form(Edit::None, incremental);
    let after = boot_form(edit, incremental);
    assert_ne!(
        after.dom_string(),
        before.dom_string(),
        "{edit:?}: the fixture changed"
    );
    (before, after)
}

#[track_caller]
fn assert_keys_read(harness: &Harness<DioxusDocument>, what: &str) {
    for key in KEYS {
        assert_eq!(id_of(harness, &format!("#{key}")), key, "{what}");
    }
}

#[test]
fn keyed_elements_keep_their_ids_under_every_edit() {
    for incremental in [false, true] {
        for edit in [Edit::SiblingAdded, Edit::SiblingRemoved, Edit::Wrapped] {
            let what = format!("{edit:?} incremental={incremental}");
            let (before, after) = boot_pair(edit, incremental);
            assert_ne!(all_ids(&after), all_ids(&before), "{what}: ids moved");
            match edit {
                Edit::SiblingAdded => {
                    assert!(after.query("input.added + #form-name").is_some(), "{what}");
                    assert!(after.query("div.added + #form-actions").is_some(), "{what}");
                }
                Edit::SiblingRemoved => {
                    assert!(before.query("label").is_some(), "{what}: a label before");
                    assert!(after.query("label").is_none(), "{what}: no label after");
                    assert!(after.query(LEAD).is_none(), "{what}: no paragraph after");
                }
                _ => {
                    assert!(after.query("fieldset > #form-name").is_some(), "{what}");
                    assert!(after.query(".row > #form-actions").is_some(), "{what}");
                    assert!(after.query("section > .form").is_some(), "{what}");
                }
            }
            assert_keys_read(&before, &what);
            assert_keys_read(&after, &what);
        }
    }
}

#[test]
fn anchored_elements_keep_their_ids_under_edits_outside_their_anchor() {
    for incremental in [false, true] {
        for edit in [Edit::SiblingAdded, Edit::Wrapped] {
            let what = format!("{edit:?} incremental={incremental}");
            let (before, after) = boot_pair(edit, incremental);
            let outside = match edit {
                Edit::SiblingAdded => "div.added + #form-actions",
                _ => ".row > #form-actions",
            };
            assert!(before.query(outside).is_none(), "{what}: not before");
            assert!(after.query(outside).is_some(), "{what}: the edit landed");
            assert_eq!(id_of(&before, HINT), "form-actions//span:0", "{what}");
            assert_eq!(id_of(&after, HINT), "form-actions//span:0", "{what}");
            assert_eq!(
                subtree_ids(&after, "#form-actions"),
                subtree_ids(&before, "#form-actions"),
                "{what}: everything under the anchor"
            );
        }
    }
}

#[test]
fn a_behaviour_edit_changes_no_id() {
    for incremental in [false, true] {
        let what = format!("incremental={incremental}");
        let (before, after) = boot_pair(Edit::Behaviour, incremental);
        assert_ne!(
            after.text_content(LEAD),
            before.text_content(LEAD),
            "{what}: the text changed"
        );
        assert_ne!(
            after.attr(".form", "class"),
            before.attr(".form", "class"),
            "{what}: a class changed"
        );
        assert!(before.attr("#form-name", "placeholder").is_none(), "{what}");
        assert!(after.attr("#form-name", "placeholder").is_some(), "{what}");
        assert!(
            before.attr("#form-name", "data-dioxus-id").is_none(),
            "{what}"
        );
        assert!(
            after.attr("#form-name", "data-dioxus-id").is_some(),
            "{what}: a handler was added"
        );
        assert_eq!(all_ids(&after), all_ids(&before), "{what}");
    }
}

#[test]
fn edits_inside_the_anchor_change_only_positional_ids() {
    for incremental in [false, true] {
        let what = format!("incremental={incremental}");

        let (before, sibling) = boot_pair(Edit::InsideSibling, incremental);
        assert_eq!(id_of(&before, HINT), "form-actions//span:0", "{what}");
        assert_eq!(id_of(&sibling, HINT), "form-actions//span:1", "{what}");
        assert_eq!(id_of(&before, LEAD), "Form/div:0/p:0", "{what}");
        assert_eq!(id_of(&sibling, LEAD), "Form/div:0/p:1", "{what}");
        assert!(sibling.query("button + #form-send").is_some(), "{what}");
        assert_keys_read(&sibling, &what);

        let (_, wrapper) = boot_pair(Edit::InsideWrapper, incremental);
        assert_eq!(
            id_of(&wrapper, HINT),
            "form-actions//div:0/span:0",
            "{what}"
        );
        assert_eq!(id_of(&wrapper, LEAD), "Form/div:0/header:0/p:0", "{what}");
        assert_keys_read(&wrapper, &what);
    }
}
