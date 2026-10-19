# Implement outcome — 2026-10-19-shared-fixtures

**Outcome: green** (smoke skipped — no boot-path or UI-surface change: test code only). Gates green in one firing:
`entries 24 · green 20 · red 0 · recorded 0 · timeout 0 · not-run 4` (the four `operator` legs), trail
`.andromeda/runs/2026-10-19T22-26-15-implement/gate-2026-10-19-shared-fixtures.json`. Counts in
`escher-0.2.0/chunks/2026-10-19-shared-fixtures/evidence/counts.md`; the before-listing in `evidence/blitz-tests-before.txt`.

## Files changed
- New: `tests/blitz-tests/tests/fixtures/mod.rs` — `boot`, `hold`, `password_labelled`, `password_named`, `covered`,
  `HeldNet` (`held` · `urls` · `answer`, `is_noop` not overridden).
- Modified (19, all research-listed): `stand_settle`, `font_face_sniffed_source`, `stand_boot`, `stand_act_diff`,
  `stand_diff`, `stand_snapshot_text`, `stand_mcp_masked`, `stand_session_recording`, `stand_act_spans`,
  `stand_mcp_tools`, `stand_act_filled`, `stand_act_screenshot`, `stand_act_obstructed`, `stand_act_scroll`,
  `stand_act_transformed`, `stand_act_timer`, `stand_replay`, `stand_keyboard`, `stand_snapshot_state` (all under
  `tests/blitz-tests/tests/`).
- Scope read: `scope: clean — changed 20 · listed 20 · recorded 0`; no `scope-record.md` line owed.
- Matrix: no capability claimed by this chunk (`claimed by 2026-10-19-shared-fixtures: 0`) — no ledger write.

## Measured against the plan's predictions
- blitz-tests 139 result lines · 586 passed · 0 failed · 12 ignored; the test listing byte-identical to the
  before-record (`cmp` exit 0; 139 targets, 598 tests).
- agent-run: boot `ready`; `run stand` 178 · 0 · 7; `run font_face_sniffed_source` 2 · 0 · 0.
- a11y leg 6 · 6 · 4 · 8 · 5 · 14 = 43. Workspace `lines 199 passed 1045 failed 0 ignored 12`. Doc leg green.
- Off-unix `cargo check --tests --target x86_64-pc-windows-gnu -p blitz-tests` green.
- No deferral: Rust delta present, every gate ran.

## Deviations from plan, with justifications
1. **`stand_mcp_tools`' covered site (plan step 6).** The step lists it under `fixtures::hold` with "its label at
   `:532`", but that site is `serve_mcp`'s boot closure: the label is the MCP caller's `app` and the closure returns
   the `Result` the server handles. A `hold` that unwraps cannot serve it. Written as step 5 writes
   `stand_mcp_masked`'s: `Session::start(app, || fixtures::boot(fixtures::covered, incremental))`. Every probe the
   plan counts reads as predicted (`Harness::from_vdom` 7; covered page stated once).
2. **`stand_replay`'s label** is written `slug(LeanTask::Counter)`, the expression the removed `hold_variant` carried,
   in place of the literal `"counter"` the step names — the same value; the helper's doc moved to a one-line comment
   at the one call site.
3. **`stand_session_recording`'s hold** now panics, on a refused label only, under the shared message ("the label is
   one of the closed set") where its own helper read "the fixture's label is a label" — panic-path text, no assertion.
4. **`stand_snapshot_text`'s page** gained the root id `fx-root` (the plan's implementation note). No reading moved:
   its two checks green in both modes, so the file's own page was not kept.
5. `font_face_sniffed_source` discards `answer`'s returned url as a plain statement (no `#[must_use]`, clippy clean).

## Surfaced
- Nothing. No spec↔reality gap.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / rustc (the gate entries' builds and tests) | this run (the gate tool's entries) | terminated — `census: none running` (`comm escher-session, cargo, rustc · args agent-run.sh`) |
| session hosts of `stand_session_lifecycle` / `stand_session_quiet` (test binaries re-run as children) | a gate entry's test run | terminated — none running in the same census |

## Decisions & corrections
- Guard refusal: the Bash guard (`bash-guard.sh`, cd arm) refused a `cd` into `tests/blitz-tests/tests`; re-issued
  once as a subshell. A recurrence of the standing learning host-linux §Paths («`cd` only inside a subshell»).
- Hook refusal: the stamp-ahead PostToolUse hook refused an estimated end time (`22:41Z`, clock `22:38:10Z`) written
  into `evidence/counts.md`; re-written to cite the agent-run boot event's own stamp.

## Next
The operator pass: entries **21** (`gate.py hygiene`, before the pre-CI commit), **22** (fast leg, clean-tree guard and
push, after the pre-CI commit `chore(2026-10-19-shared-fixtures): operator pre-CI commit, for the run this chunk's
verdict reads`), **23** (`ci.py conclusion --sha HEAD --wait 1800`) and **24** (the per-OS CI tallies into
`evidence/ci.md`) — then `/andromeda-wrap-session`.
