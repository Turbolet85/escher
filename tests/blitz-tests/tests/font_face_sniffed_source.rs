//! An `@font-face` source with no format hint whose URL has no extension — a `data:` URL — is
//! fetched, and its format is sniffed from the bytes the fetch answers: a body that opens with
//! `wOF2` is decompressed and registered under the rule's family, so text set in that family
//! lays out with a width, while a body no sniff matches registers nothing. Upstream's behaviour
//! since <https://github.com/DioxusLabs/blitz/pull/1109>; escher routes no agent- or
//! user-supplied URL through it. Each document is built with a font context that holds no
//! family and with this file's own provider, which holds every request until the check answers
//! it: offline, in both layout modes.

use std::sync::{Arc, Mutex};

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{Bytes, NetHandler, NetProvider, Request};

/// The source the rule names: no `format()` hint, and a URL with no extension.
const SOURCE: &str = "data:application/octet-stream;base64,AAAA";

const PAGE: &str = r#"<!doctype html>
<html>
  <head><style>
    @font-face { font-family: "Sniffed"; src: url("data:application/octet-stream;base64,AAAA"); }
    #fx-text { display: inline-block; font-family: "Sniffed"; font-size: 20px; }
  </style></head>
  <body><div id="fx-text">Sniffed text</div></body>
</html>"#;

/// Four bytes that open no font format the sniff knows.
const UNSNIFFABLE: &[u8] = b"abcd";

/// A `NetProvider` that keeps every request for the check to answer.
#[derive(Default)]
struct HeldNetProvider {
    requests: Mutex<Vec<(String, Box<dyn NetHandler>)>>,
}

impl HeldNetProvider {
    fn urls(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|(url, _)| url.clone())
            .collect()
    }

    /// Answers the one held request with `body`.
    #[track_caller]
    fn answer(&self, body: &'static [u8]) {
        let (url, handler) = self
            .requests
            .lock()
            .unwrap()
            .pop()
            .expect("a request is held");
        handler.bytes(url, Bytes::from_static(body));
    }
}

impl NetProvider for HeldNetProvider {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        self.requests
            .lock()
            .unwrap()
            .push((request.url.to_string(), handler));
    }
}

/// The page, under a font context that holds no family, its one font request held.
fn page(incremental: bool) -> (Harness, Arc<HeldNetProvider>) {
    let net = Arc::new(HeldNetProvider::default());
    let page = Harness::from_html_with(
        PAGE,
        HarnessOptions {
            width: 800,
            height: 600,
            net_provider: Some(Arc::clone(&net) as _),
            // Bytes that are no font: the collection holds no family, and system fonts are off.
            font_ctx: Some(blitz_dom::build_single_font_ctx(&[0u8; 16])),
            incremental: Some(incremental),
            ..HarnessOptions::default()
        },
    );
    (page, net)
}

#[test]
fn a_hintless_source_is_requested_and_a_woff2_body_is_registered() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut page, net) = page(incremental);
        assert!(
            page.layout_rect("#fx-text").width == 0.0,
            "{mode}: with no family to set it in, the text lays out with no width"
        );
        assert!(
            net.urls() == [SOURCE],
            "{mode}: one font request is made, for the data: source"
        );

        net.answer(seven_guis::DEJAVU_SANS);
        assert!(page.settle().is_ok(), "{mode}: the answered font settles");
        assert!(
            page.layout_rect("#fx-text").width > 0.0,
            "{mode}: the woff2 body was sniffed and registered as Sniffed, and the text lays out \
             with a width"
        );
        assert!(net.urls().is_empty(), "{mode}: no further request is made");
    }
}

#[test]
fn a_body_no_sniff_matches_registers_nothing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut page, net) = page(incremental);
        assert!(
            page.layout_rect("#fx-text").width == 0.0,
            "{mode}: with no family to set it in, the text lays out with no width"
        );
        assert!(
            net.urls() == [SOURCE],
            "{mode}: one font request is made, for the data: source"
        );

        net.answer(UNSNIFFABLE);
        assert!(
            page.settle().is_ok(),
            "{mode}: the answered request settles"
        );
        assert!(
            page.layout_rect("#fx-text").width == 0.0,
            "{mode}: the body registered no font, and the text still lays out with no width"
        );
    }
}
