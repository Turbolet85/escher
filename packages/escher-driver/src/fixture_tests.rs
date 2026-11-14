use blitz_test_harness::Harness;

use super::*;
use crate::command::{Command, Key, validate};
use crate::session::valid_label;

const STEPS: [FixtureStep; 4] = [
    FixtureStep::Click("a-button"),
    FixtureStep::Type("a-field", "Ada"),
    FixtureStep::Press("arrow-down"),
    FixtureStep::Advance(250),
];

const TABLE: [Fixture; 3] = [
    Fixture::new("fourteen-people", &[FixtureStep::Click("a-button")]),
    Fixture::new("person-selected", &STEPS),
    Fixture::new("x-7", &[]),
];

static FIXTURES: [Fixture; 3] = TABLE;

#[test]
fn each_step_converts_to_the_call_validate_admits_in_schema_order() {
    // Each step, the verb its call names, its argument names in order and the command it is.
    let rows: [(FixtureStep, &str, &[&str], Command); 4] = [
        (
            STEPS[0],
            "click",
            &["id"],
            Command::Click {
                id: "a-button".to_string(),
            },
        ),
        (
            STEPS[1],
            "type",
            &["id", "text"],
            Command::Type {
                id: "a-field".to_string(),
                text: "Ada".to_string(),
            },
        ),
        (
            STEPS[2],
            "press",
            &["key"],
            Command::Press {
                key: Key::ArrowDown,
                shift: false,
            },
        ),
        (STEPS[3], "advance", &["ms"], Command::Advance { ms: 250 }),
    ];
    assert_eq!(rows.len(), 4);
    for (row, (step, verb, names, command)) in rows.into_iter().enumerate() {
        let call = step.call();
        assert!(call.verb == verb, "row {row}");
        let called: Vec<&str> = call.args.iter().map(|(name, _)| name.as_str()).collect();
        assert!(called == names, "row {row}");
        assert!(validate(&call) == Ok(command), "row {row}");
    }
}

#[test]
fn a_fixture_is_found_by_its_whole_name_alone() {
    for (index, fixture) in FIXTURES.iter().enumerate() {
        assert!(
            Fixture::named(&FIXTURES, fixture.name()) == Ok(&FIXTURES[index]),
            "index {index}"
        );
    }
    let over_long = "x".repeat(33);
    let refused = [
        "",
        "absent",
        over_long.as_str(),
        "Fourteen-people",
        "FOURTEEN-PEOPLE",
        "fourteen-people/",
        "person/selected",
        "x-7.",
        "fourteen-peopl",
        "fourteen-people ",
    ];
    assert_eq!(refused.len(), 10);
    for (index, name) in refused.into_iter().enumerate() {
        assert!(
            Fixture::named(&FIXTURES, name) == Err(ReachError::UnknownState),
            "index {index}"
        );
    }
}

#[test]
fn every_name_of_the_table_is_of_the_label_form() {
    assert_eq!(FIXTURES.len(), 3);
    for (index, fixture) in FIXTURES.iter().enumerate() {
        assert!(valid_label(fixture.name()), "index {index}");
    }
}

#[test]
fn a_reach_error_reads_fixed_text_naming_no_name_tried() {
    let rows = [
        (
            ReachError::UnknownState,
            "unknown-state: no fixture state of the app carries the name",
        ),
        (
            ReachError::Refused {
                step: 1,
                cause: Cause::Disabled,
            },
            "refused: step 1 of the fixture state was refused disabled",
        ),
        (
            ReachError::NotSettled {
                step: 0,
                busy: Busy::Loads,
            },
            "not-settled: step 0 of the fixture state left loads outstanding",
        ),
    ];
    assert_eq!(rows.len(), 3);
    for (row, (error, text)) in rows.into_iter().enumerate() {
        assert!(error.to_string() == text, "row {row}");
        for fixture in &FIXTURES {
            assert!(!error.to_string().contains(fixture.name()), "row {row}");
        }
    }
}

fn empty() -> Session {
    Session::start("counter", || {
        Harness::from_component(|| Ok(Default::default()))
    })
    .expect("the session starts")
}

#[test]
fn a_reach_stops_at_the_first_refused_step_naming_its_index_and_cause() {
    static STOPS: [FixtureStep; 3] = [
        FixtureStep::Advance(1),
        FixtureStep::Click("nobody"),
        FixtureStep::Advance(1),
    ];
    let mut session = empty();
    assert!(
        session.reach(&Fixture::new("stops", &STOPS))
            == Err(ReachError::Refused {
                step: 1,
                cause: Cause::NotFound,
            })
    );
    assert!(session.reach(&Fixture::new("quiet", &[FixtureStep::Advance(1)])) == Ok(()));
    assert!(session.reach(&Fixture::new("none", &[])) == Ok(()));
}

#[test]
fn only_an_unsettled_acting_outcome_reads_busy() {
    let diff = || dioxus_native_dom::SnapshotDiff {
        added: Vec::new(),
        removed: Vec::new(),
        changed: Vec::new(),
    };
    let rows = [
        (
            Outcome::Acted {
                settled: false,
                busy: Some(Busy::Render),
                diff: diff(),
            },
            Some(Busy::Render),
        ),
        (
            Outcome::Advanced {
                settled: false,
                busy: Some(Busy::Loads),
                diff: diff(),
                advanced_ms: 1,
            },
            Some(Busy::Loads),
        ),
        (
            Outcome::Scrolled {
                settled: false,
                busy: Some(Busy::Layout),
                diff: diff(),
                in_view: true,
            },
            Some(Busy::Layout),
        ),
        (
            Outcome::Acted {
                settled: true,
                busy: None,
                diff: diff(),
            },
            None,
        ),
        (
            Outcome::Screen {
                text: String::new(),
            },
            None,
        ),
    ];
    assert_eq!(rows.len(), 5);
    for (row, (outcome, busy)) in rows.into_iter().enumerate() {
        assert!(unsettled(&outcome) == busy, "row {row}");
    }
}
