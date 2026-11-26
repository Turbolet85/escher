//! The virtual nodes a canvas declares: the targets an element painted from Rust (a
//! [`Widget`](blitz_dom::Widget)) serves inside its content box with no element of their own,
//! read in process, validated and given their reserved ids.
//!
//! A canvas answers its declarations on demand ([`Widget::virtual_nodes`](blitz_dom::Widget::virtual_nodes)).
//! [`DioxusDocument::virtual_nodes`] reads each one in declaration order and takes it, with the id
//! `{owner}///{key}` ([`virtual_node_id`](crate::virtual_node_id)), or refuses it with one closed
//! reason. Nothing here marks the document, logs, or carries a node anywhere.

use crate::{DioxusDocument, NodeId, virtual_node_id};
use blitz_dom::VirtualNodeDeclaration;
use rustc_hash::FxHashSet;

/// A declaration a canvas's read took: its reserved id and the declaration as the app wrote it.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualNode {
    /// The node's reserved id, `{owner}///{key}`, the owner the canvas element's stable id.
    pub id: String,
    /// The declaration as the widget answered it.
    pub declaration: VirtualNodeDeclaration,
}

/// Why a declaration was not taken. The first rule that applies, in this order, decides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualNodeRefusal {
    /// The key is empty.
    EmptyKey,
    /// The key holds `/`.
    KeyHoldsSlash,
    /// An earlier declaration of the canvas already claimed the key.
    DuplicateKey,
    /// The name is empty.
    EmptyName,
    /// The region does not lie wholly inside the canvas's content box, or is empty.
    RegionOutside,
}

/// A declaration a canvas's read refused: its position in the widget's list and the reason.
/// It holds nothing the app wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefusedDeclaration {
    /// The declaration's position in the list the widget answered.
    pub index: usize,
    /// Why it was not taken.
    pub refusal: VirtualNodeRefusal,
}

/// What one canvas declares now: the nodes taken, in declaration order, and the declarations
/// refused.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VirtualNodes {
    /// The declarations taken, each with its id.
    pub nodes: Vec<VirtualNode>,
    /// The declarations refused, in declaration order.
    pub refused: Vec<RefusedDeclaration>,
}

/// Why a node's virtual nodes cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualNodesUnread {
    /// The node is stale, no element, holds no widget, or reads no stable id.
    NoCanvas,
    /// The canvas has not been laid out yet, so it has no content box.
    NotLaidOut,
}

impl DioxusDocument {
    /// The virtual nodes the canvas `canvas` declares now, validated, each taken one with its
    /// reserved id `{owner}///{key}`, the owner the canvas's stable element id.
    ///
    /// A declaration is taken unless, in this order: its key is empty or holds `/`, an earlier
    /// declaration claimed its key (the first claimant keeps it, taken or not), its name is
    /// empty, or its region does not lie wholly inside the canvas's content box with a positive
    /// width and height. A refusal holds the declaration's index alone.
    ///
    /// Reads `NoCanvas` for a stale node, a non-element, an element holding no widget or one
    /// reading no stable id, and `NotLaidOut` before the canvas's first layout. Changes nothing.
    pub fn virtual_nodes(&self, canvas: NodeId) -> Result<VirtualNodes, VirtualNodesUnread> {
        let owner = self
            .element_id(canvas)
            .ok_or(VirtualNodesUnread::NoCanvas)?;
        let inner = self.inner.borrow();
        let node = inner.get_node(canvas).ok_or(VirtualNodesUnread::NoCanvas)?;
        let widget = &node
            .element_data()
            .and_then(|element| element.custom_widget_data())
            .ok_or(VirtualNodesUnread::NoCanvas)?
            .widget;
        let layout = &node
            .try_layout_data()
            .ok_or(VirtualNodesUnread::NotLaidOut)?
            .final_layout;
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
        Ok(validate(
            &owner,
            f64::from(width.max(0.0)),
            f64::from(height.max(0.0)),
            &widget.virtual_nodes(),
        ))
    }
}

#[cfg(feature = "accessibility")]
impl DioxusDocument {
    /// The page bounds of `region`, a taken node's region in the content box of `canvas`: the
    /// canvas's bounding client rect moved by its left and top border and padding, then by the
    /// region's origin, at the region's size. Exact under a translate; a rotated or scaled canvas
    /// is read unrotated and unscaled. `None` where the canvas has no client rect.
    pub(crate) fn page_bounds(
        &self,
        canvas: NodeId,
        region: &blitz_dom::BoundingRect,
    ) -> Option<blitz_dom::BoundingRect> {
        let inner = self.inner.borrow();
        let rect = inner.get_client_bounding_rect(canvas)?;
        let layout = inner.get_node(canvas)?.final_layout();
        let left = f64::from(layout.border.left + layout.padding.left);
        let top = f64::from(layout.border.top + layout.padding.top);
        Some(blitz_dom::BoundingRect {
            x: rect.x + left + region.x,
            y: rect.y + top + region.y,
            width: region.width,
            height: region.height,
        })
    }
}

/// The accessibility tree id of the `n`th taken node of `canvas`: the canvas's slot index in the
/// low 32 bits and `2·(n+1)` in the high 32. Its high half is even, so it equals no DOM node's id,
/// whose version is always odd, nor the window's `u64::MAX`. `None` past 2³¹ − 1 nodes a canvas.
#[cfg(feature = "accessibility")]
pub(crate) fn tree_id(canvas: NodeId, n: usize) -> Option<accesskit::NodeId> {
    let high = u32::try_from(n.checked_add(1)?.checked_mul(2)?).ok()?;
    let index = canvas.as_u64() & u64::from(u32::MAX);
    Some(accesskit::NodeId((u64::from(high) << 32) | index))
}

/// Reads each declaration in order against a content box of `width` × `height` and takes it
/// with its id under `owner`, or refuses it.
pub(crate) fn validate(
    owner: &str,
    width: f64,
    height: f64,
    declarations: &[VirtualNodeDeclaration],
) -> VirtualNodes {
    let mut read = VirtualNodes::default();
    let mut claimed: FxHashSet<&str> = FxHashSet::default();
    for (index, declaration) in declarations.iter().enumerate() {
        let key = declaration.key.as_str();
        let refusal = match virtual_node_id(owner, key) {
            None if key.is_empty() => Err(VirtualNodeRefusal::EmptyKey),
            None => Err(VirtualNodeRefusal::KeyHoldsSlash),
            Some(_) if !claimed.insert(key) => Err(VirtualNodeRefusal::DuplicateKey),
            Some(_) if declaration.name.is_empty() => Err(VirtualNodeRefusal::EmptyName),
            Some(_) if !inside(&declaration.region, width, height) => {
                Err(VirtualNodeRefusal::RegionOutside)
            }
            Some(id) => Ok(id),
        };
        match refusal {
            Ok(id) => read.nodes.push(VirtualNode {
                id,
                declaration: declaration.clone(),
            }),
            Err(refusal) => read.refused.push(RefusedDeclaration { index, refusal }),
        }
    }
    read
}

/// Whether `region` is non-empty and lies wholly inside a `width` × `height` box. Each bound is a
/// condition that must hold, so a NaN or an infinity fails one.
fn inside(region: &blitz_dom::BoundingRect, width: f64, height: f64) -> bool {
    region.x >= 0.0
        && region.y >= 0.0
        && region.width > 0.0
        && region.height > 0.0
        && region.x + region.width <= width
        && region.y + region.height <= height
}

#[cfg(test)]
mod tests {
    use super::{
        RefusedDeclaration, VirtualNodeRefusal, VirtualNodes, VirtualNodesUnread, validate,
    };
    use crate::{CustomWidgetAttr, DioxusDocument};
    use blitz_dom::{BoundingRect, Document, DocumentConfig, VirtualNodeDeclaration, Widget};
    use dioxus::prelude::*;
    use dioxus_core::ScopeId;

    /// A declaration of `key` named `name` at `(x, y)`, `width` × `height`.
    fn declared(
        key: &str,
        name: &str,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> VirtualNodeDeclaration {
        VirtualNodeDeclaration {
            key: key.to_string(),
            role: Default::default(),
            name: name.to_string(),
            region: BoundingRect {
                x,
                y,
                width,
                height,
            },
            disabled: false,
            selected: None,
        }
    }

    /// A valid declaration of `key`.
    fn valid(key: &str) -> VirtualNodeDeclaration {
        declared(key, "named", 10.0, 10.0, 20.0, 20.0)
    }

    /// Validated against a 100 × 60 content box under the owner `canvas`.
    fn read(declarations: &[VirtualNodeDeclaration]) -> VirtualNodes {
        validate("canvas", 100.0, 60.0, declarations)
    }

    fn refused(index: usize, refusal: VirtualNodeRefusal) -> RefusedDeclaration {
        RefusedDeclaration { index, refusal }
    }

    #[test]
    fn an_empty_key_is_refused() {
        let declarations = [valid("")];
        assert!(declarations[0].key.is_empty());
        let read = read(&declarations);
        assert!(read.nodes.is_empty(), "no node is taken");
        assert_eq!(read.refused, [refused(0, VirtualNodeRefusal::EmptyKey)]);
    }

    #[test]
    fn a_key_holding_a_slash_is_refused() {
        let declarations = [valid("a/b"), valid("/")];
        assert!(declarations.iter().all(|d| d.key.contains('/')));
        let read = read(&declarations);
        assert!(read.nodes.is_empty(), "no node is taken");
        assert_eq!(
            read.refused,
            [
                refused(0, VirtualNodeRefusal::KeyHoldsSlash),
                refused(1, VirtualNodeRefusal::KeyHoldsSlash),
            ]
        );
    }

    #[test]
    fn a_later_duplicate_key_is_refused_and_the_first_kept() {
        let declarations = [
            valid("k"),
            valid("k"),
            valid("k"),
            declared("j", "", 10.0, 10.0, 20.0, 20.0),
            valid("j"),
        ];
        assert!(declarations[3].name.is_empty());
        let read = read(&declarations);
        assert_eq!(read.nodes.len(), 1, "one node is taken");
        assert_eq!(read.nodes[0].id, "canvas///k");
        assert_eq!(read.nodes[0].declaration, declarations[0]);
        assert_eq!(
            read.refused,
            [
                refused(1, VirtualNodeRefusal::DuplicateKey),
                refused(2, VirtualNodeRefusal::DuplicateKey),
                refused(3, VirtualNodeRefusal::EmptyName),
                refused(4, VirtualNodeRefusal::DuplicateKey),
            ]
        );
    }

    #[test]
    fn an_empty_name_is_refused() {
        let declarations = [declared("k", "", 10.0, 10.0, 20.0, 20.0)];
        assert!(declarations[0].name.is_empty());
        let read = read(&declarations);
        assert!(read.nodes.is_empty(), "no node is taken");
        assert_eq!(read.refused, [refused(0, VirtualNodeRefusal::EmptyName)]);
    }

    #[test]
    fn a_region_outside_the_content_box_is_refused() {
        let rows = [
            (80.0, 10.0, 21.0, 20.0),
            (10.0, 50.0, 20.0, 11.0),
            (-1.0, 10.0, 20.0, 20.0),
            (10.0, -1.0, 20.0, 20.0),
            (10.0, 10.0, 0.0, 20.0),
            (10.0, 10.0, 20.0, 0.0),
            (f64::NAN, 10.0, 20.0, 20.0),
            (10.0, 10.0, f64::INFINITY, 20.0),
        ];
        assert_eq!(rows.len(), 8);
        for (row, &(x, y, width, height)) in rows.iter().enumerate() {
            let read = read(&[declared("k", "named", x, y, width, height)]);
            assert!(read.nodes.is_empty(), "row {row}: no node is taken");
            assert!(
                read.refused == [refused(0, VirtualNodeRefusal::RegionOutside)],
                "row {row}: refused as outside"
            );
        }
    }

    #[test]
    fn a_region_touching_the_content_edges_is_taken() {
        let declarations = [declared("k", "named", 0.0, 0.0, 100.0, 60.0)];
        let read = read(&declarations);
        assert!(read.refused.is_empty(), "nothing is refused");
        assert_eq!(read.nodes.len(), 1);
        assert_eq!(read.nodes[0].declaration, declarations[0]);
    }

    #[test]
    fn a_taken_node_reads_its_owner_key_id() {
        let read = read(&[valid("a")]);
        assert!(read.refused.is_empty(), "nothing is refused");
        assert_eq!(read.nodes.len(), 1);
        assert_eq!(read.nodes[0].id, "canvas///a");
    }

    /// A widget declaring one valid node.
    struct Declaring;

    impl Widget for Declaring {
        fn virtual_nodes(&self) -> Vec<VirtualNodeDeclaration> {
            vec![valid("a")]
        }
    }

    static SHOW: GlobalSignal<bool> = Signal::global(|| true);

    fn canvas_app() -> Element {
        let attr = use_hook(|| CustomWidgetAttr::new(Declaring));
        rsx! {
            div { id: "plain" }
            if SHOW() {
                object { id: "canvas", "data": attr }
            }
        }
    }

    fn build() -> DioxusDocument {
        let mut doc = DioxusDocument::new(VirtualDom::new(canvas_app), DocumentConfig::default());
        doc.initial_build();
        doc
    }

    fn node(doc: &DioxusDocument, id: &str) -> blitz_dom::NodeId {
        doc.inner
            .borrow()
            .get_element_by_id(id)
            .expect("the fixture's element is in the document")
    }

    #[test]
    fn an_unlaid_canvas_reads_not_laid_out() {
        let doc = build();
        let canvas = node(&doc, "canvas");
        {
            let inner = doc.inner.borrow();
            let element = inner.get_node(canvas).and_then(|n| n.element_data());
            assert!(
                element.and_then(|e| e.custom_widget_data()).is_some(),
                "the canvas holds its widget"
            );
        }
        assert_eq!(
            doc.virtual_nodes(canvas),
            Err(VirtualNodesUnread::NotLaidOut)
        );
    }

    #[test]
    fn a_non_canvas_or_stale_node_reads_no_canvas() {
        let mut doc = build();
        let plain = node(&doc, "plain");
        let canvas = node(&doc, "canvas");
        assert_eq!(doc.virtual_nodes(plain), Err(VirtualNodesUnread::NoCanvas));

        doc.vdom.in_runtime(|| *SHOW.write() = false);
        doc.vdom.mark_dirty(ScopeId::APP);
        doc.poll(None);
        assert!(
            doc.inner.borrow().get_node(canvas).is_none(),
            "the canvas is dropped"
        );
        assert_eq!(doc.virtual_nodes(canvas), Err(VirtualNodesUnread::NoCanvas));
    }
}
