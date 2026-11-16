//! The root error boundary's error, read as a mark: whether a step put a new error there.
//!
//! dioxus-core wraps every app in a default error boundary at
//! [`ScopeId::ROOT_ERROR_BOUNDARY`], so a handler's `Err` that no boundary of the app's own
//! catches lands there. The mark holds that error by identity and is compared by it; nothing of
//! the error is read, printed or carried.

use std::sync::Arc;

use dioxus_core::{CapturedError, ErrorContext, ScopeId};

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

impl DioxusDocument {
    /// The error the app's root error boundary holds now, as a mark; `None` while it holds
    /// none. An error a boundary of the app's own catches never reaches it.
    pub fn root_error_mark(&self) -> Option<RootErrorMark> {
        self.vdom
            .runtime()
            .has_context::<ErrorContext>(ScopeId::ROOT_ERROR_BOUNDARY)
            .and_then(|context| context.error())
            .map(RootErrorMark)
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

    #[test]
    fn a_handlers_err_puts_a_new_mark_and_none_stands_before() {
        let mut doc = build(failing);
        assert!(doc.root_error_mark().is_none(), "none before an error");
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
        let errors = doc
            .vdom
            .runtime()
            .has_context::<ErrorContext>(ScopeId::ROOT_ERROR_BOUNDARY)
            .expect("the root boundary");
        errors.insert_error(CapturedError::from_display("second"));
        let second = doc.root_error_mark().expect("a mark after the second Err");
        assert_ne!(first, second);
    }

    #[test]
    fn an_err_the_apps_own_boundary_catches_leaves_no_mark() {
        fn app() -> Element {
            rsx! {
                ErrorBoundary { handle_error: |_| rsx! { p { id: "caught" } },
                    Inner {}
                }
            }
        }
        #[component]
        fn Inner() -> Element {
            rsx! {
                button { id: "fail", onclick: move |_| -> Result<(), CapturedError> { Err(CapturedError::from_display("own")) }, "Fail" }
            }
        }
        let mut doc = build(app);
        let target = node(&doc, "fail");
        click(&mut doc, target);
        node(&doc, "caught");
        assert!(doc.root_error_mark().is_none());
    }
}
