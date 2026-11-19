//! An SVG element's `mask` presentation attribute reaches the engine's mask-image fetch only
//! through the stamping request constructor.
//!
//! Since upstream's SVG presentation attributes (`ea7cfe0e3`) an SVG `mask="url(…)"` is read as
//! the CSS `mask` shorthand, so a URL it names is a `mask-image` layer and is fetched as a
//! style image — a fragment reference `url(#m)` too, resolved to a `data:` URL. Every such
//! request carries the loading document as its `initiator`, so blitz-net's `file:` refusal
//! holds for it as for every other sub-resource.

use blitz_test_harness::{Harness, HarnessOptions};
use blitz_traits::net::{NetHandler, NetProvider, Request, Url};
use std::sync::{Arc, Mutex};

const REMOTE_PAGE: &str = "http://remote.test/page.html";

/// Records each request's URL and initiator, and answers none.
#[derive(Default)]
struct RecordingNet {
    requests: Mutex<Vec<(Url, Option<Url>)>>,
}

impl NetProvider for RecordingNet {
    fn fetch(&self, _doc_id: usize, request: Request, _handler: Box<dyn NetHandler>) {
        self.requests
            .lock()
            .unwrap()
            .push((request.url, request.initiator));
    }
}

fn requests_for(body: &str, incremental: bool) -> Vec<(Url, Option<Url>)> {
    let html = format!("<!doctype html><html><body>{body}</body></html>");
    let net = Arc::new(RecordingNet::default());
    let mut harness = Harness::from_html_with(
        &html,
        HarnessOptions {
            base_url: Some(REMOTE_PAGE.to_string()),
            net_provider: Some(net.clone()),
            incremental: Some(incremental),
            ..Default::default()
        },
    );
    harness.pump();
    assert!(
        harness.query("rect").is_some(),
        "the page holds the svg rect (incremental: {incremental})"
    );
    net.requests.lock().unwrap().clone()
}

#[test]
fn a_mask_attribute_url_is_fetched_stamped_with_its_document() {
    let document = Url::parse(REMOTE_PAGE).unwrap();
    let body = r#"<svg width="20" height="20"><rect width="20" height="20" fill="red" mask="url(file:///stamp/mask.png)"/></svg>"#;
    for incremental in [false, true] {
        let requests = requests_for(body, incremental);
        let mask: Vec<_> = requests
            .iter()
            .filter(|(url, _)| url.as_str() == "file:///stamp/mask.png")
            .collect();
        assert_eq!(
            mask.len(),
            1,
            "the mask URL is fetched once (incremental: {incremental}): {requests:?}"
        );
        for (url, initiator) in &requests {
            assert_eq!(
                initiator.as_ref(),
                Some(&document),
                "{url} carries its document (incremental: {incremental})"
            );
        }
    }
}

/// A fragment reference resolves against the base the SVG attribute parse is given — the
/// default document URL, an empty `data:text/css` URL — not against the page, so `url(#m)` is
/// fetched as that `data:` URL with the fragment, stamped with its document like every other
/// sub-resource. Measured at the 2026-11-19 upstream merge; the stamp is what this check holds.
#[test]
fn a_mask_attribute_naming_a_fragment_is_fetched_as_a_stamped_data_url() {
    let document = Url::parse(REMOTE_PAGE).unwrap();
    let body = r##"<svg width="20" height="20"><defs><mask id="m"><rect width="10" height="10" fill="white"/></mask></defs><rect width="20" height="20" fill="red" mask="url(#m)"/></svg>"##;
    for incremental in [false, true] {
        let requests = requests_for(body, incremental);
        let urls: Vec<&str> = requests.iter().map(|(url, _)| url.as_str()).collect();
        assert_eq!(
            urls,
            ["data:text/css;charset=utf-8;base64,#m"],
            "a fragment mask is fetched once, as a data: URL (incremental: {incremental})"
        );
        assert_eq!(
            requests[0].1.as_ref(),
            Some(&document),
            "it carries its document (incremental: {incremental})"
        );
    }
}
