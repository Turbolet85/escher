//! The UI graph: for the current screen of a [`DioxusDocument`], each agent-visible node → the
//! component that rendered it → the handlers it carries → the registered states those handlers
//! write → the nodes that read those states.
//!
//! The node set is the snapshot's ([`DioxusDocument::snapshot`]), keyed by stable id. Links 1 and
//! 2 are read from the vdom walk the stable ids run: the owning component, and the listeners the
//! element's dynamic attributes hold now. Links 3 and 4 are read for the states an app registered
//! through [`use_graph_state`](crate::use_graph_state): a state's writes are its declaration —
//! authored, shown from boot — each confirmed once a run of the handler writes it, and its
//! readers are the components its subscribers belong to: a state names each reading component
//! once, and every node such a component rendered names the state among its `reads`, so a
//! state's reading nodes are the nodes whose `component` it names. No state value, control value
//! or `NodeId` is held.

use rustc_hash::FxHashMap;

use blitz_dom::{BaseDocument, NodeId};
use dioxus_core::ScopeId;

use crate::DioxusDocument;
use crate::element_id::{ElementFacts, element_facts};
use crate::signal_registry::{DeclaredWrite, Dispatch, Registered, SignalRegistry, WriteRecord};

/// The UI graph of one screen. Built by [`DioxusDocument::graph`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiGraph {
    /// One node per snapshot node, in snapshot pre-order.
    pub nodes: Vec<GraphNode>,
    /// One state per registered state, in registration order.
    pub states: Vec<GraphState>,
}

/// One agent-visible node of a [`UiGraph`].
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    /// The node's stable element id, as its snapshot node reads it.
    pub id: String,
    /// The stable id of the element whose handler serves this node, where that is another
    /// element: a select for its options and groups. `None` where the node is its own.
    pub owner: Option<String>,
    /// The name of the component whose render holds the element; `None` outside every
    /// component the author wrote.
    pub component: Option<String>,
    /// The handlers the element carries now, in the order the vdom holds them.
    pub handlers: Vec<GraphHandler>,
    /// The registered states the node's component reads, by their graph names.
    pub reads: Vec<String>,
}

/// One handler of a [`GraphNode`].
#[derive(Debug, Clone, PartialEq)]
pub struct GraphHandler {
    /// The handler's event name, without its `on`.
    pub event: String,
    /// The registered states the handler is declared to write or was seen writing.
    pub writes: Vec<GraphWrite>,
}

/// One registered state a handler writes, as declared and as run.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphWrite {
    /// The state's graph name.
    pub state: String,
    /// Whether the state's declaration names this handler.
    pub declared: bool,
    /// Whether a run of this handler wrote the state.
    pub confirmed: bool,
    /// Where the declaration and the runs disagree; `None` where they agree.
    pub flag: Option<WriteFlag>,
}

/// How a handler's declared writes and its runs disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteFlag {
    /// A run wrote the state, and its declaration names no such handler.
    Undeclared,
    /// The declared handler ran in the session, and no run of it wrote the state.
    Unconfirmed,
}

impl WriteFlag {
    /// The flag's closed word.
    pub fn word(self) -> &'static str {
        match self {
            Self::Undeclared => "undeclared",
            Self::Unconfirmed => "unconfirmed",
        }
    }
}

/// One registered state of a [`UiGraph`].
#[derive(Debug, Clone, PartialEq)]
pub struct GraphState {
    /// The name it was registered under, `{name}:{k}` for the `k`-th later registration of a
    /// name its component already registered.
    pub name: String,
    /// The name of the component that registered it; `None` for the app root.
    pub component: Option<String>,
    /// Its declaration, as authored.
    pub declared: Vec<DeclaredWrite>,
    /// One writer per screen element whose handler a run confirmed writing it, or whose write is
    /// flagged, in snapshot pre-order. A declared write no run has confirmed or flagged is in
    /// [`declared`](Self::declared) and on its node's handler alone.
    pub writers: Vec<GraphWriter>,
    /// The names of the components that read it, each once, in the snapshot pre-order of their
    /// first node; a node of one reads the state, and names it among its [`GraphNode::reads`].
    /// The app root and dioxus-core's own wrappers carry no name, so a state they read is named
    /// in their nodes' `reads` alone.
    pub readers: Vec<String>,
}

/// One handler that writes a [`GraphState`], as declared and as run.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphWriter {
    /// The stable id of the element carrying the handler.
    pub id: String,
    /// The handler's event name.
    pub event: String,
    /// Whether the state's declaration names this handler.
    pub declared: bool,
    /// Whether a run of this handler wrote the state.
    pub confirmed: bool,
    /// Where the declaration and the runs disagree.
    pub flag: Option<WriteFlag>,
}

/// One registered state with what the graph resolves of it.
struct StateReading {
    name: String,
    component: Option<String>,
    registered: Registered,
    /// The (element, event) pairs a run of which wrote it.
    confirmed: Vec<(NodeId, &'static str)>,
}

impl DioxusDocument {
    /// The screen's [`UiGraph`], read on demand; it writes nothing.
    pub fn graph(&self) -> UiGraph {
        let snapshot = self.snapshot();
        let facts = element_facts(&self.vdom, &self.vdom_state, &self.inner.borrow());
        let registry = self
            .vdom
            .runtime()
            .has_context::<SignalRegistry>(ScopeId::ROOT);
        let (registered, writes, dispatches) = match &registry {
            Some(registry) => (
                registry.registered(),
                registry.writes(),
                registry.dispatches(),
            ),
            None => Default::default(),
        };
        let by_node: FxHashMap<NodeId, &ElementFacts> =
            facts.iter().map(|fact| (fact.node, fact)).collect();
        let by_id: FxHashMap<&str, &ElementFacts> =
            facts.iter().map(|fact| (fact.id.as_str(), fact)).collect();
        let components = scope_components(&facts);

        let doc = self.inner.borrow();
        let states = state_readings(registered, &writes, &by_node, &components, &doc);

        let mut nodes = Vec::new();
        let mut writers: Vec<Vec<GraphWriter>> = vec![Vec::new(); states.len()];
        let mut readers: Vec<Vec<String>> = vec![Vec::new(); states.len()];
        for snapshot_node in snapshot.nodes() {
            let fact = by_id.get(snapshot_node.id.as_str()).copied();
            let owner = fact
                .and_then(|fact| doc.select_of(fact.node))
                .and_then(|select| by_node.get(&select))
                .map(|select| select.id.clone());
            let mut handlers = Vec::new();
            let mut reads = Vec::new();
            if let Some(fact) = fact {
                handlers = node_handlers(fact, &states, &dispatches, &mut writers);
                reads = node_reads(fact, &states, &mut readers);
            }
            nodes.push(GraphNode {
                id: snapshot_node.id.clone(),
                owner,
                component: fact.and_then(|fact| fact.component).map(str::to_string),
                handlers,
                reads,
            });
        }

        UiGraph {
            nodes,
            states: graph_states(states, writers, readers),
        }
    }
}

/// Each registered state with its graph name — `{name}:{k}` for the `k`-th later registration of
/// a name its scope already registered — its component, and the writes a run confirmed.
fn state_readings(
    registered: Vec<Registered>,
    writes: &[WriteRecord],
    by_node: &FxHashMap<NodeId, &ElementFacts>,
    components: &FxHashMap<ScopeId, Option<&'static str>>,
    doc: &BaseDocument,
) -> Vec<StateReading> {
    let mut seen_names: FxHashMap<(&'static str, ScopeId), usize> = FxHashMap::default();
    registered
        .into_iter()
        .map(|registered| {
            let ordinal = seen_names
                .entry((registered.name, registered.scope))
                .or_default();
            let name = match *ordinal {
                0 => registered.name.to_string(),
                k => format!("{}:{k}", registered.name),
            };
            *ordinal += 1;
            let confirmed = writes
                .iter()
                .filter(|write| write.state == registered.key)
                .filter_map(|write| resolve_write(write, by_node, doc))
                .collect();
            StateReading {
                name,
                component: components
                    .get(&registered.scope)
                    .copied()
                    .flatten()
                    .map(str::to_string),
                registered,
                confirmed,
            }
        })
        .collect()
}

/// Whether a handler has run: a dispatch of its event passed through its element.
fn ran(dispatches: &[Dispatch], node: NodeId, event: &str) -> bool {
    dispatches
        .iter()
        .any(|dispatch| dispatch.event == event && dispatch.chain.contains(&node))
}

/// The handlers `fact` carries, each with the states it is declared to write or a run confirmed
/// writing; a write confirmed or flagged is also kept among its state's `writers`.
fn node_handlers(
    fact: &ElementFacts,
    states: &[StateReading],
    dispatches: &[Dispatch],
    writers: &mut [Vec<GraphWriter>],
) -> Vec<GraphHandler> {
    let mut handlers = Vec::new();
    for &event in &fact.listeners {
        let mut handler_writes = Vec::new();
        for (index, state) in states.iter().enumerate() {
            let Some(write) = handler_write(fact, event, state, dispatches) else {
                continue;
            };
            if write.confirmed || write.flag.is_some() {
                writers[index].push(GraphWriter {
                    id: fact.id.clone(),
                    event: event.to_string(),
                    declared: write.declared,
                    confirmed: write.confirmed,
                    flag: write.flag,
                });
            }
            handler_writes.push(write);
        }
        handlers.push(GraphHandler {
            event: event.to_string(),
            writes: handler_writes,
        });
    }
    handlers
}

/// The write of `state` by `fact`'s handler of `event`: declared, confirmed and flagged; `None`
/// where it is neither declared nor confirmed.
fn handler_write(
    fact: &ElementFacts,
    event: &'static str,
    state: &StateReading,
    dispatches: &[Dispatch],
) -> Option<GraphWrite> {
    let declared = state
        .registered
        .declared
        .iter()
        .any(|write| write.event == event && write.names(&fact.id));
    let confirmed = state.confirmed.contains(&(fact.node, event));
    if !declared && !confirmed {
        return None;
    }
    let flag = match (declared, confirmed) {
        (false, true) => Some(WriteFlag::Undeclared),
        (true, false) if ran(dispatches, fact.node, event) => Some(WriteFlag::Unconfirmed),
        _ => None,
    };
    Some(GraphWrite {
        state: state.name.clone(),
        declared,
        confirmed,
        flag,
    })
}

/// The states `fact`'s component reads, by their graph names; its component joins each such
/// state's `readers` once, in the pre-order of its first node.
fn node_reads(
    fact: &ElementFacts,
    states: &[StateReading],
    readers: &mut [Vec<String>],
) -> Vec<String> {
    let mut reads = Vec::new();
    let Some(scope) = fact.scope else {
        return reads;
    };
    for (index, state) in states.iter().enumerate() {
        if !reads_state(&state.registered, scope) {
            continue;
        }
        reads.push(state.name.clone());
        if let Some(component) = fact.component
            && !readers[index].iter().any(|name| name == component)
        {
            readers[index].push(component.to_string());
        }
    }
    reads
}

/// The graph's states, in registration order, each with its writers and readers.
fn graph_states(
    states: Vec<StateReading>,
    writers: Vec<Vec<GraphWriter>>,
    readers: Vec<Vec<String>>,
) -> Vec<GraphState> {
    states
        .into_iter()
        .zip(writers)
        .zip(readers)
        .map(|((state, writers), readers)| GraphState {
            name: state.name,
            component: state.component,
            declared: state.registered.declared.to_vec(),
            writers,
            readers,
        })
        .collect()
}

/// Each scope the walk met, with its component's name.
fn scope_components(facts: &[ElementFacts]) -> FxHashMap<ScopeId, Option<&'static str>> {
    facts
        .iter()
        .filter_map(|fact| fact.scope.map(|scope| (scope, fact.component)))
        .collect()
}

/// The (element, event) a write record resolves to: the first element of its chain carrying a
/// listener of its event whose owning scope is the writing scope, else the first carrying one.
/// `None` for a write outside any dispatch, and for one whose chain holds a node that no longer
/// resolves before that element: the gone node may have been the writer, so the write is not
/// handed to an ancestor.
fn resolve_write(
    write: &WriteRecord,
    by_node: &FxHashMap<NodeId, &ElementFacts>,
    doc: &BaseDocument,
) -> Option<(NodeId, &'static str)> {
    let dispatch = write.dispatch.as_ref()?;
    let listening = |at: &usize| {
        by_node
            .get(&dispatch.chain[*at])
            .is_some_and(|fact| fact.listeners.contains(&dispatch.event))
    };
    let candidates: Vec<usize> = (0..dispatch.chain.len()).filter(listening).collect();
    let at = candidates
        .iter()
        .copied()
        .find(|at| write.scope.is_some() && by_node[&dispatch.chain[*at]].scope == write.scope)
        .or_else(|| candidates.first().copied())?;
    let gone_before = dispatch.chain[..at]
        .iter()
        .any(|node| doc.get_node(*node).is_none());
    (!gone_before).then_some((dispatch.chain[at], dispatch.event))
}

/// Whether a subscriber of `state` other than the registry's own watcher belongs to `scope`.
fn reads_state(state: &Registered, scope: ScopeId) -> bool {
    let mut reads = false;
    state.subscribers.visit(|subscriber| {
        if *subscriber != state.watcher && subscriber.origin_scope() == scope {
            reads = true;
        }
    });
    reads
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_registry::tests::{click, node};
    use crate::use_graph_state;
    use blitz_dom::{Document, DocumentConfig};
    use dioxus::prelude::*;

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc.inner_mut().resolve(0.0);
        doc
    }

    #[track_caller]
    fn graph_node<'g>(graph: &'g UiGraph, id: &str) -> &'g GraphNode {
        graph
            .nodes
            .iter()
            .find(|node| node.id == id)
            .unwrap_or_else(|| panic!("{id:?} is a graph node of {graph:#?}"))
    }

    #[track_caller]
    fn state<'g>(graph: &'g UiGraph, name: &str) -> &'g GraphState {
        graph
            .states
            .iter()
            .find(|state| state.name == name)
            .unwrap_or_else(|| panic!("{name:?} is a graph state of {graph:#?}"))
    }

    fn events(node: &GraphNode) -> Vec<&str> {
        node.handlers
            .iter()
            .map(|handler| handler.event.as_str())
            .collect()
    }

    #[track_caller]
    fn write<'g>(graph: &'g UiGraph, id: &str, event: &str, state: &str) -> &'g GraphWrite {
        graph_node(graph, id)
            .handlers
            .iter()
            .find(|handler| handler.event == event)
            .and_then(|handler| handler.writes.iter().find(|write| write.state == state))
            .unwrap_or_else(|| panic!("{id:?} {event:?} writes {state:?} in {graph:#?}"))
    }

    const BUMP: &[DeclaredWrite] = &[DeclaredWrite::new("bump", "click")];

    #[component]
    fn Shown() -> Element {
        let mut count = use_signal(|| 0u32);
        use_graph_state("count", BUMP, count.subscribers());
        rsx! {
            div { id: "card",
                p { id: "value", "{count}" }
                button { id: "bump", onclick: move |_| count += 1, "Bump" }
            }
        }
    }

    fn counter() -> Element {
        rsx! { Shown {} }
    }

    #[test]
    fn link_1_names_each_nodes_component() {
        let graph = build(counter).graph();
        assert_eq!(
            graph_node(&graph, "value").component.as_deref(),
            Some("Shown")
        );
        assert_eq!(
            graph_node(&graph, "bump").component.as_deref(),
            Some("Shown")
        );
        assert!(graph.nodes.iter().all(|node| node.owner.is_none()));
        let snapshot_ids: Vec<String> = build(counter)
            .snapshot()
            .nodes()
            .map(|node| node.id.clone())
            .collect();
        let graph_ids: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).collect();
        assert_eq!(
            graph_ids, snapshot_ids,
            "the snapshot's nodes, in its order"
        );
    }

    #[test]
    fn link_2_reads_the_current_listeners_mounted_included_and_a_removed_one_gone() {
        fn page() -> Element {
            let mut armed = use_signal(|| true);
            let spread: Vec<Attribute> = if armed() {
                vec![dioxus::html::onclick(move |_: MouseEvent| {})]
            } else {
                Vec::new()
            };
            rsx! {
                button { id: "toggled", ..spread, "Toggled" }
                button { id: "disarm", onclick: move |_| armed.set(false), "Disarm" }
                div { id: "held", onmounted: move |_| {}, "Held" }
            }
        }
        let mut doc = build(page);
        let graph = doc.graph();
        assert_eq!(events(graph_node(&graph, "toggled")), ["click"]);
        assert_eq!(events(graph_node(&graph, "held")), ["mounted"]);
        let disarm = node(&doc, "disarm");
        click(&mut doc, disarm);
        doc.inner_mut().resolve(0.0);
        let graph = doc.graph();
        assert!(events(graph_node(&graph, "toggled")).is_empty());
    }

    #[test]
    fn a_declared_write_reads_declared_before_a_dispatch_and_confirmed_after() {
        let mut doc = build(counter);
        let graph = doc.graph();
        let before = write(&graph, "bump", "click", "count");
        assert!(before.declared && !before.confirmed && before.flag.is_none());
        let count = state(&graph, "count");
        assert_eq!(count.component.as_deref(), Some("Shown"));
        assert_eq!(count.declared, BUMP);
        assert!(
            count.writers.is_empty(),
            "no writer before a run confirms one"
        );

        let bump = node(&doc, "bump");
        click(&mut doc, bump);
        let graph = doc.graph();
        let after = write(&graph, "bump", "click", "count");
        assert!(after.declared && after.confirmed && after.flag.is_none());
        let writer = &state(&graph, "count").writers[0];
        assert_eq!(
            (writer.id.as_str(), writer.event.as_str()),
            ("bump", "click")
        );
        assert!(writer.confirmed && writer.flag.is_none());
    }

    #[test]
    fn link_4_two_reader_components_read_the_state_and_the_writer_does_not() {
        #[component]
        fn Reader(label: &'static str, shared: Signal<u32>) -> Element {
            rsx! { p { id: "{label}", "{shared}" } }
        }
        #[component]
        fn Writer(shared: Signal<u32>) -> Element {
            rsx! { button { id: "writer", onclick: move |_| shared += 1, "Write" } }
        }
        fn page() -> Element {
            const WRITES: &[DeclaredWrite] = &[DeclaredWrite::new("writer", "click")];
            let shared = use_signal(|| 0u32);
            use_graph_state("shared", WRITES, shared.subscribers());
            rsx! {
                Reader { label: "first", shared }
                Reader { label: "second", shared }
                Writer { shared }
            }
        }
        let graph = build(page).graph();
        let shared = state(&graph, "shared");
        assert_eq!(shared.readers, ["Reader"], "each reading component once");
        assert_eq!(shared.component, None, "the app root registered it");
        assert_eq!(graph_node(&graph, "first").reads, ["shared"]);
        assert!(graph_node(&graph, "writer").reads.is_empty());
        assert_eq!(
            graph_node(&graph, "first").component.as_deref(),
            Some("Reader")
        );
    }

    #[test]
    fn an_option_reads_its_select_as_owner() {
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
        let graph = build(page).graph();
        for id in ["one", "group", "two"] {
            assert_eq!(
                graph_node(&graph, id).owner.as_deref(),
                Some("pick"),
                "{id}"
            );
        }
        assert_eq!(graph_node(&graph, "pick").owner, None);
        assert_eq!(events(graph_node(&graph, "pick")), ["input"]);
    }

    #[test]
    fn a_prefix_declaration_names_its_elements_alone() {
        fn page() -> Element {
            const WRITES: &[DeclaredWrite] = &[DeclaredWrite::new("cell-*", "click")];
            let mut picked = use_signal(|| 0usize);
            use_graph_state("picked", WRITES, picked.subscribers());
            rsx! {
                for at in 0..3usize {
                    div { id: "cell-{at}", onclick: move |_| picked.set(at), "{at}" }
                }
                div { id: "cells", onclick: move |_| {}, "{picked}" }
            }
        }
        let mut doc = build(page);
        let graph = doc.graph();
        let declared: Vec<&str> = ["cell-0", "cell-1", "cell-2", "cells"]
            .into_iter()
            .filter(|id| {
                graph_node(&graph, id).handlers[0]
                    .writes
                    .iter()
                    .any(|write| write.state == "picked" && write.declared)
            })
            .collect();
        assert_eq!(declared, ["cell-0", "cell-1", "cell-2"]);
        assert!(state(&graph, "picked").writers.is_empty());
        let cell = node(&doc, "cell-1");
        click(&mut doc, cell);
        let graph = doc.graph();
        let writers: Vec<&str> = state(&graph, "picked")
            .writers
            .iter()
            .map(|writer| writer.id.as_str())
            .collect();
        assert_eq!(writers, ["cell-1"], "the one confirmed writer");
        let cells = graph_node(&graph, "cells");
        assert!(
            cells.handlers[0].writes.is_empty(),
            "no write declared on `cells`"
        );
    }

    #[test]
    fn a_declaration_that_drifts_from_the_code_reads_both_flags() {
        fn page() -> Element {
            // `stale` is declared written by `stale-btn`, which writes nothing; `quiet` is
            // written by `loud-btn`, whose writes no declaration names.
            const STALE: &[DeclaredWrite] = &[DeclaredWrite::new("stale-btn", "click")];
            let stale = use_signal(|| 0u32);
            use_graph_state("stale", STALE, stale.subscribers());
            let mut quiet = use_signal(|| 0u32);
            use_graph_state("quiet", &[], quiet.subscribers());
            rsx! {
                button { id: "stale-btn", onclick: move |_| {}, "{stale}" }
                button { id: "loud-btn", onclick: move |_| quiet += 1, "{quiet}" }
            }
        }
        let mut doc = build(page);
        let graph = doc.graph();
        let before = write(&graph, "stale-btn", "click", "stale");
        assert!(before.flag.is_none(), "no flag before the handler runs");
        assert!(graph_node(&graph, "loud-btn").handlers[0].writes.is_empty());

        for id in ["stale-btn", "loud-btn"] {
            let target = node(&doc, id);
            click(&mut doc, target);
        }
        let graph = doc.graph();
        let stale = write(&graph, "stale-btn", "click", "stale");
        assert_eq!(stale.flag, Some(WriteFlag::Unconfirmed));
        assert!(stale.declared && !stale.confirmed);
        let quiet = write(&graph, "loud-btn", "click", "quiet");
        assert_eq!(quiet.flag, Some(WriteFlag::Undeclared));
        assert!(!quiet.declared && quiet.confirmed);
        assert_eq!(
            state(&graph, "quiet").writers[0].flag,
            Some(WriteFlag::Undeclared)
        );
        assert_eq!(WriteFlag::Undeclared.word(), "undeclared");
        assert_eq!(WriteFlag::Unconfirmed.word(), "unconfirmed");
    }

    #[test]
    fn a_write_by_an_element_gone_since_is_not_handed_to_its_ancestor() {
        fn page() -> Element {
            let mut shown = use_signal(|| true);
            use_graph_state("shown", &[], shown.subscribers());
            rsx! {
                div { id: "outer", onclick: move |_| {},
                    if shown() {
                        button { id: "hide", onclick: move |_| shown.set(false), "Hide" }
                    }
                }
            }
        }
        let mut doc = build(page);
        let hide = node(&doc, "hide");
        click(&mut doc, hide);
        let graph = doc.graph();
        assert!(
            graph.nodes.iter().all(|node| node.id != "hide"),
            "the button is gone"
        );
        let outer = graph_node(&graph, "outer");
        assert_eq!(events(outer), ["click"]);
        assert!(
            outer.handlers[0].writes.is_empty(),
            "no write handed to the ancestor"
        );
        assert!(state(&graph, "shown").writers.is_empty());
    }

    #[test]
    fn a_repeated_name_reads_its_ordinal_and_a_page_with_no_state_reads_none() {
        fn page() -> Element {
            let first = use_signal(|| 0u32);
            use_graph_state("same", &[], first.subscribers());
            let second = use_signal(|| 0u32);
            use_graph_state("same", &[], second.subscribers());
            rsx! { p { id: "shown", "{first}{second}" } }
        }
        let graph = build(page).graph();
        let names: Vec<&str> = graph
            .states
            .iter()
            .map(|state| state.name.as_str())
            .collect();
        assert_eq!(names, ["same", "same:1"]);
        assert_eq!(graph_node(&graph, "shown").reads, ["same", "same:1"]);

        fn plain() -> Element {
            rsx! { p { id: "shown", "text" } }
        }
        let graph = build(plain).graph();
        assert!(graph.states.is_empty());
        assert!(graph.nodes.iter().all(|node| node.reads.is_empty()));
    }
}
