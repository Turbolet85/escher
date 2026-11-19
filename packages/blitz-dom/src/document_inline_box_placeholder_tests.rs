//! An inline-block is placed in its line even where the text around it has no font at all.
//!
//! Parley (0.13) places an in-flow inline box at a U+FFFC placeholder in its parent's text, and
//! a placeholder no font can shape yields no box in the line: the box keeps a 0x0 layout. Here
//! the parent (`body`) is set in a family the context does not hold, while the inline-block's
//! own text is set in a family whose font arrives as a `Resource::Font` after the first layout,
//! as an `@font-face` source does. The engine shapes the placeholder with its private
//! placeholder family last (`crate::PLACEHOLDER_FONT_FAMILY`), so the box is placed. Read red on
//! upstream `547d7514b` (the box stays 0 wide) — an upstream defect, fixed here. Built by hand
//! through `DocumentMutator` (blitz-html would be a circular dependency).

use super::*;
use crate::net::{FontFaceOverrides, Resource, ResourceLoadResponse};
use crate::qual_name;
use blitz_traits::shell::ColorScheme;

const FAMILY: &str = "LateFamily";

/// A page whose one inline-block holds three bullets set in `FAMILY`, under a font context that
/// holds no usable family; laid out once.
fn page(incremental: bool) -> (BaseDocument, NodeId) {
    let mut document = BaseDocument::new(DocumentConfig {
        viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
        font_ctx: Some(crate::build_single_font_ctx(&[0u8; 16])),
        incremental: Some(incremental),
        ..Default::default()
    });
    document.add_user_agent_stylesheet(crate::UserAgentStylesheet::parse(&format!(
        "html, body {{ display: block }} \
         span {{ display: inline-block; font-family: {FAMILY}; font-size: 20px }}"
    )));
    let root = document.root_node().id;
    let span = {
        let mut mutator = document.mutate();
        let html = mutator.create_element(qual_name!("html", html), vec![]);
        let body = mutator.create_element(qual_name!("body", html), vec![]);
        let span = mutator.create_element(qual_name!("span", html), vec![]);
        let text = mutator.create_text_node("\u{2022}\u{2022}\u{2022}");
        mutator.append_children(span, &[text]);
        mutator.append_children(body, &[span]);
        mutator.append_children(html, &[body]);
        mutator.append_children(root, &[html]);
        span
    };
    document.resolve(0.0);
    (document, span)
}

fn width(document: &BaseDocument, node: NodeId) -> f32 {
    document.get_node(node).unwrap().final_layout().size.width
}

/// The font loaded for `FAMILY`, as `fetch_font_face`'s handler hands it over.
fn font_loaded() -> ResourceLoadResponse {
    ResourceLoadResponse {
        request_id: 0,
        node_id: None,
        resolved_url: Some(String::from("test://late-family")),
        result: Ok(Resource::Font(
            blitz_traits::net::Bytes::from_static(crate::BULLET_FONT),
            FontFaceOverrides {
                family_name: Some(String::from(FAMILY)),
                weight: None,
                style: None,
                width: None,
            },
        )),
    }
}

#[test]
fn an_inline_block_in_text_no_font_can_shape_is_placed_once_its_own_font_loads() {
    for incremental in [false, true] {
        let (mut document, span) = page(incremental);
        assert_eq!(
            width(&document, span),
            0.0,
            "with no font for its family the inline-block has no width (incremental: {incremental})"
        );

        document.load_resource(font_loaded());
        document.resolve(0.0);

        assert!(
            width(&document, span) > 0.0,
            "the inline-block takes the width of its text in the loaded font \
             (incremental: {incremental})"
        );
    }
}
