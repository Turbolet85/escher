//! An `@font-face` source with no format hint whose URL has no extension — a `data:` URL — is
//! fetched, and its format is sniffed from the bytes the fetch answers: a body that opens with
//! `wOF2` is decompressed and registered under the rule's family, so text set in that family
//! lays out with a width, while a body no sniff matches registers nothing. Upstream's behaviour
//! since <https://github.com/DioxusLabs/blitz/pull/1109>; escher routes no agent- or
//! user-supplied URL through it. Each document is built with a font context that holds no
//! family and with the shared held stand-in, which holds every request until the check answers
//! it: offline, in both layout modes.

use std::sync::Arc;

use blitz_test_harness::{Harness, HarnessOptions};

mod fixtures;
use fixtures::HeldNet;

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

/// The page, under a font context that holds no family, its one font request held.
fn page(incremental: bool) -> (Harness, Arc<HeldNet>) {
    let net = Arc::new(HeldNet::default());
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
