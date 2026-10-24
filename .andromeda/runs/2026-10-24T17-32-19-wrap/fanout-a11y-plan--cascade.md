proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: "## 6. Visual Design Verification › Color contrast"
    change: "Re-point the row's two `tests/blitz-tests/tests/stand_lint.rs` citations (the stand-at-boot reading and the planted low-contrast row) to the same tests in the work tree. The chunk added lines 4-7, 110-142 (`reading_page()`) and 226-275 (`a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`) to that file, so both cited ranges moved. The prose stays as it is: no `contrast` reading, exempt rule or finding form moved, and the `a11y` leg still reads 43 tests over 6 targets."
    sidecar: "visual-design-verification#1.5 — `stand_lint.rs` citations re-pointed after 2026-10-24-stand-requirement-sweep added lines 4-7, 110-142 and 226-275; prose unchanged"
    rationale: "The report's Changes list `stand_lint.rs` (6 → 7 tests, the in-file page `reading_page()` at 110-141 and the co-reading test at 226-274), and New text, by line gives its added ranges as 4-7 · 110-142 · 226-275. Both of the row's citations lie after the first insertion, so they moved. The Outcome reads the `a11y` leg at 6 · 6 · 4 · 8 · 5 · 14 = 43, unmoved, so the row's tally holds."
    basis: tests/blitz-tests/tests/stand_lint.rs:226
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 30
    section: "## 7. Screen Reader Support › Accessibility tree output"
    change: "Re-point the row's two `tests/blitz-tests/tests/stand_snapshot_parts.rs` citations (the over-budget parts walk and the `serve_mcp` walk) to the same tests in the work tree. Name the over-budget fixture where it now lives: `fixtures::many_items` (tests/blitz-tests/tests/fixtures/mod.rs:67-85, with `ITEMS` at 64-86), a shared fixture page that `stand_snapshot_parts` and, since 2026-10-24-stand-requirement-sweep, `stand_act_spans` both read. The readings stay as they are."
    sidecar: "screen-reader-support#3.1 — `stand_snapshot_parts.rs` citations re-pointed; the over-budget fixture named as the shared `fixtures::many_items` after 2026-10-24-stand-requirement-sweep"
    rationale: "The report's Changes say `ITEMS` and `many_items` moved unchanged from `stand_snapshot_parts.rs` to `fixtures/mod.rs` (64-86), and that `stand_snapshot_parts.rs` reads `fixtures::many_items` (line 23 added). Lines left the file, so the later cited lines moved. The row's `tests/blitz-tests/tests/fixtures/mod.rs:37-44` citation (`password_named`) holds, because the only range added to that file, 64-86, comes after it."
    basis: tests/blitz-tests/tests/fixtures/mod.rs:67
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Append to the row: since 2026-10-24-stand-requirement-sweep `stand-repeat` also runs three a11y witnesses outside the `stand_` prefix — `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` — by a fixed `--test` list after its `tests/blitz-tests/tests/stand_*.rs` glob, on the same `cargo test -p blitz-tests --locked` line every round (.github/scripts/ci-leg.sh:54-58). They run three rounds under the zero-failure budget and keep their names, on the founder's word «Список в скрипте (Рекомендую)» (2026-10-24). `test_stand_repeat_leg_runs_three_rounds` pins the list's ten words (.github/scripts/test_ci_workflows.py:359-362). The a11y leg's six files and its job are unchanged, at 6 · 43 under CI#40114977936 — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md. Re-point the row's `ci-leg.sh` stand-repeat range to the block that now holds lines 54-58."
    sidecar: "ci-integration#1.0 — stand-repeat runs `accessibility_states`, `inert_subtree` and `accessibility_active_descendant_unread` by a named `--test` list (founder, 2026-10-24); a11y leg unchanged"
    rationale: "Task F1. In the report, Schema / config gives the `stand-repeat` named list at `ci-leg.sh` 54-58, and Decisions gives the founder's word (inputs#I1). Expected amendments name a11y-plan §3/§9 (the three also run in stand-repeat, the a11y leg's six unchanged). The Outcome reads the a11y leg at 43 and CI#40114977936 green, 17/17. The row's sentence on which a11y files stand-repeat runs names only the three `stand_*` files, so it is now incomplete."
    basis: .github/scripts/ci-leg.sh:54
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 9
    section: "## 9. CI Integration"
    change: "Add beside the row: `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` (tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069) reads `ci-leg.sh` and asserts that every 0.2.0 integration witness runs inside `stand-repeat`, either by the `stand_*` glob or by the named list (`STAND_REPEAT_NAMED`, tests/blitz-tests/tests/stand_requirement_sweep.rs:820-832). Dropping `accessibility_states`, `inert_subtree` or `accessibility_active_descendant_unread` from the list therefore turns the check red — as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md."
    sidecar: "ci-integration#1.0 — the a11y witnesses' stand-repeat membership is held by `stand_requirement_sweep`'s leg check (2026-10-24)"
    rationale: "Task F2. The report's Symbols / APIs give the test (994-1069): it asserts that every non-`Unit` blitz-tests witness is a `stand_*` file or one of `STAND_REPEAT_NAMED` (820-832), and that the leg's text holds `--test {name}` for all 18 names. The Outcome reads it as met at gate entry 3."
    basis: tests/blitz-tests/tests/stand_requirement_sweep.rs:994
    fact: F2
note: "The report also moves the body's runner rows `.andromeda/a11y-plan.md:53` (`accessibility_states`) and `:55` (where the shared runner clause splits: `inert_subtree` and `accessibility_active_descendant_unread` gain stand-repeat, `accessibility_merged_states` does not). Neither row is a picked entity, so neither is proposed here. The report rules `a11y-plan.md:235` no-change."
