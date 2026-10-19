# Counts — 2026-10-19-shared-fixtures

Read from the /implement gate run `.andromeda/runs/2026-10-19T22-26-15-implement/` (trail
`gate-2026-10-19-shared-fixtures.json`), one firing of the block (its agent-run boot event stamped
2026-10-19T22:33:21Z), on the working tree over base `5174f92a7335`.

## blitz-tests (entry 1, `cargo test -p blitz-tests --locked`)
- 139 result lines · 586 passed · 0 failed · 12 ignored — equal to the untouched tree's baseline.

## Test names (entry 2, the listing against `evidence/blitz-tests-before.txt`)
- `cmp` exit 0: the listing is byte-identical to the record step 1 wrote on the untouched tree.
- The record: 874 lines · 139 per-target summary lines (`N tests, M benchmarks`) · 598 `: test` lines — 139 targets,
  598 tests listed, unchanged. `fixtures/mod.rs` is no target (no `Running tests/fixtures…` line).

## Agent-run (entries 14–16)
- `boot`: outcome `ready`, cargo exit 0, head `5174f92a7335`.
- `run stand`: passed 178 · failed 0 · ignored 7 · cargo exit 0 · outcome passed — as predicted, unmoved.
- `run font_face_sniffed_source`: passed 2 · failed 0 · ignored 0 · outcome passed — as predicted.

## a11y leg (entry 17)
- accessibility_hidden 6 · accessibility_roles 6 · focusability_updates 4 · stand_accessibility_ids 8 ·
  stand_contrast 5 · stand_keyboard 14 = 43 tests, 0 failed — as predicted, unmoved.

## Workspace (entries 18–19, `ci-leg.sh fast`, `target/ci-logs/test.log` fresh)
- lines 199 · passed 1045 · failed 0 · ignored 12 — as predicted, unmoved.

## Census probes (entries 4–13)
- `NetProvider for`: `fixtures/mod.rs` and upstream's `link_rel_attribute.rs` · `render_blocking_stylesheet.rs` alone.
- File-private `password_fixture` / `covered_fixture` left: `stand_act_filled`, `stand_act_obstructed`,
  `stand_act_screenshot` (one reader each).
- File-private fixture boot or hold: 0. `Harness::from_vdom`: 7 lines. `mod fixtures;` readers: 19.
- Silence census: 0 print, sink, env, file or socket sites in `fixtures/mod.rs`; 4 over the 19 files touched with
  the module (none added). The blitz-tests env-read set unchanged.
- No tracked file changed outside the nineteen; no untracked code file but `fixtures/mod.rs`.
