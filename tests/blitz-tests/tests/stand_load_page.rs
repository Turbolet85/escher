//! The stand's load page, booted through its own boot and held as a driver session, meets a load
//! in flight and waits it out, in both layout modes: before the click the screen does not read
//! the page's message; a `click` on its button links a stylesheet that the boot's in-process
//! stand-in answers on a thread of its own, and the click returns settled with the message added
//! to its diff and on the next screen, the session's clock unmoved; Enter on the button, reached
//! by Tab through the driver's `press`, does the same. Every element an agent can act on reads an
//! author key, and the lint reads no finding at boot. The stand-in's delay is the stand's own:
//! this check reads no wall clock and sleeps nowhere. A failure message carries the layout mode
//! only.

use escher_driver::{Outcome, Session};
use seven_guis::stand;

mod session_common;
use session_common::{act, click, lint, linted, press, snapshot};

/// What the screen reads for the message once the sheet shows it.
const MESSAGE_LINE: &str = "Paragraph \"Loaded\" id=\"load-done\"";

/// A session holding the load page booted through the stand in one layout mode.
fn hold_load(incremental: bool) -> Session {
    Session::start("load", || stand::boot_load(stand::options(incremental)))
        .expect("the label is one of the closed set")
}

/// The screen a `snapshot` call answers, whole.
#[track_caller]
fn screen(session: &mut Session, mode: &str) -> String {
    match session.run(&snapshot()) {
        Ok(Outcome::Screen { text }) => text,
        _ => panic!("{mode}: a snapshot answers the whole screen"),
    }
}

#[test]
fn the_load_page_click_waits_out_its_load_and_shows_the_message() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_load(incremental);
        assert!(
            !screen(&mut session, &mode).contains("id=\"load-done\""),
            "{mode}: the message does not read before the click"
        );

        let time = session.harness().time();
        let acted = act(&mut session, &click("load-fetch"))
            .unwrap_or_else(|| panic!("{mode}: the click is accepted and acts"));
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: the click returns settled once the load is answered"
        );
        assert!(
            acted
                .diff
                .added
                .iter()
                .any(|node| node.id == "load-done" && node.name == "Loaded"),
            "{mode}: the diff adds the message"
        );
        assert!(
            session.harness().time() == time,
            "{mode}: the wait moved no time"
        );
        assert!(
            screen(&mut session, &mode).contains(MESSAGE_LINE),
            "{mode}: the next screen reads the message"
        );
    }
}

#[test]
fn the_load_page_enter_waits_out_its_load_and_shows_the_message() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_load(incremental);
        for row in 0..2 {
            assert!(
                act(&mut session, &press("tab", false)).is_some_and(|tabbed| tabbed.settled),
                "{mode}: Tab {row} is accepted and settled"
            );
        }
        assert!(
            !screen(&mut session, &mode).contains("id=\"load-done\""),
            "{mode}: the message does not read before the key"
        );

        let time = session.harness().time();
        let acted = act(&mut session, &press("enter", false))
            .unwrap_or_else(|| panic!("{mode}: the press is accepted and acts"));
        assert!(
            acted.settled && acted.busy.is_none(),
            "{mode}: the press returns settled once the load is answered"
        );
        assert!(
            acted
                .diff
                .added
                .iter()
                .any(|node| node.id == "load-done" && node.name == "Loaded"),
            "{mode}: the diff adds the message"
        );
        assert!(
            session.harness().time() == time,
            "{mode}: the wait moved no time"
        );
        assert!(
            screen(&mut session, &mode).contains(MESSAGE_LINE),
            "{mode}: the next screen reads the message"
        );
    }
}

#[test]
fn the_load_page_is_keyed_and_reads_no_finding_at_boot() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut session = hold_load(incremental);
        assert!(
            session.harness().doc.unkeyed_actionable().is_empty(),
            "{mode}: every actionable element reads an author key"
        );
        assert!(
            linted(&mut session, &lint()).is_some_and(|findings| findings.is_empty()),
            "{mode}: the lint reads no finding"
        );
    }
}
