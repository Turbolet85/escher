# P5 review — 2026-10-13-stand-contrast-harness · "Stand contrast harness"

Plan: `escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/plan.md` — 10 steps · 17 gate entries · 16 acceptance criteria.
Mechanical checks all pass (gate dry-run parsed 17 · planlint 0 hits · lists parsed · 11 new entries baselined, five
inline guards controlled both ways in a scratch clone · inputs verify clean). Validation-1: aligned with scope.md and
working-route.md:88.

## Goal
`blitz-test-harness` gains a `contrast` module:
- `contrast_ratio`, the WCAG 2.x ratio over the same `color` luminance that blitz-paint's `pub(crate)` helper uses;
- `TextContrast`, which holds the colours, the ratio, the computed size and weight, and the SC 1.4.3 threshold;
- `Harness::text_contrast(node)`, which reads a control's text colour and the background behind it from computed
  styles, up to the canvas by paint's root-else-body rule.

A new stand check, `stand_contrast`, measures all 18 rows of `controls(task)` over the four lean tasks in both layout
modes. Each row records one result: pass, fail, exempt-inactive or no-text. The record is the check's stated table,
asserted row for row and copied into `evidence/pairs.md`.

A second test checks the measurement against the painted pixels. The pixel inside each measured control's left padding
must equal the resolved background, within 1 per channel.

The chunk measures and records only. It gates nothing, re-colours nothing, changes no engine crate, and puts no package
into the lock.

## Top acceptance criteria
- 18 rows, one result each, in `controls()` order; the reading equals the stated table in both layout modes.
- A row reads exempt-inactive exactly when its snapshot node's `enabled` is `Some(false)`. At boot that predicts
  `flight-return-date`, `crud-update` and `crud-delete`.
- The ratio checks against WCAG reference rows: 21.0, 1.0, `#777777` on white 4.48, `#ffffff` on `#4a6cf7` 4.39.
- The painted-background witness holds for every measured row.
- No engine crate moves (preservation guard). No `[[package]]` enters `Cargo.lock`. Audit reads `advisories ok`. No
  print, log, subscriber, env read or `tracing` is added (three census entries).
- `stand_contrast` is green per package and under the workspace build. `run stand`, the `fast` leg and the `doc` leg
  pass. The operator's CI read reads `verdict: green`.

## Touchpoints
New: `packages/blitz-test-harness/src/contrast.rs` · `tests/blitz-tests/tests/stand_contrast.rs` · `evidence/pairs.md`,
`evidence/counts.md`.
Modified: `packages/blitz-test-harness/src/lib.rs` · `packages/blitz-test-harness/Cargo.toml` (+ `color`, workspace) ·
`Cargo.lock` (one dependency line, no package).

## Test commands (by entry)
1. `cargo test -p blitz-test-harness --locked`
2. `cargo test -p blitz-tests --locked --test stand_contrast`
3. `cargo test --workspace --locked --test stand_contrast`
4. The seven standing neighbours.
5–9. Five inline guards: the lock, two censuses, the manifest `tracing`, preservation.
10–11. `agent-run.sh boot`, then `run stand`.
12. Audit.
13. `fast`.
14. `doc`.
15–17. Operator only: hygiene, then push, then the CI read.

No mutation run: the mutation tier stays unstarted (inputs#I1).

## Leans taken without a question (each from an artifact)
- **Only the state each control boots in is measured.** Per the working entry's wording, one text/background pair per
  control.
- **The CRUD rows stay out.** `controls()` is the one stated control set; a second mapping is banned (test-plan §3,
  a11y-plan §7).
- **Colours come from computed styles, with a painted-pixel witness of the background.** Research verified that paint's
  colour sources are computed styles. A raster sample of text colour hits anti-aliased glyph edges.
- **No change to blitz-paint.** Arch's engine-visibility rule; the helper has four call sites, all inside blitz-paint.

## Hypothesis the harness will settle
From the declared hex, five controls read `#ffffff` on `#4a6cf7` = 4.39 and are predicted **fail**: `counter-increment`,
`flight-one-way` (active at boot), `flight-book`, `timer-reset` and `crud-create`. The record states what is measured.

## Concretization preview
None. The one unclaimed capability, v010-15 (Cold-agent test), belongs to "Cold-agent test" (working-route.md:100), and
this chunk claims no capability.

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | The colour-changing states this chunk does not measure: hover and active on the primary buttons and `back-btn`, the flight booker's toggle in its other position, an invalid date (`#c53030` on `#fff5f5`), and the date and buttons in their non-boot enabled state. Each is SC 1.4.3 text. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
| 2 | The CRUD list rows (`crud-person-{id}`, role `option`) are outside `controls()` and not measured. A selected row is predicted `#ffffff` on `#4a6cf7` = 4.39, which fails. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
| 3 | Re-colouring every pair this chunk records as `fail`. SC 1.4.3 cannot be asserted green on the stand until they pass. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
| 4 | Text outside the controls (task titles, labels, value displays) is SC 1.4.3 text too, and the entry names controls only. | "Stand a11y assertions" (working-route.md:94) — a `CARRY:` at this chunk's wrap |
