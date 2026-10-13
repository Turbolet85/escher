# Evolve Diagnosis — escher-0.1.0 · Epoch 5 — Agent surfaces · 2026-10-13T09:05:20Z

Nothing here is applied, queued or remembered. Each proposal can be accepted, rejected, deferred or modified, and none of those choices has any consequence for the mechanism. Raw query outputs: `q-retractions.json` · `q-health.json` · `q-typed.json` · `q-untyped.json` · `q-chains.json` · `q-level.json`. The scripts that produced them: `analysis.py` · `lookback.py`.

## Mechanism health

- **Records:** 238 (117 step / 121 friction). The ledger holds 942 records in all. Unparseable: 0. Malformed `ts`: 0.
- **Coverage:** all 8 chunks carry the full expected set (phase 5 · implement 3 · wrap 5), with 0 gaps. Records with no chunk: new-session/orientation 8, one per session start. Wrap-session 5 come from 0-pending/adaptation wraps: route-resolve 3 and curation 2. route-resolve ran 11 times (8 chunk wraps + 3 adaptation), which is not a gap.
- **Step outcomes:** ok 104 · halted-resolved 11 · ok-degraded 2.
- **Untyped:** 21 of 121 (17 %). By step: reconcile 6/35 · research 4/10 · code 3/14 · report 3/9 · fix-loop 2/8 · plan 1/3 · gates 1/2 · curation 1/13. Every other step reads 0.
- **Fill:** problem facts 53 of 117 step records (81 facts). Ids 238 of 238.
- **Signal line:** `guard-refused 20 over 117 step records · exact-hit 10 over 10 curation records`. No signal string is malformed. The 20 come from 17 records spread over 8 chunks. Implement/code holds 7 of them, at least one in 5 of the 8 chunks.
- **Retractions** (whole ledger): retracted 0, with 1 unresolvable. The unresolvable one, verbatim: `2026-10-07T04:21:25Z-a · unknown-id-or-scope · {"id": null, "note": "in step record 2026-10-07T04:21:04Z-a …"}`. It is in Epoch 4, a step-record discount. It is left for the founder to discount by hand.
- **Clause-retracted:** 2, both kept with their notes rendered. One is in this epoch: `2026-10-09T21:21:27Z-b`, whose evidence pointer is discounted. Retractions of retractions: none.
- **Calibration boundaries in range:**
  - The whole epoch postdates `guard-refused`/`exact-hit` (2026-10-09), the `new-text-*` words and the reject counts (2026-10-08), and the operator-pass entry (2026-10-07). So there is no era split.
  - Orientation's widened ladder gloss deploys 2026-10-13, after the last record. The one override of a ladder rung (`2026-10-09T19:14:33Z-a#0`) belongs to the earlier era of that gloss.
- **Chain join limit (observation):** implement/code's 9 `input.plan-step-ambiguous` anchors name `plan.md step N` or source paths, while phase/plan's `produced[]` names `plan`. As a result, only 1 of the 9 joins its producer. The naming mismatch, not the absence of a chain, keeps plan → code chains out of Stage 3.

## Proposals (typed patterns)

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 10 cases · weight 28
**Pattern:** Reconcile corrected its own first writes before the commit in all 8 chunks, at a rate of 10 per 8 runs. The corrections fall into three groups:
- Seam damage left by replacing text: ` , `, ` | ` inside a table cell, a dash before a semicolon, a symbol duplicated after a group merge.
- A claim or order beyond the source.
- Counts.

**Evidence:** all 10 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections-agent-surfaces | a11y-plan clause first said no stand task holds a textarea; reworded; sweep then showed an example fixture's textarea | iter 1 | runs/2026-10-09T20-50-14-wrap/sweep.txt |
| audit-corrections-agent-surfaces | appended playbook rule's note carried a writer's own clause beside the founder's sentence; removed on re-read | iter 1 | — |
| audit-corrections-agent-surfaces | test-plan fixture clause dropped the mutant-kill reading on a §4 wording that has no such bullet; restored in §1 | iter 1 · reads 1 | — |
| upstream-sync-agent-surfaces | 2 texts: an ambiguous writing-mode clause; a sidecar Why giving a reason the plan does not state | iter 2 | runs/2026-10-10T01-56-50-wrap/architecture-entry-4.md |
| driver-cli | sidecar payload's Supersedes named an entry whose facts still stand; rewritten as partial retirement | iter 1 | runs/2026-10-10T10-03-00-wrap/security-plan-entry-4.md |
| scrolling-box-bounds-and-hit | 3: dash+semicolon at layout-templates:10; wrong-referent "that chunk" at test-plan:181; dispositions file written with 0 rows by a script | iter 3 | runs/2026-10-10T15-29-49-wrap/cascade-dispositions.md |
| snapshot-states | 3: ` \| ` inside a table cell would split the row; a hand re-point named as an amended section; a leaf stated a reading nobody took | iter 3 | — |
| mcp-surface | 3 seams in architecture.md after replaced "and" (`Outcome` , the · named , since · lost conjunction), each caught by a grep for ` , ` | retries 3 | — |
| self-description | 3: a non-listing citation coordinate; obs-plan:70 over-extended; CLAUDE.md:37 held describe() twice after two groups applied | retries 3 | runs/2026-10-12T08-55-10-wrap/cascade-dispositions.md |
| headless-screenshot | 4: three citation rows first dispositioned false; "main raised 5" → 4; a sentence order the report does not state; a clause landed inside the wrong parenthesis | reads 4 | runs/2026-10-12T20-34-17-wrap/citation-dispositions.md |

**Proposal:** About a third of the cases are mechanical seams that a pattern can catch, and one chunk already caught them with a grep for ` , `. One direction is a post-apply seam lint over every master and leaf line the pass amended, run by the tool before the commit:
- ` ,`
- a ` | ` inside a table row that changes its column count
- `—;`
- a doubled word or symbol after the tool merges groups

The re-read would then be kept for the claim-beyond-source and order cases, which no pattern reaches.

### P2 — wrap-session/curation · `recall.corpus-recurrence` — 7 cases · weight 18
**Pattern:** Hazards already recorded in session-learnings recurred. Three of the seven are the same class: a host path written into committed run-dir text and refused by hygiene at the operator pass. That class recurred despite the 2026-10-06 learning, and one of the three halted. The other four are: CI-log colour codes, `$TMPDIR` unset, scripted edits skipping the rustfmt hook, and repeated `--pattern` flags.

**Evidence:** all 7 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-agent-surfaces | `Running tests/` pattern reads 0 over `gh run view --log` (colour codes), already Tier 3 since 10-07; repeated | iter 1 | runs/2026-10-10T01-56-50-wrap/curation.md |
| driver-cli | a record quoting a temp-dir path refused by hygiene, the 10-06 learning's exact defect; the operator-pass record's first draft refused | iter 1 | runs/2026-10-10T10-03-00-wrap/curation.md |
| scrolling-box-bounds-and-hit | operator pass stopped a second time in two chunks by a home path in the phase's copy of the operator's message | halted 1 · dlg 1 | chunks/…/report.md |
| mcp-surface | `$TMPDIR` unset in the Bash tool's shell again (redirect wrote to /x), recorded since 10-07 | retries 1 | — |
| self-description | a phase relay written verbatim carried a sibling directory's host path into a committed run dir; refused at the operator pass | retries 1 | chunks/…/evidence/operator-pass.md |
| self-description | Rust edits by python replacement skipped the rustfmt hook; the fast leg read red, as the 10-06 extension says | retries 1 | chunks/…/report.md |
| headless-screenshot | the Tier 3 entry on `cascade.py sweep --pattern` (curated 10-12) did not prevent ten repeated flags running one pattern | retries 1 | .claude/docs/session-learnings.md |

**Proposal:** The learning text does not hold these hazards. A direction is to move each one into the mechanism that meets it:
- **Host paths:** mask `$HOME` and temp-dir prefixes when a relay or input copy is written into a run dir (`inputs.py snap` and the relay writer), instead of refusing at the operator pass. This is also X3's cause.
- **`--pattern`:** make the flag reject repetition, or accept it cumulatively, in `cascade.py`.
- **`$TMPDIR`:** set it in the Bash environment.
- **Formatter hook:** see L2.

### P3 — implement/code · `input.plan-step-ambiguous` — 9 cases · weight 13
**Pattern:** In 6 of 8 chunks, implement settled plan steps that were silent or could not all hold together.

**Evidence:** all 9 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-agent-surfaces | conflict steps silent on `git add` of resolved paths | — | plan.md |
| driver-cli | step 13's idle-expiry check can never see the host gone, since step 8 restarts expiry on each request | — | plan.md steps 8, 13 |
| driver-cli | step 6 unit rows over a built node: DiffNode's role/bounds types not nameable from escher-driver's deps; writer split | reads 2 | plan.md step 6 · json.rs |
| driver-cli | step 8 silent on a call over the request bound; settled: client refuses first | — | plan.md step 8 · client.rs |
| scrolling-box-bounds-and-hit | step 6 fixed no point for a transformed scrolled box; step 5 silent on loop shape vs a probe forbidding `layout_data()` | reads 1 | plan.md |
| snapshot-states | steps 2–3 "read red first" but name fields that land at step 5 → compile error, not assertion red | retries 1 | evidence/controls.md §1 |
| snapshot-states | step 10's 17 mutations reach no red for 4 of the tests it covers; 5 controls added | — | evidence/controls.md §2 |
| mcp-surface | three silent points: `run()` takes apps; `start` outside apps; id-less parse errors vs one-line-per-id | reform 3 | cli.rs · mcp.rs · mcp_server.rs |
| headless-screenshot | step 5's white fill needs kurbo/peniko, re-exported by none of the acceptance's three harness deps | reads 3 | plan.md · screenshot.rs |

**Proposal:** Two shapes recur:
- **A step's type not reachable from the crate's pinned dependencies** (driver-cli step 6, screenshot step 5). The untyped serde-order record `2026-10-11T08:02:42Z-b` is a third, and so are 2 of the 4 product-logic workarounds in `q-level.json`. A direction is a Validate probe: for each symbol a step names from another crate, query tree.db for whether it is reachable from the crate's declared deps.
- **A "red first" step whose red cannot be an assertion.** A direction is a plan-template line that makes such a step say whether its red is a compile red or an assertion red.

### P4 — wrap-session/route-resolve · `contract.carry-no-owner` — 7 cases · weight 13
**Pattern:** Open findings, relays and directives reached route-resolve without a named owner entry. The wrap placed them itself, at a cost of 16 extra reads.

**Evidence:** all 7 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-agent-surfaces | 3 of 7 open findings named no owner; placed under the project rule | reads 1 | runs/2026-10-10T01-56-50-wrap/route-resolve.md |
| driver-cli | 3 carries with no owner (two unfixed engine readings, two untested CLI paths); placed under the curated rule | reads 1 | runs/2026-10-10T10-03-00-wrap/route-resolve.md |
| — (adaptation) | relay anchored a WATCH as unchanged while the insertion rule re-pins it; put inside the trajectory halt | dlg 1 | runs/2026-10-10T12-54-32-wrap/adaptation-record.md |
| — (adaptation) | six relayed observations, none pre-placed; six code/intent reads before the halt | dlg 1 · reads 6 | runs/2026-10-10T13-01-58-wrap/adaptation-record.md |
| scrolling-box-bounds-and-hit | five engine readings with no obvious owner; callers read; operator moved two | reads 5 · dlg 1 | runs/2026-10-10T15-29-49-wrap/adaptation-record.md |
| mcp-surface | the directive named the MCP-revision pin's content, not its entry; placed on Self-description | reads 1 | — |
| headless-screenshot | two owners chosen by the wrap (caret → Stand requirement sweep; README:48 → next entry) | reads 2 | — |

**Proposal:** The project already absorbed the placement rule (Session Learning of 2026-10-10: every unowned finding gets a route owner without a halt). What remains is where the owner is first proposed. One direction is for the report template to ask each open finding for a proposed owner entry at P1, where the context is fullest. Route-resolve would then confirm owners instead of searching for them. The same could apply to a relay's items when it is filed.

### P5 — phase/validate · `contract.mechanical-check` — 5 cases · weight 14
**Pattern:** The operator's review caught plan defects that no Validate check attempts. Four of the five cost a dialogue round.

**Evidence:** all 5 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-agent-surfaces | three checks needed resolution: criteria naming evidence no entry produces; a platform-issues slot; a mark on an entry an earlier plan names | iter 1 · reads 1 | plan.md · research.md |
| driver-cli | the plan carried half the founder's chosen option (pinned, but not stated where the tool describes itself); no check compares the plan to a fork option's text | dlg 1 | plan.md · p4-founder-forks.md |
| driver-cli | a stated wire bound (largest answer) had no check reaching its branch; no check asks for a row on each side of a bound | — | plan.md |
| self-description | step 3 required pairwise-distinct remedies; step 13 lacked a `help` call first after initialize | dlg 1 · retries 1 | runs/2026-10-12T06-49-46-phase/relay-3.md |
| headless-screenshot | an 11th verb and 9th cause break the literals of two verified caps (v010-13, v010-14); no predicate checks verified acceptances against counts the plan moves | dlg 1 · iter 1 | runs/2026-10-12T18-14-09-phase/p5-card-2.md |

**Proposal:** Two of these have a mechanical shape a P5 predicate could take:
- **Count-literal guard:** when a plan moves a count word ("ten verbs", "eight causes"), grep the verified matrix acceptances and their ref pins for the old literal.
- **Bound coverage:** every bound the plan states names a test row on each side.

The fork-option and API-shape catches (driver-cli, self-description) read as review judgment, with no evident mechanical form.

### P6 — wrap-session/reconcile · `input.report-insufficient` — 5 cases · weight 12
**Pattern:** In 5 chunks, detectors could not finish proposals from the report alone. What was missing: a count's master sites, which shared modules a new check declares, the per-file split of the unit tests, and a Coverage row for a changed behaviour. This group carries 1 halt.

**Evidence:** all 5 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| upstream-sync-agent-surfaces | security detector could not confirm the hint-less font source (one Symbols sentence, no Coverage row); text-indent left unread | halted 1 · dlg 1 | runs/2026-10-10T01-56-50-wrap/escalations.md |
| driver-cli | 11 proposals across 3 detectors "wants a re-read": no shared-module list, no per-file test split, no README wording | reads 5 | runs/2026-10-10T10-03-00-wrap/fanout-results.md |
| scrolling-box-bounds-and-hit | two detectors read source to finish proposals; both rejected for source and re-raised by main | reads 3 · reform 2 | runs/2026-10-10T15-29-49-wrap/fanout-results.md |
| snapshot-states | the Counts table under-ran by 3 test-plan sites and 4 security-plan restatements | reads 7 | runs/2026-10-10T17-47-57-wrap/fanout-results.md |
| mcp-surface | report did not say stand_mcp_tools reads session_common, so the reader counts could not be amended | reads 1 | chunks/…/report.md |

**Proposal:** Three of the missing facts can be derived from the tree:
- the `mod` declarations of new test files
- the unit-test count per file
- each retired count word's master sites, from a cascade sweep at P1

One direction is a P1 tool block that prints these into the report, the same way `new-text-*` already does for new text. The judgment items, like a Coverage row for a changed behaviour, stay with the author. See X2.

### P7 — wrap-session/reconcile · `contract.cascade-miss` — 6 cases · weight 9
**Pattern:** In 6 chunks, restatements that no detector proposed survived the apply and were found by a later sweep or by hand. Leaves (`services/*.md`, `tests-summary.md`) are the site in 3 of the 6.

**Evidence:** all 6 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections-agent-surfaces | the retired-count pattern keyed on "64 tests" and missed "64 CI-script tests" in a leaf | reads 1 | runs/2026-10-09T20-50-14-wrap/cascade-dispositions.md |
| driver-cli | five restatements in security-plan / architecture / test-plan found by sweep or leftover grep | iter 2 · reads 5 | runs/2026-10-10T10-03-00-wrap/cascade-dispositions.md |
| scrolling-box-bounds-and-hit | architecture:130 "scroll_into_view's one caller" found by the scroll-only pattern after apply | iter 1 · reads 1 | runs/2026-10-10T15-29-49-wrap/cascade-dispositions.md |
| mcp-surface | six restatements (arch:138/150/193/263, sec:123, test-plan §6) found by main's read during apply | reads 6 | — |
| self-description | test-plan:143, services/escher-driver.md:60, services/seven_guis.md:11 caught after apply | reads 3 | runs/2026-10-12T08-55-10-wrap/cascade-dispositions.md |
| headless-screenshot | WHOLE-mode comparer left escher-driver.md:60 (66 tests, mcp.rs 10) and :55 ("six instance verbs"); the re-sweep found both | reads 1 | runs/2026-10-12T20-34-17-wrap/cascade-dispositions.md |

**Proposal:** The misses are restatements of a count or name that the report's Counts table already retires. One direction is to generate the sweep's patterns from that table, covering each retired word in its numeral and spelled forms with a following-noun wildcard, and to run them before the apply. The post-apply re-sweep would then be a gate that reads zero unexplained rows.

### P8 — wrap-session/report · `input.implement-outcome-unsettled` — 3 cases · weight 11
**Pattern:** In 3 chunks the operator pass ended `fixed` after implement's green, so the report graded implement-outcome thin by rule. In 2 of the 3, the fix was to a phase run-dir file implement did not write: a host path refused by hygiene. The third was a CI network flake.

**Evidence:** all 3 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-cli | hygiene refused a phase run-dir file (27 temp-dir paths); re-fired after rewrite, and again after the pass's own record quoted the path | retries 2 | evidence/operator-pass.md · p4-founder-forks.md |
| scrolling-box-bounds-and-hit | hygiene refused the phase's copy of an operator message (home path); stop, then a one-token rewrite | dlg 1 · retries 1 | chunks/…/evidence/operator-pass.md |
| mcp-surface | CI attempt 1 red (rustup connection reset before any compile); re-run on the operator's word, green 16/16 | retries 1 · dlg 1 | chunks/…/evidence/operator-pass.md |

**Proposal:** There are two directions:
- The grading rule could tell a pass that fixed implement's own output from one that fixed a file outside it, or a flake.
- The cause of the hygiene pair could be removed upstream of the pass (P2's masking).

See X3.

### P9 — wrap-session/curation · `ambiguity.filter-borderline` — 4 cases · weight 8
**Pattern:** Curation's Filter 4 conditional signal (three cases) and Filter 5's cap (one case) were read at their edges. The `exact-hit` signal adds a count over the same epoch: 10 over 10 curation records.

**Evidence:** all 4 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections-agent-surfaces | Filter 4 no-other-home: does a chunk's restore-by-sha256 fact in a test-plan amendment count as the rule amended into a master? judged not | iter 1 | runs/2026-10-09T20-50-14-wrap/curation.md |
| — (0-pending wrap) | Filter 5: does the cap of three bind entries approved singly in review mode? the operator ruled the letter holds | dlg 1 · deferred 1 | runs/2026-10-09T21-37-22-wrap/curation.md |
| upstream-sync-agent-surfaces | Filter 4 conditional at exactly 0.6, load-bearing for the next entry; applied, the CARRY pointing at the entry | iter 1 | runs/2026-10-10T01-56-50-wrap/curation.md |
| driver-cli | Filter 4: does an Edit refused on two matches count as a real gate failure? scored as one | — | runs/2026-10-10T10-03-00-wrap/curation.md |

**Proposal:** A direction is for the curation guide to state three things: the operator's Filter 5 ruling (the cap binds singly approved entries), whether 0.6 itself is in or out for Filter 4, and the two Filter 4 conditions read here. That would leave nothing to judge at the edge.

### P10 — wrap-session/route-resolve · `contract.no-sanctioned-channel` — 4 cases · weight 8
**Pattern:** Three directive items on the 0-pending path had no door: a citation sweep, a playbook append and a WATCH tally update. A fourth finding was about a pipeline tool, which no route entry can own.

**Evidence:** all 4 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| — (0-pending) | the first citation sweep: no surface on the 0-pending path; read run, write declined, repeated at the next wrap | dlg 1 · reads 4 | runs/2026-10-09T19-17-00-wrap/citation-dispositions.md |
| — (0-pending) | a founder-approved playbook rule append: no named door; carried to the corrective chunk's wrap | dlg 1 · deferred 1 | runs/2026-10-09T19-17-00-wrap/adaptation-record.md |
| audit-corrections-agent-surfaces | cites.py's while/case row ends one line early: a pipeline-tool finding, no route owner; named in the handoff | — | report.md · session-handoff.md |
| — (0-pending) | a green CI run of the WATCH's subject has no channel into the WATCH tally on the 0-pending path | reads 1 | working-route.md |

**Proposal:** One direction is to name doors on the 0-pending wrap for two directive items: a playbook append on the founder's word, and a WATCH tally fed by a CI run. Another is a standing pipeline-findings channel. The handoff's "for the operator to relay to the pipeline owner" lines are the ad-hoc form of it today.

### P11 — wrap-session/reconcile · `contract.false-positive-proposal` — 3 cases · weight 8
**Pattern:** Detector proposals were narrowed at apply. In one, a change line's count disagreed with its own list. In another, the registry over-reached into members of registered categories. In the third, a detector asked to drop a citation the sweep had proved unmoved.

**Evidence:** all 3 cases.

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections-agent-surfaces | change line counted five mutations while listing four; the retitle not applied | reads 1 | runs/2026-10-09T20-50-14-wrap/fanout-results.md |
| upstream-sync-agent-surfaces | 5 of 54 narrowed: D-arch-resources ×3 registry over-reach (members inside registered categories); D-arch-decisions ×2 clauses not in the report | reform 5 | runs/2026-10-10T01-56-50-wrap/fanout-results.md |
| driver-cli | D-obs-stack asked to drop a line number the report does not state; the citation sweep had proved it unmoved | reads 1 | runs/2026-10-10T10-03-00-wrap/fanout-results.md |

**Proposal:** Two prompt-side directions:
- The D-arch-resources registry rule could state "categories, not their members".
- Detectors could be given the citation sweep's held/unmoved rows, so a verified number is not proposed for removal.

### P12 — universal · `contract.token-proxy-check` — 3 cases · weight 5
**Pattern:** Three checks tested a token where a property was meant. Two were false positives and one a false over-count. In every case the truth came from reading the hits.

**Evidence:** all 3 cases.

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| audit-corrections-agent-surfaces | phase/research | the stamp-ahead write hook read a bare time of day from a two-day-old CI record as a future stamp (FP) | retries 1 | runs/2026-10-09T19-31-04-phase/ci-37673662374-install-hang.md |
| upstream-sync-agent-surfaces | phase/research | a job count by a two-space-key line pattern also matched `on:` triggers (3/4/2 vs a YAML read's 1/2/1) | reform 1 | — |
| snapshot-states | implement/code | the restore check compared `git status` text; a ledger trail written mid-batch made it differ (FP); the truth was diff + sha256 | reads 1 | — |

**Proposal:** The first case is the pipeline's own hook. A direction is for the stamp-ahead check to match only a full date-time stamp. The other two are the agent's own instrument choices (a line pattern for YAML, a text compare for a restore property), with no evident pipeline direction.

### P13 — universal · `contract.premise-falsified` — 3 cases · weight 3
**Pattern:** Verification falsified three authored premises: a directive's count and two route CARRY hypotheses. All three were measured at the first step, with 0 iterations.

**Evidence:** all 3 cases.

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| — (0-pending) | route-resolve | the directive said the CI install hung in four runs on 10-07; `gh run list` read two (with re-runs) | reads 1 | runs/2026-10-09T19-17-00-wrap/relay-1.md |
| driver-cli | implement/fix-loop | CARRY hypothesis "typed text inserted inside a filled value" read false at step 1: appended after it | — | scope.md CARRY 7 · evidence/type-filled.md |
| driver-cli | implement/fix-loop | CARRY hypothesis "a control where a scrolled-out row extends reads covered" false on the named fixture; true only for a button before the box | — | scope.md CARRY 9 · evidence/engine-findings.md |

**Proposal:** At 0 iterations, this group reads as the measure-first step working as designed. If anything is wanted, a direction is to mark a CARRY's claim as `hypothesis` or `measured` when it is written, so later readers weigh it accordingly.

## Cross-step chains (starting heuristics)

### X1 — implement/smoke →conversation→ wrap-session/curation — 5 chunks
- **Consumers:** curation read `conversation` as thin or missing in 5 chunks: audit-corrections-agent-surfaces `2026-10-09T21:19:51Z-a`, upstream-sync `2026-10-10T02:39:55Z-a` (missing), driver-cli `2026-10-10T10:58:48Z-a`, scrolling-box `2026-10-10T15:59:16Z-a`, mcp-surface `2026-10-11T09:56:20Z-a`.
- **Shared cause:** in each one, a resumed wrap ran in a fresh window and the window that ran implement was gone.
- **Producer:** the last producer of `conversation` (implement/smoke: `ok`, signals `skipped` · `recorded-not-rerun` · `recorded-from-P2`) is the join's artifact, not the cause. The cause is the window split recorded as L6's override.

**Direction:** P1 writes curation's candidate list, from the live conversation, into the run dir before the stop. Curation then consumes that file instead of a conversation that will not survive.

### X2 — wrap-session/report →report→ wrap-session/reconcile — 4 chunks
- **Consumers:** reconcile read `report` thin in upstream-sync (`2026-10-10T02:37:05Z-a`), driver-cli (`2026-10-10T10:57:00Z-a`), scrolling-box (`2026-10-10T15:58:16Z-a`) and snapshot-states (`2026-10-10T19:02:09Z-a`).
- **Producers:** report records, each formally `ok`. Their signals: `reconstructed` · `new-text-rows:417`; `new-text-rows:319`; `guard-refused:1` · `inputs-uncited-closed:1`; `guard-refused:2`.
- **Hypothesis:** an `ok` report with no signal for what it does not carry (readers, per-file splits, count sites) feeds P6's frictions downstream. A producer-side signal like `counts-sites:{n}` would make the gap visible at P1. Direction: P6.

### X3 — implement/smoke →implement-outcome→ wrap-session/report — 4 chunks
- **Consumers:** report graded `implement-outcome` thin in driver-cli, scrolling-box, mcp-surface and self-description.
- **Producer:** implement/smoke was `ok` · `green` in all four.
- **Operator-pass entries:** `fixed` in all four (`red:2`, `red:1`, `red:1`, `red:1`), so the pass superseded the producer's signal.
- **Causes:** three of the four are hygiene refusals of a host path in a phase run-dir file: driver-cli, scrolling-box, and self-description (`2026-10-12T09:00:00Z-a`, red at hygiene, then masked). The fourth, mcp-surface, is a CI flake.
- **Hypothesis:** the hygiene refusals originate at phase, when a relay or fork file is written, travel through implement untouched, and land on implement's grade. Directions: P2 (masking at write) and P8 (the grading rule).

### X4 — phase/research →research→ implement/fix-loop — 2 chunks
- **Consumers:** fix-loop read `research` thin in driver-cli (`2026-10-10T09:40:57Z-a`: stand_act_spans and stand_act_disabled pin changed data, in neither list) and scrolling-box (`2026-10-10T15:11:45Z-a`: stand_actionable_keys leaned on the hit defect, missed by a verb/phrase sweep).
- **Producers:** research was `ok` in both, with signals `unresolved-questions` (+ `open-questions:3`, `baseline-run-green`).
- **Downstream frictions:** the same two checks are fix-loop's two `contract.test-expectation` frictions (`2026-10-10T09:40:57Z-b`, `2026-10-10T15:11:45Z-b`), each costing a red gate.

**Hypothesis:** research's "checks that pin the changed reading" sweep is a keyword grep, which misses a check that reaches the changed code through a helper. **Direction:** a tree.db query for the test functions that transitively call the changed symbols, run beside the grep.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 7 facts (recurring every epoch: E1 1 · E2 3 · E3 2 · E4 2)
**Facts:** All 7 are process/workaround. Each worked around the letter's "run bare" rule by filtering, redirecting or piping a pipeline tool's output.

| chunk | step | tool | id |
|---|---|---|---|
| upstream-sync | distill | sidecar.py cites, through grep, exit read from PIPESTATUS | `2026-10-10T00:55:21Z-a#1` |
| upstream-sync | validate | planlint, through grep | `2026-10-10T01:18:10Z-a#1` |
| driver-cli | distill | registry.py + sidecar.py, redirected and read by a clipped print | `2026-10-10T03:02:03Z-a#0` |
| mcp-surface | take-up | route.py pins, redirected to keep 52 rows out of the window | `2026-10-10T19:17:22Z-a#0` |
| mcp-surface | distill | sidecar.py cites, redirected | `2026-10-10T19:25:10Z-a#1` |
| headless-screenshot | distill | sidecar.py, through head/tail | `2026-10-12T18:23:05Z-a#0` |
| headless-screenshot | validate | gate.py run, through tail | `2026-10-12T18:42:39Z-a#0` |

The typed correlate is `tooling.output-cap-overflow`, across epochs E1 6 · E2 2 · E3 1 · E5 1.

**Level hypothesis:** The obstacle is the tools' output volume against the context window, a property of the tools. The fix lands in every chunk, one pipe at a time, and the rule is broken each time.

**Proposal:** The tools print a compact verdict block by default, with the full listing in the trail they already write and a flag for the whole listing. "Run bare" and the window would then agree.

### L2 — band-aid — 6 facts (E1 2 · E3 4 · E4 5)
**Facts:** All 6 are process/workaround. Each made multi-site edits through a script instead of the Edit/Write tool the letter names.

| chunk | step | what was scripted | id |
|---|---|---|---|
| audit-corrections | reconcile | 60 dispositions rows composed by script, to land byte-exact | `2026-10-09T21:17:27Z-a#0` |
| driver-cli | plan | research/plan point changes by a python heredoc | `2026-10-10T08:21:37Z-a#1` |
| driver-cli | code | edits after formatter rewrites, by exact-once specs + sed | `2026-10-10T09:23:13Z-a#1` |
| scrolling-box | code | eight text sites by one script (the fmt hook did not see them) | `2026-10-10T14:58:15Z-a#0` |
| mcp-surface | reconcile | a leaf by 12 asserted replacements | `2026-10-11T09:55:09Z-a#0` |
| self-description | code | seven files by a python heredoc | `2026-10-12T08:27:31Z-a#0` |

Correlates: untyped `2026-10-12T08:38:10Z-b` (fix-loop red on rustfmt after scripted edits) and P2's `2026-10-12T09:39:58Z-c`.

**Level hypothesis:** A multi-site change costs one Edit call per site, and the formatter hook fires only on Edit/Write. The agents reach for asserted-replacement scripts every epoch, and the hook's protection silently drops out.

**Proposal:** A sanctioned multi-site edit tool in `andromeda-tools/` that asserts exact-once replacement per site and runs the formatter on the files it touched. Or, more narrowly, make the format check a gate entry that is independent of the hook.

### L3 — band-aid — 4 facts (E1 1 · E3 1 · E4 6)
**Facts:** All 4 are reconcile, process/workaround. In each, the fan-out returns arrived as messages while the record must hold them as returned.
- `2026-10-10T02:37:05Z-a#0`: lifted from transcripts by script.
- `2026-10-10T10:57:00Z-a#1`: lifted from transcripts by script; one return arrived as a 55 KB persisted file.
- `2026-10-10T15:58:16Z-a#1`: "transcript files are not to be read", so the returns were transcribed by hand and shortened.
- `2026-10-10T19:02:09Z-a#2`: the rationale was cut by hand.

**Level hypothesis:** The contract asks for a byte-exact record, but the transport delivers messages. Chunks diverge on how to bridge the two: half read transcripts and half do not.

**Proposal:** Each detector writes its own return to a run-dir file (`return-{doc}.md`) and hands back the path. fanout-results then cites the file and its hash.

### L4 — band-aid — 7 facts (+4 adjacent; E3 1 · E4 3)
**Facts:** In each, a founder decision arrived by relay and no step names how to capture it. All are process/workaround.
- `2026-10-10T00:34:22Z-a#0`: two dialogs expired at 58 minutes; the answers came by relay message.
- `2026-10-10T00:34:22Z-a#1`: `inputs.py snap` needs a chunk dir; the 0-pending path has none.
- `2026-10-10T13:00:44Z-a#0` and `2026-10-10T13:06:26Z-a#0`: the same, relay copied by hand.
- `2026-10-10T16:03:52Z-a#0`: dialog answers in a chunk wrap have no named record.
- `2026-10-12T07:10:04Z-a#0`: the plan was written on recommendations before the founder's answers.
- `2026-10-12T08:16:22Z-a#0`: the answers were relayed at review.

Adjacent facts:
- `2026-10-10T08:21:37Z-a#0`, overridden: the dialog returned unanswered and the forks went to a file.
- `2026-10-10T14:22:26Z-a#0`, deferred: forks unanswered before the plan.
- `2026-10-10T14:32:50Z-a#0`, overridden: the forks were answered as technical forks.
- `2026-10-10T16:44:10Z-a#1`, deferred: PROVISIONAL.

Earlier epochs: `inputs.py snap` takes no P4/P5/wrap step (E3 `2026-10-06T20:21:19Z-a#0` · E4 `2026-10-07T05:17:31Z-a#1`, `2026-10-07T14:28:27Z-a#0`, `2026-10-07T18:49:37Z-a#0`). Correlates: U1, and P10's WATCH case.

**Level hypothesis:** The pipeline models a founder decision as an in-step dialog. On this project the founder is reached through an operator and an overseer session, hours later. That is a property of the process, absorbed chunk by chunk.

**Proposal:** Make the relay a first-class input:
- `inputs.py snap` accepts any step and a run-dir target with no chunk dir.
- A timed-out founder dialog writes its forks to a relay file as its designed fallback.

### L5 — band-aid — 4 facts (new in E5)
**Facts:** All 4 are reconcile, process. In each resume, the reused report was amended to cite inputs read at the resume:
- `2026-10-09T21:17:27Z-a#1`: workaround, one Decisions bullet.
- `2026-10-10T15:58:16Z-a#0`: workaround, four lines.
- `2026-10-10T19:02:09Z-a#0`: workaround, six lines, "the same shape as the previous wrap's resume".
- `2026-10-10T10:57:00Z-a#3`: overridden, two lines.

**Level hypothesis:** "A resume reuses the report as is" and "every input the wrap reads is cited by the report" cannot both hold once a resume reads an input. The pair surfaced only when L6's split became routine.

**Proposal:** A named resume addendum section in the report, which the letter permits and the uncited check reads.

### L6 — override — 5 facts (E4 2: `2026-10-07T20:34:14Z-a#0`, `2026-10-07T22:39:30Z-a#0`)
**Facts:** All 5 are wrap-session/report, process/overridden. In each, the operator bounded the wrap to "P1 only and stop":
- audit-corrections `2026-10-09T20:55:00Z-a#0`
- upstream-sync `2026-10-10T02:03:30Z-a#0`
- driver-cli `2026-10-10T10:07:56Z-a#0`
- scrolling-box `2026-10-10T15:35:05Z-a#0`
- snapshot-states `2026-10-10T17:54:12Z-a#0`

Epoch 4's first case states the reason: the window was at 43.8 % at invocation, and the previous wrap had grown it by 46 points.

**Level hypothesis:** The rule is that a wrap runs P1–P7 in one invocation. The operator keeps correcting the same call, which suggests the rule is miscalibrated for the window budget. Downstream costs in this epoch: X1 (curation without its conversation, 5 chunks), L5 (report amended at resume, 4) and the deferred graph refresh `2026-10-09T20:55:00Z-a#1`.

**Proposal:** Make the split a designed path:
- P1 ends with a stop and a resume line.
- P1 writes the curation candidates (X1) and opens the resume addendum (L5).
- Setup on resume re-fires what P1 deferred.

### L7 — band-aid — 2 facts in epoch, recurring from E3 and E4
**Facts:** Both are new-session/orientation, process/workaround: `2026-10-10T16:11:27Z-a#0` and `2026-10-10T19:13:34Z-a#1`. In both, the evolve nudge needed a per-epoch record count, no tool verb serves it, and the ledger was parsed inline against the never-read rule. Earlier cases:
- E3 `2026-10-06T23:04:26Z-a#0`: the same.
- E3 `2026-10-07T01:09:59Z-a#2`: a ledger tail read to copy the envelope spelling.
- E4 `2026-10-07T04:49:16Z-a#0`: the same tail read, unresolved.
- E4 `2026-10-07T10:07:43Z-a#0`: envelope keys read via git diff.

**Level hypothesis:** The nudge's condition and the envelope's format live in the ledger, and the ledger is closed to the agent. Each session opens it anyway.

**Proposal:** An `evolve.py` read verb that prints the per-epoch record counts, completion state and the envelope's current version/epoch spelling. The tool already owns the ledger.

### L8 — chronic-degrade — `tooling.hook-friction`, 1 in epoch (E1 2 · E2 2 · E4 2 · E5 1), never halted
**Facts:** The in-epoch case is `2026-10-10T09:23:13Z-e`: the Bash guard refused a cat heredoc with a file target twice in one step, the second time the same act as the first. Six of the seven records across all epochs are this same guard. The guard-refused signal reads 20 in this epoch, 7 of them at implement/code.

**Level hypothesis:** The guard works as designed. The act it refuses recurs every epoch, so the cost is a standing re-issue tax rather than a defect.

**Proposal:** The guard's refusal text could name the exact replacement form (Write tool + run by path). Another option is to fold the habit into the L2 tool. Or, the founder may judge the tax acceptable.

_Signatures checked, not fired:_
- **deferred-forever:** 6 deferrals. Three closed: `2026-10-09T19:28:10Z-a#1` → `2026-10-09T21:17:27Z-e`; `2026-10-10T14:22:26Z-a#0` → `2026-10-10T14:32:50Z-a`; `2026-10-11T09:55:09Z-a#1` → `2026-10-11T09:57:34Z-a` CARRY. Three are routed by their own notes: `2026-10-09T20:55:00Z-a#1`, `2026-10-09T21:19:51Z-a#0` and `2026-10-10T16:44:10Z-a#1`. Of these, `2026-10-09T21:19:51Z-a#0` and `2026-10-10T16:44:10Z-a#1` have no closure record in the ledger.
- **chronic-degrade `tooling.output-cap-overflow`** (E1 6 → E5 1): declining. It is read as L1's correlate.
- **`ok-degraded`:** 2 in this epoch, both curation, with different causes. Across epochs it has no single recurring cause.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — cross-step (curation · plan · reconcile) — 3 cases → proposed type `input.relayed-decision` (universal)
**Cluster:**
- `2026-10-10T00:34:22Z-c` (curation): two founder dialogs expired unanswered after 58 min; the answers came the next day by relay. Halted 1.
- `2026-10-10T08:21:37Z-c` (plan): the operator answered a founder dialog with every item "not answered"; the forks went to a file and came back by relay 5 h later. Dialogue 1.
- `2026-10-10T19:02:09Z-c` (reconcile): a third party's confirm arrived after the report existed; no step names how an input arriving at Validate is recorded.

Lookback: E4 `2026-10-07T02:45:21Z-b` (a delegated direction the trajectory gate does not name).

**Draft criteria line:** "`input.relayed-decision` — a decision owned by someone outside the session (founder, overseer) reached the step by relay (a message, a file, another session) instead of the step's own dialog, or its dialog timed out; record the channel, the delay and how the input was captured." It goes in evolve-system §Universal types. This is the friction-side face of L4.

### U2 — cross-step (research · fix-loop · gates) — 4 cases → proposed type `contract.authored-count-error`
**Cluster:** In each, a count or coordinate the agent wrote was wrong on first write, outside reconcile.
- `2026-10-09T19:50:06Z-d` (research): line citations taken from a view that printed three ranges unseparated.
- `2026-10-10T16:33:25Z-d` (research): two counts off by one.
- `2026-10-10T01:31:02Z-b` (fix-loop): research's lock-package count read 3 where 2.
- `2026-10-10T02:47:37Z-b` (gates): the reconcile step record says `amendments:27` where 24.

Lookback: E1 `2026-10-05T22:02:28Z-b`.

**Draft criteria line:** "`contract.authored-count-error` — a count, tally or line coordinate the agent itself wrote into an artifact or record was wrong; record whether it was caught in-pass or by a later step, and the derivation that corrected it." Reconcile's step-local `contract.in-pass-correction` stays the winner there. The founder may prefer instead to widen that type to universal.

### U3 — wrap-session (report · reconcile) — 4 cases → proposed type `tooling.citation-coordinate`
**Cluster:**
- `2026-10-09T20:55:00Z-b`: cites.py ends a while/case block one line early.
- `2026-10-09T21:17:27Z-g`: the citation tool reads bare backticked line numbers against the previous file's path.
- `2026-10-10T02:37:05Z-e`: the post-merge sweep left 129 rows to a scratch reading aid, with 129 extra reads.
- `2026-10-10T10:57:00Z-f`: a hand re-point matched a prefix twin (`:1-4` inside `:1-40`).

Lookback: E4 `2026-10-07T08:27:21Z-b`, `2026-10-07T11:08:36Z-b` (site-sweep tooling). Correlated with Pass-A theme "citation sweep has no reading aid" (3 facts: `2026-10-10T02:37:05Z-a#1`, `2026-10-10T10:57:00Z-a#2`, `2026-10-10T19:02:09Z-a#1`).

**Draft criteria line:** "`tooling.citation-coordinate` — the citation tooling (cites.py, the citation sweep) read, printed or left a file:line coordinate that a read showed wrong or unresolvable without a hand reading aid; record the grammar case and the rows it cost." Owning playbook: wrap-session. A tool-side direction is evident from the cases: print each changed row's citing window with its base and current lines, accept range re-points, and anchor re-points on the whole coordinate.

## Below threshold — no action

**Typed, per step**, each with n=2 and no halt unless stated:
- phase/validate `ambiguity.review-cycles` (n=2, w 7).
- implement/fix-loop `contract.test-expectation` (n=2; the same checks as X4).
- phase/research `contract.token-proxy-check` (n=2; counted by type alone in P12).
- wrap-session/route-resolve `tooling.long-line-edit` (n=2).
- implement/fix-loop `contract.instrument-validity` (n=2).
- implement/fix-loop `contract.premise-falsified` (n=2; in P13).
- phase/distill `contract.extract-format` (n=2).
- wrap-session/reconcile `contract.proposal-format` (n=2).

**Typed, singles:**
- route-resolve `ambiguity.trajectory-halt` (n=1, halted 1, below n≥2).
- code `tooling.hook-friction` (n=1, see L8).
- research `tooling.output-cap-overflow` (n=1, see L1).
- reconcile `ambiguity.escalation-rounds` · `ambiguity.playbook-no-match` · `contract.jointly-contradictory-instructions`.
- plan `retry.synthesis-rework` · `input.research-thin`.
- research `contract.instrument-validity` · `input.cookbook-gap` · `input.extract-signal-gap`.
- report `contract.narrow-basis-claim` · `contract.skill-reference-drift` (SKILL.md P1 names the cascade v1.4 stamp; v1.6 deployed) · `recall.change-reconstruction`.
- route-resolve `contract.grammar-irregularity` (`escher-0.1.0/working-route.md:71`, one site: the data) · `contract.premise-falsified`.
- code `contract.token-proxy-check`.
- orientation `input.handoff-git-mismatch`.
- take-up `tooling.long-line-edit`.
- curation `ambiguity.tier-routing`.
- gates `contract.narrow-basis-claim`.

**Typed, by type alone, not universal** (shown, not aggregated):
- `tooling.long-line-edit` n=3 over take-up and route-resolve. With the Pass-A route-line theme's 2 facts it makes 5 records in 3 chunks, all about working-route lines of 4–12 K characters. Watch next epoch.
- `contract.instrument-validity` n=3 over research and fix-loop.
- `contract.narrow-basis-claim` n=2 (universal, no halt).

**Untyped:**
- U4 `cascade.py sweep --pattern` repeated flags (n=2: `2026-10-12T09:00:00Z-b`, `2026-10-12T20:41:49Z-c`, plus P2's recall record). Not recurring from Epoch 4.
- Singles, 8:
  - gh issue search, four words empty
  - a restore edit from a recalled line
  - an until-loop to the 600 s ceiling
  - state-dir-too-long under the scratchpad
  - serde_json member order
  - a directive vs P2's opening sweep
  - rustfmt red after scripted edits (L2)
  - a comparer's background-work notice

**Pass-A themes below threshold:**
- Route entry lines too long for the tool's views (2 facts; see above).
- Plan step vs product reality (4 facts, nature product-logic, so not a band-aid signature; P3's correlate).
- Removed-cause singles: a stale graph DB regenerated; a stand check that opened a task through the hit defect; a cd the guard passed.

**Single overrides:**
- The ladder rung vs the operator's pause (an era).
- The 0-pending citation sweep (P10).
- The founder dialog returned unanswered, and founder forks answered as technical (L4).
- A help-text restatement (product-logic).
- The SKILL.md cascade stamp v1.4 vs deployed v1.6. That is a pipeline letter, already in the handoff for the pipeline owner.

**Unresolved facts:**
- `2026-10-10T02:47:37Z-a#0`: the gates playbook was read before its step ran.
- `2026-10-10T09:40:57Z-a#0`: a hygiene pre-read refused a phase run-dir file, left for the operator pass (X3).
