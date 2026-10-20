# P5 review — 2026-10-20-select-by-id · Select by id

Plan: escher-0.2.0/chunks/2026-10-20-select-by-id/plan.md (17 steps, 23 gate entries, 11 acceptance criteria) ·
research: escher-0.2.0/chunks/2026-10-20-select-by-id/research.md · scope: escher-0.2.0/chunks/2026-10-20-select-by-id/scope.md

## Goal
An agent drives a single-choice `select` by its stable id and reads it in the snapshot with role, name and chosen value,
its Tab stop unchanged (v020-08). Measured first (evidence/select-probe-readings.txt): today a select lays out 0×0, so
the snapshot leaves it out and every verb on its id is refused `not-found`; it holds no options in the tree and takes no
key.

Built on the founder forks the operator answered as delegate (inputs#I2; .andromeda/runs/2026-10-20T03-46-58-phase/founder-forks.md),
every answer an engine behaviour change for every document, PROVISIONAL for the founder's word at the Epoch 4 batch:
- F1 keys + typeahead — arrows, Home, End and a typed prefix on a focused select; a click focuses it. An agent chooses
  with `type --id <select> --text <label>`. No new verb, argument, key name or exit.
- F2 the engine shows the chosen option — `select` an inline block; `select:not([multiple]) > option:checked` shown.
- F3 options read as the select's children, named by their labels, the chosen one `selected`.
- F4 `input` then `change` on each choice (a new `Change` event kind); a `<label>` binds every labelable element.

The choice is the `CHECKED` state on the chosen option, written as the radio group writes it (`snapshot_node_and`):
`:checked` restyles, the changed set marks, and no `SpecialElementData` variant is added.

## Top acceptance criteria
- v020-08, in `stand_act_select`, both layout modes: `fx-pick` reads `ComboBox` · `Fruit` · `b`; `type --id fx-pick
  --text Cherry` is accepted, settled, its diff names `value`, and the select reads `c`; Tab walks fx-before → fx-pick →
  fx-named → fx-after, the disabled select skipped, unchanged across the choice.
- The engine model in `select_interaction`: box, seeding, key and typeahead tables, one `input` + one `change` per
  change and none otherwise, disabled takes nothing, a re-render re-seeds.
- `<label>` names a `select` and a `textarea` (`accessibility_label`).
- No verb, argument, field, cause, exit or span field added (describe row `[11, 13, 12, 3, 8, 4, 15]` unmoved; 2 span
  sites); no manifest, shared test module or stand file changes; blitz-dom gains no wall read.

## Touchpoints
New: blitz-dom `node/select.rs` · `select_interaction.rs` · `stand_act_select.rs`. Modified: blitz-dom `node/{mod,element}.rs`,
`document.rs`, `events/{keyboard,pointer}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `assets/default.css`;
blitz-traits `events.rs`; blitz-vibey-script `runtime.rs`; dioxus-native-dom `dioxus_document.rs`, `snapshot.rs`;
escher-driver `schema.rs`, `schema_tests.rs`; blitz-tests `accessibility_label.rs`.

## Test commands
Units (blitz-dom, dioxus-native-dom, escher-driver) · `select_interaction` · `stand_act_select` ·
`accessibility_label` + `accessibility_roles` · six probes (clock, select.rs silence, checks' silence, span sites,
no manifest/shared/stand change, write-set guard, untracked guard) · `agent-run.sh boot` / `run stand` · a11y leg ·
`ci-leg.sh fast` + tally · `ci-leg.sh doc` · operator: hygiene, push, CI read, per-OS tallies. Baselines on the
untouched tree recorded in each entry; controls in .andromeda/runs/2026-10-20T03-46-58-phase/p5-controls.md.

## Concretization preview — v020-08 (method integration)
- Current: «On a fixture holding a select, an agent chooses an option by the control's stable id through the driver,
  and the snapshot reads the select with its role, its name and the chosen value, the focus order unchanged.»
- Proposed: «On the in-file fixture page of `stand_act_select`, held in a driver session in both layout modes: the
  snapshot reads the select `fx-pick` with role `ComboBox`, name `Fruit` (from its `<label>`) and value `b` at boot;
  `type --id fx-pick --text Cherry` through `Session::run` is accepted and settled, its diff names `fx-pick` with
  `value` in `differs`, and the next snapshot reads its value `c`; the Tab walk reads `fx-before` → `fx-pick` →
  `fx-named` → `fx-after`, the disabled select skipped, before and after the choice. Gate: `cargo test -p blitz-tests
  --locked --test stand_act_select 2>&1` reads 0 failed.» — the outcome unchanged, every clause made a reading.

## Notes for the review
- HEAD's CI (CI#39365557179) read in progress at take-up and green 17/17 at P4 (the per-OS tally control).
- Stated limits the wrap records: a `multiple` select and an `optgroup`'s options; no popup; no engine `change` for a
  checkbox, radio or text input; an option with no author key reads in `unkeyed_actionable`.
- The P3 probe was a throwaway test file, run and removed (evidence/select-probe.rs.txt keeps it).

## Beyond the plan
none
