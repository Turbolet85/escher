//! The WCAG 2.x contrast of an element's text against the background it is painted on, read from
//! the document's computed styles.

use blitz_dom::util::ToColorColor;
use blitz_dom::{BaseDocument, Document, Node};
use blitz_traits::node_id::NodeId;
use color::{AlphaColor, Srgb};

use crate::Harness;

type Color = AlphaColor<Srgb>;

/// The WCAG 2.x contrast ratio (>= 1) between two colours, `(L1 + 0.05) / (L2 + 0.05)` with the
/// lighter colour's relative luminance over the darker's, matching Chrome's
/// `color_utils::GetContrastRatio`.
///
/// The alpha of either colour is ignored: a caller composites a translucent colour over what lies
/// beneath it first.
pub fn contrast_ratio(a: Color, b: Color) -> f32 {
    let la = a.split().0.relative_luminance() + 0.05;
    let lb = b.split().0.relative_luminance() + 0.05;
    if la > lb { la / lb } else { lb / la }
}

/// An element's text colour and the background that text is painted on, with their contrast
/// ratio and the computed font size and weight that decide the ratio SC 1.4.3 asks of it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextContrast {
    /// The effective text colour, opaque: the computed `color`, composited over
    /// [`background`](Self::background) when it is translucent.
    pub text: Color,
    /// The effective background colour, opaque: the backgrounds beneath the text composited down
    /// to the first opaque one, or to the canvas.
    pub background: Color,
    /// [`contrast_ratio`] of [`text`](Self::text) and [`background`](Self::background).
    pub ratio: f32,
    /// The computed `font-size`, in CSS px.
    pub font_size_px: f32,
    /// The computed `font-weight`.
    pub font_weight: f32,
}

impl TextContrast {
    /// Whether the text is WCAG 2.x "large scale": at least 18 pt, or at least 14 pt and bold,
    /// at 1 pt = 4/3 CSS px.
    pub fn is_large_text(&self) -> bool {
        self.font_size_px >= 24.0 || (self.font_size_px >= 56.0 / 3.0 && self.font_weight >= 700.0)
    }

    /// The minimum ratio SC 1.4.3 sets for this text: 3:1 for large text, else 4.5:1.
    pub fn required_ratio(&self) -> f32 {
        if self.is_large_text() { 3.0 } else { 4.5 }
    }
}

impl<D: Document> Harness<D> {
    /// The contrast of the text `node_id` paints, read from computed styles. Changes nothing.
    ///
    /// The text is a text input's own value when `node_id` is one, else that of the first text
    /// node under `node_id`, in tree order, holding a character that is not whitespace, in its
    /// parent element's computed `color` and font. Its background is each element's computed
    /// `background-color` from that element up its ancestors, composited down to the first opaque
    /// one; with none before the root, to the canvas (the root element's background, else
    /// `body`'s), and the canvas over the opaque white a screenshot is composited over.
    ///
    /// `None` when there is no such text, when an element on the walk has no computed styles, or
    /// when an element between the text and the first opaque background paints a background image,
    /// an `opacity` below 1 or a `filter`: colours alone do not determine what such text is
    /// painted on.
    pub fn text_contrast(&self, node_id: NodeId) -> Option<TextContrast> {
        let doc = self.base();
        let element = text_bearing_element(&doc, doc.get_node(node_id)?)?;
        let (text, font_size_px, font_weight) = {
            let styles = element.primary_styles()?;
            (
                styles.clone_color().as_color_color(),
                styles.clone_font_size().computed_size().px(),
                styles.clone_font_weight().value(),
            )
        };
        let background = background_behind(&doc, element)?;
        let text = over(text, background);
        Some(TextContrast {
            text,
            background,
            ratio: contrast_ratio(text, background),
            font_size_px,
            font_weight,
        })
    }
}

fn text_bearing_element<'a>(doc: &'a BaseDocument, node: &'a Node) -> Option<&'a Node> {
    if node
        .element_data()
        .is_some_and(|data| data.text_input_data().is_some())
    {
        return Some(node);
    }
    first_text_parent(doc, node)
}

fn first_text_parent<'a>(doc: &'a BaseDocument, node: &'a Node) -> Option<&'a Node> {
    for &child_id in &node.children {
        let Some(child) = doc.get_node(child_id) else {
            continue;
        };
        let holds_text = child
            .text_data()
            .is_some_and(|text| text.content.chars().any(|c| !c.is_whitespace()));
        if holds_text && node.is_element() {
            return Some(node);
        }
        if let Some(found) = first_text_parent(doc, child) {
            return Some(found);
        }
    }
    None
}

fn background_behind(doc: &BaseDocument, from: &Node) -> Option<Color> {
    let mut layers = Vec::new();
    let mut opaque = None;
    let mut current = Some(from);
    while let Some(node) = current.filter(|node| node.is_element()) {
        if paints_more_than_colour(doc, node)? {
            return None;
        }
        let background = background_colour(node)?;
        if background.components[3] >= 1.0 {
            opaque = Some(background);
            break;
        }
        layers.push(background);
        current = node.parent.and_then(|id| doc.get_node(id));
    }
    let base = match opaque {
        Some(colour) => colour,
        None => over(canvas(doc), Color::WHITE),
    };
    Some(
        layers
            .into_iter()
            .rev()
            .fold(base, |below, layer| over(layer, below)),
    )
}

fn paints_more_than_colour(doc: &BaseDocument, node: &Node) -> Option<bool> {
    let styles = node.primary_styles()?;
    if styles.clone_opacity() < 1.0 || !styles.get_effects().filter.0.is_empty() {
        return Some(true);
    }
    let images = doc.resolved_style_value(node.id, "background-image");
    Some(images.split(',').any(|layer| layer.trim() != "none"))
}

fn background_colour(node: &Node) -> Option<Color> {
    let styles = node.primary_styles()?;
    Some(
        styles
            .clone_background_color()
            .resolve_to_absolute(&styles.clone_color())
            .as_color_color(),
    )
}

/// The canvas as blitz-paint fills it: the root element's background, else `body`'s when the
/// root's is transparent.
fn canvas(doc: &BaseDocument) -> Color {
    let Some(root) = doc.try_root_element() else {
        return Color::TRANSPARENT;
    };
    let root_background = background_colour(root).unwrap_or(Color::TRANSPARENT);
    if root_background != Color::TRANSPARENT {
        return root_background;
    }
    root.children
        .iter()
        .filter_map(|&id| doc.get_node(id))
        .find(|node| {
            node.element_data()
                .is_some_and(|data| &*data.name.local == "body")
        })
        .and_then(background_colour)
        .unwrap_or(Color::TRANSPARENT)
}

/// `src` composited source-over onto the opaque `dst`.
fn over(src: Color, dst: Color) -> Color {
    let [sr, sg, sb, alpha] = src.components;
    if alpha >= 1.0 {
        return src;
    }
    let [dr, dg, db, _] = dst.components;
    let mix = |s: f32, d: f32| s * alpha + d * (1.0 - alpha);
    Color::new([mix(sr, dr), mix(sg, dg), mix(sb, db), 1.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(rgb: u32) -> Color {
        Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
    }

    fn rgb8(colour: Color) -> [u8; 3] {
        let c = colour.to_rgba8();
        [c.r, c.g, c.b]
    }

    fn reading(font_size_px: f32, font_weight: f32) -> TextContrast {
        TextContrast {
            text: Color::BLACK,
            background: Color::WHITE,
            ratio: 21.0,
            font_size_px,
            font_weight,
        }
    }

    const RATIOS: [(u32, u32, f32); 5] = [
        (0x000000, 0xffffff, 21.0),
        (0xffffff, 0x000000, 21.0),
        (0x4a6cf7, 0x4a6cf7, 1.0),
        (0x777777, 0xffffff, 4.48),
        (0xffffff, 0x4a6cf7, 4.39),
    ];

    #[test]
    fn the_ratio_is_wcag_2() {
        assert_eq!(RATIOS.len(), 5);
        for (row, (a, b, expected)) in RATIOS.into_iter().enumerate() {
            let ratio = contrast_ratio(hex(a), hex(b));
            assert!(
                (ratio - expected).abs() <= 0.005,
                "row {row}: read {ratio}, expected {expected}"
            );
        }
    }

    const THRESHOLDS: [(f32, f32, f32); 5] = [
        (23.9, 400.0, 4.5),
        (24.0, 400.0, 3.0),
        (18.67, 700.0, 3.0),
        (18.67, 600.0, 4.5),
        (18.6, 700.0, 4.5),
    ];

    #[test]
    fn large_text_asks_three_and_other_text_four_and_a_half() {
        assert_eq!(THRESHOLDS.len(), 5);
        for (row, (size, weight, expected)) in THRESHOLDS.into_iter().enumerate() {
            let reading = reading(size, weight);
            assert_eq!(reading.is_large_text(), expected == 3.0, "row {row}");
            assert_eq!(reading.required_ratio(), expected, "row {row}");
        }
    }

    #[test]
    fn a_button_reads_its_own_text_and_background() {
        let harness = Harness::from_html(
            r#"<html><body><button id="b" style="color:#ffffff; background:#4a6cf7;">Go</button></body></html>"#,
        );
        let button = harness.node("#b");
        let own = background_colour(harness.base().get_node(button).unwrap()).unwrap();
        assert_eq!(rgb8(own), [0x4a, 0x6c, 0xf7]);

        let reading = harness.text_contrast(button).unwrap();
        assert_eq!(rgb8(reading.text), [0xff, 0xff, 0xff]);
        assert_eq!(rgb8(reading.background), [0x4a, 0x6c, 0xf7]);
        assert!(
            (reading.ratio - 4.39).abs() <= 0.005,
            "read {}",
            reading.ratio
        );
    }

    #[test]
    fn a_translucent_ancestor_is_composited_over_the_page() {
        let harness = Harness::from_html(
            r#"<html><body><div style="background: rgba(0, 0, 0, 0.5);"><span id="s">Text</span></div></body></html>"#,
        );
        let span = harness.node("#s");
        {
            let doc = harness.base();
            let span_node = doc.get_node(span).unwrap();
            assert_eq!(background_colour(span_node).unwrap(), Color::TRANSPARENT);
            let div = doc.get_node(span_node.parent.unwrap()).unwrap();
            assert_eq!(background_colour(div).unwrap().components[3], 0.5);
            assert_eq!(canvas(&doc), Color::TRANSPARENT);
        }

        let reading = harness.text_contrast(span).unwrap();
        for (channel, value) in rgb8(reading.background).into_iter().enumerate() {
            assert!(value.abs_diff(0x80) <= 1, "channel {channel}: read {value}");
        }
    }

    #[test]
    fn an_element_with_no_text_reads_none() {
        let harness = Harness::from_html(r#"<html><body><div id="d"></div></body></html>"#);
        let div = harness.node("#d");
        assert!(harness.base().get_node(div).unwrap().children.is_empty());
        assert_eq!(harness.text_contrast(div), None);
    }
}
