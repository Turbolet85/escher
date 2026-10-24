# Scope — 2026-10-24-a11y-re-read

**Working entry (escher-0.2.0/working-route.md:51, verbatim):**
A11y re-read — stand Tab order, Enter/Space activation and contrast re-asserted after 0.2.0's engine changes; new a11y checks in the `a11y` leg (a11y-plan §9)

**Epoch:** Epoch 5 — Polish & ship · **Version:** escher-0.2.0 · **Freight:** none (no annotation on the entry; `route.py pins` lists blocks only on line 53).

## What the chunk builds

- **Stand Tab order re-asserted.** `stand_keyboard`'s boot Tab sequence per lean task (`common::tab_order`) and its
  enabled-state sequences (`common::ENABLED_TAB_ORDER`) read against the stand as 0.2.0 leaves it, in both layout
  modes; a sequence 0.2.0 moved is re-recorded with the change named, never silently re-baselined.
- **Enter/Space activation re-asserted.** `stand_keyboard`'s Enter and Space on each stand button read again after
  0.2.0's engine changes; a stand control 0.2.0 made keyboard-operable that no row asserts gets its row.
  [premise-corrected: the stand holds no `select` — the flight booker's controls are `flight-one-way` · `flight-return`
  buttons and two date text inputs (tests/blitz-tests/tests/common/mod.rs:23-30, `git grep select` over
  examples/seven_guis/src/ finds only Home's `on_select` handler), and no stand markup carries an attribute 0.2.0
  newly reads (`git grep -E 'inert|aria-labelledby|aria-describedby|aria-checked|aria-expanded|aria-disabled|transform|contain:'`
  over examples/seven_guis/src/: 0 hits), its `disabled` written on `button` and `input` alone — so no stand control
  gained a keyboard model in 0.2.0 and no stand row is owed; the select's keyboard model is witnessed on fixtures,
  `select_interaction` (7 tests) and `stand_act_select` (5 tests)]
- **Contrast re-asserted.** `stand_contrast`'s control rows, CRUD list rows, the text outside the controls and the
  colour states it reaches (`TEXT_ROWS`, `STATE_ROWS`) read again after 0.2.0; SC 1.4.3 held on every non-exempt
  stand row.
- **New a11y checks in the `a11y` leg (a11y-plan §9).** The leg (`.github/scripts/ci-leg.sh:39`) runs six files
  today — `accessibility_hidden` · `accessibility_roles` · `focusability_updates` · `stand_keyboard` ·
  `stand_contrast` · `stand_accessibility_ids` (6 · 43 tests at HEAD, the leg run green this take-up). 0.2.0 added
  nine a11y-bearing blitz-tests files outside it (`git diff --diff-filter=A` from 2026-10-16-handler-splits-and-readme's
  parent to HEAD over tests/blitz-tests/tests/): `accessibility_active_descendant_unread` (1), `accessibility_description`
  (3), `accessibility_label` (13), `accessibility_merged_states` (3), `focus_redraw` (4), `focus_within` (7),
  `inert_subtree` (4), `select_interaction` (7), `stand_act_select` (5); `accessibility_states` (5) and
  `accessibility_names` (5) predate 0.2.0 and ride the workspace `test` leg too. Which of them join the leg is the
  plan's fork (research.md, Open questions). The leg is widened to the ones that witness an a11y requirement, its pin
  (`test_ci_workflows.py`'s `LegScriptTest` a11y tuple, an `assertIn` per word, not an ordered list) moving with it; the
  job (`ci.yml`'s "Accessibility (a11y) tests") keeps its tier and takes no required status check. `stand_requirement_sweep`'s
  leg check reads only `stand-repeat`'s names in `ci-leg.sh`, so the a11y line moves no row of it.

## 0.2.0's engine changes the re-read covers

Read off the 0.2.0 master records and CLAUDE.md §Modules, each record present in master-route.md. Closed at P3: none
moved a stated stand table — tests/blitz-tests/tests/common/mod.rs (`tab_order`, `ENABLED_TAB_ORDER`, `controls`) has
a 0-line diff across 0.2.0, `stand_contrast` and `stand_accessibility_ids` changed only keyboard-types 0.8's
`Key::Named` spelling, and `stand_keyboard` grew its fixture rows for the `inert` and `disabled` readings
(+137 / −63); `stand_keyboard` 14 · `stand_contrast` 5 · `stand_accessibility_ids` 8 pass at HEAD in both layout modes:
- 2026-10-18-engine-debts — `inert` (skipped by Tab, passed through by a hit, out of the tree), `disabled` read on a
  form control alone (`ElementData::is_disabled`) for focusability and the activation keys, `aria-checked` on the five
  checkable roles, `contain: paint` hit stop, transformed client rect, the password click caret.
- 2026-10-18-one-virtual-clock — the double-click window on the document's clock.
- 2026-10-20-select-by-id — the select's keyboard and typeahead model, a click focusing it.
- 2026-10-17-upstream-sync-engine-debts — upstream's `aria-labelledby` · `aria-describedby` · `aria-description` ·
  `aria-expanded` · `aria-disabled` · `aria-level` reads.
- 2026-10-24-upstream-sync-polish-and-ship — ten upstream commits cherry-picked at `f9cbb2756` (engine code).

## Boundaries

- A re-read, not an engine change: a defect the re-read finds that needs engine behaviour changed for every document is
  a fork for the founder (CLAUDE.md Session Learnings, 2026-10-18), never folded silently.
- No carrier, verb, key name, field, log field, span, listener, env read or crate added.
- Out: the platform adapter and its windowed AT witness (escher-0.2.0/requirements.md:42; residuals line 14).
  Not absorbed: residuals lines 32 (inert focus fixup) and 33 (native checkedness unread by the builder), both
  `target: next` and each an engine change for every document (.andromeda/residuals.md:32-33, read at P1).
- No verification-matrix capability names this entry — `matrix.py show --unclaimed` lists v020-10 and v020-11
  alone, and working-route.md names them on lines 53 and 55.

## CI read at take-up (Setup 5a)

- CI#40125511019 on `db585b7116b5` (the last wrap's flip, HEAD) — `in progress`, checks 17/17 seen, the oldest still
  running `Test (windows)` at 555 s: **verdict not yet available**, not read as green and not folded.
- Re-read at P3 (`ci.py conclusion --sha db585b7116b5…`): CI#40125511019 **green**, checks 17/17, wall 1037 s,
  "Accessibility (a11y) tests" and "Stand checks repeated" among them — nothing to disposition.
