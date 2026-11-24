//! The draw commands an app's widget paints into a canvas element, read on the held instance by
//! stable id as closed values, numbers and digests: no glyph id, glyph position, text, font or
//! image byte, family name or `NodeId`.

use std::collections::HashMap;

use anyrender::recording::RenderCommand;
use anyrender::{BoxShadowKind, MaskMode, Paint};
use peniko::color::Srgb;
use peniko::kurbo::{Affine, BezPath, PathEl};
use peniko::{BlendMode, Color, Compose, Fill, GradientKind, Mix};

use crate::execute::resolve;
use crate::recording::fnv1a;
use crate::refusal::{Cause, Refusal};
use crate::session::Session;

/// One canvas element's draw commands, as [`Session::canvas`] reads them.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct CanvasReading {
    /// Whether the element holds a widget whose content box is not empty: its commands were
    /// read. False for any other element, which reads a zero size and no command or resource.
    pub canvas: bool,
    /// The width of the element's content box, in CSS px: its border box less border and
    /// padding, floored at 0.
    pub width: f64,
    /// The height of the element's content box, in CSS px.
    pub height: f64,
    /// The commands, in paint order.
    pub commands: Vec<CanvasCommand>,
    /// The FNV-1a 64 digest of each distinct font's bytes, in first-use order; a glyph run names
    /// its font by its index here.
    pub fonts: Vec<u64>,
    /// The FNV-1a 64 digest of each distinct image's bytes, in first-use order; an image brush
    /// names its image by its index here.
    pub images: Vec<u64>,
}

impl CanvasReading {
    /// The reading of an element that holds no widget, or whose content box is empty.
    pub(crate) fn none() -> Self {
        Self {
            canvas: false,
            width: 0.0,
            height: 0.0,
            commands: Vec::new(),
            fonts: Vec::new(),
            images: Vec::new(),
        }
    }
}

/// An affine transform's six coefficients, `[a, b, c, d, e, f]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform(pub [f64; 6]);

impl Transform {
    /// Whether it is the identity.
    pub fn is_identity(&self) -> bool {
        self.0 == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
    }
}

/// One element of a path, with its points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathStep {
    /// Move to a point.
    Move(f64, f64),
    /// A line to a point.
    Line(f64, f64),
    /// A quadratic curve: its control point, then its end.
    Quad(f64, f64, f64, f64),
    /// A cubic curve: its two control points, then its end.
    Cubic(f64, f64, f64, f64, f64, f64),
    /// Close the subpath.
    Close,
}

/// A colour as 8-bit red, green, blue and alpha, unpremultiplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba(pub [u8; 4]);

/// One stop of a gradient: its offset and its colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    /// Its offset, 0 to 1.
    pub offset: f64,
    /// Its colour.
    pub colour: Rgba,
}

/// What a shape or a glyph run is painted with.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CanvasBrush {
    /// A solid colour.
    Solid(Rgba),
    /// A gradient: its kind (`linear`, `radial` or `sweep`) and its stops.
    Gradient {
        /// The kind's word.
        kind: &'static str,
        /// Its stops, in order.
        stops: Vec<GradientStop>,
    },
    /// An image, by its index in [`CanvasReading::images`].
    Image(usize),
    /// A renderer's resource, not read.
    Resource,
    /// A pattern, not read.
    Pattern,
}

/// One draw command, as closed words and numbers. Every word is kebab-case.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CanvasCommand {
    /// A layer pushed: what follows is composed onto what came before through it.
    Layer {
        /// Its opacity.
        alpha: f64,
        /// Its blend mix's word.
        mix: &'static str,
        /// Its blend compose's word.
        compose: &'static str,
        /// Its clip's fill rule's word.
        fill: &'static str,
        /// Its transform.
        transform: Transform,
        /// Its clip path.
        clip: Vec<PathStep>,
        /// Whether a filter is set; its parameters are not read.
        filter: bool,
        /// Whether a backdrop filter is set; its parameters are not read.
        backdrop: bool,
    },
    /// A clip layer pushed.
    Clip {
        /// Its transform.
        transform: Transform,
        /// Its clip path.
        clip: Vec<PathStep>,
    },
    /// A mask layer pushed.
    Mask {
        /// Its mode's word.
        mode: &'static str,
        /// Its transform.
        transform: Transform,
        /// Its clip path.
        clip: Vec<PathStep>,
    },
    /// The innermost layer, clip or mask popped.
    Pop,
    /// A shape filled.
    Fill {
        /// Its fill rule's word.
        fill: &'static str,
        /// What it is filled with.
        brush: CanvasBrush,
        /// The brush's own transform, where one is set.
        brush_transform: Option<Transform>,
        /// Its transform.
        transform: Transform,
        /// The shape's path.
        shape: Vec<PathStep>,
    },
    /// A shape stroked.
    Stroke {
        /// The stroke's width.
        width: f64,
        /// Its join's word.
        join: &'static str,
        /// Its start cap's word.
        start_cap: &'static str,
        /// Its end cap's word.
        end_cap: &'static str,
        /// Its miter limit.
        miter: f64,
        /// Its dash lengths; empty for a solid stroke.
        dashes: Vec<f64>,
        /// Its dash offset.
        dash_offset: f64,
        /// What it is stroked with.
        brush: CanvasBrush,
        /// The brush's own transform, where one is set.
        brush_transform: Option<Transform>,
        /// Its transform.
        transform: Transform,
        /// The shape's path.
        shape: Vec<PathStep>,
    },
    /// A run of glyphs drawn. No glyph id, glyph position or text is held.
    Glyphs {
        /// Its font, by its index in [`CanvasReading::fonts`].
        font: usize,
        /// The font size.
        size: f64,
        /// What it is painted with.
        brush: CanvasBrush,
        /// The brush's opacity.
        alpha: f64,
        /// Its transform.
        transform: Transform,
        /// How many glyphs it draws.
        count: usize,
        /// The box of its glyph origins: least x, least y, greatest x, greatest y; `None` for a
        /// run of no glyph.
        origins: Option<[f64; 4]>,
        /// The FNV-1a 64 digest of its glyph ids, each as four little-endian bytes.
        ids: u64,
    },
    /// A box shadow drawn.
    Shadow {
        /// Its transform.
        transform: Transform,
        /// The box: least x, least y, greatest x, greatest y.
        rect: [f64; 4],
        /// Its corner radii, x then y, top left, top right, bottom right, bottom left.
        radii: [f64; 8],
        /// Its offset, x then y.
        offset: (f64, f64),
        /// Its spread.
        spread: f64,
        /// Its blur's standard deviation.
        deviation: f64,
        /// Its colour.
        colour: Rgba,
        /// Its kind's word.
        kind: &'static str,
    },
}

/// The words of the eight command kinds, in the order a summary counts them.
pub const COMMAND_KINDS: [&str; 8] = [
    "layer", "clip", "mask", "pop", "fill", "stroke", "glyphs", "shadow",
];

impl CanvasCommand {
    /// The index of the command's kind in [`COMMAND_KINDS`].
    pub fn kind(&self) -> usize {
        match self {
            CanvasCommand::Layer { .. } => 0,
            CanvasCommand::Clip { .. } => 1,
            CanvasCommand::Mask { .. } => 2,
            CanvasCommand::Pop => 3,
            CanvasCommand::Fill { .. } => 4,
            CanvasCommand::Stroke { .. } => 5,
            CanvasCommand::Glyphs { .. } => 6,
            CanvasCommand::Shadow { .. } => 7,
        }
    }

    /// Whether it pushes a layer, a clip or a mask, which a [`CanvasCommand::Pop`] closes.
    pub fn opens(&self) -> bool {
        self.kind() < 3
    }
}

impl Session {
    /// The draw commands the widget of the element `id` names paints, read on the held instance.
    ///
    /// The id resolves as a call's does, refused [`Cause::Stale`] or [`Cause::NotFound`]; an app
    /// with no font it can paint text with is then refused [`Cause::FontsUnavailable`], before
    /// anything is painted, since a glyph run's text would be skipped. The widget is painted once
    /// more into a recording at the viewport's scale, outside any raster, from the layout the
    /// last settle left. It runs no step, pump or settle and changes nothing a screen reads. An
    /// element with no widget, or whose content box is empty, reads `canvas` false. The `canvas`
    /// verb reaches it through [`Session::run`].
    pub fn canvas(&mut self, id: &str) -> Result<CanvasReading, Refusal> {
        let (_, target) = resolve(&self.harness, &mut self.seen, id)?;
        if !self.harness.base().can_paint_text() {
            return Err(Refusal::new(Cause::FontsUnavailable));
        }
        let node = target.node();
        let Some(scene) = self.harness.canvas_scene(node) else {
            return Ok(CanvasReading::none());
        };
        let (width, height) = self
            .harness
            .base()
            .get_node(node)
            .map_or((0.0, 0.0), |node| {
                let layout = node.final_layout();
                let width = layout.size.width
                    - layout.border.left
                    - layout.border.right
                    - layout.padding.left
                    - layout.padding.right;
                let height = layout.size.height
                    - layout.border.top
                    - layout.border.bottom
                    - layout.padding.top
                    - layout.padding.bottom;
                (f64::from(width.max(0.0)), f64::from(height.max(0.0)))
            });
        let mut reading = read_commands(&scene.commands);
        reading.width = width;
        reading.height = height;
        Ok(reading)
    }
}

/// The reading of recorded `commands`, its size zero.
pub(crate) fn read_commands(commands: &[RenderCommand]) -> CanvasReading {
    let mut resources = Resources::default();
    let commands = commands
        .iter()
        .map(|command| read_command(command, &mut resources))
        .collect();
    CanvasReading {
        canvas: true,
        width: 0.0,
        height: 0.0,
        commands,
        fonts: resources.fonts.digests,
        images: resources.images.digests,
    }
}

/// The distinct resources a reading names, each by its index.
#[derive(Default)]
struct Resources {
    fonts: Indexed,
    images: Indexed,
}

/// Digests in first-use order, with the blobs already read, so a resource's bytes are digested
/// once per blob however many commands name it.
#[derive(Default)]
struct Indexed {
    digests: Vec<u64>,
    by_blob: HashMap<u64, usize>,
}

impl Indexed {
    /// The index of the resource whose bytes `bytes` are, held by the blob `blob`.
    fn index(&mut self, blob: u64, bytes: &[u8]) -> usize {
        if let Some(index) = self.by_blob.get(&blob) {
            return *index;
        }
        let digest = fnv1a(bytes);
        let index = match self.digests.iter().position(|seen| *seen == digest) {
            Some(index) => index,
            None => {
                self.digests.push(digest);
                self.digests.len() - 1
            }
        };
        self.by_blob.insert(blob, index);
        index
    }
}

fn read_command(command: &RenderCommand, resources: &mut Resources) -> CanvasCommand {
    match command {
        RenderCommand::PushLayer(layer) => CanvasCommand::Layer {
            alpha: f64::from(layer.alpha),
            mix: mix_word(layer.blend),
            compose: compose_word(layer.blend),
            fill: fill_word(layer.fill),
            transform: transform(layer.transform),
            clip: path(&layer.clip),
            filter: layer.filter.is_some(),
            backdrop: layer.backdrop_filter.is_some(),
        },
        RenderCommand::PushClipLayer(clip) => CanvasCommand::Clip {
            transform: transform(clip.transform),
            clip: path(&clip.clip),
        },
        RenderCommand::PushMaskLayer(mask) => CanvasCommand::Mask {
            mode: match mask.mode {
                MaskMode::Alpha => "alpha",
                MaskMode::Luminance => "luminance",
            },
            transform: transform(mask.transform),
            clip: path(&mask.clip),
        },
        RenderCommand::PopLayer => CanvasCommand::Pop,
        RenderCommand::Fill(fill) => CanvasCommand::Fill {
            fill: fill_word(fill.fill),
            brush: brush(&fill.brush, resources),
            brush_transform: fill.brush_transform.map(transform),
            transform: transform(fill.transform),
            shape: path(&fill.shape),
        },
        RenderCommand::Stroke(stroke) => CanvasCommand::Stroke {
            width: stroke.style.width,
            join: match stroke.style.join {
                peniko::kurbo::Join::Bevel => "bevel",
                peniko::kurbo::Join::Miter => "miter",
                peniko::kurbo::Join::Round => "round",
            },
            start_cap: cap_word(stroke.style.start_cap),
            end_cap: cap_word(stroke.style.end_cap),
            miter: stroke.style.miter_limit,
            dashes: stroke.style.dash_pattern.to_vec(),
            dash_offset: stroke.style.dash_offset,
            brush: brush(&stroke.brush, resources),
            brush_transform: stroke.brush_transform.map(transform),
            transform: transform(stroke.transform),
            shape: path(&stroke.shape),
        },
        RenderCommand::GlyphRun(run) => {
            let font = resources
                .fonts
                .index(run.font_data.data.id(), run.font_data.data.data());
            let origins = run
                .glyphs
                .iter()
                .fold(None, |bounds: Option<[f64; 4]>, glyph| {
                    let (x, y) = (f64::from(glyph.x), f64::from(glyph.y));
                    Some(match bounds {
                        None => [x, y, x, y],
                        Some([x0, y0, x1, y1]) => [x0.min(x), y0.min(y), x1.max(x), y1.max(y)],
                    })
                });
            let ids: Vec<u8> = run
                .glyphs
                .iter()
                .flat_map(|glyph| glyph.id.to_le_bytes())
                .collect();
            CanvasCommand::Glyphs {
                font,
                size: f64::from(run.font_size),
                brush: brush(&run.brush, resources),
                alpha: f64::from(run.brush_alpha),
                transform: transform(run.transform),
                count: run.glyphs.len(),
                origins,
                ids: fnv1a(&ids),
            }
        }
        RenderCommand::BoxShadow(shadow) => {
            let rect = shadow.box_shape.rect;
            let radii = shadow.box_shape.radii;
            CanvasCommand::Shadow {
                transform: transform(shadow.transform),
                rect: [rect.x0, rect.y0, rect.x1, rect.y1],
                radii: [
                    radii.top_left.x,
                    radii.top_left.y,
                    radii.top_right.x,
                    radii.top_right.y,
                    radii.bottom_right.x,
                    radii.bottom_right.y,
                    radii.bottom_left.x,
                    radii.bottom_left.y,
                ],
                offset: (shadow.offset.x, shadow.offset.y),
                spread: shadow.spread,
                deviation: shadow.std_dev,
                colour: rgba(shadow.color),
                kind: match shadow.kind {
                    BoxShadowKind::Outset => "outset",
                    BoxShadowKind::OutsetUnclipped => "outset-unclipped",
                    BoxShadowKind::Inset => "inset",
                },
            }
        }
    }
}

fn transform(affine: Affine) -> Transform {
    Transform(affine.as_coeffs())
}

fn path(path: &BezPath) -> Vec<PathStep> {
    path.elements()
        .iter()
        .map(|element| match *element {
            PathEl::MoveTo(p) => PathStep::Move(p.x, p.y),
            PathEl::LineTo(p) => PathStep::Line(p.x, p.y),
            PathEl::QuadTo(a, b) => PathStep::Quad(a.x, a.y, b.x, b.y),
            PathEl::CurveTo(a, b, c) => PathStep::Cubic(a.x, a.y, b.x, b.y, c.x, c.y),
            PathEl::ClosePath => PathStep::Close,
        })
        .collect()
}

fn rgba(colour: Color) -> Rgba {
    let c = colour.to_rgba8();
    Rgba([c.r, c.g, c.b, c.a])
}

fn brush(paint: &Paint, resources: &mut Resources) -> CanvasBrush {
    match paint {
        Paint::Solid(colour) => CanvasBrush::Solid(rgba(*colour)),
        Paint::Gradient(gradient) => CanvasBrush::Gradient {
            kind: match gradient.kind {
                GradientKind::Linear(_) => "linear",
                GradientKind::Radial(_) => "radial",
                GradientKind::Sweep(_) => "sweep",
            },
            stops: gradient
                .stops
                .iter()
                .map(|stop| GradientStop {
                    offset: f64::from(stop.offset),
                    colour: rgba(stop.color.to_alpha_color::<Srgb>()),
                })
                .collect(),
        },
        Paint::Image(image) => CanvasBrush::Image(
            resources
                .images
                .index(image.image.data.id(), image.image.data.data()),
        ),
        Paint::Resource(_) => CanvasBrush::Resource,
        // A recording keeps no custom paint: it records one as transparent solid.
        Paint::Custom(_) => CanvasBrush::Solid(Rgba([0, 0, 0, 0])),
        Paint::Pattern(_) => CanvasBrush::Pattern,
    }
}

fn fill_word(fill: Fill) -> &'static str {
    match fill {
        Fill::NonZero => "non-zero",
        Fill::EvenOdd => "even-odd",
    }
}

fn cap_word(cap: peniko::kurbo::Cap) -> &'static str {
    match cap {
        peniko::kurbo::Cap::Butt => "butt",
        peniko::kurbo::Cap::Square => "square",
        peniko::kurbo::Cap::Round => "round",
    }
}

fn mix_word(blend: BlendMode) -> &'static str {
    match blend.mix {
        Mix::Normal => "normal",
        Mix::Multiply => "multiply",
        Mix::Screen => "screen",
        Mix::Overlay => "overlay",
        Mix::Darken => "darken",
        Mix::Lighten => "lighten",
        Mix::ColorDodge => "color-dodge",
        Mix::ColorBurn => "color-burn",
        Mix::HardLight => "hard-light",
        Mix::SoftLight => "soft-light",
        Mix::Difference => "difference",
        Mix::Exclusion => "exclusion",
        Mix::Hue => "hue",
        Mix::Saturation => "saturation",
        Mix::Color => "color",
        Mix::Luminosity => "luminosity",
        #[allow(unreachable_patterns)]
        _ => "other",
    }
}

fn compose_word(blend: BlendMode) -> &'static str {
    match blend.compose {
        Compose::Clear => "clear",
        Compose::Copy => "copy",
        Compose::Dest => "dest",
        Compose::SrcOver => "src-over",
        Compose::DestOver => "dest-over",
        Compose::SrcIn => "src-in",
        Compose::DestIn => "dest-in",
        Compose::SrcOut => "src-out",
        Compose::DestOut => "dest-out",
        Compose::SrcAtop => "src-atop",
        Compose::DestAtop => "dest-atop",
        Compose::Xor => "xor",
        Compose::Plus => "plus",
        Compose::PlusLighter => "plus-lighter",
        #[allow(unreachable_patterns)]
        _ => "other",
    }
}

#[cfg(test)]
#[path = "canvas_tests.rs"]
mod tests;
