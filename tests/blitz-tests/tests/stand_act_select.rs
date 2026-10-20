//! What the driver's verbs do to a single-choice `select`, on a fixture page beside the stand,
//! whose tasks hold none. In both layout modes, through `Session::run`: the snapshot reads the
//! select labelled `Fruit` with role `ComboBox`, that name and its chosen option's value, its
//! options under it named by their labels and the chosen one selected; `type` with an option's
//! label chooses that option, its diff naming the select's value and the two options' selection;
//! a `click` then a `press` of the arrows or Home chooses by key; the Tab walk skips the disabled
//! select and reads the same before and after the choices; a `type` into the disabled select is
//! refused `disabled` and a `click` on an option that is not chosen — seen in its select's box,
//! where the chosen option's text lies — `covered`, each with the snapshot unchanged. A failure
//! message carries the layout mode and a row index, never an id or what a screen reads.

use accesskit::Role;
use dioxus::prelude::*;
use dioxus_native_dom::Reading;
use escher_driver::{Cause, Session};

mod fixtures;
mod session_common;
use session_common::{act, click, focused, press, refused_unchanged, type_into};

/// Two buttons around a labelled select, a select named by `aria-label` and a disabled one;
/// every option carries an author `id`.
fn page() -> Element {
    rsx! {
        div { id: "fx-root",
            button { id: "fx-before", "Before" }
            label { r#for: "fx-pick", "Fruit" }
            select { id: "fx-pick",
                option { id: "fx-apple", value: "a", "Apple" }
                option { id: "fx-banana", value: "b", selected: true, "Banana" }
                option { id: "fx-cherry", value: "c", "Cherry" }
            }
            select { id: "fx-named", aria_label: "Colour",
                option { id: "fx-red", value: "r", "Red" }
                option { id: "fx-green", value: "g", "Green" }
            }
            select { id: "fx-off", aria_label: "Off", disabled: true,
                option { id: "fx-x", value: "x", "X" }
            }
            button { id: "fx-after", "After" }
        }
    }
}

/// The Tab stops from boot, in order, and the stop the walk wraps to.
const TAB_WALK: [&str; 5] = ["fx-before", "fx-pick", "fx-named", "fx-after", "fx-before"];

fn hold(incremental: bool) -> Session {
    fixtures::hold("select", page, incremental)
}

/// The value the snapshot reads for `id`.
fn value(session: &Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.value.clone())
}

/// The stops a Tab walk from no focus reaches, one per entry of [`TAB_WALK`].
fn tab_walk(session: &mut Session, mode: &str) -> Vec<String> {
    session.harness_mut().base_mut().clear_focus();
    let mut stops = Vec::new();
    for row in 0..TAB_WALK.len() {
        let pressed = act(session, &press("tab", false));
        assert!(
            pressed.is_some_and(|pressed| pressed.settled),
            "{mode}: Tab {row} is accepted and settled"
        );
        let (in_snapshot, in_tree) = focused(session);
        assert_eq!(in_snapshot, in_tree, "{mode}: Tab {row}");
        stops.extend(in_snapshot);
    }
    stops
}

#[test]
fn at_boot_the_snapshot_reads_the_select_its_name_value_and_options() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let session = hold(incremental);
        let doc = &session.harness().doc;
        assert!(
            doc.unkeyed_actionable().is_empty(),
            "{mode}: every element an agent can act on reads an author key"
        );
        let snapshot = doc.snapshot();
        let pick = snapshot.get("fx-pick");
        assert!(pick.is_some(), "{mode}: the snapshot reads the select");
        let Some(pick) = pick else { continue };
        assert!(
            (pick.role, pick.name.as_str(), pick.state.value.as_deref())
                == (Role::ComboBox, "Fruit", Some("b")),
            "{mode}: the select reads its role, its label's name and its chosen value"
        );
        let options: Vec<_> = pick
            .children
            .iter()
            .map(|option| {
                (
                    option.id.as_str(),
                    option.role,
                    option.name.as_str(),
                    option.state.selected,
                )
            })
            .collect();
        assert!(
            options
                == [
                    ("fx-apple", Role::ListBoxOption, "Apple", Some(false)),
                    ("fx-banana", Role::ListBoxOption, "Banana", Some(true)),
                    ("fx-cherry", Role::ListBoxOption, "Cherry", Some(false)),
                ],
            "{mode}: its options lie under it, named by their labels, the chosen one selected"
        );
        let named = snapshot.get("fx-named");
        assert!(
            named.is_some_and(
                |named| named.name == "Colour" && named.state.value.as_deref() == Some("r")
            ),
            "{mode}: the select named by aria-label reads its first option chosen"
        );
        let off = snapshot.get("fx-off");
        assert!(
            off.is_some_and(|off| off.state.enabled == Some(false)),
            "{mode}: the disabled select reads not enabled"
        );
    }
}

#[test]
fn type_with_a_label_chooses_that_option() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold(incremental);
        let typed = act(&mut session, &type_into("fx-pick", "Cherry"));
        assert!(typed.is_some(), "{mode}: the type is accepted");
        let Some(typed) = typed else { continue };
        assert!(typed.settled, "{mode}: the type settles");
        let differs = |id: &str| {
            typed
                .diff
                .changed
                .iter()
                .find(|node| node.id == id)
                .map(|node| node.differs.clone())
                .unwrap_or_default()
        };
        assert!(
            differs("fx-pick").contains(&Reading::Value),
            "{mode}: the diff names the select's value"
        );
        for (row, option) in ["fx-banana", "fx-cherry"].into_iter().enumerate() {
            assert!(
                differs(option).contains(&Reading::Selected),
                "{mode}: row {row}: the diff names an option's selection"
            );
        }
        assert!(
            value(&session, "fx-pick").as_deref() == Some("c"),
            "{mode}: the next snapshot reads the chosen option's value"
        );
    }
}

/// Keys pressed on `fx-named` after a click focuses it, and the value each leaves.
const KEYS: [(&str, &str); 3] = [("arrow-down", "g"), ("arrow-down", "g"), ("home", "r")];

#[test]
fn a_click_then_a_key_chooses_an_option() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold(incremental);
        let clicked = act(&mut session, &click("fx-named"));
        assert!(
            clicked.is_some_and(|clicked| clicked.settled),
            "{mode}: the click is accepted and settled"
        );
        assert!(
            focused(&session).0 == ["fx-named"],
            "{mode}: the click focuses the select"
        );
        for (row, (key, expected)) in KEYS.into_iter().enumerate() {
            let pressed = act(&mut session, &press(key, false));
            assert!(
                pressed.is_some_and(|pressed| pressed.settled),
                "{mode}: row {row}: the press is accepted and settled"
            );
            assert!(
                value(&session, "fx-named").as_deref() == Some(expected),
                "{mode}: row {row}: the key leaves its option chosen"
            );
        }
    }
}

#[test]
fn the_tab_walk_skips_the_disabled_select_before_and_after_the_choices() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold(incremental);
        let at_boot = tab_walk(&mut session, &mode);
        assert!(
            at_boot == TAB_WALK,
            "{mode}: Tab walks the stated stops, the disabled select skipped"
        );
        for (row, call) in [
            type_into("fx-pick", "a"),
            click("fx-named"),
            press("end", false),
        ]
        .iter()
        .enumerate()
        {
            assert!(
                act(&mut session, call).is_some(),
                "{mode}: row {row}: the choice is accepted"
            );
        }
        assert!(
            value(&session, "fx-pick").as_deref() == Some("a")
                && value(&session, "fx-named").as_deref() == Some("g"),
            "{mode}: the choices moved both selects"
        );
        assert!(
            tab_walk(&mut session, &mode) == at_boot,
            "{mode}: the Tab walk reads the same after the choices"
        );
    }
}

#[test]
fn a_disabled_select_and_an_unchosen_option_are_refused() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold(incremental);
        let rows = [
            (type_into("fx-off", "X"), Cause::Disabled),
            (click("fx-apple"), Cause::Covered),
        ];
        for (row, (call, expected)) in rows.iter().enumerate() {
            let (cause, unchanged) = refused_unchanged(&mut session, call);
            assert!(
                cause == Some(*expected),
                "{mode}: row {row}: the refusal's cause"
            );
            assert!(
                unchanged,
                "{mode}: row {row}: the refused call leaves the snapshot as it was"
            );
        }
    }
}
