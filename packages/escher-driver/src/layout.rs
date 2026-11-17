//! Why an element sits where it does at its size, read on the held instance by stable id: the
//! engine's [`LayoutReason`] with every node named by its stable element id.

use std::collections::HashMap;

use dioxus_native_dom::{
    BoundingRect, LayoutReason, NodeId, PlacedBy, ShiftBy, SizeCause, SizeReason,
};

use crate::execute::resolve;
use crate::lint::Bounds;
use crate::refusal::Refusal;
use crate::session::Session;

/// Why one element sits where it does at its size. Built by [`Session::layout`]. It holds
/// stable ids, closed words and numbers: no `NodeId`, page text, authored style value, colour
/// or control value.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutExplanation {
    /// The element's stable element id.
    pub id: String,
    /// Where it stands: its snapshot `bounds`.
    pub bounds: Bounds,
    /// The same rect before any transform on its chain.
    pub laid_out: Bounds,
    /// What placed it, in which element and at what offset.
    pub placed: Placement,
    /// Each box that moves it from where layout put it, innermost first, then the viewport.
    pub shifts: Vec<ShiftReading>,
    /// The cause of its width, with the computed inputs.
    pub width: SizeReason,
    /// The cause of its height, with the computed inputs.
    pub height: SizeReason,
}

/// What placed an element, in which element and at what offset.
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    /// What placed it.
    pub placer: Placer,
    /// The stable id of the element whose box it was placed in; `None` for the root element.
    pub container: Option<String>,
    /// The offset of its border box from its container's, in CSS px.
    pub at: (f64, f64),
    /// The `position: relative` inset `at` includes, where one applies.
    pub relative: Option<(f64, f64)>,
}

/// What placed an element.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Placer {
    /// The engine, by the algorithm named.
    Engine(PlacedBy),
    /// Nothing the engine reads: an element the snapshot lists for which the engine reads no
    /// layout reason, such as one with no box of its own.
    Unexplained,
    /// The app, inside the canvas element named by its stable id. Reserved: nothing places a
    /// node so yet.
    App {
        /// The canvas element's stable id.
        canvas: String,
    },
}

/// One box that moves an element from where layout put it.
#[derive(Debug, Clone, PartialEq)]
pub struct ShiftReading {
    /// How it moves it.
    pub by: ShiftBy,
    /// The stable id of the box that moves it; `None` for the viewport.
    pub holder: Option<String>,
    /// The horizontal move, in CSS px.
    pub dx: f64,
    /// The vertical move, in CSS px.
    pub dy: f64,
}

impl Session {
    /// Why the element `id` names sits where it does at its size, read from the held instance's
    /// layout and style.
    ///
    /// The id resolves as a call's does: the screen is read once and its ids recorded, and an
    /// id it does not read is refused [`Cause::Stale`](crate::Cause::Stale) when an earlier
    /// screen read it, else [`Cause::NotFound`](crate::Cause::NotFound). It runs no step and
    /// changes nothing. The `layout` verb reaches it through [`Session::run`].
    pub fn layout(&mut self, id: &str) -> Result<LayoutExplanation, Refusal> {
        let (screen, target) = resolve(&self.harness, &mut self.seen, id)?;
        let node = target.node();
        let ids: HashMap<NodeId, String> = self.harness.doc.element_ids().into_iter().collect();
        let reason = self.harness.base().layout_reason(node);
        Ok(match reason {
            Some(reason) => explain(id, reason, &|node| ids.get(&node).cloned()),
            None => {
                let bounds = screen
                    .get(id)
                    .map_or(ZERO, |listed| bounds_of(listed.bounds));
                unexplained(id, bounds)
            }
        })
    }
}

/// `reason`, read for the element `id`, with each node named by its stable id through
/// `stable_id`.
pub(crate) fn explain(
    id: &str,
    reason: LayoutReason,
    stable_id: &dyn Fn(NodeId) -> Option<String>,
) -> LayoutExplanation {
    LayoutExplanation {
        id: id.to_string(),
        bounds: bounds_of(reason.bounds),
        laid_out: bounds_of(reason.laid_out),
        placed: Placement {
            placer: Placer::Engine(reason.placed.by),
            container: reason.placed.container.and_then(stable_id),
            at: reason.placed.at,
            relative: reason.placed.relative,
        },
        shifts: reason
            .shifts
            .into_iter()
            .map(|shift| ShiftReading {
                by: shift.by,
                holder: shift.holder.and_then(stable_id),
                dx: shift.dx,
                dy: shift.dy,
            })
            .collect(),
        width: reason.width,
        height: reason.height,
    }
}

/// No area at the origin: the bounds of an element the screen no longer lists.
const ZERO: Bounds = Bounds {
    x: 0.0,
    y: 0.0,
    width: 0.0,
    height: 0.0,
};

fn bounds_of(rect: BoundingRect) -> Bounds {
    Bounds {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

/// The reading of an element the snapshot lists that the engine reads no reason for:
/// [`Placer::Unexplained`] at its snapshot `bounds`, in no container, both causes
/// [`SizeCause::Layout`].
pub(crate) fn unexplained(id: &str, bounds: Bounds) -> LayoutExplanation {
    let layout = |px: f64| SizeReason {
        px,
        by: SizeCause::Layout,
        size: dioxus_native_dom::Length::Auto,
        min: dioxus_native_dom::Length::Auto,
        max: dioxus_native_dom::Length::None,
        flex: None,
    };
    LayoutExplanation {
        id: id.to_string(),
        bounds,
        laid_out: bounds,
        placed: Placement {
            placer: Placer::Unexplained,
            container: None,
            at: (0.0, 0.0),
            relative: None,
        },
        shifts: Vec::new(),
        width: layout(bounds.width),
        height: layout(bounds.height),
    }
}

#[cfg(test)]
#[path = "layout_tests.rs"]
mod tests;
