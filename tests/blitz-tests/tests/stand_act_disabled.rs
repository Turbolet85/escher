//! A driver action aimed at a control that reads not enabled is refused `disabled`, and the
//! same control is acted on once the app enables it. On the stand, in both layout modes: on the
//! Flight Booker a `type` into the return date and a `click` on it are refused while the flight
//! is one-way, and after the driver chooses the return flight the same `type` is accepted and
//! the control holds the typed text; on CRUD a `click` on Delete is refused with no row
//! selected, and after the driver selects a row it is accepted and removes that row. A refused
//! call leaves the snapshot and both focus readings as they were. A failure message carries the
//! layout mode and the task, never an id or what a screen reads.

use escher_driver::Cause;
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, click, hold, refused_unchanged, type_into};

/// What the driver types: letters no date holds.
const TYPED: &str = "zq";

/// Whether the control `id` names reads enabled in a snapshot of the held instance.
fn enabled(session: &escher_driver::Session, id: &str) -> Option<bool> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.enabled)
}

/// The value the control `id` names reads in a snapshot of the held instance.
fn value(session: &escher_driver::Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.value.clone())
}

#[test]
fn a_disabled_date_input_refuses_typing_until_the_app_enables_it() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: FlightBooker");
        let (mut session, _ticks) = hold(LeanTask::FlightBooker, incremental);
        assert!(
            enabled(&session, "flight-return-date") == Some(false),
            "{mode}: the return date reads not enabled at boot"
        );
        let at_boot = value(&session, "flight-return-date");
        assert!(
            at_boot.as_deref().is_some_and(|held| !held.contains(TYPED)),
            "{mode}: the return date holds a value without the text to type"
        );

        let (cause, unchanged) =
            refused_unchanged(&mut session, &type_into("flight-return-date", TYPED));
        assert!(
            cause == Some(Cause::Disabled),
            "{mode}: a type into the disabled control is refused disabled"
        );
        assert!(
            unchanged,
            "{mode}: the refused type leaves the snapshot and the focus as they were"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &click("flight-return-date"));
        assert!(
            cause == Some(Cause::Disabled) && unchanged,
            "{mode}: a click on the disabled control is refused disabled, the instance unchanged"
        );

        assert!(
            act(&mut session, &click("flight-return")).is_some_and(|chosen| chosen.settled),
            "{mode}: the driver chooses the return flight"
        );
        assert!(
            enabled(&session, "flight-return-date") == Some(true),
            "{mode}: the return date reads enabled once the flight is a return"
        );
        let Some(typed) = act(&mut session, &type_into("flight-return-date", TYPED)) else {
            panic!("{mode}: the same type is accepted once the control is enabled");
        };
        assert!(typed.settled, "{mode}: the accepted type returns settled");
        let after = value(&session, "flight-return-date");
        assert!(
            after.as_deref().is_some_and(|held| held.contains(TYPED)),
            "{mode}: the control's value reads the typed text"
        );
        assert!(
            after.map(|held| held.len()) == at_boot.map(|held| held.len() + TYPED.len()),
            "{mode}: the typed text is all that the value gained"
        );
    }
}

#[test]
fn a_disabled_delete_refuses_a_click_until_a_row_is_selected() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            enabled(&session, "crud-delete") == Some(false),
            "{mode}: Delete reads not enabled at boot"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click("crud-delete"));
        assert!(
            cause == Some(Cause::Disabled),
            "{mode}: a click on the disabled Delete is refused disabled"
        );
        assert!(
            unchanged,
            "{mode}: the refused click leaves the snapshot and the focus as they were"
        );

        assert!(
            act(&mut session, &click("crud-person-0")).is_some_and(|selected| selected.settled),
            "{mode}: the driver selects a row"
        );
        assert!(
            enabled(&session, "crud-delete") == Some(true),
            "{mode}: Delete reads enabled once a row is selected"
        );
        assert!(
            act(&mut session, &click("crud-delete"))
                .is_some_and(|deleted| deleted.settled && deleted.removed() == ["crud-person-0"]),
            "{mode}: the click on Delete is accepted and removes the selected row"
        );
    }
}
