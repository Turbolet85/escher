# Implement outcome — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

**Outcome: green + smoke.** Run `.andromeda/runs/2026-10-19T19-16-25-implement/`, 2026-10-19. Base `67f17b9d6`
(HEAD unmoved). Nothing committed; no `state.yaml` written.

## What was done
- Steps 2–3: `git cherry-pick --no-commit 17de049b1` (clean), then `--no-commit 38ca4e777` (one conflict in
  `mutator.rs`'s `add_children_to_parent`). Resolved by keeping both sides as the plan says: our changed-set `if` first,
  then upstream's comment and `focus_within_old_chain` declaration. Then `git add`, `git cherry-pick --quit`; no pick is
  in progress.
- P2: the whole block, one firing, green on the first run: `entries 34 · green 29 · red 0 · recorded 1 · timeout 0 ·
  not-run 4` (the four not-run are the `operator` entries 31–34). Trail
  `.andromeda/runs/2026-10-19T19-16-25-implement/gate-2026-10-19-upstream-sync-visual-lint-and-real-app-readiness.json`.
  - The exact-take probe reads no output, and the preservation guard reads exit 0.
  - `focus_within` passed 7, `focus_redraw` passed 4; upstream's exact redraw counts hold on our engine.
  - The workspace tally went `lines 197 passed 1034` → `lines 199 passed 1045 failed 0 ignored 12`, as predicted.
  - Audit `advisories ok, licenses ok`; a11y `lines 6 passed 43 failed 0`.
  - `run stand` 178 · 0 · 7 (unmoved); `run all` 586 · 0 · 12 (+11).
  - seven_guis 33 passed, with `host_log` and `mcp_log` green under both builds.
  - The windowed smoke ran (`WAYLAND_DISPLAY` set) and logged `service.name=seven_guis`.
- P3: smoke ran as a P2 gate ✓ (entry 30, plus the agent-run entries 23–26). Not re-driven.
- Matrix: the chunk claims no cap (`claimed … 0`), so nothing was written, as the plan says (a sync makes no cap verifiable).
- No source-delta deferral: Rust changed, and every gate ran.

## Files changed
- Taken from upstream: `packages/blitz-dom/src/{document.rs, events/keyboard.rs, mutator.rs, node/node.rs, stylo.rs}`,
  `tests/blitz-tests/tests/all.rs`, `wpt/WPT_COMMIT`. New: `tests/blitz-tests/tests/focus_within.rs`, `focus_redraw.rs`.
  The last four are byte-identical to `17de049b1`. All of these are staged by the cherry-picks.
- Evidence under `escher-0.2.0/chunks/2026-10-19-upstream-sync-visual-lint-and-real-app-readiness/evidence/`:
  `before.md`, `take.md`, `lock.md`, `readings.md` (CARRY 1–4 per row, the three behaviour changes with their
  witnessing tests, the take's line map for the wrap), `sink.md`, `counts.md`. `ci.md` is the operator pass's (step 10).

## Deviations from plan
- None in code. The `mutator.rs` resolution is the plan's, line for line, and no gate-driven edit was made.
- A citation measured one line off: research.md and plan.md place our changed-set `if` at `mutator.rs:731-733`. The
  base reads it at `HEAD:mutator.rs:730-732`, and it now stands at 731–733. Recorded in `evidence/take.md` and
  `evidence/readings.md` (line map) for the wrap's re-pointing. research.md and plan.md are left as written.
- Scope record: none needed. `gate.py scope` → `scope: clean — changed 9 · listed 9 · recorded 0 … · excluded 57`.

## Surfaced / stuck
- Nothing surfaced, nothing stuck.

## Next
The operator pass, then `/andromeda-wrap-session`. Fire entries 31–34 in order:
- 31: `gate.py hygiene` as written, before the pre-CI commit.
- The pre-CI commit, with its subject exactly `chore(2026-10-19-upstream-sync-visual-lint-and-real-app-readiness):
  operator pre-CI commit, for the run this chunk's verdict reads`. Its body carries the two trailers
  `(cherry picked from commit 38ca4e777513607c7dcb659748380e7a650b018f)` and
  `(cherry picked from commit 17de049b11095808e0d093be2a8f0520e4dcbc0e)`.
- 32: the push entry, through `gate.py run --plan … --operator 32`.
- 33: `ci.py conclusion --sha HEAD --wait 1800` as written.
- 34: the per-OS tallies, through `--operator 34 --id {run id}`.
Each result goes in `evidence/ci.md`, against CI#39302945352 and with CI#39111579925's moves attributed beside.
The take is staged in the index, so the pre-CI commit takes it as it stands.

## Process hygiene
`gate.py census --comm seven_guis_native --comm escher-session --comm cargo --args agent-run.sh` → `census: none running`.

| process | started by | final state |
|---|---|---|
| `seven_guis_native` (entry 30) | this run (the smoke entry) | terminated — `timeout 10` bound, census none running |
| cargo / test binaries, `escher-session` hosts (entries 10–29) | this run (gate entries) | terminated — census none running |
| `agent-run.sh` runs (entries 23–26) | this run | terminated — census none running |

## Decisions & corrections
- Guard refusal: at Setup, the Bash guard refused `cd` into the skill's references dir ("moves the session cwd"). It
  was re-issued once with absolute paths and ran.
- No operator correction or ruling was given in this run.
