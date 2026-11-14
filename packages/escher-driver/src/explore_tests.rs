//! The exploration's own rules, with no held instance: the budget's bounds, the verbs read from
//! the table and the image field that leaves a verb out. The runs themselves are the stand
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
fn the_verbs_explored_are_the_tables_target_verbs_with_no_image() {
    let verbs: Vec<&str> = explored_verbs().iter().map(|verb| verb.name).collect();
    assert_eq!(verbs, ["click", "type", "scroll"]);
    assert!(!verbs.contains(&"screenshot"));
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

const COUNT: FieldSpec = FieldSpec {
    name: "png",
    kind: FieldKind::Count,
    always: true,
    help: "",
};

#[test]
fn a_verb_whose_result_holds_an_image_is_left_out_by_its_field_not_its_name() {
    // A synthetic verb, its result fields and whether it is explored.
    let rows: [(VerbSpec, bool); 4] = [
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
    ];
    assert_eq!(rows.len(), 4);
    for (row, (verb, explored)) in rows.iter().enumerate() {
        assert!(explorable(verb) == *explored, "row {row}");
    }
}
