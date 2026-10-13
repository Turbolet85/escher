proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts → Local baseline (`cargo test --workspace`)"
    change: "Append after the 2026-10-13-stand-contrast-harness re-count: re-counted at 2026-10-13-range-input-interaction: 181 result lines, 915 passed · 0 failed · 12 ignored (+15 passed: one new result line, blitz-tests' `range_input_interaction` 6; blitz-dom +8 in its lib line, `node/range.rs` 7 and `form.rs` 1; dioxus-native-dom +1, a snapshot test, the renamed one counted once), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (escher-0.1.0/chunks/2026-10-13-range-input-interaction/evidence/range-readings.md, §Totals the gates read)."
    sidecar: "test-plan §9 local baseline: workspace re-counted at 2026-10-13-range-input-interaction, 181 · 915 · 0 · 12 (+15: range_input_interaction 6, range.rs 7, form.rs 1, one snapshot test)."
    rationale: "Report Changes → Counts / qualifiers moved: workspace tests now 181 result lines · 915 passed · 0 failed · 12 ignored, test-plan.md:330 reading 180 · 900 · 0 · 12; the 15 added are 6 in range_input_interaction, 7 in range.rs, 1 form test, 1 snapshot test; Outcome gate ci-leg.sh fast test log reads the same."
    basis: "test-plan.md:330"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "Restate the blitz-tests file count as 120 `.rs` files, 119 test targets and the unbuilt `all.rs`, and prepend to its history: 120 and 119 since 2026-10-13-range-input-interaction added `range_input_interaction.rs`, its own auto-discovered target with no manifest line (escher-0.1.0/chunks/2026-10-13-range-input-interaction/report.md); 119 and 118 since 2026-10-13-stand-contrast-harness …"
    sidecar: "test-plan §2 directory pattern: blitz-tests 119 → 120 .rs files, 118 → 119 targets, range_input_interaction.rs added."
    rationale: "Report Changes → Files (new `tests/blitz-tests/tests/range_input_interaction.rs`; tests/blitz-tests/Cargo.toml not among the 12 changed files) and Counts / qualifiers moved: blitz-tests now 120 `.rs` files and 119 targets, it was 119 and 118."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: "Restate the head count as 68 unit tests in nine files (67 until 2026-10-13-range-input-interaction, which added one snapshot test, `a_stepped_uncontrolled_range_input_reads_its_moved_value`, packages/dioxus-native-dom/src/snapshot.rs:607-651 — as measured at escher-0.1.0/chunks/2026-10-13-range-input-interaction/report.md; 61 in eight until 2026-10-13-upstream-sync-polish-and-ship …)."
    sidecar: "test-plan §1 dioxus-native-dom: 67 → 68 unit tests, one snapshot test added by 2026-10-13-range-input-interaction."
    rationale: "Report Changes → Counts / qualifiers moved: of the 15 added workspace tests, 1 is an added snapshot test and the renamed one counts once; Coverage → snapshot value of a range input names unit test `a_stepped_uncontrolled_range_input_reads_its_moved_value`; New text lists snapshot.rs 607-651."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom (the snapshot-builder clause)"
    change: "Restate seventeen on the snapshot builder as eighteen, and replace a range input reading its `value` attribute with a range input reading its current value (`a_range_input_reads_its_current_value`, renamed at 2026-10-13-range-input-interaction from the `value`-attribute reading) and, since that chunk, an uncontrolled range input stepped through the engine reading its moved value while its `value` attribute keeps the boot value (packages/dioxus-native-dom/src/snapshot.rs:607-651)."
    sidecar: "test-plan §1 dioxus-native-dom: the snapshot's range reading is its current value, not its value attribute; snapshot-builder tests 17 → 18."
    rationale: "Report Changes → Spec claims disproved by measurement: test-plan.md:24's a range input reading its `value` attribute is false — the test is renamed `a_range_input_reads_its_current_value` and a new test proves an uncontrolled range input reads its moved value while its attribute keeps the boot value (snapshot.rs:607-651); Symbols / APIs → dioxus-native-dom: NodeState.value for a range input reads `range_input_value()`."
    basis: "test-plan.md:24"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (`stand_act_range`)"
    change: "Replace the `stand_act_range` 1 parenthetical with: since 2026-10-13-range-input-interaction the driver moves the Timer's slider by click and by key, sequences A and B — each returned diff equal to the check's own `before.diff(&after)` and equal across the two layout modes, after a value change the slider's `differs` holding `value` and the display's `name`, after the first step `differs == [focused]` exactly, and after two Tabs an arrow step leaving the slider focused with the tree's focus, role `Slider` and its name unchanged; until that chunk it pinned the boot value read after each of eleven driver calls, a measured no-op."
    sidecar: "test-plan §1 blitz-tests: stand_act_range rewritten — the driver moves the slider by click and key; the pinned no-op reading retired."
    rationale: "Report Changes → Files (`stand_act_range.rs` rewritten), Spec claims disproved (the no-op is gone; `stand_act_range` asserts the driver moves the slider by click and key, sequences A and B), Counts (`stand_act_range` keeps one test), Deviation 7 (the `differs` assertions) and Outcome (tests and a11y criteria on sequences A and B)."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (`stand_act_ids`)"
    change: "Restate the boot-change split as not empty for the 9 a click changes at boot — the Timer's slider `timer-duration` the ninth since 2026-10-13-range-input-interaction — empty for the other 3, and the Timer clause as: the Timer's Reset changes nothing at boot, so it is read after an `advance`."
    sidecar: "test-plan §1 blitz-tests: stand_act_ids' controls a click changes at boot 8 → 9 (timer-duration), the unchanged 4 → 3."
    rationale: "Report Changes → Counts / qualifiers moved: `stand_act_ids`' pinned count of stand controls that change the screen at boot moves from 8 to 9, the Timer slider joining; Deviation 6 (`stand_act_ids.rs:230-231`); Outcome (tests): `stand_act_ids` with the Timer row `timer-duration`. The 12 accepted controls are unchanged (no driver change, preservation probe exit 0), so the other 4 become 3, and the Timer no longer has none at boot."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests (engine checks on parsed pages)"
    change: "Add beside the other engine checks: since 2026-10-13-range-input-interaction `range_input_interaction` 6, an engine check on parsed pages with no `stand_` prefix, each test in both layout modes — a focused range input stepped by ArrowRight/ArrowUp, ArrowLeft/ArrowDown, Home and End (a 7-row table, one `input` event per change and none on a Home that changes nothing), a click setting the value by position and focusing the input (a 5-row table), seven degenerate attribute rows keeping the value finite, bounded and on the grid, a decimal step reading its decimals, a disabled range input taking neither keys nor a click, and attribute writes re-seeding (`7` → `8`) and re-sanitizing (`max=5` → `4`) the value — run by the workspace `test` leg and per package, not by `run stand` (tests/blitz-tests/tests/range_input_interaction.rs:113-298) — as measured at escher-0.1.0/chunks/2026-10-13-range-input-interaction/report.md."
    sidecar: "test-plan §1 blitz-tests: range_input_interaction (6, both modes) joins the engine checks — range keys, click, degenerate attributes, decimals, disabled, attribute writes."
    rationale: "Report Changes → Files (new `range_input_interaction.rs`, 298 lines), Coverage of new surfaces (integ `range_input_interaction`, both modes), Outcome (keys table 7 rows, clicks table 5 rows, one `input` per change, none on a no-change Home; attribute-write rows `7` → `8`, `max=5` → `4`; the 7 degenerate integration rows; the disabled rows) and gate `cargo test -p blitz-tests --locked --test range_input_interaction` 6 passed; `run stand` unchanged at 37 files."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (node)"
    change: "Add: since 2026-10-13-range-input-interaction `node/range.rs` holds 7 inline tests of the range value rules over stated tables — the HTML floating-point parser (20 rows), the four attributes read as bounds, step and base (9), a `value` attribute seeding a finite, bounded, on-grid value (12), moves stepping, jumping and placing on the grid (9), degenerate attributes keeping the value finite, bounded and on the grid (8), a decimal step writing its decimals, and a re-seed and a re-sanitize following the attributes (packages/blitz-dom/src/node/range.rs:296-522)."
    sidecar: "test-plan §1 blitz-dom (node): node/range.rs's 7 inline value-rule tests added."
    rationale: "Report Changes → Files (new `packages/blitz-dom/src/node/range.rs`), Counts (7 in range.rs's unit tables), Coverage (tests unit, `range.rs` tables) and New text (range.rs 296-522, `mod tests`, the PARSED 20 · RULES 9 · SEEDED 12 · MOVES 9 · DEGENERATE 8 tables and the seven test functions)."
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → blitz-dom (node)"
    change: "Add: seven range.rs tests pin the range value rules — the HTML float grammar, attribute defaults and bounds, the seeded value finite, bounded and on the grid, the six moves on the grid, degenerate attributes, decimal formatting, and re-seed and re-sanitize on attribute writes (packages/blitz-dom/src/node/range.rs:296-522)."
    sidecar: "test-plan §4 blitz-dom (node): range.rs's value-rule unit tests named."
    rationale: "Second statement of what blitz-dom's node in-file tests cover (§1 states it too); the report's Counts and New text add range.rs's 7 unit tests at 296-522."
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → blitz-dom (document, mutator, net)"
    change: "Restate as 35 `#[test]` functions: 18 in document.rs, 12 in mutator.rs, 4 in net.rs and, since 2026-10-13-range-input-interaction, 1 in form.rs's first test module, `a_range_input_submits_its_current_value` (packages/blitz-dom/src/form.rs:466-526), the scope named document, mutator, net, form."
    sidecar: "test-plan §1 blitz-dom: form.rs's first unit test (a range input submits its current value), 34 → 35."
    rationale: "Report Changes → Counts (1 form test among the 15 added), Coverage (form entry of a range input → unit `form::tests::a_range_input_submits_its_current_value`) and New text (form.rs 466-526, `mod tests` at 468)."
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → blitz-dom (net, mutator)"
    change: "Add: `construct_entry_list` is unit-tested to submit a range input's formatted current value (packages/blitz-dom/src/form.rs:466-526)."
    sidecar: "test-plan §4 blitz-dom: form.rs's range submission unit test named."
    rationale: "Second statement of blitz-dom's non-node unit coverage; the report's Coverage names `form::tests::a_range_input_submits_its_current_value` and Symbols → Form submission (form.rs:318-319)."
    dependent-of: D-tests-coverage
note: "D-tests-framework and D-tests-obs-harness hold — the report's runners are cargo test and blitz_test_harness::Harness as they stand, and Harness / gate surface reads none. test-plan.md:26's the range input reading the app's duration and session-lifecycle's stand_act_range 1 stay true as the report reads them. Not measurable from the report, so no proposal: whether clear_attribute's re-seed/re-sanitize on removal and construction's clearing of the data when type leaves range are read by any check (the Outcome states set_attribute rows only); and whether the citation ranges on test-plan.md:24 (snapshot.rs:232-786) and :15 (node/element.rs:1041-1175) shifted with the added lines."
