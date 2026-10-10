//! The driver's `type` replaces what a control holds: after it the control reads exactly the
//! text, whatever it held, and an empty text clears it. In both layout modes. *On the stand's
//! Flight Booker,* whose departure date is filled at boot: a `type` leaves the text alone in the
//! control, a second one replaces the first, and an empty one leaves the control empty. *On the
//! stand's CRUD,* whose name field is empty at boot: a `type` reads as it did before the driver
//! replaced. *On a password fixture:* the written form of a `type` holds the mask and never the
//! typed text. The check clicks and types characters only; its one deleting step is the
//! driver's own empty-text path. Guards a measured reading: `type` used to leave the old value
//! in place and put its text beside it, so a control never read what an agent typed. A failure
//! message carries the layout mode and the task or fixture, never an id or what a screen reads.

use blitz_test_harness::Harness;
use dioxus::prelude::*;
use dioxus_native_dom::MASKED_VALUE;
use escher_driver::{Call, Session};
use seven_guis::stand::{self, LeanTask};

mod common;
mod session_common;
use session_common::{act, hold, type_into};

/// The date the Flight Booker's departure field holds at boot.
const BOOT_DATE: &str = "01.01.2026";

/// What a check types into the password fixture: synthetic, and no part of any id or name.
const SECRET: &str = "Typed-Secret-58hRvm";

fn value(session: &Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.value.clone())
}

fn enabled(session: &Session, id: &str) -> Option<bool> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.enabled)
}

/// A password input and a text input, each empty.
fn password_fixture() -> Element {
    rsx! {
        div { id: "fx-root",
            input { id: "fx-secret", r#type: "password", aria_label: "Secret" }
            input { id: "fx-plain", aria_label: "Plain" }
        }
    }
}

#[test]
fn a_type_leaves_exactly_its_text_in_a_filled_control() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: FlightBooker");
        let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
        assert!(
            value(&session, "flight-start").as_deref() == Some(BOOT_DATE),
            "{mode}: the departure field is filled at boot"
        );
        assert!(
            enabled(&session, "flight-book") == Some(true),
            "{mode}: the booking is enabled at boot"
        );

        let Some(typed) = act(&mut session, &type_into("flight-start", "24.12.2026")) else {
            panic!("{mode}: the type runs");
        };
        assert!(typed.settled, "{mode}: the type returns settled");
        assert!(
            value(&session, "flight-start").as_deref() == Some("24.12.2026"),
            "{mode}: the field reads exactly the typed text"
        );
        assert!(
            common::editor_text(session.harness(), "flight-start") == "24.12.2026",
            "{mode}: the engine's editor holds exactly the typed text"
        );
        assert!(
            typed.changed().contains(&"flight-start")
                && typed.added().is_empty()
                && typed.removed().is_empty(),
            "{mode}: the diff names the field among the changed and adds and removes nothing"
        );
        assert!(
            enabled(&session, "flight-book") == Some(true),
            "{mode}: a date leaves the booking enabled"
        );

        // A second type replaces the first: nothing of it is left beside the new text.
        let Some(again) = act(&mut session, &type_into("flight-start", "next week")) else {
            panic!("{mode}: the second type runs");
        };
        assert!(
            again.settled && value(&session, "flight-start").as_deref() == Some("next week"),
            "{mode}: the field reads exactly the second text"
        );
        assert!(
            again.changed().contains(&"flight-start")
                && again.changed().contains(&"flight-book")
                && enabled(&session, "flight-book") == Some(false),
            "{mode}: a text that is no date disables the booking, and the diff names both"
        );

        // A longer text over a shorter one, and a text holding a space and a non-ASCII letter.
        for text in ["31.12.2026 or so", "é ✓", "7"] {
            assert!(
                act(&mut session, &type_into("flight-start", text)).is_some_and(|t| t.settled)
                    && value(&session, "flight-start").as_deref() == Some(text),
                "{mode}: the field reads exactly each text typed over the last"
            );
        }
    }
}

#[test]
fn a_type_with_an_empty_text_clears_the_control() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: FlightBooker");
        let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
        assert!(
            value(&session, "flight-start").as_deref() == Some(BOOT_DATE),
            "{mode}: the departure field is filled at boot"
        );
        let Some(cleared) = act(&mut session, &type_into("flight-start", "")) else {
            panic!("{mode}: the type with an empty text runs");
        };
        assert!(cleared.settled, "{mode}: the type returns settled");
        assert!(
            value(&session, "flight-start").as_deref() == Some("")
                && common::editor_text(session.harness(), "flight-start").is_empty(),
            "{mode}: the field reads empty"
        );
        assert!(
            cleared.changed().contains(&"flight-start")
                && cleared.changed().contains(&"flight-book")
                && enabled(&session, "flight-book") == Some(false),
            "{mode}: the diff names the field, and the booking the empty date disables"
        );

        // Clearing a control that is empty leaves it empty.
        assert!(
            act(&mut session, &type_into("flight-start", ""))
                .is_some_and(|again| again.settled && again.diff.is_empty())
                && value(&session, "flight-start").as_deref() == Some(""),
            "{mode}: a second empty type finds nothing to clear"
        );
        // And what is typed next is the whole value.
        assert!(
            act(&mut session, &type_into("flight-start", BOOT_DATE)).is_some_and(|t| t.settled)
                && value(&session, "flight-start").as_deref() == Some(BOOT_DATE)
                && enabled(&session, "flight-book") == Some(true),
            "{mode}: the field reads the date typed into the cleared control"
        );
    }
}

#[test]
fn a_type_into_an_empty_control_reads_as_it_did() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            value(&session, "crud-name").as_deref() == Some(""),
            "{mode}: the name field is empty at boot"
        );
        let Some(typed) = act(&mut session, &type_into("crud-name", "Ada")) else {
            panic!("{mode}: the type runs");
        };
        assert!(
            typed.settled
                && typed.changed() == ["crud-name"]
                && typed.added().is_empty()
                && typed.removed().is_empty(),
            "{mode}: the diff names the field alone"
        );
        assert!(
            value(&session, "crud-name").as_deref() == Some("Ada")
                && common::editor_text(session.harness(), "crud-name") == "Ada",
            "{mode}: the field reads the typed text"
        );
        // The other field was not touched by the select-all the type makes.
        assert!(
            value(&session, "crud-surname").as_deref() == Some(""),
            "{mode}: the surname field stays empty"
        );
        assert!(
            act(&mut session, &type_into("crud-name", "Grace")).is_some_and(|t| t.settled)
                && value(&session, "crud-name").as_deref() == Some("Grace"),
            "{mode}: a second type replaces the first"
        );
    }
}

#[test]
fn a_masked_controls_written_form_holds_the_mask_and_never_the_typed_text() {
    assert!(!SECRET.contains(MASKED_VALUE) && !MASKED_VALUE.is_empty());
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: password fixture");
        let mut session = Session::start("filled", || {
            Harness::from_vdom(
                VirtualDom::new(password_fixture),
                stand::options(incremental),
            )
        })
        .expect("the label is one of the closed set");
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element of the fixture reads an author key"
        );

        let outcome = session
            .run(&type_into("fx-secret", SECRET))
            .unwrap_or_else(|_| panic!("{mode}: the type into the password field runs"));
        assert!(
            common::editor_text(session.harness(), "fx-secret") == SECRET,
            "{mode}: the engine's editor holds the typed text"
        );
        let written = outcome.to_json();
        assert!(
            written.matches(MASKED_VALUE).count() == 1 && written.matches(SECRET).count() == 0,
            "{mode}: the written outcome holds the mask {} times and the typed text {} times",
            written.matches(MASKED_VALUE).count(),
            written.matches(SECRET).count()
        );
        assert!(
            written.matches('\n').count() == 0,
            "{mode}: the written outcome is one line"
        );

        let snapshot = Call {
            verb: "snapshot".to_string(),
            args: Vec::new(),
        };
        let screen = session
            .run(&snapshot)
            .unwrap_or_else(|_| panic!("{mode}: the snapshot runs"))
            .to_json();
        assert!(
            screen.matches(MASKED_VALUE).count() == 1 && screen.matches(SECRET).count() == 0,
            "{mode}: the written screen holds the mask {} times and the typed text {} times",
            screen.matches(MASKED_VALUE).count(),
            screen.matches(SECRET).count()
        );

        // The same text typed into the plain field is its value: the mask is the password's.
        let plain = session
            .run(&type_into("fx-plain", SECRET))
            .unwrap_or_else(|_| panic!("{mode}: the type into the plain field runs"))
            .to_json();
        assert!(
            plain.matches(SECRET).count() == 1,
            "{mode}: the written outcome of the plain field holds its text once"
        );

        // A type over a masked value replaces it like any other.
        assert!(
            act(&mut session, &type_into("fx-secret", "second")).is_some_and(|t| t.settled)
                && common::editor_text(session.harness(), "fx-secret") == "second",
            "{mode}: a second type replaces what the password field held"
        );
    }
}
