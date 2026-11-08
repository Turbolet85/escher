//! Stable element ids: a semantic name for every element of a [`DioxusDocument`](crate::DioxusDocument),
//! computed on demand from the vdom and the DOM and never written to either.
//!
//! An element's id is, in order of precedence:
//! 1. its **author key** — the HTML `id` attribute, when non-empty, free of `/` and not already
//!    claimed by an element earlier in document pre-order;
//! 2. its **anchored path** — under an element of its own component that reads an author key:
//!    that key, `//`, then one `{tag}[{key}]` / `{tag}:{n}` segment per DOM level below the keyed
//!    element (`{key}//{segment}/{segment}`). The nearest such element anchors, so an edit outside
//!    it renames nothing inside it;
//! 3. its **component path** — the names of the components between the app root and the element's
//!    owning component (`{name}:{k}` for an owner's later instance of a name it already rendered),
//!    then one `{tag}[{key}]` / `{tag}:{n}` segment per DOM level below that component's template
//!    root. A component's template root always starts here, whatever keyed element it sits under;
//! 4. its **document path** — for elements no component renders: a leading `/`, then one
//!    `{tag}:{n}` segment per DOM level from the document root.
//!
//! Paths always contain `/` and keys never do, so the two never collide. An anchored path is the
//! only id holding `//`: a key holds no `/`, a component path has no empty segment, and a document
//! path starts with `/` and has none after it. No `NodeId`, `ElementId`, `ScopeId` or pointer
//! appears in an id, so the same tree reads the same ids in any process.
//!
//! A fifth form is reserved and produced by nothing yet: a **virtual node** — a target an owner
//! element's own handler serves inside it, with no element of its own — reads its owner's id,
//! [`VIRTUAL_KEY_SEPARATOR`] (`///`), then a non-empty key holding no `/`
//! ([`virtual_node_id`]). No other form can hold `///`, two empty segments in a row: a key
//! holds no `/`, a component chain joins non-empty names, a segment is never empty, and the only
//! empty segment an id holds is the single one an anchor writes (`{key}//`).

use crate::NodeId;
use crate::mutation_writer::DioxusState;
use blitz_dom::{BaseDocument, local_name};
use dioxus_core::{
    AttributeValue, DynamicNode, ScopeId, TemplateNode, VComponent, VNode, VirtualDom,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::rc::Rc;

/// What separates a virtual node's owner id from its key: a string no element id holds.
pub const VIRTUAL_KEY_SEPARATOR: &str = "///";

/// The reserved id of the virtual node `key` its owner element `owner` serves:
/// `{owner}///{key}`. `None` for an empty key or one holding `/`. Nothing produces such a node
/// yet; this is the one spelling its producer is to write.
pub fn virtual_node_id(owner: &str, key: &str) -> Option<String> {
    (!key.is_empty() && !key.contains('/')).then(|| format!("{owner}{VIRTUAL_KEY_SEPARATOR}{key}"))
}

/// The component instance whose render produced a template root.
#[derive(Clone)]
struct Owner {
    instance: usize,
    /// `/`-joined component segments, empty for the app root and dioxus-core's own wrappers.
    chain: Rc<str>,
}

/// Who an owner instance is: its scope, and its component's name where the author wrote one.
#[derive(Clone, Copy)]
struct OwnerScope {
    scope: ScopeId,
    component: Option<&'static str>,
}

struct RootInfo {
    owner: Owner,
    key: Option<String>,
}

/// A component's name as written: dioxus-core names it by its full type path, generics included.
fn component_name(type_path: &'static str) -> &'static str {
    let path = type_path.split('<').next().unwrap_or(type_path);
    path.rsplit("::").next().unwrap_or(path)
}

/// dioxus-core mounts the app under a root wrapper, a suspense boundary and an error boundary;
/// none of them, nor the app root itself, is a component the author wrote into a path.
fn is_framework_scope(id: ScopeId) -> bool {
    [
        ScopeId::ROOT,
        ScopeId::ROOT_SUSPENSE_BOUNDARY,
        ScopeId::ROOT_ERROR_BOUNDARY,
        ScopeId::APP,
    ]
    .contains(&id)
}

struct VdomWalk<'a> {
    dom: &'a VirtualDom,
    state: &'a DioxusState,
    roots: FxHashMap<NodeId, RootInfo>,
    instances: usize,
    /// Each owner instance's scope and component name, by instance.
    scopes: FxHashMap<usize, OwnerScope>,
    /// The events of the listeners each mounted element carries now, without their `on`.
    listeners: FxHashMap<NodeId, Vec<&'static str>>,
}

impl VdomWalk<'_> {
    fn new_owner(&mut self, chain: Rc<str>, scope: OwnerScope) -> Owner {
        self.instances += 1;
        self.scopes.insert(self.instances, scope);
        Owner {
            instance: self.instances,
            chain,
        }
    }

    /// `seen` counts, per component name, the instances `owner` has rendered so far.
    fn walk(&mut self, vnode: &VNode, owner: &Owner, seen: &mut FxHashMap<&'static str, usize>) {
        self.record_roots(vnode, owner);
        self.record_listeners(vnode);

        for (idx, node) in vnode.dynamic_nodes.iter().enumerate() {
            match node {
                DynamicNode::Component(component) => {
                    self.walk_component(component, idx, vnode, owner, seen);
                }
                DynamicNode::Fragment(items) => {
                    for item in items {
                        self.walk(item, owner, seen);
                    }
                }
                DynamicNode::Text(_) | DynamicNode::Placeholder(_) => {}
            }
        }
    }

    /// Records each mounted element root of `vnode`'s template as rendered by `owner`.
    fn record_roots(&mut self, vnode: &VNode, owner: &Owner) {
        for (idx, root) in vnode.template.roots.iter().enumerate() {
            if !matches!(root, TemplateNode::Element { .. }) {
                continue;
            }
            let node_id = vnode
                .mounted_root(idx, self.dom)
                .and_then(|element_id| self.state.try_element_to_node_id(element_id));
            if let Some(node_id) = node_id {
                self.roots.insert(
                    node_id,
                    RootInfo {
                        owner: owner.clone(),
                        key: vnode.key.clone(),
                    },
                );
            }
        }
    }

    /// Records the listeners `vnode`'s dynamic attributes hold now on each mounted element: the
    /// vdom's own reading, so a listener a re-render dropped is gone and `mounted` is read.
    fn record_listeners(&mut self, vnode: &VNode) {
        for (idx, attributes) in vnode.dynamic_attrs.iter().enumerate() {
            let node = vnode
                .mounted_dynamic_attribute(idx, self.dom)
                .and_then(|element_id| self.state.try_element_to_node_id(element_id));
            let Some(node) = node else {
                continue;
            };
            for attribute in attributes.iter() {
                if let (AttributeValue::Listener(_), Some(event)) =
                    (&attribute.value, attribute.name.strip_prefix("on"))
                {
                    let events = self.listeners.entry(node).or_default();
                    if !events.contains(&event) {
                        events.push(event);
                    }
                }
            }
        }
    }

    /// Walks what the component at dynamic node `idx` of `vnode` rendered, under an owner of its
    /// own; a component that mounted no root is skipped and counts for no ordinal.
    fn walk_component(
        &mut self,
        component: &VComponent,
        idx: usize,
        vnode: &VNode,
        owner: &Owner,
        seen: &mut FxHashMap<&'static str, usize>,
    ) {
        let Some(scope) = component.mounted_scope(idx, vnode, self.dom) else {
            return;
        };
        let Some(child_root) = scope.try_root_node() else {
            return;
        };
        let (chain, name) = if is_framework_scope(scope.id()) {
            (Rc::clone(&owner.chain), None)
        } else {
            let name = component_name(component.name);
            (child_chain(owner, name, seen), Some(name))
        };
        let child_owner = self.new_owner(
            chain,
            OwnerScope {
                scope: scope.id(),
                component: name,
            },
        );
        self.walk(child_root, &child_owner, &mut FxHashMap::default());
    }
}

/// The chain of `owner`'s next instance of the component `name`: `owner`'s chain, then `name`
/// for the first instance and `{name}:{k}` for a later one. Counts the instance in `seen`.
fn child_chain(
    owner: &Owner,
    name: &'static str,
    seen: &mut FxHashMap<&'static str, usize>,
) -> Rc<str> {
    let ordinal = seen.entry(name).or_default();
    let segment = match *ordinal {
        0 => name.to_string(),
        n => format!("{name}:{n}"),
    };
    *ordinal += 1;
    if owner.chain.is_empty() {
        Rc::from(segment)
    } else {
        Rc::from(format!("{}/{segment}", owner.chain))
    }
}

fn vdom_walk<'a>(dom: &'a VirtualDom, state: &'a DioxusState) -> VdomWalk<'a> {
    let mut walk = VdomWalk {
        dom,
        state,
        roots: FxHashMap::default(),
        instances: 0,
        scopes: FxHashMap::default(),
        listeners: FxHashMap::default(),
    };
    if let Some(root) = dom.base_scope().try_root_node() {
        let base = OwnerScope {
            scope: dom.base_scope().id(),
            component: None,
        };
        let owner = walk.new_owner(Rc::from(""), base);
        walk.walk(root, &owner, &mut FxHashMap::default());
    }
    walk
}

/// Where an element sits: the component instance that owns it (`None` outside every component)
/// and its path.
struct Placed {
    node: NodeId,
    owner: Option<usize>,
    path: String,
}

/// What the walk reads of one element beside its stable id.
#[cfg(feature = "accessibility")]
pub(crate) struct ElementFacts {
    pub(crate) node: NodeId,
    pub(crate) id: String,
    /// The scope of the component instance whose render holds the element; `None` outside
    /// every component.
    pub(crate) scope: Option<ScopeId>,
    /// That component's name as its path reads it; `None` for the app root, dioxus-core's own
    /// wrappers and outside every component.
    pub(crate) component: Option<&'static str>,
    /// The events of the listeners the element carries now, without their `on`.
    pub(crate) listeners: Vec<&'static str>,
}

/// Every element of `doc` with its stable id, in document pre-order.
pub(crate) fn element_ids(
    dom: &VirtualDom,
    state: &DioxusState,
    doc: &BaseDocument,
) -> Vec<(NodeId, String)> {
    placed_ids(&vdom_walk(dom, state).roots, doc)
        .into_iter()
        .map(|(node, id, _)| (node, id))
        .collect()
}

/// Every element of `doc` with its stable id, its owner and its listeners, in document
/// pre-order.
#[cfg(feature = "accessibility")]
pub(crate) fn element_facts(
    dom: &VirtualDom,
    state: &DioxusState,
    doc: &BaseDocument,
) -> Vec<ElementFacts> {
    let mut walk = vdom_walk(dom, state);
    placed_ids(&walk.roots, doc)
        .into_iter()
        .map(|(node, id, owner)| {
            let owner = owner.and_then(|instance| walk.scopes.get(&instance).copied());
            ElementFacts {
                node,
                id,
                scope: owner.map(|owner| owner.scope),
                component: owner.and_then(|owner| owner.component),
                listeners: walk.listeners.remove(&node).unwrap_or_default(),
            }
        })
        .collect()
}

/// The events of the listeners each mounted element carries now, by node.
#[cfg(feature = "accessibility")]
pub(crate) fn element_listeners(
    dom: &VirtualDom,
    state: &DioxusState,
) -> FxHashMap<NodeId, Vec<&'static str>> {
    vdom_walk(dom, state).listeners
}

/// Every element of `doc` with its stable id and its owner instance, in document pre-order.
fn placed_ids(
    roots: &FxHashMap<NodeId, RootInfo>,
    doc: &BaseDocument,
) -> Vec<(NodeId, String, Option<usize>)> {
    let mut claimed_keys = FxHashSet::default();
    let mut ids = Vec::new();

    let document_root = doc.root_node();
    let mut stack = place_children(doc, roots, document_root.id, None, "");
    stack.reverse();

    while let Some(placed) = stack.pop() {
        let Some(node) = doc.get_node(placed.node) else {
            continue;
        };
        let author_key = node
            .element_data()
            .and_then(|element| element.attr(local_name!("id")))
            .filter(|key| !key.is_empty() && !key.contains('/'))
            .filter(|key| claimed_keys.insert(key.to_string()));
        // A keyed element anchors its same-owner children: `{key}/` + `/{segment}`.
        let (id, anchor) = match author_key {
            Some(key) => (key.to_string(), Some(format!("{key}/"))),
            None => (placed.path, None),
        };
        let base = anchor.as_deref().unwrap_or(&id);
        let mut children = place_children(doc, roots, placed.node, placed.owner, base);
        ids.push((placed.node, id, placed.owner));
        children.reverse();
        stack.extend(children);
    }
    ids
}

/// The element children of `parent`, each placed per the id grammar. `parent_path` is what a
/// child continuing its parent's path extends: the parent's own path, or `{key}/` when the parent
/// reads its author key.
fn place_children(
    doc: &BaseDocument,
    roots: &FxHashMap<NodeId, RootInfo>,
    parent: NodeId,
    parent_owner: Option<usize>,
    parent_path: &str,
) -> Vec<Placed> {
    let Some(parent_node) = doc.get_node(parent) else {
        return Vec::new();
    };
    let mut counts: FxHashMap<(Option<usize>, &str), usize> = FxHashMap::default();
    let mut keyed: FxHashSet<(Option<usize>, String)> = FxHashSet::default();
    let mut placed = Vec::new();

    for &child in &parent_node.children {
        let Some(element) = doc.get_node(child).and_then(|node| node.element_data()) else {
            continue;
        };
        let tag = &*element.name.local;
        let root = roots.get(&child);
        let owner = root.map_or(parent_owner, |root| Some(root.owner.instance));

        let n = counts.entry((owner, tag)).or_default();
        let index = *n;
        *n += 1;

        let key_segment = root
            .and_then(|root| root.key.as_deref())
            .filter(|key| !key.is_empty() && !key.contains('/'))
            .map(|key| format!("{tag}[{key}]"))
            .filter(|segment| keyed.insert((owner, segment.clone())));
        let segment = key_segment.unwrap_or_else(|| format!("{tag}:{index}"));

        let path = match root {
            Some(root) if owner != parent_owner => format!("{}/{segment}", root.owner.chain),
            _ => format!("{parent_path}/{segment}"),
        };
        placed.push(Placed {
            node: child,
            owner,
            path,
        });
    }
    placed
}

/// The stable id of `node`, or `None` when it is not an element reachable from the document root.
pub(crate) fn element_id(
    dom: &VirtualDom,
    state: &DioxusState,
    doc: &BaseDocument,
    node: NodeId,
) -> Option<String> {
    doc.get_node(node)?.element_data()?;
    element_ids(dom, state, doc)
        .into_iter()
        .find_map(|(id, element_id)| (id == node).then_some(element_id))
}

#[cfg(test)]
mod tests {
    use crate::DioxusDocument;
    use blitz_dom::{Document, DocumentConfig};
    use dioxus::prelude::*;
    use dioxus_core::ScopeId;

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc
    }

    fn ids(doc: &DioxusDocument) -> Vec<String> {
        doc.element_ids().into_iter().map(|(_, id)| id).collect()
    }

    #[track_caller]
    fn assert_distinct(ids: &[String]) {
        let mut seen = std::collections::HashSet::new();
        for id in ids {
            assert!(seen.insert(id), "id {id:?} read twice in {ids:?}");
        }
    }

    #[test]
    fn keyed_list_rows_read_their_key_segment() {
        fn app() -> Element {
            rsx! {
                ul {
                    for name in ["ada", "grace"] {
                        li { key: "{name}", "{name}" }
                    }
                }
            }
        }
        let doc = build(app);
        let ids = ids(&doc);
        assert_eq!(
            ids,
            [
                "/html:0",
                "/html:0/head:0",
                "/html:0/body:0",
                "main",
                "/ul:0",
                "/ul:0/li[ada]",
                "/ul:0/li[grace]",
            ]
        );
    }

    #[test]
    fn a_duplicate_html_id_reads_a_path() {
        fn app() -> Element {
            rsx! {
                div { id: "dup" }
                div { id: "dup" }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["dup", "/div:1"]);
        assert_distinct(&ids);
    }

    #[test]
    fn an_empty_or_slashed_html_id_reads_a_path() {
        fn app() -> Element {
            rsx! {
                div { id: "" }
                div { id: "a/b" }
                div { id: "/div:0" }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["/div:0", "/div:1", "/div:2"]);
        assert_distinct(&ids);
    }

    #[test]
    fn a_repeated_dioxus_key_falls_back_to_an_index() {
        #[component]
        fn Item() -> Element {
            rsx! { b {} }
        }
        fn app() -> Element {
            rsx! {
                ul {
                    for key in ["same", "same"] {
                        li { key: "{key}" }
                    }
                }
                section { Item {} }
                section { Item {} }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(
            &ids[4..],
            [
                "/ul:0",
                "/ul:0/li[same]",
                "/ul:0/li:1",
                "/section:0",
                "Item/b:0",
                "/section:1",
                "Item:1/b:0",
            ]
        );
        assert_distinct(&ids);
    }

    #[test]
    fn a_removed_element_reads_no_id() {
        static SHOW: GlobalSignal<bool> = Signal::global(|| true);
        fn app() -> Element {
            rsx! {
                div {
                    if SHOW() {
                        p { "gone soon" }
                    }
                }
            }
        }
        let mut doc = build(app);
        let (removed, id) = doc.element_ids().pop().expect("the p element");
        assert_eq!(id, "/div:0/p:0");

        fn render(doc: &mut DioxusDocument, show: bool) {
            doc.vdom.in_runtime(|| *SHOW.write() = show);
            doc.vdom.mark_dirty(ScopeId::APP);
            doc.poll(None);
        }

        // Removal drops the node, and its id with it (upstream #1185).
        render(&mut doc, false);
        assert!(doc.inner.borrow().get_node(removed).is_none());
        assert_eq!(doc.element_id(removed), None);
        assert!(doc.element_ids().iter().all(|(node, _)| *node != removed));
    }

    #[test]
    fn non_elements_read_no_id() {
        fn app() -> Element {
            rsx! {
                p { "text" }
            }
        }
        let doc = build(app);
        let (p, _) = doc.element_ids().pop().expect("the p element");
        let (root, text) = {
            let inner = doc.inner.borrow();
            (inner.root_node().id, inner.get_node(p).unwrap().children[0])
        };
        assert_eq!(doc.element_id(p).as_deref(), Some("/p:0"));
        assert_eq!(doc.element_id(text), None);
        assert_eq!(doc.element_id(root), None);
    }

    #[test]
    fn a_keyed_parent_anchors_its_unkeyed_children() {
        fn app() -> Element {
            rsx! {
                div { id: "box",
                    span {}
                    p {
                        b {}
                    }
                }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(
            &ids[4..],
            ["box", "box//span:0", "box//p:0", "box//p:0/b:0"]
        );
        assert_distinct(&ids);
    }

    #[test]
    fn the_nearest_keyed_ancestor_anchors() {
        fn app() -> Element {
            rsx! {
                div { id: "outer",
                    div { id: "inner",
                        i {}
                    }
                    u {}
                }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["outer", "inner", "inner//i:0", "outer//u:0"]);
        assert_distinct(&ids);
    }

    #[test]
    fn an_unusable_id_anchors_nothing() {
        fn app() -> Element {
            rsx! {
                div { id: "",
                    i {}
                }
                div { id: "a/b",
                    i {}
                }
                div { id: "dup" }
                div { id: "dup",
                    i {}
                }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(
            &ids[4..],
            [
                "/div:0",
                "/div:0/i:0",
                "/div:1",
                "/div:1/i:0",
                "dup",
                "/div:3",
                "/div:3/i:0",
            ]
        );
        assert_distinct(&ids);
    }

    #[test]
    fn a_component_root_under_a_keyed_element_restarts_at_its_chain() {
        #[component]
        fn Item() -> Element {
            rsx! { b {} }
        }
        fn app() -> Element {
            rsx! {
                div { id: "slot", Item {} }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["slot", "Item/b:0"]);
    }

    #[test]
    fn a_key_spelled_like_a_component_stays_distinct() {
        #[component]
        fn Item() -> Element {
            rsx! { b {} }
        }
        fn app() -> Element {
            rsx! {
                div { id: "Item",
                    b {}
                }
                Item {}
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["Item", "Item//b:0", "Item/b:0"]);
        assert_distinct(&ids);
    }

    #[test]
    fn a_dioxus_keyed_row_under_a_keyed_list_reads_its_key_segment() {
        fn app() -> Element {
            rsx! {
                ul { id: "list",
                    for name in ["ada", "grace"] {
                        li { key: "{name}", "{name}" }
                    }
                }
            }
        }
        let ids = ids(&build(app));
        assert_eq!(&ids[4..], ["list", "list//li[ada]", "list//li[grace]"]);
        assert_distinct(&ids);
    }

    #[test]
    fn an_edit_outside_the_anchor_keeps_the_anchored_id() {
        fn before() -> Element {
            rsx! {
                div { id: "box",
                    p {}
                }
            }
        }
        fn edited_outside() -> Element {
            rsx! {
                div {}
                section {
                    div { id: "box",
                        p {}
                    }
                }
            }
        }
        fn edited_inside() -> Element {
            rsx! {
                div { id: "box",
                    p {}
                    p {}
                }
            }
        }
        // The `p` left in place is the last element of each app.
        let last = |app: fn() -> Element| ids(&build(app)).pop().expect("the p element");
        assert_eq!(last(before), "box//p:0");
        assert_eq!(last(edited_outside), "box//p:0");
        assert_eq!(last(edited_inside), "box//p:1");
    }

    /// Author ids and Dioxus keys chosen to crowd the separators: slashes, colons, brackets and a
    /// component's name.
    fn adversarial() -> Element {
        #[component]
        fn Item() -> Element {
            rsx! {
                b { id: "Item" }
                i {}
            }
        }
        rsx! {
            div { id: "a//b",
                span {}
            }
            div { id: "x:1",
                for key in ["y[2]", "Item", "a//b", "/"] {
                    p { key: "{key}",
                        u {}
                    }
                }
            }
            section { id: "y[2]", Item {} }
            Item {}
            ul {
                for key in ["", "x:1"] {
                    li { key: "{key}", Item {} }
                }
            }
        }
    }

    #[test]
    fn no_id_the_walk_writes_holds_the_virtual_separator() {
        let ids = ids(&build(adversarial));
        assert!(ids.len() > 10, "the page holds its elements: {ids:?}");
        assert_distinct(&ids);
        for id in &ids {
            assert!(!id.contains(super::VIRTUAL_KEY_SEPARATOR), "{id:?}");
        }
    }

    #[test]
    fn a_virtual_node_id_spells_the_reserved_form_and_names_no_element() {
        use super::virtual_node_id;
        assert_eq!(virtual_node_id("", "k"), Some("///k".to_string()));
        assert_eq!(virtual_node_id("x", ""), None);
        assert_eq!(virtual_node_id("x", "a/b"), None);
        let ids = ids(&build(adversarial));
        for owner in &ids {
            let virtual_id = virtual_node_id(owner, "cell-3").expect("a key with no slash");
            assert_eq!(virtual_id, format!("{owner}///cell-3"));
            assert!(!ids.contains(&virtual_id), "{virtual_id:?}");
        }
    }
}
