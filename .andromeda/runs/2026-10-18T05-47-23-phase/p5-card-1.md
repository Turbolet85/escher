# P5 review — 2026-10-18-upstream-sync-determinism-and-traces

**Plan:** `escher-0.2.0/chunks/2026-10-18-upstream-sync-determinism-and-traces/plan.md` (336 lines plus the baselines; 30 gates; 9 criteria) · research `research.md` · scope `scope.md`

## Goal
Take upstream's one behaviour commit, `fb8d6c15e` (#1220: over no box `get_cursor` answers `Default`, so the windowed
cursor no longer vanishes), as a no-commit cherry-pick of upstream's text. Taffy stays the registry `0.15` and no git
dependency enters the root manifest. This follows F1, answered **B** by the operator in the founder's stead (delegate
the operator, 2026-10-18, PROVISIONAL, inputs#I5). The chunk also re-reads CI and audit green, the per-OS tallies, the
sink against upstream's delta, and the three CARRYs.

## Top acceptance criteria
- Only upstream's text lands: the patch-present, exact-take (17 + 3 lines, nothing else) and preservation entries
  are green. `Cargo.toml`, `Cargo.lock` and `layout/` are byte-identical to `fb842671`.
- No git dependency, and the audit reads `advisories ok · licenses ok`.
- Upstream's `hovering_below_content_reports_default_cursor` and `pointer_leave` pass.
- `fast` and `doc` pass. The tally is recorded against 192 · 990 · 0 · 12 (predicted +1 passed).
- `run stand` (170 · 0 · 7) and `run all` (562 · 0 · 12) pass, and so do `host_log` and `mcp_log`, under both builds.
- Fork CI reads green, with its per-OS tallies explained against CI#39100630123.
- CARRY 2: `keyboard_compat.rs` is still upstream's, so both exclusions stay. CARRY 3: an F1–F8 merge-surface table,
  no PR.

## Touchpoints
- `packages/blitz-dom/src/document.rs` — upstream's two hunks, by `git cherry-pick --no-commit fb8d6c15e`. Nothing of
  ours in the file changes.
- Evidence: `before.md`, `take.md`, `counts.md`, `readings.md`, `sink.md`, `lock.md`, `ci.md` and `operator-pass.md`
  under the chunk's `evidence/`.

## Test commands (30 entries)
- probes 1–8: the take, its exactness, preservation, the pin, the delta census, CARRY 2;
- upstream's test (9), `pointer_leave` (10);
- `ci-leg.sh fast` with its tally and by-name reads (11–14);
- `doc` (15), `audit` (16), `a11y` with its tally (17–18);
- `agent-run.sh` boot · stand · all, plus the 41-file count (19–22);
- `cargo test -p seven_guis` (23), the adapter census (24), the windowed build and smoke (25–26);
- operator: hygiene · `fast` + clean-tree push · `ci.py` · the per-OS tally (27–30).

Baselines:
- 24 `new` entries are baselined: 1, 2 and 9 red by construction, every other one green on an existing property.
- Must-pass controls ran in a cherry-picked scratch clone; must-fail controls ran in a clone with planted edits (`.andromeda/runs/2026-10-18T05-47-23-phase/p5-controls.md`).
- The coverage leg is left to CI's Coverage job: no floored crate changes.

## Concretization preview
None. A sync makes no 0.2.0 capability verifiable, so nothing is claimed. The pool is v020-04 … v020-11, and v020-03 is
held by the gated 2026-10-18-engine-debts.

## Notes
- The pre-CI commit subject is exactly `chore(2026-10-18-upstream-sync-determinism-and-traces): operator pre-CI commit, for the run this chunk's verdict reads`.
  Its body carries `(cherry picked from commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc)`.
- Your answer cites the founder's "ruling 9" as naming cherry-pick its fallback. That ruling is not in this repository
  (`grep -i cherry` over `.andromeda`, `escher-0.1.0` and `escher-0.2.0`), so it is recorded in your words only.
- The windowed cursor's visibility is witnessed by upstream's unit test alone. The smoke cannot see a cursor.
- Check 10 caught one citation: CARRY 3's quoted `inputs#I4` meant the engine-debts chunk's snapshot. It is re-spelt in
  `scope.md` by that chunk's path.

## Beyond the plan
| # | proposal | owner / place |
|---|---|---|
| 1 | CARRY 1 — upstream's four Taffy-main commits past `23b9b26a9` (git Taffy `ea07c4c3`, unreleased) move on, left out again on F1 B | «Upstream sync ahead of visual lint and real-app readiness» (escher-0.2.0/working-route.md:32), a CARRY written by the wrap's route-resolve |
| 2 | CARRY 2 — whether upstream retired `keyboard_compat.rs` (still listed at `fb8d6c15e`); the coverage exclusion drops with it | «Upstream sync ahead of visual lint and real-app readiness», a CARRY written by the wrap's route-resolve |
| 3 | CARRY 3 — the engine-debts seven fixes' merge surface, no upstream PR, re-read per row at the next sync | «Upstream sync ahead of visual lint and real-app readiness», a CARRY written by the wrap's route-resolve |
| 4 | F1's delegate answer, PROVISIONAL, joins the founder's epoch-boundary discharge batch beside the engine-debts batch | the playbook's «Provisional discharge» rule; the wrap lists it with its sites |
