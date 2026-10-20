//! The extent of an inline root's laid-out text beside the content box it is laid out in.

use crate::{BaseDocument, BoundingRect, NodeId};

/// Where an element's laid-out lines lie beside its content box, both in CSS px relative to the
/// viewport, in the space of [`BaseDocument::get_client_bounding_rect`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextExtent {
    /// The element's content box: its client rect less its border, its padding and any
    /// scrollbar.
    pub content: BoundingRect,
    /// The box the laid-out lines take: from the content box's top-left, moved down by the
    /// lines' block offset, as wide as the widest line and as high as the lines.
    pub text: BoundingRect,
}

impl BaseDocument {
    /// The extent of the text `node_id` lays out, beside its content box: a `text` box wider
    /// or taller than `content` holds lines that run past it.
    ///
    /// `None` for an id that does not resolve, a node that is not an inline root, and one with
    /// no inline layout. Reads layout only and changes nothing. Right-to-left and vertical text
    /// are read as left-to-right, horizontal lines from the content box's top-left, and a box
    /// under a transform is read where its client rect stands.
    pub fn text_extent(&self, node_id: NodeId) -> Option<TextExtent> {
        let node = self.get_node(node_id)?;
        if !node.flags.is_inline_root() {
            return None;
        }
        let inline_layout = node.element_data()?.inline_layout_data.as_ref()?;
        let border_box = self.get_client_bounding_rect(node_id)?;
        let (_, layout) = self.physical_unrounded_geometry(node_id);
        let left = (layout.border.left + layout.padding.left) as f64;
        let top = (layout.border.top + layout.padding.top) as f64;
        let right =
            (layout.border.right + layout.padding.right + layout.scrollbar_size.width) as f64;
        let bottom =
            (layout.border.bottom + layout.padding.bottom + layout.scrollbar_size.height) as f64;
        let content = BoundingRect {
            x: border_box.x + left,
            y: border_box.y + top,
            width: (border_box.width - left - right).max(0.0),
            height: (border_box.height - top - bottom).max(0.0),
        };
        let lines = &inline_layout.layout;
        let scale = lines.scale() as f64;
        let text = BoundingRect {
            x: content.x,
            y: content.y + inline_layout.block_offset as f64,
            width: lines.width() as f64 / scale,
            height: lines.height() as f64 / scale,
        };
        Some(TextExtent { content, text })
    }
}
