//! Drives `render_scene` over a parsed document into anyrender's no-op painter and pins the size it
//! returns: the viewport's physical window size, whatever the scale. A child module of `capture`, so
//! it reads `render_scene` and `RenderSize` privately and its tests keep their `capture::tests` paths.

use std::sync::Arc;

use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};

use super::*;

const HTML: &str = r#"<!DOCTYPE html>
<html><body><p>capture</p></body></html>
"#;

fn make_doc(width: u32, height: u32, scale: f32) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(
        HTML,
        DocumentConfig {
            viewport: Some(Viewport::new(width, height, scale, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    doc
}

#[test]
fn render_scene_returns_the_viewport_size() {
    let mut doc = make_doc(800, 600, 1.0);
    assert_eq!(doc.viewport().window_size, (800, 600));

    let size = render_scene(
        &mut doc,
        &mut anyrender::NullScenePainter,
        RenderSize::Viewport,
    );

    assert_eq!(size, (800, 600));
}

#[test]
fn render_scene_returns_the_physical_size_at_a_hidpi_scale() {
    let mut doc = make_doc(1280, 720, 2.0);
    assert_eq!(doc.viewport().window_size, (1280, 720));
    assert_eq!(doc.viewport().scale_f64(), 2.0);

    let size = render_scene(
        &mut doc,
        &mut anyrender::NullScenePainter,
        RenderSize::Viewport,
    );

    assert_eq!(size, (1280, 720));
}
