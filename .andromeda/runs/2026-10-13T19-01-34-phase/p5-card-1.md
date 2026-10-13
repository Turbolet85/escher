# P5 review — 2026-10-13-range-input-interaction

**Chunk:** Range input interaction (working-route.md:90) · escher-0.1.0 · Epoch 6 — Polish & ship
**Artifacts:** `escher-0.1.0/chunks/2026-10-13-range-input-interaction/` — `scope.md` · `research.md` · `plan.md` (523 lines, about 390 outside the gate fence) · `inputs/` (I1–I3)

## Goal
The engine gets a minimal range interaction model, for every document with an `input type="range"`. This is the
founder's ruling «Пусть будет а» (2026-10-07), shaped by his three P4 answers (2026-10-13, relayed verbatim by the
overseer as his chosen option labels; the English wording is the overseer's — inputs#I2, inputs#I3):
- **Q1 «Отдельное текущее значение»:** the moved value is the control's own current value, held apart from its
  `value` attribute.
- **Q2 «Стрелки + Home/End (Рекомендую)»:** right and up step the value up, left and down step it down, Home goes to
  `min` and End to `max`.
- **Q3 «Клик даёт фокус (Рекомендую)»:** a click sets the value by its position along the track and focuses the
  control.

Every change fires one `input` event, which reaches the app's `oninput`. The driver, the app's markup, paint and the
accessibility tree do not change.

## Top acceptance criteria (17 in the plan)
- The keys, the click and the `input` event behave as above in both layout modes. `range_input_interaction` checks
  this row by row.
- Degenerate markup never panics and always reads a finite value inside `[min, max]` on the step grid. The rows are
  `step="0"`, a negative step, `step="any"`, `min > max`, `value="abc"`, `value="1e308"` and `min="NaN"`.
- `stand_act_range` is rewritten, not deleted. On the Timer, through the driver, both layout modes read:
  - a click focuses the slider at `15`;
  - the arrow, End and Home steps read `15.5`, `16`, `30` and `0`, each display following;
  - a click after a step brings the value back to `15`.
- `stand_act_ids`' Timer row now names `timer-duration` as changing at boot, because the click focuses it. The 15
  standing checks nearest the change pass unchanged.
- The driver schema, stand markup, paint, UA sheet, the bridge's converter and the lock file are unchanged (the
  preservation and `Cargo.lock` probes).

## Touchpoints
- **New:**
  - `packages/blitz-dom/src/node/range.rs` — `RangeInputData` and the range state's value rules;
  - `tests/blitz-tests/tests/range_input_interaction.rs`.
- **Modified (blitz-dom):** `node/mod.rs`, `node/element.rs` (the new variant), `layout/construct.rs` (seed once, no
  early return), `mutator.rs` (a value/min/max/step write re-seeds), `events/keyboard.rs`, `events/pointer.rs` and
  `form.rs`.
- **Modified (elsewhere):** dioxus-native-dom `snapshot.rs`, `stand_act_range.rs` and `stand_act_ids.rs`.
- **Q1's stated costs, all planned:**
  - a public type change: `SpecialElementData::RangeInput`, a source break for an external exhaustive match, though
    none exists in this workspace;
  - the construction seed;
  - the re-seed on an attribute write;
  - a new snapshot reader;
  - a larger upstream merge surface: seven upstream files, each edited additively.

## Leans for you to confirm
- **Form submission reads the current value** (`form.rs`), per Q1. Left on the attribute, a moved range would submit
  its default. This was amended into the scope at validation-1 as intent-incomplete.
- **A `value` attribute write re-seeds the current value**, per inputs#I3 and the text input's precedent. This differs
  from the spec's dirty flag, which ignores attribute writes after a user move, but without it the Timer's re-render
  could not move the slider.
- **The value is set at pointerdown and the focus at click.** A label's synthetic click therefore focuses the slider
  without moving its value.

## Test commands (18 entries)
- **Unit:** `cargo test -p blitz-dom --locked` and `cargo test -p dioxus-native-dom --locked`.
- **Integration:**
  - `range_input_interaction`, and `stand_act_range` with `stand_act_ids`, each under `-p blitz-tests` and under
    `--workspace`;
  - 15 standing targets.
- **Five probes:** the lock, `range.rs` silence, the added-line census, the checks' silence, and preservation.
- **Agent-run and local gates:** `agent-run.sh boot` and `run stand`; `ci-leg.sh fast` and `doc`.
- **Operator:** hygiene, push, and the CI read.

Baselines on the untouched tree (12 new entries, 2026-10-13T20:17Z):
- **Units:** green, at 89 and 67 tests.
- **The new target's two entries:** red. Cargo reports no target named `range_input_interaction`, which step 9 writes.
- **`stand_act_range`/`stand_act_ids`:** green on the old reading; steps 5–6 turn them red by design.
- **Standing targets:** green.
- **`run stand`:** 141 · 0 · 7.
- **The five probes:** green. Each has must-fail and must-pass controls over a scratch clone at 97ce760d, recorded in
  `.andromeda/runs/2026-10-13T19-01-34-phase/controls-p5.md`.

No mutation control is planned or run (inputs#I1).

## Concretization preview
None: the chunk claims no capability. The only unclaimed id, v010-15 (Cold-agent test), is owned by "Cold-agent test"
(working-route.md:100). The founder's ruling added no ledger entry.

## Validation
- **Mechanical checks:** all pass. The fence parses 18 entries, planlint reports 0 hits, research's lists parse with
  12 items, and the inputs show 0 unparsed and 0 uncited.
- **Size:** about 390 lines of prose, at the top of the ~400 guide. Q1's write set is the reason.
- **Validation-1:** aligned once the scope was amended (intent-incomplete: Q1's reach to the snapshot and form).
- **Expected amendments (wrap):** security-plan §Input Validation, which the playbook escalates as a Boundary widening
  (:36). The founder's own word is relayed verbatim, so the wrap halts on it once, as the answer card. Also
  architecture's fifth direct engine change (:64), test-plan, obs-plan §3 and a11y-plan §5 and §7.

## Beyond the plan
| # | Proposal | Owner / place |
|---|---|---|
| 1 | The merge surface of this engine change for every document (playbook :66 has it "owned by a CARRY on the upstream-sync entry"): `node/element.rs`, `node/mod.rs`, `layout/construct.rs`, `mutator.rs`, `events/keyboard.rs`, `events/pointer.rs`, `form.rs`, against upstream `main` at `434e0661`. No markerless upstream-sync entry remains, because the last one, 2026-10-13-upstream-sync-polish-and-ship, is complete. | a CARRY on "Quality gates" (working-route.md:98), the release-readiness entry ahead of ship — written by this chunk's wrap |
