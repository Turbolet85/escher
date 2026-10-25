proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the **Legs:** a11y item (piece @0), keep the founder's option text «the tier-0 witnesses and the five tree-reading checks, 14 files and 84 tests» as read at that ruling and add after it: \"— 85 tests since 2026-10-25-cold-agent-bar, `stand_keyboard` 14 → 15 with `the_flight_type_select_moves_its_choice_by_its_arrows`, the files unchanged at fourteen\""
    sidecar: "test-plan ci-integration §9 Legs: the a11y leg's test count 84 → 85 (stand_keyboard's select-arrows test), fourteen files unchanged — 2026-10-25-cold-agent-bar F8"
    rationale: "Report Counts F8 (`a11y-84`) marks test-plan ci-integration.md:8 stale — the a11y leg reads 14 files, 85 passed (Outcome: `ci-leg.sh a11y` — green (14 files, 85 passed)); Symbols/APIs names the new test `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`, F7 `stand_keyboard` tests 14 → 15"
    basis: .andromeda/test-plan-detail/ci-integration.md:8
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "In the **Legs:** stand-repeat item (piece @1900, ~@c2220), \"48 files\" becomes \"50 files\", and the joiners' list gains \"`stand_cells_served` and `stand_load_page` since 2026-10-25-cold-agent-bar\" after `stand_act_select` and `stand_snapshot_parts`, each taken by its `stand_` prefix with no leg change"
    sidecar: "test-plan ci-integration §9 Legs: stand-repeat's `stand_*` selection 48 → 50 files (stand_cells_served, stand_load_page by prefix) — 2026-10-25-cold-agent-bar F10"
    rationale: "Report Counts F10 (`run-stand`) marks test-plan ci-integration.md:8 @c2220 stale («48 files»); Harness / gate surface: «`stand-repeat` takes the two new `stand_*` files by prefix. No CI step changes»; Files: new `tests/blitz-tests/tests/stand_cells_served.rs`, `stand_load_page.rs`; Outcome `stand-repeat: 3 of 3 rounds green`, `run stand` 50 files"
    basis: .andromeda/test-plan-detail/ci-integration.md:8
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 63
    section: "## 9. CI Integration"
    change: "Append to the `cargo test --workspace` re-count chain, after the `stand-repeat` ten-targets record: \"re-counted with the cold-agent bar's stand: 208 result lines, 1117 passed · 0 failed · 12 ignored — +2 result lines (`stand_cells_served`, `stand_load_page`) and +12 passed (`stand_cells_served` 4, `stand_load_page` 2, `stand_keyboard` 1, `stand_snapshot` 1, seven_guis `host_log` 1 and the stand's 3 unit tests `the_stand_options_stay_offline` · `cells_boots_in_the_shell` · `the_load_page_click_leaves_a_load_in_flight`; three tests renamed, none removed), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe)\""
    sidecar: "test-plan ci-integration §9 Local baseline: workspace tally re-counted 206 · 1105 → 208 · 1117 · 0 · 12 — 2026-10-25-cold-agent-bar F11"
    rationale: "Report Counts F11 (`tally`): «the workspace tally 206 lines · 1105 passed → 208 · 1117, 0 failed, 12 ignored»; Outcome gate `ci-leg.sh fast` tally probe `lines 208 passed 1117 failed 0 ignored 12`; the +12 composition is the report's Symbols/APIs new-test list (the two new files' six tests, `stand_keyboard`, `stand_snapshot`, `host_log` and the three stand unit tests); line 63 is the chain's last record (dated, F11)"
    basis: .andromeda/test-plan-detail/ci-integration.md:63
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 65
    section: "## 9. CI Integration"
    change: "After the a11y reading of 2026-10-24-a11y-re-read in the `ci-leg.sh a11y` clause, add: \"; re-read at 2026-10-25-cold-agent-bar: fourteen files, exit 0, 14 result lines · 85 passed · 0 failed · 0 ignored — `stand_keyboard` 15, every other file unmoved (escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)\""
    sidecar: "test-plan ci-integration §9 Local baseline: the a11y leg re-read 14 · 85 · 0 · 0 (stand_keyboard 14 → 15) — 2026-10-25-cold-agent-bar F8"
    rationale: "Report Counts F8 marks ci-integration.md:65 dated (a reading at 2026-10-24-a11y-re-read); the chunk's own reading is new detail — Outcome gate `ci-leg.sh a11y` green (14 files, 85 passed), acceptance (a11y) «a11y 14 files · 85 · 0»; F7 `stand_keyboard` tests 14 → 15"
    basis: .andromeda/test-plan-detail/ci-integration.md:65
    fact: F8
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
note: line 30 cites examples/seven_guis/tests/host_log.rs:42-132 and the report's New text lists host_log.rs additions at 15-17 · 134-278, but the report names no citation row for this file's line 30, so no renumbering is proposed; the CI run CI#40214713710 (green 17/17, 990 s) gives no per-OS tallies in the report, so the Per-OS tallies chain (line 42) gets no record; line 8 already exceeds the 2 000-char entity bound (two pieces) and these edits lengthen it.
