//! A call run on a held instance: checked, run, settled, and answered with a typed outcome.

use blitz_test_harness::{Busy, Harness, Key as KeyboardKey, Modifiers};
use dioxus_native_dom::{DioxusDocument, NodeId, Snapshot, SnapshotDiff};
use tracing::Span;
use tracing::field::Empty;

use crate::command::{Call, Command, Key, validate};
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, BUSY_CLASSES};
use crate::session::{SeenIds, Session};

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
    /// What `scroll` returns.
    Scrolled {
        /// Whether the app went quiet after the step.
        settled: bool,
        /// The class of work still outstanding; `None` when `settled` is true.
        busy: Option<Busy>,
        /// What the screen reads differently after the step: the elements added, the ids
        /// removed and the elements changed.
        diff: SnapshotDiff,
        /// Whether the element is in view after the step. False when scrolling could not
        /// bring it there: a `click` or a `type` naming it would still be refused as
        /// [`Cause::OffScreen`].
        in_view: bool,
    },
}

/// States the fields of a command's span once: the span that carries them, and the table of
/// their names the unit test reads.
macro_rules! command_span {
    ($($field:ident),+ $(,)?) => {
        /// The fields of a command's span. Each holds a fixed word of the schema or a count:
        /// none is an argument's name, and none holds what a call supplied or a screen reads.
        #[cfg(test)]
        const SPAN_FIELDS: &[&str] = &[$(stringify!($field)),+];

        /// The span of one call, every field unrecorded: one that does not apply to the call
        /// stays so.
        fn command_span() -> Span {
            tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
        }
    };
}

command_span!(verb, cause, settled, busy, passes, added, removed, changed);

impl Session {
    /// Runs `call` on the held instance and returns what it did.
    ///
    /// The call is checked first ([`validate`]), and a refused call has run nothing. An id is
    /// then looked up on the screen as it reads now. One that names no element there is
    /// refused with nothing run: as [`Cause::Stale`] when an earlier screen of this session
    /// read it, as [`Cause::NotFound`] when none did. A `click` or a `type` naming an element
    /// that cannot take it is refused the same way, with the first of three causes that
    /// holds, in this order: [`Cause::Disabled`] when the element reads not enabled,
    /// [`Cause::OffScreen`] when the point the action would land at is outside the viewport
    /// or outside the visible part of a scrolling box that holds the element, and
    /// [`Cause::Covered`] when another element is hit at that point. An `advance` on a
    /// session with no time step ([`Session::with_time`]) is refused as
    /// [`Cause::TimeUnavailable`].
    ///
    /// An acting verb takes the screen's snapshot, runs its step, settles the instance
    /// ([`Harness::settle`]) and takes the snapshot again; its outcome carries the diff of the
    /// two. `click` clicks the centre of the element's bounds; `type` clicks it the same way,
    /// which focuses a text input, and types into what holds focus — where the element reads
    /// a value, everything it holds is selected first, so the text replaces it, and an empty
    /// text clears it; `press` presses its key
    /// on what holds focus; `advance` hands its milliseconds to the session's time step;
    /// `scroll` scrolls every scrolling box that holds the element, and the viewport, until
    /// the element is in view, whether or not it is enabled or covered, and says whether it
    /// then is. Settling moves no time and waits on no load.
    ///
    /// The call leaves one `tracing` span, `command`, under the target `escher_driver`: the
    /// table's word for its verb, the name of the cause it was refused with, or how its settle
    /// went and how many nodes its diff adds, removes and changes.
    pub fn run(&mut self, call: &Call) -> Result<Outcome, Refusal> {
        let span = command_span();
        let _entered = span.enter();
        if let Some(row) = schema::verb(&call.verb) {
            span.record("verb", row.name);
        }
        let result = self.execute(call, &span);
        record_result(&span, &result);
        result
    }

    fn execute(&mut self, call: &Call, span: &Span) -> Result<Outcome, Refusal> {
        let command = validate(call)?;
        let (harness, seen) = (&mut self.harness, &mut self.seen);
        match command {
            Command::Snapshot => Ok(Outcome::Screen {
                text: read_screen(harness, seen).to_text(),
            }),
            Command::Click { id } => {
                let (before, target) = resolve(harness, seen, &id)?;
                let (x, y) = action_point(harness, &target)?;
                Ok(acted(harness, seen, span, before, |harness| {
                    harness.click_at(x, y)
                }))
            }
            Command::Type { id, text } => {
                let (before, target) = resolve(harness, seen, &id)?;
                let (x, y) = action_point(harness, &target)?;
                let holds_value = target.holds_value;
                Ok(acted(harness, seen, span, before, |harness| {
                    harness.click_at(x, y);
                    if holds_value {
                        select_all(harness);
                        if text.is_empty() {
                            press(harness, Key::Backspace, false);
                        }
                    }
                    harness.type_text(&text);
                }))
            }
            Command::Press { key, shift } => {
                let before = read_screen(harness, seen);
                Ok(acted(harness, seen, span, before, |harness| {
                    press(harness, key, shift)
                }))
            }
            Command::Advance { ms } => {
                let Some(step) = self.time.as_mut() else {
                    return Err(Refusal::new(Cause::TimeUnavailable));
                };
                let before = read_screen(harness, seen);
                let mut advanced_ms = 0;
                let (busy, diff, _) = settled_step(harness, seen, span, before, |harness| {
                    advanced_ms = step(harness, ms).min(ms);
                });
                Ok(Outcome::Advanced {
                    settled: busy.is_none(),
                    busy,
                    diff,
                    advanced_ms,
                })
            }
            Command::Scroll { id } => {
                let (before, target) = resolve(harness, seen, &id)?;
                let (busy, diff, after) = settled_step(harness, seen, span, before, |harness| {
                    harness.scroll_into_view(target.node)
                });
                let in_view =
                    locate(harness, &after, &id).is_some_and(|target| in_view(harness, &target));
                Ok(Outcome::Scrolled {
                    settled: busy.is_none(),
                    busy,
                    diff,
                    in_view,
                })
            }
        }
    }
}

/// Records on a call's span what the call returned: the cause of a refusal, or an acting
/// verb's settle reading and the size of its diff.
fn record_result(span: &Span, result: &Result<Outcome, Refusal>) {
    let (settled, busy, diff) = match result {
        Err(refusal) => {
            span.record("cause", refusal.cause().name());
            return;
        }
        Ok(Outcome::Screen { .. }) => return,
        Ok(
            Outcome::Acted {
                settled,
                busy,
                diff,
            }
            | Outcome::Advanced {
                settled,
                busy,
                diff,
                ..
            }
            | Outcome::Scrolled {
                settled,
                busy,
                diff,
                ..
            },
        ) => (*settled, *busy, diff),
    };
    span.record("settled", settled);
    if let Some(busy) = busy {
        span.record("busy", busy_word(busy));
    }
    span.record("added", diff.added.len());
    span.record("removed", diff.removed.len());
    span.record("changed", diff.changed.len());
}

/// The schema's word for a class of outstanding work.
pub(crate) fn busy_word(busy: Busy) -> &'static str {
    match busy {
        Busy::Render => BUSY_CLASSES[0],
        Busy::Layout => BUSY_CLASSES[1],
        Busy::Loads => BUSY_CLASSES[2],
    }
}

/// An element a call names, as the screen reads it.
struct Target {
    node: NodeId,
    enabled: Option<bool>,
    /// Whether the snapshot reads a value for the element: a control that holds one.
    holds_value: bool,
    /// The centre of the element's bounds, relative to the viewport.
    centre: (f64, f64),
}

/// The screen of the held instance, its ids recorded.
fn read_screen(harness: &Harness<DioxusDocument>, seen: &mut SeenIds) -> Snapshot {
    let screen = harness.doc.snapshot();
    record(seen, &screen);
    screen
}

fn record(seen: &mut SeenIds, screen: &Snapshot) {
    for node in screen.nodes() {
        seen.record(&node.id);
    }
}

/// The element `id` names on `screen`: the node the snapshot lists under the id, where the
/// first element whose stable id is `id`, in document order, still resolves.
fn locate(harness: &Harness<DioxusDocument>, screen: &Snapshot, id: &str) -> Option<Target> {
    let listed = screen.get(id)?;
    let (node, _) = harness
        .doc
        .element_ids()
        .into_iter()
        .find(|(_, read)| read == id)?;
    harness.base().get_node(node)?;
    let bounds = listed.bounds;
    Some(Target {
        node,
        enabled: listed.state.enabled,
        holds_value: listed.state.value.is_some(),
        centre: (
            bounds.x + bounds.width / 2.0,
            bounds.y + bounds.height / 2.0,
        ),
    })
}

/// Reads the screen and finds the element `id` names on it, or refuses: as stale when an
/// earlier screen of the session read the id, as not found when none did. The screen read
/// here is recorded once that answer is decided.
fn resolve(
    harness: &Harness<DioxusDocument>,
    seen: &mut SeenIds,
    id: &str,
) -> Result<(Snapshot, Target), Refusal> {
    let screen = harness.doc.snapshot();
    let read_earlier = seen.holds(id);
    let target = locate(harness, &screen, id);
    record(seen, &screen);
    match target {
        Some(target) => Ok((screen, target)),
        None if read_earlier => Err(Refusal::new(Cause::Stale)),
        None => Err(Refusal::new(Cause::NotFound)),
    }
}

/// The page point a `click` or a `type` on `target` lands at, the centre of its bounds — or
/// the first reason it cannot take the action: not enabled, then out of view, then covered.
fn action_point(harness: &Harness<DioxusDocument>, target: &Target) -> Result<(f32, f32), Refusal> {
    if target.enabled == Some(false) {
        return Err(Refusal::new(Cause::Disabled));
    }
    if !in_view(harness, target) {
        return Err(Refusal::new(Cause::OffScreen));
    }
    let scroll = harness.base().viewport_scroll();
    let page = (
        (target.centre.0 + scroll.x) as f32,
        (target.centre.1 + scroll.y) as f32,
    );
    if covered(harness, target, page) {
        return Err(Refusal::new(Cause::Covered));
    }
    Ok(page)
}

/// Whether the centre of `target` can be seen: inside the viewport and inside the visible
/// part of every scrolling box that holds the element. Read from geometry, never from a hit:
/// a hit reaches an element scrolled out of its box.
fn in_view(harness: &Harness<DioxusDocument>, target: &Target) -> bool {
    let (x, y) = target.centre;
    harness
        .base()
        .visible_region(target.node)
        .is_some_and(|seen| {
            x >= seen.x && x <= seen.x + seen.width && y >= seen.y && y <= seen.y + seen.height
        })
}

/// Whether the element hit at `page` is neither `target` nor inside it. A point over no box
/// is over the root element.
fn covered(harness: &Harness<DioxusDocument>, target: &Target, page: (f32, f32)) -> bool {
    let doc = harness.base();
    let mut hit = match harness.hit(page.0, page.1) {
        Some(hit) => doc.nearest_non_anonymous_ancestor(hit.node_id),
        None => doc.try_root_element().map(|root| root.id),
    };
    while let Some(node) = hit {
        if node == target.node {
            return false;
        }
        hit = doc.get_node(node).and_then(|node| node.parent);
    }
    true
}

/// Runs `step` after the snapshot `before` and settles the instance before the next one: the
/// class of work still outstanding, if any, the diff of the two, and the snapshot after. The
/// passes a settle that went quiet took are recorded on `span`.
fn settled_step(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    before: Snapshot,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> (Option<Busy>, SnapshotDiff, Snapshot) {
    step(harness);
    let busy = match harness.settle() {
        Ok(settled) => {
            span.record("passes", settled.passes);
            None
        }
        Err(not_settled) => Some(not_settled.busy),
    };
    let after = read_screen(harness, seen);
    (busy, before.diff(&after), after)
}

fn acted(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    before: Snapshot,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> Outcome {
    let (busy, diff, _) = settled_step(harness, seen, span, before, step);
    Outcome::Acted {
        settled: busy.is_none(),
        busy,
        diff,
    }
}

/// The modifier the engine's editor reads as its action modifier: Super on macOS, Control
/// elsewhere.
#[cfg(target_os = "macos")]
const ACTION_MODIFIER: Modifiers = Modifiers::SUPER;
#[cfg(not(target_os = "macos"))]
const ACTION_MODIFIER: Modifiers = Modifiers::CONTROL;

/// Selects everything the focused control holds, through the editor's own select-all key:
/// what is typed next replaces the selection.
fn select_all(harness: &mut Harness<DioxusDocument>) {
    harness.press_with(KeyboardKey::Character("a".to_string()), ACTION_MODIFIER);
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
    use crate::schema::VERBS;

    #[test]
    fn the_span_has_eight_fields_and_none_is_an_argument_name() {
        let rows = [
            "verb", "cause", "settled", "busy", "passes", "added", "removed", "changed",
        ];
        assert_eq!(rows.len(), 8);
        assert_eq!(SPAN_FIELDS, rows);
        let arguments: Vec<&str> = VERBS
            .iter()
            .flat_map(|verb| verb.args)
            .map(|argument| argument.name)
            .collect();
        assert_eq!(arguments.len(), 8);
        for (row, field) in SPAN_FIELDS.iter().enumerate() {
            assert!(!arguments.contains(field), "row {row}");
        }
    }

    #[test]
    fn each_busy_class_reads_as_its_schema_word() {
        let rows = [
            (Busy::Render, "render"),
            (Busy::Layout, "layout"),
            (Busy::Loads, "loads"),
        ];
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.map(|(_, word)| word), BUSY_CLASSES);
        for (row, (busy, word)) in rows.iter().enumerate() {
            assert!(busy_word(*busy) == *word, "row {row}");
        }
    }

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
