# Cascade compare — test-plan · wrap · 2026-10-24-stand-requirement-sweep
E items 28 · held 11 · lost at classify 1 · lost at strong 1 · cascade only 1 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### quality-gates-coverage-targets#2.1 · lost at strong · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "After line 16, a continuation line inside the same gate-coverage-floor block: ' At 2026-10-24-stand-requirement-sweep, a chunk of test files and one CI leg only, the three floors and `coverage_floor.py` are unchanged and the `coverage` leg reads escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, ending `coverage floor: 3 of 3 crates at or above their floors`, equal to CI#40105170961 (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, gate entry 16)'"
    sidecar: "test-plan quality-gates: the per-crate coverage reading at 2026-10-24-stand-requirement-sweep (97.99 · 85.57 · 94.15 %, floors unchanged) added after the load-wait reading"
    rationale: "Report Counts closing paragraph: 'The coverage floors are unmoved: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, equal to CI#40105170961'; Outcome: `coverage` ends `3 of 3`, the floors unchanged. Line 16's latest stated reading is escher-driver 4695/4794 = 97.93 % since 2026-10-20-load-wait, so the measured later reading is new detail; the floors themselves do not move."
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:192

### test-data-fixtures#2.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 16
    section: "## 7. Test Data & Fixtures"
    change: "after line 16, a new `<!-- detail: values · data-fixtures -->` block holding one record — `- **tests/blitz-tests/tests/fixtures (many_items):** since 2026-10-24-stand-requirement-sweep the shared fixture module also states `pub const ITEMS: usize = 400` and the page `pub fn many_items() -> Element` — a `#items` list of 400 buttons with the ids `item-NNN`, a screen that pages — moved unchanged from `stand_snapshot_parts.rs`, read by `stand_snapshot_parts` and by `stand_act_spans`, which holds it through `fixtures::hold(\"paged\", fixtures::many_items, …)` and reads `snapshot_part(2)` in its sink capture, the `#items > button` texts its name needles and the 400 `item-NNN` ids its id needles (tests/blitz-tests/tests/fixtures/mod.rs:64-86; tests/blitz-tests/tests/stand_act_spans.rs:243; tests/blitz-tests/tests/stand_act_spans.rs:685-693)`"
    sidecar: "test-data-fixtures: the shared fixture page `many_items` (`ITEMS` 400), moved into `tests/fixtures/mod.rs` and read by `stand_snapshot_parts` and `stand_act_spans`, recorded (2026-10-24-stand-requirement-sweep)"
    rationale: "the report's Changes name `fixtures/mod.rs` gaining `ITEMS` and `many_items` (64-86), moved unchanged from `stand_snapshot_parts.rs`, and `stand_act_spans` reading it as a paged part with its buttons' texts and `item-NNN` ids as needles (Counts fact 10, `fixture-pages`); the category file holds no record of the shared fixture module's pages, so the new shared page enters as one record in the file's data-fixtures shape"
    basis: escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md:57

## Held

- ci-integration#1.1
- ci-integration#4.1
- ci-integration#5.1
- ci-integration#6.1
- integration-test-strategy#2.3
- quality-gates-coverage-targets#3.1
- test-scope-summary#11.1
- test-scope-summary#2.1
- test-scope-summary#2.3
- test-scope-summary#5.1
- unit-test-strategy#3.1

## Cascade only

- test-data-fixtures#3.1
