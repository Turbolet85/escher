//! Guards the bridge behaviour no other test reads: a Dioxus key the id grammar refuses — empty,
//! or holding `/` — reads a positional segment; a head element is appended under `<head>`;
//! `mounted` listeners fire after the build and after a poll; the document's id is the inner
//! document's; and a handled event reports cancel and stop as its listener left them. A child
//! module of `dioxus_document`, because the event handler's fields are private to it.

use super::*;
use blitz_dom::{EventHandler, local_name};
use blitz_traits::events::BlitzInputEvent;
use dioxus::prelude::*;
use dioxus_core::ScopeId;
use std::cell::Cell;

fn build(app: fn() -> Element) -> DioxusDocument {
    let mut doc = DioxusDocument::new(VirtualDom::new(app), DocumentConfig::default());
    doc.initial_build();
    doc
}

fn empty_app() -> Element {
    rsx! {
        div {}
    }
}

/// The ids of the elements the app renders, the document skeleton left out.
fn app_ids(doc: &DioxusDocument) -> Vec<String> {
    doc.element_ids()
        .into_iter()
        .skip(4)
        .map(|(_, id)| id)
        .collect()
}

/// The element children of `parent`.
#[track_caller]
fn element_children(doc: &BaseDocument, parent: NodeId) -> Vec<NodeId> {
    let parent = doc.get_node(parent).expect("the parent resolves");
    parent
        .children
        .iter()
        .copied()
        .filter(|child| doc.get_node(*child).is_some_and(|node| node.is_element()))
        .collect()
}

#[test]
fn a_slashed_dioxus_key_reads_a_positional_segment() {
    fn app() -> Element {
        rsx! {
            ul {
                for key in ["ok", "a/b"] {
                    li { key: "{key}" }
                }
            }
        }
    }
    let ids = app_ids(&build(app));
    assert_eq!(ids[1], "/ul:0/li[ok]", "a Dioxus key reaches its segment");
    assert_eq!(ids, ["/ul:0", "/ul:0/li[ok]", "/ul:0/li:1"]);
}

#[test]
fn an_empty_dioxus_key_reads_a_positional_segment() {
    fn app() -> Element {
        rsx! {
            ul {
                for key in ["ok", ""] {
                    li { key: "{key}" }
                }
            }
        }
    }
    let ids = app_ids(&build(app));
    assert_eq!(ids[1], "/ul:0/li[ok]", "a Dioxus key reaches its segment");
    assert_eq!(ids, ["/ul:0", "/ul:0/li[ok]", "/ul:0/li:1"]);
}

#[test]
fn a_head_element_is_appended_under_head() {
    let mut doc = build(empty_app);
    let main_before = element_children(&doc.inner.borrow(), doc.main_element_id);
    assert!(
        element_children(&doc.inner.borrow(), doc.head_element_id).is_empty(),
        "the head holds no element after the build"
    );

    doc.create_head_element(
        "title",
        &[("lang".to_string(), "en".to_string())],
        &Some("escher".to_string()),
    );

    let inner = doc.inner.borrow();
    let added = element_children(&inner, doc.head_element_id);
    assert_eq!(added.len(), 1, "one element under the head");
    let title = inner.get_node(added[0]).expect("the new element resolves");
    let tag = title.element_data().map(|element| &*element.name.local);
    assert_eq!(tag, Some("title"));
    assert_eq!(title.attr(local_name!("lang")), Some("en"));
    assert_eq!(title.children.len(), 1, "the title holds one node");
    let text = inner
        .get_node(title.children[0])
        .expect("the title's child resolves");
    assert!(text.is_text_node(), "the title's child is text");
    assert_eq!(text.text_content(), "escher");
    assert_eq!(
        element_children(&inner, doc.main_element_id),
        main_before,
        "main holds what it held"
    );
}

/// How many `mounted` listeners fired, and whether the app renders its second element.
#[derive(Default)]
struct Mounts {
    fired: Cell<usize>,
    second: Cell<bool>,
}

#[test]
fn mounted_listeners_fire_after_the_build_and_after_a_poll() {
    fn app(mounts: Rc<Mounts>) -> Element {
        let (first, second) = (Rc::clone(&mounts), Rc::clone(&mounts));
        rsx! {
            div { onmounted: move |_| first.fired.set(first.fired.get() + 1) }
            if mounts.second.get() {
                p { onmounted: move |_| second.fired.set(second.fired.get() + 1) }
            }
        }
    }
    let mounts = Rc::new(Mounts::default());
    let vdom = VirtualDom::new_with_props(app, Rc::clone(&mounts));
    let mut doc = DioxusDocument::new(vdom, DocumentConfig::default());
    assert_eq!(mounts.fired.get(), 0, "nothing is mounted before the build");

    doc.initial_build();
    assert_eq!(mounts.fired.get(), 1, "the build mounted the first element");

    mounts.second.set(true);
    doc.vdom.mark_dirty(ScopeId::APP);
    doc.poll(None);
    assert_eq!(mounts.fired.get(), 2, "the poll mounted the second element");
}

#[test]
fn the_document_id_is_the_inner_documents_id() {
    let (first, second) = (build(empty_app), build(empty_app));
    let inner_ids = [first.inner.borrow().id(), second.inner.borrow().id()];
    assert_ne!(inner_ids[0], inner_ids[1], "two documents, two inner ids");
    assert_eq!(Document::id(&first), inner_ids[0]);
    assert_eq!(Document::id(&second), inner_ids[1]);
}

/// How often the listener of `plain`, `cancel` and `stop` ran, in that order.
type Heard = Rc<[Cell<usize>; 3]>;

fn hear(heard: &Heard, at: usize) {
    heard[at].set(heard[at].get() + 1);
}

#[test]
fn a_handled_event_reports_cancel_and_stop_as_the_listener_left_them() {
    fn app(heard: Heard) -> Element {
        let (plain, cancel, stop) = (Rc::clone(&heard), Rc::clone(&heard), heard);
        rsx! {
            input { id: "plain", oninput: move |_| hear(&plain, 0) }
            input {
                id: "cancel",
                oninput: move |event| {
                    hear(&cancel, 1);
                    event.prevent_default();
                },
            }
            input {
                id: "stop",
                oninput: move |event| {
                    hear(&stop, 2);
                    event.stop_propagation();
                },
            }
        }
    }
    let heard = Heard::default();
    let vdom = VirtualDom::new_with_props(app, Rc::clone(&heard));
    let mut doc = DioxusDocument::new(vdom, DocumentConfig::default());
    doc.initial_build();
    let ids = doc.element_ids();

    let cases = [
        ("plain", false, false),
        ("cancel", true, false),
        ("stop", false, true),
    ];
    for (at, (id, cancelled, stopped)) in cases.into_iter().enumerate() {
        let node = ids
            .iter()
            .find_map(|(node, read)| (read == id).then_some(*node))
            .unwrap_or_else(|| panic!("{id:?} is an element"));
        let data = DomEventData::Input(BlitzInputEvent {
            value: String::new(),
        });
        let mut event = DomEvent::new(node, data);
        let mut state = EventState::default();
        let mut handler = DioxusEventHandler {
            vdom: &mut doc.vdom,
            vdom_state: &mut doc.vdom_state,
        };
        handler.handle_event(&[node], &mut event, &mut doc.inner, &mut state);

        assert_eq!(heard[at].get(), 1, "{id:?}: its listener ran once");
        assert_eq!(
            (state.is_cancelled(), state.propagation_is_stopped()),
            (cancelled, stopped),
            "{id:?}: cancelled and stopped"
        );
    }
}
