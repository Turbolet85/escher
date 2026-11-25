//! Reads hand-built recorded scenes: each command kind and each brush kind as its closed value,
//! a font named once by index however many runs use it, an empty scene, and a glyph run whose
//! ids change changing its digest alone. A child module of `canvas`, so its tests keep their
//! `canvas::tests` paths.

use std::sync::Arc;

use anyrender::recording::{
    BoxShadowCommand, ClipCommand, FillCommand, GlyphRunCommand, LayerCommand, MaskLayerCommand,
    StrokeCommand,
};
use anyrender::{
    AntiAliasing, Filter, Glyph, NonUniformRoundedRect, NonUniformRoundedRectRadii, Pattern,
    ResourceId,
};
use peniko::kurbo::{Rect, Stroke, Vec2};
use peniko::{
    Blob, Gradient, ImageAlphaType, ImageBrush, ImageData, ImageFormat, ImageSampler, Style,
};

use super::*;

const RED: Color = Color::from_rgba8(255, 0, 0, 255);
const BLUE: Color = Color::from_rgba8(0, 0, 255, 128);

fn square() -> BezPath {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((10.0, 0.0));
    path.quad_to((10.0, 5.0), (10.0, 10.0));
    path.curve_to((5.0, 10.0), (0.0, 5.0), (0.0, 0.0));
    path.close_path();
    path
}

fn square_steps() -> Vec<PathStep> {
    vec![
        PathStep::Move(0.0, 0.0),
        PathStep::Line(10.0, 0.0),
        PathStep::Quad(10.0, 5.0, 10.0, 10.0),
        PathStep::Cubic(5.0, 10.0, 0.0, 5.0, 0.0, 0.0),
        PathStep::Close,
    ]
}

const SHIFT: Affine = Affine::new([1.0, 0.0, 0.0, 1.0, 5.0, 7.0]);
const SHIFTED: Transform = Transform([1.0, 0.0, 0.0, 1.0, 5.0, 7.0]);
const ID: Transform = Transform([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);

fn font(bytes: &[u8]) -> peniko::FontData {
    peniko::FontData::new(Blob::from(bytes.to_vec()), 0)
}

fn fill(brush: Paint) -> RenderCommand {
    RenderCommand::Fill(FillCommand {
        fill: Fill::NonZero,
        transform: Affine::IDENTITY,
        brush,
        brush_transform: None,
        shape: square(),
        anti_aliasing: AntiAliasing::Enabled,
    })
}

fn glyphs(font_data: peniko::FontData, ids: &[u32]) -> RenderCommand {
    RenderCommand::GlyphRun(GlyphRunCommand {
        font_data,
        font_size: 14.0,
        hint: false,
        normalized_coords: Vec::new(),
        embolden: Vec2::ZERO,
        style: Style::Fill(Fill::NonZero),
        brush: Paint::Solid(RED),
        brush_alpha: 1.0,
        transform: SHIFT,
        glyph_transform: None,
        glyphs: ids
            .iter()
            .enumerate()
            .map(|(index, id)| Glyph {
                id: *id,
                x: 3.0 + 8.0 * index as f32,
                y: 20.0 - index as f32,
            })
            .collect(),
        anti_aliasing: AntiAliasing::Enabled,
    })
}

fn image(bytes: &[u8]) -> Paint {
    Paint::Image(ImageBrush {
        image: ImageData {
            data: Blob::from(bytes.to_vec()),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width: 1,
            height: 1,
        },
        sampler: ImageSampler::default(),
    })
}

/// The FNV-1a 64 of glyph ids, each as four little-endian bytes.
fn ids_digest(ids: &[u32]) -> u64 {
    let bytes: Vec<u8> = ids.iter().flat_map(|id| id.to_le_bytes()).collect();
    fnv1a(&bytes)
}

#[test]
fn each_command_kind_reads_as_its_closed_value() {
    let font_bytes = b"a font";
    let stroke = Stroke::new(2.5)
        .with_join(peniko::kurbo::Join::Bevel)
        .with_start_cap(peniko::kurbo::Cap::Square)
        .with_end_cap(peniko::kurbo::Cap::Round)
        .with_miter_limit(3.0)
        .with_dashes(1.5, [4.0, 2.0]);
    let rows: [(RenderCommand, CanvasCommand); 8] = [
        (
            RenderCommand::PushLayer(LayerCommand {
                fill: Fill::EvenOdd,
                blend: BlendMode::new(Mix::Multiply, Compose::SrcAtop),
                alpha: 0.5,
                transform: SHIFT,
                clip: square(),
                filter: Some(Arc::new(Filter::new(Rect::new(0.0, 0.0, 1.0, 1.0)))),
                backdrop_filter: None,
            }),
            CanvasCommand::Layer {
                alpha: 0.5,
                mix: "multiply",
                compose: "src-atop",
                fill: "even-odd",
                transform: SHIFTED,
                clip: square_steps(),
                filter: true,
                backdrop: false,
            },
        ),
        (
            RenderCommand::PushClipLayer(ClipCommand {
                fill: Fill::NonZero,
                transform: Affine::IDENTITY,
                clip: square(),
            }),
            CanvasCommand::Clip {
                transform: ID,
                clip: square_steps(),
            },
        ),
        (
            RenderCommand::PushMaskLayer(MaskLayerCommand {
                fill: Fill::NonZero,
                mode: MaskMode::Luminance,
                transform: SHIFT,
                clip: square(),
            }),
            CanvasCommand::Mask {
                mode: "luminance",
                transform: SHIFTED,
                clip: square_steps(),
            },
        ),
        (RenderCommand::PopLayer, CanvasCommand::Pop),
        (
            RenderCommand::Fill(FillCommand {
                fill: Fill::EvenOdd,
                transform: SHIFT,
                brush: Paint::Solid(BLUE),
                brush_transform: Some(SHIFT),
                shape: square(),
                anti_aliasing: AntiAliasing::Enabled,
            }),
            CanvasCommand::Fill {
                fill: "even-odd",
                brush: CanvasBrush::Solid(Rgba([0, 0, 255, 128])),
                brush_transform: Some(SHIFTED),
                transform: SHIFTED,
                shape: square_steps(),
            },
        ),
        (
            RenderCommand::Stroke(StrokeCommand {
                style: stroke,
                transform: Affine::IDENTITY,
                brush: Paint::Solid(RED),
                brush_transform: None,
                shape: square(),
                anti_aliasing: AntiAliasing::Enabled,
            }),
            CanvasCommand::Stroke {
                width: 2.5,
                join: "bevel",
                start_cap: "square",
                end_cap: "round",
                miter: 3.0,
                dashes: vec![4.0, 2.0],
                dash_offset: 1.5,
                brush: CanvasBrush::Solid(Rgba([255, 0, 0, 255])),
                brush_transform: None,
                transform: ID,
                shape: square_steps(),
            },
        ),
        (
            glyphs(font(font_bytes), &[36, 61, 59]),
            CanvasCommand::Glyphs {
                font: 0,
                size: 14.0,
                brush: CanvasBrush::Solid(Rgba([255, 0, 0, 255])),
                alpha: 1.0,
                transform: SHIFTED,
                count: 3,
                origins: Some([3.0, 18.0, 19.0, 20.0]),
                ids: ids_digest(&[36, 61, 59]),
            },
        ),
        (
            RenderCommand::BoxShadow(BoxShadowCommand {
                transform: SHIFT,
                box_shape: NonUniformRoundedRect::new(
                    Rect::new(1.0, 2.0, 30.0, 40.0),
                    NonUniformRoundedRectRadii {
                        top_left: Vec2::new(1.0, 2.0),
                        top_right: Vec2::new(3.0, 4.0),
                        bottom_right: Vec2::new(5.0, 6.0),
                        bottom_left: Vec2::new(7.0, 8.0),
                    },
                ),
                offset: Vec2::new(2.0, -3.0),
                spread: 1.0,
                std_dev: 4.0,
                color: BLUE,
                kind: BoxShadowKind::Inset,
            }),
            CanvasCommand::Shadow {
                transform: SHIFTED,
                rect: [1.0, 2.0, 30.0, 40.0],
                radii: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
                offset: (2.0, -3.0),
                spread: 1.0,
                deviation: 4.0,
                colour: Rgba([0, 0, 255, 128]),
                kind: "inset",
            },
        ),
    ];
    assert_eq!(rows.len(), COMMAND_KINDS.len());
    let (commands, expected): (Vec<_>, Vec<_>) = rows.into_iter().unzip();
    let reading = read_commands(&commands);
    assert!(reading.canvas);
    for (row, (read, stated)) in reading.commands.iter().zip(&expected).enumerate() {
        assert!(read == stated, "row {row}");
        assert_eq!(read.kind(), row, "row {row}");
        assert_eq!(read.opens(), row < 3, "row {row}");
    }
    assert_eq!(reading.commands.len(), expected.len());
    assert_eq!(reading.fonts, [fnv1a(font_bytes)]);
    assert!(reading.images.is_empty());
}

#[test]
fn each_brush_kind_reads_as_its_closed_value() {
    let stops = [RED, BLUE];
    let rows: [(Paint, CanvasBrush); 7] = [
        (
            Paint::Solid(RED),
            CanvasBrush::Solid(Rgba([255, 0, 0, 255])),
        ),
        (
            Paint::Gradient(Gradient::new_linear((0.0, 0.0), (1.0, 0.0)).with_stops(stops)),
            CanvasBrush::Gradient {
                kind: "linear",
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        colour: Rgba([255, 0, 0, 255]),
                    },
                    GradientStop {
                        offset: 1.0,
                        colour: Rgba([0, 0, 255, 128]),
                    },
                ],
            },
        ),
        (
            Paint::Gradient(Gradient::new_radial((0.0, 0.0), 4.0).with_stops(stops)),
            CanvasBrush::Gradient {
                kind: "radial",
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        colour: Rgba([255, 0, 0, 255]),
                    },
                    GradientStop {
                        offset: 1.0,
                        colour: Rgba([0, 0, 255, 128]),
                    },
                ],
            },
        ),
        (
            Paint::Gradient(Gradient::new_sweep((0.0, 0.0), 0.0, 1.0).with_stops(stops)),
            CanvasBrush::Gradient {
                kind: "sweep",
                stops: vec![
                    GradientStop {
                        offset: 0.0,
                        colour: Rgba([255, 0, 0, 255]),
                    },
                    GradientStop {
                        offset: 1.0,
                        colour: Rgba([0, 0, 255, 128]),
                    },
                ],
            },
        ),
        (image(b"an image"), CanvasBrush::Image(0)),
        (
            Paint::Resource(ImageBrush {
                image: ResourceId::from_ffi(7),
                sampler: ImageSampler::default(),
            }),
            CanvasBrush::Resource,
        ),
        (
            Paint::Pattern(Pattern::new(
                anyrender::Scene::new(),
                Rect::new(0.0, 0.0, 4.0, 4.0),
            )),
            CanvasBrush::Pattern,
        ),
    ];
    assert_eq!(rows.len(), 7);
    let (paints, expected): (Vec<_>, Vec<_>) = rows.into_iter().unzip();
    let commands: Vec<_> = paints.into_iter().map(fill).collect();
    let reading = read_commands(&commands);
    for (row, (read, stated)) in reading.commands.iter().zip(&expected).enumerate() {
        let CanvasCommand::Fill { brush, .. } = read else {
            panic!("row {row} reads a fill");
        };
        assert!(brush == stated, "row {row}");
    }
    assert_eq!(reading.images, [fnv1a(b"an image")]);
}

#[test]
fn two_runs_of_one_font_name_it_once_and_a_second_font_or_image_takes_the_next_index() {
    let first = font(b"font one");
    let commands = [
        glyphs(first.clone(), &[1, 2]),
        glyphs(first, &[3]),
        // The same bytes in another blob are the same font.
        glyphs(font(b"font one"), &[4]),
        glyphs(font(b"font two"), &[5]),
        fill(image(b"image one")),
        fill(image(b"image two")),
        fill(image(b"image one")),
    ];
    let reading = read_commands(&commands);
    let fonts: Vec<usize> = reading
        .commands
        .iter()
        .filter_map(|command| match command {
            CanvasCommand::Glyphs { font, .. } => Some(*font),
            _ => None,
        })
        .collect();
    assert_eq!(fonts, [0, 0, 0, 1]);
    assert_eq!(reading.fonts, [fnv1a(b"font one"), fnv1a(b"font two")]);
    let images: Vec<CanvasBrush> = reading
        .commands
        .iter()
        .filter_map(|command| match command {
            CanvasCommand::Fill { brush, .. } => Some(brush.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        images,
        [
            CanvasBrush::Image(0),
            CanvasBrush::Image(1),
            CanvasBrush::Image(0)
        ]
    );
    assert_eq!(reading.images, [fnv1a(b"image one"), fnv1a(b"image two")]);
}

#[test]
fn an_empty_scene_reads_a_canvas_with_no_command_and_none_reads_no_canvas() {
    let reading = read_commands(&[]);
    assert!(reading.canvas);
    assert!(reading.commands.is_empty());
    assert!(reading.fonts.is_empty() && reading.images.is_empty());
    let none = CanvasReading::none();
    assert!(!none.canvas);
    assert_eq!((none.width, none.height), (0.0, 0.0));
    assert!(none.commands.is_empty() && none.fonts.is_empty() && none.images.is_empty());
    // A run of no glyph has no origins' box.
    let empty = read_commands(&[glyphs(font(b"f"), &[])]);
    assert!(matches!(
        empty.commands.as_slice(),
        [CanvasCommand::Glyphs {
            count: 0,
            origins: None,
            ..
        }]
    ));
}

/// A 2 × 2 CSS px square filled at `centre`, drawn under `scale` as a widget paints.
fn fill_at(centre: (f64, f64), scale: f64) -> RenderCommand {
    RenderCommand::Fill(FillCommand {
        fill: Fill::NonZero,
        transform: Affine::scale(scale),
        brush: Paint::Solid(RED),
        brush_transform: None,
        shape: Rect::new(
            centre.0 - 1.0,
            centre.1 - 1.0,
            centre.0 + 1.0,
            centre.1 + 1.0,
        )
        .to_path(0.1),
        anti_aliasing: AntiAliasing::Enabled,
    })
}

fn region(x: f64, y: f64) -> BoundingRect {
    BoundingRect {
        x,
        y,
        width: 20.0,
        height: 20.0,
    }
}

#[test]
fn a_command_is_grouped_under_the_region_holding_its_centre_whose_centre_lies_nearest() {
    let a = region(0.0, 0.0);
    let b = region(10.0, 10.0);
    let c = region(40.0, 0.0);
    let clip = RenderCommand::PushClipLayer(ClipCommand {
        fill: Fill::NonZero,
        transform: Affine::IDENTITY,
        clip: square(),
    });
    let rows: [(&str, RenderCommand, Vec<BoundingRect>, f64, Option<usize>); 6] = [
        (
            "in one region",
            fill_at((5.0, 5.0), 1.0),
            vec![a, b],
            1.0,
            Some(1),
        ),
        (
            "in two, the nearer centre",
            fill_at((17.0, 17.0), 1.0),
            vec![a, b],
            1.0,
            Some(2),
        ),
        (
            "a tie, the first declared",
            fill_at((50.0, 10.0), 1.0),
            vec![c, c],
            1.0,
            Some(1),
        ),
        (
            "in none",
            fill_at((100.0, 100.0), 1.0),
            vec![a, b],
            1.0,
            None,
        ),
        ("a layer line", clip, vec![a, b], 1.0, None),
        (
            "a scale of 2",
            fill_at((17.0, 17.0), 2.0),
            vec![a, b],
            2.0,
            Some(2),
        ),
    ];
    assert_eq!(rows.len(), 6);
    for (name, command, regions, scale, expected) in rows {
        let grouped = group(std::slice::from_ref(&command), &regions, scale);
        assert!(grouped == [expected], "{name}");
    }
}

#[test]
fn changed_glyph_ids_change_the_digest_and_nothing_else() {
    let before = read_commands(&[glyphs(font(b"f"), &[36, 61, 59])]);
    let after = read_commands(&[glyphs(font(b"f"), &[36, 61, 60])]);
    let ([CanvasCommand::Glyphs { ids: was, .. }], [CanvasCommand::Glyphs { ids: now, .. }]) =
        (before.commands.as_slice(), after.commands.as_slice())
    else {
        panic!("one glyph run each");
    };
    assert_ne!(was, now);
    let mut restored = after.clone();
    if let [CanvasCommand::Glyphs { ids, .. }] = restored.commands.as_mut_slice() {
        *ids = *was;
    }
    assert_eq!(restored, before);
}
