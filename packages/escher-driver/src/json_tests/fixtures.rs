//! The fixture builders of the answer writer's tests: escaped strings, keys read off a
//! written form, and the outcomes, caused items, findings, actions, layout readings and
//! graphs the tests write.

use dioxus_native_dom::{
    DeclaredWrite, GraphHandler, GraphNode, GraphState, GraphWrite, GraphWriter, Length, PlacedBy,
    ShiftBy, SizeCause, SizeReason, SnapshotDiff, UiGraph, WriteFlag,
};

use super::super::string;
use crate::actions::ActionNode;
use crate::caused::{Caused, CausedKind, CausedLevel, CausedPast};
use crate::layout::{LayoutExplanation, Placement, Placer, ShiftReading};
use crate::lint::{Bounds, Defect, Finding, Reason};
use crate::refusal::Cause;
use crate::schema::VerbSpec;

pub(super) fn written(text: &str) -> String {
    let mut out = String::new();
    string(&mut out, text);
    out
}

/// The keys of an object's own members, in order, read off its written form.
pub(super) fn keys(json: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    let mut current = String::new();
    let mut last_string = None;
    for character in json.chars() {
        if in_string {
            match (escaped, character) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => {
                    in_string = false;
                    last_string = Some(std::mem::take(&mut current));
                    continue;
                }
                _ => {}
            }
            current.push(character);
            continue;
        }
        match character {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            ':' if depth == 1 => keys.extend(last_string.take()),
            _ => {}
        }
    }
    keys
}

pub(super) fn field_names(verb: &VerbSpec, without: &[&str]) -> Vec<String> {
    verb.fields
        .iter()
        .map(|field| field.name)
        .filter(|name| !without.contains(name))
        .map(str::to_string)
        .collect()
}

pub(super) fn no_diff() -> SnapshotDiff {
    SnapshotDiff {
        added: Vec::new(),
        removed: Vec::new(),
        changed: Vec::new(),
    }
}

pub(super) fn text(text: &str, cut: usize) -> escher_telemetry::Text {
    escher_telemetry::Text {
        text: text.to_string(),
        cut,
    }
}

pub(super) fn caused_item(
    kind: CausedKind,
    level: CausedLevel,
    handler: Option<(&str, Option<&str>, &str)>,
    message: &str,
) -> Caused {
    Caused {
        kind,
        level,
        source: None,
        target: None,
        message: text(message, 0),
        file: None,
        line: None,
        column: None,
        boundary: None,
        chain: Vec::new(),
        chain_past: 0,
        fields: Vec::new(),
        fields_past: 0,
        id: handler.map(|(id, ..)| id.to_string()),
        owner: handler.and_then(|(_, owner, _)| owner.map(str::to_string)),
        event: handler.map(|(.., event)| event.to_string()),
    }
}

pub(super) fn every_caused() -> Vec<Caused> {
    let mut panicked = caused_item(
        CausedKind::Panic,
        CausedLevel::Error,
        Some(("go", Some("form"), "click")),
        "index out of bounds",
    );
    panicked.file = Some("src/app.rs".to_string());
    panicked.line = Some(12);
    panicked.column = Some(5);
    let mut errored = caused_item(
        CausedKind::Error,
        CausedLevel::Error,
        Some(("go", None, "click")),
        "could not load",
    );
    errored.boundary = Some(dioxus_native_dom::Boundary::Own);
    errored.chain = vec![text("bad \"seats\"", 0), text("x", 2)];
    errored.chain_past = 1;
    let mut logged = caused_item(
        CausedKind::Log,
        CausedLevel::Debug,
        Some(("a\"b", Some("form"), "input")),
        "line one\nline two\u{1b}",
    );
    logged.source = Some(escher_telemetry::Source::Engine);
    logged.target = Some("blitz_dom::events".to_string());
    logged.message.cut = 7;
    logged.file = Some("C:\\src\\a.rs".to_string());
    logged.line = Some(3);
    logged.fields = vec![
        escher_telemetry::KeptField {
            name: "href".to_string(),
            value: text(escher_telemetry::REDACTED, 0),
        },
        escher_telemetry::KeptField {
            name: "seats".to_string(),
            value: text("4", 0),
        },
    ];
    logged.fields_past = 4;
    vec![panicked, errored, logged, log_without_handler()]
}

pub(super) fn log_without_handler() -> Caused {
    let mut logged = caused_item(CausedKind::Log, CausedLevel::Warn, None, "full");
    logged.source = Some(escher_telemetry::Source::App);
    logged.target = Some("app".to_string());
    logged
}

pub(super) fn past() -> Vec<CausedPast> {
    vec![
        CausedPast {
            kind: CausedKind::Log,
            source: Some(escher_telemetry::Source::App),
            count: 24,
        },
        CausedPast {
            kind: CausedKind::Panic,
            source: None,
            count: 1,
        },
    ]
}

pub(super) fn rect(x: f64, y: f64, width: f64, height: f64) -> Bounds {
    Bounds {
        x,
        y,
        width,
        height,
    }
}

/// One finding of every reason, a truncation with no visible part among them.
pub(super) fn every_reason() -> Vec<Finding> {
    let found = |id: &str, defect, bounds, reason| Finding {
        id: id.to_string(),
        defect,
        bounds,
        reason,
    };
    let (cut, gone) = (rect(0.0, 40.0, 80.0, 18.0), rect(900.0, 40.0, 80.0, 18.0));
    vec![
        found(
            "a",
            Defect::Overlap,
            rect(0.0, 0.0, 10.0, 10.0),
            Reason::Overlap {
                other: "b".to_string(),
                area: rect(5.0, 5.0, 5.0, 5.0),
            },
        ),
        found(
            "c",
            Defect::Clipped,
            rect(0.0, 20.0, 200.0, 18.0),
            Reason::Clipped {
                visible: rect(0.0, 20.0, 100.0, 18.0),
            },
        ),
        found(
            "d",
            Defect::Truncated,
            cut,
            Reason::Truncated {
                text: rect(0.0, 40.0, 120.5, 18.0),
                visible: Some(cut),
            },
        ),
        found(
            "e",
            Defect::Truncated,
            gone,
            Reason::Truncated {
                text: rect(900.0, 40.0, 120.5, 18.0),
                visible: None,
            },
        ),
        found("e", Defect::OffScreen, gone, Reason::OffScreen),
        found(
            "f",
            Defect::ZeroSize,
            rect(0.0, 60.0, 0.0, 0.0),
            Reason::ZeroSize,
        ),
        found(
            "g",
            Defect::Contrast,
            rect(0.0, 80.0, 50.0, 18.0),
            Reason::Contrast {
                ratio: 2.85,
                required: 4.5,
            },
        ),
    ]
}

/// A graph holding every optional key and both flags: a select owning its option, a node
/// outside every component, and a state the app root registered.
/// Three actionable nodes: one with a verb accepted and one refused, one served by an
/// owner, one whose id is a path.
pub(super) fn action_nodes() -> Vec<ActionNode> {
    use crate::actions::VerbReading;
    let reading = |verb, cause| VerbReading { verb, cause };
    vec![
        ActionNode {
            id: "pick".to_string(),
            owner: None,
            keyed: true,
            verbs: vec![
                reading("click", None),
                reading("type", Some(Cause::Disabled)),
            ],
        },
        ActionNode {
            id: "one".to_string(),
            owner: Some("pick".to_string()),
            keyed: true,
            verbs: vec![reading("click", Some(Cause::Covered))],
        },
        ActionNode {
            id: "/div:0".to_string(),
            owner: None,
            keyed: false,
            verbs: Vec::new(),
        },
    ]
}

/// An explanation holding every optional key: a container, a relative inset, a shift by a
/// holder and one by the viewport, a flex input on its width.
pub(super) fn an_explanation() -> LayoutExplanation {
    use dioxus_native_dom::FlexInputs;
    let reason = |px, by, size, flex| SizeReason {
        px,
        by,
        size,
        min: Length::Auto,
        max: Length::None,
        flex,
    };
    LayoutExplanation {
        id: "save".to_string(),
        bounds: rect(8.0, 20.5, 120.0, 32.0),
        laid_out: rect(8.0, 40.5, 120.0, 32.0),
        placed: Placement {
            placer: Placer::Engine(PlacedBy::Flex),
            container: Some("toolbar".to_string()),
            at: (8.0, 0.5),
            relative: Some((0.0, 2.0)),
        },
        shifts: vec![
            ShiftReading {
                by: ShiftBy::Scroll,
                holder: Some("list".to_string()),
                dx: 0.0,
                dy: -20.0,
            },
            ShiftReading {
                by: ShiftBy::Scroll,
                holder: None,
                dx: 0.0,
                dy: 0.0,
            },
        ],
        width: reason(
            120.0,
            SizeCause::FlexGrow,
            Length::Auto,
            Some(FlexInputs {
                basis: Length::Px(80.0),
                grow: 1.0,
                shrink: 0.5,
            }),
        ),
        height: reason(32.0, SizeCause::Style, Length::Px(32.0), None),
    }
}

pub(super) fn a_graph() -> UiGraph {
    let write = |state: &str, declared, confirmed, flag| GraphWrite {
        state: state.to_string(),
        declared,
        confirmed,
        flag,
    };
    let writer = |id: &str, event: &str, declared, confirmed, flag| GraphWriter {
        id: id.to_string(),
        event: event.to_string(),
        declared,
        confirmed,
        flag,
    };
    let handler = |event: &str, writes| GraphHandler {
        event: event.to_string(),
        writes,
    };
    let mode = || vec!["mode".to_string()];
    let form = || Some("Form".to_string());
    UiGraph {
        nodes: vec![
            GraphNode {
                id: "pick".to_string(),
                owner: None,
                component: form(),
                handlers: vec![handler("input", vec![write("mode", true, true, None)])],
                reads: mode(),
            },
            GraphNode {
                id: "one".to_string(),
                owner: Some("pick".to_string()),
                component: form(),
                handlers: Vec::new(),
                reads: mode(),
            },
            GraphNode {
                id: "go".to_string(),
                owner: None,
                component: None,
                handlers: vec![handler(
                    "click",
                    vec![
                        write("mode", false, true, Some(WriteFlag::Undeclared)),
                        write("a\"b", true, false, Some(WriteFlag::Unconfirmed)),
                    ],
                )],
                reads: Vec::new(),
            },
        ],
        states: vec![
            GraphState {
                name: "mode".to_string(),
                component: form(),
                declared: vec![DeclaredWrite::new("pick", "input")],
                writers: vec![
                    writer("pick", "input", true, true, None),
                    writer("go", "click", false, true, Some(WriteFlag::Undeclared)),
                ],
                readers: vec!["Form".to_string()],
            },
            GraphState {
                name: "a\"b".to_string(),
                component: None,
                declared: vec![DeclaredWrite::new("g*", "click")],
                writers: vec![writer(
                    "go",
                    "click",
                    true,
                    false,
                    Some(WriteFlag::Unconfirmed),
                )],
                readers: Vec::new(),
            },
        ],
    }
}
