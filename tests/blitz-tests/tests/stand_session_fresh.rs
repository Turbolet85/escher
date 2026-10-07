//! A held stand instance is the stand's own boot, in both layout modes: before any command its
//! snapshot text equals a fresh boot's, its title measures non-empty under the bundled font and
//! its controls read their roles and names; after commands the viewport still reads the stand's
//! pin and the mount is still TaskShell, with no Home card. The snapshot text is content: this
//! file binds it only to `text`-named variables and never formats one into a message.

use escher_driver::Session;
use keyboard_types::Key;
use seven_guis::stand::{COLOR_SCHEME, HIDPI_SCALE, LeanTask, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

mod common;
mod session_common;
use common::{INPUT_NAMES, boot, controls};
use session_common::{change, hold};

/// The ids of the TaskShell chrome every lean task mounts inside.
const SHELL: [&str; 5] = [
    "task-shell",
    "task-header",
    "back-btn",
    "task-title",
    "task-body",
];

/// Whether the held instance is mounted in TaskShell: every chrome id resolves, and no element
/// reads a Home card's id.
fn mounted_in_task_shell(session: &Session) -> bool {
    let harness = session.harness();
    SHELL
        .iter()
        .all(|id| harness.query(&format!("#{id}")).is_some())
        && harness
            .doc
            .element_ids()
            .iter()
            .all(|(_, id)| !id.starts_with("task-card-"))
}

#[test]
fn a_held_instance_reads_as_a_fresh_boot() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let mode = format!("incremental={incremental}: {task:?}");
            let (session, _ticks) = hold(task, incremental);
            let held = session.harness().doc.snapshot();

            let text_held = held.to_text();
            let text_fresh = boot(task, incremental).doc.snapshot().to_text();
            assert!(
                !text_held.is_empty(),
                "{mode}: the held instance has a screen"
            );
            assert!(
                text_held == text_fresh,
                "{mode}: a held instance reads as a fresh boot"
            );

            assert!(
                held.get("task-title").is_some_and(|title| {
                    !title.name.is_empty() && title.bounds.width > 0.0 && title.bounds.height > 0.0
                }),
                "{mode}: the title's text measures non-empty under the bundled font"
            );
            for (id, role) in controls(task) {
                assert!(
                    held.get(id)
                        .is_some_and(|control| control.role == *role && !control.name.is_empty()),
                    "{mode}: every control reads its role and a name"
                );
            }
            let inputs = INPUT_NAMES
                .iter()
                .filter(|(id, _)| controls(task).iter().any(|(control, _)| control == id));
            for (id, name) in inputs {
                assert!(
                    held.get(id).is_some_and(|input| input.name == *name),
                    "{mode}: every input reads the name its markup gives it"
                );
            }
        }
    }
}

#[test]
fn the_mount_and_the_viewport_hold_after_commands() {
    for task in LeanTask::ALL {
        for incremental in [false, true] {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, ticks) = hold(task, incremental);
            assert!(
                mounted_in_task_shell(&session),
                "{mode}: a started session is mounted in TaskShell"
            );
            let screen = session.harness().doc.snapshot();

            change(task, &mut session, &ticks);
            session.harness_mut().press(Key::Tab);
            change(task, &mut session, &ticks);

            assert!(
                session.harness().doc.snapshot() != screen,
                "{mode}: the commands changed what the instance shows"
            );
            let viewport = session.harness().base().get_viewport();
            assert!(
                viewport.window_size == (VIEWPORT_WIDTH, VIEWPORT_HEIGHT)
                    && viewport.hidpi_scale == HIDPI_SCALE
                    && viewport.color_scheme == COLOR_SCHEME,
                "{mode}: the viewport reads the stand's pin after commands"
            );
            assert!(
                mounted_in_task_shell(&session),
                "{mode}: the mount is still TaskShell after commands"
            );
        }
    }
}
