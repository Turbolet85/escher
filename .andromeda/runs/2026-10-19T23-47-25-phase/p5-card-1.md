# P5 review — 2026-10-19-visual-lint · Visual lint (v020-05)

Plan: escher-0.2.0/chunks/2026-10-19-visual-lint/plan.md · research: escher-0.2.0/chunks/2026-10-19-visual-lint/research.md ·
scope: escher-0.2.0/chunks/2026-10-19-visual-lint/scope.md · forks: .andromeda/runs/2026-10-19T23-47-25-phase/founder-forks.md

## Goal
One lint reads the current screen of a held instance and names each element that overlaps another, is clipped, has
truncated text, lies off-screen, has zero size or fails text contrast. Each finding gives the stable id, one of six
closed defect words and the measured numbers. It is read-only, like `screenshot`. Contrast is read exactly as
`stand_contrast` reads it.

**F1, answered by you as delegate:** «In process only (Most conservative)», PROVISIONAL (inputs#I2). This chunk builds
`Session::lint` and the proofs, and nothing crosses a carrier. The "one command" half of v020-05 is gated on the
founder's own F1 word; its carriage is step 12, built only after his word.

## Top acceptance criteria
- The planted page names exactly 7 findings, in document order, with their reasons:
  - `fx-overlap-a` overlap, with `fx-overlap-b`
  - `fx-clipped` clipped
  - `fx-paint-clipped` clipped, through a `contain: paint` box
  - `fx-truncated` truncated
  - `fx-offscreen` off-screen
  - `fx-zero` zero-size
  - `fx-low-contrast` contrast, its ratio equal to `text_contrast`'s and below 4.5

  The 6 negatives (inert, hidden, disabled, faded, long input value, wrapped text) read nothing. Both layout modes.
- The four lean tasks at boot read 0 findings. This is a prediction from the take-up's measurements: 0 overlapping
  pairs, and the 5 no-area nodes hold no content. Truncation and contrast outside `stand_contrast`'s rows are
  unmeasured until implement. A finding there stops implement and comes to you.
- The lint changes nothing: no change marked, focus kept, time not moved, the next snapshot equal.
- After twelve Creates it names the last CRUD row cut by `crud-list`, the row a `click` is refused `off-screen` on.
- It refuses `fonts-unavailable` with no font.
- `BaseDocument::text_extent` reads its three cases in both modes and both builds.
- No span is added (two `info_span!` sites), and the four new files are silent.
- No tracked file outside the in-process six moves, and no new file beyond the four.

## Touchpoints
- New: `packages/blitz-dom/src/text_extent.rs` · `packages/escher-driver/src/lint.rs` ·
  `tests/blitz-tests/tests/text_extent.rs` · `tests/blitz-tests/tests/stand_lint.rs`
- Modified: `packages/blitz-dom/src/lib.rs` · `packages/escher-driver/src/lib.rs`
- Gated (step 12, only on his word option 1): the verb table, command, executor, JSON, MCP, CLI, recording tests,
  session helpers, the span and MCP checks, `cli_commands` / `mcp_server` / `host_log` / `mcp_log`, and the
  cold-agent verb names.

## Test commands (22 entries)
- `cargo test --test text_extent`
- `escher-driver lint` units
- `cargo test --test stand_lint`
- the windows check
- the span and silence censuses, and two git guards (base `e97a7f41b`)
- `agent-run` boot · `run stand` (predicted 183 · 0 · 7) · `run text_extent`
- `ci-leg` a11y / fast / doc, with the tally predicted at 201 lines · 1055 passed
- operator: hygiene, push, CI read, per-OS tallies (control fired on CI#39336988025)
- three gated entries: the founder-word row, the lint verb test, `cli_commands`' lint test

Baselines: 15 written, and every guard was controlled both ways in a scratch clone (`p5-controls.md`).

## Concretization preview — v020-05 (claimed GATED)
- **Current:** On a fixture that plants each of the six defects, one lint command names every planted element by
  its stable id with its measured reason, and on the stand at boot it reports no defect the stand's recorded
  contrast and layout readings do not hold.
- **Proposed:**
  - **The page and its readings.** `tests/blitz-tests/tests/stand_lint.rs`'s planted page holds the six defects
    (overlap; clipped by `overflow` and by `contain: paint`; truncated; off-screen; zero-size; contrast) and six
    negatives. On it the lint names exactly the seven planted findings by stable id, defect word and measured
    reason, and nothing on a negative.
  - **The stand.** On each of the four lean tasks at boot it names 0 findings, in both layout modes.
  - **How it is proven.** Through `Session::lint` now, and through the one command the founder's own F1 word sets,
    whose answer equals `Session::lint`'s on the same screens.
  - **The gated mark.** *gated: the founder's own word on F1 of founder-forks.md, the lint's carriage, answered by
    the operator as delegate «In process only (Most conservative)», PROVISIONAL (inputs#I2)*.
- **Never weakened.** "Every planted element" is sharpened to an exact set. "No defect the recorded readings do not
  hold" is sharpened to 0, the stand's measured state. "One command" stays, gated. The in-process reader never
  verifies the gated half.
- **Route half (step 4).** `BLOCKED-ON: {the block}` is appended to this chunk's frozen working-route line.

## Gated criterion
The block is the founder's own word on F1. Option 1 means step 12 builds the `lint` verb on every carrier, recorded,
and its two tests prove it. Option 3 means the gated wrap releases v020-05 (`matrix.py unclaim`, a dated note) and
the crossing becomes a route entry you mint. Option 2 or option 4 means a `/andromeda-phase` revision.

## Beyond the plan
| # | Proposal | Owner |
|---|---|---|
| 1 | F1 and F2, answered by the operator as delegate, stay PROVISIONAL for the founder's own word | the wrap that closes Epoch 4 lists them, with the carried `:focus-within` take, for the founder's boundary batch |
