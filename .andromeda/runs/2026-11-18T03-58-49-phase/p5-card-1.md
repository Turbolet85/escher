# P5 review — 2026-11-18-capability-statement «Capability statement»

**Plan:** `doc:2026-11-18-capability-statement/plan` rev 3. Open its render in an editor at
`.andromeda/cache/docs/2026-11-18-capability-statement/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-capability-statement`
  shows a diff of the chunk's records.

## Goal
Bring `docs/capabilities.md` from 0.2.0 to 0.3.0 for Viola's planning:
- add three missing capability sections: errors with the action (`caused`), seeded randomness, and bundled fonts with
  the cross-OS readings;
- correct the stale lines: version, stand, Not built's layout carrier, a11y reach, the MCP carried set, the claim
  map's counts;
- re-point the retired `.andromeda/residuals.md` owners (page and `deny.toml`) to residual rows;
- set the README's 0.3.0 heading to «built, its cold-agent run to come»;
- widen the page's pin to hold the two leg counts.

No code, carrier or crossing changes.

## Top acceptance criteria
1. The page states 0.3.0 and carries no stale 0.2.0 line. Gate: the `git grep … fifteen named|…` probe reads
   `exit 1`. Baseline red on 8 lines; must-pass green in a clone.
2. Three new §What it can do sections, each with its `Witnesses:` line:
   - `stand_act_errors` and `stand_act_error_text`;
   - `stand_seeded_randomness`;
   - `stand_bundled_font_families` and `stand_cross_os_readings`.
3. §Accessibility reach reads eighteen files and the reading re-read from job 134540877674 (predicted
   18 · 163 · 0 · 0). It names the hidden-focus SC 2.4.3 gap and `res-llkgqygv` (CARRY `cry-vpudls5k`).
4. §Where each claim is proven:
   - seventeen named targets, 64 `stand_*` files, 273 · 1467 · 0 · 30 on CI#44225111039;
   - `WITNESSES_030`, sixteen rows, its witness count taken by a method that first reads 46 on `WITNESSES_020`
     (CARRY `cry-5telyrb7`).
5. No product line names `.andromeda/residuals.md`. `deny.toml`'s IDs are unchanged and the audit is green
   (CARRY `cry-nmakbxjn`).
6. The README's `### 0.3.0 — built, its cold-agent run to come` section shows six built stages and 16 of 17 verified,
   with no 0.3.0 cold-agent claim (inputs#I1).
7. `test_the_leg_counts_are_stated_in_words` pins the a11y leg (18) and the stand-repeat named targets (17) from
   `ci-leg.sh`. Its `gate.py control` (`--test open_state` deleted) reads it red; `ci-scripts` reads green.
8. The operator pass reads `verdict: green` on the pushed tip.

## Touchpoints
- `docs/capabilities.md`
- `README.md`
- `deny.toml` (owner clauses only)
- `.github/scripts/test_capability_statement.py`

New files: none. The six docs-leaf sites (CARRY `cry-svjyrphi`) are named for the wrap's leaves pass and edited by no
step.

## Test commands
| # | entry | role | baseline |
|---|---|---|---|
| 1 | `ci-leg.sh ci-scripts` | unit | green 11.65 s |
| 2 | `ci-leg.sh audit` | probe | green |
| 3 | stale-token probe | probe | red 8 lines, must-pass green |
| 4 | `residuals.md` probe | probe | red 6 lines, must-pass green |
| 5 | secret search | probe | green, known-positive red |
| 6 | guard, base `6ebf42e65` | probe | green; clone pair clean / outside 1 |
| 7 | `ci-leg.sh fast` | lint | seen green at the last wrap |
| 8–10 | operator: hygiene, push, `ci.py conclusion --wait 2700` | probe | — |

Controls: `.andromeda/runs/2026-11-18T03-58-49-phase/p5-controls.md`. The guard excludes `.claude/` by its own rule, so
it does not witness the leaves. The plan words that criterion as holding by construction.

## Concretization preview
No capability is claimed. The pool holds v030-15 alone, which the «Cold-agent bar» entry names, so nothing links in
the ledger.

## Notes for the review
- Your P4 answers are snapped as inputs#I1: README «Built, cold-agent to come»; pin «Widen the pin».
- Expected amendments (wrap):
  - architecture `decision/capability-statement` gains the two pinned leg counts;
  - three test-plan pin rows join them;
  - a11y-plan `decision/capability-statement-reach-by-hand` notes the pinned leg count.
  - Masters that cite `docs/capabilities.md` by line will need re-pointing at the wrap.
- CI#44233286731 on `6ebf42e65`, in progress at take-up, now reads green 17/17 (wall 2027 s). Both standing WATCHes
  gain a green run: no cancelled job, `Coverage report` green at attempt 1.
- Mechanical checks:
  - all pass; planlint 0 hits;
  - inputs#I1 cited;
  - validation-1 aligned;
  - plan size about 230 lines.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | The README's 0.3.0 heading flips to «done and verified», with the 0.3.0 cold-agent result line, once v030-15 verifies | «Cold-agent bar» (existing entry) |

Apply? (yes / review / cancel)
