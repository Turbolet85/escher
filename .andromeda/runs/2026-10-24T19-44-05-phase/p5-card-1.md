# P5 review — 2026-10-24-a11y-re-read · "A11y re-read"

Plan: `escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md` (437 lines, 275 of prose, 21 gate entries, 10 criteria).

## Goal
Re-read the stand's tier-0 a11y readings (SC 2.1.1 · 1.4.3 · 2.4.3) after 0.2.0's engine changes, and widen the
`a11y` CI leg to the fourteen files the founder chose. His word is «Плюс проверки дерева (Рекомендую)» (2026-10-24,
by question dialog in the overseer session, relayed verbatim by the overseer, snapshotted as inputs#I1). P3 closed the
re-read: no stand table moved across 0.2.0, no stand markup carries an attribute 0.2.0 newly reads, and the stand holds
no select. So no stand row is owed, and the re-read is `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`
reading green inside the widened leg and in `stand-repeat`.

## Touchpoints
- `.github/scripts/ci-leg.sh:39` — the `a11y` arm gains eight `--test` words (`accessibility_states`,
  `accessibility_label`, `accessibility_description`, `accessibility_merged_states`,
  `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`). It stays one
  `cargo test --workspace --locked` line, and nothing else in the file moves.
- `.github/scripts/test_ci_workflows.py:298-299` — the a11y tuple names the same fourteen words. No test case is
  added (33 tests, `ci-scripts` 100).
- No engine, stand, table, `ci.yml`, manifest, lock, dependency, carrier, field, log site, span or env read changes.

## Top acceptance criteria
- The leg names the fourteen words in order under `--locked` (the names probe). `bash .github/scripts/ci-leg.sh a11y`
  reads 14 result lines and 0 failed. Predicted 84 passed: 43 for the six, plus 41 for the eight joining files,
  measured at P4.
- The three stand a11y checks are green in the leg (workspace build), in the per-package run of the fourteen, and in
  three `stand-repeat` rounds. The guard shows every stand table unedited, and nothing is re-baselined.
- `stand-repeat` is untouched: the numstat probe reads `1 1`, `stand_requirement_sweep` is green, and the leg ends
  `3 of 3 rounds green`.
- `ci.yml` is byte-identical (the guard). The job keeps its tier, cache, `needs` and upload, and has no required check.
- CI is green on the pushed sha. The a11y job's tally moves from 6 · 43 · 0 · 0 (CI#40125511019), each moved count
  attributed per file.

## Test commands
The leg and its tally, then the names probe, the numstat probe and the pin's unittest. Then `stand_requirement_sweep`,
the per-package run of the fourteen, the silence census, the `cargo tree` adapter probe, the manifest probe and the
preservation guard. Then agent-run boot / run stand, `stand-repeat`, `fast` with its tally, and `doc`. Last, the
operator pass: hygiene, the push, `ci.py conclusion --wait 1800`, and the per-job tally read.
- Baselines: eight entries are seen green on the last wrap. Of the nine new ones, six read green and three read red,
  each red on the subject not yet built (the six words, an empty numstat, a 6-line log).
- Controls fired as the entries over a scratch clone at db585b711 (`.andromeda/runs/2026-10-24T19-44-05-phase/p5-controls.md`).
  Each guard and probe reads green on steps 1–2 as the plan writes them, and red on a stray edit.

## Checks
- Mechanical set: all pass, re-run in full after the `baseline` writes. planlint's one HIT is check 4 (13), on the
  report-only per-job log read. Its note names the read ci.py never makes and carries its P4 control (CI#40125511019),
  so it is resolved. Snapshot citations: I1 unchanged, cited.
- Validation-1: aligned with scope.md and working-route.md:51. One tension was resolved by the founder's word. The a11y
  extract read a11y-plan §1 as admitting tier-0 witnesses alone, while option (b) takes five SC 4.1.2 tree reads, as
  the leg already runs `accessibility_hidden` and `accessibility_roles`. The word stands, inputs#I1.
- Playbook `escalate` patterns: none applies. A leg's file list widens no boundary, and the fork was answered by the
  founder's own word.

## Capabilities
None claimed. The pool holds v020-10 and v020-11 alone, owned by working-route.md:53 and :55, so there is no
concretization preview.

## Sampler (this take-up was sampled)
- a11y-plan — sampler: E carried 75 · the cascade lost 3 (a11y-scope-summary#1.9, a11y-strategy#1.2, screen-reader-support#4.1)
- architecture — sampler: E carried 90 · the cascade lost 9 (conventions#9.2, cross-cutting-patterns#3.1,
  design-philosophy#3.1, established-decisions#15.2, established-decisions#6.1, established-decisions#9.1,
  project-intent#1.0, project-intent#5.1, stack-and-technologies#1.10)
- design-system — sampler: E carried 15 · the cascade lost 3 (color-palette#5.2, motion#1.1, surface#1.4)
- obs-plan — sampler: E carried 15 · the cascade lost 5 (log-coverage#6.1, pii-scrubbing-compliance#1.1,
  pii-scrubbing-compliance#2.1, span-trace-coverage#3.1, telemetry-strategy#1.1)
- security-plan — sampler: E carried 27 · the cascade lost 6 (api-security#1.0, api-security#2.6,
  logging-monitoring#1.1, threat-model-summary#1.1, threat-model-summary#3.1, threat-model-summary#4.1)
- test-plan — sampler: E carried 54 · the cascade lost 2 (e2e-test-strategy#1.1, test-data-fixtures#3.1)
- All 28 lost entities stand in the plan's Provenance as `(sampler) {master}/{category}#{n}`, each with its home: a
  gate, a constraint or a note.

## Expected amendments (wrap), as the plan lists them
- a11y-plan §9 + detail `ci-integration.md:8`: fourteen files, 14 · 84, the founder's word.
- a11y-plan §3 (:53-55).
- a11y-plan detail `keyboard-navigation.md:100` and `visual-design-verification.md:14`.
- test-plan detail `ci-integration.md:8`, `:42` and `:64`.
- test-plan detail `test-scope-summary.md:97`, `:98`, `:100` and `:125`.
- test-plan detail `unit-test-strategy.md:28`: an `assertIn` per word, "in order" retired.
- architecture detail `standard-contracts.md:255` and `existing-scopes.md:102` and `:111`.

## Beyond the plan
none
