//! The reached-handler record: every handler a dispatch passed through, by the stable id of the
//! element carrying it and its event, in the order first reached.
//!
//! A root context the document provides only when asked ([`DioxusDocument::record_reached`]),
//! so a document that never asks carries none. The bridge writes it as it hands an event to the
//! vdom, before any handler runs, so the id is resolved while the element still stands: a handler
//! whose own run removes its element is still recorded. A dispatch reaches every element of its
//! chain whose current listeners hold its event, the bubbling ancestors included; a
//! `stopPropagation` is not read. No `NodeId` is held.

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

    /// Records each element of `chain` whose current listeners hold `event`, once each.
    pub(crate) fn record(
        &self,
        dom: &VirtualDom,
        state: &DioxusState,
        doc: &BaseDocument,
        chain: &[NodeId],
        event: &'static str,
    ) {
        let elements = element_listener_ids(dom, state, doc);
        let mut lists = self.lock();
        for node in chain {
            let Some((_, id, listeners)) = elements.iter().find(|(element, ..)| element == node)
            else {
                continue;
            };
            if !listeners.contains(&event) {
                continue;
            }
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
