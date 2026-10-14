proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "Add, since 2026-10-13-stand-keyboard-harness, `stand_keyboard` 11, a stand check picked up by `run stand`, each test in both layout modes: Tab and Shift+Tab walk each lean task's stated sequence (`common::tab_order`) and wrap, the engine's focus, the snapshot's `focused` and the tree's focus agreeing at every step (the engine's root fallback dropped); Enter and Space on a focused button read as its click (20 rows stated and counted, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing) and the keys that do not activate type or change nothing; on in-file fixtures the controls take the keys HTML gives them (Enter on a `button`, `summary`, submit/reset/button `input` and `a` with `href`; Space on those but the link, plus checkbox and radio), a button disabled after it took focus takes no key, and a button that is itself scrolled takes Enter as its own click; a click focuses each stand button and Tab continues from it; a `disabled` attribute of any value — bare, false, true — is skipped by Tab, the `<div disabled tabindex=0>` known limit pinned unfocusable; on CRUD the list (`crud-list`) is the third Tab stop and ArrowDown/ArrowUp/Home/End move `selected` and fill the fields as a row click does, the tree's focus staying on the list; `apple_keybinding(deleteBackward:)` deletes on every OS; and the engine honours no `inert` (Tab reaches buttons under `inert: true` and `inert: false`)"
    sidecar: "§1 tests/blitz-tests: + `stand_keyboard` 11 — Tab walk, Enter/Space activation, click focus, disabled-by-presence, CRUD list keys, apple binding, no `inert` (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Files list a new check `tests/blitz-tests/tests/stand_keyboard.rs` (618 lines) covering the new engine paths F1 (Enter/Space activation), F2 (click focuses a button), F3 (disabled by presence) and the stand's F5 (CRUD list keys); its Coverage of new surfaces names `stand_keyboard` as the integ test of each; §1 lists no such check, so the new paths' tests are unrecorded at the tier the plan lists them"
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:118-146; tests/blitz-tests/tests/stand_keyboard.rs:148-204; tests/blitz-tests/tests/stand_keyboard.rs:290-369; tests/blitz-tests/tests/stand_keyboard.rs:430-457; tests/blitz-tests/tests/stand_keyboard.rs:469-500; tests/blitz-tests/tests/stand_keyboard.rs:502-562; tests/blitz-tests/tests/stand_keyboard.rs:564-581; tests/blitz-tests/tests/stand_keyboard.rs:597-618"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "`stand_act_keys` 3 (2 until 2026-10-13-stand-keyboard-harness): add that, through the driver, `press enter` and `press space` on a Tab-focused `counter-increment` each count as its click, the returned diff naming `counter-value` with `differs: [name]`"
    sidecar: "§1 tests/blitz-tests: `stand_act_keys` 2 → 3 — Enter and Space on a focused button count as its click through the driver (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts name `stand_act_keys` +1 and its New text lists `enter_and_space_on_a_focused_button_count_as_its_click_does`; test-plan.md:26 states `stand_act_keys` 2 with no activation key"
    basis: "tests/blitz-tests/tests/stand_act_keys.rs:131-165"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "`stand_act_obstructed`: replace 'after the accepted click no element reads `focused` in the snapshot and the accessibility tree's focus carries no author id' with: after the accepted click the clicked button (`fx-before`) reads `focused` and is the accessibility tree's focus — restated at 2026-10-13-stand-keyboard-harness, a click focusing a button in every document"
    sidecar: "§1 tests/blitz-tests: `stand_act_obstructed` restated — an accepted click leaves the clicked button focused, the tree's focus (2026-10-13-stand-keyboard-harness, F2)"
    rationale: "The report's Spec claims disproved: 'A pointer click on a plain `button` leaves no element focused' measured false after F2 — restated `stand_act_obstructed` (after an accepted click `fx-before` reads focused and is the tree's focus); test-plan.md:26 still states the retired reading"
    basis: "tests/blitz-tests/tests/stand_act_obstructed.rs:421-422"
  - detector: D-tests-coverage
    severity: warning
    section: "§5 Integration Test Strategy → Boundaries covered → Session ↔ held instance"
    change: "Replace 'and after the accepted click no element reads `focused` (`stand_act_obstructed`)' with: and after the accepted click the clicked button reads `focused` and is the tree's focus (`stand_act_obstructed`, restated at 2026-10-13-stand-keyboard-harness)"
    sidecar: "§5 Session ↔ held instance: `stand_act_obstructed`'s focus reading after an accepted click restated (2026-10-13-stand-keyboard-harness)"
    rationale: "Same retired claim as the §1 `stand_act_obstructed` row, restated in §5; the report measures it false after F2 (Spec claims disproved, click-focus)"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "`stand_act_ids`: replace the tally 'for each of the 12 controls a click is accepted on … not empty for the 9 a click changes at boot … empty for the other 3' with the measured one: every accepted control row of `controls(task)` — 15 control rows — changes the screen at boot, since a click focuses a button (2026-10-13-stand-keyboard-harness), the tally counting rows over `controls(task)` (18 rows, `back-btn` ×4), not distinct controls"
    sidecar: "§1 tests/blitz-tests: `stand_act_ids` changing-at-boot tally 9 → 15 control rows, every accepted control (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: '`stand_act_ids`: the controls changing the screen at boot go 9 → 15 control rows, every accepted control, since a click focuses a button'; Decisions: the tally counts control ROWS over `controls(task)` (18 rows, `back-btn` ×4); test-plan.md:26 states 12 accepted, 9 changing, 3 empty"
    basis: "tests/blitz-tests/tests/stand_act_ids.rs:243-244"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "`stand_diff`: replace 'the Flight Booker's Book naming eight nodes that only moved, each `differs` = `[bounds]`' with: the Book naming eight nodes — seven that only moved, `differs` = `[bounds]`, and `flight-book` itself, `differs` = `[focused, bounds]`, the click focusing it (since 2026-10-13-stand-keyboard-harness)"
    sidecar: "§1 tests/blitz-tests: `stand_diff`'s Book now 7 bounds-only plus `flight-book` [focused, bounds] (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: '`stand_diff`: Book's 8 changed nodes now read 7 bounds-only plus `flight-book` `[Focused, Bounds]`'; test-plan.md:26 states all eight `[bounds]`"
    basis: "tests/blitz-tests/tests/stand_diff.rs:524-527"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "Dioxus integration: replace 'each of the 27 boolean attributes the check names … the check file unchanged and green, whether a check reads a falsy `inert` not measured here' with: each of the 28 boolean attributes the check names (`BOOLEAN_ATTRS: [&str; 28]`, `inert` its 28th since 2026-10-13-stand-keyboard-harness, read on both attribute paths), the attribute half only — `stand_keyboard` pins that the engine honours no `inert`, truthy or falsy"
    sidecar: "§1 tests/blitz-tests: `dioxus_falsy_boolean_attrs` 27 → 28 names, `inert` read; the engine's lack of `inert` pinned by `stand_keyboard` (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: '`dioxus_falsy_boolean_attrs` names 27 → 28 (`inert` added)' and test-plan.md:26 @c3032 'whether a check reads a falsy `inert` is not measured', stale; Spec claims disproved: the engine implements no `inert` (`stand_keyboard::the_engine_honours_no_inert`)"
    basis: "tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:175-177; tests/blitz-tests/tests/stand_keyboard.rs:597-618"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "Replace 'the ids after a re-render and the Tab order unchanged' with: the ids after a re-render and the Tab order of `common::tab_order` — CRUD's restated at 2026-10-13-stand-keyboard-harness to six stops, `back-btn`, `crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create` (five until then), the other three tasks' unchanged"
    sidecar: "§1 tests/blitz-tests: `stand_accessibility_ids` Tab order restated for CRUD, 5 → 6 stops with `crud-list` third (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: 'CRUD Tab sequence 5 → 6 stops (`crud-list` third). Master sites: … test-plan.md:26 @c3973 (the Tab order unchanged, still true … for the three other tasks, restated for CRUD)'; F5 gives `crud-list` `tabindex: 0`"
    basis: "tests/blitz-tests/tests/common/mod.rs:48-70"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → dioxus-native-dom"
    change: "Replace 'a present `disabled` attribute reading not enabled whatever its value (… written through `DocumentMutator`) while the element stays focusable' with: … reading not enabled and not focusable whatever its value (`a_present_disabled_attribute_reads_not_enabled_and_not_focusable_whatever_its_value`, restated at 2026-10-13-stand-keyboard-harness, focusability read by presence in every document); the count stays 68"
    sidecar: "§1 dioxus-native-dom: the snapshot `disabled` pin now reads not focusable whatever the value (2026-10-13-stand-keyboard-harness, F3)"
    rationale: "The report's Spec claims disproved: 'Focusability parses `disabled` as a boolean; `disabled=false` and a bare `disabled=` stay focusable' measured false after F3, the `snapshot.rs` unit pin restated; Files: `packages/dioxus-native-dom/src/snapshot.rs` (unit test only, 3 lines, no new test); test-plan.md:24 still says the element stays focusable"
    basis: "packages/dioxus-native-dom/src/snapshot.rs:563-564"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → tests/blitz-tests"
    change: "Add that `focusability_updates`, an a11y-leg file, gained since 2026-10-13-stand-keyboard-harness `a_bare_disabled_set_after_creation_takes_the_focusability_away` — a bare `disabled` written through `DocumentMutator` after creation removes focusability (F3's presence reading on the recompute path)"
    sidecar: "§1 tests/blitz-tests: `focusability_updates` +1, a bare `disabled` set after creation removes focusability (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts name `focusability_updates` +1 and Coverage of new surfaces names it among the tests of `disabled by presence → not focusable`; §1 does not name the file's coverage of that path"
    basis: "tests/blitz-tests/tests/focusability_updates.rs:78-96"
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "`tests/common/mod.rs` read through `mod common;` by 19 `stand_*.rs` checks (18 until 2026-10-13-stand-keyboard-harness added `stand_keyboard.rs`); 121 `.rs` files, 120 test targets and the unbuilt `all.rs` (121 and 120 since 2026-10-13-stand-keyboard-harness added `stand_keyboard.rs`, its own auto-discovered target with no manifest line)"
    sidecar: "§2 Directory pattern: `common` readers 18 → 19; blitz-tests 120 files · 119 targets → 121 · 120 (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: blitz-tests 120 `.rs` files, 119 targets → 121 · 120 (test-plan.md:66, stale) and `common` readers 18 → 19, `stand_keyboard` the nineteenth"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (stand checks)"
    change: "Add `tab_order(task)` (each lean task's Tab sequence; CRUD six stops, `crud-list` third) and `focused(harness)` (the ids reading focused in the snapshot beside the tree's focus) to the shared helpers, moved in at 2026-10-13-stand-keyboard-harness from `stand_accessibility_ids` and `stand_snapshot_state`, the module now importing `blitz_dom::Document`; add `stand_keyboard` to the readers, 19 in all; drop `tab_order` from the per-file list"
    sidecar: "§3 stand-check helpers: `tab_order` and `focused` moved into `common`; + `stand_keyboard`, 19 readers (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Symbols: 'Tests' shared module (tests/blitz-tests/tests/common/mod.rs:48-70, :72-92): `pub fn tab_order(task)` moved from `stand_accessibility_ids.rs` … `pub fn focused(harness)` moved from `stand_snapshot_state.rs`'; Counts: test-plan.md:100's reader list and helper list stale (`tab_order` named per-file)"
    basis: "tests/blitz-tests/tests/common/mod.rs:48-70; tests/blitz-tests/tests/common/mod.rs:72-92"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → blitz-test-harness (`Harness`) → Input helpers"
    change: "Replace 'no standing check calls it on Linux or Windows' with: since 2026-10-13-stand-keyboard-harness `stand_keyboard::the_apple_delete_binding_deletes_on_every_os` calls `apple_keybinding(deleteBackward:)` and reads the deletion on every OS — green on the linux leg and on CI#38482525151's `Test (windows)` and `Test (macos)`"
    sidecar: "§3 Input helpers: `apple_keybinding` now exercised on every OS by `stand_keyboard` (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's New text lists `the_apple_delete_binding_deletes_on_every_os` in the new `stand_keyboard.rs`; its Outcome: '`apple_keybinding(deleteBackward:)` deletes on the linux leg, and windows and macos read green on the pushed sha — MET'; §3 still says no standing check calls it on Linux or Windows"
    basis: "tests/blitz-tests/tests/stand_keyboard.rs:564-581"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "Append: re-counted at 2026-10-13-stand-keyboard-harness, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 153 · failed 0 · ignored 7 over 38 files — +12 passed (`stand_keyboard` 11, a new file picked up by its `stand_` prefix with no script change, `stand_act_keys` +1) — and `run all` was not re-read — as measured at escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md"
    sidecar: "§3 Agent-run Proof: `run stand` 141 → 153 passed over 38 files (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: '`run stand`: run.end passed 141 → 153 · failed 0 · ignored 7 … 37 → 38 `stand_*` files. Master site: test-plan.md:116 (ws-counts), stale'; Harness / gate surface: none — `stand_keyboard` joins `run stand` by its prefix"
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline facts"
    change: "Append to the workspace re-count chain: re-counted at 2026-10-13-stand-keyboard-harness: 182 result lines, 928 passed · 0 failed · 12 ignored (+13 passed: one new result line, blitz-tests' `stand_keyboard` 11; `stand_act_keys` +1; `focusability_updates` +1), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/report.md)"
    sidecar: "§9 Pipeline facts: workspace 181 · 915 · 0 · 12 → 182 · 928 · 0 · 12 (2026-10-13-stand-keyboard-harness)"
    rationale: "The report's Counts: 'Workspace tests: 181 result lines · 915 passed · 0 failed · 12 ignored → 182 · 928 · 0 · 12. The +13 passed are `stand_keyboard` 11, `stand_act_keys` +1 and `focusability_updates` +1 … test-plan.md:330 (the 181 · 915 figure), stale'"
note: "D-tests-framework and D-tests-obs-harness hold (cargo's built-in harness only; no harness, status shape or log format change — report Harness / gate surface: none). Not proposed for want of a stated reading: the report names `stand_act_diff` and `stand_act_spans` as restated for F2 without saying what moved, so test-plan.md:26's descriptions of them were left as written; the measured harness fact that `Harness::focused()` falls back to the root element, never `None` (report Decisions & corrections, document.rs:647-650), may merit a line in §3 Inspection helpers; and the autofocus reader that F3 moves (a disabled-attributed autofocus candidate now skipped, report Symbols F3) names no check that reads it."
