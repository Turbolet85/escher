//! On one held stand instance, in both layout modes, an id names the same element after a
//! command as before it: every rendered id resolves to one element on both sides of a
//! state-changing command, a control's stable id read before the command is the `author_id`
//! of that element's accessibility node after it, and a CRUD row keeps its person's id across
//! a later Create and under a filter.

use blitz_dom::Document;
use dioxus_native_dom::NodeId;
use escher_driver::Session;
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use common::{controls, rendered};
use session_common::{change, hold};

/// Whether each of `ids` is the stable id of exactly one element of the held instance.
fn each_names_one_element(session: &Session, ids: &[&str]) -> bool {
    let read = session.harness().doc.element_ids();
    ids.iter()
        .all(|id| read.iter().filter(|(_, read)| read == id).count() == 1)
}

/// Each control of `task` the held instance renders: its element and the stable id that
/// element reads.
fn control_ids(session: &Session, task: LeanTask) -> Vec<(NodeId, Option<String>)> {
    let harness = session.harness();
    controls(task)
        .iter()
        .filter_map(|(id, _)| harness.query(&format!("#{id}")))
        .map(|element| (element, harness.doc.element_id(element)))
        .collect()
}

/// The row whose stable id is `id`: the one element reading it, and the name its snapshot
/// node reads. `None` when no element, or more than one, reads the id.
fn row(session: &Session, id: &str) -> Option<(NodeId, String)> {
    let doc = &session.harness().doc;
    let elements: Vec<NodeId> = doc
        .element_ids()
        .into_iter()
        .filter(|(_, read)| read == id)
        .map(|(element, _)| element)
        .collect();
    let [element] = elements.as_slice() else {
        return None;
    };
    let name = doc.snapshot().get(id)?.name.clone();
    Some((*element, name))
}

#[test]
fn an_id_names_the_same_element_after_a_command() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, ticks) = hold(task, incremental);

            assert!(
                each_names_one_element(&session, rendered(task)),
                "{mode}: every rendered id names one element on a started session"
            );
            let before = control_ids(&session, task);
            assert!(
                before
                    .iter()
                    .map(|(_, id)| id.as_deref())
                    .eq(controls(task).iter().map(|(id, _)| Some(*id))),
                "{mode}: every control is rendered and reads its own id"
            );
            let screen = session.harness().doc.snapshot();

            change(task, &mut session, &ticks);

            assert!(
                session.harness().doc.snapshot() != screen,
                "{mode}: the command changed what the instance shows"
            );
            assert!(
                each_names_one_element(&session, rendered(task)),
                "{mode}: every rendered id names one element after the command"
            );
            let tree = session.harness().doc.accessibility_tree();
            for (element, id) in &before {
                let carried = tree
                    .nodes
                    .iter()
                    .find(|(node, _)| node.0 == element.as_u64())
                    .and_then(|(_, node)| node.author_id());
                assert!(
                    carried.is_some() && carried == id.as_deref(),
                    "{mode}: a control's id before the command is its node's author_id after it"
                );
            }
        }
    }
}

#[test]
fn a_crud_row_keeps_its_id_across_commands() {
    for incremental in [false, true] {
        let mode = format!("incremental={incremental}: Crud");
        let (mut session, _ticks) = hold(LeanTask::Crud, incremental);
        assert!(
            row(&session, "crud-person-2").is_some() && row(&session, "crud-person-3").is_none(),
            "{mode}: three rows on a started session"
        );

        session.harness_mut().click("#crud-name");
        session.harness_mut().type_text("Ada");
        session.harness_mut().click("#crud-surname");
        session.harness_mut().type_text("Lovelace");
        session.harness_mut().click("#crud-create");
        let created = row(&session, "crud-person-3");
        assert!(
            created.is_some(),
            "{mode}: the created row reads the next person's id"
        );

        session.harness_mut().click("#crud-create");
        assert!(
            row(&session, "crud-person-4").is_some(),
            "{mode}: a second Create counts on from the instance's history"
        );
        assert!(
            row(&session, "crud-person-3") == created,
            "{mode}: the first created row is the same element under the same id"
        );

        session.harness_mut().click("#crud-filter");
        session.harness_mut().type_text("Lovelace");
        assert!(
            row(&session, "crud-person-0").is_none(),
            "{mode}: the filter hides the rows it does not match"
        );
        assert!(
            row(&session, "crud-person-3") == created,
            "{mode}: the created row keeps its id under a filter"
        );
    }
}
