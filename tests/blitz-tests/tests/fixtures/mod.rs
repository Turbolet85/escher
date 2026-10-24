//! The one statement of what the checks beside the stand share: a fixture page booted with the
//! stand's own options, such a page held in a driver session, the fixture pages more than one
//! check reads, and a network stand-in that holds every request until the check answers it. A
//! check declares `mod fixtures;` and reads what it needs. This module holds no test.

#![allow(dead_code)]

use std::sync::Mutex;

use blitz_test_harness::Harness;
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;
use escher_driver::Session;
use seven_guis::stand;

/// A fixture booted as the stand boots a task: pinned viewport, bundled font, offline.
pub fn boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    Harness::from_vdom(VirtualDom::new(app), stand::options(incremental))
}

/// A session started under `label` on a fixture booted as the stand boots a task.
pub fn hold(label: &str, app: fn() -> Element, incremental: bool) -> Session {
    Session::start(label, || boot(app, incremental)).expect("the label is one of the closed set")
}

/// A password input labelled `Passphrase` by a `<label>`, the control the lean tasks lack.
pub fn password_labelled() -> Element {
    rsx! {
        div { id: "fx-root",
            label { r#for: "fx-secret", "Passphrase" }
            input { id: "fx-secret", r#type: "password" }
        }
    }
}

/// A password input named `Secret` by `aria-label`, empty.
pub fn password_named() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-secret", r#type: "password", aria_label: "Secret" }
        }
    }
}

/// One button under one positioned overlay with a `z-index` above it: the `covered` case the
/// stand lacks.
pub fn covered() -> Element {
    rsx! {
        div { id: "fx-root", style: "position: relative; width: 600px; height: 400px;",
            button {
                id: "fx-under",
                style: "position: absolute; left: 20px; top: 20px; width: 120px; height: 40px;",
                "Under"
            }
            div {
                id: "fx-cover",
                style: "position: absolute; left: 0; top: 0; width: 200px; height: 140px; z-index: 5; background: #cccccc;",
            }
        }
    }
}

/// The buttons on the fixture page.
pub const ITEMS: usize = 400;

/// 400 keyed buttons, each counting its clicks into one output, then a password input.
pub fn many_items() -> Element {
    let mut clicks = use_signal(|| 0u32);
    rsx! {
        div { id: "items",
            output { id: "clicks", "{clicks}" }
            for i in 1..=ITEMS {
                button {
                    key: "{i}",
                    id: format!("item-{i:03}"),
                    style: "display: block;",
                    onclick: move |_| clicks += 1,
                    "Item {i:03}"
                }
            }
            input { id: "secret", r#type: "password", aria_label: "Secret" }
        }
    }
}

/// A `NetProvider` that keeps every request with its handler for the check to answer.
#[derive(Default)]
pub struct HeldNet {
    requests: Mutex<Vec<(String, Box<dyn NetHandler>)>>,
}

impl HeldNet {
    /// How many requests are held.
    pub fn held(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    /// The urls of the held requests, in request order.
    pub fn urls(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|(url, _)| url.clone())
            .collect()
    }

    /// Answers the last held request with `body` and returns its url.
    #[track_caller]
    pub fn answer(&self, body: &'static [u8]) -> String {
        let (url, handler) = self
            .requests
            .lock()
            .unwrap()
            .pop()
            .expect("a request is held");
        handler.bytes(url.clone(), Bytes::from_static(body));
        url
    }
}

impl NetProvider for HeldNet {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        self.requests
            .lock()
            .unwrap()
            .push((request.url.to_string(), handler));
    }
}
