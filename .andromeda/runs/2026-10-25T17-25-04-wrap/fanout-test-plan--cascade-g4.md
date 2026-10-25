proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 15
    section: "## 8. Mocking & Stubbing Discipline"
    change: "add after the `LateNet` row: | Network | `LoadNet`, the load page's network stand-in since 2026-10-25-cold-agent-bar, private to `examples/seven_guis/src/stand.rs` — a third hand-written network stand-in beside `HeldNet` and `LateNet`: `stand::boot_load` replaces its options' `net_provider` with `Some(Arc::new(LoadNet))` through `..options`, every other boot keeping `net_provider: None` (`the_stand_options_stay_offline`); each `fetch` answers the fixed sheet `LOADED_CSS` for `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); it does not override `is_noop`, so the harness counts its load — `the_load_page_click_leaves_a_load_in_flight` reads it in flight and `stand_load_page`'s click waits it out and shows the message; no `blitz_net`, print, env read, socket or listener | (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175; examples/seven_guis/src/stand.rs:308-329; tests/blitz-tests/tests/stand_load_page.rs:34-68) |"
    sidecar: "§8 stand-ins: `LoadNet`, the load page's self-answering network stand-in (200 ms, `is_noop` not overridden), added beside `HeldNet` and `LateNet`."
    rationale: "Report Symbols / APIs (`boot_load`, `LoadNet` stand.rs:141-152 · 167-175) and F17 (`stand-offline`: mocking-stubbing-discipline.md:14 · :15 stale, a third hand-written network stand-in); the task's F2."
    basis: examples/seven_guis/src/stand.rs:167-175
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 352
    section: "## 8. Mocking & Stubbing Discipline"
    change: "- **Network:** `HeldNet` is escher's one held network stand-in; it does not override `is_noop`, so the harness counts its loads — beside it two self-answering stand-ins, `stand_load_wait`'s `LateNet` and, since 2026-10-25-cold-agent-bar, the load page's `LoadNet` (`stand::boot_load`, answering after 200 ms, `is_noop` not overridden either), every other stand boot offline"
    sidecar: "§8 Network: the broad line names `LateNet` and the new `LoadNet` beside `HeldNet`; the stand is offline but for the load page."
    rationale: "Report F17 lists test-plan.md:352 stale under `stand-offline`: the stand is offline except for the load page's in-process `LoadNet`, a third hand-written network stand-in beside `HeldNet` and `LateNet`."
    basis: examples/seven_guis/src/stand.rs:141-152
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 26
    section: "## 8. Mocking & Stubbing Discipline"
    change: "in the `TimerTicks` row's citations, re-point the `examples/seven_guis/src/stand.rs` range of `timer_step` to its moved lines — the chunk added stand.rs:1-9 · 12 · 17 above it, so the stated range no longer holds the function; every other citation and the row's text unchanged (the Timer keeps `with_time(timer_step)`)"
    sidecar: "§8 Time: the `TimerTicks` row's `stand.rs` citation re-pointed after the lines added at stand.rs:1-9 · 12 · 17."
    rationale: "Report New text, by line: stand.rs added 1-9 · 12 · 17 · 131-245 · 283-330; the task's Touches: stand.rs citations moved because lines were added at 1-9, 12 and 17; Symbols / APIs: the Timer keeps `with_time(timer_step)`."
    basis: examples/seven_guis/src/stand.rs:1-9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "in the stand-repeat clause read `… one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 50 files, … `stand_act_select` and `stand_snapshot_parts`, and `stand_cells_served` and `stand_load_page` since 2026-10-25-cold-agent-bar` (48 until then)"
    sidecar: "§9 Legs: stand-repeat's prefix glob 48 → 50 files with `stand_cells_served` and `stand_load_page`, no CI step changed."
    rationale: "Report F10 (`run-stand`: test-plan ci-integration.md:8 @c2220 stale, 48 → 50 files) and Harness / gate surface: `stand-repeat` takes the two new `stand_*` files by prefix, no CI step changes; gate `ci-leg.sh stand-repeat` green 3 of 3."
    basis: tests/blitz-tests/tests/stand_cells_served.rs:91-133
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 8
    section: "## 9. CI Integration"
    change: "in the a11y clause read `… the tier-0 witnesses and the five tree-reading checks, 14 files and 85 tests (84 until 2026-10-25-cold-agent-bar, whose `stand_keyboard` gained `the_flight_type_select_moves_its_choice_by_its_arrows`), over the tier-0 witnesses alone, 9 files and 59 tests …`"
    sidecar: "§9 Legs: the a11y leg 14 files, 84 → 85 tests (`stand_keyboard` 14 → 15)."
    rationale: "Report F8 (`a11y-84`: test-plan ci-integration.md:8 stale, the a11y leg 14 files, 84 → 85) and F7 (`stand_keyboard` tests 14 → 15); gate `ci-leg.sh a11y` green (14 files, 85 passed)."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:249-309
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 63
    section: "## 9. CI Integration"
    change: "append to the re-count chain: `; re-counted with the served Cells and load page: 208 result lines, 1117 passed · 0 failed · 12 ignored — +2 result lines (`stand_cells_served`, `stand_load_page`) and +12 passed (`stand_cells_served` 4, `stand_load_page` 2, seven_guis' three stand unit tests, `host_log` 1, `stand_keyboard` 1, `stand_snapshot` 1), timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (a `ci-leg.sh fast` run's tally probe)`"
    sidecar: "§9 Local baseline: workspace re-count 206 · 1105 → 208 · 1117 · 0 · 12; the earlier readings dated."
    rationale: "Report F11 (`tally`: 206 lines · 1105 passed → 208 · 1117, 0 failed, 12 ignored; ci-integration.md:63 dated) and gate `ci-leg.sh fast` tally probe `lines 208 passed 1117 failed 0 ignored 12`; new tests per Symbols / APIs and F13 (`host_log` 3 → 4), F7 (`stand_keyboard` 14 → 15)."
    basis: tests/blitz-tests/tests/stand_load_page.rs:34-68
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 65
    section: "## 9. CI Integration"
    change: "after the a11y reading's `(…as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md)` add `; re-read at 2026-10-25-cold-agent-bar: fourteen files, 85 passed · 0 failed, `stand_keyboard` 15 (escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)`"
    sidecar: "§9 Local baseline: the a11y leg re-read 84 → 85 passed, `stand_keyboard` 14 → 15."
    rationale: "Report F8 (ci-integration.md:65 dated at 84) and gate `ci-leg.sh a11y` green (14 files, 85 passed); F7 `stand_keyboard` tests 14 → 15."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:249-309
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 16
    section: "## 10. Quality Gates & Coverage Targets"
    change: "append: `; read again with the served Cells and load page, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 % (unchanged), dioxus-native-dom 2741/3202 = 85.60 % (was 85.57 %), blitz-test-harness 952/1008 = 94.44 % (was 94.15 %), `coverage floor: 3 of 3 crates at or above their floors` (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, its `ci-leg.sh coverage` gate)`"
    sidecar: "§10 Coverage floors: readings dioxus-native-dom 85.57 → 85.60 %, blitz-test-harness 94.15 → 94.44 %, escher-driver 97.99 % unchanged; floors unchanged."
    rationale: "Report F12 (`coverage`: quality-gates-coverage-targets.md:16 stale) and gate `ci-leg.sh coverage` green (3 of 3 crates at or above their floors); the task's Untouched: floor values and `coverage_floor.py` unchanged."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 22
    section: "## 10. Quality Gates & Coverage Targets"
    change: "read `… run the 50 `stand_*.rs` files (with `stand_cells_served` and `stand_load_page`, joining by the prefix since 2026-10-25-cold-agent-bar, the local leg `stand-repeat: 3 of 3 rounds green` and CI#40214713710 green 17/17; 48 before them), the ten named blitz-tests targets …` — the older readings kept as written"
    sidecar: "§10 Flakiness budget: stand-repeat over 50 `stand_*` files (was 48), 3 of 3 rounds green."
    rationale: "Report F10 (`run-stand`: quality-gates-coverage-targets.md:22 stale, \"48 files\"), Harness / gate surface (the two new files by prefix, no CI step changes), gate `ci-leg.sh stand-repeat` green 3 of 3 and CI#40214713710 green 17/17."
    basis: tests/blitz-tests/tests/stand_cells_served.rs:91-133
note: the six slice absences (#3.3–#3.8) were ruled out on the report alone — no slice membership was read; if a slice's fixed file set holds a reached file, its search wants re-running by the orchestrator.
