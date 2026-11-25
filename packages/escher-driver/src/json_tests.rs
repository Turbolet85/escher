//! The answer writer's tests: every written form pinned by its literal line.

use std::io;

use blitz_test_harness::Busy;
use dioxus_native_dom::{
    Length, MASKED_VALUE, PlacedBy, Reading, ShiftBy, SizeCause, SizeKeyword, SnapshotDiff, UiGraph,
};

use super::layout::{cause_word, length, placement, placer_word, shift_word};
use super::snapshot::{NodeParts, node, reading_word};
use super::*;
use crate::layout::{Placement, Placer};
use crate::refusal::{CAUSES, Cause, Fault};
use crate::schema::{self, NODE_FIELDS};

#[path = "json_tests/fixtures.rs"]
mod fixtures;

use fixtures::*;

#[test]
fn a_string_is_written_with_its_escapes() {
    let rows = [
        ("", r#""""#),
        ("save", r#""save""#),
        ("a\"b", r#""a\"b""#),
        ("a\\b", r#""a\\b""#),
        ("a\nb", r#""a\nb""#),
        ("a\rb", r#""a\rb""#),
        ("a\tb", r#""a\tb""#),
        ("a\u{0}b", r#""a\u0000b""#),
        ("a\u{1b}b", r#""a\u001bb""#),
        ("a\u{1f}b", r#""a\u001fb""#),
        ("a b", r#""a b""#),
        ("form//input[2]:x", r#""form//input[2]:x""#),
        ("é ✓ 日本", "\"é ✓ 日本\""),
        ("\u{7f}", "\"\u{7f}\""),
        ("'", r#""'""#),
    ];
    assert_eq!(rows.len(), 15);
    for (row, (text, json)) in rows.into_iter().enumerate() {
        assert!(written(text) == json, "row {row}");
    }
    for code in 0..0x20u32 {
        let control = char::from_u32(code).expect("a control character");
        let json = written(&control.to_string());
        assert!(json.is_ascii() && !json.chars().any(char::is_control));
    }
}

#[test]
fn a_number_is_the_shortest_decimal_and_null_when_not_finite() {
    let rows = [
        (0.0, "0"),
        (24.0, "24"),
        (112.796875, "112.796875"),
        (-269.0, "-269"),
        (64.5, "64.5"),
        (0.1, "0.1"),
        (1e21, "1000000000000000000000"),
        (f64::NAN, "null"),
        (f64::INFINITY, "null"),
        (f64::NEG_INFINITY, "null"),
    ];
    assert_eq!(rows.len(), 10);
    for (row, (value, json)) in rows.into_iter().enumerate() {
        let mut out = String::new();
        number(&mut out, value);
        assert!(out == json, "row {row}");
    }
}

#[test]
fn a_node_is_written_with_the_schema_fields_it_has() {
    let full = node(&NodeParts {
        id: "who",
        parent: Some("form"),
        role: "TextInput",
        name: "Name:",
        enabled: Some(true),
        checked: Some(false),
        value: Some("Ada"),
        selected: Some(false),
        pressed: Some(true),
        invalid: true,
        focused: true,
        clickable: true,
        highlighted: true,
        bounds: [8.0, 8.5, 200.0, 24.0],
        differs: Some(&[Reading::Value, Reading::Focused, Reading::Highlighted]),
    });
    assert_eq!(
        full,
        r#"{"id":"who","parent":"form","role":"TextInput","name":"Name:","enabled":true,"checked":false,"value":"Ada","selected":false,"pressed":true,"invalid":true,"focused":true,"clickable":true,"highlighted":true,"bounds":[8,8.5,200,24],"differs":["value","focused","highlighted"]}"#
    );
    assert_eq!(keys(&full), NODE_FIELDS.map(|key| key.name));

    let bare = node(&NodeParts {
        id: "root",
        parent: None,
        role: "GenericContainer",
        name: "",
        enabled: None,
        checked: None,
        value: None,
        selected: None,
        pressed: None,
        invalid: false,
        focused: false,
        clickable: false,
        highlighted: false,
        bounds: [0.0, 0.0, f64::NAN, 600.0],
        differs: None,
    });
    assert_eq!(
        bare,
        r#"{"id":"root","role":"GenericContainer","name":"","invalid":false,"focused":false,"clickable":false,"bounds":[0,0,null,600]}"#
    );
    assert_eq!(
        keys(&bare),
        [
            "id",
            "role",
            "name",
            "invalid",
            "focused",
            "clickable",
            "bounds"
        ]
    );

    // A masked control's value is the mask: the writer writes what the snapshot reads.
    let masked = node(&NodeParts {
        id: "secret",
        parent: None,
        role: "PasswordInput",
        name: "",
        enabled: Some(true),
        checked: None,
        value: Some(MASKED_VALUE),
        selected: None,
        pressed: None,
        invalid: false,
        focused: true,
        clickable: false,
        highlighted: false,
        bounds: [0.0, 0.0, 1.0, 1.0],
        differs: None,
    });
    assert!(masked.contains(&format!(r#""value":"{MASKED_VALUE}""#)));
    assert_eq!(
        list([full.clone(), bare.clone()].into_iter()),
        format!("[{full},{bare}]")
    );
    assert_eq!(list(std::iter::empty()), "[]");

    // Each reading is written as the node key it reads.
    assert_eq!(Reading::ALL.len(), 13);
    for (row, reading) in Reading::ALL.into_iter().enumerate() {
        let word = reading_word(reading);
        assert!(word == NODE_FIELDS[row + 1].name, "row {row}");
        assert!(word == format!("{reading:?}").to_lowercase(), "row {row}");
    }
}

#[test]
fn each_outcome_is_written_with_its_verbs_fields_in_order() {
    let removed = SnapshotDiff {
        removed: vec!["row-1".to_string(), "a\"b".to_string()],
        ..no_diff()
    };
    let rows = [
        (
            Outcome::Screen {
                text: "Button \"Save\" id=\"save\" @0,0 1x1\n".to_string(),
            },
            &schema::SNAPSHOT,
            &["part", "parts"][..],
            r#"{"text":"Button \"Save\" id=\"save\" @0,0 1x1\n"}"#,
        ),
        (
            Outcome::Part {
                text: "  Button \"Save\" id=\"save\" @0,0 1x1\n".to_string(),
                part: 2,
                parts: 3,
            },
            &schema::SNAPSHOT,
            &[][..],
            r#"{"text":"  Button \"Save\" id=\"save\" @0,0 1x1\n","part":2,"parts":3}"#,
        ),
        (
            Outcome::Part {
                text: String::new(),
                part: 4,
                parts: 3,
            },
            &schema::SNAPSHOT,
            &[][..],
            r#"{"text":"","part":4,"parts":3}"#,
        ),
        (
            Outcome::Acted {
                settled: true,
                busy: None,
                diff: no_diff(),
                caused: Vec::new(),
                caused_past: Vec::new(),
            },
            &schema::CLICK,
            &["busy", "caused", "caused_past"][..],
            r#"{"settled":true,"added":[],"removed":[],"changed":[]}"#,
        ),
        (
            Outcome::Acted {
                settled: false,
                busy: Some(Busy::Loads),
                diff: removed.clone(),
                caused: Vec::new(),
                caused_past: Vec::new(),
            },
            &schema::TYPE,
            &["caused", "caused_past"][..],
            r#"{"settled":false,"busy":"loads","added":[],"removed":["row-1","a\"b"],"changed":[]}"#,
        ),
        (
            Outcome::Acted {
                settled: true,
                busy: None,
                diff: no_diff(),
                caused: every_caused(),
                caused_past: past(),
            },
            &schema::PRESS,
            &["busy"][..],
            concat!(
                r#"{"settled":true,"added":[],"removed":[],"changed":[],"caused":["#,
                r#"{"kind":"panic","level":"error","message":"index out of bounds","file":"src/app.rs","line":12,"column":5,"id":"go","owner":"form","event":"click"},"#,
                r#"{"kind":"error","level":"error","message":"could not load","boundary":"own","chain":[{"message":"bad \"seats\""},{"message":"x","cut":2}],"chain_past":1,"id":"go","event":"click"},"#,
                r#"{"kind":"log","level":"debug","source":"engine","target":"blitz_dom::events","message":"line one\nline two\u001b","cut":7,"file":"C:\\src\\a.rs","line":3,"fields":[{"name":"href","value":"[redacted]"},{"name":"seats","value":"4"}],"fields_past":4,"id":"a\"b","owner":"form","event":"input"},"#,
                r#"{"kind":"log","level":"warn","source":"app","target":"app","message":"full","fields":[],"fields_past":0}]"#,
                r#","caused_past":[{"kind":"log","source":"app","count":24},{"kind":"panic","count":1}]}"#,
            ),
        ),
        (
            Outcome::Advanced {
                settled: true,
                busy: None,
                diff: no_diff(),
                advanced_ms: 200,
                caused: Vec::new(),
                caused_past: Vec::new(),
            },
            &schema::ADVANCE,
            &["busy", "caused", "caused_past"][..],
            r#"{"settled":true,"added":[],"removed":[],"changed":[],"advanced_ms":200}"#,
        ),
        (
            Outcome::Scrolled {
                settled: false,
                busy: Some(Busy::Render),
                diff: no_diff(),
                in_view: false,
                caused: Vec::new(),
                caused_past: Vec::new(),
            },
            &schema::SCROLL,
            &["caused", "caused_past"][..],
            r#"{"settled":false,"busy":"render","added":[],"removed":[],"changed":[],"in_view":false}"#,
        ),
        (
            Outcome::Advanced {
                settled: true,
                busy: None,
                diff: no_diff(),
                advanced_ms: 5,
                caused: vec![log_without_handler()],
                caused_past: Vec::new(),
            },
            &schema::ADVANCE,
            &["busy", "caused_past"][..],
            r#"{"settled":true,"added":[],"removed":[],"changed":[],"advanced_ms":5,"caused":[{"kind":"log","level":"warn","source":"app","target":"app","message":"full","fields":[],"fields_past":0}]}"#,
        ),
        (
            Outcome::Scrolled {
                settled: true,
                busy: None,
                diff: no_diff(),
                in_view: true,
                caused: Vec::new(),
                caused_past: past(),
            },
            &schema::SCROLL,
            &["busy", "caused"][..],
            r#"{"settled":true,"added":[],"removed":[],"changed":[],"in_view":true,"caused_past":[{"kind":"log","source":"app","count":24},{"kind":"panic","count":1}]}"#,
        ),
        (
            Outcome::Shot {
                width: 800,
                height: 600,
                png: b"foobar".to_vec(),
            },
            &schema::SCREENSHOT,
            &[][..],
            r#"{"width":800,"height":600,"png":"Zm9vYmFy"}"#,
        ),
        (
            Outcome::Linted {
                findings: Vec::new(),
            },
            &schema::LINT,
            &[][..],
            r#"{"findings":[]}"#,
        ),
        (
            Outcome::Linted {
                findings: every_reason(),
            },
            &schema::LINT,
            &[][..],
            concat!(
                r#"{"findings":[{"id":"a","defect":"overlap","bounds":[0,0,10,10],"other":"b","area":[5,5,5,5]},"#,
                r#"{"id":"c","defect":"clipped","bounds":[0,20,200,18],"visible":[0,20,100,18]},"#,
                r#"{"id":"d","defect":"truncated","bounds":[0,40,80,18],"text":[0,40,120.5,18],"visible":[0,40,80,18]},"#,
                r#"{"id":"e","defect":"truncated","bounds":[900,40,80,18],"text":[900,40,120.5,18]},"#,
                r#"{"id":"e","defect":"off-screen","bounds":[900,40,80,18]},"#,
                r#"{"id":"f","defect":"zero-size","bounds":[0,60,0,0]},"#,
                r#"{"id":"g","defect":"contrast","bounds":[0,80,50,18],"ratio":2.85,"required":4.5}]}"#,
            ),
        ),
        (
            Outcome::Graphed {
                graph: UiGraph::default(),
            },
            &schema::GRAPH,
            &["part", "parts"][..],
            r#"{"nodes":[],"states":[]}"#,
        ),
        (
            Outcome::GraphPart {
                graph: UiGraph {
                    nodes: a_graph().nodes.into_iter().skip(1).take(1).collect(),
                    states: Vec::new(),
                },
                part: 2,
                parts: 3,
            },
            &schema::GRAPH,
            &[][..],
            concat!(
                r#"{"nodes":[{"id":"one","owner":"pick","component":"Form","handlers":[],"reads":["mode"]}],"#,
                r#""states":[],"part":2,"parts":3}"#,
            ),
        ),
        (
            Outcome::GraphPart {
                graph: UiGraph::default(),
                part: 4,
                parts: 3,
            },
            &schema::GRAPH,
            &[][..],
            r#"{"nodes":[],"states":[],"part":4,"parts":3}"#,
        ),
        (
            Outcome::Graphed { graph: a_graph() },
            &schema::GRAPH,
            &["part", "parts"][..],
            concat!(
                r#"{"nodes":[{"id":"pick","component":"Form","handlers":[{"event":"input","writes":[{"state":"mode","declared":true,"confirmed":true}]}],"reads":["mode"]},"#,
                r#"{"id":"one","owner":"pick","component":"Form","handlers":[],"reads":["mode"]},"#,
                r#"{"id":"go","handlers":[{"event":"click","writes":[{"state":"mode","declared":false,"confirmed":true,"flag":"undeclared"},"#,
                r#"{"state":"a\"b","declared":true,"confirmed":false,"flag":"unconfirmed"}]}],"reads":[]}],"#,
                r#""states":[{"name":"mode","component":"Form","declared":[{"id":"pick","event":"input"}],"#,
                r#""writers":[{"id":"pick","event":"input","declared":true,"confirmed":true},"#,
                r#"{"id":"go","event":"click","declared":false,"confirmed":true,"flag":"undeclared"}],"readers":["Form"]},"#,
                r#"{"name":"a\"b","declared":[{"id":"g*","event":"click"}],"#,
                r#""writers":[{"id":"go","event":"click","declared":true,"confirmed":false,"flag":"unconfirmed"}],"readers":[]}]}"#,
            ),
        ),
        (
            Outcome::Actions { nodes: Vec::new() },
            &schema::ACTIONS,
            &[][..],
            r#"{"nodes":[]}"#,
        ),
        (
            Outcome::Actions {
                nodes: action_nodes(),
            },
            &schema::ACTIONS,
            &[][..],
            concat!(
                r#"{"nodes":[{"id":"pick","keyed":true,"verbs":[{"verb":"click"},{"verb":"type","cause":"disabled"}]},"#,
                r#"{"id":"one","owner":"pick","keyed":true,"verbs":[{"verb":"click","cause":"covered"}]},"#,
                r#"{"id":"/div:0","keyed":false,"verbs":[]}]}"#,
            ),
        ),
        (
            Outcome::Layout {
                explanation: an_explanation(),
            },
            &schema::LAYOUT,
            &[][..],
            concat!(
                r#"{"id":"save","bounds":{"x":8,"y":20.5,"width":120,"height":32},"#,
                r#""laid_out":{"x":8,"y":40.5,"width":120,"height":32},"#,
                r#""placed":{"by":"flex","container":"toolbar","at":{"x":8,"y":0.5},"relative":{"x":0,"y":2}},"#,
                r#""shifts":[{"by":"scroll","holder":"list","dx":0,"dy":-20},{"by":"scroll","dx":0,"dy":0}],"#,
                r#""width":{"px":120,"by":"flex-grow","size":"auto","min":"auto","max":"none","flex":{"basis":{"px":80},"grow":1,"shrink":0.5}},"#,
                r#""height":{"px":32,"by":"style","size":{"px":32},"min":"auto","max":"none"}}"#,
            ),
        ),
        (
            Outcome::Layout {
                explanation: crate::layout::unexplained("group", rect(1.0, 2.0, 3.0, 4.0)),
            },
            &schema::LAYOUT,
            &[][..],
            concat!(
                r#"{"id":"group","bounds":{"x":1,"y":2,"width":3,"height":4},"#,
                r#""laid_out":{"x":1,"y":2,"width":3,"height":4},"#,
                r#""placed":{"by":"unexplained","at":{"x":0,"y":0}},"shifts":[],"#,
                r#""width":{"px":3,"by":"layout","size":"auto","min":"auto","max":"none"},"#,
                r#""height":{"px":4,"by":"layout","size":"auto","min":"auto","max":"none"}}"#,
            ),
        ),
    ];
    assert_eq!(rows.len(), 21);
    for (row, (outcome, verb, absent, json)) in rows.into_iter().enumerate() {
        let line = outcome.to_json();
        assert!(line == json, "row {row}");
        assert!(keys(&line) == field_names(verb, absent), "row {row}");
        assert!(!line.contains('\n'), "row {row}");
    }
    assert_eq!(
        field_names(&schema::CLICK, &[]),
        field_names(&schema::PRESS, &[])
    );
    let without = shot_without_image(369, 420);
    assert_eq!(without, r#"{"width":369,"height":420}"#);
    assert_eq!(keys(&without), field_names(&schema::SCREENSHOT, &["png"]));
}

#[test]
fn every_layout_word_is_written_once() {
    let placers = [
        (Placer::Engine(PlacedBy::Block), "block"),
        (Placer::Engine(PlacedBy::Flex), "flex"),
        (Placer::Engine(PlacedBy::Grid), "grid"),
        (Placer::Engine(PlacedBy::Table), "table"),
        (Placer::Engine(PlacedBy::Inline), "inline"),
        (Placer::Engine(PlacedBy::Float), "float"),
        (Placer::Engine(PlacedBy::Absolute), "absolute"),
        (Placer::Engine(PlacedBy::Fixed), "fixed"),
        (Placer::Engine(PlacedBy::Root), "root"),
        (Placer::Engine(PlacedBy::Select), "select"),
        (Placer::Engine(PlacedBy::SelectList), "select-list"),
        (Placer::Unexplained, "unexplained"),
        (
            Placer::App {
                canvas: "board".to_string(),
            },
            "app",
        ),
    ];
    assert_eq!(placers.len(), 13);
    let shifts = [
        (ShiftBy::Scroll, "scroll"),
        (ShiftBy::Transform, "transform"),
    ];
    assert_eq!(shifts.len(), 2);
    let causes = [
        (SizeCause::Style, "style"),
        (SizeCause::Min, "min"),
        (SizeCause::Max, "max"),
        (SizeCause::FlexGrow, "flex-grow"),
        (SizeCause::FlexShrink, "flex-shrink"),
        (SizeCause::Stretch, "stretch"),
        (SizeCause::Grid, "grid"),
        (SizeCause::Table, "table"),
        (SizeCause::Replaced, "replaced"),
        (SizeCause::Content, "content"),
        (SizeCause::Layout, "layout"),
    ];
    assert_eq!(causes.len(), 11);
    let lengths = [
        (Length::Auto, r#""auto""#),
        (Length::None, r#""none""#),
        (Length::Px(12.5), r#"{"px":12.5}"#),
        (Length::Percent(50.0), r#"{"percent":50}"#),
        (Length::Calc { px: 0.1 }, r#"{"calc":0.1}"#),
        (Length::Keyword(SizeKeyword::MinContent), r#""min-content""#),
        (Length::Keyword(SizeKeyword::MaxContent), r#""max-content""#),
        (Length::Keyword(SizeKeyword::FitContent), r#""fit-content""#),
        (Length::Keyword(SizeKeyword::Stretch), r#""stretch""#),
    ];
    assert_eq!(lengths.len(), 9);

    let mut words: Vec<String> = Vec::new();
    for (row, (placer, word)) in placers.iter().enumerate() {
        assert!(placer_word(placer) == *word, "placer row {row}");
        let placed = placement(&Placement {
            placer: placer.clone(),
            container: None,
            at: (0.0, 0.0),
            relative: None,
        });
        assert!(
            placed.starts_with(&format!(r#"{{"by":"{word}""#)),
            "placer row {row}"
        );
        words.push(format!("placer {word}"));
    }
    let app = placement(&Placement {
        placer: placers[12].0.clone(),
        container: Some("page".to_string()),
        at: (1.0, 2.0),
        relative: None,
    });
    assert_eq!(
        app,
        r#"{"by":"app","container":"page","canvas":"board","at":{"x":1,"y":2}}"#
    );
    for (row, (by, word)) in shifts.into_iter().enumerate() {
        assert!(shift_word(by) == word, "shift row {row}");
        words.push(format!("shift {word}"));
    }
    for (row, (by, word)) in causes.into_iter().enumerate() {
        assert!(cause_word(by) == word, "cause row {row}");
        words.push(format!("cause {word}"));
    }
    for (row, (value, json)) in lengths.iter().enumerate() {
        assert!(length(value) == *json, "length row {row}");
        words.push(format!("length {json}"));
    }
    // No word is written for two members of one set.
    let total = words.len();
    words.sort();
    words.dedup();
    assert_eq!((total, words.len()), (35, 35));
    // Every word the writer writes is named in the help of the field that carries it.
    let help = |name: &str| {
        schema::LAYOUT
            .fields
            .iter()
            .find(|field| field.name == name)
            .map_or("", |field| field.help)
    };
    for (_, word) in &placers {
        assert!(help("placed").contains(word), "{word}");
    }
    for (_, word) in shifts {
        assert!(help("shifts").contains(word), "{word}");
    }
    for (_, word) in causes {
        assert!(help("width").contains(word), "{word}");
    }
    for word in [
        "auto",
        "none",
        "min-content",
        "max-content",
        "fit-content",
        "stretch",
        "px",
        "percent",
        "calc",
    ] {
        assert!(help("width").contains(word), "{word}");
    }
}

#[test]
fn each_refusal_is_written_from_its_causes_fixed_strings() {
    let faults = [
        Fault::Missing("id"),
        Fault::Unnamed,
        Fault::Repeated("id"),
        Fault::WrongKind("ms"),
        Fault::OutOfBound("text"),
    ];
    let refusals: Vec<Refusal> = CAUSES
        .into_iter()
        .map(Refusal::new)
        .chain(faults.into_iter().map(Refusal::malformed))
        .collect();
    assert_eq!(refusals.len(), 13);
    for (row, refusal) in refusals.iter().enumerate() {
        let cause = refusal.cause();
        let mut expected = format!(
            "{{\"refused\":{{\"cause\":\"{}\",\"meaning\":\"{}\",\"remedy\":\"{}\"",
            cause.name(),
            cause.meaning(),
            cause.remedy()
        );
        if let Some(fault) = refusal.fault() {
            expected.push_str(&format!(",\"fault\":\"{fault}\""));
        }
        expected.push_str("}}");
        let line = refusal.to_json();
        assert!(line == expected, "row {row}");
        assert!(keys(&line) == ["refused"], "row {row}");
        assert!(!line.contains('\n') && !line.contains('/'), "row {row}");
    }
    // The limits measured for `covered` are in every `covered` refusal an agent is handed.
    let covered = Refusal::new(Cause::Covered).to_json();
    assert!(covered.contains(
        "an option of a closed select lies under it: click the select first, then the option; \
         a box clipped on one axis stops a hit at its whole padding box, so an element lying \
         outside it along the axis it does not clip can read `covered` though it shows; a box \
         with no width or no height is hit at its centre on the element that holds it, so it \
         can read `covered` though nothing covers it"
    ));
}

#[test]
fn each_session_error_is_written_with_its_kind_its_message_and_its_remedy() {
    let rows = [
        (SessionError::AlreadyRunning, "already-running"),
        (SessionError::NoSession, "no-session"),
        (SessionError::Dead, "dead"),
        (SessionError::HostExited(Some(101)), "host-exited"),
        (SessionError::Timeout, "timeout"),
        (SessionError::Protocol, "protocol"),
        (SessionError::InvalidLabel, "invalid-label"),
        (SessionError::StateDirTooLong, "state-dir-too-long"),
        (SessionError::StateDirNotPrivate, "state-dir-not-private"),
        (SessionError::Unsupported, "unsupported"),
        (SessionError::Io(io::ErrorKind::NotFound), "io"),
        (SessionError::NotSettled(Busy::Layout), "not-settled"),
        (SessionError::AnswerTooLarge, "answer-too-large"),
        (SessionError::RecordDirNotPrivate, "record-dir-not-private"),
        (SessionError::RecordingFailed, "recording-failed"),
    ];
    assert_eq!(rows.len(), 15);
    for (row, (error, kind)) in rows.into_iter().enumerate() {
        let line = error.to_json();
        let expected = format!(
            "{{\"error\":{{\"kind\":\"{kind}\",\"message\":\"{error}\",\"remedy\":\"{}\"}}}}",
            error.remedy()
        );
        assert!(line == expected, "row {row}");
        assert!(!line.contains('\n') && !line.contains('/'), "row {row}");
        for (other, (_, later)) in rows.iter().enumerate().skip(row + 1) {
            assert!(kind != *later, "rows {row} and {other}");
        }
    }
}

#[test]
fn each_session_answer_is_written_with_its_verbs_fields_in_order() {
    let hello = Hello {
        pid: 4242,
        label: "flight-booker".to_string(),
        served: 7,
        idle_expiry_s: 1800,
    };
    let rows = [
        (
            started(&hello),
            &schema::START,
            r#"{"label":"flight-booker","pid":4242,"idle_expiry_s":1800}"#,
        ),
        (
            status(&hello),
            &schema::STATUS,
            r#"{"label":"flight-booker","pid":4242,"served":7,"idle_expiry_s":1800}"#,
        ),
        (stopped(), &schema::STOP, r#"{"stopped":true}"#),
    ];
    assert_eq!(rows.len(), 3);
    for (row, (line, verb, json)) in rows.into_iter().enumerate() {
        assert!(line == json, "row {row}");
        assert!(keys(&line) == field_names(verb, &[]), "row {row}");
    }
}

#[test]
fn a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner() {
    use crate::coverage::{Coverage, HandlerRef};

    let handler = |id: &str, owner: Option<&str>, event: &str| HandlerRef {
        id: id.to_string(),
        owner: owner.map(str::to_string),
        event: event.to_string(),
    };
    let coverage = Coverage {
        unexercised: vec![
            handler("back-btn", None, "click"),
            handler("kind-a", Some("kind"), "input"),
            handler("say \"hi\"", None, "keydown"),
        ],
        checks: 3,
        held: 2,
    };
    assert_eq!(
        coverage.to_json(),
        r#"{"unexercised":[{"id":"back-btn","event":"click"},{"id":"kind-a","owner":"kind","event":"input"},{"id":"say \"hi\"","event":"keydown"}],"checks":3,"held":2}"#
    );
    assert_eq!(keys(&coverage.to_json()), ["unexercised", "checks", "held"]);
    let none = Coverage {
        unexercised: Vec::new(),
        checks: 0,
        held: 0,
    };
    assert_eq!(none.to_json(), r#"{"unexercised":[],"checks":0,"held":0}"#);
}

/// Every command kind and brush form beside the line it writes before its tag, then a glyph run
/// of no glyph — its line written whole, tag ` node=-` included.
fn canvas_line_rows() -> (
    [(crate::canvas::CanvasCommand, &'static str); 13],
    crate::canvas::CanvasCommand,
    &'static str,
) {
    use crate::canvas::{CanvasBrush, CanvasCommand, GradientStop, PathStep, Rgba, Transform};
    let id = Transform([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    let scaled = Transform([2.0, 0.0, 0.0, 2.0, 0.1234, -0.0001]);
    let shape = vec![
        PathStep::Move(0.0, 0.0),
        PathStep::Line(10.5, 0.0),
        PathStep::Quad(1.0, 2.0, 3.0, 4.0),
        PathStep::Cubic(1.0, 2.0, 3.0, 4.0, 5.0, 6.0),
        PathStep::Close,
    ];
    let red = Rgba([255, 0, 0, 255]);
    let rows: [(CanvasCommand, &str); 13] = [
        (
            CanvasCommand::Layer {
                alpha: 0.5,
                mix: "multiply",
                compose: "src-over",
                fill: "even-odd",
                transform: id,
                clip: vec![PathStep::Move(0.0, 0.0), PathStep::Close],
                filter: true,
                backdrop: true,
            },
            "layer alpha=0.5 blend=multiply,src-over fill=even-odd t=id clip=M0,0Z filter backdrop",
        ),
        (
            CanvasCommand::Layer {
                alpha: 1.0,
                mix: "normal",
                compose: "src-over",
                fill: "non-zero",
                transform: scaled,
                clip: Vec::new(),
                filter: false,
                backdrop: false,
            },
            "layer alpha=1 blend=normal,src-over fill=non-zero t=2,0,0,2,0.123,0 clip=",
        ),
        (
            CanvasCommand::Clip {
                transform: id,
                clip: shape.clone(),
            },
            "clip t=id clip=M0,0L10.5,0Q1,2,3,4C1,2,3,4,5,6Z",
        ),
        (
            CanvasCommand::Mask {
                mode: "luminance",
                transform: id,
                clip: Vec::new(),
            },
            "mask mode=luminance t=id clip=",
        ),
        (CanvasCommand::Pop, "pop"),
        (
            CanvasCommand::Fill {
                fill: "non-zero",
                brush: CanvasBrush::Solid(red),
                brush_transform: None,
                transform: id,
                shape: vec![PathStep::Move(1.0, 1.0)],
            },
            "fill fill=non-zero brush=#ff0000ff t=id shape=M1,1",
        ),
        (
            CanvasCommand::Fill {
                fill: "even-odd",
                brush: CanvasBrush::Gradient {
                    kind: "radial",
                    stops: vec![
                        GradientStop {
                            offset: 0.0,
                            colour: red,
                        },
                        GradientStop {
                            offset: 0.25,
                            colour: Rgba([0, 16, 32, 128]),
                        },
                    ],
                },
                brush_transform: Some(scaled),
                transform: id,
                shape: Vec::new(),
            },
            "fill fill=even-odd brush=radial:0#ff0000ff,0.25#00102080 bt=2,0,0,2,0.123,0 t=id shape=",
        ),
        (
            CanvasCommand::Fill {
                fill: "non-zero",
                brush: CanvasBrush::Image(1),
                brush_transform: None,
                transform: id,
                shape: Vec::new(),
            },
            "fill fill=non-zero brush=image:1 t=id shape=",
        ),
        (
            CanvasCommand::Fill {
                fill: "non-zero",
                brush: CanvasBrush::Resource,
                brush_transform: None,
                transform: id,
                shape: Vec::new(),
            },
            "fill fill=non-zero brush=resource t=id shape=",
        ),
        (
            CanvasCommand::Fill {
                fill: "non-zero",
                brush: CanvasBrush::Pattern,
                brush_transform: None,
                transform: id,
                shape: Vec::new(),
            },
            "fill fill=non-zero brush=pattern t=id shape=",
        ),
        (
            CanvasCommand::Stroke {
                width: 2.0,
                join: "round",
                start_cap: "butt",
                end_cap: "round",
                miter: 4.0,
                dashes: vec![4.0, 2.0],
                dash_offset: 1.0,
                brush: CanvasBrush::Solid(red),
                brush_transform: None,
                transform: id,
                shape: Vec::new(),
            },
            "stroke w=2 join=round cap=butt,round miter=4 dash=4,2@1 brush=#ff0000ff t=id shape=",
        ),
        (
            CanvasCommand::Glyphs {
                font: 0,
                size: 13.333_333,
                brush: CanvasBrush::Solid(red),
                alpha: 0.5,
                transform: id,
                count: 3,
                origins: Some([1.0, f64::NAN, 3.0, 4.0]),
                ids: 0x00ab_cdef_0123_4567,
            },
            "glyphs font=0 size=13.333 brush=#ff0000ff alpha=0.5 t=id n=3 at=1,null,3,4 ids=00abcdef01234567",
        ),
        (
            CanvasCommand::Shadow {
                transform: id,
                rect: [0.0, 0.0, 10.0, 20.0],
                radii: [1.0; 8],
                offset: (2.0, -2.0),
                spread: 0.0,
                deviation: 3.0,
                colour: Rgba([0, 0, 0, 64]),
                kind: "outset",
            },
            "shadow t=id box=0,0,10,20 radii=1,1,1,1,1,1,1,1 offset=2,-2 spread=0 dev=3 colour=#00000040 kind=outset",
        ),
    ];
    // A glyph run of no glyph has no origins' box, and its alpha of 1 is left out.
    let empty = CanvasCommand::Glyphs {
        font: 2,
        size: 12.0,
        brush: CanvasBrush::Solid(red),
        alpha: 1.0,
        transform: id,
        count: 0,
        origins: None,
        ids: 0,
    };
    (
        rows,
        empty,
        "glyphs font=2 size=12 brush=#ff0000ff t=id n=0 at=none ids=0000000000000000 node=-\n",
    )
}

#[test]
fn every_canvas_command_kind_and_brush_form_is_written_once() {
    let (rows, empty, empty_line) = canvas_line_rows();
    assert_eq!(rows.len(), 13);
    for (row, (command, line)) in rows.iter().enumerate() {
        let written = super::canvas::canvas_lines(std::slice::from_ref(command), &[None], 0);
        assert!(written == [format!("{line} node=-\n")], "row {row}");
        let grouped = super::canvas::canvas_lines(std::slice::from_ref(command), &[Some(7)], 0);
        assert!(grouped == [format!("{line} node=7\n")], "row {row} grouped");
    }
    assert_eq!(super::canvas::canvas_lines(&[empty], &[], 0), [empty_line]);
}

#[test]
fn no_canvas_line_holds_a_key_twice() {
    let (rows, empty, _) = canvas_line_rows();
    assert_eq!(rows.len(), 13);
    let commands: Vec<_> = rows
        .into_iter()
        .map(|(command, _)| command)
        .chain([empty])
        .collect();
    assert_eq!(commands.len(), 14);
    for (row, command) in commands.iter().enumerate() {
        for node in [None, Some(7)] {
            let lines = super::canvas::canvas_lines(std::slice::from_ref(command), &[node], 0);
            let mut keys: Vec<&str> = lines
                .iter()
                .flat_map(|line| line.split_whitespace())
                .filter_map(|word| word.split_once('=').map(|(key, _)| key))
                .collect();
            let all = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert!(
                keys.len() == all,
                "row {row}, grouped {}: a key stands twice",
                node.is_some()
            );
        }
    }
}

#[test]
fn a_canvas_summary_and_a_full_dump_are_written_whole_and_a_part_indents_from_its_depth() {
    use crate::canvas::{CanvasBrush, CanvasCommand, CanvasReading, PathStep, Rgba, Transform};
    let id = Transform([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    let commands = vec![
        CanvasCommand::Clip {
            transform: id,
            clip: vec![PathStep::Move(0.0, 0.0), PathStep::Close],
        },
        CanvasCommand::Fill {
            fill: "non-zero",
            brush: CanvasBrush::Solid(Rgba([1, 2, 3, 255])),
            brush_transform: None,
            transform: id,
            shape: vec![PathStep::Move(1.0, 1.0)],
        },
        CanvasCommand::Pop,
    ];
    let summary = Outcome::CanvasSummary {
        canvas: true,
        width: 300.0,
        height: 120.5,
        kinds: [0, 1, 0, 1, 1, 0, 0, 0],
        depth: 1,
        digest: 0x0123_4567_89ab_cdef,
    };
    assert_eq!(
        summary.to_json(),
        r##"{"canvas":true,"width":300,"height":120.5,"kinds":{"layer":0,"clip":1,"mask":0,"pop":1,"fill":1,"stroke":0,"glyphs":0,"shadow":0},"depth":1,"digest":"0123456789abcdef"}"##
    );
    let reading = CanvasReading {
        canvas: true,
        width: 300.0,
        height: 120.5,
        commands: commands.clone(),
        nodes: vec![None, Some(1), None],
        fonts: vec![0xfeed_beef_0000_0001],
        images: Vec::new(),
    };
    let dump = Outcome::CanvasDump {
        reading: reading.clone(),
    };
    assert_eq!(
        dump.to_json(),
        r##"{"canvas":true,"text":"clip t=id clip=M0,0Z node=-\n  fill fill=non-zero brush=#010203ff t=id shape=M1,1 node=1\npop node=-\n","fonts":["feedbeef00000001"],"images":[]}"##
    );
    // A part opens inside the clip: its first line is indented from the depth it is given.
    let part = Outcome::CanvasPart {
        reading: CanvasReading {
            commands: commands[1..].to_vec(),
            nodes: vec![Some(1), None],
            fonts: Vec::new(),
            ..reading
        },
        depth: 1,
        part: 2,
        parts: 2,
    };
    assert_eq!(
        part.to_json(),
        r##"{"canvas":true,"text":"  fill fill=non-zero brush=#010203ff t=id shape=M1,1 node=1\npop node=-\n","fonts":[],"images":[],"part":2,"parts":2}"##
    );
    let none = Outcome::CanvasDump {
        reading: CanvasReading::none(),
    };
    assert_eq!(
        none.to_json(),
        r#"{"canvas":false,"text":"","fonts":[],"images":[]}"#
    );
}
