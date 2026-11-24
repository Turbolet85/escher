//! Why an element sits where it does at its size: the placing algorithm, the box it was placed
//! in and at what offset, the scrolls and transforms that move it from there, and per physical
//! axis the cause of its size with the computed inputs that decided it.
//!
//! The read reproduces the element's client rect
//! ([`BaseDocument::get_client_bounding_rect`], cssom-view
//! <https://drafts.csswg.org/cssom-view/#dom-element-getboundingclientrect>): with no transform
//! on its chain, an element's rect is its container's rect, plus its offset in the container,
//! less the container's scroll offset.
//!
//! The cause of a size is a stated rule, read per axis, first match wins, lengths compared at
//! 1/64 px ([`SizeCause`]). Its limits: vertical writing modes are read on physical axes
//! against the physical properties; a `calc()` length reads its resolved px alone; a size an
//! `aspect-ratio` derives reads [`SizeCause::Layout`] unless an earlier rule matches; and where
//! no rule singles out one cause the read says [`SizeCause::Layout`] rather than name a wrong
//! one.

use markup5ever::local_name;
use style::computed_values::flex_direction::T as FlexDirection;
use style::properties::ComputedValues;
use style::properties::generated::longhands::box_sizing::computed_value::T as BoxSizing;
use style::properties::generated::longhands::position::computed_value::T as Position;
use style::values::computed::length_percentage::{CalcLengthPercentage, Unpacked};
use style::values::computed::{Float, LengthPercentage};
use style::values::generics::NonNegative;
use style::values::generics::flex::GenericFlexBasis;
use style::values::generics::length::{GenericMaxSize, GenericSize};
use style::values::generics::position::GenericInset;
use style::values::specified::align::AlignFlags;
use style::values::specified::box_::DisplayOutside;

use crate::layout::replaced::is_replaced_element;
use crate::node::Node;
use crate::{BaseDocument, BoundingRect, NodeId};
use stylo_taffy::CopyStyleGetters;

/// Why an element sits where it does at its size. Built by [`BaseDocument::layout_reason`].
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutReason {
    /// Where the element stands, as a snapshot reads it: an open list's row by the list,
    /// else [`BaseDocument::get_client_bounding_rect`] (of its select, for an option of a
    /// closed one).
    pub bounds: BoundingRect,
    /// The same rect before any transform on the chain: where layout put it, less the scrolls.
    pub laid_out: BoundingRect,
    /// The algorithm that placed the box, the box it was placed in and where.
    pub placed: Placed,
    /// Each box that moves the element from where layout put it, innermost first, then the
    /// viewport.
    pub shifts: Vec<Shift>,
    /// The cause of the border box's width.
    pub width: SizeReason,
    /// The cause of the border box's height.
    pub height: SizeReason,
}

/// Where layout placed a box.
#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    /// The algorithm that placed it.
    pub by: PlacedBy,
    /// The element whose box it was placed in: its containing block, an anonymous block folded
    /// into the nearest element holding it. `None` for the root element.
    pub container: Option<NodeId>,
    /// The offset of its border box from `container`'s border box, in physical CSS px snapped
    /// to 1/64; from the document's origin for the root element.
    pub at: (f64, f64),
    /// The `position: relative` inset `at` includes, where one applies.
    pub relative: Option<(f64, f64)>,
}

/// The algorithm that placed a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlacedBy {
    /// Block flow (CSS 2 §9.4.1), a flow root and the document node included.
    Block,
    /// A flex container (css-flexbox-1 §9).
    Flex,
    /// A grid container (css-grid-2 §11).
    Grid,
    /// A table, its cells placed by the table's grid.
    Table,
    /// An inline formatting context: an inline-level box, atomic or not, laid out in lines.
    Inline,
    /// A float (CSS 2 §9.5).
    Float,
    /// `position: absolute`, by its containing block.
    Absolute,
    /// `position: fixed`, by its containing block.
    Fixed,
    /// The root element, by the initial containing block.
    Root,
    /// An option of a closed select, read at its select's box.
    Select,
    /// A row of an open select list, placed by the engine's own list.
    SelectList,
}

/// One box that moves an element from where layout put it.
#[derive(Debug, Clone, PartialEq)]
pub struct Shift {
    /// How it moves it.
    pub by: ShiftBy,
    /// The box that moves it; `None` for the viewport.
    pub holder: Option<NodeId>,
    /// The horizontal move: a scroll offset's negation, 0 for a transform.
    pub dx: f64,
    /// The vertical move: a scroll offset's negation, 0 for a transform.
    pub dy: f64,
}

/// How a box moves what it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShiftBy {
    /// Its scroll offset, or the viewport's.
    Scroll,
    /// Its transform, which maps the box: the element's `bounds` is then the bounding box of
    /// the mapped `laid_out`.
    Transform,
}

/// Why a box has its extent on one physical axis.
#[derive(Debug, Clone, PartialEq)]
pub struct SizeReason {
    /// The border box's extent, in CSS px.
    pub px: f64,
    /// The cause, by the stated rule.
    pub by: SizeCause,
    /// The computed `width` or `height`.
    pub size: Length,
    /// The computed `min-width` or `min-height`.
    pub min: Length,
    /// The computed `max-width` or `max-height`.
    pub max: Length,
    /// The flex inputs, where the box is a flex item and this axis its container's main axis.
    pub flex: Option<FlexInputs>,
}

/// A flex item's inputs on its container's main axis.
#[derive(Debug, Clone, PartialEq)]
pub struct FlexInputs {
    /// The computed `flex-basis`; `content` reads [`Length::Auto`].
    pub basis: Length,
    /// The computed `flex-grow`.
    pub grow: f32,
    /// The computed `flex-shrink`.
    pub shrink: f32,
}

/// A computed length as closed words and numbers.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Length {
    /// `auto`.
    Auto,
    /// `none`, a `max-*` with no limit.
    None,
    /// A length in CSS px.
    Px(f32),
    /// A percentage, 0 to 100.
    Percent(f32),
    /// A `calc()`, resolved against its basis.
    Calc {
        /// The resolved length, in CSS px.
        px: f32,
    },
    /// A sizing keyword.
    Keyword(SizeKeyword),
}

/// A sizing keyword (css-sizing-3 §3.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SizeKeyword {
    /// `min-content`.
    MinContent,
    /// `max-content`.
    MaxContent,
    /// `fit-content`, or `fit-content()`.
    FitContent,
    /// `stretch`.
    Stretch,
}

/// The cause of a box's extent on one axis, by the stated rule: first match wins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SizeCause {
    /// The computed `width` or `height`, resolved (CSS 2 §10.2, §10.5).
    Style,
    /// Clamped up to the computed minimum (css-sizing-3 §5.2).
    Min,
    /// Clamped down to the computed maximum (css-sizing-3 §5.2).
    Max,
    /// Grown past its flex basis (css-flexbox-1 §9.7).
    FlexGrow,
    /// Shrunk below its flex basis (css-flexbox-1 §9.7).
    FlexShrink,
    /// Stretched to fill: a block's auto width (CSS 2 §10.3.3), a flex item's cross size
    /// (css-flexbox-1 §9.4 step 11), or an out-of-flow box between two insets (CSS 2 §10.3.7).
    Stretch,
    /// Sized by its grid area (css-grid-2 §11).
    Grid,
    /// Sized by its table.
    Table,
    /// A replaced element's or a leaf control's intrinsic size.
    Replaced,
    /// Its content.
    Content,
    /// The layout, no one cause singled out: an open list's row, an option of a closed select,
    /// or any box no earlier rule matches.
    Layout,
}

/// A physical axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    Width,
    Height,
}

/// Lengths equal to 1/64 px, the grid client rects snap to.
fn same(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0 / 64.0
}

fn snap(value: f64) -> f64 {
    (value * 64.0).round() / 64.0
}

fn point(pos: crate::util::Point<f32>) -> (f64, f64) {
    (pos.x as f64, pos.y as f64)
}

/// What [`BaseDocument::layout_reason`] reads of one element before it decides causes.
struct Read<'a> {
    node: &'a Node,
    style: &'a ComputedValues,
    by: PlacedBy,
    /// The box layout placed the element in, before anonymous blocks are folded.
    parent: Option<&'a Node>,
    /// The element's border box: physical size, padding and border.
    layout: taffy::Layout,
    laid_out: BoundingRect,
}

impl BaseDocument {
    /// Why `node_id` sits where it does at its size, read from the layout and style the engine
    /// already holds.
    ///
    /// `None` for an id that does not resolve, a node that is not an element, an element with
    /// no computed styles and one with no box (`display: none`, `display: contents`, or held by
    /// a `display: none` ancestor) — but for an option of a closed select, read at its select's
    /// box, and a row of an open list. Reads layout and style only and changes nothing; its
    /// numbers are unrounded layout snapped to 1/64 px, as client rects read.
    pub fn layout_reason(&self, node_id: NodeId) -> Option<LayoutReason> {
        let node = self.get_node(node_id)?;
        if !node.is_element() {
            return None;
        }
        if let Some(reason) = self.select_reason(node) {
            return Some(reason);
        }
        if !self.has_box(node) {
            return None;
        }
        let styles = node.primary_styles()?;
        let style: &ComputedValues = &styles;
        let (pos, layout) = self.physical_unrounded_geometry(node_id);
        let non_atomic = node.is_non_atomic_inline();
        let laid_out = if non_atomic {
            self.inline_fragment_rects(node_id)?
                .reduce(BoundingRect::union)?
        } else {
            let scroll = self.viewport_scroll();
            BoundingRect {
                x: snap(pos.x as f64 - scroll.x),
                y: snap(pos.y as f64 - scroll.y),
                width: snap(layout.size.width as f64),
                height: snap(layout.size.height as f64),
            }
        };
        let parent = self.placing_parent(node).and_then(|id| self.get_node(id));
        let read = Read {
            node,
            style,
            by: self.placed_by(node, style, parent),
            parent,
            layout,
            laid_out,
        };
        let bounds = self.get_client_bounding_rect(node_id)?;
        Some(LayoutReason {
            bounds,
            laid_out,
            placed: self.placed(&read),
            shifts: self.shifts(node_id, non_atomic),
            width: self.size_reason(&read, Axis::Width, non_atomic),
            height: self.size_reason(&read, Axis::Height, non_atomic),
        })
    }

    /// An option of a closed drop-down select, read at its select's box, or a row of an open
    /// list, read at the list's row; `None` for any other node.
    fn select_reason(&self, node: &Node) -> Option<LayoutReason> {
        let element = node.element_data()?;
        if element.name.local != local_name!("option") {
            return None;
        }
        let select = self.select_of(node.id)?;
        if let Some(row) = self.select_list_bounds(node.id) {
            let at = self.get_client_bounding_rect(select)?;
            return Some(fixed_reason(
                row,
                row,
                PlacedBy::SelectList,
                select,
                (snap(row.x - at.x), snap(row.y - at.y)),
                Vec::new(),
            ));
        }
        if self.is_list_box(select) || self.open_select() == Some(select) {
            return None;
        }
        let bounds = self.get_client_bounding_rect(select)?;
        let shown = self.layout_reason(select)?;
        Some(fixed_reason(
            bounds,
            shown.laid_out,
            PlacedBy::Select,
            select,
            (0.0, 0.0),
            shown.shifts,
        ))
    }

    /// Whether `node` generates a box: in the document, and neither it nor an ancestor
    /// `display: none`, nor itself `display: contents`.
    fn has_box(&self, node: &Node) -> bool {
        node.flags.is_in_document()
            && !node
                .primary_styles()
                .is_some_and(|style| style.clone_display().is_contents())
            && !node.under_display_none()
    }

    /// The box `node` was placed in: a non-atomic inline's inline root, else its containing
    /// block.
    fn placing_parent(&self, node: &Node) -> Option<NodeId> {
        if node.is_non_atomic_inline() {
            return node.inline_root_ancestor().map(|root| root.id);
        }
        node.containing_block()
    }

    /// The algorithm that placed `node`, in the stated order.
    fn placed_by(&self, node: &Node, style: &ComputedValues, parent: Option<&Node>) -> PlacedBy {
        if node.is_out_of_flow() {
            return match style.clone_position() {
                Position::Fixed => PlacedBy::Fixed,
                _ => PlacedBy::Absolute,
            };
        }
        if style.clone_float() != Float::None {
            return PlacedBy::Float;
        }
        if style.clone_display().outside() == DisplayOutside::Inline {
            return PlacedBy::Inline;
        }
        if parent.is_some_and(|parent| parent.flags.is_table_root()) {
            return PlacedBy::Table;
        }
        if self.try_root_element().map(|root| root.id) == Some(node.id) {
            return PlacedBy::Root;
        }
        match parent.map(|parent| parent.taffy_display()) {
            Some(taffy::Display::Flex) => PlacedBy::Flex,
            Some(taffy::Display::Grid) => PlacedBy::Grid,
            _ => PlacedBy::Block,
        }
    }

    /// The container, the offset in it and the relative inset of the element `read` holds.
    fn placed(&self, read: &Read) -> Placed {
        let scroll = self.viewport_scroll();
        let origin = (read.laid_out.x + scroll.x, read.laid_out.y + scroll.y);
        let mut held = (0.0, 0.0);
        let mut container = None;
        let mut current = read.parent;
        while let Some(holder) = current {
            if !holder.is_element() && !holder.is_anonymous() {
                break;
            }
            let offset = holder.scroll_offset();
            held.0 += offset.x;
            held.1 += offset.y;
            if holder.is_element() {
                container = Some(holder.id);
                break;
            }
            current = holder.containing_block().and_then(|id| self.get_node(id));
        }
        let base = match container {
            Some(id) => point(self.physical_unrounded_geometry(id).0),
            None => (0.0, 0.0),
        };
        Placed {
            by: read.by,
            container,
            at: (
                snap(origin.0 - base.0 + held.0),
                snap(origin.1 - base.1 + held.1),
            ),
            relative: self.relative_inset(read),
        }
    }

    /// The `position: relative` inset of the element `read` holds, resolved against its
    /// containing block.
    fn relative_inset(&self, read: &Read) -> Option<(f64, f64)> {
        if read.style.clone_position() != Position::Relative {
            return None;
        }
        let basis = self.basis(read);
        let inset = read.style.get_position();
        let side = |start: &GenericInset<_, LengthPercentage>,
                    end: &GenericInset<_, LengthPercentage>,
                    basis: f32| match (start, end) {
            (GenericInset::LengthPercentage(lp), _) => resolve(lp, basis),
            (_, GenericInset::LengthPercentage(lp)) => -resolve(lp, basis),
            _ => 0.0,
        };
        Some((
            snap(side(&inset.left, &inset.right, basis.0) as f64),
            snap(side(&inset.top, &inset.bottom, basis.1) as f64),
        ))
    }

    /// Each box that moves `node_id` from where layout put it: a box on its chain with a
    /// transform (itself included) or a scroll offset (itself excluded), innermost first, then
    /// the viewport's scroll.
    fn shifts(&self, node_id: NodeId, non_atomic: bool) -> Vec<Shift> {
        let mut shifts = Vec::new();
        let mut current = Some(node_id);
        let mut own = true;
        while let Some(id) = current {
            let Some(holder) = self.get_node(id) else {
                break;
            };
            if !holder.is_element() && !holder.is_anonymous() {
                break;
            }
            if !own {
                shifts.extend(scroll_shift(Some(id), *holder.scroll_offset()));
            }
            if holder.transform().is_some() && !(own && non_atomic) {
                shifts.push(Shift {
                    by: ShiftBy::Transform,
                    holder: Some(id),
                    dx: 0.0,
                    dy: 0.0,
                });
            }
            current = match (own, non_atomic) {
                (true, true) => holder.inline_root_ancestor().map(|root| root.id),
                _ => holder.containing_block(),
            };
            own = false;
        }
        shifts.extend(scroll_shift(None, self.viewport_scroll()));
        shifts
    }

    /// The percentage basis of the element `read` holds: its containing block's padding box
    /// for an out-of-flow box, else its content box; the viewport for the root.
    fn basis(&self, read: &Read) -> (f32, f32) {
        let Some(parent) = read
            .parent
            .filter(|parent| parent.is_element() || parent.is_anonymous())
        else {
            let size = self.viewport.window_size;
            let scale = self.viewport.scale();
            return (size.0 as f32 / scale, size.1 as f32 / scale);
        };
        let (_, layout) = self.physical_unrounded_geometry(parent.id);
        let border = layout.border;
        let mut width = layout.size.width - border.left - border.right;
        let mut height = layout.size.height - border.top - border.bottom;
        if !read.node.is_out_of_flow() {
            let padding = layout.padding;
            width -= padding.left + padding.right + layout.scrollbar_size.width;
            height -= padding.top + padding.bottom + layout.scrollbar_size.height;
        }
        (width.max(0.0), height.max(0.0))
    }

    /// The cause of the extent of the element `read` holds on `axis`.
    fn size_reason(&self, read: &Read, axis: Axis, non_atomic: bool) -> SizeReason {
        let basis = match axis {
            Axis::Width => self.basis(read).0,
            Axis::Height => self.basis(read).1,
        };
        let position = read.style.get_position();
        let (size, min, max) = match axis {
            Axis::Width => (&position.width, &position.min_width, &position.max_width),
            Axis::Height => (&position.height, &position.min_height, &position.max_height),
        };
        let px = match axis {
            Axis::Width => read.laid_out.width,
            Axis::Height => read.laid_out.height,
        };
        let flex = self.flex_inputs(read, axis, basis);
        let inputs = Inputs {
            read,
            axis,
            px,
            basis,
            size: length_of_size(size, basis),
            min: length_of_size(min, basis),
            max: length_of_max(max, basis),
            flex,
        };
        let by = if non_atomic {
            SizeCause::Content
        } else {
            self.size_cause(&inputs)
        };
        SizeReason {
            px,
            by,
            size: inputs.size,
            min: inputs.min,
            max: inputs.max,
            flex: inputs.flex,
        }
    }

    /// The flex inputs of the element `read` holds where `axis` is its flex container's main
    /// axis.
    fn flex_inputs(&self, read: &Read, axis: Axis, basis: f32) -> Option<FlexInputs> {
        let parent = read.parent?;
        if parent.taffy_display() != taffy::Display::Flex || read.node.is_out_of_flow() {
            return None;
        }
        if main_axis(parent)? != axis {
            return None;
        }
        let position = read.style.get_position();
        let flex_basis = match &position.flex_basis {
            GenericFlexBasis::Content => Length::Auto,
            GenericFlexBasis::Size(size) => length_of_size(size, basis),
        };
        Some(FlexInputs {
            basis: flex_basis,
            grow: position.flex_grow.0,
            shrink: position.flex_shrink.0,
        })
    }

    /// The stated rule, first match wins.
    fn size_cause(&self, inputs: &Inputs) -> SizeCause {
        if let Some(cause) = leaf_cause(inputs) {
            return cause;
        }
        if inputs.read.by == PlacedBy::Table {
            return SizeCause::Table;
        }
        if let Some(cause) = clamp_cause(inputs) {
            return cause;
        }
        if let Some(cause) = flex_cause(inputs) {
            return cause;
        }
        auto_cause(inputs)
    }
}

/// The shift a scroll offset of `holder` (the viewport's for `None`) makes; `None` for no
/// offset.
fn scroll_shift(holder: Option<NodeId>, scroll: crate::Point<f64>) -> Option<Shift> {
    (scroll.x != 0.0 || scroll.y != 0.0).then_some(Shift {
        by: ShiftBy::Scroll,
        holder,
        dx: -scroll.x,
        dy: -scroll.y,
    })
}

/// A reason whose numbers its placer gives, its causes [`SizeCause::Layout`].
fn fixed_reason(
    bounds: BoundingRect,
    laid_out: BoundingRect,
    by: PlacedBy,
    container: NodeId,
    at: (f64, f64),
    shifts: Vec<Shift>,
) -> LayoutReason {
    let layout = |px: f64| SizeReason {
        px,
        by: SizeCause::Layout,
        size: Length::Auto,
        min: Length::Auto,
        max: Length::None,
        flex: None,
    };
    LayoutReason {
        bounds,
        laid_out,
        placed: Placed {
            by,
            container: Some(container),
            at,
            relative: None,
        },
        shifts,
        width: layout(bounds.width),
        height: layout(bounds.height),
    }
}

/// What the size rule reads on one axis.
struct Inputs<'a, 'r> {
    read: &'r Read<'a>,
    axis: Axis,
    px: f64,
    basis: f32,
    size: Length,
    min: Length,
    max: Length,
    flex: Option<FlexInputs>,
}

impl Inputs<'_, '_> {
    /// `length` resolved to the border box it sets, where it is definite: a percentage of a
    /// height only against a container whose height is not `auto`.
    fn resolved(&self, length: &Length) -> Option<f64> {
        let content = match *length {
            Length::Px(px) => px,
            Length::Calc { px } => px,
            Length::Percent(percent) if self.percent_definite() => percent / 100.0 * self.basis,
            _ => return None,
        };
        let layout = &self.read.layout;
        let edges = match self.axis {
            Axis::Width => {
                layout.padding.left
                    + layout.padding.right
                    + layout.border.left
                    + layout.border.right
            }
            Axis::Height => {
                layout.padding.top
                    + layout.padding.bottom
                    + layout.border.top
                    + layout.border.bottom
            }
        };
        let border_box = match self.read.style.get_position().box_sizing {
            BoxSizing::ContentBox => content + edges,
            BoxSizing::BorderBox => content.max(edges),
        };
        Some(border_box as f64)
    }

    fn percent_definite(&self) -> bool {
        if self.axis == Axis::Width {
            return true;
        }
        self.read.parent.is_none_or(|parent| {
            parent
                .primary_styles()
                .is_some_and(|style| !matches!(style.get_position().height, GenericSize::Auto))
        })
    }

    fn equals(&self, length: &Length) -> bool {
        self.resolved(length).is_some_and(|px| same(self.px, px))
    }
}

/// Rule 2: a replaced element or a leaf control.
fn leaf_cause(inputs: &Inputs) -> Option<SizeCause> {
    let element = inputs.read.node.element_data()?;
    let name = &element.name.local;
    let leaf_input = *name == local_name!("input")
        && matches!(
            element.attr(local_name!("type")),
            None | Some("text" | "password" | "email" | "tel" | "url" | "search" | "checkbox")
        );
    if !is_replaced_element(name) && *name != local_name!("textarea") && !leaf_input {
        return None;
    }
    Some(if inputs.equals(&inputs.size) {
        SizeCause::Style
    } else {
        SizeCause::Replaced
    })
}

/// Rules 4 to 6: the stated size, then the maximum, then a minimum above 0.
fn clamp_cause(inputs: &Inputs) -> Option<SizeCause> {
    if inputs.equals(&inputs.size) {
        return Some(SizeCause::Style);
    }
    if inputs.equals(&inputs.max) {
        return Some(SizeCause::Max);
    }
    let min_above_zero = inputs.resolved(&inputs.min).is_some_and(|min| {
        let edges = inputs.resolved(&Length::Px(0.0)).unwrap_or(0.0);
        min > edges
    });
    (min_above_zero && inputs.equals(&inputs.min)).then_some(SizeCause::Min)
}

/// Rules 7 and 8: a flex item's main size against its basis, its cross size stretched.
fn flex_cause(inputs: &Inputs) -> Option<SizeCause> {
    if let Some(flex) = &inputs.flex {
        let basis = inputs.resolved(&flex.basis)?;
        if inputs.px > basis && !same(inputs.px, basis) && flex.grow > 0.0 {
            return Some(SizeCause::FlexGrow);
        }
        if inputs.px < basis && !same(inputs.px, basis) && flex.shrink > 0.0 {
            return Some(SizeCause::FlexShrink);
        }
        return None;
    }
    let parent = inputs.read.parent?;
    if inputs.read.by != PlacedBy::Flex || inputs.size != Length::Auto {
        return None;
    }
    if main_axis(parent)? == inputs.axis || !stretches(inputs.read.style, parent) {
        return None;
    }
    let line = cross_extent(parent, inputs.axis);
    let layout = &inputs.read.layout;
    let margins = match inputs.axis {
        Axis::Width => layout.margin.left + layout.margin.right,
        Axis::Height => layout.margin.top + layout.margin.bottom,
    };
    same(inputs.px, (line - margins) as f64).then_some(SizeCause::Stretch)
}

/// Rules 9 to 12: an `auto` size by what placed the box.
fn auto_cause(inputs: &Inputs) -> SizeCause {
    if inputs.size != Length::Auto {
        return SizeCause::Layout;
    }
    match inputs.read.by {
        PlacedBy::Grid => SizeCause::Grid,
        PlacedBy::Block if inputs.axis == Axis::Width => SizeCause::Stretch,
        PlacedBy::Absolute | PlacedBy::Fixed if both_insets(inputs.read.style, inputs.axis) => {
            SizeCause::Stretch
        }
        _ => SizeCause::Content,
    }
}

/// Whether both insets of `axis` are set.
fn both_insets(style: &ComputedValues, axis: Axis) -> bool {
    let position = style.get_position();
    let set = |inset: &GenericInset<_, _>| !matches!(inset, GenericInset::Auto);
    match axis {
        Axis::Width => set(&position.left) && set(&position.right),
        Axis::Height => set(&position.top) && set(&position.bottom),
    }
}

/// The main axis of the flex container `parent`; `None` for a node with no styles.
fn main_axis(parent: &Node) -> Option<Axis> {
    let style = parent.primary_styles()?;
    Some(match style.get_position().flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse => Axis::Width,
        FlexDirection::Column | FlexDirection::ColumnReverse => Axis::Height,
    })
}

/// Whether an item of `style` in the flex container `parent` self-aligns to stretch (`normal`
/// behaves as `stretch` for a flex item, css-align-3 §6.1).
fn stretches(style: &ComputedValues, parent: &Node) -> bool {
    let own = style.get_position().align_self.0.value();
    let flags = if own == AlignFlags::AUTO {
        match parent.primary_styles() {
            Some(parent) => parent.get_position().align_items.0.value(),
            None => AlignFlags::NORMAL,
        }
    } else {
        own
    };
    flags == AlignFlags::NORMAL || flags == AlignFlags::STRETCH
}

/// The content-box extent of the flex container `parent` on `axis`: its one line's cross
/// extent.
fn cross_extent(parent: &Node, axis: Axis) -> f32 {
    let layout = parent.unrounded_layout();
    let (size, start, end, scrollbar) = match axis {
        Axis::Width => (
            layout.size.width,
            layout.border.left + layout.padding.left,
            layout.border.right + layout.padding.right,
            layout.scrollbar_size.width,
        ),
        Axis::Height => (
            layout.size.height,
            layout.border.top + layout.padding.top,
            layout.border.bottom + layout.padding.bottom,
            layout.scrollbar_size.height,
        ),
    };
    (size - start - end - scrollbar).max(0.0)
}

/// `lp` resolved against `basis`; a `calc()` through the reviewed resolver.
fn resolve(lp: &LengthPercentage, basis: f32) -> f32 {
    match lp.unpack() {
        Unpacked::Length(length) => length.px(),
        Unpacked::Percentage(percentage) => percentage.0 * basis,
        Unpacked::Calc(calc) => stylo_taffy::convert::resolve_calc_value(
            taffy::CalcHandle::from_ptr(calc as *const CalcLengthPercentage as *const ()),
            basis,
        ),
    }
}

fn length_of(lp: &LengthPercentage, basis: f32) -> Length {
    match lp.unpack() {
        Unpacked::Length(length) => Length::Px(length.px()),
        Unpacked::Percentage(percentage) => Length::Percent(percentage.0 * 100.0),
        Unpacked::Calc(_) => Length::Calc {
            px: resolve(lp, basis),
        },
    }
}

fn length_of_size(size: &GenericSize<NonNegative<LengthPercentage>>, basis: f32) -> Length {
    match size {
        GenericSize::LengthPercentage(lp) => length_of(&lp.0, basis),
        GenericSize::MinContent => Length::Keyword(SizeKeyword::MinContent),
        GenericSize::MaxContent => Length::Keyword(SizeKeyword::MaxContent),
        GenericSize::FitContent | GenericSize::FitContentFunction(_) => {
            Length::Keyword(SizeKeyword::FitContent)
        }
        GenericSize::Stretch | GenericSize::WebkitFillAvailable => {
            Length::Keyword(SizeKeyword::Stretch)
        }
        // `auto`, and anchor sizes, which the engine does not lay out.
        _ => Length::Auto,
    }
}

fn length_of_max(max: &GenericMaxSize<NonNegative<LengthPercentage>>, basis: f32) -> Length {
    match max {
        GenericMaxSize::LengthPercentage(lp) => length_of(&lp.0, basis),
        GenericMaxSize::MinContent => Length::Keyword(SizeKeyword::MinContent),
        GenericMaxSize::MaxContent => Length::Keyword(SizeKeyword::MaxContent),
        GenericMaxSize::FitContent | GenericMaxSize::FitContentFunction(_) => {
            Length::Keyword(SizeKeyword::FitContent)
        }
        GenericMaxSize::Stretch | GenericMaxSize::WebkitFillAvailable => {
            Length::Keyword(SizeKeyword::Stretch)
        }
        // `none`, and anchor sizes, which the engine does not lay out.
        _ => Length::None,
    }
}
