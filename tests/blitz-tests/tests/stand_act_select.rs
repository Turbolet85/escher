//! What the driver's verbs do to a single-choice `select`, on a fixture page beside the stand,
//! whose tasks hold none. In both layout modes, through `Session::run`: the snapshot reads the
//! select labelled `Fruit` with role `ComboBox`, that name and its chosen option's value, its
//! options under it named by their labels and the chosen one selected; `type` with an option's
//! label chooses that option, its diff naming the select's value and the two options' selection;
//! a `click` then a `press` of the arrows or Home chooses by key; the Tab walk skips the disabled
//! select and reads the same before and after the choices; a `type` into the disabled select is
//! refused `disabled` and a `click` on an option that is not chosen — seen in its select's box,
//! where the chosen option's text lies — `covered`, each with the snapshot unchanged. A `multiple`
//! select reads its name, `ListBox` and its rows, each row with bounds of its own: a `click` on a
//! row is accepted and chooses it, its diff naming the row's selection and the select's value
//! where the first chosen moved; a `press` with shift spans; a `type` chooses its match alone with
//! no click on the row at the select's centre; and an option under a closed select's group stays
//! refused `covered`. A failure message carries the layout mode and a row index, never an id or
//! what a screen reads.

use accesskit::Role;
use dioxus::prelude::*;
use dioxus_native_dom::Reading;
use escher_driver::{Cause, Session};

mod fixtures;
mod session_common;
use session_common::{Acted, act, click, focused, press, refused_unchanged, type_into};

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

/// A `multiple` select of three rows, and a closed select whose options lie under a group.
fn rows_page() -> Element {
    rsx! {
        div { id: "fx-root",
            select { id: "fx-rows", aria_label: "Rows", multiple: true,
                option { id: "fx-one", value: "1", "One" }
                option { id: "fx-two", value: "2", "Two" }
                option { id: "fx-three", value: "3", "Three" }
            }
            select { id: "fx-closed", aria_label: "Closed",
                optgroup { id: "fx-closed-group", label: "Group",
                    option { id: "fx-closed-a", value: "a", "A" }
                    option { id: "fx-closed-b", value: "b", "B" }
                }
            }
        }
    }
}

fn hold_rows(incremental: bool) -> Session {
    fixtures::hold("select", rows_page, incremental)
}

/// What `id` is named as changing in `acted`'s diff.
fn differs(acted: &Acted, id: &str) -> Vec<Reading> {
    acted
        .diff
        .changed
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.differs.clone())
        .unwrap_or_default()
}

/// The rows of `fx-rows` reading selected, in order.
fn chosen_rows(session: &Session) -> Vec<String> {
    let snapshot = session.harness().doc.snapshot();
    ["fx-one", "fx-two", "fx-three"]
        .into_iter()
        .filter(|id| {
            snapshot
                .get(id)
                .is_some_and(|row| row.state.selected == Some(true))
        })
        .map(str::to_string)
        .collect()
}

#[test]
fn a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let session = hold_rows(incremental);
        let doc = &session.harness().doc;
        assert!(
            doc.unkeyed_actionable().is_empty(),
            "{mode}: every element an agent can act on reads an author key"
        );
        let snapshot = doc.snapshot();
        let rows = snapshot.get("fx-rows");
        assert!(
            rows.is_some_and(|rows| (rows.role, rows.name.as_str()) == (Role::ListBox, "Rows")),
            "{mode}: the select reads its role and its name"
        );
        let bounds: Vec<_> = ["fx-one", "fx-two", "fx-three"]
            .into_iter()
            .filter_map(|id| snapshot.get(id))
            .map(|row| {
                assert!(
                    row.role == Role::ListBoxOption && row.state.selected == Some(false),
                    "{mode}: a row reads its role and no selection"
                );
                (row.bounds.y, row.bounds.height)
            })
            .collect();
        assert!(bounds.len() == 3, "{mode}: the select holds its three rows");
        assert!(
            bounds
                .windows(2)
                .all(|pair| pair[0].0 + pair[0].1 <= pair[1].0),
            "{mode}: each row reads bounds of its own, below the last"
        );
        assert!(
            snapshot.get("fx-closed-group").is_some_and(|group| {
                (group.role, group.name.as_str()) == (Role::Group, "Group")
            }),
            "{mode}: a group reads its role and its label as its name"
        );
    }
}

#[test]
fn a_click_on_a_row_is_accepted_and_chooses_it() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_rows(incremental);
        let clicked = act(&mut session, &click("fx-two"))
            .unwrap_or_else(|| panic!("{mode}: the click is accepted"));
        assert!(clicked.settled, "{mode}: the click settles");
        assert!(
            differs(&clicked, "fx-two").contains(&Reading::Selected),
            "{mode}: the diff names the row's selection"
        );
        assert!(
            differs(&clicked, "fx-rows").contains(&Reading::Value),
            "{mode}: the diff names the select's value, the first chosen having moved"
        );
        assert!(
            value(&session, "fx-rows").as_deref() == Some("2"),
            "{mode}: the select reads the row's value"
        );
        assert!(
            focused(&session).0 == ["fx-rows"],
            "{mode}: the click focuses the select"
        );
    }
}

#[test]
fn a_press_with_shift_spans_the_rows() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_rows(incremental);
        assert!(
            act(&mut session, &click("fx-one")).is_some(),
            "{mode}: the click is accepted"
        );
        for (row, expected) in [
            vec!["fx-one", "fx-two"],
            vec!["fx-one", "fx-two", "fx-three"],
        ]
        .into_iter()
        .enumerate()
        {
            let pressed = act(&mut session, &press("arrow-down", true));
            assert!(
                pressed.is_some_and(|pressed| pressed.settled),
                "{mode}: row {row}: the press is accepted and settled"
            );
            assert!(
                chosen_rows(&session) == expected,
                "{mode}: row {row}: the span reads its rows"
            );
        }
    }
}

#[test]
fn type_on_a_multiple_select_chooses_its_match_alone_with_no_stray_click() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_rows(incremental);
        let typed = act(&mut session, &type_into("fx-rows", "Three"))
            .unwrap_or_else(|| panic!("{mode}: the type is accepted"));
        assert!(
            differs(&typed, "fx-three").contains(&Reading::Selected),
            "{mode}: the diff names the matched row's selection"
        );
        assert!(
            differs(&typed, "fx-two").is_empty(),
            "{mode}: the row at the select's centre takes no click"
        );
        assert!(
            chosen_rows(&session) == ["fx-three"],
            "{mode}: the text chooses its match alone"
        );

        let retyped = act(&mut session, &type_into("fx-rows", "One"));
        assert!(retyped.is_some(), "{mode}: a second type is accepted");
        assert!(
            chosen_rows(&session) == ["fx-one"],
            "{mode}: a second type starts its text afresh"
        );

        for (row, text) in ["No such row", "", " One"].into_iter().enumerate() {
            let left = act(&mut session, &type_into("fx-rows", text));
            assert!(left.is_some(), "{mode}: row {row}: the type is accepted");
            assert!(
                chosen_rows(&session) == ["fx-one"],
                "{mode}: row {row}: a text no label starts leaves the set"
            );
        }
    }
}

#[test]
fn an_option_under_a_closed_selects_group_is_refused_covered() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_rows(incremental);
        let (cause, unchanged) = refused_unchanged(&mut session, &click("fx-closed-b"));
        assert!(cause == Some(Cause::Covered), "{mode}: the refusal's cause");
        assert!(unchanged, "{mode}: the refused call changes nothing");
    }
}
