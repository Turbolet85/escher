//! What the Dioxus 0.7 runtime and escher's bridge expose of the UI graph's four links, read on
//! the headless stand in both layout modes with no fork-level change: (1) node → the component
//! that rendered it, through dioxus-core's public vdom walk and the bridge's public `vdom` and
//! `vdom_state`; (2) node → the handlers it carries, through the listener attributes the bridge
//! writes, a removed listener's attribute left standing, and exactly through the same walk's
//! dynamic attributes, `mounted` included; (3) handler → the state it writes, read
//! by its effect alone, the snapshot diff of a click; (4) state → the nodes that read it, through
//! a held signal's subscribers and their origin scopes. The stand holds its own state in
//! hook-local signals, which dioxus-core keeps crate-private, so links 3 and 4 for that state are
//! read from the code and not run here.
//!
//! It also pins the MountedData hazard as it stands: a task spawned from a key handler that writes
//! a signal is first polled inside `render_immediate`, while `DioxusDocument::poll` holds the
//! document's `RefCell`, so a `MountedData` call in it panics `RefCell already borrowed`. Any app
//! can meet it; the engine-debts reading of 2026-10-31 measured it beside its step 15. Upstream
//! fixed it by rendering into a mutation buffer with the document unborrowed
//! (https://github.com/DioxusLabs/blitz/pull/1260), in the commit that moves Dioxus to 0.8; its
//! port inverts the pin. The same handler calling `set_focus` with no task acts.
//!
//! Every failure message names a layout mode, a task or page and a link or an index; none holds
//! a component name, a listener read off a node, a signal's value or a node id.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use accesskit::Role;
use blitz_dom::{BaseDocument, Document, NodeId};
use blitz_test_harness::{Harness, Key, NamedKey};
use dioxus::prelude::*;
use dioxus_core::{AttributeValue, DynamicNode, TemplateNode, VNode};
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::{self, LeanTask};

/// The value the bridge writes for an attribute that stands for a listener.
const LISTENER_VALUE: &str = "<rust func>";

/// What the public vdom walk reads.
struct Walk {
    /// Each mounted template root element, with the name of the component whose render produced
    /// it, `None` above every component.
    roots: HashMap<NodeId, Option<&'static str>>,
    /// Each mounted component instance: its name and its scope's number.
    scopes: Vec<(&'static str, usize)>,
    /// The events of the listeners each mounted element carries now, read from its template's
    /// dynamic attributes.
    listeners: HashMap<NodeId, BTreeSet<&'static str>>,
}

/// A component's name as written: the last segment of `VComponent.name`'s type path.
fn component_name(type_path: &'static str) -> &'static str {
    let path = type_path.split('<').next().unwrap_or(type_path);
    path.rsplit("::").next().unwrap_or(path)
}

/// Walks the vdom from its base scope through dioxus-core's public API alone.
fn walk(doc: &DioxusDocument) -> Walk {
    let mut walk = Walk {
        roots: HashMap::new(),
        scopes: Vec::new(),
        listeners: HashMap::new(),
    };
    if let Some(root) = doc.vdom.base_scope().try_root_node() {
        visit(doc, root, None, &mut walk);
    }
    walk
}

fn visit(doc: &DioxusDocument, vnode: &VNode, owner: Option<&'static str>, walk: &mut Walk) {
    for (idx, root) in vnode.template.roots.iter().enumerate() {
        if !matches!(root, TemplateNode::Element { .. }) {
            continue;
        }
        let node = vnode
            .mounted_root(idx, &doc.vdom)
            .and_then(|element| doc.vdom_state.try_element_to_node_id(element));
        if let Some(node) = node {
            walk.roots.insert(node, owner);
        }
    }
    for (idx, attributes) in vnode.dynamic_attrs.iter().enumerate() {
        let node = vnode
            .mounted_dynamic_attribute(idx, &doc.vdom)
            .and_then(|element| doc.vdom_state.try_element_to_node_id(element));
        let Some(node) = node else {
            continue;
        };
        for attribute in attributes.iter() {
            if let (AttributeValue::Listener(_), Some(event)) =
                (&attribute.value, attribute.name.strip_prefix("on"))
            {
                walk.listeners.entry(node).or_default().insert(event);
            }
        }
    }
    for (idx, dynamic) in vnode.dynamic_nodes.iter().enumerate() {
        match dynamic {
            DynamicNode::Component(component) => {
                let Some(scope) = component.mounted_scope(idx, vnode, &doc.vdom) else {
                    continue;
                };
                let name = component_name(component.name);
                walk.scopes.push((name, scope.id().0));
                if let Some(child) = scope.try_root_node() {
                    visit(doc, child, Some(name), walk);
                }
            }
            DynamicNode::Fragment(items) => {
                for item in items {
                    visit(doc, item, owner, walk);
                }
            }
            DynamicNode::Text(_) | DynamicNode::Placeholder(_) => {}
        }
    }
}

/// The component that rendered `node`: its nearest ancestor-or-self template root's owner.
fn owner_of(doc: &BaseDocument, walk: &Walk, node: NodeId) -> Option<&'static str> {
    let mut at = Some(node);
    while let Some(id) = at {
        if let Some(owner) = walk.roots.get(&id) {
            return *owner;
        }
        at = doc.get_node(id)?.parent;
    }
    None
}

/// Whether `node` lies strictly under `ancestor`.
fn is_under(doc: &BaseDocument, node: NodeId, ancestor: NodeId) -> bool {
    let mut at = doc.get_node(node).and_then(|node| node.parent);
    while let Some(id) = at {
        if id == ancestor {
            return true;
        }
        at = doc.get_node(id).and_then(|node| node.parent);
    }
    false
}

/// The node the stable id `id` names. `id` is one this file names.
#[track_caller]
fn named(ids: &[(NodeId, String)], id: &str) -> NodeId {
    ids.iter()
        .find(|(_, read)| read == id)
        .map(|(node, _)| *node)
        .unwrap_or_else(|| panic!("{id:?} is a stable id of the screen"))
}

/// The events of the listener attributes the bridge wrote on `node`.
fn listeners(doc: &BaseDocument, node: NodeId) -> BTreeSet<String> {
    doc.get_node(node)
        .and_then(|node| node.element_data())
        .map(|element| {
            element
                .attrs
                .iter()
                .filter(|attr| attr.value == LISTENER_VALUE)
                .map(|attr| attr.name.local.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Whether `node` carries the bridge's listener mark.
fn carries_mark(doc: &BaseDocument, node: NodeId) -> bool {
    doc.get_node(node)
        .and_then(|node| node.element_data())
        .is_some_and(|element| {
            element
                .attrs
                .iter()
                .any(|attr| *attr.name.local == *"data-dioxus-id")
        })
}

fn boot(task: LeanTask, incremental: bool) -> Harness<DioxusDocument> {
    stand::boot(task, stand::options(incremental))
}

fn boot_page(page: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(page), stand::options(incremental))
}

/// The ids of the nodes the snapshot diff of `act` names changed, and whether it added or
/// removed none.
fn changed_by(
    harness: &mut Harness<DioxusDocument>,
    act: impl FnOnce(&mut Harness<DioxusDocument>),
) -> (BTreeSet<String>, bool) {
    harness.base_mut().take_changed_nodes();
    let before = harness.doc.snapshot();
    act(harness);
    let diff = before.diff(&harness.doc.snapshot());
    let changed = diff.changed.iter().map(|entry| entry.id.clone()).collect();
    (changed, diff.added.is_empty() && diff.removed.is_empty())
}

#[test]
fn link_1_the_vdom_walk_names_the_component_that_rendered_each_node() {
    const TASKS: [(LeanTask, &str, &[&str]); 2] = [
        (
            LeanTask::Counter,
            "Counter",
            &["counter-value", "counter-increment"],
        ),
        (
            LeanTask::Crud,
            "Crud",
            &[
                "crud-create",
                "crud-person-0",
                "crud-person-1",
                "crud-person-2",
            ],
        ),
    ];
    for incremental in [false, true] {
        for (task, component, ids) in TASKS {
            let harness = boot(task, incremental);
            let walk = walk(&harness.doc);
            let read = harness.doc.element_ids();
            let doc = harness.base();
            for (index, id) in ids.iter().enumerate() {
                let node = named(&read, id);
                assert!(
                    owner_of(&doc, &walk, node) == Some(component),
                    "incremental={incremental} {task:?}: link 1, named id {index}"
                );
            }
            let body = named(&read, "task-body");
            let in_body: Vec<NodeId> = read
                .iter()
                .map(|(node, _)| *node)
                .filter(|node| is_under(&doc, *node, body))
                .collect();
            assert!(
                !in_body.is_empty(),
                "incremental={incremental} {task:?}: the task body holds elements"
            );
            for (index, node) in in_body.iter().enumerate() {
                assert!(
                    owner_of(&doc, &walk, *node).is_some(),
                    "incremental={incremental} {task:?}: link 1, body element {index}"
                );
            }
        }
    }
}

#[test]
fn link_1_a_deleted_row_resolves_to_nothing() {
    for incremental in [false, true] {
        let mut harness = boot(LeanTask::Crud, incremental);
        let row = named(&harness.doc.element_ids(), "crud-person-1");
        assert!(
            walk(&harness.doc).roots.contains_key(&row),
            "incremental={incremental} CRUD: the walk reaches the row before its deletion"
        );
        harness.click("#crud-person-1");
        harness.click("#crud-delete");
        assert!(
            !harness
                .doc
                .element_ids()
                .iter()
                .any(|(_, id)| id == "crud-person-1"),
            "incremental={incremental} CRUD: the deleted row's id is gone"
        );
        let walk = walk(&harness.doc);
        assert!(
            !walk.roots.contains_key(&row),
            "incremental={incremental} CRUD: link 1, the walk holds no root for the deleted row"
        );
        assert!(
            owner_of(&harness.base(), &walk, row).is_none(),
            "incremental={incremental} CRUD: link 1, the deleted row resolves to no component"
        );
    }
}

#[test]
fn link_2_the_listener_attributes_name_each_nodes_handlers() {
    for incremental in [false, true] {
        let harness = boot(LeanTask::Counter, incremental);
        let read = harness.doc.element_ids();
        let doc = harness.base();
        let increment = named(&read, "counter-increment");
        assert!(
            listeners(&doc, increment) == BTreeSet::from(["click".to_string()]),
            "incremental={incremental} Counter: link 2, the button carries one click listener"
        );
        assert!(
            carries_mark(&doc, increment),
            "incremental={incremental} Counter: link 2, the button carries the mark"
        );
        let value = named(&read, "counter-value");
        assert!(
            listeners(&doc, value).is_empty() && !carries_mark(&doc, value),
            "incremental={incremental} Counter: link 2, the value carries no listener"
        );
        let walk = walk(&harness.doc);
        assert!(
            walk.listeners.get(&increment) == Some(&BTreeSet::from(["click"]))
                && !walk.listeners.contains_key(&value),
            "incremental={incremental} Counter: link 2, the vdom walk reads the same listeners"
        );
        drop(doc);

        let harness = boot(LeanTask::Crud, incremental);
        let list = named(&harness.doc.element_ids(), "crud-list");
        assert!(
            listeners(&harness.base(), list).contains("keydown"),
            "incremental={incremental} CRUD: link 2, the list carries a keydown listener"
        );
    }
}

/// Of the elements whose accessibility node is a generic container and which carry the mark,
/// how many carry each set of listener events.
fn reacting_without_role(harness: &Harness<DioxusDocument>) -> BTreeMap<Vec<String>, usize> {
    let roles: HashMap<u64, Role> = harness
        .doc
        .accessibility_tree()
        .nodes
        .iter()
        .map(|(id, node)| (id.0, node.role()))
        .collect();
    let doc = harness.base();
    let mut counts = BTreeMap::new();
    for (node, _) in harness.doc.element_ids() {
        if carries_mark(&doc, node) && roles.get(&node.as_u64()) == Some(&Role::GenericContainer) {
            let events: Vec<String> = listeners(&doc, node).into_iter().collect();
            *counts.entry(events).or_default() += 1;
        }
    }
    counts
}

#[test]
fn link_2_on_cells_the_listeners_name_what_the_677_react_to() {
    for incremental in [false, true] {
        let harness = stand::boot_cells(stand::options(incremental));
        let counts = reacting_without_role(&harness);
        assert!(
            counts.values().sum::<usize>() == 677,
            "incremental={incremental} Cells: link 2, the pinned count of elements that react \
             and state no role"
        );
        // A `mounted` listener writes no attribute, so the cells' and the grid's read nothing here.
        let expected = BTreeMap::from([
            (vec!["click".to_string()], 676),
            (vec!["keydown".to_string()], 1),
        ]);
        assert!(
            counts == expected,
            "incremental={incremental} Cells: link 2, 676 cells react to a click and the grid to \
             a key"
        );
        let grid = named(&harness.doc.element_ids(), "cells-grid");
        assert!(
            walk(&harness.doc).listeners.get(&grid)
                == Some(&BTreeSet::from(["keydown", "mounted"])),
            "incremental={incremental} Cells: link 2, the vdom walk reads the grid's mounted \
             listener beside its key listener"
        );
    }
}

/// A button whose click listener is spread in while a signal holds, and a second button that
/// clears it; the first one's clicks counted.
fn toggled_listener() -> Element {
    let mut armed = use_signal(|| true);
    let mut clicks = use_signal(|| 0u32);
    let spread: Vec<Attribute> = if armed() {
        vec![dioxus::html::onclick(move |_: MouseEvent| clicks += 1)]
    } else {
        Vec::new()
    };
    rsx! {
        button { id: "toggled", ..spread, "Toggled" }
        button { id: "disarm", onclick: move |_| armed.set(false), "Disarm" }
        p { id: "clicks", "{clicks}" }
    }
}

#[test]
fn link_2_a_removed_listeners_attribute_stays() {
    for incremental in [false, true] {
        let mut harness = boot_page(toggled_listener, incremental);
        let toggled = harness.node("#toggled");
        let click = BTreeSet::from(["click".to_string()]);
        assert!(
            listeners(&harness.base(), toggled) == click,
            "incremental={incremental} toggled page: the armed button carries its listener"
        );
        assert!(
            walk(&harness.doc).listeners.get(&toggled) == Some(&BTreeSet::from(["click"])),
            "incremental={incremental} toggled page: the vdom walk reads the armed listener"
        );
        harness.click("#toggled");
        assert!(
            harness.text_content("#clicks") == "1",
            "incremental={incremental} toggled page: the armed listener acts"
        );

        harness.click("#disarm");
        harness.pump();
        assert!(
            harness.node("#toggled") == toggled,
            "incremental={incremental} toggled page: the button is the same element"
        );
        assert!(
            listeners(&harness.base(), toggled) == click,
            "incremental={incremental} toggled page: link 2, the removed listener's attribute \
             stays"
        );
        assert!(
            !walk(&harness.doc).listeners.contains_key(&toggled),
            "incremental={incremental} toggled page: link 2, the vdom walk reads the listener \
             gone"
        );
        harness.click("#toggled");
        assert!(
            harness.text_content("#clicks") == "1",
            "incremental={incremental} toggled page: link 2, no listener acts behind it"
        );
    }
}

#[test]
fn link_3_a_handlers_effect_is_the_diff_of_its_step() {
    for incremental in [false, true] {
        let mut harness = boot(LeanTask::Counter, incremental);
        let (changed, kept) =
            changed_by(&mut harness, |harness| harness.click("#counter-increment"));
        assert!(
            kept && changed.contains("counter-value"),
            "incremental={incremental} Counter: link 3, the click's diff names the value"
        );
    }
}

static SHARED: GlobalSignal<u32> = Signal::global(|| 0);

#[component]
fn FirstReader() -> Element {
    rsx! { p { id: "first-reader", "{SHARED}" } }
}

#[component]
fn SecondReader() -> Element {
    rsx! { p { id: "second-reader", "{SHARED}" } }
}

#[component]
fn Writer() -> Element {
    rsx! {
        button { id: "writer", onclick: move |_| *SHARED.write() += 1, "Write" }
    }
}

/// A signal the test names, read by two components and written by a third one's click.
fn shared_state() -> Element {
    rsx! {
        FirstReader {}
        SecondReader {}
        Writer {}
    }
}

#[test]
fn links_3_and_4_a_held_signals_subscribers_are_its_readers() {
    for incremental in [false, true] {
        let mut harness = boot_page(shared_state, incremental);
        let walk = walk(&harness.doc);
        let scopes_of = |names: &[&str]| -> BTreeSet<usize> {
            walk.scopes
                .iter()
                .filter(|(name, _)| names.contains(name))
                .map(|(_, scope)| *scope)
                .collect()
        };
        let readers = scopes_of(&["FirstReader", "SecondReader"]);
        let writer = scopes_of(&["Writer"]);
        assert!(
            readers.len() == 2 && writer.len() == 1,
            "incremental={incremental} shared page: the walk names both readers and the writer"
        );
        let subscribed: BTreeSet<usize> = harness.doc.vdom.in_runtime(|| {
            let mut scopes = BTreeSet::new();
            SHARED
                .subscribers()
                .visit(|context| _ = scopes.insert(context.origin_scope().0));
            scopes
        });
        assert!(
            subscribed == readers,
            "incremental={incremental} shared page: link 4, the subscribers' origin scopes are \
             the two readers'"
        );

        // A first click focuses the writer, so the step's diff reads no focus move.
        harness.click("#writer");
        let (changed, kept) = changed_by(&mut harness, |harness| harness.click("#writer"));
        let expected = BTreeSet::from(["first-reader".to_string(), "second-reader".to_string()]);
        assert!(
            kept && changed == expected,
            "incremental={incremental} shared page: link 3, the writer's click changes exactly \
             the readers' nodes"
        );
    }
}

/// An element whose `onmounted` holds its `MountedData`, beside a focusable element whose key
/// handler writes a signal and asks that handle for focus, from a spawned task or at once.
fn hazard(spawned: bool) -> Element {
    let mut handle: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let mut presses = use_signal(|| 0u32);
    rsx! {
        div {
            id: "focus-target",
            onmounted: move |evt: MountedEvent| handle.set(Some(evt.data())),
            "Target"
        }
        div {
            id: "key-target",
            tabindex: "0",
            onkeydown: move |_| {
                presses += 1;
                let Some(target) = handle() else {
                    return;
                };
                if spawned {
                    spawn(async move {
                        _ = target.set_focus(true).await;
                    });
                } else {
                    drop(target.set_focus(true));
                }
            },
            "Keys"
        }
        p { id: "mounted", if handle.read().is_some() { "held" } else { "none" } }
        p { id: "presses", "{presses}" }
    }
}

fn hazard_spawned() -> Element {
    hazard(true)
}

fn hazard_synchronous() -> Element {
    hazard(false)
}

/// The hazard page booted with its handle held and the key target focused by Tab, since a click
/// does not focus it.
fn hazard_ready(page: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    let mut harness = boot_page(page, incremental);
    harness.pump();
    assert!(
        harness.text_content("#mounted") == "held",
        "incremental={incremental} hazard page: the mounted handle is held"
    );
    let key_target = harness.node("#key-target");
    harness.press(Key::Named(NamedKey::Tab));
    assert!(
        harness.focused() == Some(key_target),
        "incremental={incremental} hazard page: Tab focuses the key target"
    );
    harness
}

fn press_with_spawned_focus(incremental: bool) {
    let mut harness = hazard_ready(hazard_spawned, incremental);
    harness.press(Key::Named(NamedKey::ArrowDown));
}

#[test]
#[should_panic(expected = "RefCell already borrowed")]
fn mounted_data_in_a_task_a_key_handler_spawned_panics_non_incremental() {
    press_with_spawned_focus(false);
}

#[test]
#[should_panic(expected = "RefCell already borrowed")]
fn mounted_data_in_a_task_a_key_handler_spawned_panics_incremental() {
    press_with_spawned_focus(true);
}

#[test]
fn mounted_data_asked_at_once_in_a_key_handler_acts() {
    for incremental in [false, true] {
        let mut harness = hazard_ready(hazard_synchronous, incremental);
        let focus_target = harness.node("#focus-target");
        harness.press(Key::Named(NamedKey::ArrowDown));
        assert!(
            harness.text_content("#presses") == "1",
            "incremental={incremental} hazard page: the key handler ran once"
        );
        assert!(
            harness.focused() == Some(focus_target),
            "incremental={incremental} hazard page: the handle's focus request acts"
        );
    }
}
