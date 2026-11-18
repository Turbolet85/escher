# P5 review — 2026-11-18-capability-statement «Capability statement» (card 2)

**Plan:** `doc:2026-11-18-capability-statement/plan` rev 5. Open its render in an editor at
`.andromeda/cache/docs/2026-11-18-capability-statement/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-capability-statement`
  shows a diff of the chunk's records.

## What changed since card 1
Your review is snapped as inputs#I2, answering `.andromeda/runs/2026-11-18T03-58-49-phase/p5-card-1.md`.
- **Step 10 is new: the docs leaves are landed in this chunk.** Each page is copied to `evidence/leaves/`, edited
  there, and landed with `rec.py harness land --file {copy} {page}` (`--dry-run` first). Each stale count is removed,
  never re-measured: the `a11y` leg as «fourteen», «115 tests», and `stand_keyboard`'s «23 tests». Every other word of
  each line is kept. The CARRY closes at the wrap.
- **The sites.** The carry lists six. A word search of the four pages finds the same counts at **9 lines**, which the
  step takes too:
  - `.claude/rules/a11y.md:19`, `:35`, `:42` (two counts there);
  - `.claude/docs/a11y-summary.md:8`, `:14`, `:35`;
  - `.claude/docs/tests-summary.md:21` (three), `:29`;
  - `.claude/docs/services/seven_guis.md:38`.
- **Touchpoints** gain the four pages. Research's lists are synced (rev 2) and its `## Sweeps` records the search:
  18 hits · 13 changed · 5 no-change.
- **The check (gate 6).** It is `git grep -n -E "fourteen files|leg's fourteen|its fourteen\)|fourteen since|115
  tests|23 tests"` over the four pages, expecting `exit 1` with no output.
  - It does not forbid every «fourteen». Five on those pages are no leg count and stay true: `NODE_FIELDS` fourteen
    (`a11y.md:26`, `a11y-summary.md:25`), «the fourteen node keys» (`tests-summary.md:24`), the fixture name
    `fourteen-people` (`seven_guis.md:19`) and the ordinal «fourteenth» (`tests-summary.md:21`). A bare «no fourteen»
    check would stay red on correct text.
  - Baseline: red on the 9 lines. Must-pass: green in a clone with the counts removed and those five kept.
- **Scope amended** (rev 4, `[premise-corrected]`, citing inputs#I2). Its «wait on a comparer» bullet now states the
  landing.
- **Also changed.** The leaves criterion now asserts the landing, each land's verdict, gate 6 and a line diff showing
  only counts removed. The «no hand edit of a leaf» constraint is replaced by «only through `rec.py harness land`,
  nothing re-measured».

## Goal
Bring `docs/capabilities.md` from 0.2.0 to 0.3.0 for Viola's planning:
- add three missing sections (`caused`, seeded randomness, bundled fonts and the cross-OS readings);
- correct the stale lines;
- re-point the retired residual owners (page and `deny.toml`);
- set the README's 0.3.0 heading to «built, its cold-agent run to come»;
- widen the pin to the two leg counts;
- remove the stale counts from the four docs leaves.

No code, carrier or crossing changes.

## Top acceptance criteria
1. The page states 0.3.0 with no stale 0.2.0 line (gate 3: `exit 1`; baseline red 8 lines, must-pass green).
2. Three new §What it can do sections with their `Witnesses:` lines.
3. §Accessibility reach reads eighteen files and the re-read reading of job 134540877674, names the hidden-focus SC
   2.4.3 gap, and names `res-llkgqygv` (`cry-vpudls5k`).
4. §Where each claim is proven reads seventeen named targets, 64 files and 273 · 1467 · 0 · 30 on CI#44225111039, and
   names `WITNESSES_030` with a method controlled on `WITNESSES_020` (`cry-5telyrb7`).
5. No product line names `.andromeda/residuals.md` (gate 4). The `deny.toml` IDs are unchanged and the audit is green
   (`cry-nmakbxjn`).
6. The README's `### 0.3.0 — built, its cold-agent run to come` claims no 0.3.0 cold-agent result (inputs#I1).
7. `test_the_leg_counts_are_stated_in_words` pins 18 and 17 from `ci-leg.sh`. Its control reads it red;
   `ci-scripts` reads green.
8. The four leaf pages are landed with every stale count removed (gate 6, each land's verdict, a line diff), and the
   CARRY closes (`cry-svjyrphi`, inputs#I2).
9. The operator pass reads `verdict: green` on the pushed tip.

## Touchpoints
- Product files: `docs/capabilities.md`, `README.md`, `deny.toml` (owner clauses only),
  `.github/scripts/test_capability_statement.py`.
- Docs leaves, through `rec.py harness land`: `.claude/rules/a11y.md`, `.claude/docs/a11y-summary.md`,
  `.claude/docs/tests-summary.md`, `.claude/docs/services/seven_guis.md`.
- New files: none.

## Test commands
| # | entry | role | baseline |
|---|---|---|---|
| 1 | `ci-leg.sh ci-scripts` | unit | green 11.65 s |
| 2 | `ci-leg.sh audit` | probe | green |
| 3 | page stale-token probe | probe | red 8 lines, must-pass green |
| 4 | `residuals.md` probe | probe | red 6 lines, must-pass green |
| 5 | secret search | probe | green, known-positive red |
| 6 | leaves stale-count probe (new) | probe | red 9 lines, must-pass green |
| 7 | guard, base `6ebf42e65` (product files) | probe | green; clone pair clean / outside 1 |
| 8 | `ci-leg.sh fast` | lint | seen green at the last wrap |
| 9–11 | operator: hygiene, push, `ci.py conclusion --wait 2700` | probe | — |

Controls: `.andromeda/runs/2026-11-18T03-58-49-phase/p5-controls.md`.
- The guard excludes `.claude/` by its own rule, so gate 6 reads the leaves.
- planlint lists gate 6 as reading `.claude/` paths the wrap writes before its light gate. Its assertion holds on both
  sides of that write: the masters state no such count for a comparer to write back.

## Concretization preview
No capability is claimed. The pool holds v030-15 alone, «Cold-agent bar»'s.

## Notes for the review
- Expected amendments (wrap):
  - architecture `decision/capability-statement`, three test-plan pin rows and a11y-plan
    `decision/capability-statement-reach-by-hand` gain the two pinned leg counts;
  - masters citing `docs/capabilities.md` by line are re-pointed.
- CI#44233286731 on `6ebf42e65` reads green 17/17. Both standing WATCHes gain a green run.
- Mechanical checks re-run whole on rev 5: all pass, planlint 0 hits, inputs#I1 and #I2 cited, research lists parse
  (8 items), validation-1 aligned after the scope amendment.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | The README's 0.3.0 heading flips to «done and verified», with the 0.3.0 cold-agent result line, once v030-15 verifies | «Cold-agent bar» (existing entry) |

Apply? (yes / review / cancel)
