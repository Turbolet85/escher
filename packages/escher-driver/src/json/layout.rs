//! The layout answer: an element's placement, shifts and sizes, and the closed words they read.

use dioxus_native_dom::{Length, PlacedBy, ShiftBy, SizeCause, SizeKeyword, SizeReason};

use super::{Object, list, number, ratio, string};
use crate::layout::{LayoutExplanation, Placement, Placer, ShiftReading};
use crate::lint::Bounds;

/// The word a placer is written as. A member the engine's set gains after this writer reads as
/// `unexplained`: the driver reads no reason it can name for it.
pub(super) fn placer_word(placer: &Placer) -> &'static str {
    match placer {
        Placer::Engine(by) => match by {
            PlacedBy::Block => "block",
            PlacedBy::Flex => "flex",
            PlacedBy::Grid => "grid",
            PlacedBy::Table => "table",
            PlacedBy::Inline => "inline",
            PlacedBy::Float => "float",
            PlacedBy::Absolute => "absolute",
            PlacedBy::Fixed => "fixed",
            PlacedBy::Root => "root",
            PlacedBy::Select => "select",
            PlacedBy::SelectList => "select-list",
            _ => "unexplained",
        },
        Placer::Unexplained => "unexplained",
        Placer::App { .. } => "app",
    }
}

/// The word a shift is written as. A member the engine's set gains after this writer reads as
/// `transform`, the move that maps the box rather than offsetting it.
pub(super) fn shift_word(by: ShiftBy) -> &'static str {
    match by {
        ShiftBy::Scroll => "scroll",
        _ => "transform",
    }
}

/// The word a size cause is written as. A member the engine's set gains after this writer reads
/// as `layout`, the cause that singles out none.
pub(super) fn cause_word(by: SizeCause) -> &'static str {
    match by {
        SizeCause::Style => "style",
        SizeCause::Min => "min",
        SizeCause::Max => "max",
        SizeCause::FlexGrow => "flex-grow",
        SizeCause::FlexShrink => "flex-shrink",
        SizeCause::Stretch => "stretch",
        SizeCause::Grid => "grid",
        SizeCause::Table => "table",
        SizeCause::Replaced => "replaced",
        SizeCause::Content => "content",
        _ => "layout",
    }
}

/// The word a sizing keyword is written as. A member the engine's set gains after this writer
/// reads as `auto`.
fn keyword_word(keyword: SizeKeyword) -> &'static str {
    match keyword {
        SizeKeyword::MinContent => "min-content",
        SizeKeyword::MaxContent => "max-content",
        SizeKeyword::FitContent => "fit-content",
        SizeKeyword::Stretch => "stretch",
        _ => "auto",
    }
}

/// A computed length: a word, or an object keyed `px`, `percent` or `calc`. A member the
/// engine's set gains after this writer reads as `"auto"`.
pub(super) fn length(value: &Length) -> String {
    let word = |word: &str| {
        let mut text = String::new();
        string(&mut text, word);
        text
    };
    let keyed = |key: &str, amount: f32| {
        let mut object = Object::new();
        ratio(object.member(key), amount);
        object.end()
    };
    match value {
        Length::None => word("none"),
        Length::Px(px) => keyed("px", *px),
        Length::Percent(percent) => keyed("percent", *percent),
        Length::Calc { px } => keyed("calc", *px),
        Length::Keyword(keyword) => word(keyword_word(*keyword)),
        _ => word("auto"),
    }
}

/// A box as an object keyed `x`, `y`, `width` and `height`.
fn box_object(rect: Bounds) -> String {
    let mut object = Object::new();
    number(object.member("x"), rect.x);
    number(object.member("y"), rect.y);
    number(object.member("width"), rect.width);
    number(object.member("height"), rect.height);
    object.end()
}

/// An offset as an object keyed `x` and `y`.
fn offset((x, y): (f64, f64)) -> String {
    let mut object = Object::new();
    number(object.member("x"), x);
    number(object.member("y"), y);
    object.end()
}

/// A layout answer's `placed`, keyed as the field's help names.
pub(super) fn placement(placed: &Placement) -> String {
    let mut object = Object::new();
    object.text("by", placer_word(&placed.placer));
    if let Some(container) = &placed.container {
        object.text("container", container);
    }
    if let Placer::App { canvas } = &placed.placer {
        object.text("canvas", canvas);
    }
    object.json("at", &offset(placed.at));
    if let Some(relative) = placed.relative {
        object.json("relative", &offset(relative));
    }
    object.end()
}

/// One entry of a layout answer's `shifts`, keyed as the field's help names.
fn shift(shift: &ShiftReading) -> String {
    let mut object = Object::new();
    object.text("by", shift_word(shift.by));
    if let Some(holder) = &shift.holder {
        object.text("holder", holder);
    }
    number(object.member("dx"), shift.dx);
    number(object.member("dy"), shift.dy);
    object.end()
}

/// A layout answer's `width` or `height`, keyed as the field's help names.
fn size(reason: &SizeReason) -> String {
    let mut object = Object::new();
    number(object.member("px"), reason.px);
    object.text("by", cause_word(reason.by));
    object.json("size", &length(&reason.size));
    object.json("min", &length(&reason.min));
    object.json("max", &length(&reason.max));
    if let Some(flex) = &reason.flex {
        let mut inputs = Object::new();
        inputs.json("basis", &length(&flex.basis));
        ratio(inputs.member("grow"), flex.grow);
        ratio(inputs.member("shrink"), flex.shrink);
        object.json("flex", &inputs.end());
    }
    object.end()
}

/// What `layout` returns, keyed as its result fields.
pub(super) fn layout_answer(explanation: &LayoutExplanation) -> String {
    let mut object = Object::new();
    object.text("id", &explanation.id);
    object.json("bounds", &box_object(explanation.bounds));
    object.json("laid_out", &box_object(explanation.laid_out));
    object.json("placed", &placement(&explanation.placed));
    object.json("shifts", &list(explanation.shifts.iter().map(shift)));
    object.json("width", &size(&explanation.width));
    object.json("height", &size(&explanation.height));
    object.end()
}
