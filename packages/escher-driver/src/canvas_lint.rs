//! The labels a canvas widget paints, as the lint reads them from a recorded scene: each glyph
//! run's ink box and the box its enclosing clip, layer and mask pushes leave, both in the
//! scene's device px. No glyph id, glyph position or text leaves this module.

use anyrender::recording::{GlyphRunCommand, RenderCommand};
use peniko::kurbo::{Affine, BezPath, Rect, Shape as _};
use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::{FontRef, GlyphId, MetadataProvider as _};

/// One glyph run of a recorded scene, in the scene's device px.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Label {
    /// The run's ink box: the union of its glyphs' outline bounds.
    pub(crate) ink: Rect,
    /// The box its enclosing pushes leave; `None` where no push encloses it.
    pub(crate) clip: Option<Rect>,
}

/// Every glyph run of `commands` with an ink box that is not empty, in paint order.
pub(crate) fn labels(commands: &[RenderCommand]) -> Vec<Label> {
    commands
        .iter()
        .zip(clip_boxes(commands))
        .filter_map(|(command, clip)| match command {
            RenderCommand::GlyphRun(run) => ink(run).map(|ink| Label { ink, clip }),
            _ => None,
        })
        .collect()
}

/// The box the pushes standing after each command leave, one per command: a push's own clip
/// path's bounding box, mapped through its transform and cut to the box beneath it.
fn clip_boxes(commands: &[RenderCommand]) -> Vec<Option<Rect>> {
    let mut stack: Vec<Rect> = Vec::new();
    commands
        .iter()
        .map(|command| {
            let pushed = match command {
                RenderCommand::PushLayer(layer) => Some((layer.transform, &layer.clip)),
                RenderCommand::PushClipLayer(clip) => Some((clip.transform, &clip.clip)),
                RenderCommand::PushMaskLayer(mask) => Some((mask.transform, &mask.clip)),
                RenderCommand::PopLayer => {
                    stack.pop();
                    None
                }
                _ => None,
            };
            if let Some((transform, path)) = pushed {
                let own = path_box(transform, path);
                let cut = stack.last().map_or(own, |beneath| own.intersect(*beneath));
                stack.push(cut);
            }
            stack.last().copied()
        })
        .collect()
}

/// The bounding box of `path` mapped through `transform`.
fn path_box(transform: Affine, path: &BezPath) -> Rect {
    (transform * path.clone()).bounding_box()
}

/// The ink box of `run`: each glyph's outline bounds, read through skrifa from the run's own
/// font at its size and normalized coordinates, placed at its origin, widened by the run's
/// embolden and mapped through its glyph transform and its transform. `None` when no glyph
/// adds an outline: an empty run, glyphs with no outline, a font skrifa cannot read, or glyph
/// ids past its count.
fn ink(run: &GlyphRunCommand) -> Option<Rect> {
    let font = FontRef::from_index(run.font_data.data.data(), run.font_data.index).ok()?;
    let coords: Vec<NormalizedCoord> = run
        .normalized_coords
        .iter()
        .map(|coord| NormalizedCoord::from_bits(*coord))
        .collect();
    let metrics = font.glyph_metrics(Size::new(run.font_size), LocationRef::new(&coords));
    let glyph_transform = run.glyph_transform.unwrap_or(Affine::IDENTITY);
    let (wide, tall) = (run.embolden.x, run.embolden.y);
    run.glyphs
        .iter()
        .filter_map(|glyph| {
            let bounds = metrics.bounds(GlyphId::new(glyph.id))?;
            if bounds.x_max <= bounds.x_min || bounds.y_max <= bounds.y_min {
                return None;
            }
            // The outline's box is y-up about the origin; the scene is y-down.
            let local = Rect::new(
                f64::from(bounds.x_min) - wide,
                -f64::from(bounds.y_max) - tall,
                f64::from(bounds.x_max) + wide,
                -f64::from(bounds.y_min) + tall,
            );
            let origin = Affine::translate((f64::from(glyph.x), f64::from(glyph.y)));
            Some((run.transform * origin * glyph_transform).transform_rect_bbox(local))
        })
        .reduce(|all, one| all.union(one))
}

#[cfg(test)]
#[path = "canvas_lint_tests.rs"]
mod tests;
