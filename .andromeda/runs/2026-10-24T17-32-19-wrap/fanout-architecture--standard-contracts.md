proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 256
    section: "## Standard Contracts"
    change: "In the `stand-repeat` leg, after «with one `--test` per `tests/blitz-tests/tests/stand_*.rs`» add «and, since 2026-10-24-stand-requirement-sweep, ten named targets after that glob on the same line every round — `inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent`, the 0.2.0 witnesses outside the `stand_` prefix, written as literal `--test {name}` words, the files keeping their names, ratified by the founder (2026-10-24, «Список в скрипте (Рекомендую)») (.github/scripts/ci-leg.sh:54-58)»; restate «the script reads the floor script and the `stand_*.rs` set beside itself» as «the script reads the floor script and the `stand_*.rs` set beside itself and names the ten in its own text»; and add after the leg's description «its selection pinned by `test_stand_repeat_leg_runs_three_rounds`, which asserts the ten words (.github/scripts/test_ci_workflows.py:359-362), and read by `stand_requirement_sweep`'s `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`, which finds the `stand_*.rs` glob and `--test {name}` for all 18 named targets (the ten and seven_guis' eight) in the script (tests/blitz-tests/tests/stand_requirement_sweep.rs:994-1069)»; the seven_guis half, `--locked`, no retry, no `--ignored` and no env var unchanged."
    sidecar: "CI contracts: stand-repeat runs the stand_* glob plus ten named blitz-tests targets (founder, 2026-10-24); the leg pin and the sweep record's leg check read them."
    rationale: "Report Changes › Schema / config (ci-leg.sh stand-repeat 54-58, test_ci_workflows.py 359-362) and Counts fact 1 name standard-contracts.md:256 moved on both clauses (`--test` per `stand_*.rs`; the script reads the `stand_*.rs` set); Decisions › the founder's word inputs#I1; Expected amendments › architecture §Standard Contracts → CI contracts."
    basis: .andromeda/architecture-detail/standard-contracts.md:256
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 181
    section: "## Standard Contracts"
    change: "Replace «a node both clipped or off-screen and holding text also reads `truncated`, asserted by no check» with «a node both clipped or off-screen and holding text also reads `truncated`, asserted since 2026-10-24-stand-requirement-sweep by `stand_lint`'s `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`, in both layout modes, on its in-file page `reading_page()`: `fx-cut-text` (a nowrap line past its 100 × 40 `overflow: hidden` box) reads `clipped` and `truncated`, its line seen in part; `fx-padded-text` (a 60 px paragraph whose `padding-right: 100px` alone passes its box) reads `clipped` alone; `fx-off-text` (wholly right of the 800 px viewport) reads `truncated`, no part seen, and `off-screen`; the `lint` verb answers the same (tests/blitz-tests/tests/stand_lint.rs:110-141; tests/blitz-tests/tests/stand_lint.rs:226-274)», keeping the 2026-10-19-visual-lint measurement cite."
    sidecar: "Driver session › visual lint: the clipped/off-screen + truncated co-reading is now asserted by stand_lint (7 tests)."
    rationale: "Report Counts fact 4 names standard-contracts.md:181 moved («asserted by no check» → asserted by stand_lint); Symbols › stand_lint.rs states the page, the test and its five findings; Expected amendments › → Driver session (the co-reading asserted)."
    basis: .andromeda/architecture-detail/standard-contracts.md:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 54
    section: "## Standard Contracts"
    change: "After «Its limits: RTL and vertical text are read as LTR horizontal lines, and a transformed box is read at its client rect.» add «— each limit asserted as it reads since 2026-10-24-stand-requirement-sweep by a `text_extent` test, in both layout modes and under the per-package and the workspace build, and run inside `stand-repeat`: a `dir=\"rtl\"` paragraph reads a left-to-right line from its content box's left, a `writing-mode: vertical-lr` paragraph a horizontal line, and a `transform: scale(2)` box a content box equal to its `get_client_bounding_rect` — twice its `offset_rect` about the same centre — with its lines unscaled (tests/blitz-tests/tests/text_extent.rs:127-152; tests/blitz-tests/tests/text_extent.rs:154-174; tests/blitz-tests/tests/text_extent.rs:176-205)»; the limits stand as limits."
    sidecar: "Scrolling, selection, tree › text extent: its three stated limits are now asserted by text_extent's three new tests (3 → 6), inside stand-repeat."
    rationale: "Report Counts fact 5 keeps standard-contracts.md:54 as a limit and says a pointer to their check may be added; Symbols › text_extent.rs names the three tests and their readings; Expected amendments › → Scrolling, selection, tree (`text_extent`'s limits asserted, standard-contracts.md:54)."
    basis: .andromeda/architecture-detail/standard-contracts.md:54
note: "ci-leg.sh gained 5 lines at 54-58 inside the stand-repeat leg, so the file's ci-leg.sh range cites past line 53 (`:48-65` and `:8-89` at line 256, possibly `:41-42`/`:44-45` unaffected) likely shift by 5 — the report states no new range, so they are left to a cites re-point; no detector fired on D-arch-decisions (no dependency, runtime or library added)."
