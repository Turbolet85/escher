proposals:
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
note: "no detector fires on this file's existing text (no F-site or Retired item lands in it); F17's stale site test-plan.md:336 (body §7 Builders, `stand::options` `net_provider: None`) is the body's to amend — `boot_load` replaces it with `LoadNet`; D-tests-coverage, D-tests-framework and D-tests-obs-harness read clean for this category."
