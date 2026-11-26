//! Virtual nodes a canvas declares, read in process: a widget painted from Rust answers a list of
//! declarations, and `DioxusDocument::virtual_nodes` takes each one inside the canvas's content
//! box with the reserved id `{owner}///{key}`, or refuses it by index with one closed reason. On a
//! padded, bordered canvas a region that fits the border box but not the content box is refused;
//! a widget that declares nothing reads no node, and an element holding no widget reads no
//! canvas. A declaring canvas puts each taken id, and no declared name or refused declaration's
//! key, on the graph the driver answers; the snapshot reading them is
//! `virtual_nodes_as_elements`'s. Each case runs in both layout
//! modes. A failure message names a layout mode only.

use accesskit::Role;
use blitz_dom::{BoundingRect, VirtualNodeDeclaration, Widget};
use dioxus::prelude::*;
use dioxus_native_dom::{
    CustomWidgetAttr, DioxusDocument, RefusedDeclaration, VirtualNodeRefusal, VirtualNodesUnread,
};
use escher_driver::Call;

use blitz_test_harness::Harness;

mod fixtures;

/// A content-box object of 100 × 60 with 12 px of padding and a 3 px border, as
/// `stand_canvas_placement` places it.
const PADDED: &str = "display: block; box-sizing: content-box; width: 100px; height: 60px; \
                      padding: 12px; border: 3px solid black;";

/// The names the planted list declares, none of them text the page holds elsewhere.
const ALPHA: &str = "Planted alpha";
const BETA: &str = "Planted beta";

fn declared(
    key: &str,
    role: Role,
    name: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> VirtualNodeDeclaration {
    VirtualNodeDeclaration {
        key: key.to_string(),
        role,
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

/// The planted list: two valid declarations, then one refused for each reason.
fn planted() -> Vec<VirtualNodeDeclaration> {
    vec![
        declared("a", Role::Button, ALPHA, 0.0, 0.0, 40.0, 20.0),
        declared("b", Role::CheckBox, BETA, 60.0, 40.0, 40.0, 20.0),
        declared("c", Role::Button, "Planted gamma", 0.0, 0.0, 120.0, 80.0),
        declared("a", Role::Button, "Planted delta", 0.0, 0.0, 10.0, 10.0),
        declared("d", Role::Button, "", 0.0, 0.0, 10.0, 10.0),
        declared("", Role::Button, "Planted epsilon", 0.0, 0.0, 10.0, 10.0),
        declared("e/f", Role::Button, "Planted zeta", 0.0, 0.0, 10.0, 10.0),
    ]
}

/// A widget declaring the planted list.
struct Declaring;

impl Widget for Declaring {
    fn virtual_nodes(&self) -> Vec<VirtualNodeDeclaration> {
        planted()
    }
}

/// A widget that overrides nothing.
struct Plain;

impl Widget for Plain {}

fn page() -> Element {
    let declaring = use_hook(|| CustomWidgetAttr::new(Declaring));
    let plain = use_hook(|| CustomWidgetAttr::new(Plain));
    rsx! {
        object { id: "vn-canvas", style: PADDED, "data": declaring }
        object { id: "vn-plain", style: PADDED, "data": plain }
        button { id: "vn-button", "Press" }
    }
}

fn node(harness: &Harness<DioxusDocument>, id: &str) -> blitz_dom::NodeId {
    harness
        .doc
        .inner
        .borrow()
        .get_element_by_id(id)
        .expect("the page's element is in the document")
}

#[test]
fn the_padded_canvas_reads_its_declared_nodes_in_its_content_box() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = fixtures::boot(page, incremental);
        let Ok(read) = harness.doc.virtual_nodes(node(&harness, "vn-canvas")) else {
            panic!("{at}: the canvas reads its declarations");
        };
        let ids: Vec<&str> = read.nodes.iter().map(|n| n.id.as_str()).collect();
        assert!(
            ids == ["vn-canvas///a", "vn-canvas///b"],
            "{at}: the two declarations inside the content box are taken"
        );
        let planted = planted();
        assert!(
            read.nodes[0].declaration == planted[0] && read.nodes[1].declaration == planted[1],
            "{at}: each taken node holds its role, name and region as declared"
        );
        let refused = [
            (2, VirtualNodeRefusal::RegionOutside),
            (3, VirtualNodeRefusal::DuplicateKey),
            (4, VirtualNodeRefusal::EmptyName),
            (5, VirtualNodeRefusal::EmptyKey),
            (6, VirtualNodeRefusal::KeyHoldsSlash),
        ]
        .map(|(index, refusal)| RefusedDeclaration { index, refusal });
        assert!(
            read.refused == refused,
            "{at}: each other declaration is refused by its index and reason"
        );
    }
}

#[test]
fn a_widget_that_declares_nothing_reads_no_node() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let harness = fixtures::boot(page, incremental);
        let read = harness.doc.virtual_nodes(node(&harness, "vn-plain"));
        assert!(
            read.is_ok_and(|read| read.nodes.is_empty() && read.refused.is_empty()),
            "{at}: a widget that declares nothing reads no node and no refusal"
        );
        assert!(
            harness.doc.virtual_nodes(node(&harness, "vn-button"))
                == Err(VirtualNodesUnread::NoCanvas),
            "{at}: a button reads no canvas"
        );
    }
}

fn call(verb: &str) -> Call {
    Call {
        verb: verb.to_string(),
        args: Vec::new(),
    }
}

#[test]
fn a_declaring_canvas_puts_its_ids_and_no_name_on_the_graph() {
    for incremental in [false, true] {
        let at = format!("incremental={incremental}");
        let mut session = fixtures::hold("virtual-nodes", page, incremental);
        {
            let harness = session.harness();
            let Ok(read) = harness.doc.virtual_nodes(node(harness, "vn-canvas")) else {
                panic!("{at}: the canvas reads its declarations");
            };
            assert!(
                read.nodes
                    .iter()
                    .any(|n| n.id == "vn-canvas///a" && n.declaration.name == ALPHA),
                "{at}: the in-process read holds the reserved id and a planted name"
            );
        }
        let Ok(answer) = session.run(&call("graph")) else {
            panic!("{at}: the graph is accepted");
        };
        let line = answer.to_json();
        for id in ["vn-canvas///a", "vn-canvas///b"] {
            assert!(
                line.contains(&format!("\"{id}\"")),
                "{at}: the graph holds each taken id"
            );
        }
        for id in ["vn-canvas///c", "vn-canvas///d", "vn-canvas///e/f"] {
            assert!(
                !line.contains(id),
                "{at}: the graph holds no refused declaration's key"
            );
        }
        for name in planted()
            .iter()
            .map(|d| d.name.as_str())
            .filter(|n| !n.is_empty())
        {
            assert!(
                !line.contains(name),
                "{at}: the graph holds no declared name"
            );
        }
    }
}
