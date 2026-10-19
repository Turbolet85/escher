# P5 review — 2026-10-19-shared-fixtures · Shared fixtures

Plan: `escher-0.2.0/chunks/2026-10-19-shared-fixtures/plan.md` (8 steps · 24 gate entries · 10 criteria) · research:
`escher-0.2.0/chunks/2026-10-19-shared-fixtures/research.md` · scope: `escher-0.2.0/chunks/2026-10-19-shared-fixtures/scope.md`

## Goal
One new shared module, `tests/blitz-tests/tests/fixtures/mod.rs`, states once what checks now repeat file by file:
- `boot(app, incremental)`, the stand-options harness boot of a fixture page;
- `hold(label, app, incremental)`, a fixture page held in a driver session;
- three shared pages: `password_labelled`, `password_named` and `covered`;
- `HeldNet`, one in-process held network stand-in.

Nineteen checks read the module. Each asserts what it asserted, under the same test name, in both layout modes. The
module is the base Visual lint, Load wait, Select by id and Large screen in parts prove on.

## Your two P4 answers
- Module: «Third module fixtures/ (Recommended)».
- Agent-run: «Existing selections (Recommended)». No script change. Test-plan §3's «NOT YET MEASURED — test-data
  bootstrap» closes at the wrap on the measured reading: `boot` compiles the fixtures into each target, and nothing is
  seeded.

Neither fork widened a boundary or turned direction, so no boundary-fork file was owed.

## Top acceptance criteria
- `cargo test -p blitz-tests` green, and every target's test listing byte-equal to the pre-edit record.
- One escher `NetProvider` stand-in. Upstream's `render_blocking_stylesheet` and `link_rel_attribute` stay untouched.
- `stand_settle` still reads `NotSettled { busy: Loads }` while a load is held. This holds by construction: `HeldNet`
  keeps the default `is_noop` = false, so the harness wraps it in its load counter.
- The password checks still read `MASKED_VALUE` with 0 typed-text occurrences.
- The shared module has no print, env, file, socket or `blitz_net` site. The blitz-tests env-read set is unchanged.
- `agent-run boot` reads ready. `run stand` reads 178 · 0 · 7, and `run font_face_sniffed_source` reads 2.
- The a11y leg reads 6 · 6 · 4 · 8 · 5 · 14. `ci-leg.sh fast` and `doc` are green, and the workspace tally is
  199 · 1045 · 0 · 12. CI on the pushed sha is green.

## Touchpoints
- New: `tests/blitz-tests/tests/fixtures/mod.rs`.
- Modified (19): `stand_settle`, `font_face_sniffed_source`, `stand_boot`, `stand_act_diff`, `stand_diff`,
  `stand_snapshot_text`, `stand_mcp_masked`, `stand_session_recording`, `stand_act_spans`, `stand_mcp_tools`,
  `stand_act_filled`, `stand_act_screenshot`, `stand_act_obstructed`, `stand_act_scroll`, `stand_act_transformed`,
  `stand_act_timer`, `stand_replay`, `stand_keyboard`, `stand_snapshot_state`.
- Untouched: `common/mod.rs`, `session_common/mod.rs`, `examples/`, `scripts/`, manifests, and upstream's two tests.

## Test commands (24 entries)
- Unit: `cargo test -p blitz-tests`.
- The before/after test-name listing.
- The off-unix check of blitz-tests' tests.
- Ten probes: the stand-in files, the page files, the helpers, the `from_vdom` count, the readers, silence, the
  census, env reads, the tracked-change guard and the untracked-file guard.
- `agent-run boot`, `run stand` and `run font_face_sniffed_source`.
- The `ci-leg.sh` a11y, fast and doc legs, plus the workspace tally.
- Four operator entries: hygiene, push, CI read and the per-OS tallies.

18 entries are baselined on the untouched tree. Entries 4–10 and 2 read red, as predicted, each with a must-pass
reading on a scratch clone. Entries 11–13 read green, each with a must-fail control.
Controls: `.andromeda/runs/2026-10-19T22-00-26-phase/p5-controls.md`.

## Concretization preview
None. No capability is claimed: every unclaimed cap (v020-05 · 07 · 08 · 09 · 10 · 11) names a fixture case or
surface that a later entry builds. This chunk is the infrastructure under them.

## CI read at the take-up
CI#39331047864 on 5174f92a7335, the last wrap: green, 17/17 checks.

## Beyond the plan
none
