//! The lint finding: a defect's element, its bounds and its reason.

use super::{Object, number, ratio};
use crate::lint::{Bounds, Finding, Reason};

/// A box as a JSON list of x, y, width and height, as a node's bounds are written.
fn bounds(rect: Bounds) -> String {
    let mut text = String::from("[");
    for (index, bound) in [rect.x, rect.y, rect.width, rect.height]
        .into_iter()
        .enumerate()
    {
        if index > 0 {
            text.push(',');
        }
        number(&mut text, bound);
    }
    text.push(']');
    text
}

/// One finding as an object keyed `id`, `defect`, `bounds`, then its reason's keys.
pub(super) fn finding(finding: &Finding) -> String {
    let mut object = Object::new();
    object.text("id", &finding.id);
    object.text("defect", finding.defect.name());
    object.json("bounds", &bounds(finding.bounds));
    match &finding.reason {
        Reason::Overlap { other, area } => {
            object.text("other", other);
            object.json("area", &bounds(*area));
        }
        Reason::Clipped { visible } => object.json("visible", &bounds(*visible)),
        Reason::Truncated { text, visible } => {
            object.json("text", &bounds(*text));
            if let Some(visible) = visible {
                object.json("visible", &bounds(*visible));
            }
        }
        Reason::OffScreen | Reason::ZeroSize => {}
        Reason::Contrast {
            ratio: measured,
            required,
        } => {
            ratio(object.member("ratio"), *measured);
            ratio(object.member("required"), *required);
        }
    }
    object.end()
}
