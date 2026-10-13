# Scope — 2026-10-13-stand-contrast-harness

**Working entry (escher-0.1.0/working-route.md:88, verbatim):** Stand contrast harness — text/background pair of every
stand control measured, per-pair result recorded (per a11y-plan §6)

**Epoch:** Epoch 6 — Polish & ship. **Freight on the entry:** none (`route.py pins` lists no block on line 88).

## What it builds
- A contrast harness: given an element of a booted stand document, it reads the element's text colour and the
  background that text is painted on, and computes their WCAG 2.x contrast ratio (relative luminance, `(L1 + 0.05) /
  (L2 + 0.05)`) — the measurement a11y-plan §3 records as absent ("Contrast verification harness: contrast checks are
  observed absent — see §6") and §6 records as NOT YET MEASURED ("design-token contrast pairs").
- The measurement taken on every stand control: one text/background pair per control, measured on the stand as it
  boots headlessly (`seven_guis::stand::boot`, pinned 800 × 600 · scale 1 · `ColorScheme::Light` · bundled font ·
  offline).
- A per-pair result recorded: for each pair, the control's stable id, the two colours, the ratio, and the result
  against SC 1.4.3.

## Boundaries
- "Every stand control" is the stand's own stated control set, `tests/blitz-tests/tests/common/mod.rs`
  `controls(task)` over `LeanTask::ALL` (counter, flight booker, timer, CRUD — 18 rows, the shell's `back-btn` once per
  task); the three non-lean tasks (temperature converter, circle drawer, cells) boot through no `stand` entry point and
  are out (their keys are carried on "Stand requirement sweep", working-route.md:96). Verified: `common/mod.rs:16-45`
  holds the 18 rows, `stand.rs:24` the four tasks. The CRUD list's rows (`crud-person-{id}`, role `option`, clickable)
  are in neither `controls` nor `rendered`; whether they enter the measured set is a P4 decision (research.md, Open
  questions).
- A control that paints no text — the timer's range input `timer-duration` (role Slider in `controls`, `timer.rs:80-86`)
  — has no text/background pair; it is recorded as such, not measured (a non-text contrast reading is SC 1.4.11, outside
  the tier). Verified against the markup.
- The result per pair names the threshold SC 1.4.3 sets for it — 4.5:1, or 3:1 for large text (at least 18 pt, or
  14 pt bold) — so the record carries the text's computed font size and weight; a control that reads not enabled is
  "part of an inactive user interface component", which SC 1.4.3 exempts, and its pair is recorded with that exemption
  rather than a pass or fail. Verified: the stand sheets declare control text at 13–16 px with weight 400–600, none
  large by either rule (research.md, Files inspected); three controls carry `disabled` at boot — `flight-return-date`
  (`disabled: !is_return`, one-way at boot), `crud-update` and `crud-delete` (`disabled: !has_selection`, nothing
  selected at boot) — and the snapshot's `enabled` reads `Some(false)` on the attribute's presence
  (`packages/dioxus-native-dom/src/snapshot.rs:34-52`).
- This chunk measures and records; it does not gate. Asserting SC 1.4.3 on every stand control and a CI leg that gates
  merges on it are "Stand a11y assertions" (working-route.md:94), and a stand colour that fails is not re-coloured here.
  [inferred] hypothesis, re-derived from the declared hex by the WCAG formula (scratch computation, not a measurement):
  `#ffffff` on `#4a6cf7` reads 4.394:1, under 4.5:1 — the pair of five controls at boot (`counter-increment`,
  `flight-one-way` as the active toggle, `flight-book`, `timer-reset`, `crud-create`); the harness's reading decides it.
- The pair is measured at the control's state as booted. Research lists the colour-changing states the stand states:
  hover and active on the primary buttons and `back-btn`, the flight booker's pressed toggle (`type-btn-active`), an
  invalid date (`#c53030` on `#fff5f5`), the disabled date and buttons, and a selected CRUD row (`#ffffff` on `#4a6cf7`);
  which of them beyond the boot state is measured here is a P4 decision (research.md, Open questions).
- Out: dark colour scheme (the stand pins Light), text outside the controls (titles, labels, value displays — SC 1.4.3
  covers them, the entry names controls), focus-ring visibility (SC 2.4.7, outside the tier), non-text contrast
  (SC 1.4.11), the windowed binary.

## Surfaces and contracts it touches
- The engine's computed styles (`color`, `background-color` up the ancestor chain to the canvas) through blitz-dom,
  read-only; or the painted raster through `Harness::screenshot_rect` — which source the measurement reads is a P4
  decision. Verified grounding: paint strokes glyphs in the computed `color` of their brush node, an input's value
  included (`packages/blitz-paint/src/text.rs:156-167, 616-638`, `packages/blitz-dom/src/layout/construct.rs:875-910`), fills a box
  with its computed `background-color` (`packages/blitz-paint/src/render/background.rs:334-336`) and the canvas with the
  root's, else body's, background (`packages/blitz-paint/src/render.rs:193-219`); no control's ancestor chain on the
  stand paints a gradient, image, opacity or filter (research.md, Sweeps).
- blitz-paint already holds a WCAG ratio helper matching Chrome's `GetContrastRatio` (`packages/blitz-paint/src/color.rs:29`,
  `contrast_ratio`, `pub(crate)`, not feature-gated, over the `color` crate's `relative_luminance`); its four call sites
  are all inside blitz-paint (graph trace); whether the harness reuses it (a visibility change in an engine crate) or
  states its own is a P4 decision. Verified.
- The harness's home — `blitz-test-harness` (beside `inspect` and `screenshot`) or the stand checks' shared
  `tests/common/mod.rs` — and the check that runs it (`tests/blitz-tests/tests/stand_*.rs`, both layout modes) are P4's.
- The a11y-plan §3 and §6 readings ("observed absent", "NOT YET MEASURED") are what the wrap amends with what this
  chunk measures.

## Directives
- No mutation runs — the mutation tier stays unstarted; pipeline tool output is redirected to a file and read whole,
  never piped into `head`/`tail`/`grep`/`cut`/`sed`/`awk` (the operator relaying the founder, inputs#I1). Verified
  against the snapshot.

## CI read at Setup (5a)
- `198bf85cb674` (the last wrap's flip commit, HEAD) — green · checks 16/16 · wall 904 s · CI#38432770340
  completed/success. No red and no `not green` to fold.

## Verification matrix
- No capability of escher-0.1.0's matrix names contrast; the one unclaimed id, v010-15 (Cold-agent test), is owned by
  "Cold-agent test" (working-route.md:100). This chunk is expected to claim none. Verified at P3 against the matrix's
  17 titles; P4 re-reads the pool through `matrix.py show --unclaimed`.
