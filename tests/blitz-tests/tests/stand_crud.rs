//! The stand's CRUD task, booted headlessly, adds exactly one list entry built
//! from the typed name and surname on Create, and keeps Update and Delete
//! disabled until an entry is selected.

use seven_guis::stand::{self, LeanTask};

const LIST_ITEM: &str = ".crud-root .list > .list-item";
const NAME_INPUT: &str = ".crud-root .fields > input:nth-of-type(1)";
const SURNAME_INPUT: &str = ".crud-root .fields > input:nth-of-type(2)";
const CREATE_BTN: &str = ".crud-root .crud-buttons > button:nth-child(1)";
const UPDATE_BTN: &str = ".crud-root .crud-buttons > button:nth-child(2)";
const DELETE_BTN: &str = ".crud-root .crud-buttons > button:nth-child(3)";

#[test]
fn update_and_delete_are_disabled_before_a_selection() {
    let harness = stand::boot(LeanTask::Crud, stand::options(true));
    assert_eq!(harness.text_content(UPDATE_BTN), "Update");
    assert_eq!(harness.text_content(DELETE_BTN), "Delete");

    assert!(harness.attr(UPDATE_BTN, "disabled").is_some());
    assert!(harness.attr(DELETE_BTN, "disabled").is_some());
}

#[test]
fn create_adds_one_entry_from_the_typed_fields() {
    for incremental in [false, true] {
        let mut harness = stand::boot(LeanTask::Crud, stand::options(incremental));
        let before = harness.query_all(LIST_ITEM).len();
        assert!(before > 0);
        assert_eq!(harness.text_content(CREATE_BTN), "Create");

        harness.click(NAME_INPUT);
        harness.type_text("Ada");
        harness.click(SURNAME_INPUT);
        harness.type_text("Lovelace");
        harness.click(CREATE_BTN);

        let items = harness.query_all(LIST_ITEM);
        assert_eq!(items.len(), before + 1);
        let added = harness
            .base()
            .get_node(*items.last().unwrap())
            .unwrap()
            .text_content();
        assert!(
            added.contains("Ada") && added.contains("Lovelace"),
            "added entry = {added:?}"
        );
    }
}
