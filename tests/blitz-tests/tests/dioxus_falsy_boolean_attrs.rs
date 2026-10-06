//! A Dioxus boolean attribute rendered with a falsy value leaves the element without that
//! attribute, so a `hidden: false` element is displayed and is a node of the snapshot.
//!
//! dioxus-native-dom removed a falsy `checked` and `disabled` only and wrote every other
//! boolean attribute as the literal `"false"`; blitz-dom reads `hidden` by presence and gives
//! it `display: none`, so an element rendered `hidden: false` left the screen, the
//! accessibility tree and the snapshot. The names are the boolean-attribute list of Dioxus
//! 0.7.10's own web renderer, which removes them the same way.

use blitz_test_harness::{Harness, HarnessOptions};
use dioxus::prelude::*;
use dioxus_native_dom::DioxusDocument;

fn boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
    let options = HarnessOptions {
        incremental: Some(incremental),
        ..HarnessOptions::default()
    };
    Harness::from_vdom(VirtualDom::new(app), options)
}

fn toggle_app() -> Element {
    let mut hidden = use_signal(|| false);
    rsx! {
        button {
            id: "toggle",
            style: "width: 100px; height: 30px; display: block;",
            onclick: move |_| hidden.toggle(),
            "Toggle"
        }
        div {
            id: "target",
            style: "width: 100px; height: 30px;",
            hidden: hidden(),
            "Target"
        }
    }
}

/// The target carries no `hidden` attribute, has a layout box and is a snapshot node.
#[track_caller]
fn assert_displayed(harness: &Harness<DioxusDocument>, what: &str) {
    let rect = harness.layout_rect("#target");
    let read = (
        harness.attr("#target", "hidden"),
        rect.width > 0.0 && rect.height > 0.0,
        harness.doc.snapshot().get("target").is_some(),
    );
    assert_eq!(
        read,
        (None, true, true),
        "{what}: the hidden attribute, a layout box, a snapshot node"
    );
}

#[test]
fn a_falsy_hidden_element_is_displayed() {
    for incremental in [false, true] {
        let mut harness = boot(toggle_app, incremental);
        assert!(harness.query("#target").is_some(), "the fixture renders");
        assert_displayed(&harness, &format!("at boot, incremental={incremental}"));

        harness.click("#toggle");
        assert!(
            harness.attr("#target", "hidden").is_some(),
            "incremental={incremental}: a truthy hidden is written"
        );
        assert!(
            harness.doc.snapshot().get("target").is_none(),
            "incremental={incremental}: a hidden element is no snapshot node"
        );

        harness.click("#toggle");
        assert_displayed(
            &harness,
            &format!("toggled back, incremental={incremental}"),
        );
    }
}

/// Declares the boolean-attribute names once: as `BOOLEAN_ATTRS`, and as an app holding one
/// element that carries every name with a literal falsy value (a static template attribute)
/// and one that carries every name with a signal-driven falsy value (a dynamic attribute).
macro_rules! boolean_attrs {
    ($($name:tt),* $(,)?) => {
        const BOOLEAN_ATTRS: [&str; 27] = [$($name),*];

        fn falsy_app() -> Element {
            let off = use_signal(|| false);
            rsx! {
                div { id: "literal", $($name: "false",)* }
                div { id: "driven", $($name: off(),)* }
            }
        }
    };
}

boolean_attrs![
    "allowfullscreen",
    "allowpaymentrequest",
    "async",
    "autofocus",
    "autoplay",
    "checked",
    "controls",
    "default",
    "defer",
    "disabled",
    "formnovalidate",
    "hidden",
    "ismap",
    "itemscope",
    "loop",
    "multiple",
    "muted",
    "nomodule",
    "novalidate",
    "open",
    "playsinline",
    "readonly",
    "required",
    "reversed",
    "selected",
    "truespeed",
    "webkitdirectory",
];

#[test]
fn every_listed_boolean_attribute_is_removed_when_falsy() {
    for incremental in [false, true] {
        let harness = boot(falsy_app, incremental);
        for element in ["literal", "driven"] {
            let selector = format!("#{element}");
            assert_eq!(
                harness.attr(&selector, "id").as_deref(),
                Some(element),
                "the fixture renders {element:?} with readable attributes"
            );
            for name in BOOLEAN_ATTRS {
                assert_eq!(
                    harness.attr(&selector, name),
                    None,
                    "incremental={incremental}: a falsy {name:?} on {element:?}"
                );
            }
        }
    }
}

fn kept_app() -> Element {
    let on = use_signal(|| true);
    let off = use_signal(|| false);
    rsx! {
        div {
            id: "literal",
            "readonly": "true",
            "aria-hidden": "false",
            "data-flag": "false",
        }
        div {
            id: "driven",
            "readonly": on(),
            "aria-hidden": off(),
            "data-flag": off(),
        }
    }
}

#[test]
fn a_truthy_or_unlisted_attribute_is_kept() {
    assert!(BOOLEAN_ATTRS.contains(&"readonly"), "a listed name");
    for unlisted in ["aria-hidden", "data-flag"] {
        assert!(!BOOLEAN_ATTRS.contains(&unlisted), "an unlisted name");
    }
    for incremental in [false, true] {
        let harness = boot(kept_app, incremental);
        for element in ["literal", "driven"] {
            let selector = format!("#{element}");
            let what = format!("incremental={incremental}: on {element:?}");
            assert_eq!(
                harness.attr(&selector, "readonly").as_deref(),
                Some("true"),
                "{what}"
            );
            assert_eq!(
                harness.attr(&selector, "aria-hidden").as_deref(),
                Some("false"),
                "{what}"
            );
            assert_eq!(
                harness.attr(&selector, "data-flag").as_deref(),
                Some("false"),
                "{what}"
            );
        }
    }
}
