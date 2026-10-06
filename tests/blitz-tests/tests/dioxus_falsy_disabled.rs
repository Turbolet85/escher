//! A Dioxus `disabled: false` leaves the element without a `disabled` attribute,
//! so an enabled control does not match `:disabled`.
//!
//! dioxus-native-dom wrote a falsy `disabled` as the literal `disabled="false"`;
//! blitz-dom keys the DISABLED element state and `:disabled` on the attribute's
//! presence, so every enabled Dioxus control was styled and stated disabled.

use blitz_test_harness::Harness;
use dioxus::prelude::*;

fn toggle_app() -> Element {
    let mut disabled = use_signal(|| false);
    rsx! {
        button {
            id: "toggle",
            style: "width: 100px; height: 30px; display: block;",
            onclick: move |_| disabled.toggle(),
            "Toggle"
        }
        button { id: "target", disabled: disabled(), "Target" }
    }
}

#[test]
fn falsy_disabled_is_absent_and_truthy_disabled_is_present() {
    let mut harness = Harness::from_component(toggle_app);
    assert!(harness.query("#target").is_some());

    assert_eq!(harness.attr("#target", "disabled"), None);
    assert!(harness.query("#target:disabled").is_none());

    harness.click("#toggle");
    assert!(harness.attr("#target", "disabled").is_some());
    assert!(harness.query("#target:disabled").is_some());

    harness.click("#toggle");
    assert_eq!(harness.attr("#target", "disabled"), None);
    assert!(harness.query("#target:disabled").is_none());
}
