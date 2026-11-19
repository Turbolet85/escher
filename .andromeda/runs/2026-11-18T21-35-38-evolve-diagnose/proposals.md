# Evolve Diagnosis — escher-0.3.0 · Epoch 6 — Polish & ship · 2026-11-18T21:35:38Z

Read at HEAD bc766124059bc619b94019fe200681196553acbc · friction seq ≤ 2640

Captures (gitignored, never sealed): `.andromeda/cache/evolve-diagnose/2026-11-18T21-35-38/` — `friction.ndjson`,
`q-retractions.json`, `q-health.json`, `q-typed.json`, `q-cross.json`, `q-steps.json`, `q-by-check.json`,
`q-chains.json`, `q-level.json`, `q-lookback.json`. Analysis scripts in this run dir: `prepass.py`, `analyse.py`,
`lookback.py`, `twins.py` (each runs from the root).

## Mechanism health

- **Records:** 198 (133 step / 65 friction), 8 chunks plus 8 new-session starts. Unparseable lines: 0 of 2640.
  Malformed `ts`: 0. Id fill: 198/198.
- **Coverage vs expected:** every chunk has its full set (phase 5 · implement 4 · wrap 5), so there are no gaps.
  The step records above the base set are all accounted for:
  - `2026-11-17-stand-requirement-sweep`: gated second pass (phase research/plan/validate +1, implement code/fix-loop/smoke/report +1).
  - `2026-11-18-served-error-page`: a gate-only fix-loop re-entry and a rewritten report (+1 each).
  - `2026-11-18-cold-agent-live-run`: a phase revision (research/plan/validate +1). Its fix-loop ran twice: first
    `soft-exit` (`surfaced`), then a re-entry, green. A report followed the green run.
  - New-session: 8 orientation records over 8 session starts.
- **Outcomes:**
  - `ok` everywhere except: implement/code `ok-degraded` ×1 (upstream-sync), implement/fix-loop `soft-exit` ×1
    (cold-agent-live-run), and wrap/reconcile `halted-resolved` ×6 of 8.
- **Untyped rate per step:**
  - phase/research 3/7, phase/take-up 1/2, phase/validate 1/8.
  - implement/code 1/8, implement/fix-loop 1/9.
  - wrap/reconcile 1/20.
  - Every other step 0.
- **Problem-fact fill:** 24 of 133 step records carry facts, 28 facts in all.
- **Signal line:** guard-refused 41 over 133 step records · exact-hit 7 over 8 curation records.
  - The `{n}` strings were all well-formed; nothing was listed as odd.
  - The producers' notes break the 41 down by arm (count only, no proposal):
    - cd arm 17
    - pipe-into-filter arm 10
    - stamp-ahead hook 6
    - heredoc arm 3
    - recursive-rm deny 3
    - write-guard (harness family) 1
    - a quoted `-->` read as a redirect 1
- **Cascade rows:** all 16 are row runs (`agents-pick`, no `agents-classify`).

  | chunk | step | first try | re-spawned | wall s | rows: picked of | one draw | dead | fit-rejected | growth |
  |---|---|---|---|---|---|---|---|---|---|
  | upstream-sync-ahead-of-polish-and-ship | distill | 131/131 (pick 117/117 rowhistory 7/7 rowtask 7/7) | 0 | 1382 | 1433 of 3491 | 256 | 0 | - | |
  | upstream-sync-ahead-of-polish-and-ship | reconcile | 111/111 (cites 1/1 fit 7/7 leaf 11/11 pick 78/78 propose 7/7 rowtask 7/7) | 0 | 1368 | 578 of 3491 | 224 | 0 | 4 | 0 |
  | answer-writer-by-family | distill | 131/131 | 0 | 1320 | 917 of 3489 | 152 | 0 | - | |
  | answer-writer-by-family | reconcile | 90/90 (cites 1/1 pick 78/78 propose 4/4 rowtask 7/7) | 0 | 601 | 94 of 3489 | 63 | 0 | 0 | 0 |
  | stand-requirement-sweep | distill | 131/131 | 0 | 1478 | 1178 of 3489 | 244 | 0 | - | |
  | stand-requirement-sweep | reconcile | 108/108 (cites 1/1 fit 4/4 leaf 13/13 pick 78/78 propose 5/5 rowtask 7/7) | 0 | 791 | 90 of 3489 | 36 | 0 | 0 | 0 |
  | a11y-re-read | distill | 131/131 | 0 | 1261 | 830 of 3490 | 140 | 0 | - | |
  | a11y-re-read | reconcile | 108/108 (cites 1/1 fit 6/6 leaf 10/10 pick 78/78 propose 6/6 rowtask 7/7) | 1 (ruled 1) | 821 | 68 of 3490 | 19 | 0 | 4 | 0 |
  | capability-statement | distill | 131/131 | 0 | 1125 | 858 of 3490 | 228 | 0 | - | |
  | capability-statement | reconcile | 109/109 (cites 1/1 fit 4/4 leaf 13/13 pick 78/78 propose 6/6 rowtask 7/7) | 0 | 905 | 113 of 3490 | 50 | 0 | 2 | 0 |
  | cold-agent-bar | distill | 131/131 | 0 | 1214 | 772 of 3490 | 199 | 0 | - | |
  | cold-agent-bar | reconcile | 114/114 (cites 1/1 fit 6/6 leaf 15/15 pick 78/78 propose 7/7 rowtask 7/7) | 0 | 1044 | 133 of 3490 | 59 | 0 | 6 | 0 |
  | served-error-page | distill | 131/131 | 0 | 1320 | 897 of 3503 | 211 | 0 | - | |
  | served-error-page | reconcile | 118/118 (cites 1/1 fit 10/10 leaf 15/15 pick 78/78 propose 7/7 rowtask 7/7) | 2 (ruled 2) | 1397 | 231 of 3503 | 85 | 0 | 7 | 0 |
  | cold-agent-live-run | distill | 131/131 | 0 | 1310 | 1297 of 3515 | 285 | 0 | - | |
  | cold-agent-live-run | reconcile | 110/110 (cites 1/1 fit 3/3 leaf 15/15 pick 78/78 propose 6/6 rowtask 7/7) | 0 | 850 | 58 of 3515 | 37 | 0 | 0 | 0 |

  Chunk markers are shortened by their date prefix (2026-11-17 for the first three, 2026-11-18 for the rest). Each
  wall figure sums the record's `wall-*` signals. No row has a `wall-classify`, and false-yes does not apply to row runs.
- **Epoch trend (first → last, min, median, max):**
  - distill: first try 1.000 → 1.000 (all 1.000); one draw 0.179 → 0.220, min 0.166, median 0.213, max 0.266.
  - reconcile: first try 1.000 throughout; one draw 0.388 → 0.638, min 0.279, median 0.421, max 0.670; growth flags 0
    throughout.
- **Retractions:**
  - Whole ledger: 3 records retracted and 1 step's problem facts retracted. None of them falls in this epoch.
  - 1 unresolvable, carried verbatim: `[2026-10-07T04:21:25Z-a, unknown-id-or-scope, {"id": null, "note": "in step
    record 2026-10-07T04:21:04Z-a (a step record, so no record or clause scope applies), produced git-state note:
    discount 'add -A staged 43 paths' — the measured count is 47 files changed"}]`.
  - Clause-retracted: 4 in the ledger, none in this epoch. No retraction targets another retraction.
- **Calibration boundaries in range:** every record in this epoch postdates every boundary. The last of them is the
  C batch at 2026-11-08T22:31:55Z, and the epoch starts 2026-11-17. So every field and type was live for the whole
  range, and nothing here is era.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 7 cases · weight 16
**Pattern:** In 6 of 8 chunks, P5's mechanical checks either passed a plan defect that only a later baseline or the
operator's review caught, or warned by construction on a chunk shape the check does not fit. The rate is 7 per 10
validate runs. The previous epoch (E5) had 8 cases of this type.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | check 4 (6) REQUIRED-RESOLUTION: two one-shot controls named `fast` as their entry, which stops at the first red target; two one-target entries added, plan re-put | iterations 1 | .andromeda/runs/2026-11-17T11-43-12-phase/p5-card-1.md |
| 2026-11-17-stand-requirement-sweep | check 7 (code-graph consulted) WARNED on a test-only chunk — a WARN by construction | — | — |
| 2026-11-18-a11y-re-read | check 7 WARNED: modify-set holds .rs test files, research recorded the graph not applicable | — | — |
| 2026-11-18-a11y-re-read | the plan rested a CARRY reading on an unmeasured absence no predicate attempts; the operator's review caught it (one conditional focusable node found) | dialogue 1 · iterations 1 | .andromeda/runs/2026-11-18T01-29-40-phase/p5-card-2.md |
| 2026-11-18-capability-statement | a plan leaving a CARRY's docs-leaf counts to the wrap's leaves pass passed every check, though no comparer has a master source for such a count; the operator's review caught it | dialogue 1 | .andromeda/runs/2026-11-18T03-58-49-phase/p5-card-2.md |
| 2026-11-18-cold-agent-bar | check 4 (9) baselines read entries 2 and 4 red on entry defects the P4 dry-run and planlint cannot see (`unittest` on a `.github` path; a secret search over files that spell `apiKeySource` and a `/home/<user>` fixture) | retries 2 | $TMPDIR/andromeda-gate/2026-11-18-cold-agent-bar/phase-2026-11-18T06-33-34/2.log |
| 2026-11-18-served-error-page | check 4 (2): a touched served table had no role smoke entry, following an earlier plan's stand-in prose | iterations 1 | — |

**The cases fall into three forms:**
- **Check 7 warns by construction (2).** Both are test-only chunks. E5 had two more on a sync chunk and on a chunk
  measured by compile (`2026-11-15-upstream-sync-errors-and-explanation`, `2026-11-15-taffy-from-upstream-git`).
- **Gate entries are only shown wrong at baseline (3).** The defects are an entry that stops at its first red, a
  unittest path form, and a missing smoke entry.
- **The operator's review caught what no predicate reads (2).** One plan rested on an unmeasured absence. The other
  routed a CARRY to the comparers, which cannot discharge it (see P3).

**Proposal (directions, one per form):**
- (a) Let check 7 accept a recorded `graph-not-applicable` from research (or a test-only modify-set) as satisfying it,
  so its WARN keeps its signal.
- (b) Have the P4 dry-run (or planlint) fire each gate entry's command form once, so entry-syntax defects surface
  before P5's baseline.
- (c) Add a predicate that flags a CARRY routed to "the wrap's comparers" when its subject has no master source. This
  ties to P3.

### P2 — wrap-session/reconcile · `contract.false-positive-proposal` — 7 cases · weight 10
**Pattern:** In 6 of 8 wraps, master proposers offered operations Validate rejected. Most were check-2 duplicates: one
new fact offered by several masters' proposers. The rest misread a ruling's scope (checks 1, 3, 4). The type recurs in
every 0.3.0 epoch. Earlier rejected shares were 22/64, 28/60 (untyped in E5), 37/57 (E4) and 31/94 (0.2.0 E5).

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-answer-writer-by-family | test-plan proposer offered 2 ops, both rejected: a code-held layout fact the architecture proposer dropped (check 2) and the operator's per-pass no-runner directive as a master rule (check 3), rejected on the same ground the wrap before | — | .andromeda/runs/2026-11-17T19-37-35-wrap/fanout-results.md |
| 2026-11-17-stand-requirement-sweep | obs-plan proposer set a why to facts that were not the exception's reason (check 3); leaf comparers gave 2 findings that would have written false text | — | .andromeda/runs/2026-11-18T00-09-04-wrap/fanout-results.md |
| 2026-11-18-a11y-re-read | a11y-plan proposer set contract/cells-grid/editor-enter as contradicted; the measured editor was click-opened (check 4) | — | .andromeda/runs/2026-11-18T03-17-48-wrap/fanout-results.md |
| 2026-11-18-a11y-re-read | a11y-plan proposer's set dropped «in every document» and «takes keys»; rejected, re-written by main | reformulations 1 | .andromeda/runs/2026-11-18T03-17-48-wrap/fanout-results.md |
| 2026-11-18-capability-statement | 8 of 15 operations rejected: 7 under check 2 (one fact offered in 2–3 masters), 1 under check 1 | — | .andromeda/runs/2026-11-18T05-48-10-wrap/fanout-results.md |
| 2026-11-18-cold-agent-bar | security-plan set read F5 as built (check 1), restated by main; 7 further adds repeated obs-plan's facts (check 2) | reformulations 1 | .andromeda/runs/2026-11-18T09-02-58-wrap/fanout-results.md |
| 2026-11-18-cold-agent-live-run | test-plan add generalised the founder's one run to «once per version»; obs-plan set put the byte-faithful rule on the raw transcript row | reformulations 1 | .andromeda/runs/2026-11-18T20-45-44-wrap/fanout-results.md |

**Proposal:** Before the propose wave, give each proposer a home map: each new report fact against the one master that
owns it, derived once by main or by a cheap classifier. A proposer would then add only facts its master owns, and set
the rest by pointer. This targets the check-2 share, the largest share in every epoch. For the check-1/3 share, quote
each ruling's scope sentence verbatim in the proposer prompt beside the fact it governs.

### P3 — wrap-session/reconcile · `contract.cascade-miss` — 5 cases · weight 11
**Pattern:** In 5 of 8 wraps, stale leaf statements reached no comparer finding. These were counts, leg membership and
versions sourced from code or from no master row. The step-2 sweep listed them; the comparers did not offer them. The
sites were then written by hand, carried to a later entry, or removed on the operator's word. The type recurs in every
0.3.0 epoch: E3 ×6, E4 ×1, E5 ×3.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | CLAUDE.md:9's stack line kept old anyrender/Vello versions; no master row holds versions, so the pass-mode comparer could not reach it; P1's site sweep caught it, hand-written | — | .andromeda/runs/2026-11-17T16-04-14-wrap/sweep-pre-pass-1.1.txt |
| 2026-11-17-answer-writer-by-family | `cascade.py leaves` pass mode with no master changed printed «no group» and ignored two `--add` leaves; only whole mode was left, so the fix was carried on the operator's word | dialogue 1 · deferred 1 | .andromeda/runs/2026-11-17T19-37-35-wrap/cascade-dispositions.md |
| 2026-11-18-a11y-re-read | six leaf sites kept the a11y leg's old count; the sweep listed them, no pass comparer offered them, one offered finding kept the stale count | deferred 6 | .andromeda/runs/2026-11-18T03-17-48-wrap/sweep-pass-1.1.txt |
| 2026-11-18-capability-statement | 8 leaf clauses stated a11y-leg checks as outside it; none of 13 comparers flagged any, the report named 3, a class grep found 5; removed by hand on the operator's word | dialogue 2 | .andromeda/runs/2026-11-18T05-48-10-wrap/cascade-dispositions.md |
| 2026-11-18-cold-agent-bar | two leaf lines the sweep listed as stale drew no comparer finding in whole mode; written by hand | — | .andromeda/runs/2026-11-18T09-02-58-wrap/cascade-dispositions.md |

The project absorbed part of this mid-epoch. The stand-requirement-sweep wrap's operator directed per-file test counts
removed rather than re-measured (problem fact `2026-11-18T00:32:35Z-a`). The remaining gap is the pipeline's: a leaf
statement with no master source has no comparer route.

**Proposal:**
- Give `cascade.py leaves` a sweep-listed route: every leaf site the step-2 sweep prints is handed to a comparer as an
  explicit `--add`, in pass mode too, whether or not a master changed.
- Or give such sites a sanctioned hand-edit disposition in the letter, instead of an operator override each time
  (see L5).

### P4 — implement/fix-loop · `contract.instrument-validity` — 4 cases · weight 9
**Pattern:** In 3 of the 4 cases, a P5-planned control on a Python unittest entry read `INDETERMINATE no result line`.
The cause is that `gate.py control`'s case reader knows only nextest, libtest and pytest lines. Each case was then read
by hand. The fourth is a gate probe whose planted controls lacked the real artefact's shape. It cost a soft-exit and a
phase revision.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-18-capability-statement | `gate.py control`'s case grammar cannot read a python unittest result line; a control on the ci-scripts leg reads INDETERMINATE whatever the removed firing shows | — | .andromeda/cache/chunks/2026-11-18-capability-statement/evidence/control-1.txt |
| 2026-11-18-cold-agent-bar | the case reader knows nextest, libtest and pytest only; every control on the ci-scripts (unittest) entry read INDETERMINATE; FAIL lines read by hand | — | .andromeda/cache/chunks/2026-11-18-cold-agent-bar/evidence/control-1.txt |
| 2026-11-18-served-error-page | three controls on a unittest entry each read INDETERMINATE though the removed run was red | — | .andromeda/runs/2026-11-18T10-58-34-implement/gate-2026-11-18-served-error-page.json |
| 2026-11-18-cold-agent-live-run | entry 12's credential probe reads 1 on any real transcript (its regex matches the field name `apiKeySource`); its controls used planted files without an init record | dialogue 1 · soft_exit 1 | .andromeda/runs/2026-11-18T19-16-57-implement/gate-2026-11-18-cold-agent-live-run.json |

The unittest gap was already met in E5 (problem fact, `2026-11-15-session-binary-smoke` plan, worked around). This
epoch it recurs as a typed friction in 3 consecutive chunks. See L1.

**Proposal:**
- Teach `gate.py control`'s case reader the unittest result forms (`FAIL: name (module.Class...)`, `OK`, `FAILED
  (failures=n)`). That removes three of the four cases at the source.
- For the probe case: have P5 run each known-negative control of a content probe against a real artefact of the probed
  kind where one exists (for example a kept transcript), not only planted files.

### P5 — implement/code · `input.plan-step-ambiguous` — 4 cases · weight 8
**Pattern:** Plan steps stated an order, an import set or a precondition that the tree or the toolchain contradicts.
Implement found each on the first compile, format or run. There were 4 cases in 3 chunks, 4 of 9 code runs. E5 had 5
cases, E4 2 and E3 4.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | step 3's `cargo update -w` precedes step 5's rename; the renamed workspace key breaks every member until step 5 lands (exit 101) | retries 1 · reformulations 1 | $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/17fc2619-edcc-41cd-80aa-0414e44f0264/scratchpad/update.log |
| 2026-11-17-answer-writer-by-family | step 5 listed the tests' new `use` lines; the tests also reached `placement` and 20 types through `use super::*`, lost with the split | iterations 1 | — |
| 2026-11-17-answer-writer-by-family | step 3's mod order conflicts with rustfmt's `reorder_modules`, which the fmt leg gates on | — | — |
| 2026-11-18-a11y-re-read | step 3's tree-focus precondition cannot exist under FocusWithin (the `:focus-within` rule removes the box as Tab focuses) | reformulations 1 | — |

The import case is the defect a standing Tier-3 entry already names (P9, answer-writer curation).

**Proposal:** Add a P4/P5 planning prompt, or a planlint heuristic, for the three shapes seen:
- A manifest or workspace rename ordered after a resolver call.
- A module split whose tests use `use super::*`: list what they reach.
- A precondition stated "before X" for a case where X happens at the same event.

Each is checkable by one dry compile or format pass at P4, which the plan could name as a step.

### P6 — wrap-session/reconcile · `contract.structural-blind-spot` — 2 cases · weight 8 (one halt)
**Pattern:** The reconcile tooling has two blind spots:
- The leaf-comparer path cannot re-run a comparer whose receipt `--apply` refuses, because `--respawn` refuses an agent
  that failed no check.
- Pass-mode comparers cannot see code-sourced leaf statements. A CARRY routed to them halted the wrap.

**Evidence:** ALL 2 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | fanout check passed a leaf comparer whose `@leaf_read last:` receipt mis-quoted CLAUDE.md's last line; `cascade.py leaves --apply` refused it naming re-spawn, `render --respawn` refuses an agent that failed no check — neither path could re-run it | retries 1 | .andromeda/runs/2026-11-17T16-04-14-wrap/cascade-dispositions.md |
| 2026-11-17-stand-requirement-sweep | pass-mode comparers read a leaf against its masters' changes; leaf statements sourced from code alone are invisible by construction — a CARRY routed to them could not be discharged | dialogue 1 · halted 1 | .andromeda/runs/2026-11-18T00-09-04-wrap/cascade-dispositions.md |

**Proposal:**
- (a) Make the receipt check fanout-side: check the receipt in `fanout.py check`, so a bad receipt fails a check and
  re-spawn becomes reachable. Or let `--apply`'s refusal mark the agent as failed for `--respawn`.
- (b) For the second blind spot, see P3.

The same receipt refusal recurs below threshold as `retry.detector-respawn` (a11y-re-read, served-error-page).

### P7 — `contract.structural-blind-spot` across phase/plan, wrap-session/reconcile — 3 cases · weight 11 (one halt)
**Pattern:** These are P6's two reconcile cases plus one at plan. In that one, the phase reads only `verdict='escalate'`
playbook rows before P4, so a routine pattern row was out of view. A fork went unasked, and that cost a second round.

**Evidence:** ALL 3 cases — the two reconcile rows of P6, and:

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | the skill reads only verdict='escalate' playbook rows before P4, so rul-47a6zcoz (an engine change ratified only by the founder's word at the P4 fork) was not in view; the shear commit went unasked, the clamp answered by a delegate | dialogue 1 | .andromeda/runs/2026-11-17T11-43-12-phase/relay-4.md |

**Proposal:** Widen phase P4's playbook read to the routine rows whose trigger matches the plan's change classes (for
example "engine change for every document"). Alternatively, have planlint flag a plan touching such a class with no
fork for it.

### P8 — `contract.premise-falsified` across implement/code, implement/fix-loop, wrap-session/route-resolve — 4 cases · weight 7
**Pattern:** A plan or card carried a count, name or text forward from an earlier reading without re-measuring it at
HEAD. Examples: a witness count read at the commit that wrote it, test names since renamed, a carried-commit count
predating two takes, a residual's figure that no residual holds.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-stand-requirement-sweep | three verified 0.3.0 ledger refs name tests since renamed; the new table's fn-existence assert read each red | iterations 3 | tests/blitz-tests/tests/stand_requirement_sweep.rs WITNESSES_030 |
| 2026-11-18-capability-statement | the plan's premise that WITNESSES_020 holds 46 at HEAD is false: 46 at e7e71221d, 49 since 2026-10-25-cold-agent-bar | — | .andromeda/cache/chunks/2026-11-18-capability-statement/evidence/readings.md |
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | plan step 11 and the folded freight name 112 carried commits; 15 were taken since, so 97 are carried | — | .andromeda/cache/chunks/2026-11-17-upstream-sync-ahead-of-polish-and-ship/evidence/lock.md |
| 2026-11-18-cold-agent-bar | the P5 card named residual res-7cc5bi4k as stating the Cells actions answer at 106,088 B; no open residual's text holds that figure | — | rec.py query open_residuals, 0 rows matching 106 or actions |

**Proposal:** At research or P5, require that every figure or name a plan step relies on cites the command that read
it at the plan's HEAD. A planlint pattern could flag a bare count in a step's premise with no reading cited. This ties
to U1 and to E5's 4 premise-falsified cases at implement/code.

### P9 — `recall.corpus-recurrence` across wrap-session/curation, wrap-session/gates — 3 cases · weight 6
**Pattern:** Three times, a curated Tier-3 entry stood when its defect recurred. The rule was in the corpus and did not
prevent the defect. This type recurs in every 0.3.0 epoch, and E5 had 6.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-11-17-answer-writer-by-family | implement's split lost 22 type names reached through `use super::*` — the defect a Tier 3 entry on splitting a module records with its remedy | iterations 1 | .andromeda/runs/2026-11-17T19-37-35-wrap/curation.md |
| 2026-11-18-cold-agent-bar | the 2026-10-27 Tier-3 entry (plant a home-path control in the `<user>` form from the first write) did not prevent the P5 control record planting a made-up home path; hygiene refused it | retries 1 | .andromeda/runs/2026-11-18T09-02-58-wrap/curation.md |
| 2026-11-17-upstream-sync-ahead-of-polish-and-ship | the curated entry «keep the probe's pattern out of the files it scans» did not prevent this wrap writing the probe's pattern into session-learnings and the handoff; light-gate entry 5 red, one ~18 min re-run | retries 1 | .andromeda/runs/2026-11-17T16-04-14-wrap/light-gate-record.md |

**Proposal:** Promote the entries that keep recurring into checks that run at write time instead of prose. A planlint
pattern could cover the split-module imports (also P5's second case). A hygiene pre-check on the P5 control writer
could cover home paths. A light-gate self-probe exclusion could cover the pattern in its own files. The prose corpus
reaches the defect only when it is read at the right moment, and these three show it was not.

## Cross-step chains (starting heuristics)

### X1 — phase/plan →plan→ implement/fix-loop — 3 chunks
**Consumers** (`consumed plan: thin`):
- `2026-11-17-upstream-sync-ahead-of-polish-and-ship` (`2026-11-17T14:45:03Z-a`): the step 7 control names
  construct.rs, but the call stands in layout/mod.rs.
- `2026-11-17-stand-requirement-sweep` (second pass): entry 4's guard carries no `--allow` for the plan's own
  touchpoints. The typed friction is `input.plan-step-ambiguous`.
- `2026-11-18-cold-agent-live-run`: entry 12's credential regex matches `apiKeySource`. It is P4's soft-exit case.

**Producers:** the `phase/plan` records, each `ok`. They carry only `designed-dialogue:*` and `guard-refused:1`
signals, with no quality signal.

**Hypothesis:** Each producer looked fine and the defect lay in a gate entry or control. The P5 baseline ran on an
untouched tree or on planted fixtures. Research's `contract.instrument-validity` (stand-requirement-sweep) says so
directly: "the take-up P5 baseline of the guard entry read green on an untouched tree… no must-fail/must-pass pair over
a changed tree was fired".

**Direction:** P5 fires each guard or probe entry once over a synthetic changed tree, or over a real artefact, as a
must-fail/must-pass pair.

### X2 — phase/plan →plan→ implement/code — 3 chunks
**Chunks:** `2026-11-17-upstream-sync-ahead-of-polish-and-ship`, `2026-11-17-answer-writer-by-family`,
`2026-11-18-a11y-re-read`.

**The chain:** The consumers graded plan `thin` and filed P5's `input.plan-step-ambiguous` cases. Each producer plan
record is `ok`, with no quality signal.

**Hypothesis:** Build-order, import and precondition premises go unexercised between P4 and implement. P1's validate
checks did not read them either. The direction is P5's dry-compile step.

### X3 — phase/plan →plan→ phase/validate — 4 chunks
**Chunks:** `2026-11-18-a11y-re-read`, `2026-11-18-capability-statement`, `2026-11-18-cold-agent-bar`,
`2026-11-18-served-error-page`.

**The chain:** Validate graded the plan `thin` and P1's cases are the same events. This is the designed catch point
working. It is listed because 4 of 8 plans needed it, and in 2 of them the catch was the operator's review, not a
check.

### X4 — phase/research →research→ implement/code (and fix-loop) — 2 chunks
**Cases:**
- `2026-11-18-capability-statement`: the research witness-count note under-read and misattributed the gap. Research's
  record is `ok` with `graph-not-applicable` · `unresolved-questions`.
- `2026-11-18-served-error-page`: the files list omitted `examples/seven_guis/Cargo.toml` and `Cargo.lock`. The typed
  friction is `input.research-files-wrong`. Research's record is `ok` with `unresolved-questions`. Fix-loop also
  graded research `thin` there, with no note; that is the `ambiguity.scope-pressure` on the guard's allow set.

**Hypothesis:** These two producers each carried a signal, but neither signal was about the missed item. That is a
two-chunk pattern candidate. The served case is the manifest-edge shape: a crate that gains a dependency it reaches
through no re-export.

### X5 — implement/fix-loop →implement-outcome→ wrap-session/report-close — 2 chunks (different causes)
**Cases:**
- `2026-11-18-cold-agent-bar`: operator pass `as-left`. The outcome was superseded by the founder's F1–F5 answers
  between implement and the close, not by the pass.
- `2026-11-18-served-error-page`: operator pass `fixed` (`commits:2`, `red:5`). An android CI red on order.rs, a file
  implement wrote, was fixed by bc9be0469.

**Hypothesis:** Not one pattern. The first is a precondition change, the second a platform-only compile path. Both
are listed at k = 2 for visibility.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 3 facts (2 in-epoch, recurring from E5)
**Facts:**
- 2026-11-18-capability-statement · implement/fix-loop · process · workaround. `gate.py control` read the leg-count
  case INDETERMINATE: its grammar reads libtest, nextest and pytest lines, not unittest's FAIL form. The case was read
  by hand from the removed log.
- 2026-11-18-served-error-page · implement/fix-loop · process · workaround. `gate.py control` reads no unittest result
  line, so controls a–c printed INDETERMINATE. Each was read by hand.
- (E5) 2026-11-15-session-binary-smoke · phase/plan · process · workaround. The new unittest cases'
  red-before-green control cannot be a `gate.py control`. The plan records the reading against HEAD's script instead.

Typed correlate: P4's 3 unittest cases.

**Level hypothesis:** The cause lives in a pipeline tool (`gate.py control`'s case grammar). The fixes so far are
per-chunk hand readings and plan-side workarounds.

**Proposal:** The case-grammar extension in P4.

### L2 — band-aid — 3 facts (1 in-epoch, recurring from E5) · strengthened by 6 signal notes
**Facts:**
- 2026-11-18-served-error-page · phase/plan · process · removed-cause. The stamp-ahead hook refused the plan's
  Generated time, written as an estimate 7 min ahead of the clock. It was re-stamped from `date -u` (rev 2).
- (E5) 2026-11-16-error-field-in-the-answer · phase/plan · process · workaround: the same, 9 min ahead.
- (E5) 2026-11-17-a-component-in-isolation · phase/plan · process · workaround: the same, 6 min ahead.

**Strengthening:**
- This epoch's `phase/plan` step records carry `guard-refused:1` for the same refusal in 6 more chunks:
  answer-writer-by-family, stand-requirement-sweep, a11y-re-read, capability-statement, cold-agent-bar and
  cold-agent-live-run. With served-error-page, that is 7 of 8 plans.
- E5 also holds an untyped record of it (2026-11-15-taffy-from-upstream-git, plan).

**Level hypothesis:** The cause lives in the plan writer's instructions or template. The `Generated` field is filled
with an estimate rather than a clock read. The hook removes it each time, one plan at a time. This is also the
removed-cause observation: the same cause, removed in nearly every chunk, keeps returning.

**Proposal:** Have phase P4 fill `Generated` from a clock read, by a `date -u` step or by the put tool stamping it, so
the hook has nothing to refuse.

### L3 — band-aid — 2 facts (1 in-epoch, recurring from E5)
**Facts:**
- 2026-11-18-a11y-re-read · phase/research · process · workaround. The premise closure named a file the pinned scope
  did not (`key_press_after_container_lost_its_box.rs`). The row read was not re-run; it was read as the same named
  surface.
- (E5) 2026-11-17-layout-on-the-carriers · phase/research · process · workaround. The closure added `explore.rs` and
  `scripts/cold-agent.sh`, files the pinned scope did not name. The row read was not re-run whole; they were judged
  inside the scope's stated surfaces.

**Level hypothesis:** The phase gives no sanctioned path for a closure that widens the pinned scope by a file. Each
chunk judges it ad hoc.

**Proposal:** Give research a stated rule for this case: a delta row read over the added files alone, or a recorded
same-surface judgment with a fixed form.

### L4 — band-aid — 2 facts (1 in-epoch, recurring from E5) · nature resources
**Facts:**
- 2026-11-18-a11y-re-read · phase/plan · resources · workaround. The history files of six masters (about 130 KB) were
  read by a grep, not whole.
- (E5) 2026-11-17-a-component-in-isolation · phase/distill · resources · workaround. The obs, a11y and design rows
  files (28.5 + 25.9 + 9.4 KB) were not read whole, to hold the window.

**Level hypothesis:** The history and rows files outgrow a whole read inside one window. The skill's whole-read
expectation is bent per chunk.

**Proposal:** A sanctioned, recorded partial-read form, or a per-master digest the pickers write, so the read stays
honest and bounded.

### L5 — override — 2 facts (2 chunks, same rule) · with a band-aid workaround behind it
**Facts:**
- 2026-11-17-stand-requirement-sweep · wrap/reconcile · process · overridden. For CARRY cry-b3h6o7do, no comparer
  flagged the per-file test counts. The operator lifted the no-hand-edit rule for two lines once (inputs#I3) and
  directed the counts removed.
- 2026-11-18-capability-statement · phase/validate · process · overridden. The operator's review overrode the take-up's
  "leaves wait on a comparer". The CARRY cry-svjyrphi sites landed in the chunk through `rec.py harness land`.

**Behind them:**
- 2026-11-17-upstream-sync-ahead-of-polish-and-ship · wrap/reconcile · process · workaround: a refused leaf
  comparer's finding was written by hand through `rec.py harness land`.
- E5 `2026-11-15-upstream-sync-errors-and-explanation` · reconcile · workaround: the pass-mode comparers could not see
  two stale leaves, so they were written by hand.
- Their deferrals (answer-writer → stand-requirement-sweep; a11y-re-read → capability-statement) were routed and
  closed, so they are not deferred-forever.

**Level hypothesis:** The rule "a docs leaf changes only by a comparer finding" is miscalibrated for leaf statements
with no master source (P3). The operator keeps correcting the same call.

**Proposal:** A sanctioned disposition in the letter for sweep-listed leaf sites no comparer can reach. This is the
same change-shape as P3's second option.

### L6 — band-aid — 2 facts (1 in-epoch, recurring from E5)
**Facts:**
- 2026-11-17-upstream-sync-ahead-of-polish-and-ship · implement/code · process · workaround. A rename script wrote
  three `.claude/docs` harness renders directly; the write guard covers Edit/Write, not a Python write. They were
  reconciled with `rec.py render --family harness --reconcile` and landed through `rec.py harness land`. An untyped
  friction for the same event is in Below threshold.
- (E5) 2026-11-16-layout-explained · phase/take-up · process · workaround. The scope's citation edits went through a
  Python string replace and sed on the rendered working copy, in place of the Edit tool.

**Lookback strengthening:** E5's `recall.corpus-recurrence` cases on scripted edits firing no format hook
(2026-11-16-error-text-in-the-answer, 2026-11-17-layout-on-the-carriers).

**Level hypothesis:** The guards and hooks are bound to the Edit/Write tools, so a scripted write slips past them, and
bulk edits make scripted writes attractive.

**Proposal:** Bind the protected-family check to the land or put step (or a pre-commit-side check) rather than the
tool, so the path of the write no longer matters.

### L7 — chronic-degrade — `tooling.output-cap-overflow` across 5 epochs of 0.3.0, never halting
**Hits:**

| epoch | chunk | step | what |
|---|---|---|---|
| E1 | 2026-10-27-readme-and-records | report | `git diff --name-only` listed every run-dir file and passed the tool-result cap |
| E2 | 2026-10-27-upstream-sync-engine-debts | report | the pre-pass sweep's 73.9 KB output exceeded the Read cap |
| E3 | 2026-11-07-ui-graph | research | three graph queries printed 41 KB |
| E3 | 2026-11-08-what-can-i-do-here | research | an impact query returned 52.6 KB, its Read hitting the cap |
| E5 | 2026-11-17-layout-on-the-carriers | research | a verb-count `git grep -n` printed 248.7 KB |
| E6 | 2026-11-18-served-error-page | research | a served-slug `git grep` whose exclusions did not bind matched the store's ndjson journals, 1.9 MB |

The one implement/code `ok-degraded` in this epoch (upstream-sync) is a different cause (guard refusal, trial reuse).

**Level hypothesis:** Research's broad sweeps have no bounded form. Each time is recovered by a narrower re-run.

**Proposal:** A sanctioned two-step sweep form in research's reference: `git grep -c` per file first, then `-n` over
the named files, with the store's journals excluded by default. Or a sweep helper that does both.

### Deferred-forever — no hits
Two `deferred` facts and two `deferred` impacts fall in this epoch. Each named its destination entry, and each was
closed in that entry (L5's facts). There are no `deferral-open` signals.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/research — 2 cases (1 in-epoch, recurring from E5) → proposed type: none new — the existing Universal `contract.narrow-basis-claim`
**Cluster:**
- 2026-11-18-cold-agent-live-run · research (untyped): research's first put cited `scripts/cold-agent.sh` line numbers
  two lines early. A line-number grep before the scope edit caught it, and research was re-put at rev 2.
- (E5) 2026-11-15-session-binary-smoke · research (untyped): the agent-run sweep's hit count was written into research
  (39) before it was counted. The re-derive read 44 over 18 files.

**Neighbour:** In this epoch, 2026-11-18-served-error-page's research filed the same shape typed as
`contract.narrow-basis-claim`: hit counts written before any count command ran. So the type fits, and the gap is in
the research playbook's criteria.

**Draft criteria line (research playbook):** "A count, line number or site citation written into a research document
before the command that reads it ran — caught by a later re-derive — is `contract.narrow-basis-claim`."

## Below threshold — no action

**Typed (n < 3, no halt/soft-exit):**
- phase/research `contract.instrument-validity` ×2:
  - stand-requirement-sweep: the guard baseline was read on an untouched tree (see X1).
  - capability-statement: a witness-count regex under-read.
- implement/code `contract.premise-falsified` ×2 (inside P8).
- wrap/curation `recall.corpus-recurrence` ×2 (inside P9).
- wrap/reconcile `contract.in-pass-correction` ×2: a capability-statement close re-put; a cold-agent-bar set over the
  300-char cell.
- wrap/reconcile `retry.detector-respawn` ×2, by check: `respawn-ruled` 3 (a11y-re-read 1, served-error-page 2). This
  is a leaf-comparer receipt/anchor refusal, re-spawned RULED; the shape is P6(a)'s.
- wrap/report-close `input.implement-outcome-unsettled` ×2 (see X5).
- Singletons:
  - implement/code `input.research-files-wrong` (X4)
  - implement/fix-loop `ambiguity.scope-pressure` (guard allow set excluded Cargo.lock)
  - implement/fix-loop `contract.premise-falsified` (112 vs 97 carried, in P8)
  - implement/fix-loop `contract.vacuous-check-found` (the goal word "UI graph" also stood in the Cells sentence)
  - implement/fix-loop `input.plan-step-ambiguous` (the guard entry without `--allow`, X1)
  - implement/report `recall.change-reconstruction` (a re-entry after /clear authored an earlier run's report)
  - new-session/orientation `contract.schema-assumed` (a guessed friction view with `$.version` paths answered 0 rows)
  - phase/plan `contract.structural-blind-spot` (inside P7)
  - phase/research `contract.narrow-basis-claim` (U1's neighbour)
  - phase/research `tooling.output-cap-overflow` (L7)
  - phase/take-up `input.working-entry-thin` (an entry hint read as open work already done)
  - wrap/gates `recall.corpus-recurrence` (inside P9)
  - wrap/reconcile `contract.token-proxy-check` (the bash guard read a quoted `-->` as a redirect)
  - wrap/route-resolve `contract.carry-no-owner` (six stale leaf counts pinned to the nearest docs entry)
  - wrap/route-resolve `contract.no-sanctioned-channel` (`matrix.py release` cannot clear a CLEARED gated text)
  - wrap/route-resolve `contract.premise-falsified` (in P8)

**Untyped clusters (below F-4):**
- **Guessed store-view schema, answered silently empty or with a binder error — 2 untyped in-epoch:**
  - capability-statement research: a residual view probed with `$.record.*` paths.
  - capability-statement take-up: a guessed residual column, after a `head` pipe refusal.

  The existing type `contract.schema-assumed` fits both; orientation filed it typed. So this is a typing gap at
  research and take-up. Watch next epoch.
- **gh CLI repo flag — 1 untyped (capability-statement research: `gh api` rejects `-R`; log escape sequences).** There
  is also 1 problem fact (upstream-sync research, environment workaround: `gh api` has no `-R`, used the full repos
  path). n = 2 in-epoch, no prior.
- Singletons:
  - implement/code: harness-family renders listed as Files to modify with no note (L6's event).
  - implement/fix-loop: a control named construct.rs for a call in layout/mod.rs (X1).
  - wrap/reconcile: an Edit failed on an ambiguous anchor issued in the same message as the fit render, which rendered
    "no agent".
  - phase/validate: a snapshot origin line named the operator with a gendered pronoun no one had stated. It was
    re-snapped, and I2 stands uncited because `inputs.py` has no retire verb.

**Pass-A themes under threshold (in-epoch, workaround/removed-cause):**
- Host shell and guard mechanics, 2 environment facts:
  - upstream-sync research: a bare `cd` in a compound persisted the cwd.
  - stand-requirement-sweep validate: the recursive `rm` of a scratch clone was refused, and the clone left behind.

  The signal line's arm counts carry the same family (cd 17, pipe 10, heredoc 3, rm 3) as counts only.
- upstream-sync code: the plan put a 197 MB binary in evidence, past `rec.py land`'s 16 MiB bound.
- upstream-sync code: plan step order was swapped to run the manifest rename first (P5's first case).
- upstream-sync fix-loop: a control's named mutation was a call shared with `<img>`, so the SVG arm was removed
  instead.
- upstream-sync distill: a scope edit was held to P3 to avoid SCOPE_MOVED after a mid-P2 relay.
- stand-requirement-sweep validate: baseline `expect` spellings were aligned to the previous chunk's so they read seen.
  E3's mechanical-check (2026-11-04-dioxus-runtime-probe) records the same spelling-sensitivity. Watch.
- cold-agent-bar code: the contract tests were written before the pipe for red-before-green.
- upstream-sync gates (removed-cause): the light-gate probe's pattern was in the wrap's own text (P9's third case).
- capability-statement code and served-error-page code (product-logic workarounds): the witness method was controlled
  at an older commit; a `tracing` dependency and lock edge were added against the guard regex. These are not band-aid
  natures.

**Overridden facts not recurring on one rule:**
- cold-agent-bar route-resolve: F5 overrode a CLEARED gated criterion.
- cold-agent-live-run fix-loop ×2: entry 12 ruled surfaced, then re-entered alone (inputs#I4, #I5).
