//! A call run on a held instance: checked, run, settled, and answered with a typed outcome.

use blitz_test_harness::{Busy, Harness, Key as KeyboardKey, Modifiers, Rect};
use dioxus_native_dom::{DioxusDocument, SnapshotDiff};

use crate::command::{Call, Command, Key, validate};
use crate::refusal::{Cause, Refusal};
use crate::session::Session;

/// What a call that ran returns: one variant per result shape of the verb table.
///
/// A step that ran and did not go quiet is a result, not a refusal: `settled` reads false,
/// `busy` names the class of work still outstanding, and the step is not rolled back.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// What `snapshot` returns.
    Screen {
        /// The screen as one text: one line per element, nested by indent.
        text: String,
    },
    /// What `click`, `type` and `press` return.
    Acted {
        /// Whether the app went quiet after the step.
        settled: bool,
        /// The class of work still outstanding; `None` when `settled` is true.
        busy: Option<Busy>,
        /// What the screen reads differently after the step: the elements added, the ids
        /// removed and the elements changed.
        diff: SnapshotDiff,
    },
    /// What `advance` returns.
    Advanced {
        /// Whether the app went quiet after the step.
        settled: bool,
        /// The class of work still outstanding; `None` when `settled` is true.
        busy: Option<Busy>,
        /// What the screen reads differently after the step: the elements added, the ids
        /// removed and the elements changed.
        diff: SnapshotDiff,
        /// The time the app actually moved, in milliseconds: never more than was asked.
        advanced_ms: u32,
    },
}

impl Session {
    /// Runs `call` on the held instance and returns what it did.
    ///
    /// The call is checked first ([`validate`]), and a refused call has run nothing. An id is
    /// then looked up among the ids the screen reads now: one that names no element is refused
    /// as [`Cause::NotFound`], with nothing run, and nothing of the lookup outlives the call.
    /// An `advance` on a session with no time step ([`Session::with_time`]) is refused as
    /// [`Cause::TimeUnavailable`].
    ///
    /// An acting verb takes the screen's snapshot, runs its step, settles the instance
    /// ([`Harness::settle`]) and takes the snapshot again; its outcome carries the diff of the
    /// two. `click` clicks the centre of the element's border box; `type` clicks it the same
    /// way, which focuses a text input, and types into what holds focus; `press` presses its
    /// key on what holds focus; `advance` hands its milliseconds to the session's time step.
    /// Settling moves no time and waits on no load.
    pub fn run(&mut self, call: &Call) -> Result<Outcome, Refusal> {
        match validate(call)? {
            Command::Snapshot => Ok(Outcome::Screen {
                text: self.harness.doc.snapshot().to_text(),
            }),
            Command::Click { id } => {
                let (x, y) = centre_of(&self.harness, &id)?;
                Ok(acted(&mut self.harness, |harness| harness.click_at(x, y)))
            }
            Command::Type { id, text } => {
                let (x, y) = centre_of(&self.harness, &id)?;
                Ok(acted(&mut self.harness, |harness| {
                    harness.click_at(x, y);
                    harness.type_text(&text);
                }))
            }
            Command::Press { key, shift } => Ok(acted(&mut self.harness, |harness| {
                press(harness, key, shift)
            })),
            Command::Advance { ms } => {
                let Some(step) = self.time.as_mut() else {
                    return Err(Refusal::new(Cause::TimeUnavailable));
                };
                let mut advanced_ms = 0;
                let (busy, diff) = settled_step(&mut self.harness, |harness| {
                    advanced_ms = step(harness, ms).min(ms);
                });
                Ok(Outcome::Advanced {
                    settled: busy.is_none(),
                    busy,
                    diff,
                    advanced_ms,
                })
            }
        }
    }
}

/// Runs `step` between two snapshots and settles the instance before the second: the class of
/// work still outstanding, if any, and the diff of the two.
fn settled_step(
    harness: &mut Harness<DioxusDocument>,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> (Option<Busy>, SnapshotDiff) {
    let before = harness.doc.snapshot();
    step(harness);
    let busy = harness.settle().err().map(|not_settled| not_settled.busy);
    let after = harness.doc.snapshot();
    (busy, before.diff(&after))
}

fn acted(
    harness: &mut Harness<DioxusDocument>,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> Outcome {
    let (busy, diff) = settled_step(harness, step);
    Outcome::Acted {
        settled: busy.is_none(),
        busy,
        diff,
    }
}

/// The centre of the border box of the element `id` names, in page coordinates: the first
/// element whose stable id is `id`, in document order.
fn centre_of(harness: &Harness<DioxusDocument>, id: &str) -> Result<(f32, f32), Refusal> {
    let not_found = Refusal::new(Cause::NotFound);
    let (node_id, _) = harness
        .doc
        .element_ids()
        .into_iter()
        .find(|(_, listed)| listed == id)
        .ok_or(not_found)?;
    let doc = harness.base();
    let node = doc.get_node(node_id).ok_or(not_found)?;
    let position = node.absolute_position(0.0, 0.0);
    let size = node.final_layout().size;
    let border_box = Rect {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    };
    Ok(border_box.center())
}

fn press(harness: &mut Harness<DioxusDocument>, key: Key, shift: bool) {
    let modifiers = if shift {
        Modifiers::SHIFT
    } else {
        Modifiers::default()
    };
    harness.press_with(keyboard_key(key), modifiers);
    // A window on macOS delivers the backward delete as a standard key binding, and the
    // editor's own Backspace arm is compiled out there.
    #[cfg(target_os = "macos")]
    if key == Key::Backspace {
        harness.apple_keybinding("deleteBackward:");
    }
}

/// The keyboard key a schema key stands for.
fn keyboard_key(key: Key) -> KeyboardKey {
    match key {
        Key::Tab => KeyboardKey::Tab,
        Key::Enter => KeyboardKey::Enter,
        Key::Escape => KeyboardKey::Escape,
        Key::Backspace => KeyboardKey::Backspace,
        Key::Delete => KeyboardKey::Delete,
        Key::Space => KeyboardKey::Character(" ".to_string()),
        Key::ArrowUp => KeyboardKey::ArrowUp,
        Key::ArrowDown => KeyboardKey::ArrowDown,
        Key::ArrowLeft => KeyboardKey::ArrowLeft,
        Key::ArrowRight => KeyboardKey::ArrowRight,
        Key::Home => KeyboardKey::Home,
        Key::End => KeyboardKey::End,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_schema_key_stands_for_one_keyboard_key() {
        let rows = [
            (Key::Tab, KeyboardKey::Tab),
            (Key::Enter, KeyboardKey::Enter),
            (Key::Escape, KeyboardKey::Escape),
            (Key::Backspace, KeyboardKey::Backspace),
            (Key::Delete, KeyboardKey::Delete),
            (Key::Space, KeyboardKey::Character(" ".to_string())),
            (Key::ArrowUp, KeyboardKey::ArrowUp),
            (Key::ArrowDown, KeyboardKey::ArrowDown),
            (Key::ArrowLeft, KeyboardKey::ArrowLeft),
            (Key::ArrowRight, KeyboardKey::ArrowRight),
            (Key::Home, KeyboardKey::Home),
            (Key::End, KeyboardKey::End),
        ];
        assert_eq!(rows.len(), 12);
        assert_eq!(rows.len(), Key::ALL.len());
        for (row, (key, keyboard)) in rows.iter().enumerate() {
            assert!(Key::ALL[row] == *key, "row {row}");
            assert!(keyboard_key(*key) == *keyboard, "row {row}");
            for (other, (_, later)) in rows.iter().enumerate().skip(row + 1) {
                assert!(keyboard != later, "rows {row} and {other}");
            }
        }
    }
}
