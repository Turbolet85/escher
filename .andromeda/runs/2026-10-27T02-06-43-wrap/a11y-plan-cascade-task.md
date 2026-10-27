# Task — a11y-plan · 2026-10-26-codebase-health

## The session
The chunk is a refactor for code health (v030-01). It split three functions that were over the cognitive-complexity ceiling of 15 into private helpers. The three are escher-driver's lint `findings` walk, the MCP listing test `the_listing_is_the_twelve_verbs_of_the_table` and the 0.2.0 requirement sweep test `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`.
No public symbol, signature, verb, field, carrier, socket, port, env var, crate, dependency, CI leg, CI step or engine file changed. Nothing under packages/blitz-dom/ changed, and the lint's findings, their order and its fonts-unavailable refusal are unchanged.
New private items. In lint.rs, `readings` and `placement`. In mcp_tests.rs, `assert_described`, `assert_schema` and `assert_property`. In stand_requirement_sweep.rs, `assert_0_2_0_row`, `assert_0_2_0_witness` and `inside_stand_repeat`. Nothing was retired.
Measured: the complexity population over 15 went from 88 to 85. escher-driver coverage went from 4918/5019 to 4930/5031, still 97.99 %. The a11y leg reads 14 · 85 · 0 · 0, stand-repeat 204 · 864 · 0 · 21 over 3 rounds, the stand checks 205 · 0 · 7 and the workspace 208 · 1117 · 0 · 12, all unchanged. CI#40434241601 is green 17/17.
Disproved by measurement: the sweep test reads ci-leg.sh whole by `contains`. If `--test select_interaction` is dropped from the stand-repeat list alone (ci-leg.sh:57), the test stays green, because the a11y leg (ci-leg.sh:39) still names it. Only removing both reads red. This predates the chunk. Today stand-repeat membership is held jointly with .github/scripts/test_ci_workflows.py:361-366 (`test_stand_repeat_leg_runs_three_rounds`). An owner for the gap is placed at P5.

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/inputs/andromeda-inputs.json
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson

## Touches
- §9 CI Integration (ci-integration): the claim that the stand-repeat list's names have their membership held by `stand_requirement_sweep`'s leg check is contradicted. The check reads ci-leg.sh whole by `contains`. A name such as `select_interaction` dropped from the stand-repeat list alone stays green while the a11y leg still names it. Membership is held jointly with test_ci_workflows.py's `test_stand_repeat_leg_runs_three_rounds` (lines 361-366).
- §9 CI Integration: any sentence that credits the sweep test alone with guarding which a11y files stand-repeat runs three times now overstates its reach. This is indirect, through the disproved claim above.
- §9 CI Integration: the gap's owner is still to be placed (P5, a `.andromeda/residuals.md` line or a route CARRY). An entry that lists open a11y-leg or stand-repeat owners may need to name it.
- §9 CI Integration counts, re-read and unmoved: the a11y leg has fourteen files and reads 14 · 85 · 0 · 0. Eight of those files also run in stand-repeat, which reads 204 · 864 · 0 · 21 over 3 rounds. ci-leg.sh is byte-unchanged.
- Citations into tests/blitz-tests/tests/stand_requirement_sweep.rs moved. The sweep test now runs from :1016, and with its three helpers it runs to the file's end at :1109 (the old span was :1015-1090). Spans that end above :1017 did not move. An a11y-plan citation of the sweep test's span moves with it.
- Citations into packages/escher-driver/src/lint.rs past :273 moved. `ordered` went from :299 to :315, `overlaps` from :334 to :350, and the inline `mod tests` from :526-604 to :542-620. The file is now 620 lines. Spans above :273 did not move, including `Session::lint` at :221-230 and the contrast reader.
- Citations into packages/escher-driver/src/mcp_tests.rs past base line :480 moved by +17. The file is now 803 lines.
- §6 visual-design-verification lines 14-15: the `lint.rs:` hits there are `tests/blitz-tests/tests/stand_lint.rs` citations, a file this chunk did not edit. Their spans did not move. Re-read and unchanged.
- §6 visual lint's contrast reading (visual-contrast): the `findings` walk is now factored into `readings` and `placement`. The contrast arm still reads through `Harness::text_contrast` and `required_ratio()` with `stand_contrast`'s exempt rule, with no second formula. The planted row reads 2.8490272 against 4.5. Re-read and unchanged.
- §6 visual lint's defect order: the order per entry is unchanged. A box with no area yields zero-size alone; otherwise the order is placement (off-screen or clipped), then truncated, then contrast. Overlap pushes stay first, and findings keep `Defect::ALL` order. Re-read.
- §6 lint readings on the stand: `[0, 1, 0, 0]` at boot (the flight booker's select-option overlap), the planted seven, and the twelfth-Create `clipped` row inside `crud-list`. `stand_lint` has 7 tests, green in both layout modes and both builds. Re-read and unchanged.
- Test inventory counts, re-read and unmoved: blitz-tests has 147 `.rs` files and 146 targets, `stand_requirement_sweep` has 3 tests, escher-driver has 100 unit tests and `mcp_tests.rs` has 15 on unix. Any a11y-plan count of tests or targets that rests on these holds.
- Gate surface: ci.yml, ci-leg.sh, listing.rs, `VERBS` and the committed checks are byte-unchanged. The a11y job's command, its tier and its lack of a required status check stand. Re-read.
- The complexity population (88 to 85, with `clear_interaction_state_for_removed_node` still at 17.0 in blitz-dom) is the audit's and the matrix's reading. No master states it, so a11y-plan holds no site for it. If an entity states it, that entity moves.

## New facts
- F1 · ci-integration · `stand_requirement_sweep`'s leg check reads ci-leg.sh's text whole. ci-leg.sh names `--test select_interaction` both in the a11y leg (:39) and in the stand-repeat list (:57). So the check cannot see a target dropped from the stand-repeat list alone while the a11y leg still names it, and only removing both sites reads red. This is a measured reach gap that predates 2026-10-26-codebase-health, and its owner is placed at P5.
- F2 · ci-integration · Stand-repeat membership of the a11y-leg files is held jointly by two checks: `stand_requirement_sweep`'s leg check and `.github/scripts/test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds` (:361-366). The second asserts `select_interaction` among the stand-repeat leg's own logged words.

## Untouched
- §1 A11y Scope Summary: the Dioxus crates' `accessibility` feature, its manifests and the adapter's residual. No Cargo.toml, Cargo.lock or deny.toml change.
- §2 A11y Strategy: the accessibility tree lifecycle, the changed set, role derivation, the ARIA state reads and upstream's six reads. No engine or accessibility.rs file was edited.
- §3 harness tests (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread`) and their citations. Those files are unedited, and their counts and legs are unchanged.
- §3 contrast harness: `Harness::text_contrast`, `contrast_ratio`, `required_ratio` and `stand_contrast`'s 17 control rows, 14 `TEXT_ROWS` and 18 `STATE_ROWS` with their citations. blitz-test-harness and stand_contrast.rs are unedited.
- §4 ARIA Patterns & Roles: the landmark mapping, the state attribute mapping and hidden content.
- §5 Keyboard Navigation: focusability, disabled-ness, click focus, the double click, the range and select key models, Enter and Space activation, the Dioxus boolean attributes, the CRUD list keys, `stand_keyboard`'s tables and the open load-fetch gap. None of these was edited.
- §6 visibility and hit-testing, the focus ring, motion, state colour and preferences.
- §7 Screen Reader Support: labels, `author_id`, the snapshot model, `MASKED_VALUE`, the actionable-key rule, the stand's ARIA states and the platform adapter.
- §8 Cognitive Accessibility: the flight booker's `aria-invalid` cue.
- The obs log format paragraph in §3. No span, event, field or `info_span!` was added; there are still 2.
