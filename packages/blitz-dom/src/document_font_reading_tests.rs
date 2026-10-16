//! Guards `can_paint_text`: a font context holding a decodable font can paint text, one holding no
//! family cannot, and the reading changes nothing. A child module of `document`, because it builds
//! its document through the module's own imports.

use super::*;
use crate::build_single_font_ctx;

fn doc_with(font_ctx: FontContext) -> BaseDocument {
    BaseDocument::new(DocumentConfig {
        font_ctx: Some(font_ctx),
        ..Default::default()
    })
}

#[test]
fn a_context_holding_a_decodable_font_can_paint_text() {
    let doc = doc_with(build_single_font_ctx(crate::BULLET_FONT));
    assert!(doc.can_paint_text());
}

#[test]
fn a_context_holding_no_family_cannot_paint_text() {
    let doc = doc_with(build_single_font_ctx(&[0u8; 16]));
    assert!(!doc.can_paint_text());
    assert!(!doc.can_paint_text(), "the reading changes nothing");
}
