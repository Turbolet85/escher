//! The list a single-choice `select` opens: what it holds, the rows it reads from the select's
//! options and groups, where it stands, and the colours paint draws it in.
//!
//! The list is an overlay and lays nothing out. Its state names DOM nodes only, its rows are
//! derived each time they are read, and paint, hit testing and the snapshot read one geometry.

use blitz_traits::node_id::NodeId;
use markup5ever::local_name;
use style::computed_values::visibility::T as Visibility;

use super::{Node, select};
use crate::{BaseDocument, BoundingRect};

/// The most rows a list shows before it scrolls.
pub(crate) const MAX_ROWS: usize = 20;
/// The width of the list's border, in CSS px.
pub(crate) const BORDER_WIDTH: f64 = 1.0;
/// A row's padding above and below its text, in CSS px.
pub(crate) const ROW_PADDING_Y: f64 = 1.0;
/// A row's padding before and after its text, in CSS px.
pub(crate) const ROW_PADDING_X: f64 = 4.0;
/// How far an option under a group's header is indented past the header, in CSS px.
pub(crate) const GROUP_INDENT: f64 = 8.0;

/// The most rows a `size` attribute asks for, as HTML's rendering section caps a list box.
const MAX_SIZE: u32 = 65534;

/// An open list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SelectList {
    /// The select that opened it.
    pub(crate) select: NodeId,
    /// The highlighted option; `None` where no option is enabled.
    pub(crate) current: Option<NodeId>,
    /// How far the rows are scrolled, in CSS px.
    pub(crate) scroll_y: f64,
    /// The height of every row, measured at opening, in CSS px.
    pub(crate) row_height: f64,
    /// The width the rows' text asks for, measured at opening, in CSS px.
    pub(crate) width: f64,
    /// The select's client rect, which the list stays open only while the select keeps.
    pub(crate) anchor: BoundingRect,
    /// Whether no layout has run since opening: a click focuses the select and opens the list in
    /// one step, so a `:focus` rule that moves its box has not been applied to `anchor` yet.
    pub(crate) fresh: bool,
}

/// One row of a list.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Row {
    /// The `option`, or the `optgroup` a header stands for.
    pub(crate) node: NodeId,
    /// Whether the row is a group's header, which nothing chooses.
    pub(crate) header: bool,
    /// Whether the row is an option under a group's header.
    pub(crate) indented: bool,
}

/// Where a list stands, and how many rows it shows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Placement {
    /// The list's border box, relative to the viewport.
    pub(crate) bounds: BoundingRect,
    /// How many rows fit its scrollport.
    pub(crate) shown: usize,
}

/// The one geometry paint, hit testing and the snapshot read, relative to the viewport.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Geometry {
    pub(crate) rows: Vec<Row>,
    pub(crate) bounds: BoundingRect,
    pub(crate) scrollport: BoundingRect,
    pub(crate) row_height: f64,
    pub(crate) scroll_y: f64,
}

/// What paint reads of an open list, relative to the viewport, in CSS px.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectListView {
    /// The select that opened the list.
    pub select: NodeId,
    /// The list's border box.
    pub bounds: BoundingRect,
    /// The part of the list its rows show through.
    pub scrollport: BoundingRect,
    /// The rows that reach the scrollport, in order.
    pub rows: Vec<SelectListRow>,
    /// The scroll thumb, where the rows pass the scrollport.
    pub thumb: Option<BoundingRect>,
}

/// One row of a [`SelectListView`].
#[derive(Debug, Clone, PartialEq)]
pub struct SelectListRow {
    /// The `option`, or the `optgroup` a header stands for.
    pub node: NodeId,
    /// The row's box.
    pub bounds: BoundingRect,
    /// Where its text starts.
    pub text_x: f64,
    /// The text it shows: an option's label or a group's `label`.
    pub label: String,
    /// Whether the row is the highlighted one.
    pub highlighted: bool,
    /// Whether the row reads disabled: an option's own attribute or its group's.
    pub disabled: bool,
}

impl SelectListView {
    /// A highlighted row's text: `#000000` on [`Self::HIGHLIGHT_BACKGROUND`], the pair a chosen
    /// row of a `multiple` select reads, 12.56:1.
    pub const HIGHLIGHT_TEXT: [u8; 3] = [0x00, 0x00, 0x00];
    /// A highlighted row's background, Chromium's inactive selection colour.
    pub const HIGHLIGHT_BACKGROUND: [u8; 3] = [0xc8, 0xc8, 0xc8];
    /// A disabled row's text: dimmed past a plain row and still above 4.5:1 on white.
    pub const DISABLED_TEXT: [u8; 3] = [0x6d, 0x6d, 0x6d];
    /// The list's border: the default sheet's border of a `select`, on every page, since the
    /// engine holds no tokens to read a page's own from.
    pub const BORDER: [u8; 3] = [0x99, 0x99, 0x99];
    /// The width of the border, in CSS px.
    pub const BORDER_WIDTH: f64 = BORDER_WIDTH;
}

/// The `size` a `multiple` select's attribute asks for: a whole number above 0, capped; `None` for
/// `0` and for what does not parse.
pub(crate) fn size_rows(value: &str) -> Option<u32> {
    let rows = style::servo::attr::parse_unsigned_integer(value.chars()).ok()?;
    (rows > 0).then_some(rows.min(MAX_SIZE))
}

/// Whether `node` can stand in a list: it carries no `hidden`, it has been styled, and it computes
/// neither `visibility: hidden` nor `collapse`.
fn shows(node: &Node) -> bool {
    let carries_hidden = node
        .element_data()
        .is_none_or(|element| element.has_attr(local_name!("hidden")));
    !carries_hidden
        && node.primary_styles().is_some_and(|style| {
            !matches!(
                style.clone_visibility(),
                Visibility::Hidden | Visibility::Collapse
            )
        })
}

/// A list's rows, in tree order: one header for each `optgroup` with a non-empty `label`, then its
/// options. Derived at each use, so a node dropped since is no row.
pub(crate) fn rows(doc: &BaseDocument, select: NodeId) -> Vec<Row> {
    let Some(holder) = doc.get_node(select) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for child in holder.children.iter().copied() {
        let Some(node) = doc.get_node(child).filter(|node| shows(node)) else {
            continue;
        };
        if node.data.is_element_with_tag_name(&local_name!("option")) {
            rows.push(Row {
                node: child,
                header: false,
                indented: false,
            });
        } else if node.data.is_element_with_tag_name(&local_name!("optgroup")) {
            let header = group_label(node).is_some();
            if header {
                rows.push(Row {
                    node: child,
                    header,
                    indented: false,
                });
            }
            rows.extend(
                node.children
                    .iter()
                    .copied()
                    .filter(|option| {
                        doc.get_node(*option).is_some_and(|option| {
                            option.data.is_element_with_tag_name(&local_name!("option"))
                                && shows(option)
                        })
                    })
                    .map(|option| Row {
                        node: option,
                        header: false,
                        indented: header,
                    }),
            );
        }
    }
    rows
}

/// The non-empty `label` of a group.
pub(crate) fn group_label(node: &Node) -> Option<&str> {
    node.element_data()?
        .attr(local_name!("label"))
        .filter(|label| !label.is_empty())
}

/// The text `row` of `node` shows: an option's label, or a group's `label`.
pub(crate) fn row_label(node: &Node, row: &Row) -> String {
    if row.header {
        group_label(node).unwrap_or_default().to_string()
    } else {
        select::option_label(node)
    }
}

/// The options among `rows`.
pub(crate) fn option_rows(rows: &[Row]) -> Vec<NodeId> {
    rows.iter()
        .filter(|row| !row.header)
        .map(|row| row.node)
        .collect()
}

/// Where a list of `row_count` rows opens from the select at `select`, in a `viewport` of the
/// given width and height: below the select where its rows fit, else on the side with more room;
/// at most [`MAX_ROWS`] rows, and as many as that side holds, at least one; at least as wide as the
/// select and at most as wide as the viewport; moved to stay inside the viewport. A value that is
/// not finite reads 0, so every edge is finite.
pub(crate) fn place(
    select: BoundingRect,
    viewport: (f64, f64),
    row_count: usize,
    row_height: f64,
    width: f64,
) -> Placement {
    let finite = |value: f64| if value.is_finite() { value } else { 0.0 };
    let (viewport_width, viewport_height) =
        (finite(viewport.0).max(0.0), finite(viewport.1).max(0.0));
    let (select_x, select_y) = (finite(select.x), finite(select.y));
    let select_bottom = select_y + finite(select.height);
    let row_height = if row_height.is_finite() && row_height > 0.0 {
        row_height
    } else {
        1.0
    };
    let border = 2.0 * BORDER_WIDTH;
    let wanted = row_count.min(MAX_ROWS);
    let (below, above) = (viewport_height - select_bottom, select_y);
    let opens_below = wanted as f64 * row_height + border <= below || below >= above;
    let room = if opens_below { below } else { above };
    let fits = ((room - border) / row_height).floor().max(1.0) as usize;
    let shown = wanted.min(fits);
    let height = shown as f64 * row_height + border;
    let y = if opens_below {
        select_bottom
    } else {
        select_y - height
    };
    let width = finite(width).max(finite(select.width)).min(viewport_width);
    Placement {
        bounds: BoundingRect {
            x: select_x.min(viewport_width - width).max(0.0),
            y: y.min(viewport_height - height).max(0.0),
            width,
            height,
        },
        shown,
    }
}

/// The geometry of `list`, its scroll offset kept within its rows.
pub(crate) fn geometry(doc: &BaseDocument, list: &SelectList) -> Geometry {
    let rows = rows(doc, list.select);
    let scale = doc.viewport().scale_f64();
    let (width, height) = doc.viewport().window_size;
    let placement = place(
        list.anchor,
        (f64::from(width) / scale, f64::from(height) / scale),
        rows.len(),
        list.row_height,
        list.width,
    );
    let bounds = placement.bounds;
    let scrollport = BoundingRect {
        x: bounds.x + BORDER_WIDTH,
        y: bounds.y + BORDER_WIDTH,
        width: (bounds.width - 2.0 * BORDER_WIDTH).max(0.0),
        height: (bounds.height - 2.0 * BORDER_WIDTH).max(0.0),
    };
    let mut geometry = Geometry {
        rows,
        bounds,
        scrollport,
        row_height: list.row_height,
        scroll_y: 0.0,
    };
    geometry.scroll_y = geometry.clamped(list.scroll_y);
    geometry
}

impl Geometry {
    /// `scroll_y` kept within the rows.
    pub(crate) fn clamped(&self, scroll_y: f64) -> f64 {
        let most = (self.rows.len() as f64 * self.row_height - self.scrollport.height).max(0.0);
        if scroll_y.is_finite() {
            scroll_y.clamp(0.0, most)
        } else {
            0.0
        }
    }

    /// The box of the row at `index`, where it stands past the scrollport too.
    pub(crate) fn row_rect(&self, index: usize) -> BoundingRect {
        BoundingRect {
            x: self.scrollport.x,
            y: self.scrollport.y + index as f64 * self.row_height - self.scroll_y,
            width: self.scrollport.width,
            height: self.row_height,
        }
    }

    /// Whether the point lies on the list, its border included.
    pub(crate) fn holds(&self, x: f64, y: f64) -> bool {
        within(self.bounds, x, y)
    }

    /// The row under the point; `None` on the border.
    pub(crate) fn row_at(&self, x: f64, y: f64) -> Option<&Row> {
        if !within(self.scrollport, x, y) || self.row_height <= 0.0 {
            return None;
        }
        let index = ((y - self.scrollport.y + self.scroll_y) / self.row_height).floor() as usize;
        self.rows.get(index)
    }

    /// The scroll offset that brings the row at `index` wholly into the scrollport by the least
    /// movement.
    pub(crate) fn scroll_showing(&self, index: usize) -> f64 {
        let top = index as f64 * self.row_height;
        let bottom = top + self.row_height;
        let scroll_y = if top < self.scroll_y {
            top
        } else if bottom > self.scroll_y + self.scrollport.height {
            bottom - self.scrollport.height
        } else {
            self.scroll_y
        };
        self.clamped(scroll_y)
    }

    /// The indices of the rows that reach the scrollport.
    pub(crate) fn rows_in_view(&self) -> std::ops::Range<usize> {
        if self.row_height <= 0.0 {
            return 0..0;
        }
        let first = (self.scroll_y / self.row_height).floor() as usize;
        let last = ((self.scroll_y + self.scrollport.height) / self.row_height).ceil() as usize;
        first.min(self.rows.len())..last.min(self.rows.len())
    }

    /// The scroll thumb, its height the scrollport's share of the rows, where they pass it.
    pub(crate) fn thumb(&self) -> Option<BoundingRect> {
        const THUMB_WIDTH: f64 = 6.0;
        const MIN_THUMB_HEIGHT: f64 = 16.0;
        let total = self.rows.len() as f64 * self.row_height;
        let port = self.scrollport;
        if total <= port.height || total <= 0.0 {
            return None;
        }
        let height = (port.height * port.height / total)
            .max(MIN_THUMB_HEIGHT)
            .min(port.height);
        let most = total - port.height;
        Some(BoundingRect {
            x: port.x + port.width - THUMB_WIDTH,
            y: port.y + (port.height - height) * self.scroll_y / most,
            width: THUMB_WIDTH,
            height,
        })
    }
}

fn within(rect: BoundingRect, x: f64, y: f64) -> bool {
    x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
}

/// Whether `node` is an option, or a header's group, whose row reads disabled.
pub(crate) fn row_disabled(node: &Node) -> bool {
    select::option_disabled(node)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: f64, y: f64, width: f64, height: f64) -> BoundingRect {
        BoundingRect {
            x,
            y,
            width,
            height,
        }
    }

    #[test]
    fn a_list_opens_where_its_rows_fit_and_else_on_the_side_with_more_room() {
        // (select, row count, the list's y, its height, the rows it shows)
        let rows = [
            (rect(10.0, 100.0, 80.0, 20.0), 3, 120.0, 62.0, 3),
            (rect(10.0, 500.0, 80.0, 20.0), 3, 520.0, 62.0, 3),
            (rect(10.0, 550.0, 80.0, 20.0), 3, 488.0, 62.0, 3),
            (rect(10.0, 290.0, 80.0, 20.0), 20, 310.0, 282.0, 14),
            (rect(10.0, 400.0, 80.0, 20.0), 30, 18.0, 382.0, 19),
            (rect(10.0, 10.0, 80.0, 20.0), 30, 30.0, 402.0, 20),
            (rect(10.0, 590.0, 80.0, 20.0), 1, 568.0, 22.0, 1),
        ];
        assert_eq!(rows.len(), 7);
        for (index, (select, count, y, height, shown)) in rows.into_iter().enumerate() {
            let placed = place(select, (800.0, 600.0), count, 20.0, 80.0);
            assert_eq!(placed.bounds.y, y, "row {index}");
            assert_eq!(placed.bounds.height, height, "row {index}");
            assert_eq!(placed.shown, shown, "row {index}");
        }
    }

    #[test]
    fn a_list_is_at_least_its_selects_width_at_most_the_viewports_and_stays_inside_it() {
        // (select's x, select's width, the rows' width, the list's x, its width)
        let rows = [
            (10.0, 80.0, 40.0, 10.0, 80.0),
            (10.0, 80.0, 200.0, 10.0, 200.0),
            (700.0, 80.0, 200.0, 600.0, 200.0),
            (10.0, 80.0, 5000.0, 0.0, 800.0),
            (-30.0, 80.0, 40.0, 0.0, 80.0),
        ];
        assert_eq!(rows.len(), 5);
        for (index, (x, select_width, width, list_x, list_width)) in rows.into_iter().enumerate() {
            let select = rect(x, 100.0, select_width, 20.0);
            let placed = place(select, (800.0, 600.0), 2, 20.0, width);
            assert_eq!(placed.bounds.x, list_x, "row {index}");
            assert_eq!(placed.bounds.width, list_width, "row {index}");
        }
    }

    #[test]
    fn every_edge_of_a_placed_list_is_finite() {
        let hostile = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
        for (index, value) in hostile.into_iter().enumerate() {
            let select = rect(value, value, value, value);
            for placed in [
                place(select, (800.0, 600.0), 2, value, value),
                place(select, (value, value), 2, 20.0, 80.0),
            ] {
                let edges = [
                    placed.bounds.x,
                    placed.bounds.y,
                    placed.bounds.width,
                    placed.bounds.height,
                ];
                assert!(edges.iter().all(|edge| edge.is_finite()), "row {index}");
            }
        }
    }

    #[test]
    fn size_reads_a_whole_number_above_zero_capped() {
        let rows = [
            ("3", Some(3)),
            (" 4", Some(4)),
            ("+5", Some(5)),
            ("0", None),
            ("-1", None),
            ("many", None),
            ("", None),
            ("70000", Some(MAX_SIZE)),
        ];
        assert_eq!(rows.len(), 8);
        for (index, (value, expected)) in rows.into_iter().enumerate() {
            assert_eq!(size_rows(value), expected, "row {index}");
        }
    }

    fn geometry_of(count: usize, scroll_y: f64) -> Geometry {
        let mut geometry = Geometry {
            rows: vec![
                Row {
                    node: NodeId::from_u64(1),
                    header: false,
                    indented: false,
                };
                count
            ],
            bounds: rect(9.0, 99.0, 102.0, 62.0),
            scrollport: rect(10.0, 100.0, 100.0, 60.0),
            row_height: 20.0,
            scroll_y: 0.0,
        };
        geometry.scroll_y = geometry.clamped(scroll_y);
        geometry
    }

    #[test]
    fn a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement() {
        let geometry = geometry_of(10, 500.0);
        assert_eq!(geometry.scroll_y, 140.0);
        assert_eq!(geometry_of(2, 500.0).scroll_y, 0.0);
        assert_eq!(geometry_of(10, f64::NAN).scroll_y, 0.0);
        // (scrolled, the row, the offset that shows it)
        let rows = [
            (0.0, 0, 0.0),
            (0.0, 2, 0.0),
            (0.0, 3, 20.0),
            (40.0, 1, 20.0),
            (40.0, 3, 40.0),
            (0.0, 9, 140.0),
        ];
        assert_eq!(rows.len(), 6);
        for (index, (scrolled, row, expected)) in rows.into_iter().enumerate() {
            assert_eq!(
                geometry_of(10, scrolled).scroll_showing(row),
                expected,
                "row {index}"
            );
        }
    }

    #[test]
    fn a_point_reads_the_row_it_lies_on_where_the_rows_are_scrolled() {
        let geometry = geometry_of(10, 40.0);
        let id = |row: &Row| row.node;
        assert_eq!(
            geometry.row_at(50.0, 100.0).map(id),
            Some(NodeId::from_u64(1))
        );
        assert!(geometry.row_at(50.0, 99.5).is_none());
        assert!(geometry.row_at(9.5, 110.0).is_none());
        assert!(geometry.holds(9.5, 110.0));
        assert!(!geometry.holds(8.5, 110.0));
        assert_eq!(geometry.row_rect(2).y, 100.0);
        assert_eq!(geometry.rows_in_view(), 2..5);
        assert_eq!(geometry_of(10, 50.0).rows_in_view(), 2..6);
        assert!(geometry_of(2, 0.0).thumb().is_none());
        assert!(geometry_of(10, 0.0).thumb().is_some());
    }
}
