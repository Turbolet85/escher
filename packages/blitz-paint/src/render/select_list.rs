//! A single-choice select's open list: an engine overlay painted last, above every stacking level
//! and outside every effect layer and clip, from the one read
//! [`BaseDocument::select_list_view`](blitz_dom::BaseDocument::select_list_view).

use anyrender::PaintScene;
use blitz_dom::{BoundingRect, SelectListRow, SelectListView};
use kurbo::{Affine, Rect, Stroke};
use peniko::Fill;

use super::BlitzDomPainter;
use crate::color::{Color, ToColorColor as _};
use crate::text::RunPaint;

/// The colour of one of the list's fixed `[r, g, b]` values.
fn fixed(rgb: [u8; 3]) -> Color {
    let [r, g, b] = rgb;
    Color::from_rgba8(r, g, b, 255)
}

impl BlitzDomPainter<'_, '_> {
    /// Paints the list that stands open, if any.
    pub(super) fn paint_select_list(&self, scene: &mut impl PaintScene) {
        let Some(view) = self.dom.select_list_view() else {
            return;
        };
        let Some(bounds) = self.device_rect(view.bounds) else {
            return;
        };
        let Some(scrollport) = self.device_rect(view.scrollport) else {
            return;
        };
        self.paint_list_box(scene, &view, bounds);
        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &scrollport);
        for row in &view.rows {
            self.paint_list_row(scene, row);
        }
        #[cfg(feature = "scrollbars")]
        self.paint_list_thumb(scene, &view);
        scene.pop_layer();
    }

    /// `rect`, in CSS px relative to the viewport, as device pixels on the scene; `None` for a box
    /// with an edge that is not finite, which is not painted.
    fn device_rect(&self, rect: BoundingRect) -> Option<Rect> {
        let device = Rect::new(
            self.initial_x + rect.x * self.scale,
            self.initial_y + rect.y * self.scale,
            self.initial_x + (rect.x + rect.width) * self.scale,
            self.initial_y + (rect.y + rect.height) * self.scale,
        );
        device.is_finite().then_some(device)
    }

    /// The list's background and border. The background is the select's own over opaque white, so
    /// no page shows through the list.
    fn paint_list_box(&self, scene: &mut impl PaintScene, view: &SelectListView, bounds: Rect) {
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(255, 255, 255, 255),
            None,
            &bounds,
        );
        if let Some(styles) = self
            .dom
            .get_node(view.select)
            .and_then(|node| node.primary_styles())
        {
            let background = styles
                .clone_background_color()
                .resolve_to_absolute(&styles.clone_color())
                .as_srgb_color();
            scene.fill(Fill::NonZero, Affine::IDENTITY, background, None, &bounds);
        }
        let width = SelectListView::BORDER_WIDTH * self.scale;
        scene.stroke(
            &Stroke::new(width),
            Affine::IDENTITY,
            fixed(SelectListView::BORDER),
            None,
            &bounds.inset(-width / 2.0),
        );
    }

    /// One row: the highlight behind a highlighted one, then its text, clipped to the row so that
    /// a larger font or a longer label than the row holds crosses no other.
    fn paint_list_row(&self, scene: &mut impl PaintScene, row: &SelectListRow) {
        let Some(bounds) = self.device_rect(row.bounds) else {
            return;
        };
        if row.highlighted {
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                fixed(SelectListView::HIGHLIGHT_BACKGROUND),
                None,
                &bounds,
            );
        }
        let Some(layout) = self.dom.select_row_layout(row.node, &row.label) else {
            return;
        };
        let color = if row.highlighted {
            Some(fixed(SelectListView::HIGHLIGHT_TEXT))
        } else if row.disabled {
            Some(fixed(SelectListView::DISABLED_TEXT))
        } else {
            None
        };
        let inset = (bounds.height() - f64::from(layout.height())) / 2.0;
        let transform =
            Affine::translate((self.initial_x + row.text_x * self.scale, bounds.y0 + inset));
        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &bounds);
        crate::text::stroke_text_as(
            scene,
            layout.lines(),
            self.dom,
            transform,
            self.scale,
            row.node,
            &mut self.draw_text_context.borrow_mut(),
            RunPaint {
                color,
                synthesize_bold: true,
            },
        );
        scene.pop_layer();
    }

    /// The scroll thumb, painted and not interactive, in the default palette for the used scheme at
    /// the opacity the select's overlay scrollbars read.
    #[cfg(feature = "scrollbars")]
    fn paint_list_thumb(&self, scene: &mut impl PaintScene, view: &SelectListView) {
        let opacity = self.dom.scrollbar_opacity(view.select);
        if opacity == 0.0 {
            return;
        }
        let Some(thumb) = view.thumb.and_then(|thumb| self.device_rect(thumb)) else {
            return;
        };
        let dark = self.dom.viewport().color_scheme == blitz_traits::shell::ColorScheme::Dark;
        let color = if dark {
            Color::from_rgba8(214, 214, 214, 178)
        } else {
            Color::from_rgba8(128, 128, 128, 178)
        };
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            color.multiply_alpha(opacity),
            None,
            &thumb.to_rounded_rect(thumb.width() / 2.0),
        );
    }
}
