//! One `Session` holds one headless stand instance across commands, in both layout modes: a
//! click's effect and a created row are read by a later command, the focus one command leaves
//! is what the next one reads, and the timer's time moves only with ticks delivered on the
//! handle the check holds.

use blitz_dom::Document;
use escher_driver::Session;
use keyboard_types::Key;
use seven_guis::stand::{self, LeanTask};

mod common;
use common::boot;

/// The accessible name a snapshot of the held instance reads for `id`: `None` when it holds no
/// such node.
fn name_of(session: &Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .map(|node| node.name.clone())
}

/// The value a snapshot of the held instance reads for the input `id`.
fn value_of(session: &Session, id: &str) -> Option<String> {
    session
        .harness()
        .doc
        .snapshot()
        .get(id)
        .and_then(|node| node.state.value.clone())
}

/// The ids reading focused in a snapshot of the held instance, and the id the accessibility
/// tree's focus carries: none when that is the `Window`.
fn focused(session: &Session) -> (Vec<String>, Vec<String>) {
    let doc = &session.harness().doc;
    let in_snapshot = doc
        .snapshot()
        .nodes()
        .filter(|node| node.state.focused)
        .map(|node| node.id.clone())
        .collect();
    let tree = doc.accessibility_tree();
    let in_tree = tree
        .nodes
        .iter()
        .filter(|(id, _)| *id == tree.focus)
        .filter_map(|(_, node)| node.author_id())
        .map(str::to_string)
        .collect();
    (in_snapshot, in_tree)
}

#[test]
fn a_later_command_reads_what_an_earlier_command_left() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");

        let mut counter = Session::start("counter", || boot(LeanTask::Counter, incremental))
            .expect("a counter session starts");
        assert!(
            name_of(&counter, "counter-value").as_deref() == Some("0"),
            "{mode}: Counter: the count reads zero on a started session"
        );
        counter.harness_mut().click("#counter-increment");
        assert!(
            name_of(&counter, "counter-value").as_deref() == Some("1"),
            "{mode}: Counter: a later snapshot reads the first click"
        );
        counter.harness_mut().click("#counter-increment");
        assert!(
            name_of(&counter, "counter-value").as_deref() == Some("2"),
            "{mode}: Counter: the second click adds to what the first left"
        );

        let mut crud = Session::start("crud", || boot(LeanTask::Crud, incremental))
            .expect("a CRUD session starts");
        assert!(
            name_of(&crud, "crud-person-2").is_some() && name_of(&crud, "crud-person-3").is_none(),
            "{mode}: Crud: three rows on a started session"
        );
        crud.harness_mut().click("#crud-name");
        crud.harness_mut().type_text("Ada");
        crud.harness_mut().click("#crud-surname");
        crud.harness_mut().type_text("Lovelace");
        assert!(
            value_of(&crud, "crud-name").as_deref() == Some("Ada"),
            "{mode}: Crud: the name typed first is still read after the surname was typed"
        );
        crud.harness_mut().click("#crud-create");
        assert!(
            name_of(&crud, "crud-person-3")
                .is_some_and(|row| row.contains("Ada") && row.contains("Lovelace")),
            "{mode}: Crud: a later snapshot reads the row built from the typed fields"
        );
    }
}

#[test]
fn focus_left_by_one_command_is_read_by_the_next() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let mut crud = Session::start("crud", || boot(LeanTask::Crud, incremental))
            .expect("a CRUD session starts");
        assert!(
            crud.harness().doc.snapshot().nodes().count() > 0,
            "{mode}: Crud: the snapshot has nodes"
        );
        let (in_snapshot, in_tree) = focused(&crud);
        assert!(
            in_snapshot.is_empty() && in_tree.is_empty(),
            "{mode}: Crud: nothing reads focused on a started session"
        );

        crud.harness_mut().press(Key::Tab);
        crud.harness_mut().pump();

        let (in_snapshot, in_tree) = focused(&crud);
        assert!(
            in_snapshot == ["back-btn"],
            "{mode}: Crud: a later snapshot reads exactly the first control focused"
        );
        assert!(
            in_tree == ["back-btn"],
            "{mode}: Crud: the accessibility tree's focus is that control"
        );
    }
}

#[test]
fn timer_time_moves_only_with_delivered_ticks() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (harness, ticks) = stand::boot_timer(stand::options(incremental));
        let mut timer = Session::start("timer", || harness).expect("a timer session starts");
        let elapsed = |timer: &Session| timer.harness().text_content("#timer-elapsed");

        assert!(
            elapsed(&timer) == "Elapsed: 0.0s",
            "{mode}: Timer: no time has passed on a started session"
        );
        timer.harness_mut().pump();
        assert!(
            elapsed(&timer) == "Elapsed: 0.0s",
            "{mode}: Timer: a pump with no delivered tick leaves the time unchanged"
        );

        ticks.deliver(3);
        timer.harness_mut().pump();
        assert!(
            elapsed(&timer) == "Elapsed: 0.3s",
            "{mode}: Timer: delivered ticks then a pump move the time"
        );
        timer.harness_mut().pump();
        assert!(
            elapsed(&timer) == "Elapsed: 0.3s",
            "{mode}: Timer: the time holds where the ticks left it"
        );

        ticks.deliver(2);
        timer.harness_mut().pump();
        assert!(
            elapsed(&timer) == "Elapsed: 0.5s",
            "{mode}: Timer: later ticks add to the time the earlier ones left"
        );
    }
}
