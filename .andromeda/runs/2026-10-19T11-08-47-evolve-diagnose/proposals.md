# Evolve Diagnosis — escher-0.2.0 · Epoch 3 — Determinism and traces · 2026-10-19T11:12:54Z

Scope: the epoch's four chunks — 2026-10-18-upstream-sync-determinism-and-traces · 2026-10-18-one-virtual-clock ·
2026-10-19-session-recording · 2026-10-19-recorded-check-replay — all `complete` in master-route. Evidence twins
beside this file: `q-retractions.json` · `q-health.json` · `q-typed.json` · `q-untyped-raw.json` · `q-untyped.json` ·
`q-problems-raw.json` · `q-problems-lookback-raw.json` · `q-untyped-lookback-raw.json` · `q-chains.json` ·
`q-chains-joined.json` · `q-level.json`; the pass script `analyze.py`.

## Mechanism health
- **Records:** 84 in-epoch (56 step / 28 friction) of 1,362 in the ledger · unparseable 0 · malformed-`ts` 0 · id fill 84/84.
- **Coverage per chunk:** every chunk carries phase 5/5 · implement 3/3 · wrap 5/5. No gaps. new-session/orientation: 4 records with `chunk: null`.
- **Untyped rate per step:** 3 of 28 friction records are untyped: phase/plan 1/1, implement/code 1/5, wrap/report 1/3. Every other step has 0 untyped.
- **Problem-fact fill:** 7 of 56 step records carry facts, 9 facts in all: 6 workaround · 3 overridden · 0 prohibition · 0 removed-cause · 0 deferred.
- **Outcomes:** 54 ok · 2 `halted-resolved` (both at reconcile: the epoch's boundary-widening cards) · 0 ok-degraded.
- **Signal line:** guard-refused 18 over 56 step records · exact-hit 10 over 4 curation records.
  - guard-refused by step: reconcile 5 · implement/code 4 (one in every chunk) · distill 3 · curation 2 · plan, validate, take-up and smoke 1 each.
- **Retractions:** 0 retracted · 0 clause-retracted · 0 retraction-of-retraction, all in-epoch. The whole ledger holds one unresolvable entry, outside this epoch, reported verbatim for the founder's manual discount:
  - Record `2026-10-07T04:21:25Z-a` (2026-10-07-audit-corrections), reason `unknown-id-or-scope`: `{"id": null, "note": "in step record 2026-10-07T04:21:04Z-a (a step record, so no record or clause scope applies), produced git-state note: discount 'add -A staged 43 paths' — the measured count is 47 files changed"}`
- **Calibration boundaries in range:** the epoch is dated 2026-10-18 to 2026-10-19, after every deploy date the pass names. These fields are all live for the whole range:
  - the deviation scan, `id`/`retracts`, the universal types and `contract.in-pass-correction`
  - the `operator-pass` entry and `implement-outcome`'s 2026-10-13 grading
  - `new-text-*` and the rejected-for counts
  - `guard-refused`/`exact-hit` and the widened ladder gloss

  No era reading applies.

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 4 cases · weight 8
**Pattern:** a master or run-dir line written during reconcile was wrong at first write in 3 of 4 chunks (4 of 4 reconcile runs), and each was caught by a re-read or by classify before it propagated. Two of the four are provenance-shape errors the classification contract keeps out of a body line.
**Evidence:** all 4 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-18-upstream-sync-determinism-and-traces | architecture.md:233 first written with a cause («wait for a Taffy release») the report does not carry; a re-read against the report caught it before the sweep | iterations 1 | — (`.andromeda/architecture.md:233`) |
| 2026-10-18-upstream-sync-determinism-and-traces | design-system.md:316 first written with the retired value in the body, which is provenance the classification contract keeps out; moved to the sidecar entry's Change on a re-read | iterations 1 | — (`.andromeda/design-system.md:316`) |
| 2026-10-18-one-virtual-clock | citation-dispositions.md's closing line said «Claim-false rows raised at Validate (10)» over a list of 9; corrected on a re-read before the fan-out | retries 1 | — (`runs/2026-10-18T08-45-11-wrap/citation-dispositions.md`) |
| 2026-10-19-session-recording | three in-line edits at architecture.md:182/325 changed the anchor of a standing provenance parenthesis; classify's standing-FAIL provenance check (5 spans) caught them, re-shaped before the sidecar entries | retries 1 | `runs/2026-10-19T05-20-24-wrap/classify-2026-10-19-session-recording.json` |

**Lookback (read-only):** the group fired in every epoch on record: 0.1.0 E1–E6 7 · 4 · 7 · 9 · 10 · 3; 0.2.0 E2 4; this epoch 4. It is a standing per-wrap cost, not a spike.

**Proposal:** move the checks that catch these to before the write. classify's provenance arm, plus a count-against-list check, would run over a *candidate* line before it is applied: a pre-apply lint in cascade.py or classify, fed the line and its neighbours.

- Cases 2 and 4 are exactly what classify already flags after the fact. Case 4's shape also recurs as P2's third case.
- Case 3 is a stated count read against the list beside it.
- Case 1 (a cause the report does not carry) needs a report-grounding read, which stays a judgment step.

### P2 — recall · `recall.corpus-recurrence` (by type, across steps) — 3 cases · weight 6
**Pattern:** a hazard the session-learnings corpus already states recurred three times in two chunks: twice at curation and once at reconcile. One of the three recurred in the chunk right after the entry was written.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-19-session-recording | the gh api job-log escape-sequence refusal recurred at the implement pass (a fetch read 0 failures from a refusal text) past the 2026-10-11 T3 entry stating its remedy | retries 1 | `escher-0.2.0/chunks/2026-10-19-session-recording/report.md` |
| 2026-10-19-session-recording | a bare-digit sweep pattern (`\b134\b`) matched 26 line-range citations at P1 past the 2026-10-12 T3 entry on bare-count patterns | iterations 1 | `runs/2026-10-19T05-20-24-wrap/cascade-2026-10-19-session-recording-pre-pass.json` |
| 2026-10-19-recorded-check-replay | obs-plan.md:70: inserting `stand_replay` just before a provenance parenthesis made classify read it as new provenance — the 2026-10-19 Tier-3 entry states this exact trap | retries 1 | `runs/2026-10-19T09-49-14-wrap/replace-5.md` |

**Lookback:** the type is chronic, with 2–16 per epoch since 0.1.0 E1. The bare-count hazard alone also recurred at 0.1.0 E6 (2026-10-14-stand-a11y-assertions, `\b153\b`).

**Proposal:** each of the three hazards is mechanical, so a tool can catch it where it is made, instead of the agent having to recall a prose rule:

- cascade.py sweep would warn on (or refuse) a pattern that is a bare `\b\d+\b`.
- The gh job-log fetch would go through a wrapper that tells a refusal text apart from a log.
- The provenance-anchor trap is P1's pre-apply lint.

A generalisation for the founder's judgment: a T3 learning that recurs at n ≥ 2 becomes a candidate for a tool guard rather than a corpus extension.

### P3 — phase/validate · `contract.mechanical-check` — 3 cases · weight 6
**Pattern:** a Validate check failed on an artefact the pass itself wrote, in 2 chunks (3 of 4 validate runs). Two of the three are one site class: a CARRY folded verbatim into `scope.md` cites another chunk's `inputs#In` token, which check 10 then reads against this chunk's inputs.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-18-upstream-sync-determinism-and-traces | check 10 (inputs.py verify) listed scope.md:28 citing this chunk's I4 where the folded CARRY 3 cites 2026-10-18-engine-debts' inputs#I4; the fold had copied a foreign token, re-spelt by path at P5 | iterations 1 | `runs/2026-10-18T05-47-23-phase/inputs-2026-10-18-upstream-sync-determinism-and-traces.json` |
| 2026-10-19-recorded-check-replay | check 10 read UNPARSED inputs#I4 at scope.md:19: the verbatim-folded CARRY cites another chunk's snapshot by its inputs id; re-pointed to that chunk's file path and re-verified clean | retries 1 | — (`scope.md:19`, `working-route.md:29`) |
| 2026-10-19-recorded-check-replay | check 8's reach read found the plan's MCP first-activation index written as 4 where the script's steps give 3; no predicate counts a script's steps; corrected before the review | retries 1 | — (`plan.md`) |

**Site count (per diagnosis-pass, a universal type):** the folded-CARRY `inputs#In` site fired n = 2 across 2 chunks. That points at the CARRY writer or the fold, not the data. The working route's open Epoch 4 CARRY still writes «its inputs#I3 … its inputs#I4» in the bare form.

**Lookback:** the type is chronic, with 2–10 per epoch since 0.1.0 E1.

**Proposal:** fix the site class at its writer. Either of these would remove the class:

- route-resolve writes a CARRY's foreign input citations chunk-qualified (by path), or
- take-up's fold rewrites a foreign bare `inputs#In` to its path form, or inputs.py verify accepts a chunk-qualified token.

Case 3 (a stated count against a listed sequence) is the same count-vs-list shape as P1's case 3, which suggests one shared predicate.

### P4 — wrap-session/reconcile · `contract.cascade-miss` — 3 cases · weight 5
**Pattern:** standing restatements of a fact the chunk retired were in no detector's proposals and no comparer's findings. In 2 chunks the step-2 sweep, or a re-fired sweep, caught them, and they were amended by hand.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-19-session-recording | three standing master restatements of the v2 wire and the bare host-role spawn (architecture.md:182, :220; test-plan.md:144) no detector proposed; caught by the step-2 sweep, amended in the pass | iterations 1 | `runs/2026-10-19T05-20-24-wrap/cascade-dispositions.md` |
| 2026-10-19-recorded-check-replay | architecture.md:193 and security-plan.md:125 restated «no log, event or file carries a diff», retired with the snapshot-text claim; no detector proposed them, the step-2 sweep's no-file pattern caught both | iterations 1 | `runs/2026-10-19T09-49-14-wrap/cascade-dispositions.md` |
| 2026-10-19-recorded-check-replay | seven_guis.md:38's «byte-identical in two child runs and in the parent» was in no comparer finding; the re-fired sweep's shot-ident leaf row caught it, written by hand | extra_reads 1 | `runs/2026-10-19T09-49-14-wrap/cascade-dispositions.md` |

**Correlates (same family, below threshold alone):**
- `input.report-insufficient` at 2026-10-18-one-virtual-clock: the P1 site sweep reached 0 rows for six moved facts its wording did not name.
- reconcile's thin `drift-base` reading at 2026-10-18-upstream-sync-determinism-and-traces: 15 detectors, none bound to a count, tally or sync-record fact.

**Lookback:** chronic since 0.1.0 E3 (4 · 6 · 4 in 0.1.0 E4–E6).

**Proposal:** derive the P1 report's sweep patterns from what the chunk *retires*: the old wire token, the replaced claim phrase, the old count. That way a retired fact's restatements surface at P1 instead of at the step-2 sweep, and detectors keyed to retired-claim tokens would cover the gap the drift-base reading names. The step-2 sweep already catches every case, so the cost is one iteration per chunk, not a miss.

### P5 — implement/code · `input.plan-step-ambiguous` — 3 cases · weight 3
**Pattern:** in 2 chunks implement met plan steps that contradict another step, leave an edge case undecided, or name something whose name changes behaviour. Each was settled by implement's own judgment and surfaced.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| 2026-10-19-session-recording | plan step 8 states help's line byte-identical to before the chunk, but step 5's 13→15 session errors are served by describe(), so help's line must change; built step 5, surfaced in the report | iterations 0 | — (`plan.md`) |
| 2026-10-19-session-recording | plan steps 3–4 do not say what a recording keeps of a call no record can spell, or of a refused type whose control cannot be read; chose: not kept, and kept masked; both surfaced | iterations 0 | — (`plan.md`) |
| 2026-10-19-recorded-check-replay | plan step 10 named the test-only variant VariantCounter, but a component's name enters every unkeyed element's id, so the checks would go red at the first snapshot; settled by reading element_id.rs | extra_reads 1 | — (`plan.md`, `element_id.rs`) |

**Correlate:** `contract.test-expectation` at 2026-10-19-session-recording fix-loop is the same «unchanged» claim. The plan listed `stand_session_lifecycle.rs` as unchanged while the wire moved v2→v3, and four gates went red. Together with case 1, that is two «unchanged / byte-identical» plan claims contradicted by the chunk's own steps.

**Lookback:** chronic, with 1–9 per epoch since 0.1.0 E1.

**Proposal:** add a planlint or Validate predicate: every «unchanged», «byte-identical» or «no change» claim on a file or output is read against the steps that change what that file or output is built from. That covers cases 1 and the correlate.

- Case 3 is project-local knowledge: a name enters stable ids. A project rule line in `.claude/rules/` for plan authors may fit it better than a pipeline change.
- Case 2 sits within implement's designed judgment.

## Cross-step chains (starting heuristics)

### X1 — implement/smoke →implement-outcome→ wrap-session/report — 2 chunks
- **Both ends:**
  - 2026-10-19-session-recording: producer `2026-10-19T04:09:44Z-a` (ok, `green`) → consumer `2026-10-19T05:27:08Z-a` (thin; operator-pass `fixed · commits:3 · red:2`).
  - 2026-10-19-recorded-check-replay: producer `2026-10-19T08:44:05Z-a` (ok, `green`) → consumer `2026-10-19T09:55:54Z-a` (thin; operator-pass `fixed · commits:2 · red:1`).
  - The consumer frictions are `2026-10-19T05:27:08Z-b` and `2026-10-19T09:55:54Z-b` (`input.implement-outcome-unsettled`, n=2, below threshold alone).
- **Hypothesis:** the operator pass's `fixed` supersedes a formally-green implement both times, and both reds were per-OS:
  - Windows CI: an ungated recorded test, then a de-gated helper calling a unix-only fn.
  - macOS CI: a shot's PNG bytes differ by OS.

  Implement's gate runs on one Linux host. Platform-gated code (`cfg(unix)`) and byte-compared committed artefacts pass there and go red elsewhere.
- **Direction:** when a chunk's diff adds or moves a platform gate, or commits a byte-compared artefact, implement's gate would add a cross-target compile reading (e.g. a `cargo check`/clippy for a Windows target), or the plan would name a per-OS reading as an entry.

### X2 — phase/plan →plan→ implement/code — 2 chunks
- **Both ends:**
  - 2026-10-19-session-recording: producer `2026-10-19T03:18:48Z-a` (ok; `planlint-hit-resolved:12c`, two designed dialogues) → consumer `2026-10-19T03:49:26Z-a`, frictions `-b`, `-c`.
  - 2026-10-19-recorded-check-replay: producer `2026-10-19T08:09:01Z-a` (ok; `unresolved-questions`, two designed dialogues) → consumer `2026-10-19T08:31:04Z-a`, friction `-b`.
- **Hypothesis:** this is P5's lineage. Both plans passed planlint and Validate formally `ok` and fed a downstream `input.plan-step-ambiguous`. Each consumer also graded the plan `quality: ok` in the same record that filed the friction against it, so the consumer verdict and the friction disagree.
- **Direction:** P5's predicate, plus a consistency rule at the code checkpoint: a record that files an `input.*` friction against an artefact grades that artefact no better than `thin`.

## Level candidates (systemic-masked-as-project)

### L1 — override — 2 facts in-epoch (recurring from the prior epoch)
**Facts:**
- 2026-10-18-one-virtual-clock · phase/plan · process · overridden: «the operator's directive (inputs#I1) routed the founder's forks to one founder-forks.md in the run dir with the turn ended, in place of P4's AskUserQuestion round; the delegate's answer arrived by message and was snapped as inputs#I2» (`2026-10-18T07:41:31Z-a`).
- 2026-10-19-session-recording · phase/plan · process · overridden: «the operator directed that founder forks go to one founder-forks file answered by a delegate, PROVISIONAL, in place of the P4 dialog (inputs#I1)» (`2026-10-19T03:18:48Z-a`).
- Same-epoch signals:
  - 2026-10-18-upstream-sync-determinism-and-traces plan: `designed-dialogue:… a founder fork answered by the operator as delegate through founder-forks.md`.
  - 2026-10-19-recorded-check-replay plan: `… founder F1, one round through the overseer`.

  That puts the route in use in 4 of the 4 chunks.
- Lookback:
  - 0.2.0 E2 2026-10-18-engine-debts plan: overridden, the same directive.
  - 0.2.0 E2 2026-10-17-upstream-sync-engine-debts plan: workaround, «the AskUserQuestion round went unanswered for 58 min through the driver's dialog hook».
  - 0.1.0 E5 2026-10-10-driver-cli plan: overridden, forks to one file answered by relay. An untyped curation record of the same epoch has two dialogs expiring unanswered after 58 minutes.

**Level hypothesis:** the operator keeps overriding one rule, P4's founder-fork question dialog, the same way across chunks and epochs. This points at the rule rather than the project. In the overseer-pair topology the founder is reached through an operator in another session, and an in-session dialog either expires or gets replaced by the file route.
**Proposal:** phase P4 would carry the founder-forks file as a sanctioned arm alongside the dialog: one run-dir file, the turn ended, the answer snapped as an input, and PROVISIONAL marking built in where a delegate answers. It would be selected when the session runs under the overseer pair, so the route stops being an override.

### L2 — band-aid — 1 fact in-epoch, recurring in each of the two prior epochs
**Facts:**
- 2026-10-19-recorded-check-replay · wrap-session/reconcile · process · workaround: «cites.py apply's stdout was sent to /dev/null against the letter; its listing and counts were recovered from the trail's first applied record» (`2026-10-19T10:47:19Z-a` #0).
- Lookback, the same act:
  - 0.2.0 E2 2026-10-17-upstream-sync-engine-debts reconcile: «the citation sweep's writing call had its stdout sent to /dev/null against Setup 3's letter».
  - 0.2.0 E1 2026-10-16-file-splits reconcile: «the cites.py apply call's stdout was sent to /dev/null against the tools rule».

  That is once per epoch for three consecutive epochs.

**Threshold reading:** n = 1 in-epoch. It is counted at threshold under «n ≥ 2 recurring from a prior epoch», across the target and prior epochs. The founder may read it as below threshold instead.
**Level hypothesis:** the same deviation at the same call, absorbed by the same recovery (read the trail) each time. Something at that call invites discarding the output, such as its size or its trail duplicating it, and the fix so far lives in each chunk's recovery.
**Proposal:** remove the occasion at the tool. cites.py apply would write its listing to the run dir itself and print a short count line, or the skill's letter would name the trail as the canonical read. Either way, discarding stdout stops being a deviation.

### L3 — band-aid — 2 facts in-epoch (recurring from the prior epoch)
**Facts:**
- 2026-10-18-one-virtual-clock · wrap-session/reconcile · process · workaround: «67 point amendments on master lines up to 49 KB applied by one scratch python script of exact-match replacements (each asserted to occur once, dry run first) instead of 67 anchored Edit calls» (`2026-10-18T09:23:19Z-a` #0).
- 2026-10-19-session-recording · wrap-session/reconcile · process · workaround: «the architecture subject line is a comment line cascade.py replace preserves; edited it with the Edit tool after a ranged Read» (`2026-10-19T06:41:41Z-a` #1).
- Same-epoch friction correlates: `2026-10-19T10:47:19Z-d` (leaves --apply read anchor moved; four rows written by hand) and `2026-10-19T10:47:19Z-e` (a row written by hand).
- Lookback: 0.2.0 E2 2026-10-17-upstream-sync-engine-debts reconcile, «an anchored Edit … missed; the bullet was inserted by a scratch script». In the same epoch an untyped record has leaves --apply refusing 2 of 35 findings over an escaped backslash.

**Level hypothesis:** reconcile's write path keeps routing around its own amendment tools: bulk point amendments, comment lines, and anchors that moved. Each chunk rebuilds a scratch equivalent.
**Proposal:** cascade.py would gain the shape the scratch scripts already proved: a bulk exact-match point-amendment mode (each match asserted once, dry run first), comment-line edits, and a re-anchor on a moved line. The workaround would then become the tool.

No hits for the other signatures:
- chronic-degrade: 0 `tooling.*` frictions and 0 ok-degraded outcomes in-epoch.
- deferred-forever: 0 deferred facts and 0 `deferral-open` signals.
- removed-cause: 0 facts.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — phase/plan — 1 case in-epoch, recurring from the previous epoch → proposed type `contract.stamp-estimated`
**Cluster:**
- In-epoch: `2026-10-19T08:09:01Z-b` (2026-10-19-recorded-check-replay, phase/plan, untyped). «plan.md's Generated stamp was written as an estimated 2026-10-19 08:20Z without reading the clock; the stamp-ahead PostToolUse hook refused it (clock 08:08:44Z) and it was corrected by one Edit».
- Previous epoch (0.2.0 E2), all the same shape:
  - 2026-10-17-upstream-sync-engine-debts phase/plan, untyped: a control-reading time estimated, refused 3 min ahead.
  - 2026-10-18-engine-debts reconcile, typed `contract.in-pass-correction`: relay stamps estimated, refused by inputs.py snap.
  - 2026-10-17-upstream-sync-engine-debts route-resolve, a problem fact: a hand-typed `ts` ahead of the clock.
- Earlier (0.1.0 E6): five more untyped records of the shape, at plan ×2 and validate ×3.
- This pass's own run tripped the same hook once on a quoted past time; it is noted here, not counted.

**Draft criteria line:** «`contract.stamp-estimated` — a time written into an artefact (a Generated stamp, an `--origin`, a relay stamp, a record `ts`) from an estimate rather than a clock read, refused or flagged by the stamp-ahead hook or inputs.py.» The owner would be the phase playbook, with the type usable at any step.

A pipeline-level direction for the founder's judgment: the plan template's Generated stamp, and `--origin`, would be filled by the tool from `date -u`, which removes the occasion.

## Below threshold — no action
**Typed groups (per step):**
- wrap/report `input.implement-outcome-unsettled`: 2, no halt or soft-exit. This is X1's consumer friction.
- wrap/curation `recall.corpus-recurrence`: 2. Counted in P2 by type.
- wrap/reconcile `recall.corpus-recurrence`: 1. Counted in P2.
- wrap/reconcile `retry.detector-respawn`: 1. A leaf comparer wrote a 44-line read count for a 43-line leaf; re-spawned once.
- implement/code `input.research-files-wrong`: 1. Two escher-driver unit tests outside research's lists pinned the old cause count.
- wrap/report UNTYPED: 1. Per-master counts hand-tallied (U-cluster below).
- implement/code UNTYPED: 1. An inline python heredoc broken by nested triple quotes (U-cluster below).
- implement/fix-loop `contract.test-expectation`: 1. P5's correlate.
- phase/plan UNTYPED: 1. U1.
- wrap/reconcile `input.report-insufficient`: 1. P4's correlate.
- wrap/route-resolve `contract.carry-no-owner`: 1. A coverage gap pinned to the earliest markerless entry able to take it.
- wrap/reconcile `contract.proposal-format`: 1. A leaf comparer cited lines past the files' ends.
- wrap/reconcile `contract.premise-falsified`: 1. The resume directive's «each OS» against a `#![cfg(unix)]` file.

**Universal types by type alone:** `contract.test-expectation` 1 · `contract.carry-no-owner` 1 · `contract.proposal-format` 1 · `contract.premise-falsified` 1.

**Untyped clusters:**
- Inline-heredoc quoting: 1, no previous-epoch member. host-linux.md §Transports already states the rule.
- Hand-tallied per-master counts: 1, no previous-epoch member.

**Chains:**
- phase/research →research→ (implement/code | phase/plan): two chunks, but two different consumers, so it is not one shape. phase/research signalled `unresolved-questions` and a `premise-corrected` scope in 4 of 4 chunks, so neither signal separates the 2 thin research files from the 2 ok ones.
- wrap/report →report→ wrap/reconcile: 1 chunk (2026-10-18-one-virtual-clock).
- Thin with no in-chunk producer:
  - `drift-base` at reconcile (2026-10-18-upstream-sync-determinism-and-traces).
  - `working-entry` at take-up (2026-10-19-session-recording).

**Problem-fact themes:**
- The classify provenance arm vs in-line edits beside a provenance parenthesis: 1 fact, product-logic. Carried by P1 and P2.
- A sweep that lacks the breakdown the report needs, met by a scratch reader: 1 fact, no prior-epoch fact.
- A plan name that enters stable ids: 1 fact, product-logic. Carried by P5.
- An operator directive's premise contradicted by the file: 1 fact.
