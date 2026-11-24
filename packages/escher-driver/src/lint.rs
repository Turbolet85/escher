//! The visual lint: the screen of a held instance read for six defects, each found on an
//! element named by its stable id, with the measured reason.

use std::collections::HashMap;

use anyrender::Scene;
use blitz_test_harness::Harness;
use dioxus_native_dom::{DioxusDocument, NodeId, Snapshot, SnapshotNode, closed_select_of};
use peniko::kurbo::Rect;

use crate::canvas_lint;
use crate::refusal::{Cause, Refusal};
use crate::session::Session;

/// How far, in CSS px, one box may pass another's edge and still read inside it, and how much
/// of an intersection is no area: the layout's 1/64 px grid, below anything painted.
const SLACK: f64 = 1.0 / 64.0;

/// A rectangle in CSS px relative to the viewport, in the space of a snapshot node's bounds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// The left edge.
    pub x: f64,
    /// The top edge.
    pub y: f64,
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

/// The [`Bounds`] of an engine rectangle, read field by field.
macro_rules! bounds {
    ($rect:expr) => {{
        let rect = $rect;
        Bounds {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }};
}

impl Bounds {
    fn right(self) -> f64 {
        self.x + self.width
    }

    fn bottom(self) -> f64 {
        self.y + self.height
    }

    fn has_area(self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    /// The part of `self` inside `other`, when it holds more than the slack on both axes.
    fn intersection(self, other: Bounds) -> Option<Bounds> {
        let (x, y) = (self.x.max(other.x), self.y.max(other.y));
        let width = self.right().min(other.right()) - x;
        let height = self.bottom().min(other.bottom()) - y;
        (width > SLACK && height > SLACK).then_some(Bounds {
            x,
            y,
            width,
            height,
        })
    }

    /// Whether `self` lies inside `other`, to the slack.
    fn within(self, other: Bounds) -> bool {
        self.x >= other.x - SLACK
            && self.y >= other.y - SLACK
            && self.right() <= other.right() + SLACK
            && self.bottom() <= other.bottom() + SLACK
    }

    /// `self` narrowed to `other` on each axis asked; empty when nothing is left.
    fn cut(self, other: Bounds, (on_x, on_y): (bool, bool)) -> Bounds {
        let (x, right) = match on_x {
            true => (self.x.max(other.x), self.right().min(other.right())),
            false => (self.x, self.right()),
        };
        let (y, bottom) = match on_y {
            true => (self.y.max(other.y), self.bottom().min(other.bottom())),
            false => (self.y, self.bottom()),
        };
        Bounds {
            x,
            y,
            width: (right - x).max(0.0),
            height: (bottom - y).max(0.0),
        }
    }
}

/// What the lint finds wrong with an element, one closed word each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Defect {
    /// The element's visible box intersects another's, neither holding the other.
    Overlap,
    /// Part of the element's box is cut from view by a box that clips it, or by the viewport.
    Clipped,
    /// Part of the element's laid-out text, or of a label its canvas widget paints, is cut from
    /// view.
    Truncated,
    /// No part of the element's box can be seen.
    OffScreen,
    /// The element holds content in a box with no width or no height.
    ZeroSize,
    /// The element's text reads below the contrast ratio WCAG 2.x SC 1.4.3 sets for it.
    Contrast,
}

impl Defect {
    /// Every defect, in the order an element's findings follow.
    pub const ALL: [Defect; 6] = [
        Defect::Overlap,
        Defect::Clipped,
        Defect::Truncated,
        Defect::OffScreen,
        Defect::ZeroSize,
        Defect::Contrast,
    ];

    /// The defect's closed word.
    pub fn name(self) -> &'static str {
        match self {
            Defect::Overlap => "overlap",
            Defect::Clipped => "clipped",
            Defect::Truncated => "truncated",
            Defect::OffScreen => "off-screen",
            Defect::ZeroSize => "zero-size",
            Defect::Contrast => "contrast",
        }
    }

    /// Where the defect stands in [`Defect::ALL`].
    fn rank(self) -> usize {
        self as usize
    }
}

/// The measured reading that shows a defect: numbers and stable ids only.
#[derive(Debug, Clone, PartialEq)]
pub enum Reason {
    /// The element's visible box and another's intersect.
    Overlap {
        /// The stable id of the other element, later in document order.
        other: String,
        /// Where the two intersect.
        area: Bounds,
    },
    /// The element's box is partly cut from view.
    Clipped {
        /// The part of the box that can be seen.
        visible: Bounds,
    },
    /// The element's laid-out text is partly or wholly cut from view.
    Truncated {
        /// The box the laid-out lines take, or a canvas label's ink box.
        text: Bounds,
        /// The part of that box that can be seen; `None` when none can.
        visible: Option<Bounds>,
    },
    /// No part of the element's box can be seen.
    OffScreen,
    /// The element's box has no width or no height.
    ZeroSize,
    /// The element's text contrast is below the ratio it needs.
    Contrast {
        /// The measured WCAG 2.x contrast ratio.
        ratio: f32,
        /// The ratio SC 1.4.3 sets for the text: 3 for large text, else 4.5.
        required: f32,
    },
}

/// One defect the lint finds on one element.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    /// The element's stable element id, as its snapshot node lists it.
    pub id: String,
    /// What is wrong.
    pub defect: Defect,
    /// The element's bounds, as its snapshot node reads them.
    pub bounds: Bounds,
    /// The reading that shows it.
    pub reason: Reason,
}

impl Session {
    /// Reads the screen of the held instance and returns every defect found on it, in document
    /// order, one element's findings in [`Defect::ALL`]'s order.
    ///
    /// The elements read are the snapshot's nodes, each with its bounds, and their ids join the
    /// session's record of ids read, as a `snapshot`'s do. Each element can be seen through its
    /// clip region: the part of the viewport left by every box that clips it by `overflow`
    /// along its containing blocks, and by every ancestor whose computed `contain` includes
    /// `paint`, cut to its padding box. Read over the elements whose box has an area:
    ///
    /// - [`Defect::Overlap`]: the visible parts of two elements, neither holding the other,
    ///   intersect. A pair is found once, at the highest level it occurs, on the earlier
    ///   element: not where an ancestor of either, below the two's nearest common ancestor,
    ///   already overlaps the other. An option or a group of a closed select, read in its
    ///   select's box, overlaps nothing, nor does a row or a group of an open list, which sits
    ///   over the page by design; a `multiple` select's rows are read like any element.
    /// - [`Defect::Clipped`]: part of the box, not all of it, lies outside its clip region.
    /// - [`Defect::Truncated`]: the box the element's laid-out lines take is not inside its clip
    ///   region, cut further by its own padding box where it clips. A text control's value is
    ///   never measured. On a canvas element, once per label its widget paints, in paint order:
    ///   the label's ink box — its glyphs' outline bounds — is not inside its region, the
    ///   element's content box cut by every clip, layer or mask the widget pushed around it and
    ///   by the element's clip region.
    /// - [`Defect::OffScreen`]: no part of the box lies inside its clip region.
    /// - [`Defect::Contrast`]: an element holding text of its own, or a text control, whose
    ///   [`Harness::text_contrast`] reads below its required ratio; an element that reads not
    ///   enabled is exempt, and one whose contrast is not measured reads nothing.
    ///
    /// And over the elements with no area, [`Defect::ZeroSize`]: one that holds content — a
    /// name, a value, a form control, a focusable element, or text among its descendants.
    ///
    /// Geometry only: no hit is tested. Right-to-left and vertical text is read as
    /// left-to-right, horizontal lines. An id two elements share reads the first of them. A
    /// canvas's pushed clip, layer or mask path is read as its bounding box; a canvas under a
    /// transform reads its labels from its client rect's top left, unrotated and unscaled; a
    /// glyph with no outline adds nothing to a label's ink box; and a canvas element the snapshot
    /// does not list is not linted.
    ///
    /// It runs no step and settles nothing: no input is dispatched, no time moves and nothing
    /// is marked changed. An app with no font it can paint text with is refused as
    /// [`Cause::FontsUnavailable`] before anything is read.
    pub fn lint(&mut self) -> Result<Vec<Finding>, Refusal> {
        if !self.harness.base().can_paint_text() {
            return Err(Refusal::new(Cause::FontsUnavailable));
        }
        let screen = self.harness.doc.snapshot();
        for node in screen.nodes() {
            self.seen.record(&node.id);
        }
        Ok(findings(&mut self.harness, &screen))
    }
}

/// One snapshot node as the lint reads it.
struct Entry<'a> {
    node: &'a SnapshotNode,
    /// The index of its parent in the snapshot's pre-order.
    parent: Option<usize>,
    /// The element the node lists, where its id resolves.
    element: Option<NodeId>,
    bounds: Bounds,
    /// The part of the viewport the element can be seen through.
    region: Option<Bounds>,
    /// The part of its box that can be seen, for a box with an area.
    visible: Option<Bounds>,
    /// Whether the element is an option or a group of a closed select, read in its select's box,
    /// or a row or a group of an open list, which sits over the page by design.
    in_select_box: bool,
}

fn findings(harness: &mut Harness<DioxusDocument>, screen: &Snapshot) -> Vec<Finding> {
    let mut elements = HashMap::new();
    for (node, id) in harness.doc.element_ids() {
        elements.entry(id).or_insert(node);
    }
    let mut entries = Vec::new();
    for root in &screen.roots {
        flatten(harness, &elements, root, None, &mut entries);
    }
    let scenes = canvas_scenes(harness, &entries);
    let harness = &*harness;

    let mut found: Vec<(usize, Finding)> = Vec::new();
    let finding = |entry: &Entry, defect: Defect, reason: Reason| Finding {
        id: entry.node.id.clone(),
        defect,
        bounds: entry.bounds,
        reason,
    };
    for (earlier, later, area) in overlaps(&entries) {
        let other = entries[later].node.id.clone();
        let reason = Reason::Overlap { other, area };
        found.push((earlier, finding(&entries[earlier], Defect::Overlap, reason)));
    }
    for (index, (entry, scene)) in entries.iter().zip(&scenes).enumerate() {
        let Some(element) = entry.element else {
            continue;
        };
        for (defect, reason) in readings(harness, entry, element, scene.as_ref()) {
            found.push((index, finding(entry, defect, reason)));
        }
    }
    ordered(found)
}

/// The defects `entry` reads, each with its reason, in the order they are found: zero-size alone
/// for a box with no area, else off-screen or clipped, then truncated — its laid-out lines, then
/// each label of its canvas `scene` — then contrast.
fn readings(
    harness: &Harness<DioxusDocument>,
    entry: &Entry,
    element: NodeId,
    scene: Option<&Scene>,
) -> Vec<(Defect, Reason)> {
    if !entry.bounds.has_area() {
        return holds_content(harness, entry.node, element)
            .then_some((Defect::ZeroSize, Reason::ZeroSize))
            .into_iter()
            .collect();
    }
    let lines = truncated(harness, entry, element).map(|reason| (Defect::Truncated, reason));
    let labels = scene
        .into_iter()
        .flat_map(|scene| canvas_truncations(harness, entry, element, scene))
        .map(|reason| (Defect::Truncated, reason));
    let colours = contrast(harness, entry.node, element).map(|reason| (Defect::Contrast, reason));
    placement(entry)
        .into_iter()
        .chain(lines)
        .chain(labels)
        .chain(colours)
        .collect()
}

/// The off-screen or clipped reading of `entry`, a box with an area.
fn placement(entry: &Entry) -> Option<(Defect, Reason)> {
    match (entry.region, entry.visible) {
        (_, None) => Some((Defect::OffScreen, Reason::OffScreen)),
        (Some(region), Some(visible)) if !entry.bounds.within(region) => {
            Some((Defect::Clipped, Reason::Clipped { visible }))
        }
        _ => None,
    }
}

/// The findings in document order, one element's in [`Defect::ALL`]'s order, and findings of one
/// element and defect in the order they were found.
fn ordered(mut found: Vec<(usize, Finding)>) -> Vec<Finding> {
    found.sort_by_key(|(index, finding)| (*index, finding.defect.rank()));
    found.into_iter().map(|(_, finding)| finding).collect()
}

/// Appends `node` and its descendants to `out` in pre-order, each with its clip region.
fn flatten<'a>(
    harness: &Harness<DioxusDocument>,
    elements: &HashMap<String, NodeId>,
    node: &'a SnapshotNode,
    parent: Option<usize>,
    out: &mut Vec<Entry<'a>>,
) {
    let element = elements.get(&node.id).copied();
    let bounds = bounds!(node.bounds);
    let region = element.and_then(|element| clip_region(harness, element));
    let visible = region
        .filter(|_| bounds.has_area())
        .and_then(|region| bounds.intersection(region));
    let in_select_box = element.is_some_and(|element| {
        let doc = harness.base();
        closed_select_of(&doc, element).is_some() || doc.select_list_bounds(element).is_some()
    });
    let index = out.len();
    out.push(Entry {
        node,
        parent,
        element,
        bounds,
        region,
        visible,
        in_select_box,
    });
    for child in &node.children {
        flatten(harness, elements, child, Some(index), out);
    }
}

/// Every pair of entries, neither an ancestor of the other, whose visible parts intersect, at
/// the highest level it occurs: the earlier, the later and where they intersect.
fn overlaps(entries: &[Entry]) -> Vec<(usize, usize, Bounds)> {
    let ancestors: Vec<Vec<usize>> = (0..entries.len())
        .map(|index| {
            std::iter::successors(entries[index].parent, |&up| entries[up].parent).collect()
        })
        .collect();
    let mut pairs = Vec::new();
    for (earlier, first) in entries.iter().enumerate() {
        let Some(a) = first.visible.filter(|_| !first.in_select_box) else {
            continue;
        };
        for (later, second) in entries.iter().enumerate().skip(earlier + 1) {
            let Some(b) = second.visible.filter(|_| !second.in_select_box) else {
                continue;
            };
            if ancestors[later].contains(&earlier) {
                continue;
            }
            let Some(area) = a.intersection(b) else {
                continue;
            };
            let common = ancestors[later]
                .iter()
                .find(|up| ancestors[earlier].contains(up));
            let overlaps_above = |up: &[usize], other: Bounds| {
                up.iter().take_while(|&up| Some(up) != common).any(|&up| {
                    entries[up]
                        .visible
                        .is_some_and(|seen| seen.intersection(other).is_some())
                })
            };
            if overlaps_above(&ancestors[earlier], b) || overlaps_above(&ancestors[later], a) {
                continue;
            }
            pairs.push((earlier, later, area));
        }
    }
    pairs
}

/// The part of the viewport `element` can be seen through: the engine's visible region, cut to
/// the padding box of every ancestor below the root whose `contain` includes `paint`.
fn clip_region(harness: &Harness<DioxusDocument>, element: NodeId) -> Option<Bounds> {
    let doc = harness.base();
    let mut region = bounds!(doc.visible_region(element)?);
    // An open list is painted past every clip, so nothing that holds its select cuts its rows.
    if doc.select_list_bounds(element).is_some() {
        return region.has_area().then_some(region);
    }
    let root = doc.try_root_element().map(|root| root.id);
    let mut current = doc.get_node(element).and_then(|node| node.parent);
    while let Some(id) = current.filter(|&id| Some(id) != root) {
        if contains_paint(harness, id) {
            region = region.cut(padding_box(harness, id)?, (true, true));
        }
        current = doc.get_node(id).and_then(|node| node.parent);
    }
    region.has_area().then_some(region)
}

/// Whether `element`'s computed `contain` includes `paint` on a box it applies to: neither a
/// non-atomic inline, an internal table box other than a cell, nor no box at all.
fn contains_paint(harness: &Harness<DioxusDocument>, element: NodeId) -> bool {
    let doc = harness.base();
    let contain = doc.resolved_style_value(element, "contain");
    let paint = contain
        .split_whitespace()
        .any(|word| matches!(word, "paint" | "strict" | "content"));
    let display = doc.resolved_style_value(element, "display");
    let applies = !matches!(display.as_str(), "inline" | "contents" | "none" | "")
        && !(display.starts_with("table-")
            && !matches!(display.as_str(), "table-cell" | "table-caption"));
    paint && applies
}

/// The axes `element` clips what it holds on: by `overflow`, each axis apart, or both by
/// `contain: paint`.
fn clips(harness: &Harness<DioxusDocument>, element: NodeId) -> (bool, bool) {
    let clips_axis = |property| {
        let overflow = harness.base().resolved_style_value(element, property);
        !matches!(overflow.as_str(), "visible" | "")
    };
    let paint = contains_paint(harness, element);
    (
        paint || clips_axis("overflow-x"),
        paint || clips_axis("overflow-y"),
    )
}

/// `element`'s padding box, relative to the viewport: its client rect less its border and any
/// scrollbar.
fn padding_box(harness: &Harness<DioxusDocument>, element: NodeId) -> Option<Bounds> {
    let doc = harness.base();
    let rect = bounds!(doc.get_client_bounding_rect(element)?);
    let layout = doc.get_node(element)?.final_layout();
    let (border, bar) = (layout.border, layout.scrollbar_size);
    Some(Bounds {
        x: rect.x + f64::from(border.left),
        y: rect.y + f64::from(border.top),
        width: rect.width - f64::from(border.left + border.right + bar.width),
        height: rect.height - f64::from(border.top + border.bottom + bar.height),
    })
}

/// Whether text holding a character that is not whitespace is `element`'s direct child, or,
/// with `deep`, among its descendants.
fn holds_text(harness: &Harness<DioxusDocument>, element: NodeId, deep: bool) -> bool {
    let doc = harness.base();
    let mut stack: Vec<NodeId> = doc
        .get_node(element)
        .map(|node| node.children.to_vec())
        .unwrap_or_default();
    while let Some(id) = stack.pop() {
        let Some(node) = doc.get_node(id) else {
            continue;
        };
        if node
            .text_data()
            .is_some_and(|text| text.content.chars().any(|c| !c.is_whitespace()))
        {
            return true;
        }
        if deep {
            stack.extend(node.children.iter().copied());
        }
    }
    false
}

/// Whether `element` is a text control: an editor holds its value.
fn is_text_control(harness: &Harness<DioxusDocument>, element: NodeId) -> bool {
    harness
        .base()
        .get_node(element)
        .and_then(|node| node.element_data())
        .is_some_and(|data| data.text_input_data().is_some())
}

/// Whether a node with no area holds anything a reader would miss.
fn holds_content(harness: &Harness<DioxusDocument>, node: &SnapshotNode, element: NodeId) -> bool {
    let focusable = harness
        .base()
        .get_node(element)
        .is_some_and(|node| node.is_focussable());
    !node.name.is_empty()
        || node.state.value.is_some()
        || node.state.enabled.is_some()
        || focusable
        || holds_text(harness, element, true)
}

/// The truncated reading of `entry`: its laid-out lines against the region its text can be seen
/// through.
fn truncated(harness: &Harness<DioxusDocument>, entry: &Entry, element: NodeId) -> Option<Reason> {
    if is_text_control(harness, element) {
        return None;
    }
    let text = bounds!(harness.base().text_extent(element)?.text);
    if !text.has_area() {
        return None;
    }
    let axes = clips(harness, element);
    let region = match axes {
        (false, false) => entry.region,
        axes => entry
            .region
            .zip(padding_box(harness, element))
            .map(|(region, own)| region.cut(own, axes)),
    };
    if region.is_some_and(|region| text.within(region)) {
        return None;
    }
    let visible = region.and_then(|region| text.intersection(region));
    Some(Reason::Truncated { text, visible })
}

/// The scene each entry's canvas widget records, read for an entry with an element and a box
/// with an area; `None` for any other entry, and for an element with no widget or an empty
/// content box.
fn canvas_scenes(harness: &mut Harness<DioxusDocument>, entries: &[Entry]) -> Vec<Option<Scene>> {
    entries
        .iter()
        .map(|entry| {
            let element = entry.element.filter(|_| entry.bounds.has_area())?;
            harness.canvas_scene(element)
        })
        .collect()
}

/// The truncated readings of the labels a canvas's `scene` paints, in paint order: each label
/// whose ink box is not inside its region — the element's content box, cut by the box its
/// pushes leave and by the element's clip region.
fn canvas_truncations(
    harness: &Harness<DioxusDocument>,
    entry: &Entry,
    element: NodeId,
    scene: &Scene,
) -> Vec<Reason> {
    let Some(content) = content_box(harness, element) else {
        return Vec::new();
    };
    let scale = harness.base().get_viewport().scale_f64();
    // The widget paints in device px with its origin at the content box's top left.
    let to_css = |rect: Rect| Bounds {
        x: content.x + rect.x0 / scale,
        y: content.y + rect.y0 / scale,
        width: rect.width() / scale,
        height: rect.height() / scale,
    };
    canvas_lint::labels(&scene.commands)
        .into_iter()
        .filter_map(|label| {
            let text = to_css(label.ink);
            let own = label
                .clip
                .map_or(content, |clip| content.cut(to_css(clip), (true, true)));
            let region = entry.region.map(|region| own.cut(region, (true, true)));
            if region.is_some_and(|region| text.within(region)) {
                return None;
            }
            let visible = region.and_then(|region| text.intersection(region));
            Some(Reason::Truncated { text, visible })
        })
        .collect()
}

/// `element`'s content box, relative to the viewport: its client rect less its border and
/// padding, floored at no size.
fn content_box(harness: &Harness<DioxusDocument>, element: NodeId) -> Option<Bounds> {
    let doc = harness.base();
    let rect = bounds!(doc.get_client_bounding_rect(element)?);
    let layout = doc.get_node(element)?.final_layout();
    let (border, padding) = (layout.border, layout.padding);
    let (left, top) = (border.left + padding.left, border.top + padding.top);
    let across = left + border.right + padding.right;
    let down = top + border.bottom + padding.bottom;
    Some(Bounds {
        x: rect.x + f64::from(left),
        y: rect.y + f64::from(top),
        width: (rect.width - f64::from(across)).max(0.0),
        height: (rect.height - f64::from(down)).max(0.0),
    })
}

/// The contrast reading of an element holding text of its own, or of a text control.
fn contrast(
    harness: &Harness<DioxusDocument>,
    node: &SnapshotNode,
    element: NodeId,
) -> Option<Reason> {
    if node.state.enabled == Some(false) {
        return None;
    }
    if !is_text_control(harness, element) && !holds_text(harness, element, false) {
        return None;
    }
    let reading = harness
        .highlight_contrast(element)
        .or_else(|| harness.text_contrast(element))?;
    let required = reading.required_ratio();
    (reading.ratio < required).then_some(Reason::Contrast {
        ratio: reading.ratio,
        required,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_six_defects_are_named_in_order() {
        let rows = [
            (Defect::Overlap, "overlap"),
            (Defect::Clipped, "clipped"),
            (Defect::Truncated, "truncated"),
            (Defect::OffScreen, "off-screen"),
            (Defect::ZeroSize, "zero-size"),
            (Defect::Contrast, "contrast"),
        ];
        assert_eq!(rows.len(), 6);
        assert_eq!(rows.map(|(defect, _)| defect), Defect::ALL);
        for (row, (defect, word)) in rows.iter().enumerate() {
            assert!(defect.name() == *word, "row {row}");
            assert!(defect.rank() == row, "row {row}");
        }
    }

    fn finding(id: &str, defect: Defect) -> Finding {
        let reason = match defect {
            Defect::Overlap => Reason::Overlap {
                other: "later".to_string(),
                area: Bounds {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                },
            },
            Defect::OffScreen => Reason::OffScreen,
            _ => Reason::ZeroSize,
        };
        Finding {
            id: id.to_string(),
            defect,
            bounds: Bounds {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            reason,
        }
    }

    #[test]
    fn findings_follow_document_order_then_the_defect_order() {
        let found = vec![
            (2, finding("c", Defect::Contrast)),
            (0, finding("a", Defect::Contrast)),
            (2, finding("c", Defect::Overlap)),
            (1, finding("b", Defect::ZeroSize)),
            (0, finding("a", Defect::OffScreen)),
            (0, finding("a", Defect::Overlap)),
            (2, finding("c", Defect::Truncated)),
        ];
        let rows = [
            ("a", Defect::Overlap),
            ("a", Defect::OffScreen),
            ("a", Defect::Contrast),
            ("b", Defect::ZeroSize),
            ("c", Defect::Overlap),
            ("c", Defect::Truncated),
            ("c", Defect::Contrast),
        ];
        assert_eq!(rows.len(), found.len());
        let ordered: Vec<(String, Defect)> = ordered(found)
            .into_iter()
            .map(|finding| (finding.id, finding.defect))
            .collect();
        for (row, (id, defect)) in rows.iter().enumerate() {
            assert!(ordered[row] == (id.to_string(), *defect), "row {row}");
        }
    }
}
