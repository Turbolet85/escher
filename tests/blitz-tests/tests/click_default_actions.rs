//! A click's default actions that leave the walk from its target, in every document: a link whose
//! href resolves outside the document hands the navigation provider one navigation to that URL,
//! and a fragment-only link of the same page hands it none; an `<a>` with no href walks on, so a
//! label holding it checks its bound checkbox; a click no element takes clears focus, so the
//! document reads its root element as focused; a disabled submit button stops the walk, so focus
//! stays and its form submits nothing; a label bound to a `multiple` select focuses the select and
//! leaves its chosen set as it was; and two clicks on one point with no time between them are one
//! double click an app hears, where 500 ms of the clock between them make none. In both layout
//! modes; a failure message carries the mode and the case.
//!
//! <https://html.spec.whatwg.org/multipage/webappapis.html#fire-a-click-event>

use std::sync::{Arc, Mutex};
use std::time::Duration;

use blitz_dom::{DocumentConfig, FontContext, VirtualClock};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_test_harness::Harness;
use blitz_traits::navigation::{NavigationOptions, NavigationProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use dioxus::prelude::*;

mod fixtures;

/// The page every HTML case loads at.
const BASE: &str = "https://example.org/page.html";

/// The URLs the document handed its navigation provider, in order.
#[derive(Default)]
struct Navigations(Mutex<Vec<String>>);

impl NavigationProvider for Navigations {
    fn navigate_to(&self, options: NavigationOptions) {
        if let Ok(mut urls) = self.0.lock() {
            urls.push(options.url.to_string());
        }
    }
}

impl Navigations {
    fn urls(&self) -> Vec<String> {
        self.0.lock().map(|urls| urls.clone()).unwrap_or_default()
    }
}

const PAGE: &str = r##"<html><body style="margin:0">
    <a id="away" href="https://example.com/next" style="display:block;width:120px;height:40px"></a>
    <a id="fragment" href="#here" style="display:block;width:120px;height:40px"></a>
    <label id="holder" for="cb" style="display:block;width:200px;height:60px">
        <a id="bare" style="display:block;width:120px;height:40px"></a>
    </label>
    <input id="cb" type="checkbox" style="display:block;width:30px;height:30px">
    <div id="plain" style="width:120px;height:40px"></div>
    <form action="https://example.com/submit">
        <button id="off" type="submit" disabled style="display:block;width:120px;height:40px"></button>
    </form>
    <label id="rows-label" for="rows" style="display:block;width:120px;height:40px"></label>
    <select id="rows" multiple style="display:block;width:120px;height:60px">
        <option value="a">A</option>
        <option value="b" selected>B</option>
    </select>
    <div id="here" style="width:120px;height:40px"></div>
</body></html>"##;

/// `PAGE` laid out at `BASE` on a clock no case moves, with a recording navigation provider.
fn page(incremental: bool) -> (Harness<HtmlDocument>, Arc<Navigations>) {
    let navigations = Arc::new(Navigations::default());
    let doc = HtmlDocument::from_html(
        PAGE,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            base_url: Some(BASE.to_string()),
            navigation_provider: Some(Arc::clone(&navigations) as _),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            font_ctx: Some(FontContext::new()),
            incremental: Some(incremental),
            clock: Some(Arc::new(VirtualClock::new()) as _),
            ..Default::default()
        },
    );
    let mut harness = Harness::wrap(doc);
    harness.pump();
    (harness, navigations)
}

fn checked(harness: &Harness<HtmlDocument>, selector: &str) -> Option<bool> {
    let node = harness.node(selector);
    harness
        .base()
        .get_node(node)
        .and_then(|node| node.element_data())
        .and_then(|el| el.checkbox_input_checked())
}

/// A page with `#cb` focused by a click, which checks it.
fn focused_checkbox(incremental: bool, mode: &str) -> (Harness<HtmlDocument>, Arc<Navigations>) {
    let (mut harness, navigations) = page(incremental);
    harness.click("#cb");
    let cb = harness.node("#cb");
    assert!(
        harness.focused() == Some(cb),
        "{mode}: the click on the checkbox focuses it"
    );
    (harness, navigations)
}

#[test]
fn a_link_leaving_the_document_navigates_and_a_fragment_link_does_not() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: link");
        let (mut harness, navigations) = page(incremental);
        assert!(
            navigations.urls().is_empty(),
            "{mode}: loading the page navigates nowhere"
        );
        harness.click("#fragment");
        assert!(
            navigations.urls().is_empty(),
            "{mode}: a fragment-only link hands the provider nothing"
        );
        harness.click("#away");
        assert!(
            navigations.urls() == vec!["https://example.com/next".to_string()],
            "{mode}: a link leaving the document hands the provider its URL once"
        );
    }
}

#[test]
fn a_link_with_no_href_walks_on_to_its_label() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: bare link");
        let (mut harness, _navigations) = page(incremental);
        assert!(
            checked(&harness, "#cb") == Some(false),
            "{mode}: the checkbox starts unchecked"
        );
        harness.click("#bare");
        assert!(
            checked(&harness, "#cb") == Some(true),
            "{mode}: the click walks on to the label, which checks its checkbox"
        );
    }
}

#[test]
fn a_click_no_element_takes_clears_focus() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: unmatched");
        let (mut harness, _navigations) = focused_checkbox(incremental, &mode);
        harness.click("#plain");
        // With no node focused, the document reads its root element as focused.
        let root = harness.node("html");
        assert!(
            harness.focused() == Some(root),
            "{mode}: a click no arm takes clears focus"
        );
    }
}

#[test]
fn a_disabled_submit_button_stops_the_walk() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: disabled");
        let (mut harness, navigations) = focused_checkbox(incremental, &mode);
        let cb = harness.node("#cb");
        harness.click("#off");
        assert!(
            harness.focused() == Some(cb),
            "{mode}: focus stays where it was"
        );
        assert!(
            navigations.urls().is_empty(),
            "{mode}: the disabled submit button submits nothing"
        );
    }
}

#[test]
fn a_label_of_a_multiple_select_focuses_it_and_chooses_nothing() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: multiple label");
        let (mut harness, _navigations) = page(incremental);
        let rows = harness.node("#rows");
        assert!(
            harness.base().select_values(rows) == vec!["b".to_string()],
            "{mode}: the select starts holding its `selected` option"
        );
        assert!(
            harness.focused() != Some(rows),
            "{mode}: the select starts unfocused"
        );
        harness.click("#rows-label");
        assert!(
            harness.focused() == Some(rows),
            "{mode}: the label's click focuses its select"
        );
        assert!(
            harness.base().select_values(rows) == vec!["b".to_string()],
            "{mode}: the label's click leaves the chosen set as it was"
        );
    }
}

/// A Dioxus page counting the double clicks its target hears.
fn double_page() -> Element {
    let mut heard = use_signal(|| 0u32);
    rsx! {
        div { id: "fx-root",
            div {
                id: "fx-target",
                style: "width: 120px; height: 40px;",
                ondoubleclick: move |_| *heard.write() += 1,
            }
            p { id: "fx-heard", "{heard}" }
        }
    }
}

fn heard(harness: &Harness<dioxus_native_dom::DioxusDocument>) -> String {
    harness.text_content("#fx-heard").trim().to_string()
}

#[test]
fn two_clicks_with_no_time_between_them_are_one_double_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: double click");

        let mut harness = fixtures::boot(double_page, incremental);
        assert!(
            heard(&harness) == "0",
            "{mode}: the app starts hearing none"
        );
        let (x, y) = harness.center_of("#fx-target");
        harness.click_at(x, y);
        harness.click_at(x, y);
        assert!(
            heard(&harness) == "1",
            "{mode}: two clicks with no time between them are one double click"
        );

        let mut harness = fixtures::boot(double_page, incremental);
        let (x, y) = harness.center_of("#fx-target");
        harness.click_at(x, y);
        harness.advance_time(Duration::from_millis(500));
        harness.click_at(x, y);
        assert!(
            heard(&harness) == "0",
            "{mode}: two clicks 500 ms apart are no double click"
        );
    }
}
