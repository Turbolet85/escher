//! The errors the app's error boundaries hold, read as marks: whether a step put a new error in
//! one, and what that error says.
//!
//! dioxus-core wraps every app in a default error boundary at
//! [`ScopeId::ROOT_ERROR_BOUNDARY`], so a handler's `Err` that no boundary of the app's own
//! catches lands there. [`DioxusDocument::new`] provides dioxus-core's boundary factory at the
//! root scope before the first build, so every boundary built under it — the root one and each
//! of the app's own — registers its [`ErrorContext`] in the document's [`BoundaryRegistry`] with
//! the id of the scope that holds it. Before each render the document drops the entries whose
//! scope is gone, so a remounting app does not grow the registry, while a boundary dropped in a
//! step still reads in that step's marks. An app that provides a factory of its own shadows the
//! document's for the scopes under it, and those boundaries stay unread.
//!
//! A mark holds an error by identity and is compared by it. [`BoundaryMark::read`] reads its
//! top line and its cause chain; nothing here prints or logs them.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use dioxus_core::{
    CapturedError, ErrorContext, Runtime, ScopeId, VirtualDom, provide_context,
    provide_create_error_boundary, try_consume_context,
};

use crate::DioxusDocument;

/// The error the root error boundary holds, compared by identity alone.
#[derive(Clone)]
pub struct RootErrorMark(CapturedError);

impl PartialEq for RootErrorMark {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0.0, &other.0.0)
    }
}

impl Eq for RootErrorMark {}

impl std::fmt::Debug for RootErrorMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RootErrorMark")
    }
}

/// Every error boundary built under the document's boundary factory whose scope still stands as
/// of the last render, in the order built, each beside the id of the scope that holds it.
#[derive(Clone, Default)]
pub struct BoundaryRegistry(Rc<RefCell<Vec<RegisteredBoundary>>>);

/// A registered boundary: the scope that holds it, where the factory ran inside one, and its
/// context.
type RegisteredBoundary = (Option<ScopeId>, ErrorContext);

/// The document's boundary factory: a boundary like dioxus-core's default one, registered in
/// the [`BoundaryRegistry`] its scope reads, with the scope it is built in.
fn registered_boundary() -> ErrorContext {
    let context = ErrorContext::new(None);
    if let Some(registry) = try_consume_context::<BoundaryRegistry>() {
        let scope = Runtime::try_current().and_then(|runtime| runtime.try_current_scope_id());
        registry.0.borrow_mut().push((scope, context.clone()));
    }
    context
}

/// Drops each registered boundary whose scope `vdom` no longer holds, or holds with another
/// boundary in a reused slot. An entry with no scope is kept.
pub(crate) fn prune_dropped_boundaries(vdom: &VirtualDom) {
    let Some(registry) = vdom
        .runtime()
        .has_context::<BoundaryRegistry>(ScopeId::ROOT)
    else {
        return;
    };
    // `has_context` panics on a dropped scope, so the scope is looked up first.
    registry.0.borrow_mut().retain(|(scope, context)| {
        scope.is_none_or(|id| {
            vdom.get_scope(id).is_some()
                && vdom.runtime().has_context::<ErrorContext>(id).as_ref() == Some(context)
        })
    });
}

/// Provides the registry and the factory at `vdom`'s root scope.
pub(crate) fn provide_boundary_registry(vdom: &VirtualDom) {
    vdom.in_scope(ScopeId::ROOT, || {
        provide_context(BoundaryRegistry::default());
        provide_create_error_boundary(registered_boundary);
    });
}

/// Which boundary holds an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    /// The app's root error boundary.
    Root,
    /// A boundary of the app's own.
    Own,
}

/// An error a boundary holds, compared by identity alone.
#[derive(Clone)]
pub struct BoundaryMark {
    boundary: Boundary,
    error: CapturedError,
}

impl PartialEq for BoundaryMark {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.error.0, &other.error.0)
    }
}

impl Eq for BoundaryMark {}

impl std::fmt::Debug for BoundaryMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BoundaryMark").field(&self.boundary).finish()
    }
}

/// What a boundary's error says.
#[derive(Clone, PartialEq, Eq)]
pub struct BoundaryError {
    /// The boundary that holds it.
    pub boundary: Boundary,
    /// Its top line.
    pub message: String,
    /// Each cause after the top line, in order.
    pub chain: Vec<String>,
}

impl std::fmt::Debug for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoundaryError")
            .field("boundary", &self.boundary)
            .field("chain", &self.chain.len())
            .finish()
    }
}

impl BoundaryMark {
    /// The boundary that holds the error.
    pub fn boundary(&self) -> Boundary {
        self.boundary
    }

    /// The error's top line and its cause chain.
    pub fn read(&self) -> BoundaryError {
        BoundaryError {
            boundary: self.boundary,
            message: self.error.to_string(),
            chain: self
                .error
                .chain()
                .skip(1)
                .map(ToString::to_string)
                .collect(),
        }
    }
}

impl DioxusDocument {
    fn root_boundary(&self) -> Option<ErrorContext> {
        self.vdom
            .runtime()
            .has_context::<ErrorContext>(ScopeId::ROOT_ERROR_BOUNDARY)
    }

    /// The error the app's root error boundary holds now, as a mark; `None` while it holds
    /// none. An error a boundary of the app's own catches never reaches it.
    pub fn root_error_mark(&self) -> Option<RootErrorMark> {
        self.root_boundary()
            .and_then(|context| context.error())
            .map(RootErrorMark)
    }

    /// One mark per boundary holding an error now, each error once: the boundaries the
    /// document's factory registered, in the order built, and the root one where it is not
    /// among them (a document built after its virtual DOM's first build).
    pub fn boundary_marks(&self) -> Vec<BoundaryMark> {
        let root = self.root_boundary();
        let mut contexts: Vec<ErrorContext> = self
            .vdom
            .runtime()
            .has_context::<BoundaryRegistry>(ScopeId::ROOT)
            .map(|registry| {
                registry
                    .0
                    .borrow()
                    .iter()
                    .map(|(_, context)| context.clone())
                    .collect()
            })
            .unwrap_or_default();
        if let Some(root) = &root
            && !contexts.contains(root)
        {
            contexts.insert(0, root.clone());
        }
        let mut marks: Vec<BoundaryMark> = Vec::new();
        for (index, context) in contexts.iter().enumerate() {
            if contexts[..index].contains(context) {
                continue;
            }
            let Some(error) = context.error() else {
                continue;
            };
            let boundary = if root.as_ref() == Some(context) {
                Boundary::Root
            } else {
                Boundary::Own
            };
            let mark = BoundaryMark { boundary, error };
            if !marks.contains(&mark) {
                marks.push(mark);
            }
        }
        marks
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

    fn failing() -> Element {
        rsx! {
            button { id: "fail", onclick: move |_| -> Result<(), CapturedError> { Err(CapturedError::from_display("failed")) }, "Fail" }
        }
    }

    #[derive(Debug)]
    struct Link(&'static str, Option<Box<Link>>);

    impl std::fmt::Display for Link {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }

    impl std::error::Error for Link {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.1.as_deref().map(|link| link as _)
        }
    }

    fn three_links() -> CapturedError {
        CapturedError::new(Link(
            "could not load",
            Some(Box::new(Link(
                "bad setting",
                Some(Box::new(Link("invalid digit", None))),
            ))),
        ))
    }

    #[test]
    fn a_handlers_err_puts_a_new_mark_and_none_stands_before() {
        let mut doc = build(failing);
        assert!(doc.root_error_mark().is_none(), "none before an error");
        assert!(doc.boundary_marks().is_empty());
        let target = node(&doc, "fail");
        click(&mut doc, target);
        let first = doc.root_error_mark().expect("a mark after the Err");
        let again = doc.root_error_mark().expect("the mark stands");
        assert_eq!(first, again, "two reads with no new error compare equal");
        assert_eq!(format!("{first:?}"), "RootErrorMark");
    }

    #[test]
    fn a_second_err_compares_unequal() {
        fn app() -> Element {
            let mut shown = use_signal(|| 0u32);
            rsx! {
                button {
                    id: "fail",
                    onclick: move |_| -> Result<(), CapturedError> {
                        shown += 1;
                        Err(CapturedError::from_display("again"))
                    },
                    "{shown}"
                }
            }
        }
        let mut doc = build(app);
        let target = node(&doc, "fail");
        click(&mut doc, target);
        let first = doc.root_error_mark().expect("a mark after the first Err");
        let marks = doc.boundary_marks();
        let errors = doc
            .vdom
            .runtime()
            .has_context::<ErrorContext>(ScopeId::ROOT_ERROR_BOUNDARY)
            .expect("the root boundary");
        errors.insert_error(CapturedError::from_display("second"));
        let second = doc.root_error_mark().expect("a mark after the second Err");
        assert_ne!(first, second);
        let after = doc.boundary_marks();
        assert_eq!(after.len(), 1);
        assert!(!marks.contains(&after[0]), "the second error reads new");
    }

    #[test]
    fn a_root_err_reads_root_with_its_chain() {
        fn app() -> Element {
            rsx! {
                button { id: "fail", onclick: move |_| -> Result<(), CapturedError> { Err(three_links()) }, "Fail" }
            }
        }
        let mut doc = build(app);
        let target = node(&doc, "fail");
        click(&mut doc, target);
        let marks = doc.boundary_marks();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].boundary(), Boundary::Root);
        let read = marks[0].read();
        assert_eq!(read.boundary, Boundary::Root);
        assert_eq!(read.message, "could not load");
        assert_eq!(read.chain, ["bad setting", "invalid digit"]);
        assert_eq!(
            format!("{read:?}"),
            "BoundaryError { boundary: Root, chain: 2 }"
        );
    }

    #[component]
    fn Inner() -> Element {
        rsx! {
            button { id: "fail", onclick: move |_| -> Result<(), CapturedError> { Err(three_links()) }, "Fail" }
        }
    }

    fn own_boundary() -> Element {
        rsx! {
            ErrorBoundary { handle_error: |_| rsx! { p { id: "caught" } },
                Inner {}
            }
        }
    }

    #[test]
    fn an_err_the_apps_own_boundary_catches_reads_own_and_the_root_holds_none() {
        let mut doc = build(own_boundary);
        let registered = doc
            .vdom
            .runtime()
            .has_context::<BoundaryRegistry>(ScopeId::ROOT)
            .map(|registry| registry.0.borrow().len());
        assert_eq!(registered, Some(2), "the root boundary and the app's own");
        let target = node(&doc, "fail");
        click(&mut doc, target);
        node(&doc, "caught");
        assert!(doc.root_error_mark().is_none());
        let marks = doc.boundary_marks();
        assert_eq!(marks.len(), 1, "each error read once");
        let read = marks[0].read();
        assert_eq!(read.boundary, Boundary::Own);
        assert_eq!(read.message, "could not load");
        assert_eq!(read.chain, ["bad setting", "invalid digit"]);
    }

    #[test]
    fn a_second_step_leaves_the_first_errors_mark_unchanged() {
        fn app() -> Element {
            let mut shown = use_signal(|| 0u32);
            rsx! {
                ErrorBoundary { handle_error: |_| rsx! { p { id: "caught" } },
                    Inner {}
                }
                button { id: "count", onclick: move |_| shown += 1, "{shown}" }
            }
        }
        let mut doc = build(app);
        let fail = node(&doc, "fail");
        click(&mut doc, fail);
        let before = doc.boundary_marks();
        let count = node(&doc, "count");
        click(&mut doc, count);
        assert_eq!(doc.boundary_marks(), before);
        assert_eq!(format!("{:?}", before[0]), "BoundaryMark(Own)");
    }

    #[test]
    fn a_document_built_after_the_first_build_reads_the_root_boundarys_error() {
        fn app() -> Element {
            Err(CapturedError::from_display("failed").into())
        }
        let mut vdom = VirtualDom::new(app);
        vdom.rebuild_in_place();
        let doc = DioxusDocument::new(vdom, DocumentConfig::default());
        let marks = doc.boundary_marks();
        assert_eq!(marks.len(), 1, "the root boundary read once");
        assert_eq!(marks[0].read().boundary, Boundary::Root);
    }

    /// A boundary of the app's own, keyed by a count a button moves, so each click remounts it.
    fn remounting() -> Element {
        let mut generation = use_signal(|| 0u32);
        rsx! {
            button { id: "remount", onclick: move |_| generation += 1, "{generation}" }
            for current in [generation()] {
                ErrorBoundary { key: "{current}", handle_error: |_| rsx! { p { id: "caught" } },
                    Inner {}
                }
            }
        }
    }

    /// The registry's length after twenty remounts, as measured: the root boundary, the one the
    /// last render dropped (pruned at the next) and the live one.
    const REMOUNTED_LEN: usize = 3;

    fn registry_len(doc: &DioxusDocument) -> Option<usize> {
        doc.vdom
            .runtime()
            .has_context::<BoundaryRegistry>(ScopeId::ROOT)
            .map(|registry| registry.0.borrow().len())
    }

    fn remounted_twenty_times() -> DioxusDocument {
        let mut doc = build(remounting);
        assert_eq!(
            registry_len(&doc),
            Some(2),
            "the root boundary and the app's own"
        );
        for _ in 0..20 {
            let remount = node(&doc, "remount");
            click(&mut doc, remount);
        }
        doc
    }

    #[test]
    fn twenty_remounts_keep_the_registry_bounded() {
        let doc = remounted_twenty_times();
        assert_eq!(registry_len(&doc), Some(REMOUNTED_LEN));
    }

    #[test]
    fn an_err_after_the_remounts_still_reads_a_new_mark() {
        let mut doc = remounted_twenty_times();
        assert!(doc.boundary_marks().is_empty(), "no mark before the Err");
        let fail = node(&doc, "fail");
        click(&mut doc, fail);
        let marks = doc.boundary_marks();
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].boundary(), Boundary::Own);
    }

    #[component]
    fn Leaving() -> Element {
        let mut shown = use_context::<Signal<bool>>();
        rsx! {
            button {
                id: "fail",
                onclick: move |_| -> Result<(), CapturedError> {
                    shown.set(false);
                    Err(CapturedError::from_display("left"))
                },
                "Fail"
            }
        }
    }

    #[test]
    fn a_boundary_dropped_in_a_step_reads_until_the_next_render() {
        fn app() -> Element {
            let shown = use_context_provider(|| Signal::new(true));
            let mut count = use_signal(|| 0u32);
            rsx! {
                if shown() {
                    ErrorBoundary { handle_error: |_| rsx! { p { id: "caught" } },
                        Leaving {}
                    }
                }
                button { id: "count", onclick: move |_| count += 1, "{count}" }
            }
        }
        let mut doc = build(app);
        let fail = node(&doc, "fail");
        click(&mut doc, fail);
        assert!(
            doc.element_ids().iter().all(|(_, id)| id != "fail"),
            "the step removes the boundary"
        );
        let marks = doc.boundary_marks();
        assert_eq!(marks.len(), 1, "the dropped boundary reads in its step");
        assert_eq!(marks[0].boundary(), Boundary::Own);
        let count = node(&doc, "count");
        click(&mut doc, count);
        assert!(doc.boundary_marks().is_empty(), "the next render drops it");
    }
}
