//! The clip box each push leaves and each pop restores, over hand-built recorded scenes of fills,
//! clips, layers and masks with no font: a clip narrows the box and its pop restores the one
//! beneath, nested pushes intersect, a push's transform maps its path before the box is taken,
//! a layer's and a mask's paths cut as a clip's does, and a pop on an empty stack is no panic.
//! A glyph run whose font skrifa cannot read is no label. A child module of `canvas_lint`.

use anyrender::{AntiAliasing, Glyph, MaskMode, PaintScene as _};
use peniko::kurbo::Vec2;
use peniko::{BlendMode, Blob, Color, Fill, Style};

use super::*;

const RED: Color = Color::from_rgba8(255, 0, 0, 255);

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Rect {
    Rect::new(x0, y0, x1, y1)
}

fn fill(scene: &mut anyrender::Scene) {
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        RED,
        None,
        &rect(0.0, 0.0, 5.0, 5.0),
        AntiAliasing::Enabled,
    );
}

#[test]
fn a_clip_narrows_the_box_and_its_pop_restores_the_one_beneath() {
    let mut scene = anyrender::Scene::new();
    fill(&mut scene);
    scene.push_clip_layer(
        Fill::NonZero,
        Affine::IDENTITY,
        &rect(0.0, 0.0, 100.0, 40.0),
    );
    fill(&mut scene);
    scene.push_clip_layer(
        Fill::NonZero,
        Affine::IDENTITY,
        &rect(10.0, 0.0, 50.0, 40.0),
    );
    fill(&mut scene);
    scene.pop_layer();
    fill(&mut scene);
    scene.pop_layer();
    fill(&mut scene);
    let rows = [
        None,
        Some(rect(0.0, 0.0, 100.0, 40.0)),
        Some(rect(0.0, 0.0, 100.0, 40.0)),
        Some(rect(10.0, 0.0, 50.0, 40.0)),
        Some(rect(10.0, 0.0, 50.0, 40.0)),
        Some(rect(0.0, 0.0, 100.0, 40.0)),
        Some(rect(0.0, 0.0, 100.0, 40.0)),
        None,
        None,
    ];
    let boxes = clip_boxes(&scene.commands);
    assert_eq!(boxes.len(), rows.len());
    for (row, expected) in rows.iter().enumerate() {
        assert!(boxes[row] == *expected, "row {row}");
    }
}

#[test]
fn nested_pushes_intersect() {
    let mut scene = anyrender::Scene::new();
    scene.push_clip_layer(
        Fill::NonZero,
        Affine::IDENTITY,
        &rect(0.0, 0.0, 100.0, 40.0),
    );
    scene.push_clip_layer(
        Fill::NonZero,
        Affine::IDENTITY,
        &rect(50.0, 20.0, 150.0, 80.0),
    );
    fill(&mut scene);
    let boxes = clip_boxes(&scene.commands);
    assert_eq!(boxes.len(), 3);
    assert!(boxes[2] == Some(rect(50.0, 20.0, 100.0, 40.0)));
}

#[test]
fn a_pushs_transform_maps_its_path_before_the_box_is_taken() {
    let mut scene = anyrender::Scene::new();
    let moved = Affine::translate((30.0, 5.0)) * Affine::scale(2.0);
    scene.push_clip_layer(Fill::NonZero, moved, &rect(0.0, 0.0, 10.0, 10.0));
    fill(&mut scene);
    let boxes = clip_boxes(&scene.commands);
    assert_eq!(boxes.len(), 2);
    assert!(boxes[1] == Some(rect(30.0, 5.0, 50.0, 25.0)));
}

#[test]
fn a_layers_and_a_masks_paths_cut_as_a_clips_does() {
    let mut scene = anyrender::Scene::new();
    scene.push_layer(
        Fill::NonZero,
        BlendMode::default(),
        1.0,
        Affine::IDENTITY,
        &rect(0.0, 0.0, 60.0, 60.0),
        None,
        None,
    );
    fill(&mut scene);
    scene.push_mask_layer(
        Fill::NonZero,
        MaskMode::Alpha,
        Affine::IDENTITY,
        &rect(20.0, 30.0, 90.0, 90.0),
    );
    fill(&mut scene);
    scene.pop_layer();
    fill(&mut scene);
    let rows = [
        Some(rect(0.0, 0.0, 60.0, 60.0)),
        Some(rect(0.0, 0.0, 60.0, 60.0)),
        Some(rect(20.0, 30.0, 60.0, 60.0)),
        Some(rect(20.0, 30.0, 60.0, 60.0)),
        Some(rect(0.0, 0.0, 60.0, 60.0)),
        Some(rect(0.0, 0.0, 60.0, 60.0)),
    ];
    let boxes = clip_boxes(&scene.commands);
    assert_eq!(boxes.len(), rows.len());
    for (row, expected) in rows.iter().enumerate() {
        assert!(boxes[row] == *expected, "row {row}");
    }
}

#[test]
fn a_pop_on_an_empty_stack_is_no_panic() {
    let mut scene = anyrender::Scene::new();
    scene.pop_layer();
    fill(&mut scene);
    scene.pop_layer();
    let boxes = clip_boxes(&scene.commands);
    assert_eq!(boxes, vec![None, None, None]);
}

#[test]
fn a_glyph_run_whose_font_cannot_be_read_is_no_label() {
    let scene = anyrender::Scene {
        commands: vec![RenderCommand::GlyphRun(GlyphRunCommand {
            font_data: peniko::FontData::new(Blob::from(vec![0u8; 16]), 0),
            font_size: 14.0,
            hint: false,
            normalized_coords: Vec::new(),
            embolden: Vec2::ZERO,
            style: Style::Fill(Fill::NonZero),
            brush: anyrender::Paint::Solid(RED),
            brush_alpha: 1.0,
            transform: Affine::IDENTITY,
            glyph_transform: None,
            glyphs: vec![Glyph {
                id: 3,
                x: 1.0,
                y: 1.0,
            }],
            anti_aliasing: AntiAliasing::Enabled,
        })],
        ..anyrender::Scene::default()
    };
    assert!(labels(&scene.commands).is_empty());
}
