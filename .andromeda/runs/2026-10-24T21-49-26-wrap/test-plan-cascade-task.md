# Task — test-plan · 2026-10-24-a11y-re-read

## The session
The session widened the `a11y` CI leg from six to fourteen `--test` targets, one line replaced in the leg script (line 39), still one `cargo test --workspace --locked` line with no comment line.
The eight joining files are accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction and stand_act_select, after the six it already ran (accessibility_hidden, accessibility_roles, focusability_updates, stand_keyboard, stand_contrast, stand_accessibility_ids), order kept.
The leg's pin, the a11y row of `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`, now names the fourteen words, one assertIn each over the shimmed log, in no order; its lines moved from 298-299 to 298-302.
It measured the leg at 14 result lines · 84 passed · 0 failed · 0 ignored (was 6 · 43) locally and on CI#40134371519 (green 17/17 on 4cbf0a4c9), every other tally unmoved.
It re-read the stand's Tab order, Enter/Space activation and contrast after 0.2.0's engine changes: green, no table edited, nothing re-baselined. No Rust source, stand, ci.yml, manifest or lock changed, and nothing was retired.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/scope.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/research.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/inputs/andromeda-inputs.json
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/operator-pass.md

## Touches
- §9 CI Integration, the a11y leg's selection: the six words and "six files since 2026-10-14-stand-a11y-assertions" are now fourteen, the six first and then the eight joining files in the order the session names (report cites ci-integration line 8).
- §9 CI Integration, the per-job CI readings: the a11y job read 6 · 43 · 0 · 0 on CI#40125511019 and now reads 14 · 84 · 0 · 0 on CI#40134371519. The last dated reading (CI#40092092468, a11y 6 · 43 "unmoved", line 42) stays true as dated, and this run's reading is owed after it.
- §9 CI Integration, the other jobs on CI#40134371519, all equal to CI#40125511019's: Test [default features] 206 · 1105 · 0 · 12, Test (macos) 187 · 1102 · 0 · 12, Coverage report 187 · 1102 · 0 · 12, Test (windows) 187 · 1057 · 0 · 9, Stand checks repeated 198 · 837 · 0 · 21. This is a re-read, and they did not move.
- §9 CI Integration, the local `ci-leg.sh a11y` reading (line 64): six files and 43 tests become fourteen files and 84 tests. The six keep their counts (accessibility_hidden 6, accessibility_roles 6, focusability_updates 4, stand_keyboard 14, stand_contrast 5, stand_accessibility_ids 8).
- §9 CI Integration, per-file counts of the eight joining files in the leg: accessibility_states 5, accessibility_label 13, accessibility_description 3, accessibility_merged_states 3, accessibility_active_descendant_unread 1, inert_subtree 4, select_interaction 7, stand_act_select 5, for 41 in all.
- §1 Test Scope Summary, accessibility_states: its "not by the `a11y` leg, which runs six files … not among them" (line 97) is contradicted, because the file now runs in the leg.
- §1 Test Scope Summary, accessibility_label and accessibility_description: "not by the `a11y` leg's six" (line 98) is contradicted, because both joined the leg.
- §1 Test Scope Summary, focus_within and focus_redraw: "not by the `a11y` leg's six" (line 99) still keeps them outside the leg, but the count six is stale and is now fourteen.
- §1 Test Scope Summary, hit_test_dropped_nodes: it stays outside the leg, and "six" is stale (line 100). On the same line, accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree are marked outside the leg's six, but they have now joined.
- §1 Test Scope Summary, select_interaction and stand_act_select (line 125): the legs each runs under must now include the `a11y` leg. select_interaction runs in test, stand-repeat, by name and in `run all`, and not in `run stand`. stand_act_select runs in `run stand` and in stand-repeat by its prefix.
- §1 Test Scope Summary, stand_accessibility_ids (line 93) and stand_contrast and stand_keyboard (line 116): each is still run by the `a11y` leg. This is a re-read, and the claims did not move.
- Every joining file keeps the legs it already ran in: the workspace `test` leg and its per-package run for all eight, and the stand-repeat leg for accessibility_states, inert_subtree, accessibility_active_descendant_unread, select_interaction and stand_act_select. stand_act_select now runs in both the `a11y` and stand-repeat legs, as stand_keyboard, stand_contrast and stand_accessibility_ids already did.
- §4 Unit Test Strategy, the a11y row of the leg pin (line 28): "pinning all six test files in order" is stale on both counts. It now pins fourteen words, one assertIn per word, in no order.
- §4 Unit Test Strategy, any citation of the pin file: the a11y tuple's range moves from 298-299 to 298-302, and every cited line after 299 in that file moves by +3.
- §4 Unit Test Strategy, the CI scripts' counts: the workflow test file still reads Ran 33 tests and the `ci-scripts` leg Ran 100 tests. The dated "25 over six files" of 2026-10-13 (line 29) stays true as dated. This is a re-read, and the counts did not move.
- §3 Test Harness Contract and the stand counts: `run stand` reads passed 197 · failed 0 · ignored 7, and `agent-run.sh boot` reads ready. This is a re-read, and the counts did not move.
- The workspace tally `lines 206 passed 1105 failed 0 ignored 12` and stand-repeat 198 · 837 · 0 · 21, `3 of 3 rounds green`, did not move, read again in this session. The leg's ten named words and its stand glob are unchanged.
- §9 CI Integration, the a11y job's ci.yml shape is byte-identical: slow tier, its `needs`, the `workspace-test` restore with `save-if: false`, failure-only log upload, no required status check. This is a re-read, and it did not move.
- The stand_requirement_sweep leg check, which reads the leg script by `include_str!`, stays green with the leg line changed. It is a test inventory entry whose subject moved.
- The a11y re-read of the stand: stand_keyboard 14, stand_contrast 5 and stand_accessibility_ids 8 are green in the leg, in the per-package run of the fourteen (14 · 84) and in each stand-repeat round. tab_order, ENABLED_TAB_ORDER, controls, TEXT_ROWS and STATE_ROWS are unedited and nothing was re-baselined, so this is a re-read against 0.2.0's engine changes with nothing moved.

## New facts
- F1 · ci-integration · The founder's word of 2026-10-24 (inputs#I1, «Плюс проверки дерева (Рекомендую)») set the `a11y` leg's membership: option (b), the tier-0 witnesses plus the five tree-reading checks, 14 files and 84 tests. He passed over option (a), the tier-0 witnesses alone, 9 files and 59 tests. focus_redraw, focus_within and accessibility_names were deliberately not added.
- F2 · ci-integration · Each file in the `a11y` leg witnesses one of three things: an SC 2.1.1 or 2.4.3 requirement, a tree reading the founder took, or a recorded gap. accessibility_active_descendant_unread is the leg's pinned-gap witness.
- F3 · ci-integration · A per-job tally read from a CI job log through `gh run view --log` must strip ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`. The log spells the escape as the two literal characters `^[`, so stripping `\x1b\[[0-9;]*m` alone leaves every cargo `Running tests/{name}.rs` line unattributed (measured on CI#40134371519).
- F4 · test-scope-summary · aria-checked is asserted by accessibility_states alone (`aria_checked_reads_its_three_values_on_a_checkable_role_alone`, `aria_pressed_on_a_checkable_role_reads_nothing_and_a_long_aria_checked_is_nowhere`). accessibility_merged_states reads aria-expanded, aria-disabled and aria-level, not aria-checked.
- F5 · test-scope-summary · The stand's normal dependency graph holds neither platform accessibility adapter (`accesskit_xplat`, `accesskit_winit`), measured by a `cargo tree --locked -p seven_guis -e normal` probe reading 0. So no check in the `a11y` leg reaches the adapter crossing.

## Untouched
- Dated per-job CI readings of earlier runs (a11y 6 · 43 before CI#40092092468) and §3's dated agent-run readings (`run all` over the three a11y-leg files, 2026-10-06) stay true as dated. Only the newest reading is owed after them.
- Counts that only share the token "6 files": the 86 files of slice s04, the 16 files of slice s08, and `run stand`'s 26, 36 and 46 files. None of them is about the a11y leg.
- §9's cache fact, where the a11y and stand-repeat jobs restore `workspace-test` with `save-if: false`, is unchanged.
- §10's flakiness budget and the stand-repeat leg's composition are unchanged: the `stand_*.rs` glob, the ten named targets, seven_guis' process checks and three rounds with no retry.
- §10's coverage floors and the coverage leg are unchanged (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, keyboard_compat excluded), and so is the dev-host-only mutation tier.
- §9's fast/slow split, triggers, package installs, per-OS tallies, engine features by runner, fonts and local baseline are unchanged: no job, `needs`, trigger or install was edited.
- §4's Rust unit tests in every crate are unchanged: no Rust source, symbol or crate changed.
- §5 Integration, §6 E2E, §7 Test Data & Fixtures and §8 Mocking & Stubbing are unchanged: no fixture, page, stand-in or check body was edited.
- §2 Test Strategy and its naming conventions are unchanged.
- The agent-run contract tests and the cold-agent pipe tests are unchanged, and `agent-run.sh` was not edited.
- The owed check of accessibility states as the platform adapter hands them to an assistive technology is still owed by its residuals line. The session adds no windowed or adapter check.
