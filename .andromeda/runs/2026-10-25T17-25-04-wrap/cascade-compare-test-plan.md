# Cascade compare — test-plan · wrap · 2026-10-25-cold-agent-bar
E items 54 · held 17 · lost at classify 3 · lost at strong 1 · beside a placed fact 1 · cascade only 4 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### ci-integration#4.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 30
    section: "## 9. CI Integration"
    change: "After «…`host_log`'s third test, green under the `-p seven_guis` and the workspace build (…)», add: \"and, since 2026-10-25-cold-agent-bar, `host_log`'s fourth test, `a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`, reads a paged part through the host's own sink — `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, then `snapshot --part 2` — 0 id and 0 name needles under both builds\""
    sidecar: "test-plan ci-integration §9 Engine features by runner: host_log's fourth test reads a Cells paged part through the host's sink at trace, 0 needles under both builds — 2026-10-25-cold-agent-bar F13"
    rationale: "Report Counts F13 (`host-sink-part`): «host_log 3 → 4 tests; the paged part read through the host's own sink is measured … 0 id and 0 name needles under both builds»; Symbols/APIs new test seven_guis `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`; Outcome (obs) «`host_log`'s Cells part read 0 id and name needles under both builds»"
    basis: .andromeda/test-plan-detail/ci-integration.md:30
    fact: F13

### mocking-stubbing-discipline#1.5 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 14
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'In the `HeldNet` row, after «answering a head stylesheet and then its `@import` on `.test` URLs with no real fetch» add: «; since 2026-10-25-cold-agent-bar it is one of three hand-written network stand-ins, beside the self-answering `LateNet` and the stand''s own `LoadNet` (rows below), and still the one that holds»'
    sidecar: 'test-plan §8 detail: the `HeldNet` row says it is one of three network stand-ins since `LoadNet` — 2026-10-25-cold-agent-bar.'
    rationale: 'F17 marks mocking-stubbing-discipline.md:14 stale under `stand-offline`: the row reads `HeldNet` as escher''s one held network stand-in with no word of the stand''s new `LoadNet` beside it.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:187
    dependent-of: D-new-detail

### test-data-fixtures#3.18 · lost at classify · beside a placed fact F6 (near 23, test-data-fixtures#3.2) · .andromeda/test-plan-detail/test-data-fixtures.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 39
    section: "## 7. Test Data & Fixtures"
    change: "add a data-seeds row after the `Fetch body` row — | Network answer (stand load page) | `seven_guis::stand::boot_load` replaces the stand options' `net_provider` with the private stand-in `LoadNet`, which answers every fetch of the fixed URL `http://stand.test/load.css` with the fixed bytes `LOADED_CSS` (`.load-body .load-done { display: block; }`) from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), no `is_noop` override so the harness counts its load; every other stand boot keeps `net_provider: None`; read by `stand_load_page` and the stand unit test `the_load_page_click_leaves_a_load_in_flight` | (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175; tests/blitz-tests/tests/stand_load_page.rs:34-68) |"
    sidecar: "Test Data & Fixtures: the stand load page's fixed network answer (`LoadNet`, `http://stand.test/load.css`, `LOADED_CSS` after 200 ms) recorded as a seed row (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs name `boot_load`, `LOAD_URL`, `LOADED_CSS`, `LOAD_ANSWER_DELAY` and `LoadNet` (stand.rs:141-152, 167-175) and F17 names it a third hand-written network stand-in; the seed table holds no row for this fixed URL and body, the stand's only non-offline data"
    basis: examples/seven_guis/src/stand.rs:167-175
    fact: F17

### test-data-fixtures#3.18 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 39
    section: "## 7. Test Data & Fixtures"
    change: "add a data-seeds row after the `Shared fixture page (rsx!)` row's neighbours, beside it in the table — | Served screen over the budget (Cells) | `seven_guis::stand::boot_cells` mounts the Cells task in TaskShell (no `LeanTask`), a stand screen over `SNAPSHOT_TEXT_BUDGET` — 47,949 bytes, 740 lines, 5 parts — walked in parts through the driver by `stand_cells_served` in both layout modes | (examples/seven_guis/src/stand.rs:131-135; tests/blitz-tests/tests/stand_cells_served.rs:91-133) |"
    sidecar: "Test Data & Fixtures: Cells, booted by `stand::boot_cells`, recorded as the served over-budget screen (47,949 B, 5 parts) beside `many_items` (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs add `boot_cells` (stand.rs:131-135) and its Spec claims disproved state Cells reads 47,949 bytes, 740 lines, 5 parts (F16, `stand_cells_served::cells_is_walked_in_parts_through_the_driver`); the table's only over-budget screen row is the fixture `many_items`"
    basis: tests/blitz-tests/tests/stand_cells_served.rs:91-133
    fact: F16

### test-data-fixtures#3.18 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 39
    section: "## 7. Test Data & Fixtures"
    change: "add a data-seeds row — | Log needles (Cells part read) | seven_guis' `common::cells_needles(part)` derives the needles from the in-process Cells screen — part `part`'s author keys and the middle column-A cell it holds — and `host_log`'s Cells part read commits a sentinel into a part-2 cell first (Cells' part-2 names at boot are empty cells and row numbers, no attributable needle) before `snapshot --part 2` at `RUST_LOG=trace` | (examples/seven_guis/tests/common/mod.rs:425-454; examples/seven_guis/tests/host_log.rs:134-277) |"
    sidecar: "Test Data & Fixtures: the Cells part read's needles (`cells_needles`, a committed part-2 sentinel) recorded as a seed row (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs add `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454) and its Deviations state `host_log`'s Cells read commits a sentinel first (F13, host_log.rs:134-277); the seed table states no needle data for a log check"
    basis: examples/seven_guis/tests/common/mod.rs:425-454
    fact: F13

### test-strategy#6.1 · lost at strong · beside a placed fact F10 (near 8, test-strategy#1.1) · .andromeda/test-plan-detail/test-strategy.md:40
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 40
    section: "## 2. Test Strategy"
    change: "New procedure item after Font-dependent tests: `- **A click right after a scroll (since 2026-10-25-cold-agent-bar):** right after a driver `scroll`, the engine's overlay scrollbar is shown and takes a main-button pointerdown in its band as a thumb drag, dispatching nothing; on the virtual clock it stays shown until an `advance` passes its fade, and a pointer left resting on the thumb keeps it shown — so a check that clicks a cell inside a scrolled box after scrolling advances past the fade first (`stand_cells_served` takes `advance 5000` after the scroll) and holds any click that would leave the pointer on the thumb in a test of its own (`stand_cells_served::a_click_right_after_the_scroll_lands_on_the_shown_scrollbar`) (tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:91-133)`"
    sidecar: "test-strategy: the procedure for a click after a scroll — advance past the overlay scrollbar's fade, measured at 2026-10-25-cold-agent-bar"
    rationale: "F21 names it a new fact with no master site; the report's Decisions & corrections measure the thumb-drag hazard (events/pointer.rs, hit_with_scrollbar with scrollbar_opacity > 0), Deviations record the walk's `advance 5000` and the early click moved to its own test, and Reverted facts record why (the pointer left on the thumb defeats the later advance) — a procedure a stand check must follow, this category's procedure · strategy kind"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:355
    fact: F21

### unit-test-strategy#7.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 58
    section: '## 4. Unit Test Strategy'
    change: 'A new `<!-- detail: procedure · unit -->` block after line 58 holds one entity — `- **seven_guis (stand):** since 2026-10-25-cold-agent-bar three library unit tests in `stand.rs` pin the boots beside the lean four — `stand::options` keeps `net_provider: None` for every caller (`the_stand_options_stay_offline`), `boot_cells` mounts Cells in `TaskShell` (`cells_boots_in_the_shell`), and a click on the load page leaves a load in flight, its `LoadNet` stand-in carrying no `is_noop` override so the harness counts the load, answered after `LOAD_ANSWER_DELAY` (200 ms) (`the_load_page_click_leaves_a_load_in_flight`); `cargo test -p seven_guis --locked --lib stand::` green, 5 passed (examples/seven_guis/src/stand.rs:283-291; examples/seven_guis/src/stand.rs:293-306; examples/seven_guis/src/stand.rs:308-329)`'
    sidecar: 'unit-test-strategy: seven_guis stand unit tests of the offline options, the Cells boot and the load page''s load in flight (2026-10-25-cold-agent-bar)'
    rationale: 'Report Symbols / APIs: `boot_cells`, `boot_load`, `LoadNet` (no `is_noop` override), `stand::options` keeping `net_provider: None` (unit test `the_stand_options_stay_offline`); New tests list the three stand unit tests; New text, by line stand.rs 283-291 · 293-306 · 308-329; Coverage of new surfaces: tests unit + integ; Gates: `--lib stand::` green (5 passed). The category holds no seven_guis unit-test entity.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:27-38
    fact: F17

## Held

- ci-integration#1.1
- ci-integration#6.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- e2e-test-strategy#2.2
- integration-test-strategy#2.1
- integration-test-strategy#2.2
- integration-test-strategy#2.3
- mocking-stubbing-discipline#1.6
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- test-scope-summary#1.1
- test-scope-summary#11.1
- test-scope-summary#2.1
- test-strategy#8.1
- unit-test-strategy#3.1
- unit-test-strategy#5.1

## Cascade only

- mocking-stubbing-discipline#1.17
- test-data-fixtures#3.2
- test-scope-summary#7.2
- test-strategy#1.1
