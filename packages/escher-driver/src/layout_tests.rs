use dioxus_native_dom::{
    BoundingRect, LayoutReason, Length, NodeId, Placed, PlacedBy, Shift, ShiftBy, SizeCause,
    SizeReason,
};

use super::{Placer, explain, unexplained};
use crate::lint::Bounds;

fn rect(x: f64, y: f64) -> BoundingRect {
    BoundingRect {
        x,
        y,
        width: 40.0,
        height: 20.0,
    }
}

fn reason(px: f64, by: SizeCause) -> SizeReason {
    SizeReason {
        px,
        by,
        size: Length::Px(px as f32),
        min: Length::Auto,
        max: Length::None,
        flex: None,
    }
}

/// A reason placed by flex in node 2, scrolled by node 2, moved by node 9's transform and the
/// viewport.
fn engine_reason() -> LayoutReason {
    LayoutReason {
        bounds: rect(10.0, 5.0),
        laid_out: rect(12.0, 6.0),
        placed: Placed {
            by: PlacedBy::Flex,
            container: Some(NodeId::from_u64(2)),
            at: (4.0, 3.0),
            relative: Some((1.0, 0.0)),
        },
        shifts: vec![
            Shift {
                by: ShiftBy::Scroll,
                holder: Some(NodeId::from_u64(2)),
                dx: 0.0,
                dy: -8.0,
            },
            Shift {
                by: ShiftBy::Transform,
                holder: Some(NodeId::from_u64(9)),
                dx: 0.0,
                dy: 0.0,
            },
            Shift {
                by: ShiftBy::Scroll,
                holder: None,
                dx: 0.0,
                dy: -1.0,
            },
        ],
        width: reason(40.0, SizeCause::Style),
        height: reason(20.0, SizeCause::Stretch),
    }
}

fn stable_id(node: NodeId) -> Option<String> {
    (node == NodeId::from_u64(2)).then(|| "list".to_string())
}

#[test]
fn an_engine_reason_reads_by_stable_id() {
    let explained = explain("row", engine_reason(), &stable_id);
    assert_eq!(explained.id, "row");
    assert_eq!(
        explained.bounds,
        Bounds {
            x: 10.0,
            y: 5.0,
            width: 40.0,
            height: 20.0
        }
    );
    assert_eq!(explained.laid_out.x, 12.0);
    assert_eq!(explained.placed.placer, Placer::Engine(PlacedBy::Flex));
    assert_eq!(explained.placed.container.as_deref(), Some("list"));
    assert_eq!(explained.placed.at, (4.0, 3.0));
    assert_eq!(explained.placed.relative, Some((1.0, 0.0)));
    let holders: Vec<Option<&str>> = explained
        .shifts
        .iter()
        .map(|shift| shift.holder.as_deref())
        .collect();
    assert_eq!(holders, [Some("list"), None, None]);
    assert_eq!(explained.shifts[0].dy, -8.0);
    assert_eq!(explained.shifts[1].by, ShiftBy::Transform);
    assert_eq!(explained.width.by, SizeCause::Style);
    assert_eq!(explained.height.by, SizeCause::Stretch);
}

#[test]
fn a_container_with_no_stable_id_reads_none() {
    let explained = explain("row", engine_reason(), &|_| None);
    assert_eq!(explained.placed.container, None);
    assert!(explained.shifts.iter().all(|shift| shift.holder.is_none()));
}

#[test]
fn an_unexplained_element_reads_its_bounds_and_layout_causes() {
    let bounds = Bounds {
        x: 1.0,
        y: 2.0,
        width: 3.0,
        height: 4.0,
    };
    let explained = unexplained("group", bounds);
    assert_eq!(explained.placed.placer, Placer::Unexplained);
    assert_ne!(explained.placed.placer, Placer::Engine(PlacedBy::Block));
    assert_eq!(explained.bounds, bounds);
    assert_eq!(explained.laid_out, bounds);
    assert_eq!(explained.placed.container, None);
    assert!(explained.shifts.is_empty());
    assert_eq!(explained.width.by, SizeCause::Layout);
    assert_eq!(explained.height.by, SizeCause::Layout);
    assert_eq!(explained.height.px, 4.0);
}

#[test]
fn the_reserved_app_placer_is_spelled_and_matched() {
    let placer = Placer::App {
        canvas: "board".to_string(),
    };
    let canvas = match &placer {
        Placer::App { canvas } => Some(canvas.as_str()),
        Placer::Engine(_) | Placer::Unexplained => None,
    };
    assert_eq!(canvas, Some("board"));
    assert_ne!(placer, Placer::Engine(PlacedBy::Block));
}
