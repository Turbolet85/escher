//! The exploration's own rules, with no held instance: the budget's bounds, the verbs read from
//! the table and the image and layout fields that leave a verb out. The runs themselves are the stand
//! check's (`stand_explore`). A child module of `explore`, so it reads the module's private
//! items.

use super::*;
use crate::schema::{ArgKind, ArgSpec, FieldSpec, Level, VERBS};

#[test]
fn a_budget_is_a_whole_number_from_one_to_the_bound() {
    let rows: [(u32, Option<u32>); 4] = [
        (0, None),
        (1, Some(1)),
        (10_000, Some(10_000)),
        (10_001, None),
    ];
    assert_eq!(rows.len(), 4);
    for (row, (steps, budget)) in rows.into_iter().enumerate() {
        assert!(
            StepBudget::new(steps).map(StepBudget::get) == budget,
            "row {row}"
        );
    }
    assert_eq!(MAX_EXPLORE_STEPS, 10_000);
}

#[test]
fn the_verbs_explored_are_the_tables_target_verbs_with_no_image_no_layout_and_no_canvas_reading() {
    let verbs: Vec<&str> = explored_verbs().iter().map(|verb| verb.name).collect();
    assert_eq!(verbs, ["click", "type", "scroll"]);
    assert!(!verbs.contains(&"screenshot"));
    assert!(!verbs.contains(&"layout"));
    assert!(!verbs.contains(&"canvas"));
    // Every verb of the table that takes an id and holds no image is explored.
    let taking_an_id = VERBS
        .iter()
        .filter(|verb| verb.level == Level::Instance)
        .filter(|verb| verb.args.iter().any(|arg| arg.kind == ArgKind::Id))
        .filter(|verb| explorable(verb))
        .count();
    assert_eq!(taking_an_id, verbs.len());
}

const ID: ArgSpec = ArgSpec {
    name: "id",
    kind: ArgKind::Id,
    required: true,
    help: "",
};

const IMAGE: FieldSpec = FieldSpec {
    name: "picture",
    kind: FieldKind::Png,
    always: true,
    help: "",
};

const PLACED: FieldSpec = FieldSpec {
    name: "where",
    kind: FieldKind::Placement,
    always: true,
    help: "",
};

const BOX: FieldSpec = FieldSpec {
    name: "box",
    kind: FieldKind::Rect,
    always: true,
    help: "",
};

const MOVES: FieldSpec = FieldSpec {
    name: "moves",
    kind: FieldKind::Shifts,
    always: true,
    help: "",
};

const EXTENT: FieldSpec = FieldSpec {
    name: "extent",
    kind: FieldKind::Size,
    always: true,
    help: "",
};

const LINES: FieldSpec = FieldSpec {
    name: "lines",
    kind: FieldKind::Commands,
    always: false,
    help: "",
};

const TALLY: FieldSpec = FieldSpec {
    name: "tally",
    kind: FieldKind::CommandCounts,
    always: false,
    help: "",
};

const COUNT: FieldSpec = FieldSpec {
    name: "png",
    kind: FieldKind::Count,
    always: true,
    help: "",
};

#[test]
fn a_verb_whose_result_holds_an_image_or_a_layout_reading_is_left_out_by_its_field_not_its_name() {
    // A synthetic verb, its result fields and whether it is explored.
    let rows: [(VerbSpec, bool); 11] = [
        (
            VerbSpec {
                name: "click",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[IMAGE],
            },
            false,
        ),
        (
            VerbSpec {
                name: "screenshot",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[COUNT],
            },
            true,
        ),
        (
            VerbSpec {
                name: "drag",
                level: Level::Instance,
                help: "",
                args: &[ID, ID],
                fields: &[],
            },
            true,
        ),
        (
            VerbSpec {
                name: "paint",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[COUNT, IMAGE],
            },
            false,
        ),
        (
            VerbSpec {
                name: "click",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[COUNT, PLACED],
            },
            false,
        ),
        (
            VerbSpec {
                name: "scroll",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[BOX],
            },
            false,
        ),
        (
            VerbSpec {
                name: "type",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[MOVES],
            },
            false,
        ),
        (
            VerbSpec {
                name: "click",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[EXTENT],
            },
            false,
        ),
        (
            VerbSpec {
                name: "layout",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[COUNT],
            },
            true,
        ),
        (
            VerbSpec {
                name: "click",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[COUNT, LINES],
            },
            false,
        ),
        (
            VerbSpec {
                name: "type",
                level: Level::Instance,
                help: "",
                args: &[ID],
                fields: &[TALLY],
            },
            false,
        ),
    ];
    assert_eq!(rows.len(), 11);
    for (row, (verb, explored)) in rows.iter().enumerate() {
        assert!(explorable(verb) == *explored, "row {row}");
    }
}
