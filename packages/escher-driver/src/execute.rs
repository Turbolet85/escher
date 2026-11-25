//! A call run on a held instance: checked, run, settled, and answered with a typed outcome.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::Duration;

use blitz_test_harness::{
    Busy, Harness, Key as KeyboardKey, Modifiers, NamedKey, NotSettled, PixelRect, ScreenshotError,
};
use dioxus_native_dom::{
    DioxusDocument, NodeId, SNAPSHOT_TEXT_BUDGET, Snapshot, SnapshotDiff, SnapshotNode, UiGraph,
    VIRTUAL_KEY_SEPARATOR, is_masked_input, is_select,
};
use tracing::Span;
use tracing::field::Empty;

use crate::actions::ActionNode;
use crate::canvas::{CanvasCommand, CanvasReading};
use crate::caused::{Caused, CausedPast, StepReading, caused};
use crate::command::{Call, Command, Key, validate};
use crate::image;
use crate::json;
use crate::layout::LayoutExplanation;
use crate::lint::Finding;
use crate::refusal::{Cause, Refusal};
use crate::schema::{self, BUSY_CLASSES, VerbSpec};
use crate::session::{SeenIds, Session, TimeStep, settle_waiting};

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
        /// What the step caused, one item per panic, error and record, each tied to the first
        /// handler it reached, at most [`CAUSED_KEPT`](crate::CAUSED_KEPT). Empty when it caused
        /// nothing.
        caused: Vec<Caused>,
        /// The items past [`CAUSED_KEPT`](crate::CAUSED_KEPT), counted by kind and source.
        caused_past: Vec<CausedPast>,
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
        /// What the step caused, one item per panic, error and record, each tied to the first
        /// handler it reached, at most [`CAUSED_KEPT`](crate::CAUSED_KEPT). Empty when it caused
        /// nothing.
        caused: Vec<Caused>,
        /// The items past [`CAUSED_KEPT`](crate::CAUSED_KEPT), counted by kind and source.
        caused_past: Vec<CausedPast>,
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
        /// What the step caused, one item per panic, error and record, each tied to the first
        /// handler it reached, at most [`CAUSED_KEPT`](crate::CAUSED_KEPT). Empty when it caused
        /// nothing.
        caused: Vec<Caused>,
        /// The items past [`CAUSED_KEPT`](crate::CAUSED_KEPT), counted by kind and source.
        caused_past: Vec<CausedPast>,
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
    /// What `graph` returns for a graph read whole.
    Graphed {
        /// The screen's UI graph, as [`DioxusDocument::graph`] reads it.
        graph: UiGraph,
    },
    /// What `graph` returns when the graph is read in parts: a `part` asked for, or a graph whose
    /// answer is over [`SNAPSHOT_TEXT_BUDGET`].
    GraphPart {
        /// The part's nodes, in the whole graph's order, and in part 1 alone every state; both
        /// empty for a part past the last.
        graph: UiGraph,
        /// The part this answer holds, counted from 1.
        part: u32,
        /// How many parts the graph reads as.
        parts: u32,
    },
    /// What `actions` returns.
    Actions {
        /// Every actionable element of the screen with what a call naming it would meet now,
        /// as [`Session::actions`](crate::Session::actions) reads it.
        nodes: Vec<ActionNode>,
    },
    /// What `layout` returns.
    Layout {
        /// Why the element sits where it does at its size, as
        /// [`Session::layout`](crate::Session::layout) reads it.
        explanation: LayoutExplanation,
    },
    /// What `canvas` returns without `full` or `part`: a summary of the commands
    /// [`Session::canvas`](crate::Session::canvas) reads.
    CanvasSummary {
        /// Whether the element holds a widget whose content box is not empty.
        canvas: bool,
        /// The content box's width, in CSS px; 0 where `canvas` is false.
        width: f64,
        /// The content box's height, in CSS px; 0 where `canvas` is false.
        height: f64,
        /// How many commands of each kind, in the order of
        /// [`COMMAND_KINDS`](crate::COMMAND_KINDS): layer, clip, mask, pop, fill, stroke, glyphs
        /// and shadow.
        kinds: [usize; 8],
        /// The deepest nesting of layers, clips and masks.
        depth: usize,
        /// The FNV-1a 64 digest of the whole full dump's `text`, as `full` writes it unpaged.
        digest: u64,
    },
    /// What `canvas` returns with `full` for a dump read whole.
    CanvasDump {
        /// The commands, as [`Session::canvas`](crate::Session::canvas) reads them.
        reading: CanvasReading,
    },
    /// What `canvas` returns when the full dump is read in parts: a `part` asked for, or a dump
    /// whose answer is over [`SNAPSHOT_TEXT_BUDGET`].
    CanvasPart {
        /// The part's commands, in the whole dump's order, and in part 1 alone every font and
        /// image; all empty for a part past the last.
        reading: CanvasReading,
        /// The layers, clips and masks open where the part's first command stands, which its
        /// lines are indented from.
        depth: usize,
        /// The part this answer holds, counted from 1.
        part: u32,
        /// How many parts the dump reads as.
        parts: u32,
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
    /// stands, or its refusal. Nor does `graph`, which answers the screen's UI graph
    /// ([`DioxusDocument::graph`]) as it stands, recording every id it reads — whole when no
    /// `part` is asked and its answer is at most [`SNAPSHOT_TEXT_BUDGET`] bytes, otherwise cut,
    /// as `snapshot` cuts its text, into parts of whole nodes within the budget, the states in
    /// part 1 alone — nor `actions`,
    /// which answers [`Session::actions`](crate::Session::actions): each actionable element
    /// with the cause the one check below would refuse each of its verbs with now, nor `layout`,
    /// which answers [`Session::layout`](crate::Session::layout) for the element an id names, or
    /// its refusal.
    ///
    /// An acting verb takes the screen's snapshot, runs its step, settles the instance
    /// ([`Harness::settle`]) and takes the snapshot again; its outcome carries the diff of the
    /// two. `click` clicks the centre of the element's bounds, which opens a closed select's list
    /// and chooses an open list's row; `type` clicks it the same way, which focuses a text input,
    /// focuses a select with no click, and types into what holds focus — where the element reads
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
            Command::Snapshot { part } => Ok(snapshot(harness, seen, part)),
            Command::Click { ref id } => click(harness, seen, span, &command, id),
            Command::Type { ref id, ref text } => {
                type_into(harness, seen, span, &command, id, text, masked)
            }
            Command::Press { key, shift, ctrl } => {
                Ok(press_key(harness, seen, span, key, shift, ctrl))
            }
            Command::Advance { ms } => Ok(advance(harness, seen, &mut self.time, span, ms)),
            Command::Scroll { ref id } => scroll(harness, seen, span, &command, id),
            Command::Screenshot { ref id } => screenshot(harness, seen, &command, id.as_deref()),
            Command::Lint => Ok(Outcome::Linted {
                findings: self.lint()?,
            }),
            Command::Graph { part } => Ok(graph(harness, seen, part)),
            Command::Actions => Ok(Outcome::Actions {
                nodes: self.actions(),
            }),
            Command::Layout { ref id } => Ok(Outcome::Layout {
                explanation: self.layout(id)?,
            }),
            Command::Canvas { ref id, full, part } => Ok(canvas(self.canvas(id)?, full, part)),
        }
    }
}

/// `snapshot`: the screen whole, or the `part` asked of it — part 1 without one — when one is
/// asked or the screen's text is over the budget.
fn snapshot(harness: &Harness<DioxusDocument>, seen: &mut SeenIds, part: Option<u32>) -> Outcome {
    let screen = read_screen(harness, seen);
    let text = screen.to_text();
    if part.is_none() && text.len() <= SNAPSHOT_TEXT_BUDGET {
        return Outcome::Screen { text };
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
    Outcome::Part { text, part, parts }
}

/// `click`: the centre of the element `id` names, once its check clears it.
fn click(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    command: &Command,
    id: &str,
) -> Result<Outcome, Refusal> {
    let (before, target) = resolve_for(harness, seen, command.spec(), id)?;
    let (x, y) = check_target(harness, command, &target)?.point();
    Ok(acted(harness, seen, span, before, |harness| {
        harness.click_at(x, y)
    }))
}

/// `type`: `text` into the element `id` names, once its check clears it. `masked` is set before
/// any refusal, from what the id resolves to.
fn type_into(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    command: &Command,
    id: &str,
    text: &str,
    masked: &mut bool,
) -> Result<Outcome, Refusal> {
    let resolved = resolve(harness, seen, id);
    *masked = resolved
        .as_ref()
        .ok()
        .is_none_or(|(_, target)| target.masked);
    let (before, target) = resolved?;
    let point = check_target(harness, command, &target)?.point();
    Ok(acted(harness, seen, span, before, |harness| {
        type_step(harness, &target, point, text)
    }))
}

/// The step of a `type`: the select focused or the point clicked, everything a control holds
/// selected — and cleared for an empty text — then the text typed.
fn type_step(
    harness: &mut Harness<DioxusDocument>,
    target: &Target,
    (x, y): (f32, f32),
    text: &str,
) {
    if target.select {
        focus_select(harness, target.node);
    } else {
        harness.click_at(x, y);
    }
    if target.holds_value {
        select_all(harness);
        if text.is_empty() {
            press(harness, Key::Backspace, false, false);
        }
    }
    // No label starts with a space, and on a select a Space with no text before it
    // flips a row or opens the list.
    if !(target.select && text.starts_with(' ')) {
        harness.type_text(text);
    }
}

/// `press`: `key` on what holds focus.
fn press_key(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    key: Key,
    shift: bool,
    ctrl: bool,
) -> Outcome {
    let before = read_screen(harness, seen);
    acted(harness, seen, span, before, |harness| {
        press(harness, key, shift, ctrl)
    })
}

/// `advance`: the held instance's clock moved by `ms`, then the session's time step handed them.
fn advance(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    step: &mut Option<TimeStep>,
    span: &Span,
    ms: u32,
) -> Outcome {
    let before = read_screen(harness, seen);
    let (busy, diff, _, (caused, caused_past)) =
        settled_step(harness, seen, span, before, |harness| {
            harness.advance_time(Duration::from_millis(u64::from(ms)));
            if let Some(step) = step.as_mut() {
                step(harness, ms);
            }
        });
    Outcome::Advanced {
        settled: busy.is_none(),
        busy,
        diff,
        advanced_ms: ms,
        caused,
        caused_past,
    }
}

/// `scroll`: the element `id` names brought into view, and whether it then is.
fn scroll(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    command: &Command,
    id: &str,
) -> Result<Outcome, Refusal> {
    let (before, target) = resolve(harness, seen, id)?;
    check_target(harness, command, &target)?;
    let (busy, diff, after, (caused, caused_past)) =
        settled_step(harness, seen, span, before, |harness| {
            harness.scroll_into_view(target.node)
        });
    let in_view = locate(harness, &after, id).is_some_and(|target| in_view(harness, &target));
    Ok(Outcome::Scrolled {
        settled: busy.is_none(),
        busy,
        diff,
        in_view,
        caused,
        caused_past,
    })
}

/// `screenshot`: the whole screen, or the visible part of the element `id` names once its
/// check clears it, as a PNG.
fn screenshot(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    command: &Command,
    id: Option<&str>,
) -> Result<Outcome, Refusal> {
    let shot = match id {
        None => harness.screenshot(),
        Some(id) => {
            let (_, target) = resolve(harness, seen, id)?;
            match check_target(harness, command, &target)? {
                Cleared::Rect(rect) => harness.screenshot_rect(rect),
                Cleared::Point(..) | Cleared::Free => harness.screenshot(),
            }
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

/// `graph`: the screen's UI graph whole, or the `part` asked of it — part 1 without one — when
/// one is asked or its answer is over the budget; every id it reads recorded either way.
fn graph(harness: &Harness<DioxusDocument>, seen: &mut SeenIds, part: Option<u32>) -> Outcome {
    let mut graph = harness.doc.graph();
    for node in &graph.nodes {
        seen.record(&node.id);
    }
    let (nodes, states) = json::graph_lengths(&graph);
    let whole = GRAPH_WHOLE_FRAME + joined(&nodes) + states;
    if part.is_none() && whole <= SNAPSHOT_TEXT_BUDGET {
        return Outcome::Graphed { graph };
    }
    let cut = graph_parts(&nodes, states);
    let part = part.unwrap_or(1);
    let parts = u32::try_from(cut.len()).unwrap_or(u32::MAX);
    let range = part
        .checked_sub(1)
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| cut.get(index).cloned());
    let (nodes, states) = match range {
        Some(range) => {
            let states = if range.start == 0 {
                std::mem::take(&mut graph.states)
            } else {
                Vec::new()
            };
            (graph.nodes.drain(range).collect(), states)
        }
        None => (Vec::new(), Vec::new()),
    };
    Outcome::GraphPart {
        graph: UiGraph { nodes, states },
        part,
        parts,
    }
}

/// The bytes a whole graph's answer takes beyond its nodes and states: `{"nodes":[`, `],
/// "states":[` and `]}`.
const GRAPH_WHOLE_FRAME: usize = 24;

/// The most bytes a graph part's answer takes beyond its nodes and states: the whole answer's
/// frame and `,"part":` and `,"parts":`, each count at its widest `u32`.
const GRAPH_PART_FRAME: usize = GRAPH_WHOLE_FRAME + 8 + 9 + 2 * 10;

/// The written length of items joined by commas.
fn joined(lengths: &[usize]) -> usize {
    lengths.iter().sum::<usize>() + lengths.len().saturating_sub(1)
}

/// The graph cut into parts of whole nodes, in order: each part's answer at most
/// [`SNAPSHOT_TEXT_BUDGET`] bytes, the first carrying the states, a node that cannot fit with
/// its part's other content a part alone. A graph with no node is one part.
fn graph_parts(nodes: &[usize], states: usize) -> Vec<std::ops::Range<usize>> {
    let mut cut = Vec::new();
    let mut start = 0;
    let mut size = GRAPH_PART_FRAME + states;
    for (index, length) in nodes.iter().enumerate() {
        let comma = usize::from(index > start);
        if index > start && size + comma + length > SNAPSHOT_TEXT_BUDGET {
            cut.push(start..index);
            start = index;
            size = GRAPH_PART_FRAME + length;
        } else {
            size += comma + length;
        }
    }
    cut.push(start..nodes.len());
    cut
}

/// `canvas`: the summary without `full` or `part`; with `full` the whole dump when its answer is
/// within the budget, else part 1; with a `part` that part of the full dump.
fn canvas(mut reading: CanvasReading, full: bool, part: Option<u32>) -> Outcome {
    if !full && part.is_none() {
        let mut kinds = [0; 8];
        for command in &reading.commands {
            kinds[command.kind()] += 1;
        }
        return Outcome::CanvasSummary {
            canvas: reading.canvas,
            width: reading.width,
            height: reading.height,
            kinds,
            depth: deepest(&reading.commands),
            digest: crate::recording::fnv1a(
                json::canvas_text(&reading.commands, &reading.nodes, 0).as_bytes(),
            ),
        };
    }
    let lines = json::canvas_line_lengths(&reading.commands, &reading.nodes);
    let resources = json::canvas_resource_lengths(&reading);
    let whole = CANVAS_WHOLE_FRAME + lines.iter().sum::<usize>() + resources;
    if part.is_none() && whole <= SNAPSHOT_TEXT_BUDGET {
        return Outcome::CanvasDump { reading };
    }
    let cut = canvas_parts(&lines, resources);
    let part = part.unwrap_or(1);
    let parts = u32::try_from(cut.len()).unwrap_or(u32::MAX);
    let range = part
        .checked_sub(1)
        .and_then(|index| usize::try_from(index).ok())
        .and_then(|index| cut.get(index).cloned());
    let (depth, commands, nodes, fonts, images) = match range {
        Some(range) => {
            let nodes = reading
                .nodes
                .get(range.clone())
                .map(<[Option<usize>]>::to_vec)
                .unwrap_or_default();
            let depth = open_at(&reading.commands[..range.start]);
            let (fonts, images) = if range.start == 0 {
                (
                    std::mem::take(&mut reading.fonts),
                    std::mem::take(&mut reading.images),
                )
            } else {
                (Vec::new(), Vec::new())
            };
            (
                depth,
                reading.commands.drain(range).collect(),
                nodes,
                fonts,
                images,
            )
        }
        None => (0, Vec::new(), Vec::new(), Vec::new(), Vec::new()),
    };
    Outcome::CanvasPart {
        reading: CanvasReading {
            commands,
            nodes,
            fonts,
            images,
            ..reading
        },
        depth,
        part,
        parts,
    }
}

/// The layers, clips and masks left open after `commands`.
fn open_at(commands: &[CanvasCommand]) -> usize {
    commands.iter().fold(0, |depth, command| match command {
        CanvasCommand::Pop => depth.saturating_sub(1),
        opening if opening.opens() => depth + 1,
        _ => depth,
    })
}

/// The deepest nesting of layers, clips and masks `commands` reach.
fn deepest(commands: &[CanvasCommand]) -> usize {
    let mut depth = 0usize;
    let mut deepest = 0;
    for command in commands {
        if matches!(command, CanvasCommand::Pop) {
            depth = depth.saturating_sub(1);
        } else if command.opens() {
            depth += 1;
            deepest = deepest.max(depth);
        }
    }
    deepest
}

/// The most bytes a whole full dump's answer takes beyond its text's lines and its resources:
/// `{"canvas":false,"text":"`, `","fonts":[`, `],"images":[` and `]}`, `canvas` at its widest.
const CANVAS_WHOLE_FRAME: usize = 49;

/// The most bytes a dump part's answer takes beyond its lines and resources: the whole answer's
/// frame and `,"part":` and `,"parts":`, each count at its widest `u32`.
const CANVAS_PART_FRAME: usize = CANVAS_WHOLE_FRAME + 8 + 9 + 2 * 10;

/// The full dump cut into parts of whole lines, in order: each part's answer at most
/// [`SNAPSHOT_TEXT_BUDGET`] bytes, the first carrying the fonts and images, a line that cannot fit
/// with its part's other content a part alone. A dump with no line is one part.
fn canvas_parts(lines: &[usize], resources: usize) -> Vec<std::ops::Range<usize>> {
    let mut cut = Vec::new();
    let mut start = 0;
    let mut size = CANVAS_PART_FRAME + resources;
    for (index, length) in lines.iter().enumerate() {
        if index > start && size + length > SNAPSHOT_TEXT_BUDGET {
            cut.push(start..index);
            start = index;
            size = CANVAS_PART_FRAME + length;
        } else {
            size += length;
        }
    }
    cut.push(start..lines.len());
    cut
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
            | Outcome::Linted { .. }
            | Outcome::Graphed { .. }
            | Outcome::GraphPart { .. }
            | Outcome::Actions { .. }
            | Outcome::Layout { .. }
            | Outcome::CanvasSummary { .. }
            | Outcome::CanvasDump { .. }
            | Outcome::CanvasPart { .. },
        ) => return,
        Ok(
            Outcome::Acted {
                settled,
                busy,
                diff,
                ..
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
pub(crate) struct Target {
    node: NodeId,
    enabled: Option<bool>,
    /// Whether the snapshot reads a value for the element: a control that holds one.
    holds_value: bool,
    /// Whether the element is a masked control ([`is_masked_input`]).
    masked: bool,
    /// Whether the element is a `select` ([`is_select`]), which a click would open or whose
    /// centre lies on one of its rows.
    select: bool,
    /// The centre of the element's bounds, relative to the viewport.
    centre: (f64, f64),
    /// The element's bounds as the snapshot reads them: x, y, width and height, relative to
    /// the viewport.
    bounds: (f64, f64, f64, f64),
}

impl Target {
    /// The element the call names.
    pub(crate) fn node(&self) -> NodeId {
        self.node
    }
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
    target_of(harness, listed, node)
}

/// The virtual node `id` names on `screen`: the node the snapshot lists under the id, acted on
/// at its region's centre on its canvas, the first element whose stable id is the id's owner,
/// the text before [`VIRTUAL_KEY_SEPARATOR`].
fn locate_virtual(
    harness: &Harness<DioxusDocument>,
    screen: &Snapshot,
    id: &str,
) -> Option<Target> {
    let listed = screen.get(id)?;
    let (owner, _) = id.split_once(VIRTUAL_KEY_SEPARATOR)?;
    let (canvas, _) = harness
        .doc
        .element_ids()
        .into_iter()
        .find(|(_, read)| read == owner)?;
    target_of(harness, listed, canvas)
}

/// The [`Target`] of `node`, which the snapshot lists as `listed`; `None` where the node no
/// longer resolves.
pub(crate) fn target_of(
    harness: &Harness<DioxusDocument>,
    listed: &SnapshotNode,
    node: NodeId,
) -> Option<Target> {
    let doc = harness.base();
    let element = doc.get_node(node)?.element_data();
    let masked = element.is_some_and(is_masked_input);
    let select = element.is_some_and(is_select);
    let bounds = listed.bounds;
    Some(Target {
        node,
        enabled: listed.state.enabled,
        holds_value: listed.state.value.is_some(),
        masked,
        select,
        centre: (
            bounds.x + bounds.width / 2.0,
            bounds.y + bounds.height / 2.0,
        ),
        bounds: (bounds.x, bounds.y, bounds.width, bounds.height),
    })
}

/// The verbs a canvas's virtual node id is taken by: `click` and `layout`. Every other verb that
/// names an element reads such an id not found, through [`resolve`].
const VIRTUAL_ID_VERBS: [&VerbSpec; 2] = [&schema::CLICK, &schema::LAYOUT];

/// Whether the verb `spec` names takes a canvas's virtual node id; [`resolve_for`] and
/// [`Session::actions`](crate::Session::actions) both read it, so a listed cause and a call's
/// refusal agree.
pub(crate) fn takes_virtual_id(spec: &VerbSpec) -> bool {
    VIRTUAL_ID_VERBS.iter().any(|verb| verb.name == spec.name)
}

/// Reads the screen and finds the element `id` names on it, or refuses: as stale when an
/// earlier screen of the session read the id, as not found when none did. The screen read
/// here is recorded once that answer is decided. An id holding [`VIRTUAL_KEY_SEPARATOR`] is
/// refused not found: only `click` and `layout` take one, through [`resolve_for`].
pub(crate) fn resolve(
    harness: &Harness<DioxusDocument>,
    seen: &mut SeenIds,
    id: &str,
) -> Result<(Snapshot, Target), Refusal> {
    let screen = harness.doc.snapshot();
    if id.contains(VIRTUAL_KEY_SEPARATOR) {
        record(seen, &screen);
        return Err(Refusal::new(Cause::NotFound));
    }
    let read_earlier = seen.holds(id);
    let target = locate(harness, &screen, id);
    record(seen, &screen);
    match target {
        Some(target) => Ok((screen, target)),
        None if read_earlier => Err(Refusal::new(Cause::Stale)),
        None => Err(Refusal::new(Cause::NotFound)),
    }
}

/// [`resolve`] for the verb `spec` names. For `click` and `layout`, the verbs a canvas's virtual
/// node takes ([`takes_virtual_id`]), an id holding [`VIRTUAL_KEY_SEPARATOR`] that the screen
/// lists is found on its canvas, at its region's centre; one it does not list is refused as
/// stale or not found, as any id.
pub(crate) fn resolve_for(
    harness: &Harness<DioxusDocument>,
    seen: &mut SeenIds,
    spec: &VerbSpec,
    id: &str,
) -> Result<(Snapshot, Target), Refusal> {
    if !takes_virtual_id(spec) || !id.contains(VIRTUAL_KEY_SEPARATOR) {
        return resolve(harness, seen, id);
    }
    let screen = harness.doc.snapshot();
    let read_earlier = seen.holds(id);
    let target = locate_virtual(harness, &screen, id);
    record(seen, &screen);
    match target {
        Some(target) => Ok((screen, target)),
        None if read_earlier => Err(Refusal::new(Cause::Stale)),
        None => Err(Refusal::new(Cause::NotFound)),
    }
}

/// What an id-taking call's check after [`resolve`] leaves its arm to act on.
pub(crate) enum Cleared {
    /// The page point a `click` or a `type` lands at.
    Point(f32, f32),
    /// The device pixels a `screenshot` of an element paints.
    Rect(PixelRect),
    /// Nothing to act on: a `scroll`, which no check stops.
    Free,
}

impl Cleared {
    /// The page point; the origin for a check that gives none, which no `click` or `type` meets.
    fn point(self) -> (f32, f32) {
        match self {
            Self::Point(x, y) => (x, y),
            Self::Rect(_) | Self::Free => (0.0, 0.0),
        }
    }
}

/// The one check an id-taking `command` meets once its `target` resolves, in its verb's order,
/// before its arm acts: `click` and `type` the three of [`action_point`]; `scroll` and `layout`
/// none; a
/// `screenshot` of an element out of view, then one whose visible part holds no pixel, then an
/// app with no font it can paint text with; a `canvas` of an app with no font it can paint text
/// with. [`Session::run`](crate::Session::run)'s arms and
/// [`Session::actions`](crate::Session::actions) both decide through it, so the read names the
/// cause a call meets.
pub(crate) fn check_target(
    harness: &Harness<DioxusDocument>,
    command: &Command,
    target: &Target,
) -> Result<Cleared, Refusal> {
    match command {
        Command::Click { .. } | Command::Type { .. } => {
            let (x, y) = action_point(harness, target)?;
            Ok(Cleared::Point(x, y))
        }
        Command::Screenshot { id: Some(_) } => {
            if !in_view(harness, target) {
                return Err(Refusal::new(Cause::OffScreen));
            }
            let rect = crop(harness, target).ok_or(Refusal::new(Cause::OffScreen))?;
            if !harness.base().can_paint_text() {
                return Err(Refusal::new(Cause::FontsUnavailable));
            }
            Ok(Cleared::Rect(rect))
        }
        Command::Layout { .. } => Ok(Cleared::Free),
        Command::Canvas { .. } => {
            if harness.base().can_paint_text() {
                Ok(Cleared::Free)
            } else {
                Err(Refusal::new(Cause::FontsUnavailable))
            }
        }
        _ => Ok(Cleared::Free),
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

/// What a step caused: the items kept and those past the count.
type StepCaused = (Vec<Caused>, Vec<CausedPast>);

/// Runs `step` after the snapshot `before` and settles the instance before the next one: the
/// class of work still outstanding, if any, the diff of the two, the snapshot after and what
/// the step caused. The passes a settle that went quiet took are recorded on `span`.
///
/// A panic in the step or the settle is caught and counted, and the settle runs on: once after
/// a panic in the step, once more after a panic in the settle, and a second panic there reads
/// as render work outstanding.
fn settled_step(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    before: Snapshot,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> (Option<Busy>, SnapshotDiff, Snapshot, StepCaused) {
    harness.doc.take_step_reached();
    let marks_before = harness.doc.boundary_marks();
    let ((panics, settle), captured) = escher_telemetry::capture(|| {
        let mut panics = 0u32;
        if catch_unwind(AssertUnwindSafe(|| step(harness))).is_err() {
            panics += 1;
        }
        let mut settle = catch_unwind(AssertUnwindSafe(|| settle_waiting(harness)));
        if settle.is_err() {
            panics += 1;
            settle = catch_unwind(AssertUnwindSafe(|| settle_waiting(harness)));
            if settle.is_err() {
                panics += 1;
            }
        }
        let settle = settle.unwrap_or(Err(NotSettled { busy: Busy::Render }));
        (panics, settle)
    });
    let reached = harness.doc.take_step_reached();
    let busy = match settle {
        Ok(settled) => {
            span.record("passes", settled.passes);
            None
        }
        Err(not_settled) => Some(not_settled.busy),
    };
    let after = read_screen(harness, seen);
    let caused = caused(
        &harness.doc,
        StepReading {
            panics,
            marks_before,
            captured,
            reached,
        },
    );
    (busy, before.diff(&after), after, caused)
}

fn acted(
    harness: &mut Harness<DioxusDocument>,
    seen: &mut SeenIds,
    span: &Span,
    before: Snapshot,
    step: impl FnOnce(&mut Harness<DioxusDocument>),
) -> Outcome {
    let (busy, diff, _, (caused, caused_past)) = settled_step(harness, seen, span, before, step);
    Outcome::Acted {
        settled: busy.is_none(),
        busy,
        diff,
        caused,
        caused_past,
    }
}

/// The modifier the engine's editor reads as its action modifier: Super on macOS, Control
/// elsewhere.
#[cfg(target_os = "macos")]
const ACTION_MODIFIER: Modifiers = Modifiers::META;
#[cfg(not(target_os = "macos"))]
const ACTION_MODIFIER: Modifiers = Modifiers::CONTROL;

/// Focuses a select with no click, which would open its list or land on the row at its centre:
/// any list that stands closes and its typed text starts afresh, as a click's would. No DOM
/// focus event fires.
fn focus_select(harness: &mut Harness<DioxusDocument>, select: NodeId) {
    let mut doc = harness.base_mut();
    doc.close_select_list();
    doc.clear_select_typeahead();
    doc.set_focus_silently(select);
}

/// Selects everything the focused control holds, through the editor's own select-all key:
/// what is typed next replaces the selection.
fn select_all(harness: &mut Harness<DioxusDocument>) {
    harness.press_with(KeyboardKey::Character("a".to_string()), ACTION_MODIFIER);
}

fn press(harness: &mut Harness<DioxusDocument>, key: Key, shift: bool, ctrl: bool) {
    let mut modifiers = Modifiers::default();
    if shift {
        modifiers |= Modifiers::SHIFT;
    }
    if ctrl {
        modifiers |= ACTION_MODIFIER;
    }
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
    fn click_and_layout_alone_take_a_virtual_id() {
        let taking: Vec<&str> = crate::actions::target_verbs()
            .filter(|spec| takes_virtual_id(spec))
            .map(|spec| spec.name)
            .collect();
        assert_eq!(taking, ["click", "layout"]);
    }

    #[test]
    fn a_graph_is_cut_into_parts_of_whole_nodes_within_the_budget() {
        // The frames are the written form's own: `{"nodes":[`, `],"states":[`, `]}`, and a part's
        // `,"part":` and `,"parts":` with two ten-digit counts.
        assert_eq!(GRAPH_WHOLE_FRAME, 24);
        assert_eq!(GRAPH_PART_FRAME, 61);
        // No node: one part, the states alone.
        assert_eq!(graph_parts(&[], 100), [0..0]);
        // Nodes of 1,000 bytes: the first part holds the states' 1,000 bytes beside eight nodes.
        let nodes = [1000; 30];
        let cut = graph_parts(&nodes, 1000);
        assert_eq!(cut.first(), Some(&(0..8)));
        assert_eq!(cut.get(1), Some(&(8..17)));
        assert_eq!(cut.last().map(|range| range.end), Some(30));
        for pair in cut.windows(2) {
            assert_eq!(pair[0].end, pair[1].start);
        }
        for (index, range) in cut.iter().enumerate() {
            let states = if index == 0 { 1000 } else { 0 };
            let size = GRAPH_PART_FRAME + states + joined(&nodes[range.clone()]);
            assert!(size <= SNAPSHOT_TEXT_BUDGET, "part {index}");
        }
        // A node longer than the budget is a part alone.
        assert_eq!(graph_parts(&[10, 20_000, 10], 0), [0..1, 1..2, 2..3]);
    }

    #[test]
    fn a_canvas_dump_is_cut_into_parts_of_whole_lines_within_the_budget() {
        // `{"canvas":false,"text":"`, `","fonts":[`, `],"images":[`, `]}`, and a part's
        // `,"part":` and `,"parts":` with two ten-digit counts.
        assert_eq!(
            CANVAS_WHOLE_FRAME,
            r#"{"canvas":false,"text":"","fonts":[],"images":[]}"#.len()
        );
        assert_eq!(CANVAS_PART_FRAME, 86);
        assert_eq!(canvas_parts(&[], 100), [0..0]);
        let lines = [450; 60];
        let cut = canvas_parts(&lines, 18);
        assert_eq!(cut.first(), Some(&(0..21)));
        assert_eq!(cut.last().map(|range| range.end), Some(60));
        for pair in cut.windows(2) {
            assert_eq!(pair[0].end, pair[1].start);
        }
        for (index, range) in cut.iter().enumerate() {
            let resources = if index == 0 { 18 } else { 0 };
            let size = CANVAS_PART_FRAME + resources + lines[range.clone()].iter().sum::<usize>();
            assert!(size <= SNAPSHOT_TEXT_BUDGET, "part {index}");
        }
        assert_eq!(canvas_parts(&[10, 20_000, 10], 0), [0..1, 1..2, 2..3]);
    }

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
        assert_eq!(arguments.len(), 16);
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
