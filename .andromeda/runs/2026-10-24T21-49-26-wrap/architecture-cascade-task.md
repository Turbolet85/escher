# Task — architecture · 2026-10-24-a11y-re-read

## The session
The `a11y` CI leg (`bash .github/scripts/ci-leg.sh a11y`, one `cargo test --workspace --locked` line at `.github/scripts/ci-leg.sh:39`) widened from six `--test` targets to fourteen, on the founder's «Плюс проверки дерева (Рекомендую)» (2026-10-24, option b over the tier-0-only option a).
The six kept, in their order: accessibility_hidden, accessibility_roles, focusability_updates, stand_keyboard, stand_contrast, stand_accessibility_ids; the eight joining: accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction, stand_act_select.
Its pin, the a11y row of `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`, now names the fourteen, one assertIn per word over the shimmed log, in no order; its range `.github/scripts/test_ci_workflows.py:298-299` became `:298-302`, and every later line of that file moved +3.
Measured: the leg 6 result lines · 43 passed → 14 · 84 · 0 failed · 0 ignored, locally and in the CI a11y job on CI#40134371519 (green 17/17); every other tally unmoved (workspace 206 · 1105 · 0 · 12, stand-repeat 198 · 837 · 0 · 21, run stand 197 passed, test_ci_workflows.py 33 tests, ci-scripts 100).
Re-read: stand Tab order, Enter/Space activation and contrast re-asserted green after 0.2.0's engine changes, with tab_order, ENABLED_TAB_ORDER, controls, TEXT_ROWS and STATE_ROWS unedited — nothing re-baselined.
No engine, stand, Rust source, `ci.yml`, manifest, lock or dependency changed; nothing retired.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/scope.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/research.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/operator-pass.md

## Touches
- standard-contracts, contract-ci-contracts (the a11y leg's integration tests, around line 255): its count six, its qualifier "six since 2026-10-14-stand-a11y-assertions" and its list are stale — the leg now runs fourteen targets.
- standard-contracts, contract-ci-contracts: the pin's citation `.github/scripts/test_ci_workflows.py:298-299` is now `:298-302` (the range stretched, not moved).
- standard-contracts, contract-ci-contracts: any citation into `.github/scripts/test_ci_workflows.py` past line 299 (for instance the two coverage `LegScriptTest` cases near line 259) moved +3; citations at or before line 298 are unmoved.
- standard-contracts: any wording that the a11y pin asserts its files in order is stale — the pin is one assertIn per word, in no order.
- existing-scopes (around line 102): accessibility_states described as not run by the `a11y` CI leg, which runs six named files — now contradicted; it runs in the leg.
- existing-scopes (line 102's neighbours): the accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree lines name no leg — each now runs in the `a11y` leg as well as its other legs.
- existing-scopes (around line 111): stand_act_select and select_interaction, described with no leg named — both now run in the `a11y` leg; stand_act_select in both `a11y` and `stand-repeat` (by its `stand_*` glob), as stand_keyboard, stand_contrast and stand_accessibility_ids already do.
- existing-scopes: any entry for accessibility_label or accessibility_description that places them outside the `a11y` leg is now stale.
- existing-scopes (around lines 106 and 112): stand_keyboard in the `a11y` leg since 2026-10-14 and stand_contrast run by it — re-read, still true; their tables unedited and green.
- existing-scopes: any count of the a11y leg's files or tests (six, 43) is stale — fourteen files, 84 tests.
- Every joining file keeps the legs it already ran in: the workspace `test` leg and per package for all eight; `stand-repeat` for accessibility_states, inert_subtree, accessibility_active_descendant_unread and select_interaction (by name) and stand_act_select (by glob); `run stand` for stand_act_select — a leg list in any entry that names these legs gains `a11y` and loses none.
- Body, Infrastructure Patterns, CI/CD (line 246): the job list naming accessibility (a11y) tests, thirteen linux jobs each running `bash .github/scripts/ci-leg.sh {leg}` and the citation `.github/scripts/ci-leg.sh:18-67` — re-read, unmoved (ci-leg.sh's line count is unchanged).
- Body, Infrastructure Patterns, CI/CD (line 248): the citation `.github/scripts/test_ci_workflows.py:186-199` lies before line 299 — unmoved.
- occupied-resources, res-ci-infrastructure (around line 71): the `a11y` job restoring the test job's `workspace-test` cache with `save-if: false` — re-read, unmoved.
- Indirect, through the leg: the a11y CI job's runtime work grows (eight more test targets in the one cargo line) while its tier, `needs`, restore-only cache, failure-only log upload and absence of a required status check are unchanged.
- Indirect, through the founder's ruling: the WCAG SC 2.1.1 · 2.4.3 · 1.4.3 witnesses (stand_keyboard, stand_contrast, inert_subtree, select keyboard model) now run in CI's a11y job, not only in the workspace test and stand-repeat legs.

## New facts
- F1 · standard-contracts · The `a11y` leg's fourteen targets were set by the founder's «Плюс проверки дерева (Рекомендую)» (2026-10-24): the five tree-reading checks (accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread) joined beside the three tier-0 witnesses (inert_subtree, select_interaction, stand_act_select), the six earlier files kept in order.
- F2 · standard-contracts · focus_redraw, focus_within, accessibility_names and hit_test_dropped_nodes stay outside the `a11y` leg (run by the workspace `test` leg); the leg remains one `cargo test --workspace --locked` line with no comment line.
- F3 · standard-contracts · `LegScriptTest`'s a11y row asserts each of the fourteen target names by one assertIn over the shimmed log, in no order (`.github/scripts/test_ci_workflows.py:298-302`).
- F4 · existing-scopes · select_interaction and stand_act_select run in the `a11y` CI leg; stand_act_select runs in both the `a11y` and `stand-repeat` legs.
- F5 · existing-scopes · accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread and inert_subtree run in the `a11y` CI leg, beside the workspace `test` leg (and, for accessibility_states, accessibility_active_descendant_unread and inert_subtree, `stand-repeat`).

## Untouched
- Engine contracts of blitz-dom (accessibility builder and its aria reads, inert, the select and range input models, keyboard activation, focusability, the document's clock): no engine source changed; the chunk only re-ran their checks.
- dioxus-native-dom's snapshot, stable ids, text parts and diff contracts: no source changed.
- escher-driver's session, schema, command line, MCP server, recording and lint contracts and dec-driver-session: no source changed.
- The headless test harness and headless stand contracts (settle, screenshot, contrast reader, stand boot): unchanged; stand tables were re-read, not edited.
- `.github/workflows/ci.yml` job structure (the a11y job's tier, `needs`, cache key, upload, no required check): byte-identical.
- dec-dependency-pinning, stack-and-technologies and inherited-defaults: no manifest, lock, `deny.toml` or toolchain change, and the stand graph still holds no platform AccessKit adapter.
- occupied-resources' listeners, filesystem, environment variables and names: no listener, port, env var, state directory or workspace crate added.
- Telemetry bootstrap, logging conventions and the crosscut-logging-and-timing rule: no log site added (silence census 0).
- Conventions (tests, formatting and lints, feature gating): no test file added or reshaped; one test target per file holds.
- Project intent, design philosophy and the upstream-sync line: no product, scope or upstream change.
