//! The escher signal registry: the states an app hands it by name, each with the authored
//! declaration of the handlers that write it, and a record of every write each one takes.
//!
//! dioxus-core keeps a component's hooks crate-private, so a hook-local signal cannot be found
//! from outside its component. An app that wants its state graphed registers each signal's
//! subscriber list through [`use_graph_state`]; the registry, a context the first registering
//! hook provides at the vdom's root, holds a reactive context subscribed to that list whose
//! callback records one write against the event the bridge is dispatching at that moment — or
//! against none, for a write no event's handler made. A registered state's entry leaves with the
//! scope that registered it.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use blitz_dom::NodeId;
use dioxus_core::{
    ReactiveContext, Runtime, ScopeId, Subscribers, current_scope_id, provide_root_context,
    try_consume_context, use_drop, use_hook,
};

/// One handler an app declares writes a registered state: the element carrying it and its
/// event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredWrite {
    /// The stable id of the element carrying the handler, or an id prefix ending in `*`, which
    /// names every element whose id starts with what precedes it.
    pub id: &'static str,
    /// The handler's event name, without its `on` (`click`, `keydown`, `input`).
    pub event: &'static str,
}

impl DeclaredWrite {
    /// The handler of `event` on the element `id` (or on every element an `id` ending in `*`
    /// prefixes).
    pub const fn new(id: &'static str, event: &'static str) -> Self {
        Self { id, event }
    }

    /// Whether this declaration names the element whose stable id is `element`.
    pub(crate) fn names(&self, element: &str) -> bool {
        match self.id.strip_suffix('*') {
            Some(prefix) => element.starts_with(prefix),
            None => element == self.id,
        }
    }
}

/// Registers the state whose subscriber list is `subscribers` under `name`, declaring `writers`
/// the handlers that write it, for the life of the calling component. A hook: call it on every
/// render, in a fixed place, as any other. A signal's list is its
/// `Readable::subscribers()`; an empty declaration declares no writer.
pub fn use_graph_state(
    name: &'static str,
    writers: &'static [DeclaredWrite],
    subscribers: Subscribers,
) {
    let (registry, key) = use_hook(move || {
        let registry = try_consume_context::<SignalRegistry>()
            .unwrap_or_else(|| provide_root_context(SignalRegistry::default()));
        let key = registry.register(name, writers, current_scope_id(), subscribers);
        (registry, key)
    });
    use_drop(move || registry.unregister(key));
}

/// The event the bridge is dispatching: its chain, target first, and its name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Dispatch {
    pub(crate) chain: Vec<NodeId>,
    pub(crate) event: &'static str,
}

/// One write a registered state took.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WriteRecord {
    /// The written state's registry key.
    pub(crate) state: usize,
    /// The dispatch standing at the write; `None` for a write no event's handler made.
    pub(crate) dispatch: Option<Dispatch>,
    /// The scope on the runtime's stack at the write: the writing handler's.
    pub(crate) scope: Option<ScopeId>,
}

/// A registered state as the graph reads it.
#[derive(Clone)]
pub(crate) struct Registered {
    pub(crate) key: usize,
    pub(crate) name: &'static str,
    pub(crate) scope: ScopeId,
    pub(crate) declared: &'static [DeclaredWrite],
    pub(crate) subscribers: Subscribers,
    /// The registry's own subscriber, which is no reader.
    pub(crate) watcher: ReactiveContext,
}

#[derive(Default)]
struct State {
    next_key: usize,
    registered: Vec<Registered>,
    current: Option<Dispatch>,
    /// Every dispatch seen, once each.
    dispatches: Vec<Dispatch>,
    /// Every write taken, once each.
    writes: Vec<WriteRecord>,
}

/// The registry an app's registered states live in, provided at its vdom's root.
#[derive(Clone, Default)]
pub struct SignalRegistry {
    state: Arc<Mutex<State>>,
}

impl std::fmt::Debug for SignalRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.lock();
        f.debug_struct("SignalRegistry")
            .field("registered", &state.registered.len())
            .field("dispatches", &state.dispatches.len())
            .field("writes", &state.writes.len())
            .finish()
    }
}

fn lock(state: &Mutex<State>) -> MutexGuard<'_, State> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

impl SignalRegistry {
    fn lock(&self) -> MutexGuard<'_, State> {
        lock(&self.state)
    }

    fn register(
        &self,
        name: &'static str,
        declared: &'static [DeclaredWrite],
        scope: ScopeId,
        subscribers: Subscribers,
    ) -> usize {
        let key = {
            let mut state = self.lock();
            state.next_key += 1;
            state.next_key
        };
        let shared = Arc::clone(&self.state);
        let watcher = ReactiveContext::new_with_callback(
            move || record_write(&shared, key),
            scope,
            std::panic::Location::caller(),
        );
        watcher.subscribe(subscribers.clone());
        self.lock().registered.push(Registered {
            key,
            name,
            scope,
            declared,
            subscribers,
            watcher,
        });
        key
    }

    fn unregister(&self, key: usize) {
        let mut state = self.lock();
        state.registered.retain(|registered| registered.key != key);
        state.writes.retain(|write| write.state != key);
    }

    /// Sets the dispatch every write until [`end_dispatch`](Self::end_dispatch) is recorded
    /// against, and counts it seen.
    pub(crate) fn begin_dispatch(&self, chain: &[NodeId], event: &'static str) {
        let dispatch = Dispatch {
            chain: chain.to_vec(),
            event,
        };
        let mut state = self.lock();
        if !state.dispatches.contains(&dispatch) {
            state.dispatches.push(dispatch.clone());
        }
        state.current = Some(dispatch);
    }

    pub(crate) fn end_dispatch(&self) {
        self.lock().current = None;
    }

    /// The registered states, in registration order.
    pub(crate) fn registered(&self) -> Vec<Registered> {
        self.lock().registered.clone()
    }

    /// Every write taken, in the order first taken.
    pub(crate) fn writes(&self) -> Vec<WriteRecord> {
        self.lock().writes.clone()
    }

    /// Every dispatch seen, in the order first seen.
    pub(crate) fn dispatches(&self) -> Vec<Dispatch> {
        self.lock().dispatches.clone()
    }
}

/// The watcher's callback: one write of the state `key`, against the dispatch standing now.
fn record_write(state: &Mutex<State>, key: usize) {
    let scope = Runtime::try_current().and_then(|runtime| runtime.try_current_scope_id());
    let mut state = lock(state);
    let record = WriteRecord {
        state: key,
        dispatch: state.current.clone(),
        scope,
    };
    if !state.writes.contains(&record) {
        state.writes.push(record);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::DioxusDocument;
    use blitz_dom::{Document, DocumentConfig};
    use blitz_traits::events::{DomEvent, DomEventData};
    use dioxus::prelude::*;

    const BUMP_WRITES: &[DeclaredWrite] = &[DeclaredWrite::new("bump", "click")];

    fn counter() -> Element {
        let mut count = use_signal(|| 0u32);
        use_graph_state("count", BUMP_WRITES, count.subscribers());
        let mut mounted = use_signal(|| false);
        use_graph_state("mounted", &[], mounted.subscribers());
        rsx! {
            p { id: "value", onmounted: move |_| mounted.set(true), "{count}" }
            button { id: "bump", onclick: move |_| count += 1, "Bump" }
        }
    }

    fn build(app: fn() -> Element) -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
        doc.initial_build();
        doc
    }

    pub(crate) fn registry(doc: &DioxusDocument) -> SignalRegistry {
        doc.vdom
            .runtime()
            .has_context::<SignalRegistry>(ScopeId::ROOT)
            .expect("a registered state provides the registry")
    }

    pub(crate) fn node(doc: &DioxusDocument, id: &str) -> NodeId {
        doc.element_ids()
            .into_iter()
            .find_map(|(node, read)| (read == id).then_some(node))
            .unwrap_or_else(|| panic!("{id:?} is an element"))
    }

    /// Dispatches a click on `target`, bubbling, through the bridge's event handler, then polls.
    pub(crate) fn click(doc: &mut DioxusDocument, target: NodeId) {
        let (data, chain) = {
            let inner = doc.inner.borrow();
            let node = inner.get_node(target).expect("the target resolves");
            (
                node.synthetic_click_event_data(Default::default()),
                inner.node_chain(target),
            )
        };
        let mut event = DomEvent::new(target, DomEventData::Click(data));
        doc.dispatch_to_vdom(&chain, &mut event);
        doc.poll(None);
    }

    #[test]
    fn a_handlers_write_is_recorded_against_its_dispatch_and_one_outside_any_against_none() {
        let mut doc = build(counter);
        let registry = registry(&doc);
        let names: Vec<&str> = registry.registered().iter().map(|r| r.name).collect();
        assert_eq!(names, ["count", "mounted"]);
        let count_key = registry.registered()[0].key;
        let mounted_key = registry.registered()[1].key;

        let mounted: Vec<WriteRecord> = registry
            .writes()
            .into_iter()
            .filter(|write| write.state == mounted_key)
            .collect();
        assert_eq!(mounted.len(), 1, "the mounted handler's write");
        assert_eq!(mounted[0].dispatch, None, "no event was dispatching");

        let bump = node(&doc, "bump");
        click(&mut doc, bump);
        let counted: Vec<WriteRecord> = registry
            .writes()
            .into_iter()
            .filter(|write| write.state == count_key)
            .collect();
        assert_eq!(counted.len(), 1, "one write record of the click");
        let dispatch = counted[0].dispatch.as_ref().expect("the click's dispatch");
        assert_eq!(dispatch.event, "click");
        assert_eq!(dispatch.chain.first(), Some(&bump));
        assert!(counted[0].scope.is_some(), "the writing handler's scope");
        assert_eq!(registry.dispatches().len(), 1);

        click(&mut doc, bump);
        assert_eq!(
            registry
                .writes()
                .iter()
                .filter(|write| write.state == count_key)
                .count(),
            1,
            "the same write twice is one record"
        );
    }

    #[test]
    fn a_states_entry_leaves_with_its_scope_and_its_debug_prints_counts() {
        static SHOW: GlobalSignal<bool> = Signal::global(|| true);
        #[component]
        fn Held() -> Element {
            let flag = use_signal(|| 7u32);
            use_graph_state("flag", &[], flag.subscribers());
            rsx! { p { "{flag}" } }
        }
        fn app() -> Element {
            rsx! {
                if SHOW() {
                    Held {}
                }
            }
        }
        let mut doc = build(app);
        let registry = registry(&doc);
        assert_eq!(registry.registered().len(), 1);
        assert_eq!(
            format!("{registry:?}"),
            "SignalRegistry { registered: 1, dispatches: 0, writes: 0 }"
        );
        doc.vdom.in_runtime(|| *SHOW.write() = false);
        doc.vdom.mark_dirty(ScopeId::APP);
        doc.poll(None);
        assert!(registry.registered().is_empty());
    }

    #[test]
    fn a_declaration_names_its_element_or_its_prefix() {
        let exact = DeclaredWrite::new("cell-A1", "click");
        assert!(exact.names("cell-A1") && !exact.names("cell-A10"));
        let prefix = DeclaredWrite::new("cell-*", "click");
        assert!(prefix.names("cell-A1") && prefix.names("cell-") && !prefix.names("cells-grid"));
    }

    #[test]
    fn a_page_with_no_registered_state_has_no_registry() {
        fn app() -> Element {
            let mut count = use_signal(|| 0u32);
            rsx! { button { id: "bump", onclick: move |_| count += 1, "{count}" } }
        }
        let mut doc = build(app);
        let bump = node(&doc, "bump");
        click(&mut doc, bump);
        assert!(
            doc.vdom
                .runtime()
                .has_context::<SignalRegistry>(ScopeId::ROOT)
                .is_none()
        );
        let text = doc
            .inner
            .borrow()
            .get_node(bump)
            .map(|node| node.text_content());
        assert_eq!(text.as_deref(), Some("1"), "the click acted");
    }
}
