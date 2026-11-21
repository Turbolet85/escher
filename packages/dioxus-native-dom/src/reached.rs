//! The reached-handler record: every handler a dispatch passed through, by the stable id of the
//! element carrying it and its event, in the order first reached.
//!
//! A root context the document provides only when asked ([`DioxusDocument::record_reached`]),
//! so a document that never asks carries none. The bridge resolves a dispatch's ids as it hands
//! the event to the vdom, before any handler runs, so an id is resolved while its element still
//! stands: a handler whose own run removes its element is still recorded. It writes them once the
//! dispatch returns, reading whether it was stopped: a bubbling dispatch nothing stopped reached
//! every element of its chain whose current listeners hold its event, the bubbling ancestors
//! included; one something stopped is recorded at its innermost such element alone; and a
//! non-bubbling event at its target alone. The stop is read as one flag after the dispatch, never
//! per listener, so where an ancestor stops the event, the listeners between it and the target
//! that ran read unreached. A handler's panic unwinding out of the dispatch writes it as stopped:
//! the innermost element alone. No `NodeId` is held past the dispatch.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use blitz_dom::{BaseDocument, NodeId};
use dioxus_core::{ScopeId, VirtualDom};

use crate::DioxusDocument;
use crate::element_id::element_listener_ids;
use crate::mutation_writer::DioxusState;

/// One handler a dispatch reached: the element carrying it and its event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachedHandler {
    /// The stable id of the element carrying the handler, resolved at the dispatch.
    pub id: String,
    /// The handler's event name, without its `on`.
    pub event: String,
}

/// A dispatch's elements whose listeners hold its event, resolved before it runs: each by its
/// node and stable id, in the chain's order, innermost first.
pub(crate) struct Resolved {
    event: &'static str,
    entries: Vec<(NodeId, String)>,
}

/// A dispatch's write, made once the dispatch returns ([`PendingWrite::finish`]) or, where a
/// handler's panic unwinds out of it, on drop, read as a stopped dispatch: the listeners past the
/// one that panicked never ran, and which one panicked is not known.
pub(crate) struct PendingWrite {
    record: ReachedRecord,
    resolved: Option<Resolved>,
    target: NodeId,
    bubbles: bool,
}

impl PendingWrite {
    /// Writes the dispatch as it returned, `propagates` read after it.
    pub(crate) fn finish(mut self, propagates: bool) {
        if let Some(resolved) = self.resolved.take() {
            self.record
                .record(resolved, self.target, self.bubbles, propagates);
        }
    }
}

impl Drop for PendingWrite {
    fn drop(&mut self) {
        if let Some(resolved) = self.resolved.take() {
            self.record
                .record(resolved, self.target, self.bubbles, false);
        }
    }
}

/// The record a document's dispatches write, provided at its vdom's root.
#[derive(Clone, Default)]
pub(crate) struct ReachedRecord {
    lists: Arc<Mutex<Lists>>,
}

/// The record's two lists: every handler reached since the record was switched on, and those
/// reached since the step list was last drained, each once per list.
#[derive(Default)]
struct Lists {
    reached: Vec<ReachedHandler>,
    step: Vec<ReachedHandler>,
}

fn push_once(list: &mut Vec<ReachedHandler>, id: &str, event: &str) {
    if !list
        .iter()
        .any(|handler| handler.id == id && handler.event == event)
    {
        list.push(ReachedHandler {
            id: id.to_string(),
            event: event.to_string(),
        });
    }
}

impl std::fmt::Debug for ReachedRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReachedRecord")
            .field("reached", &self.lock().reached.len())
            .finish()
    }
}

impl ReachedRecord {
    fn lock(&self) -> MutexGuard<'_, Lists> {
        self.lists.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Resolves, before the dispatch, each element of `chain` whose current listeners hold
    /// `event`, in the chain's order, by its node and its stable id.
    pub(crate) fn resolve(
        dom: &VirtualDom,
        state: &DioxusState,
        doc: &BaseDocument,
        chain: &[NodeId],
        event: &'static str,
    ) -> Resolved {
        let elements = element_listener_ids(dom, state, doc);
        let entries = chain
            .iter()
            .filter_map(|node| {
                elements
                    .iter()
                    .find(|(element, _, listeners)| element == node && listeners.contains(&event))
                    .map(|(_, id, _)| (*node, id.clone()))
            })
            .collect();
        Resolved { event, entries }
    }

    /// The write of a dispatch whose ids `resolved` holds, pending until the dispatch returns.
    pub(crate) fn pending(
        &self,
        resolved: Resolved,
        target: NodeId,
        bubbles: bool,
    ) -> PendingWrite {
        PendingWrite {
            record: self.clone(),
            resolved: Some(resolved),
            target,
            bubbles,
        }
    }

    /// Writes what the dispatch `resolved` was taken for reached, once each, after it returned:
    /// for a bubbling event every resolved element when it still `propagates`, else the innermost
    /// alone; for a non-bubbling one the `target` the vdom handed it to, when it holds the
    /// listener.
    fn record(&self, resolved: Resolved, target: NodeId, bubbles: bool, propagates: bool) {
        let Resolved { event, entries } = resolved;
        let reached: Vec<String> = if !bubbles {
            entries
                .into_iter()
                .filter(|(node, _)| *node == target)
                .take(1)
                .map(|(_, id)| id)
                .collect()
        } else if propagates {
            entries.into_iter().map(|(_, id)| id).collect()
        } else {
            entries.into_iter().take(1).map(|(_, id)| id).collect()
        };
        let mut lists = self.lock();
        for id in &reached {
            push_once(&mut lists.reached, id, event);
            push_once(&mut lists.step, id, event);
        }
    }

    fn handlers(&self) -> Vec<ReachedHandler> {
        self.lock().reached.clone()
    }

    fn take_step(&self) -> Vec<ReachedHandler> {
        std::mem::take(&mut self.lock().step)
    }
}

impl DioxusDocument {
    /// Switches the reached-handler record on: from now on every dispatch records the handlers it
    /// reaches. Switching it on again keeps the one record.
    pub fn record_reached(&mut self) {
        let runtime = self.vdom.runtime();
        if runtime
            .has_context::<ReachedRecord>(ScopeId::ROOT)
            .is_none()
        {
            runtime.provide_context(ScopeId::ROOT, ReachedRecord::default());
        }
    }

    /// The handlers the dispatches reached since [`record_reached`](Self::record_reached), each
    /// once, in the order first reached; empty where the record was never switched on.
    pub fn reached_handlers(&self) -> Vec<ReachedHandler> {
        self.vdom
            .runtime()
            .has_context::<ReachedRecord>(ScopeId::ROOT)
            .map(|record| record.handlers())
            .unwrap_or_default()
    }

    /// The handlers the dispatches reached since this was last asked, each once, in the order
    /// first reached; empty where the record was never switched on.
    /// [`reached_handlers`](Self::reached_handlers) reads as before.
    pub fn take_step_reached(&self) -> Vec<ReachedHandler> {
        self.vdom
            .runtime()
            .has_context::<ReachedRecord>(ScopeId::ROOT)
            .map(|record| record.take_step())
            .unwrap_or_default()
    }
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
        doc
    }

    fn reached(id: &str, event: &str) -> ReachedHandler {
        ReachedHandler {
            id: id.to_string(),
            event: event.to_string(),
        }
    }

    fn bump() -> Element {
        let mut count = use_signal(|| 0u32);
        rsx! { button { id: "bump", onclick: move |_| count += 1, "{count}" } }
    }

    #[test]
    fn a_click_is_recorded_with_the_record_on_and_nothing_with_it_off() {
        let mut doc = build(bump);
        let target = node(&doc, "bump");
        click(&mut doc, target);
        assert!(doc.reached_handlers().is_empty(), "off by default");

        doc.record_reached();
        doc.record_reached();
        click(&mut doc, target);
        click(&mut doc, target);
        assert_eq!(doc.reached_handlers(), [reached("bump", "click")]);
        let text = doc
            .inner
            .borrow()
            .get_node(target)
            .map(|node| node.text_content());
        assert_eq!(text.as_deref(), Some("3"), "every click acted");
    }

    #[test]
    fn a_bubbling_dispatch_records_its_ancestors_listener_of_the_same_event() {
        fn app() -> Element {
            let mut outer = use_signal(|| 0u32);
            let mut inner = use_signal(|| 0u32);
            rsx! {
                div { id: "outer", onclick: move |_| outer += 1, onkeydown: move |_| {},
                    button { id: "inner", onclick: move |_| inner += 1, "{inner} {outer}" }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        click(&mut doc, target);
        assert_eq!(
            doc.reached_handlers(),
            [reached("inner", "click"), reached("outer", "click")],
            "the target first, then its ancestor, and no other event"
        );
    }

    #[test]
    fn a_handler_whose_run_removes_its_element_is_recorded_by_its_id() {
        fn app() -> Element {
            let mut shown = use_signal(|| true);
            rsx! {
                if shown() {
                    button { id: "dismiss", onclick: move |_| shown.set(false), "Dismiss" }
                }
                p { id: "after" }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "dismiss");
        click(&mut doc, target);
        assert!(
            doc.inner.borrow().get_node(target).is_none(),
            "the click removed its element"
        );
        assert_eq!(doc.reached_handlers(), [reached("dismiss", "click")]);
    }

    #[test]
    fn two_clicks_of_one_handler_drain_one_entry_each_time() {
        let mut doc = build(bump);
        doc.record_reached();
        let target = node(&doc, "bump");
        click(&mut doc, target);
        click(&mut doc, target);
        assert_eq!(doc.take_step_reached(), [reached("bump", "click")]);
        assert!(doc.take_step_reached().is_empty(), "drained");
        click(&mut doc, target);
        assert_eq!(doc.take_step_reached(), [reached("bump", "click")]);
        click(&mut doc, target);
        assert_eq!(doc.take_step_reached(), [reached("bump", "click")]);
        assert_eq!(
            doc.reached_handlers(),
            [reached("bump", "click")],
            "the deduplicated record is unmoved by the drains"
        );
    }

    #[test]
    fn a_bubbling_dispatch_drains_its_target_then_its_ancestor() {
        fn app() -> Element {
            let mut outer = use_signal(|| 0u32);
            let mut inner = use_signal(|| 0u32);
            rsx! {
                div { id: "outer", onclick: move |_| outer += 1,
                    button { id: "inner", onclick: move |_| inner += 1, "{inner} {outer}" }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        click(&mut doc, target);
        assert_eq!(
            doc.take_step_reached(),
            [reached("inner", "click"), reached("outer", "click")]
        );
    }

    #[test]
    fn a_dispatch_its_target_stops_records_the_target_alone() {
        fn app() -> Element {
            let mut outer = use_signal(|| 0u32);
            let mut inner = use_signal(|| 0u32);
            rsx! {
                div { id: "outer", onclick: move |_| outer += 1,
                    button {
                        id: "inner",
                        onclick: move |event| {
                            event.stop_propagation();
                            inner += 1;
                        },
                        "{inner} {outer}"
                    }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        click(&mut doc, target);
        let text = doc
            .inner
            .borrow()
            .get_node(target)
            .map(|node| node.text_content());
        assert_eq!(
            text.as_deref(),
            Some("1 0"),
            "the stop kept the outer handler from running"
        );
        assert_eq!(doc.reached_handlers(), [reached("inner", "click")]);
        assert_eq!(doc.take_step_reached(), [reached("inner", "click")]);
    }

    #[test]
    fn a_dispatch_an_ancestor_stops_records_the_innermost_alone() {
        fn app() -> Element {
            let mut counts = use_signal(|| [0u32; 3]);
            rsx! {
                section { id: "outer", onclick: move |_| counts.write()[0] += 1,
                    div {
                        id: "middle",
                        onclick: move |event| {
                            event.stop_propagation();
                            counts.write()[1] += 1;
                        },
                        button { id: "inner", onclick: move |_| counts.write()[2] += 1,
                            "{counts.read()[0]} {counts.read()[1]} {counts.read()[2]}"
                        }
                    }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        click(&mut doc, target);
        let text = doc
            .inner
            .borrow()
            .get_node(target)
            .map(|node| node.text_content());
        assert_eq!(
            text.as_deref(),
            Some("0 1 1"),
            "the middle handler ran and stopped the event"
        );
        // The stated limit: the middle listener ran, yet only the innermost is recorded.
        assert_eq!(doc.reached_handlers(), [reached("inner", "click")]);
    }

    #[test]
    fn a_dispatch_a_handlers_panic_unwinds_records_the_innermost_alone() {
        fn panics() {
            panic!("a planted handler panic");
        }
        fn app() -> Element {
            let mut outer = use_signal(|| 0u32);
            rsx! {
                div { id: "outer", onclick: move |_| outer += 1,
                    button { id: "inner", onclick: move |_| panics(), "{outer}" }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        let unwound =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| click(&mut doc, target)));
        assert!(
            unwound.is_err(),
            "the handler's panic unwinds out of the dispatch"
        );
        assert_eq!(doc.take_step_reached(), [reached("inner", "click")]);
    }

    #[test]
    fn a_non_bubbling_event_records_its_target_alone() {
        fn app() -> Element {
            let mut entered = use_signal(|| 0u32);
            rsx! {
                div { id: "outer", onmouseenter: move |_| entered += 10,
                    button { id: "inner", onmouseenter: move |_| entered += 1, "{entered}" }
                }
            }
        }
        let mut doc = build(app);
        doc.record_reached();
        let target = node(&doc, "inner");
        let (data, chain) = {
            let inner = doc.inner.borrow();
            let node = inner.get_node(target).expect("the target resolves");
            (
                node.synthetic_click_event_data(Default::default()),
                inner.node_chain(target),
            )
        };
        let mut event = blitz_traits::events::DomEvent::new(
            target,
            blitz_traits::events::DomEventData::MouseEnter(data),
            blitz_traits::events::EventDispatchSource::Input,
        );
        assert!(!event.bubbles, "a mouseenter does not bubble");
        doc.dispatch_to_vdom(&chain, &mut event);
        blitz_dom::Document::poll(&mut doc, None);
        let text = doc
            .inner
            .borrow()
            .get_node(target)
            .map(|node| node.text_content());
        assert_eq!(text.as_deref(), Some("1"), "the target's handler alone ran");
        assert_eq!(doc.reached_handlers(), [reached("inner", "mouseenter")]);
    }

    #[test]
    fn with_the_record_off_the_drain_is_empty() {
        let mut doc = build(bump);
        let target = node(&doc, "bump");
        click(&mut doc, target);
        assert!(doc.take_step_reached().is_empty());
    }

    #[test]
    fn the_records_debug_prints_its_count_alone() {
        let mut doc = build(bump);
        doc.record_reached();
        let target = node(&doc, "bump");
        click(&mut doc, target);
        let record = doc
            .vdom
            .runtime()
            .has_context::<ReachedRecord>(ScopeId::ROOT)
            .expect("the record is switched on");
        assert_eq!(format!("{record:?}"), "ReachedRecord { reached: 1 }");
    }
}
