# Evolve Diagnosis — escher-0.1.0 · Epoch 3 — Observation model · 2026-10-07T02:09:40Z

Every proposal below is obligation-free: accept, reject, defer or modify — nothing here is applied, queued or
remembered. Each is a direction for the founder's judgment, never a verdict.

**How to read the citations.** `L340` is line 340 of `.andromeda/friction-log.ndjson`; `L345#0` is the first
problem fact of the step record on line 345. Chunks are named by their marker without the date prefix
(`2026-10-06-…`, and `2026-10-07-` for change-tracking-and-diff). "Lookback 7 · 4 · 7" gives the same group's
count in Epochs 1 · 2 · 3 — recurrence context only; every proposal rests on Epoch 3's records. The raw query
outputs sit beside this file as `q-*.json`.

## Mechanism health

Observations only.

- **Records:** 151 (84 step / 67 friction) over 6 chunks, plus 9 chunkless records (6 session-start
  orientation steps and their 3 frictions). Unparseable lines: 0 of 458 in the whole ledger. Malformed `ts`: 0.
- **Coverage:** complete. Every chunk carries phase 5/5 · implement 3/3 · wrap 5/5; 6 orientation records for 6
  session starts. No checkpoint failed to fire.

  | chunk | phase | implement | wrap | gaps |
  |---|---|---|---|---|
  | upstream-sync-observation-model | 5 | 3 | 5 | none |
  | snapshot-model | 5 | 3 | 5 | none |
  | id-stability-across-code-edits | 5 | 3 | 5 | none |
  | snapshot-state-fidelity | 5 | 3 | 5 | none |
  | compact-snapshot-serialization | 5 | 3 | 5 | none |
  | change-tracking-and-diff | 5 | 3 | 5 | none |

- **Untyped rate:** 4 of 67 (6%) — one each at phase/take-up (1 of 2 frictions), implement/code (1 of 7),
  implement/fix-loop (1 of 3) and wrap/report (1 of 5); zero at the other ten steps. Lookback: 9 of 80 in
  Epoch 1, 10 of 58 in Epoch 2. The shapes that were untyped in Epoch 2 (an estimated stamp, the leading-cd
  guard, a scripted edit bypassing the format hook) are recorded typed in Epoch 3.
- **Problem-fact fill:** 27 of 84 step records carry facts — 34 facts (30 workaround, 2 removed-cause,
  2 overridden, 0 deferred, 0 prohibition).
- **Id fill:** 151 of 151; no duplicate ids. **Evidence pointers:** 26 of 67 friction records carry one; the 22
  that are paths all resolve on disk, 4 are prose.
- **Retractions (whole-ledger scan):** retracted 0 (0 unresolvable) · clause-retracted 1 (kept; its target is
  an Epoch 1 record, so no table below renders the note) · retraction targeted by retraction: 0.
- **Skill spelling is split inside the epoch:** 36 records carry the long form (`andromeda-phase` 20,
  `andromeda-wrap-session` 11, `andromeda-new-session` 5) and 115 the bare form (`wrap-session` 56,
  `implement` 28, `phase` 27, `new-session` 4). Folded to the bare form before every count here. Two records
  speak to the cause: L449 (no reference the implement skill reads states the envelope's spelling, and
  report-template.md's filter on the long form misses the six bare `implement` records of that chunk) and
  L442#2 (a writer tried to read the ledger's tail to copy the spelling, against the never-read rule). The
  epoch label has one spelling across all 151 records.
- **Record order:** one friction record (L388, implement/fix-loop) was appended after its chunk's smoke
  record — it was written during the operator pass, 25 minutes after the fix-loop step it names. The three implement
  step records of upstream-sync-observation-model share one `ts` (18:08:16Z).
- **Outcomes:** 81 ok · 3 halted-resolved · 0 ok-degraded (lookback: 2 and 3 ok-degraded in Epochs 1 and 2).
- **Calibration boundaries in range:** none of the dated boundaries falls inside it — the ledger's first
  record is 2026-10-05, after the `id`/`retracts` boundary (2026-08-18), the Universal types' deploy
  (2026-09-08/09) and `contract.in-pass-correction`'s (2026-09-27); problem facts are present from the
  ledger's first record. Two era notes specific to this range: `retry.distiller-respawn` first appears in this
  epoch (0 · 0 · 4) — whether the type or its check is new the ledger cannot say; and L429 records the
  deployed pipeline being rewritten under a running wrap (matrix v1.3 → v1.4, the wrap skill body,
  route-resolve.md), so the last chunk ran on a newer pipeline than the first five.

## Proposals (typed patterns)

Ten groups meet the threshold (n ≥ 3 in-epoch; no group qualifies through the n ≥ 2 halt arm). Sorted by
n × summed weight, weight = 1 + iterations + retries + reformulations + 2·dialogue_rounds + 3·halted +
3·soft_exit per record. P2 and P3 are grouped by type alone across steps (`recall.*` and a Universal type).

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 7 cases · weight 17

**Pattern:** every reconcile run of the epoch recorded at least one first write corrected before the commit
(7 records, 10 sites, 6 of 6 chunks; rate 7 per 6 runs); 6 of the 10 sites are a number or a claim written
before it was measured, the other 4 are form or wording slips caught on read-back.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-observation-model · L322 | the design-system raw fan-out twin was first written with the detector's absolute doc path shortened to a relative one — a rewrite of a verbatim twin; restored on re-reading the never-rewrite rule | retries 1 | artifact: `.andromeda/runs/2026-10-06T18-09-10-wrap/.raw-fanout-design-system.md` |
| snapshot-model · L340 | 2 sites: a security-plan row cited snapshot.rs:105-153, a re-read gave 105-151; a test-plan clause first carried an unmeasured cause, rewritten to the bare fact | iterations 2 | artifact: `.andromeda/security-plan.md:107`, `.andromeda/test-plan.md:96` |
| id-stability-across-code-edits · L366 | 2 sites: the design-summary leaf read "each task's six inputs" where the stand has six in all; cascade-dispositions.md stated rows had been read at both offsets before those reads were run | retries 2 | artifact: `.claude/docs/design-summary.md`, `cascade-dispositions.md` |
| snapshot-state-fidelity · L394 | 2 sites: an a11y-plan citation range written 309-346, measured 308-344; a test-plan citation written inline and moved to the sentence's citation list | retries 2 | artifact: `.andromeda/a11y-plan.md`, `.andromeda/test-plan.md` |
| compact-snapshot-serialization · L422 | a correction note in report.md carried an estimated time four minutes ahead of the clock; the stamp hook refused the write | retries 1 | artifact: `report.md` |
| compact-snapshot-serialization · L423 | the Applied tally in fanout-results.md first read 19 routine; a per-doc recount (9 + 4 + 5 routine, 4 escalated, 2 re-raised, over 24) replaced it — the 19 had already been said to the operator in the escalation question | retries 1 | artifact: `fanout-results.md` |
| change-tracking-and-diff · L452 | the architecture sidecar entry was appended at 3068 B, over the 3000 B form cap: the append loop ran after the check without being gated on its verdict; trimmed to 2984 B in payload and sidecar | retries 1 | artifact: `.andromeda/architecture-amendments.md`, run-dir `architecture-entry.md` |

**Proposal:** the type records the mechanism working (each was caught before the commit), so the open question
is the rate, which holds across the ledger (lookback 7 · 4 · 7). Two directions the sites suggest: (1) the
three numeric kinds — a citation range, a tally, a stamp — enter a reconcile document from a tool's printed
output rather than from the agent's typing (L7 covers the range, L8 the stamp); (2) the sidecar
append is gated on the form check inside the tool (an append that refuses over the cap), which would remove
L452's ordering slip by construction.

### P2 — wrap-session/curation + reconcile · `recall.corpus-recurrence` — 7 cases · weight 14

**Pattern:** in 5 of 6 chunks a learning already curated into Tier 3, or carried as deferred in the handoff,
recurred — 7 records (5 at curation, 2 at reconcile) naming 9 recurrences of 7 distinct entries;
three records state the mechanism themselves: the entry "was not consulted before" the act (L425, L426) and
"session-learnings.md is on-demand and no implement step reads it" (L455).

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L342 · reconcile | two deferred Tier-3 learnings recurred in the wrap: a leading cd refused by the Bash guard (once), and ugrep rejecting a bounded repetition `{0,130}` on five patterns (re-run through python re) | retries 2 | artifact: `.claude/session-handoff.md` |
| id-stability-across-code-edits · L367 · reconcile | a cat heredoc with a file target was fired to write a helper script and the Bash guard refused it; the handoff already carried this as a recurrence-despite-learning entry | retries 1 | artifact: `.claude/session-handoff.md` |
| id-stability-across-code-edits · L369 · curation | the wrap's typed citation tally matched the curated Tier-3 entry "Count from the listing you just read, never from the plan's forecast" — the report draft reproduced the failure with a number from no listing at all | retries 0 | artifact: `.claude/docs/session-learnings.md` |
| snapshot-state-fidelity · L397 · curation | the Tier-3 entry on a chunk that moves cited source lines was in the corpus; the wrap's report still listed the citing sites of one shifted file and not of the other | extra_reads 1 | artifact: `.claude/docs/session-learnings.md`, `report.md` |
| compact-snapshot-serialization · L425 · curation | a Tier-3 entry already says to key a line-map search on full repository paths, never basenames; the report keyed on a basename that is the tail of another file's name and miscounted 5 citations; the entry was not consulted before the search was written | retries 1 | artifact: `.claude/docs/session-learnings.md`, `report.md` |
| compact-snapshot-serialization · L426 · curation | a Tier-3 entry already says a hit is read at its match offset before it is dispositioned; the report turned 6 hits into 6 statements of the retired claim without reading each, and 2 were another subject's; the entry was not consulted before the count was written | retries 1 | artifact: `.claude/docs/session-learnings.md`, `report.md` |
| change-tracking-and-diff · L455 · curation | two Tier-3 entries that state their rule correctly recurred at implement: a scripted Rust edit not followed by cargo fmt (one red fast leg), and a heredoc with a file target refused by the guard (one retry); neither was consulted before the act | retries 2 | artifact: `.claude/docs/session-learnings.md` |

**Proposal:** the records point at delivery, not content — L455 says the entries state their rule correctly.
Directions: a step-keyed read (a step's setup loads the Tier-3 entries tagged for that step, so the entry
is in the window when the act happens); a promotion rule (an entry with a recorded recurrence moves to a tier
that loads without being asked for); or, for the entries with a mechanical form, a guard instead of a
sentence — L1, L3 and L4 name three of the seven entries (the Bash guard's forms, grep on this host, format
after a scripted edit). The project's absorption so far is the handoff's deferred-learnings list; the group
has not fallen (lookback 5 · 7 · 7).

### P3 — phase/research + wrap-session/report + implement/fix-loop · `contract.narrow-basis-claim` — 5 cases · weight 13

**Pattern:** 5 records at three steps (research 2, report 2, fix-loop 1) in 4 chunks; four are a count or a
line number written from recall or a hand count and corrected in-step from a numbered listing, the fifth
(L388) is a platform claim resting on a Linux-only run that reached CI red.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| id-stability-across-code-edits · L364 · report | the first draft of report.md carried a citation tally typed without a count (88 found, 41 change, 47 unchanged) and a Date five minutes ahead of the clock; the stamp-ahead hook refused the date, and the tally re-derived from the line-map script read 96, 61, 35 | retries 1 | artifact: `report.md` |
| snapshot-state-fidelity · L388 · fix-loop | the Backspace step added at the fix-loop was checked by a grep hit on the editor arm and a Linux run only; the arm is `cfg(not(target_os = "macos"))`, so the pushed pre-CI commit read red on the macOS CI job; the host could not reproduce it; replaced by a click-and-type sequence on the operator's direction, second CI run green | iterations 1 · retries 2 · dialogue 1 | `escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/evidence/operator-pass.md` |
| compact-snapshot-serialization · L407 · research | four file:line citations were first written from a hand count over listings printed without line numbers and read 1 to 2 lines off; a grep with line numbers re-derived them before research.md was checked | extra_reads 1 | artifact: `scope.md`, `research.md` |
| compact-snapshot-serialization · L417 · report | the report first placed the doubled hit of one fixed string on security-plan line 108, read off a per-file total and a line list; a per-line count put it on line 107, and three statements were corrected | retries 1 | artifact: `report.md` |
| change-tracking-and-diff · L436 · research | two counts were first written into research.md from recall (10 graph queries; a sweep as 2 changed and 4 no-change); the trace and the grep listing gave 8, and 3 changed with 3 no-change | iterations 1 | `.andromeda/runs/2026-10-07T00-24-53-phase/tree-query-2026-10-07-change-tracking-and-diff.json` |

**Proposal:** no step's own group reaches the threshold (research 2, report 2, fix-loop 1); the pattern exists
only across steps (lookback 4 · 0 · 5). The figure-before-listing shape is wider than this type: it also sits in P1 (L340,
L394, L422, L423) and in the below-threshold `retry.synthesis-rework` record L438 — 9 records in the epoch.
The rule is already curated (P2's L369 names the Tier-3 entry), so the remaining direction is mechanical: a
check that resolves every file:line a document cites and every count it states against a listing produced in
the same step (L7 covers the citation half). L388 is a single case of a different shape — a test step over
platform-conditional code verified on one platform — and is listed here because the type groups it.

### P4 — wrap-session/route-resolve · `contract.carry-no-owner` — 4 cases · weight 9

**Pattern:** in 4 of 6 wraps a CARRY's disposition was known and its owning route entry was not; the first
case halted for one round, the next three were placed by judgment and labelled as the wrap's placement.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L346 | the directive placed a new route entry "ahead of the diff and driver work"; three slots in Epoch 3 satisfied it, so the slot was a one-round trajectory halt (answer: the next entry) | dialogue 1 · halted 1 | artifact: `escher-0.1.0/working-route.md:41` |
| id-stability-across-code-edits · L371 | the founder's word named the disposition for the three non-lean tasks (pin a CARRY) and no entry; two markerless entries were plausible owners; the pin went on the sweep without a halt, the choice stated in the pin and in the console report | extra_reads 1 | artifact: `working-route.md` |
| snapshot-state-fidelity · L399 | the recorded directions said each of five engine defects gets a route owner and named no entry; the wrap placed them on three entries by subject, naming the alternate candidate in one pin and in the handoff | extra_reads 2 | artifact: `escher-0.1.0/working-route.md` |
| change-tracking-and-diff · L457 | a defect of two existing blitz-dom unit tests, measured at the wrap, has no entry whose scope names blitz-dom unit tests; pinned to Quality gates as the nearest plausible owner and labelled the wrap's placement, without a halt | extra_reads 1 | artifact: `escher-0.1.0/working-route.md:86` |

**Proposal:** three of the four originate in an operator direction that named what to do and not where
(L346, L371, L399). Directions: the owner is asked for when the direction is given (the review card or the
directive form carries a "which entry" slot); route-resolve.md states the behaviour the last three wraps
converged on — place on the nearest owner and label it, or halt — so each wrap does not re-derive it; and for
a found defect with no natural owner (L457), a standing route entry that owns such findings. Lookback
0 · 2 · 4.

### P5 — phase/distill · `retry.distiller-respawn` — 4 cases · weight 8

**Pattern:** 4 of 6 distill runs re-spawned one distiller once, each time on check 3 (Anchors) and each time
for one shape — a Constraints item citing its section by back-reference; 3 of the 4 are the layouts distiller,
in three consecutive chunks, and each retry that named the check passed.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| id-stability-across-code-edits · L352 | the security distiller failed check 3 (anchors): two Constraints items cited their source as "the row" with no section anchor of their own; one re-spawn naming the check passed | retries 1 | `.andromeda/runs/2026-10-06T19-38-37-phase/.raw-security.md` lines 8 and 10 |
| snapshot-state-fidelity · L378 | the layouts distiller re-spawned once on check 3: 2 of 7 Constraints items carried no plan section anchor (one reading "the same bullet requires", one stating an absence with no section named); the retry anchored all 11 Constraints and Acceptance items | retries 1 | `.andromeda/runs/2026-10-06T21-32-20-phase/layouts.md` |
| compact-snapshot-serialization · L405 | the layouts distiller failed check 3: 3 of 7 Constraints items cited their section by back-reference ("the same bullet requires"); re-spawned once with the check name, the second extract passed all six checks | retries 1 | `.andromeda/runs/2026-10-06T23-05-03-phase/.raw-layouts.md` |
| change-tracking-and-diff · L434 | the layouts distiller re-spawned once on check 3: constraints 2 to 5 of 7 cited their section by back-reference ("the same bullet", "that bullet"); the retry carried the check name and every item came back anchored | retries 1 | `.andromeda/runs/2026-10-07T00-24-53-phase/.raw-layouts.md` |

**Proposal:** what fixes the extract is known before the first spawn — naming check 3 is the whole of each
retry. A direction: the distiller prompt carries the anchor rule with the failing shape as its counterexample
("every item carries its own section anchor; 'the same bullet' is not one"), for the layouts extract at least.
The group is new to the ledger (lookback 0 · 0 · 4); see the era note in mechanism health.

### P6 — wrap-session/reconcile · `input.report-insufficient` — 4 cases · weight 6

**Pattern:** in 3 of 6 chunks reconcile found the report short on the coordinates the masters' citation form
needs — no file:line for new or shifted source (L339, L392) or citation counts that were wrong (L419, L420);
in the two chunks whose report step produced a scripted line map (signals `line-maps-scripted` at L363,
`line-map-content-proven` at L448) reconcile's verdict on the report was ok.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L339 | 13 proposals from 3 docs; 10 of the 13 cited source lines the report does not carry (snapshot.rs, stand_snapshot.rs, element_id.rs) or sat under such a primary, so each was rejected by the re-derivation tell and re-raised by the orchestrator from its own read; all 13 facts applied | extra_reads 4 | `.andromeda/runs/2026-10-06T19-14-10-wrap/fanout-results.md` |
| snapshot-state-fidelity · L392 | the report listed the masters citing mutation_writer.rs lines but not those citing snapshot.rs lines, and carried shift rules rather than new targets; five detectors measured new line numbers on the working tree, against the prompt's ban on re-deriving from source; every applied target was re-measured by the orchestrator | extra_reads 3 | `.andromeda/runs/2026-10-06T22-39-16-wrap/fanout-results.md` |
| compact-snapshot-serialization · L419 | the report's citation search keyed on the basename snapshot.rs and so counted 5 citations of the unchanged stand_snapshot.rs as the edited file's, giving one a moved range; the truth came from reading the hit on a11y-plan line 314 while the detectors were running; the a11y detector's flag had already repeated the wrong map | retries 1 · extra_reads 1 | artifact: `report.md`, `.andromeda/a11y-plan.md:314` |
| compact-snapshot-serialization · L420 | the report read 6 hits of one fixed string as 6 statements of the retired claim; 2 of them, on the same two long lines, state it of a subject the chunk did not touch; the security-plan detector reported it, the architecture detector confirmed, and an offset read settled it before any disposition | retries 1 · extra_reads 1 | artifact: `report.md`, `.andromeda/security-plan.md:107`, `.andromeda/architecture.md:134` |

**Proposal:** the project absorbed this inside the epoch — the last wrap's report located 181 citations of
the five edited files by full path and proved 169 moves by content (L448), and reconcile rated that report
ok — though L451 still shows 5 proposals citing coordinates of new source the report did not hold. What remains is the pipeline-level generalization: the report template asks for the line map and
the file:line of new source as a section of its own, produced by a tool rather than by a per-wrap script
(X2, P9 and L7 are the same subject from the chain, the detector and the level side). Lookback 4 · 1 · 4.

### P7 — implement/code · `input.plan-step-ambiguous` — 4 cases · weight 5

**Pattern:** 4 of 6 code steps met a plan step that either left a shape undetermined (L334, L360 — five
points) or stated a premise the first run falsified (L384, L444); all were settled in the implementation and
surfaced in the P4 report, and one cost a retry.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L334 | two plan points underdetermined: step 5's "omitted" unresolved node (subtree kept or dropped — chose descendants attach to the nearest kept ancestor) and step 10's "no NodeId-derived token" (chose the NodeId Debug form); both surfaced in the P4 report | iterations 0 | artifact: `escher-0.1.0/chunks/2026-10-06-snapshot-model/plan.md` |
| id-stability-across-code-edits · L360 | three plan steps left a shape to the implementer: step 4 (the entry carries which of three readers matched, no type given), step 8 (how the keyed test counts a row as keyed), step 10 (the role list a test may use); each settled and sent to the P4 report | extra_reads 0 | artifact: `plan.md` |
| snapshot-state-fidelity · L384 | plan step 4 names a fixture (a div filled by dangerous_inner_html) and a builder (the snapshot unit module's build helper) that conflict: the helper has no HTML parser, so the fixture condition failed on the first run; resolved in the implementation and carried to the P4 report, no soft-exit | retries 1 · extra_reads 2 | `escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/evidence/red-before-green.txt` |
| change-tracking-and-diff · L444 | step 4's control says the attribute-write test reads red because the flag answers false after a write; with take_changed_nodes added that holds only on a document nothing has drained, so the test writes before its first drain to measure the claim; surfaced in the report | retries 0 | artifact: `plan.md` §Implementation Steps 4 |

**Proposal:** the cost per case is low and the rate is steady (lookback 4 · 3 · 4), so one reading is that
this is latitude working as designed. If it is not: for the undetermined kind, the review card could list
"shapes left to the implementer" so the latitude is chosen rather than discovered; for the falsified-premise
kind, one scratch run of the fixture or API a step names, at P4, is what exposed each case here (also the
untyped neighbours L386 and L412 in the appendix). X1 is this group seen as a chain.

### P8 — phase/validate · `contract.mechanical-check` — 3 cases · weight 6

**Pattern:** 3 of 6 validate runs caught a mechanical-check hit by hand read that planlint has no arm for
(L332 and L358 say so in terms); 2 of the 3 are check 4 (2) — a boot-path file in the modify-set with no smoke
entry, the plan's prose asserting the opposite.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L332 | check 2 (an Expected-amendments sub-bullet named lib.rs, not a master — planlint reads only the matrix# form, so the hand read caught it) and check 4 (9) (the agent-run logs content-key census was a recency selector satisfied by the predecessor run, copied unpinned from the prior plan); both resolved by P5 edits, then the whole set re-ran | iterations 1 | `.andromeda/runs/2026-10-06T18-16-44-phase/planlint-2026-10-06-snapshot-model.json` |
| id-stability-across-code-edits · L358 | check 4 (2), boot-path touchpoint with no smoke entry: the re-synthesized plan added app.rs to the modify-set while its prose still stated no smoke; the hand read on the full re-run caught it before the second card, and the windowed boot smoke was added with its build entry; planlint has no arm for this check | iterations 1 | plan.md Test Commands prose and the smoke entry; precedent `escher-0.1.0/chunks/2026-10-06-headless-stand/plan.md` |
| change-tracking-and-diff · L440 | check 4 (2) fired at P5's read: the plan edits View::poll in blitz-shell window.rs, a boot-path touchpoint by shape, and listed no smoke entry; its prose stated the absence on the reason that the windowed boot path was untouched, which the modify-set contradicts; resolved before the review, the full set re-run | iterations 1 | artifact: `plan.md` |

**Proposal:** a planlint arm for check 4 (2) — the modify-set intersected with a project-declared boot-path
list requires a smoke entry — would move the two repeated cases from a hand read to the tool; the same for
check 2's sub-bullet form. A second direction sits one step earlier: P4 derives the smoke entry from the
modify-set instead of stating its absence in prose. Lookback 10 · 2 · 3.

### P9 — wrap-session/reconcile · `contract.false-positive-proposal` — 3 cases · weight 6

**Pattern:** in the last 3 wraps detector proposals were narrowed or rejected at reconcile; in two (L421,
L451) the rejection was the re-derivation tell firing on proposals whose facts were accurate, which the
orchestrator then re-raised itself with the same substance — 7 proposals, and P6's L339 records the same loop
for 10 more; detectors stating they read source against the prompt's ban number five (L392), three (L421)
and two (L451).

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-state-fidelity · L393 | three proposals (architecture, security-plan, a11y-plan) wrote the provisional status and an unmeasured until-found example into the body; applied narrowed — the status went to the sidecar entries per the falsy-disabled precedent, the unmeasured example was dropped; one architecture citation range was off by one line | reformulations 3 | `.andromeda/runs/2026-10-06T22-39-16-wrap/fanout-results.md` |
| compact-snapshot-serialization · L421 | 2 of 24 proposals (architecture primary, test-plan primary) were rejected on the re-derivation tell: each cited a source line range the report did not carry; both facts were accurate and were applied through the expected-amendments check, the coordinates measured by the orchestrator and added to the report; 3 detector returns state outright that they re-read source files | extra_reads 2 | artifact: `fanout-results.md` |
| change-tracking-and-diff · L451 | 5 of 29 proposals carried a basis or citation the report does not hold (architecture ×2, security-plan ×3) — rejected as returned per the re-derivation tell and re-raised by the orchestrator under check 5 with the same substance; the test-plan and obs-plan detectors also stated they read the source to confirm computed line numbers | retries 0 | `.andromeda/runs/2026-10-07T01-31-03-wrap/fanout-results.md` |

**Proposal:** the tell and the detectors' behaviour disagreed in each of these wraps, and the disagreement was
resolved by the orchestrator redoing the detector's work (17 proposals: 10 at L339, 2 at L421, 5 at L451). Two directions,
which exclude each other: the report carries every coordinate a proposal could need (P6, L7), so a detector
has no reason to open source; or the rule is re-cut — a detector may cite a source coordinate in a file the
report names, and the orchestrator verifies the coordinate instead of rejecting the proposal and raising it
again. L393 is a different shape (a status and an example written into the body) and is here because the type
groups it. Lookback 2 · 2 · 3.

### P10 — wrap-session/curation · `ambiguity.filter-borderline` — 3 cases · weight 5

**Pattern:** in 3 of 6 curation runs, 6 candidates sat exactly on Filter 4's 0.6 line and the outcome turned
on whether one conditional signal applied — a different reading question each time.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| snapshot-model · L344 | one candidate (a document stamp is read from the clock, never estimated — proven by the stamp-ahead hook refusing the report's Date) scored exactly 0.6 at Filter 4 and was rejected; the no-other-home signal was judged not to apply because a hook already enforces it | iterations 0 | — |
| snapshot-state-fidelity · L396 | the applied candidate scored 1.0 only by counting the operator choosing the click-and-type fix among offered options as an explicit correction; without it the candidate sits exactly at 0.6 and rejects; a second candidate landed exactly at 0.6 and was rejected with a master home | reformulations 1 | `.andromeda/runs/2026-10-06T22-39-16-wrap/curation.md` |
| change-tracking-and-diff · L454 | three candidates landed exactly at 0.6 before a conditional signal: two took reached-no-other-durable-home (+0.2) and were applied; for the fixture-namespace rule the signal's route clause was read at P3, before P5 pinned a CARRY naming the same mechanism as a hypothesis about two existing tests — the rule and the defect were judged different facts | reformulations 1 | `.andromeda/runs/2026-10-07T01-31-03-wrap/curation.md` |

**Proposal:** the three reading questions the records leave open are: does a hook's enforcement count as
another home (L344); does choosing among offered options count as an explicit correction (L396); does a
CARRY pinned later in the same wrap count as a durable home (L454). Directions: Filter 4's letter answers
them, or the score grain keeps a base score off the threshold so a single conditional signal does not decide.
One fact beside L344: the candidate it rejected (stamps are read from the clock) is the failure L8 shows
recurring three times in this epoch. Lookback 2 · 2 · 3.

## Cross-step chains (starting heuristics)

Anchors: 13 `input.*` friction records and 12 `consumed` verdicts of thin or wrong (none missing). Joined
within the chunk to an earlier step record that produced the artifact; six producer–artifact–consumer shapes
span two or more chunks, rendered as four chains (X1 folds in phase/validate's re-production of the plan, X3
folds fix-loop and smoke).
For an `input.*` friction the artifact is the one its type names (the map is in `q-chains.json`).

### X1 — phase/plan →plan→ implement/code — 4 chunks

Consumer side: L334, L360, L384, L444 (P7's four records) and the step record L383, which rates the plan
`wrong` ("step 4 second test: the fixture it names cannot be built through the helper it names"). Producer
side: the plan steps L330, L354, L380 and L437 — all outcome ok, their signals `designed-dialogue:*`,
`authority-resolved:*` and, at L380, `planlint-zero-hits-first-fire`; the notes of L330 and L437 also read
planlint 0 hits. In two of the four chunks phase/validate re-produced the plan (L331, L381) before the code
step met it.

Chain hypothesis: a plan that is clean to planlint and through review still carries step-level
under-determination or an unrun premise, and neither P4's nor P5's checks reach a step's executability — the
first run at implement is the first reader that does. Direction: as P7.

### X2 — wrap-session/report →report→ wrap-session/reconcile — 3 chunks

Consumer side: reconcile rates the report `thin` at L338 ("carried no file:line for the two new source
files, which the masters' citation form needs") and L391 ("its moved-citation site list named the
mutation_writer.rs sites only; ten snapshot.rs citations in four masters were found by the detectors") and
`wrong` at L418 ("two Changes counts were false on first write"); with them P6's four friction records.
Producer side: the report steps L337, L389 and L415 — all outcome ok. L337 itself consumed the plan `thin`
and carries the signal `stamp-guard-fired`; L389 and L415 consumed the implement outcome `thin` (X3); L415's
own friction is the narrow-basis record L417.

Chain hypothesis: a report step that closes ok hands reconcile a document whose citation coordinates are
missing or miscounted, and reconcile pays for it in rejected-and-re-raised proposals (P9) and its own
re-measurement. The two chunks outside the chain are the ones whose report step produced a scripted line map
(L363, L448). Direction: as P6.

### X3 — implement/fix-loop + implement/smoke →implement-outcome→ wrap-session/report — 2 chunks

Consumer side: the report rates the implement outcome `thin` at L389 ("its green was superseded by the
operator pass: the first CI read was red on a step implement added, fixed in a second commit") and L415 ("a
step ran after it: the operator directed a quoting change … and then the operator pass"); with them the two
`input.implement-outcome-unsettled` records L390 and L416. Producer side: fix-loop L385 and L413 and smoke
L387 and L414, all outcome ok with the signal `green`.

Chain hypothesis: the operator pass — a pre-CI commit, a push, a CI read and, twice here, a source change —
runs between implement's last checkpoint and the wrap's first and has no step record of its own, so
`implement-outcome: green` is produced before the step that can change it. L388 (the late fix-loop record in
mechanism health) is the same gap seen from the ledger's side. Direction: the operator pass gets a checkpoint
that produces the final implement outcome, or the report template names `evidence/operator-pass.md` as the
outcome's basis whenever one exists. Lookback for the consumer's type: 0 · 1 · 2.

### X4 — phase/plan →plan→ wrap-session/report — 2 chunks

Consumer side: the report rates the plan `thin` at L337 ("Expected amendments entry 5 names design-system and
obs-plan as carrying lib.rs citations; grep reads 0 in both") and L363 ("its Expected amendments list says
app.rs holds 12 design citations; the sweep reads 7 on 6 design-system lines"). Producer side: the plan steps
L330 and L354, both outcome ok.

Chain hypothesis: the plan's Expected-amendments list states a forecast — which masters cite a file, and how
many times — that the wrap then measures differently. P8's L332 is a third record on the same list (check 2
caught a sub-bullet naming a non-master). Direction: the list names the masters without counts, or the counts
come from a grep at P4.

### Anchors with no in-chunk producer

The join is within one chunk, so these stay unjoined — listed so the founder sees them:

- **handoff → new-session/orientation** — L308 (consumed `thin`), L309 and L374
  (`input.handoff-git-mismatch`). Orientation records carry no chunk and no record in the epoch produces
  `handoff`. L374 and the `contract.skill-reference-drift` record L431 describe one mechanism at two
  consecutive session starts: the handoff reads clean while git shows the wrap run dir's tracked evolve trail
  modified (+40 lines, the post-commit gates checkpoint), a file the session-state contract's
  expected-transient list does not name. L428#0 is the same subject from the wrap's side (a tool call moved
  ahead of the commit so its trail would not dirty the committed run dir).
- **matrix → phase/validate** — L356 (consumed `thin`: v010-16's observed_gap held v010-01's text, and no
  ledger verb edits that field). The capability was appended one chunk earlier by a scripted edit because the
  ledger tool had no create verb (L345#0, L347) — a cross-chunk link the heuristic does not join.
- **working-entry → phase/take-up** — L310 (consumed `wrong`: the sync entry presumes an upstream delta;
  upstream had not moved), with the `contract.premise-falsified` record L311.
- **drift-base → wrap-session/reconcile** — L450 (consumed `thin`: a11y-plan's two detectors and obs-plan's
  three have no invariant that reaches a retired lifecycle claim or a moved citation; 4 a11y-plan amendments
  and the 119-citation line map were raised by the orchestrator). The same reach gap shows at L341
  (`contract.cascade-miss`: "no detector proposed it") and in L365's `orchestrator-raised` signals.

## Level candidates (systemic-masked-as-project)

Pass A read all 32 workaround and removed-cause facts of the epoch (no prohibition facts) and clustered them
by the obstacle each routes around; every fact sits in exactly one theme. Six themes reach the threshold as
band-aid candidates; a seventh (L7) re-cuts three L4 facts by subject. Each hypothesis below is a hypothesis —
whether the level is wrong is the founder's call.

### L1 — band-aid — 4 facts (15 in Epochs 1–2)

**Facts:** the project's Bash guard refuses a leading `cd` and a heredoc written to a file.
- upstream-sync-observation-model · phase/distill · environment · workaround — L312#0: the guard refused a leading cd into the run dir when reading the extracts; re-issued with absolute paths ("deferred learning, sixth recurrence")
- snapshot-model · wrap/reconcile · environment · workaround — L338#0: the guard refused a leading cd for a python heredoc writing four raw twins; written with the Write tool instead
- id-stability-across-code-edits · phase/validate · environment · workaround — L356#0: the guard refused a call that wrote a scratch script through a cat heredoc with a file target; the script went through the Write tool
- change-tracking-and-diff · implement/code · process · workaround — L442#0: the guard refused a cat heredoc with a file target for an edit script; written to the scratchpad with the Write tool and run by path
- Typed correlates in the epoch: L342, L367, L455 (three of P2's seven records). Lookback: 11 facts in Epoch 1, 4 in Epoch 2.

**Level hypothesis:** the collision is between a deliberate project hook and two command shapes the agent
reaches for by default; it has been met about once per chunk since the ledger's first record. The fixes so
far live in the project — a retry at each occurrence and a Tier-3 learning that P2 shows is not in the window
when the command is written.

**Proposal:** a direction at the guard (its refusal text names the sanctioned form — absolute paths, the Write
tool — so the first refusal is the last one of the session), at the tier (one always-loaded line instead of an
on-demand entry), or at the pipeline (the letters' own command examples in the guard-safe form).

### L2 — band-aid — 3 facts (6 in Epochs 1–2)

**Facts:** a pipeline tool's output read through `head` or `tail` against the run-bare rule.
- snapshot-model · phase/distill · process · workaround — L328#0: sidecar.py summary piped through `head -3` against the bare-call rule; the verdict lines needed were within the first 3
- snapshot-model · phase/research · process · workaround — L329#0: the first code-graph query read through `tail -40` against the cookbook's never-clip rule; re-read whole from the trace file (45 rows)
- snapshot-state-fidelity · phase/validate · process · workaround — L381#0: one re-run of the fence parse and planlint read through a tail-clipped view; the review card said so, and both were re-run bare after the review edits
- No typed correlate — no friction record accompanies any of the nine. Lookback: 2 facts in Epoch 1, 4 in Epoch 2.

**Level hypothesis:** the rule says bare and the habit clips, each time for a verdict line that sits in the
first rows; the deviation is self-reported and then either re-read whole or left. The cause appears to be a
tool whose useful answer is short and whose output is not.

**Proposal:** the tools print the verdict first and offer a sanctioned short form (a `--verdict` flag), so the
bare call is the cheap one; or the rule distinguishes a verdict read from a full read.

### L3 — band-aid + chronic-degrade — 2 facts in-epoch, recurring from Epoch 2 (2 facts)

**Facts:** on this host `grep` is ugrep, and a bounded-repetition context pattern is refused or prints nothing.
- id-stability-across-code-edits · phase/research · environment · workaround — L353#0: name sweeps run as python regex passes over git ls-files instead of grep, "since grep on this host is ugrep with a recorded bounded-repetition trap"
- snapshot-state-fidelity · phase/validate · environment · workaround — L381#1: a bounded-repetition pattern over test-plan.md was refused (exceeds complexity limits); the fact was read from a plain pattern instead
- Typed correlates: L382 (`tooling.host-shell`: a search with bounded repetitions of 160 and 200 characters errored, printing no match) and L342 (five patterns rejected). Chronic-degrade: `tooling.host-shell` recurs 0 · 1 · 1 without ever halting. Lookback facts: L192#0 (an alternation sweep "printed nothing (false zero)") and L250#0, both Epoch 2.

**Level hypothesis:** a host-environment property is absorbed by a per-session fallback to python and a
Tier-3 learning. One co-cause the records name: the pattern exists to cut a window out of master entries that
are single lines of several KB (L376, L418#0, L420) — the long line is what makes the window pattern
necessary.

**Proposal:** a direction at the host (a GNU grep ahead of ugrep for agent shells), at the project (one search
helper the letters name, so no step writes the pattern by hand), or at the masters' form (entries wrapped so a
plain line match is already a readable window).

### L4 — band-aid — 5 facts (8 in Epoch 1)

**Facts:** scripted replaces run through Bash where the letter's write rule names an anchored Edit.
- id-stability-across-code-edits · implement/code · process · workaround — L359#0: app.rs and the four stand re-pins edited by a scripted python replace, so the PostToolUse fmt hook did not fire; cargo fmt run once by hand afterwards
- snapshot-state-fidelity · implement/code · process · workaround — L383#2: multi-site edits, after a formatter hook rewrote the file, applied by a python replace script with count assertions instead of the Edit tool
- compact-snapshot-serialization · phase/research · process · workaround — L406#0: three line citations in scope.md corrected by one scripted replace through a python heredoc rather than three anchored Edits; each replace asserted exactly one match
- change-tracking-and-diff · phase/plan · process · workaround — L437#0: eight point corrections to plan.md applied through one python heredoc script where the letter requires an anchored Edit; each replacement printed a match count of 1
- change-tracking-and-diff · wrap/reconcile · process · workaround — L450#0: the 119-citation line map and ten single-site leaf replacements written by position- or uniqueness-asserted scripts: "the map holds prefix-colliding ranges an Edit replace_all would corrupt"
- Typed correlates: L446 (`retry.fix-iterations`: the fast leg read red on rustfmt because the edit was applied by a script and the format hook never saw it) and L455.

**Level hypothesis:** the write rule fits a single-site change; every fact here is a multi-site one, and each
already imposes its own safety (a match count, a position assertion). The rule is being worked around in the
same way each time, and the workaround has one side effect the project keeps paying for — the format hook
does not fire on a scripted source edit.

**Proposal:** the letter gains a sanctioned multi-site form (an asserted scripted replace — the facts show
what the assertion looks like), with a format run after any scripted source edit; or a small pipeline tool
does the asserted replace and leaves a trail.

### L5 — band-aid — 4 facts and one second clause

**Facts:** a tool lacks the verb, or the letter names no mechanism, for an act the step needs.
- snapshot-model · wrap/route-resolve · process · workaround — L345#0: the ledger tool has no verb that creates a capability; on the founder's answer v010-16 was appended to verification-matrix.json by a scripted edit that first proved a byte-exact re-dump, then read back through matrix.py
- id-stability-across-code-edits · phase/take-up · process · workaround — L350#0: the entry assigns its decisive question to promotion but P1 names no ask mechanism; the question was put at P1 in P4's marked-recommendation form
- id-stability-across-code-edits · phase/plan · process · workaround — L354#0: inputs.py snap accepts only phase:P1, phase:P3 and implement as its step; the founder's P4 answers were snapped under phase:P3 with the P4 origin stated in the why line
- id-stability-across-code-edits · phase/validate — L356#0, second clause: the review directive was snapped under phase:P3 again, the inputs tool having no later phase step
- (chunkless) · new-session/orientation · process · workaround — L402#0: the evolve nudge's condition names no tool verb, so the per-epoch record count came from an inline python read of friction-log.ndjson, "which evolve-system.md says the agent never reads"
- Typed correlates: L347 and L400 (`contract.no-sanctioned-channel`, n = 2 — a mid-version requirement has no sanctioned writer on the wrap path; the route grammar has no pin for a pending ratification) and L355 (`contract.structural-blind-spot`: inputs.py refused `--step phase:P4`).

**Level hypothesis:** each is a pipeline gap closed by a one-off project-side act recorded in a run dir. The
acts are careful; they are also invisible to the next chunk, which meets the same gap (the inputs step twice
in one chunk; the ledger's missing verb surfacing again one chunk later as L356's `thin` matrix verdict).

**Proposal:** four concrete verbs the facts name: a capability-create verb for the ledger tool (L429 records
matrix v1.4 adding one verb on the route-adaptation path mid-epoch — whether it is this one the record does
not say); `phase:P4` and `phase:P5` as inputs.py steps; a route pin for a pending ratification; a tool verb
that answers the nudge's per-epoch count without a ledger read.

### L6 — band-aid — 2 facts in-epoch, recurring from Epoch 1 (1 fact)

**Facts:** a throwaway probe test written into the tests tree to print ids or diffs, then deleted.
- id-stability-across-code-edits · implement/fix-loop · process · workaround — L361#0: plan step 11 asks the report for the anchored ids Home's card children read, but no listed entry or test prints Home's ids and the stand checks may not print; measured with a throwaway test file under tests/blitz-tests/tests, run once with --nocapture and deleted
- change-tracking-and-diff · implement/code · process · workaround — L442#1: a throwaway probe test (zz_probe_diff.rs) printed each planned step's diff in both layout modes before stand_diff's literals were pinned, then was deleted before the gates
- Lookback: L117#1 (Epoch 1: a probe test file written with the Write tool and deleted after the run). No typed correlate.

**Level hypothesis:** plans ask for measured ids and diffs, the stand checks may not print, and nothing kept
in the tree prints a stand screen — so each chunk builds and deletes its own printer. This reads as a
project-tooling gap rather than a pipeline one, and plausibly the gap the driver's snapshot command will fill
(L410 routes that command to the Driver CLI entry).

**Proposal:** until the driver lands, one kept printing affordance (an ignored test, or a harness dump
command) a plan step can name; or the plan template states the throwaway probe as a sanctioned step so it is
planned rather than improvised.

### L7 — band-aid — 3 facts (the L4 members that edit citations) and 13 friction records

**Facts:** file:line citations into source are typed a line or two off, go stale when a chunk shifts lines,
and are re-pointed by a script each wrap.
- Problem facts: L406#0 (three line citations in scope.md corrected), L437#0 (five line ranges in plan.md among eight corrections), L450#0 (a 119-citation line map applied by script). Lookback: L38#0 (112 stale citations re-pointed by script), L95#0 (80 re-pointed; "no pipeline tool re-points citations after a chunk's line shifts"), L198#0 (the operator chose fix-now for 53 stale citations).
- Friction records in the epoch whose subject is a file:line citation or a line map — 13 of 67: L339, L392, L419 (P6) · L340, L394 (P1) · L364, L407 (P3) · L393, L421, L451 (P9) · L397, L425 (P2) · L438 (below threshold). Step signals: `line-maps-scripted` (L363: 96 found, 61 change, 35 unchanged) and `line-map-content-proven` (L448: 181 located, 169 moves proven by content, 12 read by hand).

**Level hypothesis:** the masters cite source by line number, so every chunk that moves a line stales them,
and the citation is a number someone must type. The project has scripted the remedy by hand in at least four
wraps (L38#0, L95#0, L363, L448); the cost that remains shows up under six different friction types, which is
why no single typed group shows its size.

**Proposal:** two directions at different depths. The band-aid automated: a pipeline verb that verifies every
master citation against the content it was written for and re-points it from the chunk's diff, proven by
content — what L448's script already does. The cause removed: a citation form that does not move with the
lines (a symbol name or a content anchor), which would also empty most of P6 and P9.

### L8 — chronic-degrade — 3 occurrences in-epoch (5 in Epoch 2)

**Facts:** a stamp written from an estimate instead of the clock, refused by the stamp-ahead hook.
- snapshot-model · wrap/report — L337, signal `stamp-guard-fired`: the hook refused an estimated Date (+12 min); re-read from `date -u`
- id-stability-across-code-edits · wrap/report — L364 (in P3): a Date five minutes ahead of the clock, refused
- compact-snapshot-serialization · wrap/reconcile — L422 (in P1): a correction note's time four minutes ahead of the clock, refused on the write
- Lookback, Epoch 2: four untyped friction records (L197, L268, L274, L296) and one removed-cause fact (L213#0). Never a halt.

**Level hypothesis:** the hook catches the estimate each time, so nothing downstream is wrong — and the cause
(a stamp typed rather than read) keeps returning, three wraps out of six here. A curation candidate stating
the rule was rejected at exactly 0.6 because the hook already enforces it (P10's L344). The hook, as the
records describe it, refuses a stamp ahead of the clock; whether an estimate behind the clock would pass is
not measured in the ledger.

**Proposal:** the templates take their stamp from the clock at write time (a tool fills the field), leaving
the hook as the backstop it already is.

### L9 — override, recorded as friction — 3 records

**Facts:** the override signature proper has no hit: the epoch's two `overridden` facts sit on different rules
(L310#0, the operator ruled the sync entry a measured no-op; L345#1, intent.md and requirements.md edited on
the operator's word where no wrap step names them as its write). The same shape — the operator correcting one
kind of call — shows instead in three friction records at phase/validate:
- id-stability-across-code-edits · L357 (`ambiguity.review-cycles`): the operator rejected two leans the plan had stated without asking; one return to P4 re-synthesized the plan — dialogue 1 · iterations 1
- compact-snapshot-serialization · L410 (`contract.matrix-claim`): the P5 preview concretized v010-04's snapshot command as an in-process call; the operator declined the claim, the cap stayed pooled with a note — dialogue 1
- change-tracking-and-diff · L441 (`contract.matrix-claim`): P4 leaned to claim v010-06 on its acceptance wording and a precedent "without reading that the working route lists the cap on Act by id as well (working-route.md:58) and that requirements.md:12 names the driver"; the operator rejected the claim — dialogue 1 · iterations 1

**Level hypothesis:** in three of the last four chunks P4 settled something by lean that the operator then took
back at the review; two of the three are a capability claim. If the rule that lets P4 lean is the thing being
corrected, the correction is being made by hand each time.

**Proposal:** P4's claim lean reads the route for other entries listing the capability, and the requirement's
named surface, before a claim is proposed (L441 names exactly these two reads); or a claim is always a
question on the card, never a lean.

### Signatures with no hit

- **Deferred-forever:** 0 `deferred` facts and 0 `deferral-open` signals in the epoch. One adjacent record:
  L400 — a provisional direction "waits for the founder at the Epoch 3 boundary" and the route grammar has no
  pin for it, so it lives in three sidecar entries and the handoff notes. It is routed, and its destination is
  this boundary.
- **Override:** see L9.

## Playbook-extension candidates (untyped patterns, F-4)

None. The epoch has 4 untyped records in 3 clusters; no cluster reaches n ≥ 3, and none recurs from Epoch 2's
untyped records. They are listed in the appendix.

## Below threshold — no action

Typed groups (n · weight):
- phase/validate · `contract.matrix-claim` — 2 · 7 — L410, L441; both are operator-declined claims (see L9)
- wrap-session/report · `input.implement-outcome-unsettled` — 2 · 4 — L390, L416 (see X3)
- wrap-session/route-resolve · `contract.no-sanctioned-channel` — 2 · 4 — L347, L400 (see L5)
- new-session/orientation · `input.handoff-git-mismatch` — 2 · 2 — L309, L374; L374 and L431 (next line) describe one mechanism under two types
- `contract.skill-reference-drift` (Universal, by type alone) — 2 · 2 — L429 (the deployed pipeline rewritten under a running wrap), L431 (the session-state contract's expected-transient list does not name the wrap's tracked evolve trail)
- implement/code · `input.conventions-gap` — 1 · 4 — L443: research's unit-test convention did not say that `qual_name!("div")` creates an element in no namespace; three retries
- phase/validate · `ambiguity.review-cycles` — 1 · 4 — L357 (see L9)
- implement/fix-loop · `retry.fix-iterations` — 1 · 3 — L446: the fast leg red twice, rustfmt after a scripted edit and a clippy lint (see L4)
- `contract.premise-falsified` (Universal) — 1 · 3 — L311: the epoch-boundary sync entry presumed upstream had moved; it had not, ~16 h after the previous sync (lookback 0 · 3 · 1)
- `contract.structural-blind-spot` (Universal) — 1 · 2 — L355: inputs.py refused `--step phase:P4` (see L5)
- phase/plan · `retry.synthesis-rework` — 1 · 2 — L438: four line ranges in the plan's steps recounted; a clause missing from the check's oracle added (see L7)
- `tooling.host-shell` (Universal) — 1 · 2 — L382 (see L3)
- `tooling.output-cap-overflow` (Universal) — 1 · 1 — L317: three references read in one cat exceeded the tool-result cap twice. Matches the chronic-degrade signature by recurrence (6 · 2 · 1, never a halt) and is falling; left here on one in-epoch record
- wrap-session/reconcile · `contract.cascade-miss` — 1 · 1 — L341: a false a11y-plan §8 sentence no detector proposed, found by the cascade sweep (see the drift-base anchor under chains)

Untyped clusters (emerging — watch next epoch):
- a plan step states a claim the implementation run falsifies or leaves uncovered — 2 — L386 (a matrix acceptance clause had no assertion in the plan step that proves it; found at the ref write, one block re-run), L412 (a plan step lists what `str::escape_debug` escapes; a scratch run measured otherwise on two points). Typed neighbours: L384, L444 (P7)
- a line-matching search printed whole multi-KB single-line master entries into the window — 1 — L376. Nearest Universal: `tooling.output-cap-overflow`, though no cap was exceeded
- the envelope's skill spelling is stated by no reference — 1 — L449 (see mechanism health). Nearest Universal: `contract.skill-reference-drift`

Note themes under threshold (problem facts):
- plan step order changed so a red-before-green reading can compile — 1 in-epoch (L383#1), 2 in Epoch 1 (L33#0, L56#0)
- the smoke's `just seven_guis` recipe replaced by a dev-profile build under a timeout — 1 in-epoch (L387#0, nature resources), 1 in Epoch 2 (L292#0)
- the letter's hand-driven smoke and the gate tool's listed smoke entry disagree on who runs it — 1 in-epoch (L447#0), 1 in Epoch 1 (L89#0)
- the friction ledger read against the never-read rule to copy the envelope spelling — 1 (L442#2); L402#0, counted under L5, also read it
- long single-line master entries read by inline windows instead of `cascade.py window` — 1 (L418#0)
- a tool call moved ahead of the commit so its trail would not dirty the committed run dir — 1 (L428#0)

Single facts:
- L345#2 — the P5 trajectory question put to the operator before P2's apply, in one card with the ledger question
- L365#0 — fan-out returns reach the orchestrator only as messages, so the parsed lists were copied by a script from the hand-back records on disk
- L433#0 — fan-out.md gives the history prompt's sidecar path as relative; the absolute path was substituted because the agent's Read tool takes absolute paths only
- L383#0 (product-logic) — a plan step's reader pin rebuilt through DocumentMutator because the unit helper has no HTML parser
- L379#0 (removed-cause) — the code-graph DB was absent or stale and was regenerated in 28 s before the first query
- L385#0 (removed-cause, product-logic) — an acceptance clause the plan step did not exercise got an added step and assertion; that step is the one L388 records going red on macOS
- L310#0, L345#1 (overridden) — see L9

Single chain:
- phase/research →research→ implement/code — 1 chunk — L443 against L435 (the conventions section did not carry the namespace fact)
