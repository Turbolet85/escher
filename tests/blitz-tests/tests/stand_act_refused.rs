//! A call the driver refuses changes nothing. On the stand's CRUD task, in both layout modes, a
//! table of refused calls — a verb the table lacks, a missing argument, an id of the longest
//! admitted length that names nothing, and the id of a row the driver has just deleted — each
//! returns its cause of the closed set, leaves the snapshot of the held instance equal to the
//! one taken before the call, does not panic, and reads as a text that holds none of the bytes
//! the call supplied. An id no screen of the session read is `not-found`; the id of the deleted
//! row, which an earlier screen read, is `stale` — and one id reads the first and then the
//! second, by what the session read between the two calls. A failure message carries the layout
//! mode and a row index, never an id, an argument or what a screen reads.

use escher_driver::{ArgValue, CAUSES, Call, Cause, Fault, MAX_ID_BYTES, Session};
use seven_guis::stand::LeanTask;

mod session_common;
use session_common::{act, click, hold, refused_unchanged, type_into};

/// A marker no cause's text holds, carried by every value a refused call supplies.
const MARK: &str = "zq7~";

/// The row the driver deletes before the last call names it.
const DELETED: &str = "crud-person-0";

/// The id the next row created on a fresh CRUD session reads.
const NEXT_ROW: &str = "crud-person-3";

/// One refused call: what the driver does on the session first, the call, the cause it is
/// refused with, the rule a malformed one broke, and the texts the call supplied.
struct Row {
    prepare: fn(&mut Session) -> bool,
    call: Call,
    cause: Cause,
    fault: Option<Fault>,
    supplied: Vec<String>,
}

fn nothing(_session: &mut Session) -> bool {
    true
}

/// Selects the first row and deletes it, through the driver: whether the delete removed it.
fn delete_the_first_row(session: &mut Session) -> bool {
    act(session, &click(DELETED)).is_some()
        && act(session, &click("crud-delete")).is_some_and(|deleted| deleted.removed() == [DELETED])
}

fn rows() -> Vec<Row> {
    let unknown_verb = format!("{MARK}capture");
    let target = format!("{MARK}target");
    let longest = MARK.repeat(MAX_ID_BYTES / MARK.len());
    vec![
        Row {
            prepare: nothing,
            call: Call {
                verb: unknown_verb.clone(),
                args: vec![("id".to_string(), ArgValue::Text(target.clone()))],
            },
            cause: Cause::UnknownVerb,
            fault: None,
            supplied: vec![unknown_verb, target.clone()],
        },
        Row {
            prepare: nothing,
            call: Call {
                verb: "type".to_string(),
                args: vec![("id".to_string(), ArgValue::Text(target.clone()))],
            },
            cause: Cause::Malformed,
            fault: Some(Fault::Missing("text")),
            supplied: vec![target],
        },
        Row {
            prepare: nothing,
            call: click(&longest),
            cause: Cause::NotFound,
            fault: None,
            supplied: vec![MARK.to_string(), longest],
        },
        Row {
            prepare: delete_the_first_row,
            call: click(DELETED),
            cause: Cause::Stale,
            fault: None,
            supplied: vec![DELETED.to_string()],
        },
    ]
}

#[test]
fn a_refused_call_names_its_cause_and_leaves_the_instance_unchanged() {
    for incremental in [false, true] {
        let rows = rows();
        assert_eq!(rows.len(), 4);
        for (index, row) in rows.into_iter().enumerate() {
            let mode = format!("incremental={incremental}: row {index}");
            let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
            assert!(
                (row.prepare)(&mut session),
                "{mode}: the steps before the call ran"
            );
            let before = session.harness().doc.snapshot();
            assert!(before.nodes().count() > 0, "{mode}: the snapshot has nodes");
            assert!(
                before.get(DELETED).is_some() == (index != 3),
                "{mode}: the first row is on the screen unless the driver deleted it"
            );

            let Err(refusal) = session.run(&row.call) else {
                panic!("{mode}: the call is refused");
            };
            assert!(
                refusal.cause() == row.cause && CAUSES.contains(&refusal.cause()),
                "{mode}: the refusal names its cause of the closed set"
            );
            assert!(
                refusal.fault() == row.fault,
                "{mode}: the refusal names the rule a malformed call broke, and none otherwise"
            );
            let after = session.harness().doc.snapshot();
            assert!(
                before == after && before.to_text() == after.to_text(),
                "{mode}: the snapshots before and after the refused call are equal"
            );

            let text = refusal.to_string();
            assert!(
                text.starts_with(row.cause.name()),
                "{mode}: the refusal reads as its cause first"
            );
            assert!(
                row.supplied.iter().all(|supplied| !text.contains(supplied)),
                "{mode}: the refusal's text holds none of the bytes the call supplied"
            );
        }
    }
}

#[test]
fn the_longest_admitted_id_is_looked_up_and_one_byte_more_is_malformed() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        let longest = MARK.repeat(MAX_ID_BYTES / MARK.len());
        assert_eq!(longest.len(), 1024);

        assert!(
            session
                .run(&click(&longest))
                .is_err_and(|refusal| refusal.cause() == Cause::NotFound),
            "{mode}: an id of the longest admitted length reaches the lookup"
        );
        let over = format!("{longest}z");
        assert!(
            session.run(&click(&over)).is_err_and(|refusal| {
                refusal.cause() == Cause::Malformed
                    && refusal.fault() == Some(Fault::OutOfBound("id"))
            }),
            "{mode}: one byte more is refused before any lookup"
        );
    }
}

#[test]
fn one_id_is_not_found_before_a_screen_read_it_and_stale_after() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            session.harness().doc.snapshot().get(NEXT_ROW).is_none(),
            "{mode}: the row is not on the screen at boot"
        );

        let (cause, unchanged) = refused_unchanged(&mut session, &click(NEXT_ROW));
        assert!(
            cause == Some(Cause::NotFound) && unchanged,
            "{mode}: an id no screen of the session read is refused not-found, the instance \
             unchanged"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &type_into(NEXT_ROW, MARK));
        assert!(
            cause == Some(Cause::NotFound) && unchanged,
            "{mode}: a type naming that id is refused not-found too, the instance unchanged"
        );

        assert!(
            act(&mut session, &click("crud-create"))
                .is_some_and(|created| created.added() == [NEXT_ROW]),
            "{mode}: the driver creates the row"
        );
        assert!(
            act(&mut session, &click(NEXT_ROW)).is_some(),
            "{mode}: the driver selects the row"
        );
        assert!(
            act(&mut session, &click("crud-delete"))
                .is_some_and(|deleted| deleted.removed() == [NEXT_ROW]),
            "{mode}: the driver deletes the row"
        );

        let before = session.harness().doc.snapshot();
        let Err(refusal) = session.run(&click(NEXT_ROW)) else {
            panic!("{mode}: the call naming the deleted row is refused");
        };
        assert!(
            refusal.cause() == Cause::Stale && refusal.fault().is_none(),
            "{mode}: the same id is refused stale once a screen has read it"
        );
        assert!(
            before == session.harness().doc.snapshot(),
            "{mode}: the snapshots before and after the refused call are equal"
        );
        assert!(
            !refusal.to_string().contains(NEXT_ROW),
            "{mode}: the refusal's text holds none of the bytes the call supplied"
        );
        let (cause, unchanged) = refused_unchanged(&mut session, &type_into(NEXT_ROW, MARK));
        assert!(
            cause == Some(Cause::Stale) && unchanged,
            "{mode}: a type naming the deleted row is refused stale too, the instance unchanged"
        );
    }
}
