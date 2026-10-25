# Task — a11y-plan · 2026-10-24-a11y-re-read

## The session
The `a11y` CI leg (`bash .github/scripts/ci-leg.sh a11y`, still one `cargo test --workspace --locked` line) widened from six `--test` targets to fourteen: the six it ran since 2026-10-14-stand-a11y-assertions, order kept, then `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`.
The leg's tally moved 6 result lines · 43 passed → 14 · 84 · 0 failed · 0 ignored, locally and on the CI "Accessibility (a11y) tests" job (CI#40125511019 6 · 43 → CI#40134371519 14 · 84); the eight joining files hold 41 tests, the six 43, unmoved.
The leg's pin, `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log`'s a11y row, now names the fourteen words, one `assertIn` each over the shimmed log, in no order; its coordinates `.github/scripts/test_ci_workflows.py:298-299` → `:298-302`.
The stand's Tab order, Enter/Space activation and contrast were re-read after 0.2.0's engine changes and found unmoved: `stand_keyboard` 14, `stand_contrast` 5, `stand_accessibility_ids` 8 green, the tables `tab_order`, `ENABLED_TAB_ORDER`, `controls`, `TEXT_ROWS`, `STATE_ROWS` unedited, nothing re-baselined.
The leg's composition is the founder's choice (inputs#I1, «Плюс проверки дерева (Рекомендую)», 2026-10-24): option (b), fourteen files and 84 tests, over (a), the tier-0 witnesses alone (9 files, 59 tests); no engine, stand, table, `ci.yml`, manifest or Rust source changed.

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
- The CI group's statement of the `a11y` leg's file list: six words → the fourteen, in the order above (detail ci-integration, and the body §9 line naming "over six files").
- The CI group's per-file log counts: the six (accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_keyboard 14 · stand_contrast 5 · stand_accessibility_ids 8) unmoved, joined by accessibility_states 5 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 4 · select_interaction 7 · stand_act_select 5.
- The leg's tally 6 · 43 → 14 · 84 · 0 · 0, and the CI a11y job's per-job tally 6 · 43 on CI#40114977936 and CI#40125511019 → 14 · 84 on CI#40134371519 (sha `4cbf0a4c9`), every other CI job's tally unmoved.
- The CI group's claim that the a11y leg's six files and its job are unchanged, reading 6 · 43 on CI#40114977936 — the files moved; the job's `ci.yml` properties did not.
- The pin's description: "the six" and "in order" are stale — the a11y tuple names fourteen words, one `assertIn` each, in no order; citation `.github/scripts/test_ci_workflows.py:298-299` → `:298-302`.
- Line citations into `.github/scripts/test_ci_workflows.py` past line 299 moved +3; `.github/scripts/ci-leg.sh` keeps its line count, the a11y arm at line 39.
- `test_ci_workflows.py` still runs 33 tests and the `ci-scripts` leg 100 — a count a standing group may cite, unmoved.
- Assertion-harness group (§3): `accessibility_states` "not in the `a11y` leg, which runs six files" — now in the leg, beside the workspace test leg, per package and three `stand-repeat` rounds.
- Assertion-harness group (§3): `accessibility_label` (13) and `accessibility_description` (3) "outside the a11y leg's six files" — now in the leg.
- Assertion-harness group (§3): `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread` "all three outside the a11y leg's six files" — now in the leg; `inert_subtree` and `accessibility_active_descendant_unread` keep their `stand-repeat` rounds.
- Keyboard-harness group: `select_interaction` and `stand_act_select` stated as "neither file in the a11y leg's six" — both now in the leg; `select_interaction` keeps its named place in `stand-repeat`, `stand_act_select` its `stand_*` glob place there and in `run stand`.
- `stand_act_select` now runs in both the `a11y` and `stand-repeat` legs, as `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` already did — any line saying the three `stand_*` files are the leg's ones in `stand-repeat` is now four.
- Visual-contrast group: "the `a11y` leg reads 43 tests over 6 targets, unmoved" (dated 2026-10-19-visual-lint) — the leg now reads 84 over 14; a newer reading is owed after it.
- Keyboard-shortcuts and sr-stand-states groups: the pinned gap `accessibility_active_descendant_unread` (no `aria-activedescendant` read) is now a witness the `a11y` leg runs in CI.
- Keyboard groups (focus order, Enter/Space activation, the CRUD list keys): re-read after 0.2.0, unmoved — `stand_keyboard` 14 green, `tab_order` and `ENABLED_TAB_ORDER` unedited; a re-read, no rule moved.
- Visual-contrast group: SC 1.4.3 met on every measured non-exempt stand row — re-asserted, `stand_contrast` 5 green, `controls`, `TEXT_ROWS`, `STATE_ROWS` unedited.
- sr-author-id / sr-tree groups: the tree the shell hands the adapter equals the harness's — re-asserted by `stand_accessibility_ids` 8 green in the leg, unmoved.
- Scope / platform-adapter statements that no stand binary builds a platform adapter: re-measured, the stand graph holding neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree` probe 0), unmoved.
- The leg's coverage claim: SC 2.1.1 · 2.4.3 · 1.4.3 witnesses run in CI through the leg — any statement of which a11y checks CI runs as their own leg reaches its new membership.
- Untouched-but-stale counts sharing the token: lines saying `focus_within` / `focus_redraw` stay outside "the a11y leg's six" — they stay outside, the "six" is stale wherever an a11y-plan group says it.

## New facts
- F1 · ci-integration · The leg's fourteen files are the founder's choice, inputs#I1 «Плюс проверки дерева (Рекомендую)» (2026-10-24): option (b), the five tree-reading checks joining the three tier-0 witnesses (14 files, 84 tests), passed over option (a), the tier-0 witnesses alone (9 files, 59 tests).
- F2 · ci-integration · `focus_redraw`, `focus_within` and `accessibility_names` were deliberately not added to the `a11y` leg (plan constraints); they run outside it.
- F3 · ci-integration · Every file joining the leg witnesses an SC 2.1.1 or 2.4.3 requirement, a tree reading the founder took, or a recorded gap (`accessibility_active_descendant_unread` the pinned gap), and each keeps every leg it already ran in (the workspace `test` leg, per package, its `stand-repeat` place).
- F4 · ci-integration · Each 0.2.0 engine change is recorded with its a11y witness in the chunk's re-read record (`escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md`); `aria-checked` is witnessed by `accessibility_states` alone, `accessibility_merged_states` reading `aria-expanded` · `aria-disabled` · `aria-level`.

## Untouched
- §2 strategy (role derivation, the three ARIA state reads, the six upstream reads, the changed set and the tree's refresh): no engine or builder code changed.
- §4 ARIA patterns and roles (HTML-AAM mapping, hidden content, `role="switch"`): no mapping changed.
- §5 engine rules — focusability, disabled-ness, Enter/Space activation, range and select key models, the double-click window: re-read only, no rule moved.
- §7 screen-reader rules (names, labels, `author_id`, the snapshot's readings, the masked value, the actionable-key rule): no symbol changed.
- §8 cognitive (the flight booker's `aria-invalid` dates): not reached.
- The stand contrast tables' row counts (18 control rows, 14 `TEXT_ROWS`, 19 `STATE_ROWS`) and the accent's ratio: unedited.
- The CI a11y job's `ci.yml` properties — slow tier, `needs`, `workspace-test` cache restored with `save-if: false`, failure-only log upload, no required status check: `ci.yml` byte-identical.
- Statements that `stand_contrast`, `stand_keyboard` and `stand_accessibility_ids` run in the `a11y` leg (body §3 contrast harness line, visual-design-verification's matching line): still true.
- Dated per-job readings of earlier CI runs (a11y 6 · 43): true as dated.
- Absence rows naming "the 17 listed s06 files", "86 files of slice s04" or "16 files of slice s08": slice counts, not the a11y leg.
- The `.andromeda/residuals.md` owners (windowed AT witness at the adapter, HTML's focus fixup, native checkbox `toggled`, the click-reaction reading): none moved.
- The key file `registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md` (accessibility checks run in CI as the `a11y` job): still true.
- §10 SLO invariants, §11 anti-patterns, §12 decisions log: no recorded intent, not reached.
