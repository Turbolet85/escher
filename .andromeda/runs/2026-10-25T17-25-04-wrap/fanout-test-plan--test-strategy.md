proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "In the Directory pattern's target count, read `147 .rs files, 146 test targets and the unbuilt all.rs (145 and 144 before 2026-10-25-cold-agent-bar added `stand_cells_served.rs` and `stand_load_page.rs`, each its own auto-discovered target with no manifest line, `tests/blitz-tests/Cargo.toml` untouched; 144 and 143 before upstream's `hit_test_dropped_nodes.rs`, …` in place of `145 .rs files, 144 test targets and the unbuilt all.rs (144 and 143 before upstream's `hit_test_dropped_nodes.rs`, …`, every older step of the history kept"
    sidecar: "test-strategy Directory pattern: blitz-tests 145 → 147 .rs files and 144 → 146 targets, the two new auto-discovered stand files of 2026-10-25-cold-agent-bar"
    rationale: "The report's Changes list two new target files directly under tests/blitz-tests/tests/ (`stand_cells_served.rs` 209 lines, `stand_load_page.rs` 84 lines) and no manifest change (Files: 26 changed, no tests/blitz-tests/Cargo.toml; Dependencies: the Cargo.toml diff probe reads 0); F10 counts them as the two new `stand_*` files of `run stand` (48 → 50 files); the file's count of .rs files and targets is not among any pattern's swept sites"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:13
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 52
    section: "## 2. Test Strategy"
    change: "In the Directory pattern, read `tests/session_common/mod.rs`, read through `mod session_common;` by 30 checks (`stand_cells_served.rs` the latest, since 2026-10-25-cold-agent-bar; 29 with `stand_snapshot_parts.rs`; 27 before `stand_act_select.rs`, …` in place of `by 29 checks (`stand_snapshot_parts.rs` the latest; 27 before `stand_act_select.rs`, …`, the rest of the history kept — and 31 with `stand_load_page.rs` named beside it if the apply-time read finds that file declaring `mod session_common;` too"
    sidecar: "test-strategy Directory pattern: session_common readers 29 → 30 with stand_cells_served (2026-10-25-cold-agent-bar)"
    rationale: "The report's New text, by line, lists stand_cells_served.rs:19-21 «use session_common::{», a new reader of the shared session module, and its tests hold sessions through `Session::start` (33-37); the report states no `mod common;` declaration for either new file and says neither declares `mod fixtures;` (F10), so those two counts stand"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:549
    fact: F10
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
note: "Citations here whose files this chunk grew are left to the P2 citation sweep with no number written (examples/seven_guis/src/stand.rs:59-61 at :40, stand.rs added 1-9 · 12 · 17; examples/seven_guis/tests/common/mod.rs:35-44 · :46-55 · :57-128 · :203-211 · :332-337 at :14-16, added 6-7; tests/blitz-tests/tests/session_common/mod.rs:323-408 at :15, added 305-318); the body's own reader count at .andromeda/test-plan.md:149 (29 readers) moves with proposal 2."
