//! A call run on a held instance: checked, run, settled, and answered with a typed outcome.

use std::time::Duration;

use blitz_test_harness::{
    Busy, Harness, Key as KeyboardKey, Modifiers, NamedKey, PixelRect, ScreenshotError,
};
use dioxus_native_dom::{
    DioxusDocument, NodeId, SNAPSHOT_TEXT_BUDGET, Snapshot, SnapshotDiff, is_masked_input,
    is_multiple_select,
};
use tracing::Span;
use tracing::field::Empty;

use crate::command::{Call, Command, Key, validate};
use crate::image;
use crate::lint::Finding;
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, BUSY_CLASSES};
use crate::session::{SeenIds, Session, settle_waiting};

/// What a call that ran returns: one variant per result shape of the verb table.
///
/// A step that ran and did not go quiet is a result, not a refusal: `settled` reads false,
/// `busy` names the class of work still outstanding, and the step is not rolled back.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// What `snapshot` returns for a screen read whole.
    Screen {
        /// The screen as one text: one line per element, nested by indent.
        text: String,
    },
    /// What `snapshot` returns when the screen is read in parts: a `part` asked for, or a
    /// screen whose text is over [`SNAPSHOT_TEXT_BUDGET`].
    Part {
        /// The part's lines, as they read in the whole text; empty for a part past the last.
        text: String,
        /// The part this answer holds, counted from 1.
        part: u32,
        /// How many parts the screen reads as.
        parts: u32,
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
    /// What `screenshot` returns.
    Shot {
        /// The image's width, in pixels.
        width: u32,
        /// The image's height, in pixels.
        height: u32,
        /// The image: a PNG of 8-bit RGBA pixels as painted.
        png: Vec<u8>,
    },
    /// What `lint` returns.
    Linted {
        /// Every defect found on the screen, as [`Session::lint`] reads it.
        findings: Vec<Finding>,
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
    /// [`Cause::Covered`] when another element is hit at that point.
    ///
    /// `screenshot` runs no step and settles nothing, as `snapshot` does: it paints the screen
    /// as it stands, the whole of it, or the element an id names, cropped to the part of its
    /// bounds that can be seen, rounded outward to whole pixels. An element out of view is
    /// refused as [`Cause::OffScreen`]; one that is disabled or covered is not refused, since
    /// the image shows what is painted there. An app with no font it can paint text with is
    /// refused as [`Cause::FontsUnavailable`], with nothing painted.
    ///
    /// `snapshot` answers the screen whole when no `part` is asked and its text is at most
    /// [`SNAPSHOT_TEXT_BUDGET`] bytes; otherwise it cuts the screen as it stands into parts of
    /// whole lines within the budget ([`Snapshot::text_parts`]) and answers the one asked, part 1
    /// without a `part`, or the empty text for a part past the last. Either way every id of the
    /// whole screen is recorded.
    ///
    /// `lint` runs no step either: it answers [`Session::lint`]'s findings on the screen as it
    /// stands, or its refusal.
    ///
    /// An acting verb takes the screen's snapshot, runs its step, settles the instance
    /// ([`Harness::settle`]) and takes the snapshot again; its outcome carries the diff of the
    /// two. `click` clicks the centre of the element's bounds; `type` clicks it the same way,
    /// which focuses a text input, and types into what holds focus — where the element reads
    /// a value, everything it holds is selected first, so the text replaces it, and an empty
    /// text clears it; `press` presses its key
    /// on what holds focus; `advance` moves the held instance's clock by its milliseconds —
    /// the clock every time the engine reads takes — then hands them to the session's time
    /// step, where one was handed ([`Session::with_time`]);
    /// `scroll` scrolls every scrolling box that holds the element, and the viewport, until
    /// the element is in view, whether or not it is enabled or covered, and says whether it
    /// then is. Settling moves no time; a load in flight is waited for up to
    /// [`LOAD_WAIT`](crate::LOAD_WAIT).
    ///
    /// The call leaves one `tracing` span, `command`, under the target `escher_driver`: the
    /// table's word for its verb, the name of the cause it was refused with, or how its settle
    /// went and how many nodes its diff adds, removes and changes.
    ///
    /// A session recording into a directory ([`Session::recording_into`]) keeps the call, refused
    /// or not, once it is answered. A `type` keeps its text only when the element it names
    /// resolves to a control that is not masked.
    pub fn run(&mut self, call: &Call) -> Result<Outcome, Refusal> {
        let span = command_span();
        let _entered = span.enter();
        if let Some(row) = schema::verb(&call.verb) {
            span.record("verb", row.name);
        }
        let mut masked = false;
        let result = self.execute(call, &span, &mut masked);
        record_result(&span, &result);
        if let Some(recording) = &mut self.recording
            && !recording.record(call, masked, &result)
        {
            self.recording = None;
            self.recording_failed = true;
        }
        result
    }

    /// Runs `call`. `masked` is set when the call is a `type` whose text a recording must not
    /// keep: its element resolves to a masked control, or resolves to none, so its control
    /// cannot be read.
    fn execute(&mut self, call: &Call, span: &Span, masked: &mut bool) -> Result<Outcome, Refusal> {
        let command = validate(call)?;
        let (harness, seen) = (&mut self.harness, &mut self.seen);
        match command {
            Command::Snapshot { part } => {
                let screen = read_screen(harness, seen);
                let text = screen.to_text();
                if part.is_none() && text.len() <= SNAPSHOT_TEXT_BUDGET {
                    return Ok(Outcome::Screen { text });
                }
                let mut cut = screen.text_parts(SNAPSHOT_TEXT_BUDGET);
                let part = part.unwrap_or(1);
                let parts = u32::try_from(cut.len()).unwrap_or(u32::MAX);
                let text = part
                    .checked_sub(1)
                    .and_then(|index| usize::try_from(index).ok())
                    .filter(|index| *index < cut.len())
                    .map(|index| cut.swap_remove(index).text)
                    .unwrap_or_default();
                Ok(Outcome::Part { text, part, parts })
            }
            Command::Click { id } => {
                let (before, target) = resolve(harness, seen, &id)?;
                let (x, y) = action_point(harness, &target)?;
                Ok(acted(harness, seen, span, before, |harness| {
                    harness.click_at(x, y)
                }))
            }
            Command::Type { id, text } => {
                let resolved = resolve(harness, seen, &id);
                *masked = resolved
                    .as_ref()
                    .ok()
                    .is_none_or(|(_, target)| target.masked);
                let (before, target) = resolved?;
                let (x, y) = action_point(harness, &target)?;
                let holds_value = target.holds_value;
                let (node, multiple) = (target.node, target.multiple);
                Ok(acted(harness, seen, span, before, |harness| {
                    if multiple {
                        focus_multiple_select(harness, node);
                    } else {
                        harness.click_at(x, y);
                    }
                    if holds_value {
                        select_all(harness);
                        if text.is_empty() {
                            press(harness, Key::Backspace, false);
                        }
                    }
                    // No label starts with a space, and on a `multiple` select a Space with no
                    // text before it flips a row.
                    if !(multiple && text.starts_with(' ')) {
                        harness.type_text(&text);
                    }
                }))
            }
            Command::Press { key, shift } => {
                let before = read_screen(harness, seen);
                Ok(acted(harness, seen, span, before, |harness| {
                    press(harness, key, shift)
                }))
            }
            Command::Advance { ms } => {
                let step = &mut self.time;
                let before = read_screen(harness, seen);
                let (busy, diff, _) = settled_step(harness, seen, span, before, |harness| {
                    harness.advance_time(Duration::from_millis(u64::from(ms)));
                    if let Some(step) = step.as_mut() {
                        step(harness, ms);
                    }
                });
                Ok(Outcome::Advanced {
                    settled: busy.is_none(),
                    busy,
                    diff,
                    advanced_ms: ms,
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
            Command::Screenshot { id } => {
                let shot = match id {
                    None => harness.screenshot(),
                    Some(id) => {
                        let (_, target) = resolve(harness, seen, &id)?;
                        if !in_view(harness, &target) {
                            return Err(Refusal::new(Cause::OffScreen));
                        }
                        let rect = crop(harness, &target).ok_or(Refusal::new(Cause::OffScreen))?;
                        harness.screenshot_rect(rect)
                    }
                };
                let raster = shot.map_err(|error| match error {
                    ScreenshotError::FontsUnavailable => Refusal::new(Cause::FontsUnavailable),
                    ScreenshotError::EmptyRect => Refusal::new(Cause::OffScreen),
                })?;
                // A raster always fills its own non-empty size, and the encoder writes to
                // memory, so the empty fallback is never taken.
                let png = image::png(raster.width, raster.height, &raster.rgba).unwrap_or_default();
                Ok(Outcome::Shot {
                    width: raster.width,
                    height: raster.height,
                    png,
                })
            }
            Command::Lint => Ok(Outcome::Linted {
                findings: self.lint()?,
            }),
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
        Ok(
            Outcome::Screen { .. }
            | Outcome::Part { .. }
            | Outcome::Shot { .. }
            | Outcome::Linted { .. },
        ) => return,
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
    /// Whether the element is a masked control ([`is_masked_input`]).
    masked: bool,
    /// Whether the element is a `multiple` select ([`is_multiple_select`]), whose centre lies on
    /// one of its rows.
    multiple: bool,
    /// The centre of the element's bounds, relative to the viewport.
    centre: (f64, f64),
    /// The element's bounds as the snapshot reads them: x, y, width and height, relative to
    /// the viewport.
    bounds: (f64, f64, f64, f64),
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
    let doc = harness.base();
    let element = doc.get_node(node)?.element_data();
    let masked = element.is_some_and(is_masked_input);
    let multiple = element.is_some_and(is_multiple_select);
    let bounds = listed.bounds;
    Some(Target {
        node,
        enabled: listed.state.enabled,
        holds_value: listed.state.value.is_some(),
        masked,
        multiple,
        centre: (
            bounds.x + bounds.width / 2.0,
            bounds.y + bounds.height / 2.0,
        ),
        bounds: (bounds.x, bounds.y, bounds.width, bounds.height),
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
/// a hit at a point scrolled out of view answers what shows there, which reads as covered.
fn in_view(harness: &Harness<DioxusDocument>, target: &Target) -> bool {
    let (x, y) = target.centre;
    harness
        .base()
        .visible_region(target.node)
        .is_some_and(|seen| {
            x >= seen.x && x <= seen.x + seen.width && y >= seen.y && y <= seen.y + seen.height
        })
}

/// The device pixels of `target` that can be seen: its bounds within the visible part of
/// every scrolling box that holds it, at the viewport's scale, rounded outward — the near
/// edges down, the far edges up. `None` when no pixel is left.
fn crop(harness: &Harness<DioxusDocument>, target: &Target) -> Option<PixelRect> {
    let doc = harness.base();
    let seen = doc.visible_region(target.node)?;
    let scale = doc.viewport().scale_f64();
    let (x, y, width, height) = target.bounds;
    let left = (x.max(seen.x) * scale).floor().max(0.0);
    let top = (y.max(seen.y) * scale).floor().max(0.0);
    let right = ((x + width).min(seen.x + seen.width) * scale).ceil();
    let bottom = ((y + height).min(seen.y + seen.height) * scale).ceil();
    if right <= left || bottom <= top {
        return None;
    }
    Some(PixelRect {
        x: left as u32,
        y: top as u32,
        width: (right - left) as u32,
        height: (bottom - top) as u32,
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
    let busy = match settle_waiting(harness) {
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
const ACTION_MODIFIER: Modifiers = Modifiers::META;
#[cfg(not(target_os = "macos"))]
const ACTION_MODIFIER: Modifiers = Modifiers::CONTROL;

/// Focuses a `multiple` select with no click, which would land on the row at its centre, its
/// typed text starting afresh as a click on a select's would.
fn focus_multiple_select(harness: &mut Harness<DioxusDocument>, select: NodeId) {
    let mut doc = harness.base_mut();
    doc.clear_select_typeahead();
    doc.set_focus_to(select);
}

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
        Key::Tab => KeyboardKey::Named(NamedKey::Tab),
        Key::Enter => KeyboardKey::Named(NamedKey::Enter),
        Key::Escape => KeyboardKey::Named(NamedKey::Escape),
        Key::Backspace => KeyboardKey::Named(NamedKey::Backspace),
        Key::Delete => KeyboardKey::Named(NamedKey::Delete),
        Key::Space => KeyboardKey::Character(" ".to_string()),
        Key::ArrowUp => KeyboardKey::Named(NamedKey::ArrowUp),
        Key::ArrowDown => KeyboardKey::Named(NamedKey::ArrowDown),
        Key::ArrowLeft => KeyboardKey::Named(NamedKey::ArrowLeft),
        Key::ArrowRight => KeyboardKey::Named(NamedKey::ArrowRight),
        Key::Home => KeyboardKey::Named(NamedKey::Home),
        Key::End => KeyboardKey::Named(NamedKey::End),
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
        assert_eq!(arguments.len(), 10);
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
            (Key::Tab, KeyboardKey::Named(NamedKey::Tab)),
            (Key::Enter, KeyboardKey::Named(NamedKey::Enter)),
            (Key::Escape, KeyboardKey::Named(NamedKey::Escape)),
            (Key::Backspace, KeyboardKey::Named(NamedKey::Backspace)),
            (Key::Delete, KeyboardKey::Named(NamedKey::Delete)),
            (Key::Space, KeyboardKey::Character(" ".to_string())),
            (Key::ArrowUp, KeyboardKey::Named(NamedKey::ArrowUp)),
            (Key::ArrowDown, KeyboardKey::Named(NamedKey::ArrowDown)),
            (Key::ArrowLeft, KeyboardKey::Named(NamedKey::ArrowLeft)),
            (Key::ArrowRight, KeyboardKey::Named(NamedKey::ArrowRight)),
            (Key::Home, KeyboardKey::Named(NamedKey::Home)),
            (Key::End, KeyboardKey::Named(NamedKey::End)),
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
