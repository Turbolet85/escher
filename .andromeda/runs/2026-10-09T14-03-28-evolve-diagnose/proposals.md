# Evolve Diagnosis — escher-0.1.0 · Epoch 4 — Driver core · 2026-10-09T14:18:00Z

Read-only diagnosis of `.andromeda/friction-log.ndjson`, filtered to the epoch. Nothing here is applied, queued or remembered; each item is open to accept, reject, defer or modify. Records are cited by ledger line (`L###`) and `id`; chunk markers are shown without their shared `2026-10-07-` prefix. Raw query outputs sit beside this file as `q-*.json`.

## Mechanism health

- **Records:** 246 (137 step / 109 friction) of the ledger's 704 · unparseable: 0 · malformed-ts: 0 · ids: 246 of 246, none duplicated, each led by its `ts` · `ts` order: monotonic across the ledger.
- **Coverage:** 9 chunks, 0 checkpoint gaps against the expected 13 step records per chunk (phase 5 · implement 3 · wrap 5). Repeats, all explained by the records themselves: `audit-corrections` ran research, plan, validate and all three implement steps twice (a plan revision and an implement re-entry — 19 step records); `driver-session` and `refusal-detection` each ran `phase/plan` twice (a re-synthesis after the review — 14 each). Outside a chunk: 10 `new-session/orientation` records (10 session starts) and one no-op wrap at 02:44Z carrying `curation` and `route-resolve` (the adaptation form).
- **Step outcomes:** ok 132 · halted-resolved 3 (L643 take-up, L664 and L695 reconcile) · ok-degraded 2 (L476 fix-loop, L671 curation). No friction record carries halt or soft-exit impact, so the n >= 2 arm of the threshold fires nowhere in this epoch.
- **Untyped rate:** 14 of 109 friction records (13%). Per step (untyped / friction): implement/code 3/15 · implement/fix-loop 0/3 · implement/smoke 0/1 · new-session/orientation 1/5 · phase/distill 0/6 · phase/plan 1/6 · phase/research 1/4 · phase/take-up 1/6 · phase/validate 0/8 · wrap-session/curation 0/13 · wrap-session/gates 1/2 · wrap-session/reconcile 3/26 · wrap-session/report 2/7 · wrap-session/route-resolve 1/7.
- **Problem-fact fill:** 60 of 137 step records carry a deviation fact (82 facts: 65 workaround · 8 overridden · 3 removed-cause · 3 unresolved · 2 deferred · 1 prohibition). Evidence pointers: 55 of 109 friction records; 53 name a repository path and 52 of those exist (L670's `sidecar-2026-10-07-refusal-detection.json` does not).
- **Spellings:** the `epoch` label has one spelling. `skill` has two per skill (`andromeda-{name}` 172 · `{name}` 74), folded before every group. `version` has two: `escher-0.1.0` on 233 records and `0.1.0` on 13 — L513 to L525, one window (an orientation and the driver-session phase run), which L513 #0 explains: the version form was to be copied from the ledger, the read failed, and the value 'was written from the contract's wording unverified'. No stage groups by `version`.
- **Retractions (whole-ledger pre-pass):** retracted 0 (1 unresolvable) · clause-retracted 1 (kept, note rendered; its target L118 is an Epoch 1 record and appears in no table here) · no retraction targeted by a retraction. Unresolvable, verbatim, for manual discount — on L493 `2026-10-07T04:21:25Z-a`: `{"id": null, "note": "in step record 2026-10-07T04:21:04Z-a (a step record, so no record or clause scope applies), produced git-state note: discount 'add -A staged 43 paths' — the measured count is 47 files changed"}`.
- **Reading-order and never-read rules, as recorded by the steps themselves:** a checkpoint's playbook opened before its step had completed, twice (L478 #0, L614 #0); the ledger read against the never-read rule, twice (L513 #0, L564 #0), with `contract.jointly-contradictory-instructions` L496 naming the nudge condition that needs such a read.
- **Zero-cost untyped records:** 3 (L559, L583, L690) carry no counted impact; L559 reads as a positive measurement (a rule resolving without a halt), which the record shape assigns to `signals`.
- **Calibration boundaries in range:** the report step's `operator-pass` entry is live from 2026-10-07 — 4 of the 9 report records carry it (from L606 on); its absence on the five earlier ones is era, not 'no pass ran'. The reconcile record's `rejected-for-source` / `rejected-for-coordinate` counts and the report's `new-text-*` words deploy 2026-10-08, after every record here — rejections of that class live in free wording (P2, P10, the untyped L534). `contract.in-pass-correction`, the deviation scan, the Universal types and required ids are all live across the range.

## Proposals (typed patterns)

46 groups by (skill, step, type); 16 reach the threshold (n >= 3). Two of those are untyped groups, handled under the extension candidates; one (`contract.skill-reference-drift` at orientation, n 3) is shown by type across steps with its fourth record (P12); and one further pattern reaches the threshold only when a Universal type is grouped by type alone (P4). Order is n x weight.

### P1 — wrap-session/reconcile · `contract.in-pass-correction` — 9 cases · weight 24 · rate 9/9 runs · chunks: 7
**Pattern:** Seven of the nine reconcile runs recorded first writes of their own corrected before the commit — 9 records, three of them at refusal-detection; audit-corrections and upstream-sync-driver-core recorded none. The corrections fall into typed-before-counted tallies (535, 558, 669), the sidecar entry's 3000 B cap or form (637, 670), cited ranges (535, 698), statements of work not done ('re-derived', 'read and left', 're-measured' — 586, 669, 698) and prose slips caught on re-read (637, 668).
**Evidence:** ALL 9 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-session | 4 first writes corrected before the commit: two cited ranges one line short; two sidecar payloads miscounted ratified clauses; one sweep time estimated, then read from the listing's file time | retries 4 | L535 `2026-10-07T07:18:59Z-c` |
| sink-target-allowlist | fanout-results.md said five of the six escalate-graded obs-plan proposals were named by the expected amendments; four are — caught on a re-read against the proposal list | retries 1 | L558 `2026-10-07T08:48:45Z-d` |
| settle-detection | cascade-dispositions.md said a leaf row was re-derived with its section; the leaf was then read and left unchanged — row corrected before any sidecar entry landed | retries 1 | L586 `2026-10-07T11:28:53Z-c` |
| command-and-refusal-schema | fanout-results.md's parsed lists first written with two leading spaces stripped per line (an assumed display indent), altering the YAML; rebuilt from the extracted files unaltered | retries 1 | L609 `2026-10-07T12:50:42Z-c` |
| act-by-id | 7 first writes corrected: a lost joining word; a false 'no pass of their own'; a wrap dated a day early; a dash-semicolon join; two sidecar payloads over the 3000 B cap; all seven payloads missing the leading blank line; three raw twins hand-written, then rewritten by script | retries 2, iterations 7 | L637 `2026-10-07T14:57:13Z-e` |
| refusal-detection | test-plan.md:26 — the first write dropped the separator before `stand_act_range`; caught on re-read | retries 1 | L668 `2026-10-07T21:03:54Z-e` |
| refusal-detection | cascade-dispositions.md said '20 leaf rows over 9 files' and 'read and left' for ten leaves; a recount read 21 over 8, and those leaves had been grepped, not read whole | retries 2 | L669 `2026-10-07T21:03:54Z-f` |
| refusal-detection | architecture-entry.md was 3060 B on first write, over the 3000 B entry cap; `sidecar.py check` refused it; cut to 2991 B | retries 1 | L670 `2026-10-07T21:03:54Z-g` · .andromeda/runs/2026-10-07T20-27-47-wrap/sidecar-2026-10-07-refusal-detection.json |
| driver-command-spans | 3 corrections: 'every by-level reading re-measured' where one was; a feature-graph reading stated as measured; `sink_layer` cited 102-117 at 4 sites where the function ends at 116 | iterations 3 | L698 `2026-10-07T23:06:24Z-d` |

**Proposal:** The type records a catch, so the cost here (retries 13, iterations 10) is the price of the re-read working; two sub-classes have a removable cause. (a) The 3000 B sidecar cap was overrun in three chunks — 637 and 670 here plus the untyped L587 (settle-detection, two payloads off-form twice) — each time when one entry had to carry many edits (14 and 24 at L587): a direction is a sanctioned split form for a many-edit entry, or a size pre-flight the authoring step runs before the payload is finished. (b) Tallies written into fanout-results.md and cascade-dispositions.md (558, 669, and 535's clause counts) were typed and then recounted: a direction is for the tool that owns each listing to print the tally the record quotes. For the founder's eye: the group's count per epoch reads 7 / 4 / 7 / 9 over 7 / 5 / 6 / 9 reconcile runs, so the count per run has not moved across four epochs.

### P2 — wrap-session/reconcile · `input.report-insufficient` — 6 cases · weight 24 · rate 6/9 runs · chunks: 6
**Pattern:** In 6 of 9 reconcile runs the report's site lists or coordinates fell short of what the detectors needed: no coordinates for new tests or test modules (585, 608, 696), and site lists built from token or fixed-string searches that under-ran the restatements (556, 608, 634, 665); the detectors then read the tree against their prompt's ban and their proposals were rejected as proposed and re-raised (reformulations 22, extra reads 21).
**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| sink-target-allowlist | site list counted 2 architecture hits of the superseded phrase without reading them (both another sense); the coordinate table listed symbols, not the composite ranges the masters cite — three detectors opened source to derive ranges | extra reads 3 | L556 `2026-10-07T08:48:45Z-b` |
| settle-detection | 8 of 25 proposals carried a basis or coordinate read from the tree: the report gave ranges for the new public symbols, none for the new tests; each rejected as proposed and re-raised | — | L585 `2026-10-07T11:28:53Z-b` · .andromeda/runs/2026-10-07T11-06-31-wrap/fanout-results.md |
| command-and-refusal-schema | the count-site search matched one word order only and read a keyed contract as no-change; no coordinates for three new test modules — two proposals rejected as proposed and re-raised | extra reads 2, reformulations 3 | L608 `2026-10-07T12:50:42Z-b` · .andromeda/runs/2026-10-07T12-34-00-wrap/fanout-results.md |
| act-by-id | moved claims were located by token search; the detectors found nine more sites worded otherwise across four masters; all applied | extra reads 3, iterations 1 | L634 `2026-10-07T14:57:13Z-b` · .andromeda/runs/2026-10-07T14-22-35-wrap/fanout-results.md |
| refusal-detection | site lists wrong or short in 13 places: lib.rs citation moves omitted, one line read as unmoved, one mis-cited symbol, ten restatements the fixed-string counts did not reach | extra reads 13 | L665 `2026-10-07T21:03:54Z-b` · .andromeda/runs/2026-10-07T20-27-47-wrap/fanout-results.md check 4 |
| driver-command-spans | 19 of 56 proposals cited source locations the report does not carry; three detectors said they read line numbers from the tree, against the prompt's ban; all rejected as proposed, re-raised, applied | reformulations 19 | L696 `2026-10-07T23:06:24Z-b` · .andromeda/runs/2026-10-07T22-32-46-wrap/fanout-results.md |

**Proposal:** The cause sits upstream of reconcile — see chain X1 and level candidate L8. Directions: a shipped report-step tool that prints (i) an old-to-new coordinate map for every symbol the diff moves, new test functions and composite ranges included, and (ii) occurrence-level site counts with the hit text, so the report's lists stop being hand-built from token searches; and a report-template bullet for the coordinates of new tests. Mid-epoch the reports began to carry a citation map built by scratch script (signal `citation-map-carried` at act-by-id; `contract.detector-fact-gap` at L632 and L662), which did not stop the class (665, 696 follow it) — the remaining proposal is the tool. The reconcile record's `rejected-for-source` / `rejected-for-coordinate` counts deploy on 2026-10-08, after this epoch, so these rejections live only in free wording here.

### P3 — phase/validate · `contract.mechanical-check` — 6 cases · weight 8 · rate 6/10 runs · chunks: 5
**Pattern:** Six plan defects surfaced at validate in five chunks: three are the same check 4 (2) miss — a boot-path touchpoint in the modify-set with no smoke-role entry, each caught by a hand read at P5 (525, 625, 687); two are defects no mechanical predicate attempts, caught only by the operator's review and each costing a return to P4 (524, 653); one is a baseline that failed for a reason other than the predicted red (474).
**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections | check 4 (9): the new mutation entry's baseline exited 1 in 0.8 s (cargo-mutants refuses an --output with no parent dir) where a red exit 2 was expected; `mkdir -p` added, re-run read 7 missed | retries 1 | L474 `2026-10-07T03:27:14Z-b` · .andromeda/runs/2026-10-07T02-48-43-phase/gate-2026-10-07-audit-corrections.json |
| driver-session | the operator's review caught a defect no mechanical predicate attempts: the new library crate depended on the example crate; the plan went back to P4 and was re-synthesized whole | iterations 1, dialogue rounds 1 | L524 `2026-10-07T05:31:15Z-b` · .andromeda/runs/2026-10-07T04-50-09-phase/relay-2.md |
| driver-session | check 4 (2), read by hand: a new binary boot path with no smoke-role entry; an existing entry re-roled to smoke | — | L525 `2026-10-07T05:31:15Z-c` |
| act-by-id | check 4 (2): no smoke entry while the modify-set holds the session host source; caught at P5's read, an entry re-keyed and a windowed boot smoke added | iterations 1, extra reads 3 | L625 `2026-10-07T13:28:25Z-b` · .andromeda/runs/2026-10-07T13-00-01-phase/planlint-2026-10-07-act-by-id.json |
| refusal-detection | the first plan passed every mechanical check while its off-screen remedy could not be followed on the stand (viewport-only scroll, nested scrollers); the operator's review caught it and the run returned to P4 | iterations 1 | L653 `2026-10-07T18:49:37Z-b` · .andromeda/runs/2026-10-07T15-18-58-phase/relay-3.md |
| driver-command-spans | check 4 (2): the plan stated no boot path was edited while its modify-set held the sink both stand binaries install; resolved at P5 by listing the standing smoke entry | iterations 1, extra reads 2 | L687 `2026-10-07T21:46:37Z-b` · .andromeda/runs/2026-10-07T21-16-38-phase/planlint-2026-10-07-driver-command-spans.json |

**Proposal:** For the three check 4 (2) cases a direction is to make the check mechanical at P4 authoring — the plan lint comparing the modify-set against the project's known boot-path sources and asking for a smoke-role entry — since the same hand catch recurred in three plans by the same author-step. For 524 and 653 the ledger shows the review as the only catcher (see L12); whether a P4 self-question ('can each stated remedy be followed on the stand', 'which way does the new dependency point') is worth adding is the founder's judgment.

### P4 — across steps (wrap-session/gates 1, wrap-session/route-resolve 1, phase/research 2, phase/take-up 1) · `contract.narrow-basis-claim` — 5 cases · weight 4 · chunks: 5
**Pattern:** A universal type firing once or twice at each of four steps (research 2, take-up 1, gates 1, route-resolve 1) — below threshold at every step, n = 5 by type: a count or a cause was stated from a source narrower than the claim — a clipped excerpt (570), a query key mixing two definitions (622), a typed count (493), an unmeasured handoff note (511), a config rule read as a cause (644, the one case that cost a dialogue round).
**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections | the gates step record stated a staged-path count typed without a reading; `git show --stat` reads 47 files [this record carries the unresolvable `retracts` entry listed in Mechanism health] | extra reads 1 | L493 `2026-10-07T04:21:25Z-a` · git show --stat 8d156de1 |
| upstream-sync-driver-core | the previous handoff's note (repeated in scope.md) read three tail lines as our only new lines in dioxus_document.rs; git diff reads 58 added lines in 4 hunks — found because the wrap measured the note before pinning it as a CARRY | extra reads 3 | L511 `2026-10-07T04:46:13Z-b` · escher-0.1.0/chunks/2026-10-07-upstream-sync-driver-core/report.md (Decisions and corrections, the P5 line) |
| settle-detection | two scope citations counted off a clipped sed excerpt read one line low; corrected by grep -n before the plan used them | extra reads 1 | L570 `2026-10-07T10:25:51Z-b` |
| act-by-id | research stated 13 outside call sites from a calls query keyed on callee name plus crate, which mixes two `start` definitions; keyed on the definition's file it reads 10 | retries 1 | L622 `2026-10-07T13:16:02Z-b` · .andromeda/runs/2026-10-07T13-00-01-phase/tree-query-2026-10-07-act-by-id.json |
| refusal-detection | the Setup 5a halt named a superseding push as the probable cause of a red with no failed check, from ci.yml alone; the run had ended before the next was created and Actions had an outage, which the operator supplied | dialogue rounds 1, extra reads 3 | L644 `2026-10-07T15:23:37Z-b` · .andromeda/runs/2026-10-07T15-18-58-phase/relay-1.md |

**Proposal:** The shared act in 493, 570 and 622 is a number written before the command that measures it ran — the same act as the typed tallies in P1 and the estimated stamps in L9. Site-specific directions: the code-graph cookbook's calls template keyed on the definition's file rather than callee name plus crate (622); the Setup 5a halt wording stating 'cause not established' where no failed check is printed (644, see L7).

### P5 — wrap-session/reconcile · `contract.cascade-miss` — 4 cases · weight 5 · rate 4/9 runs · chunks: 4
**Pattern:** In four chunks a restatement survived the applies because no proposal named it: three sat in the very line or section an apply had just edited (536, 557, 636), and one was a sibling master's copy of a clause another detector had escalated (697).
**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-session | test-plan §3 kept 'the one statement of what the stand checks share' after a second shared module was added; the sweep's pattern listed the line | retries 1 | L536 `2026-10-07T07:18:59Z-d` · .andromeda/runs/2026-10-07T06-51-52-wrap/cascade-dispositions.md |
| sink-target-allowlist | after 40 applies: 3 restatements in masters found by the sweep, and 3 leaf lines carrying none of the swept wording found only by reading the leaf whole | extra reads 2, iterations 2 | L557 `2026-10-07T08:48:45Z-c` |
| act-by-id | a11y-plan.md:14 kept an older clause contradicted by the clause this pass appended to the same line; no proposal named it (the a11y detector returned none) | iterations 1 | L636 `2026-10-07T14:57:13Z-d` |
| driver-command-spans | security-plan restates the founder-ratified diff clause the architecture detector escalated; the security detector did not propose it; the sweep found it on a 10 501-char line | extra reads 2 | L697 `2026-10-07T23:06:24Z-c` · .andromeda/runs/2026-10-07T22-32-46-wrap/cascade-dispositions.md |

**Proposal:** Directions: after an apply, re-read the whole edited line or bullet for a clause the edit now contradicts (the masters' bullets are long single lines, so an appended clause and the older one sit side by side — 636, 557); and when one detector escalates a clause, sweep its wording across every master before validation rather than waiting for the cascade sweep (697). 557's three leaf lines carried none of the swept wording and were found only by reading the leaf whole — a pattern sweep cannot reach that class by construction.

### P6 — implement/code · `input.plan-step-ambiguous` — 6 cases · weight 3 · rate 6/10 runs · chunks: 6
**Pattern:** In six chunks a plan step proved underdetermined or at odds with a neighbouring step once code was written: a mechanism the plan asserts that the language does not allow (578 a private field read from a sibling module, 689 a table feeding a macro that takes literal tokens, 527 a byte bound its own reply exceeds), an unstated case (549, 602), or a screen fact the plan did not hold (627). Each was settled in-intent and carried to the report; total cost iterations 2.
**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-session | step 4 bounds a line at 64 bytes while its own hello reply is 87 at its widest; settled as 64 for a request, 128 for a reply | — | L527 `2026-10-07T05:45:20Z-b` |
| sink-target-allowlist | step 4 names three third-party records but not their levels; step 5 has format_event ask a verdict of the target where decide takes a field | — | L549 `2026-10-07T08:06:55Z-b` |
| settle-detection | step 3 names 'one private field' whose readers steps 4 and 6 place in a sibling module; settled as pub(crate) | — | L578 `2026-10-07T10:47:59Z-d` |
| command-and-refusal-schema | step 4 does not say what validation returns for a listed verb in a shape no Command variant has; implemented as unknown-verb | — | L602 `2026-10-07T12:16:04Z-b` |
| act-by-id | three steps rested on a screen fact the plan did not hold (no boot-effect control on the Timer; click-then-keys leaves the slider unfocused; mutation 4 does not isolate the clamp) | iterations 1 | L627 `2026-10-07T13:54:47Z-b` |
| driver-command-spans | step 2 asks for the field names 'as a private table' while tracing's span macro takes literal tokens; settled by a macro_rules | iterations 1, extra reads 1 | L689 `2026-10-07T21:59:30Z-b` |

**Proposal:** The plans that produced these read clean on both authoring self-checks (see chain X2) — the fence dry-run and the lint test the gate block's shape, not a step's internal consistency. A direction: a P4 self-read question per step — 'which other step reads what this step creates, and can it reach it as stated' — or leave it as designed, given the near-zero cost and that each case reached the report. The count per epoch reads 4 / 3 / 4 / 6.

### P7 — phase/distill · `retry.distiller-respawn` — 3 cases · weight 6 · rate 3/9 runs · chunks: 3
**Pattern:** Five distiller retries in three chunks, all on check 3 (Anchors), and four of the five on the same defect: a Constraints item citing its section by back-reference ('the same row', 'the three behaviours above') instead of carrying its own anchor.
**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections | arch, security and layouts failed check 3 (Anchors): Constraints items cited their section by back-reference; one retry each passed | retries 3 | L468 `2026-10-07T03:02:42Z-b` · .andromeda/runs/2026-10-07T02-48-43-phase/.raw-arch.md .raw-security.md .raw-layouts.md |
| upstream-sync-driver-core | tests distiller re-spawned on check 3: one Constraints item cited no test-plan section | retries 1 | L500 `2026-10-07T04:31:21Z-b` |
| driver-session | layouts distiller re-spawned on check 3: four Constraints items cited their section by back-reference | retries 1 | L517 `2026-10-07T05:00:44Z-b` · .andromeda/runs/2026-10-07T04-50-09-phase/.raw-layouts.md |

**Proposal:** A direction: state the rule in the distiller prompt with the failing form shown ('every Constraints item names its own section; no back-reference to a neighbouring item'). The last case is at 05:00Z (driver-session); the six later distill runs of the epoch record none, and the ledger does not say whether a prompt change absorbed it. Epoch 3 recorded the type 4 times.

### P8 — wrap-session/curation · `recall.corpus-recurrence` — 4 cases · weight 4 · rate 4/10 runs · chunks: 4
**Pattern:** Four curation runs recorded a learning that recurred after it had been curated correctly: the per-crate clippy trap three times (589, 611, 701), the heredoc-file-target guard (589) and the backslash-pair collapse (674).
**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| settle-detection | the heredoc-file-target guard was hit again at implement (a Tier-3 entry and an auto-memory line exist); the per-crate clippy trap (Tier 3) was run and reported as a new observation | retries 1 | L589 `2026-10-07T11:29:30Z-b` |
| command-and-refusal-schema | the per-crate clippy trap reproduced at implement in the session that had just read the handoff line listing it as recurring; Tier 3 loads on demand and was not opened | retries 1 | L611 `2026-10-07T12:52:59Z-b` · .andromeda/runs/2026-10-07T12-34-00-wrap/curation.md |
| refusal-detection | a backslash pair collapsed by the Bash transport in an inline heredoc cost a repeated job-log parse; the host rule file already states the collapse and the remedy | retries 1 | L674 `2026-10-07T21:06:01Z-d` · escher-0.1.0/chunks/2026-10-07-refusal-detection/report.md (Decisions & corrections) |
| driver-command-spans | a per-crate clippy run at implement, red on blitz-dom, discarded — the fourth recurrence; a Tier-3 entry and an auto-memory entry state the rule | — | L701 `2026-10-07T23:08:04Z-c` |

**Proposal:** The same recurrence was also recorded where it happened: untyped at implement/code (L603, L690) and as deviation facts (L526 #1, L601 #0). 611 names the mechanism — the entry is Tier 3, loaded on demand, and was not opened before the run — and 701 records that an auto-memory entry existed as well. Directions, pipeline-level: a curation rule that a second recurrence of a correctly-stated entry moves it to a tier that loads (or converts it into a mechanical guard, as the project's cd and heredoc guards already are); and a loaded home for host and tooling learnings (see P11). The type's count per epoch reads 5 / 7 / 7 / 4.

### P9 — wrap-session/curation · `ambiguity.filter-borderline` — 6 cases · weight 2 · rate 6/10 runs · chunks: 5
**Pattern:** In six curation runs a candidate could not be scored from the letter; five of the six sit at exactly 0.6 on Filter 4 (490, 561, 590, 673, 700), and in four of those the open question is what counts as an 'other durable home' — the same wrap's sidecar entry (490), a master this wrap amended (590, 700), the operator's auto-memory (673).
**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | Filter 1: the operator asked to apply a learning already standing in Tier 3; a dedup reject, an in-place extension and a Tier-1 pointer were all readable from the letter | extra reads 2 | L461 `2026-10-07T02:44:20Z-b` · .andromeda/runs/2026-10-07T02-41-58-wrap/curation.md |
| audit-corrections | a candidate at exactly 0.6 on Filter 4; whether the same wrap's sidecar entry counts as a durable home is not stated | — | L490 `2026-10-07T04:16:57Z-b` · .andromeda/runs/2026-10-07T03-59-09-wrap/curation.md |
| sink-target-allowlist | Filter 4: a hook refusal is none of the three named proofs, so +0.4 was withheld (0.2); with it the score sits exactly at 0.6 | — | L561 `2026-10-07T08:50:06Z-b` |
| settle-detection | two measured facts at exactly 0.6; both rejected by the exact-hit rule because this wrap amended them into masters | — | L590 `2026-10-07T11:29:30Z-c` |
| refusal-detection | exactly 0.6: the no-other-durable-home signal judged for a fact saved to the operator's auto-memory, a home the filter's list does not name | — | L673 `2026-10-07T21:06:01Z-c` · .andromeda/runs/2026-10-07T20-27-47-wrap/curation.md |
| driver-command-spans | exactly 0.6 after Filter 1 read it as a facet of an existing Tier-3 entry; rejected because this wrap amended the fact into two masters | iterations 1 | L700 `2026-10-07T23:08:04Z-b` |

**Proposal:** Directions for curation-tier-decision.md Filter 4: say whether 0.6 itself passes, and enumerate the durable homes the no-other-home signal reads (a sidecar entry, a master amendment made in the same wrap, an auto-memory entry); the weights 'measurement 0.4 + technical detail 0.2' (590) land on the threshold by arithmetic, so the boundary case is the common case. Epoch 3 recorded the same exact-0.6 case once (L344).

### P10 — wrap-session/reconcile · `contract.false-positive-proposal` — 3 cases · weight 4 · rate 3/9 runs · chunks: 3
**Pattern:** In three chunks a detector's proposal rested on its own read of the tree (a grep, a diff read, a source range) rather than on the report; each was rejected as proposed and its fact re-raised by the orchestrator.
**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| audit-corrections | D-security-input cited a source range the report does not carry and added two claims beyond it; rejected as proposed, its fact re-raised from the report alone | extra reads 1 | L488 `2026-10-07T04:15:42Z-b` · .andromeda/runs/2026-10-07T03-59-09-wrap/fanout-results.md |
| act-by-id | D-obs-stack O2 rested on the detector's own grep over the new test files; rejected as proposed, re-raised from the orchestrator's census | extra reads 1 | L635 `2026-10-07T14:57:13Z-c` |
| refusal-detection | one D-obs-stack proposal rested on the detector's own grep; the lib.rs halves of two D-arch-resources proposals on its own diff read — rejected or narrowed, re-raised | reformulations 3 | L666 `2026-10-07T21:03:54Z-c` · .andromeda/runs/2026-10-07T20-27-47-wrap/fanout-results.md |

**Proposal:** The same act appears in P2's records (585, 608, 696) and in the untyped L534 (all five proposal-carrying detectors read the tree, 75 facts re-entered): the detector prompt's ban on reading the tree was not held in 8 of the 9 reconciles (P2's 556 records it for sink-target-allowlist as well; the ninth, L508, returned zero proposals). Directions: give the detectors a sanctioned coordinate source (P2's tool) so the read is not needed, or enforce the ban mechanically through the sub-agents' tool allowlist — the ledger shows the prose ban alone does not hold.

### P11 — wrap-session/curation · `ambiguity.tier-routing` — 3 cases · weight 2 · rate 3/10 runs · chunks: 2
**Pattern:** Three curation runs had a host, tooling or CI-reading learning with no Tier-2 home: the registry's host class file is host-win32.md, not rendered on a POSIX host (462, 538), and no rule file's paths cover reading a CI run (672).
**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | a host-shell learning had no Tier-2 home: the registry's class file host-win32.md is not rendered on a POSIX host; routed to Tier 1 by the path-scopability tiebreaker | extra reads 1 | L462 `2026-10-07T02:44:20Z-c` · .andromeda/runs/2026-10-07T02-41-58-wrap/curation.md |
| driver-session | two host and tooling candidates (an unset TMPDIR; per-crate clippy) had no rule file covering them; both went to Tier 3, where nothing loads them before the next occurrence | — | L538 `2026-10-07T07:20:44Z-b` · .andromeda/runs/2026-10-07T06-51-52-wrap/curation.md |
| refusal-detection | an imperative learning about reading a fork CI run had no Tier-2 home (no rule file's paths cover a CI read); it went to Tier 3 | reformulations 1 | L672 `2026-10-07T21:06:01Z-b` · .andromeda/runs/2026-10-07T20-27-47-wrap/curation.md |

**Proposal:** By 21:06Z the ledger cites `.claude/rules/host-linux.md` as holding a host rule (L674), so the host-class gap of 462 and 538 appears absorbed mid-epoch; the remaining direction is the general one — a registry class for learnings about a tool or an external service (a CI read, a lint form) that no source path scopes, so they do not default to Tier 3 'where nothing loads them before the next occurrence' (538) and feed P8.

### P12 — across steps (new-session/orientation 3, phase/plan 1) · `contract.skill-reference-drift` — 4 cases · weight 1 · chunks: 1
**Pattern:** Four records by type (orientation 3, plan 1). Three are one site — session-state-contract.md's expected-transient list (:57) does not name the wrap run dir's tracked evolve trail, which the wrap's post-commit evolve append modifies, so every session after a wrap starts on a dirty tree the handoff calls clean (565, 616, 680). The fourth is the inputs tool's step set (521).
**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| (no chunk) | a third modified tracked file at session start (the wrap run dir's evolve trail, +40 lines) is not among session-state-contract.md's expected-transient members | extra reads 1 | L565 `2026-10-07T10:07:43Z-b` |
| (no chunk) | same site: a tracked evolve trail (+40) beside the ledger's +1, not named by the contract's list (references/session-state-contract.md:57) | extra reads 2 | L616 `2026-10-07T12:59:24Z-b` |
| (no chunk) | same site: the handoff reads clean, git reads 3 modified; the third is the evolve trail grown by the wrap's post-commit append | extra reads 3 | L680 `2026-10-07T21:15:25Z-b` · git diff --stat at session start: 3 files, 45 insertions |
| driver-session | inputs-contract.md's --step set (phase:P1, phase:P3, implement) has no value for a relay answering a P4 fork, which SKILL.md P4 records; the snapshot was written as phase:P3 | extra reads 1 | L521 `2026-10-07T05:17:31Z-c` |

**Proposal:** The session-start site has six records in the ledger: these three, the untyped L495 earlier in this epoch, and two in Epoch 3 (L374 as `input.handoff-git-mismatch`, L431). Directions: name the trail in the contract's list, or remove the cause — append the wrap's last evolve records before the commit, or keep the trail untracked — so a post-commit write no longer touches a committed file. 521 is the same obstacle as level candidate L5.

### P13 — implement/code · `contract.premise-falsified` — 4 cases · weight 1 · rate 4/10 runs · chunks: 3
**Pattern:** In three chunks a plan step stated something about the current state of the code or screen that a measurement at implement refuted: a predicted not-red (576), a flag predicted false at boot (577), a box's bounds after a scroll (656), a check whose environment variable is inert on its child (528). Cost: extra reads 3, no redo.
**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-session | step 12's quiet check sets RUST_LOG=trace on a child that installs no sink; a hand probe of the sink-installing binary read ids and a row name on stderr at trace | extra reads 2 | L528 `2026-10-07T05:45:20Z-c` |
| settle-detection | plan and research predicted the Timer step not red before the fix; measured red in the order test 1 uses | — | L576 `2026-10-07T10:47:59Z-b` · escher-0.1.0/chunks/2026-10-07-settle-detection/evidence/red-first.md |
| settle-detection | step 9 test 2 asserts has_changes() false after an idle act on a fresh session; measured true at boot on all four tasks | — | L577 `2026-10-07T10:47:59Z-c` · escher-0.1.0/chunks/2026-10-07-settle-detection/evidence/red-first.md |
| refusal-detection | step 10 asserts the scrolled row's centre inside crud-list's bounds after the scroll; the list's own bounds read 18 lower though it did not move | extra reads 1 | L656 `2026-10-07T19:11:24Z-b` · escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md |

**Proposal:** All four are predictions a short probe at P5 would have read — the plan already baselines its gate entries there. A direction: where a step's assertion depends on a state the plan predicts rather than cites, P5 runs the prediction as a probe beside the baselines; or leave as designed, since implement caught each at no redo cost. See chain X2.

### P14 — wrap-session/route-resolve · `contract.carry-no-owner` — 4 cases · weight 1 · rate 4/10 runs · chunks: 4
**Pattern:** In four chunks a follow-up reached route-resolve with a disposition but no owning entry — the operator's direction named 'a route owner' and no entry (676, 703), a ruling left the slot to route-resolve (640), or the owner was named by a condition (540) — and the wrap placed it.
**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| driver-session | two follow-ups name their owner by a condition, not an entry; both pinned on Act by id as the first plausible owner; the new entry minted with no capability | extra reads 1 | L540 `2026-10-07T07:22:16Z-b` |
| act-by-id | the ruling minted an entry and left its slot to route-resolve, which reads slot ambiguity as a trajectory halt; the replaced carry's select half got no owner | extra reads 2 | L640 `2026-10-07T14:58:48Z-b` |
| refusal-detection | the operator's direction named four findings and 'a route owner' but no entry; placement was the wrap's (three on an entry already carrying seven blocks) | extra reads 4 | L676 `2026-10-07T21:08:16Z-b` · .andromeda/runs/2026-10-07T20-27-47-wrap/operator-rulings.md |
| driver-command-spans | the direction named one fact's disposition and no entry; placed by the wrap under the project's standing direction that placement is the wrap's | extra reads 1 | L703 `2026-10-07T23:09:17Z-b` |

**Proposal:** The letter reads slot ambiguity after a partial direction as a trajectory halt (640, and the untyped L464: 'no delegated form'), while the work proceeds without one under a project-level standing direction that placement is the wrap's (703; deviation fact L639 #0). A direction: give route-resolve.md the delegated form — the direction names the disposition, the wrap places by dependency, states the placement and its reason in the pin and lists it in the handoff for review — so the project's standing direction stops routing around the letter. Count per epoch: 0 / 2 / 4 / 4.

### P15 — phase/distill · `contract.extract-format` — 3 cases · weight 1 · rate 3/9 runs · chunks: 3
**Pattern:** Three distill runs recorded history files whose bold spans go beyond the prompt's letter — an inner **Kept** or **Why** span, a label inside the bold beside the marker (14 of 14 items in the obs history, twice) — which `sidecar.py cites` reported as not-a-marker rows or resolved by the marker-shaped token. Cost: none.
**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| settle-detection | two history files carry a second bold span **Kept** inside an item; `sidecar.py cites` printed a not-a-marker row for each; no fix owed | — | L568 `2026-10-07T10:18:38Z-b` |
| command-and-refusal-schema | three shapes outside the prompt's letter: a third receipt line; a label inside the bold beside the marker on 14 items; two non-marker bold spans | — | L596 `2026-10-07T11:54:44Z-b` · .andromeda/runs/2026-10-07T11-45-39-phase/ |
| act-by-id | obs history: 14 of 14 bold spans hold the marker plus a label where the prompt asks for the marker alone; resolved by the marker-shaped token; 2 inner **Kept** spans in the security history | — | L620 `2026-10-07T13:10:38Z-b` · .andromeda/runs/2026-10-07T13-00-01-phase/obs-history.md |

**Proposal:** The prompt's letter and the tool's tolerance disagree, and the tool's reading has been sufficient each time. A direction: align the prompt to the form the tool resolves (marker plus optional label) and have the cites listing stay silent on the known inner labels, so the step stops recording a zero-cost irregularity; or tighten the distiller prompt if the letter is the intent.

## Cross-step chains (starting heuristics)

28 anchors (an `input.*` friction, or a consumed artifact graded thin or wrong); 22 joined to an earlier producer in the same chunk, 6 not joinable by construction. No transitive chains, no scoring.

### X1 — wrap-session/report →report→ wrap-session/reconcile — 6 chunks
Consumer end: `input.report-insufficient` in sink-target-allowlist (L556), settle-detection (L585), command-and-refusal-schema (L608), act-by-id (L634), refusal-detection (L665) and driver-command-spans (L696), each with `report` consumed as thin; two of those reconciles ended halted-resolved (L664, L695). Producer end: all six report steps closed `ok` (L553, L582, L606, L631, L660, L694), and their own records show the site work done without a tool — signals `reconstructed` (L553) and `citation-map-carried`, `sweep-hits-read-by-offset` (L631); frictions at the producer: untyped L554 (`grep -c` under-read) and L583 (no shipped tool; two scratch scripts), `contract.detector-fact-gap` L632 and L662 (no template bullet for a moved-citation map). **Hypothesis:** the report step builds its site lists and coordinate maps by hand-made searches, closes clean on its own checks, and the shortfall becomes visible only when detectors work from it. **Direction:** the tool named in P2 and L8, at the producer.

### X2 — phase/plan →plan→ implement/code — 5 chunks
Consumer end: `input.plan-step-ambiguous` in driver-session (L527), settle-detection (L578), command-and-refusal-schema (L602), act-by-id (L627) and driver-command-spans (L689); a sixth case (sink-target-allowlist, L549) joins to `phase/validate` as the plan's last producer. Producer end: every plan step closed `ok` with its authoring self-checks clean on the first fire — signals `planlint-0-hits`, `gate-fence-parsed-first-firing`, `fence-parsed-first-time`, `authoring-self-check-clean` (L598, L623, L685), and L571's note 'both authoring self-checks clean on the first fire'; two producers carry `retry.synthesis-rework` (L572, L599), a self-read that had already rewritten steps. **Hypothesis:** the plan's mechanical self-checks certify the gate block, and a clean reading there says nothing about whether a step's stated mechanism holds against its neighbours; the consumer finds that at the first compile. **Direction:** as P6.

### X3 — phase/research →research→ phase/plan — 2 chunks
Consumer end: `input.research-thin` in driver-session (L520) and refusal-detection (L650, extra reads 14, one dialogue round), `research` consumed as thin at both of each chunk's plan runs (L519, L522; L648, L649). Producer end: both research steps closed `ok` carrying `unresolved-questions` (L518, L647) — open plan-decision forks. These are the two chunks whose plan was re-synthesized after the review. **Hypothesis:** research closes before the P4 fork is answered, having read the unchosen branch 'by signature only' (L648's note); when the answer picks that branch, the plan needs the reads research did not make. **Direction:** a short research addendum after the fork answer, for the branch chosen — or forks put before research closes.

### Joined, and not a producer chain
`implement/smoke →conversation→ wrap-session/curation` joins in 2 chunks (L671, L699) only because `conversation` is one artifact name for every window; both consumer notes name the cause as the wrap resuming in a fresh window, which is level candidate L10. The six unjoined anchors have their producer outside the chunk by construction: `working-entry` thin (L497, L514) and `input.carry-context-gap` on route CARRYs (L618, L682) are written by an earlier chunk's route-resolve or by the operator; `ci-verdict` thin (L617, L643) is produced by no step.

## Level candidates (systemic-masked-as-project)

69 workaround / prohibition / removed-cause facts read and clustered by the obstacle each routes around: 9 themes at threshold (L1-L9: 42 facts, one of them counted in two themes), 5 below it (7 facts) and 19 singletons, listed in the appendix; the remaining fact is carried by L13. Then the form signatures: override 3 hits, chronic-degrade 2 (L3, L13), deferred-forever 0 — both `deferred` facts of the epoch (L664 #1, L695 #2) name route-resolve as their destination and were pinned in the same wrap (L676, L703).

### L1 — band-aid — 7 facts
**Theme:** detector returns reach the orchestrator only as conversation text and are pulled from the agents' transcripts by a scratch script.
**Facts:**
- L487 `2026-10-07T04:15:42Z-a` #0 · audit-corrections · wrap-session/reconcile · process / workaround — a proposal-carrying return reaches the orchestrator only as conversation text, so fanout-results.md's parsed lists were extracted from the seven agent transcripts by a scratchpad script rather than retyped
- L555 `2026-10-07T08:48:45Z-a` #0 · sink-target-allowlist · wrap-session/reconcile · process / workaround — offset reads and occurrence counts over the multi-KB master lines were taken with scratchpad readers (sites.py, lines.py) rather than cascade.py window and splice.py index; the detector returns were pulled from the agent transcripts by a scratchpad script and the parsed lists copied into fanout-results.md by script instead of being retyped
- L607 `2026-10-07T12:50:42Z-a` #1 · command-and-refusal-schema · wrap-session/reconcile · process / workaround — the parsed lists of fanout-results.md were assembled by a script from each agent's own final message in its transcript file, never read into context, instead of being transcribed by hand from the messages
- L633 `2026-10-07T14:57:13Z-a` #0 · act-by-id · wrap-session/reconcile · process / workaround — the detector returns arrive as messages and the contract forbids a hand-transcribed list, so the seven returns were pulled from the agents' transcripts by a scratch script (entity-decode, probe, parse) into fanout-results.md
- L664 `2026-10-07T21:03:54Z-a` #0 · refusal-detection · wrap-session/reconcile · process / workaround — the returns were collected from the subagent transcripts by a script, not from the hand-back text in the window, so the record holds the returns and not a retyped copy
- L695 `2026-10-07T23:06:24Z-a` #1 · driver-command-spans · wrap-session/reconcile · process / workaround — the five proposal-carrying returns were extracted from the detectors' task output by script into fanout-results.md rather than transcribed from the hand-back messages; 56 proposals would otherwise be a hand copy
- L533 `2026-10-07T07:18:59Z-a` #2 · driver-session · wrap-session/reconcile · process / workaround — the ten sidecar payloads and the fan-out record were assembled by python from the session rather than through the Write tool; each payload passed sidecar.py check and the splice read-back

Six of the nine reconciles record the script; a seventh (L533 #2) records the fan-out record and the sidecar payloads assembled by python from the session. Typed correlates: P1's 609 (the parsed lists first written with their indentation altered) and 637 item 7 (raw twins hand-written, then rewritten by script); L677 #0 (two detector returns carried host-root paths into the record). Lookback: Epoch 3 L365 #0 records the same script.
**Level hypothesis:** The cause appears to live in the pipeline — the fan-out contract asks for each return's parsed list in fanout-results.md and forbids a hand-transcribed copy, but ships no way to get a sub-agent's return onto disk; the fix so far is a scratch script rewritten in each wrap session.
**Proposal:** A direction: a shipped extractor (or a contract that has each detector write its return to a named file in the run dir), so the record is made by the pipeline rather than by a per-session script.

### L2 — band-aid — 10 facts
**Theme:** point changes applied by a scripted, count-asserted replace instead of the anchored Edit the letter names.
**Facts:**
- L502 `2026-10-07T04:35:57Z-a` #0 · upstream-sync-driver-core · phase/plan · process / workaround — two point changes to the freshly written plan.md (the jq separator made ASCII in the run and in two atoms' text; one a11y anchor extended) were applied by a scripted python replace inside a Bash call, not by anchored Edit calls as the letter prescribes for point changes
- L571 `2026-10-07T10:33:50Z-a` #0 · settle-detection · phase/plan · process / workaround — six point corrections to plan.md were applied through one exact-match python script fed by a heredoc instead of six anchored Edit calls; the phase letter reserves point changes for the Edit tool and bars a shell heredoc for a document payload; each replacement asserted exactly one match
- L607 `2026-10-07T12:50:42Z-a` #0 · command-and-refusal-schema · wrap-session/reconcile · process / workaround — the leaf .claude/docs/services/escher-driver.md was re-derived through a python script of asserted single-match substitutions run from a heredoc, where the skill names the Write and Edit tools for a document; every other body and leaf edit used Edit
- L647 `2026-10-07T15:37:39Z-a` #0 · refusal-detection · phase/research · process / workaround — the premise closure's 13 point changes to scope.md were applied by one scripted replace with a count assertion per anchor, not by anchored Edit calls
- L648 `2026-10-07T16:28:58Z-a` #0 · refusal-detection · phase/plan · process / workaround — the decided forks were written into scope.md and two line citations into research.md and plan.md by scripted replaces with a count assertion per anchor, not by anchored Edit calls
- L533 `2026-10-07T07:18:59Z-a` #1 · driver-session · wrap-session/reconcile · process / workaround — the 76 moved root-manifest citations were rewritten by a scratchpad script that verifies each site against the base file, not by 76 anchored Edits; the listing is in the run dir
- L553 `2026-10-07T08:27:21Z-a` #0 · sink-target-allowlist · wrap-session/report · process / workaround — the 70-row loss listing was spliced into report.md by a scratchpad script from evidence/by-level.md instead of being typed into the Write payload, to keep 70 measured rows free of transcription
- L475 `2026-10-07T03:33:38Z-a` #0 · audit-corrections · implement/code · process / workaround — step 4's block deletions and the unused-import removals in six stand checks were made by a scratchpad python script and sed rather than the Edit tool, so the rustfmt write hook did not fire on them; cargo fmt --all was run after each scripted pass and fmt --check read clean
- L655 `2026-10-07T19:11:24Z-a` #1 · refusal-detection · implement/code · process / workaround — multi-site source edits were made by inline python heredocs rather than scratchpad files run by path (host rule); each landed and was read back by grep, continuation backslashes included
- L688 `2026-10-07T21:59:30Z-a` #0 · driver-command-spans · implement/code · process / workaround — multi-site source edits were applied through quoted python heredocs in the Bash call instead of per-site Edit calls or a scratchpad script file; the backslash-pair collapse host-linux.md records for a quoted heredoc did not occur on these payloads (the escapes landed as written, read back from the file)

Seven facts are document edits (plan.md, scope.md, research.md, a leaf doc, 76 moved citations, a 70-row listing), three are source edits. Every note states the batch size or the assertion per anchor. Lookback: Epoch 1 L82 #1, L103 #0, L122 #0; Epoch 3 L359 #0, L383 #2, L406 #0, L437 #0, L450 #0 — and the side effect is recorded as friction in Epochs 2-3 (a scripted .rs edit bypasses the format hook: L229, L455).
**Level hypothesis:** The letter prescribes one anchored Edit per point change and has no batch form; when a pass holds six, thirteen or seventy-six sites the work leaves the letter every time, in every skill. The cause appears to be the rule's missing arm, not the sessions.
**Proposal:** A direction: a sanctioned batch-edit form in the write rule (an exact-match replace that asserts one hit per anchor and is run from a file), with the format-hook consequence for source stated beside it.

### L3 — band-aid + chronic-degrade — 6 facts
**Theme:** the project's Bash guards and the permission layer refuse a call, which is re-sent in another form.
**Facts:**
- L494 `2026-10-07T04:22:51Z-a` #0 · (no chunk) · new-session/orientation · process / workaround — the Bash cd guard refused a `cd` into the skill's references dir (outside the project root) on a grep of session-state-contract.md; the same grep was re-run by absolute path with no cd
- L499 `2026-10-07T04:31:21Z-a` #0 · upstream-sync-driver-core · phase/distill · process / workaround — the Bash cd guard refused a second `cd`, into the phase run dir, on the entity probe of the seven extracts; the probe was re-run with absolute paths and no cd (the first refusal was at new-session orientation in this window)
- L516 `2026-10-07T05:00:44Z-a` #0 · driver-session · phase/distill · process / workaround — the stage-1 probe command used cd into the run dir; the session's cd guard refused the whole call before anything ran; re-run with absolute paths
- L683 `2026-10-07T21:27:00Z-a` #0 · driver-command-spans · phase/distill · process / workaround — the validation probe was first sent with a leading cd into the run dir and the project's Bash guard refused it; re-sent in a subshell, nothing was written between the snapshots by it
- L523 `2026-10-07T05:31:15Z-a` #0 · driver-session · phase/validate · environment / workaround — the first baseline run, one long command with a shell function and a recursive remove, was declined by the session's permission layer; the baselines ran as a script file in the scratchpad with fixtures written by the Write tool and nothing removed
- L526 `2026-10-07T05:45:20Z-a` #0 · driver-session · implement/code · environment / workaround — a compound Bash hand-probe of the new binary (background host + rm -rf + heredoc) was declined by the permission mode; the probe was rewritten as a scratchpad python script run by path

Four leading-cd refusals and two compound commands declined by the permission layer; the overridden fact L473 #0 is a third declined compound. Typed correlates: `tooling.hook-friction` twice (L550, L579 — a cat heredoc with a file target) and P8's 589. Lookback — the chronic-degrade form: guard and permission refusals are recorded as deviation facts in every epoch (13 / 4 / 4 before this one, by a keyword read of the notes), `tooling.hook-friction` reads 2 / 2 / 0 / 2, the untyped recurrences L156, L185, L211, L238 name the same guard, and no occurrence ever halted.
**Level hypothesis:** The guard works as built and the learning about it is curated, yet the refused form keeps being sent — one retry each, never a halt, so the halt policy never surfaces it. The recurring cost appears to come from pipeline recipes and habits that lead with `cd` into a run dir or the skill's references dir, absorbed one retry at a time in the project.
**Proposal:** Directions: have the skills' own recipes use absolute paths or a subshell wherever they now imply a `cd` (the refusals cluster at phase/distill's extract probe and at orientation's reference read — four of four cd facts); and see P8 for the tier of the learning.

### L4 — band-aid — 6 facts
**Theme:** reads taken ahead of their slot while the P2 distiller batches run.
**Facts:**
- L469 `2026-10-07T03:10:15Z-a` #0 · audit-corrections · phase/research · process / workaround — read-only research reads (the two source files, the stand checks, the audit dirs, tool availability) were taken while the P2 batches ran, because nothing may be written between a batch's snapshots; no query that writes a trace ran before P2 closed
- L518 `2026-10-07T05:06:40Z-a` #0 · driver-session · phase/research · process / workaround — codebase-research.md and the code-graph cookbook were read during P2's wait for the distillers, ahead of P3's before-starting slot; no P3 read of the tree or query ran before P2 closed
- L595 `2026-10-07T11:54:44Z-a` #0 · command-and-refusal-schema · phase/distill · process / workaround — the wait on the two background batches was used for P3's targeted reads (driver sources, harness input, the stub, manifests, the playbook) before this checkpoint; nothing was written between a snapshot pair and no code-graph query fired
- L647 `2026-10-07T15:37:39Z-a` #1 · refusal-detection · phase/research · process / workaround — six source files (execute.rs, refusal.rs, the two stand_act checks, the harness inspect and blitz-dom hit readers) were read while the P2 batches ran, before the extracts existed, so those reads were not extract-directed
- L502 `2026-10-07T04:35:57Z-a` #1 · upstream-sync-driver-core · phase/plan · process / workaround — validation.md was read during P4 authoring, ahead of its Before-starting slot at P5, to decide which gate entries count as new and how check 9 reads the Platform issues slot
- L516 `2026-10-07T05:00:44Z-a` #1 · driver-session · phase/distill · process / workaround — the seven sidecar.py summary calls of stage 2 ran in the same call that moved the failed layouts extract aside, before the layouts re-spawn returned and stage 1 closed; they ran ahead of that pair's before-snapshot

Four facts are P3's reads made during P2's wait; two are a later step's reference or tool call made early. Two `unresolved` facts record the evolve reading order itself breached the same way — a checkpoint's playbook opened before the step it observes had completed (L478 #0, L614 #0). Lookback: no fact in Epochs 1-3 matches these words (ahead of, while the batches ran, before-starting).
**Level hypothesis:** The phase letter orders P3's reads after P2 closes, and P2 is a wait on background agents; the work fills the wait with read-only reads each time and records it as a deviation. The cause appears to be the letter's ordering leaving an idle window, not a project condition.
**Proposal:** A direction: either sanction read-only P3 reads during the P2 wait (the facts each state that nothing was written between a snapshot pair), or state the reason the reads are to be extract-directed (L647 #1 is the one fact that names a consequence: 'those reads were not extract-directed').

### L5 — band-aid — 4 facts
**Theme:** the inputs tool's step set has no value for a word given at P4, P5 or the wrap.
**Facts:**
- L519 `2026-10-07T05:17:31Z-a` #1 · driver-session · phase/plan · process / workaround — the founder's P4 answer was snapshotted as a relay with --step phase:P3, the nearest value: inputs.py's step set is phase:P1, phase:P3 and implement
- L652 `2026-10-07T18:49:37Z-a` #0 · refusal-detection · phase/validate · process / workaround — inputs.py snap takes --step from phase:P1, phase:P3 and implement only; the approving word, given at P5, was first sent with phase:P5 (exit 2) and then snapped as phase:P3, like the review directive before it
- L685 `2026-10-07T21:42:43Z-a` #0 · driver-command-spans · phase/plan · process / workaround — the fork answers were snapshotted with the step value phase:P3 because the inputs tool takes no P4 step; the same form the previous chunk used for its fork answers
- L631 `2026-10-07T14:28:27Z-a` #0 · act-by-id · wrap-session/report · process / workaround — five rulings reached the wrap by relay and inputs.py has no wrap-side snap (verify only), so they were copied whole by hand to the run dir's operator-rulings.md for the report and sidecars to cite

Three chunks snapshotted a P4 fork answer or a P5 approving word as `phase:P3`, 'the nearest value' (L652 #0 first tried phase:P5, exit 2); at the wrap the tool has no snap at all and five rulings were copied by hand (L631 #0). Typed correlate: P12's 521. Lookback: Epoch 3 L354 #0 and L355 (`contract.structural-blind-spot`) record the same refusal.
**Level hypothesis:** The tool's enumeration is narrower than the flow the skill letter describes, so every operator word after P3 is filed under a step it was not given at; the record of who said what, when, is being bent in the project to fit the tool.
**Proposal:** A direction: widen the step set to the steps where the letter records an operator word (P4, P5, wrap), or give the snapshot an explicit `given-at` field.

### L6 — band-aid — 3 facts
**Theme:** pipeline tool calls read through a filter or a clipped view, against the run-bare rule.
**Facts:**
- L514 `2026-10-07T04:52:40Z-a` #1 · driver-session · phase/take-up · process / workaround — the P1 route.py pins call was piped through grep -v to drop the other entries' CARRY rows, against the run-bare rule; the unfiltered listing of the same file had been read 4 minutes earlier and the trail in the run dir holds the whole
- L523 `2026-10-07T05:31:15Z-a` #1 · driver-session · phase/validate · process / workaround — one re-run of gate.py --dry-run was piped through grep -v, against the run-bare rule; the same plan had been read bare before it and was read bare twice after
- L641 `2026-10-07T15:06:33Z-a` #0 · act-by-id · wrap-session/gates · process / workaround — the light gate, the flip, hygiene and scope calls were read through a width-clipping or tail pipe to hold a context window at 85 percent, against the letter's run-bare rule; each verdict was read from its printed summary line and the full listings are in the run dir's trails

Each note gives the reason: to drop other entries' rows (L514 #1), or to hold a context window at 85 percent (L641 #0). Lookback: Epoch 2 L215 #2, L287 #0, L236 #1; Epoch 3 L381 #0.
**Level hypothesis:** The tools print whole listings and the rule forbids filtering them; where the listing is larger than the read needs, the filter is applied anyway. The cause appears to be the tools' output size against a growing route and a finite window.
**Proposal:** A direction: a tool-side narrowing (one entry's pins, a summary-only mode) so a short read does not need a shell filter the rule forbids.

### L7 — band-aid — 3 facts
**Theme:** the CI verdict row carries too little, and the read-once rule is routed around.
**Facts:**
- L501 `2026-10-07T04:33:22Z-a` #0 · upstream-sync-driver-core · phase/research · process / workaround — ci.py conclusion prints no check names on a green row, and two extracts require the audit and a11y jobs read as their own jobs; the 16 check names were read once through gh api .../commits/{sha}/check-runs with the fork named in the path, beside the tool's row
- L545 `2026-10-07T07:46:01Z-a` #0 · sink-target-allowlist · phase/research · process / workaround — Setup 5a reads the CI verdict once and it read in progress; the run was read a second time at the P3 closure so the scope bullet could close on the run's own verdict (green 16/16) instead of standing unresolved into the plan
- L643 `2026-10-07T15:23:37Z-a` #0 · refusal-detection · phase/take-up · environment / workaround — ci.py's red row named no failed check, so the run's jobs and annotations were read through gh by run id, and ci.py was read a second time for HEAD before the ask (the letter says once)

A green row prints no check names (L501 #0), a red row named no failed check (L643 #0), and a run read in progress at Setup was read again at P3 (L545 #0). Correlates: `ci-verdict` consumed thin twice (L617, L643), the one halted take-up of the epoch (L643), P4's 644 (a cause inferred from the workflow file) and the untyped L645 (`gh api` given a flag it does not take). Lookback: no fact in Epochs 1-3 names the verdict tool.
**Level hypothesis:** The verdict tool's row is the letter's single read of CI, and when the row is thin (in progress, red without a failed job, green without names) the work goes to `gh` directly and re-reads; the cause appears to be the row's content and the read-once rule having no in-progress arm.
**Proposal:** A direction: the verdict row names its checks (and the failed or absent ones on a red), and the letter's single read gains a stated re-read for an in-progress verdict.

### L8 — band-aid — 2 facts
**Theme:** scratch readers written each wrap to count and locate sites inside the masters' multi-KB single-line bullets.
**Facts:**
- L553 `2026-10-07T08:27:21Z-a` #1 · sink-target-allowlist · wrap-session/report · process / workaround — site counts for the report were taken with a scratchpad occurrence-level reader (sites.py) rather than grep -c, because the masters' bullets are single multi-KB lines and a line count under-reads occurrences
- L555 `2026-10-07T08:48:45Z-a` #0 · sink-target-allowlist · wrap-session/reconcile · process / workaround — offset reads and occurrence counts over the multi-KB master lines were taken with scratchpad readers (sites.py, lines.py) rather than cascade.py window and splice.py index; the detector returns were pulled from the agent transcripts by a scratchpad script and the parsed lists copied into fanout-results.md by script instead of being retyped

L555 #0 is also counted in L1 (its note has two halves). The same obstacle is recorded untyped at the report step (L554: `grep -c` read 2 for 3 and 6 for 8; L583: 'no shipped tool serves either at P1'), typed as `contract.detector-fact-gap` (L632, L662: no template bullet and no tool for a moved-citation map) and in P5's 697 (a 10 501-char line read at its offset). Lookback: Epoch 1 L170 (a report site counted from `grep -c`), Epoch 3 L376.
**Level hypothesis:** The masters' format — a bullet is one line, and bullets run to several KB — is a pipeline convention; line-oriented tools under-read it, the shipped sweep runs only after the apply, and the report step's site work is done by scripts rewritten per session. The fixes land in the project's scratchpad; the cause appears to sit in the format and the tool set.
**Proposal:** A direction: ship the occurrence-level site counter and the line mapper the sessions keep re-writing (the same tool as P2's proposal), usable at the report step. See the extension candidate U1.

### L9 — removed-cause (observation) — 2 facts
**Theme:** a time stamp typed from an estimate, refused by the stamp hook and rewritten from the clock.
**Facts:**
- L649 `2026-10-07T16:46:22Z-a` #1 · refusal-detection · phase/plan · process / removed-cause — three baseline strings were written with an estimated time before their baselines had run; the stamp-ahead hook refused the edits; the baselines were then run and the stamps written from the clock
- L699 `2026-10-07T23:08:04Z-a` #0 · driver-command-spans · wrap-session/curation · process / prohibition — a frontmatter time stamp in the operator's auto-memory file was typed from an estimate two minutes ahead of the clock; the host's stamp hook refused it and the harness's own stamp stood

The same event is the untyped L651, and P1's 535 records an estimated sweep time caught on re-read where no hook stood. Lookback: Epoch 2 L213 #0 and four untyped records (L197, L268, L274, L296); Epoch 3 L364 and L422.
**Level hypothesis:** The hook removes each instance and the act returns — in three of four epochs and at five different steps (plan, validate, report, reconcile, curation). The guard is doing its work; the recurring cause is authoring a stamp before reading the clock.
**Proposal:** An observation rather than a proposal: the hook is the standing fix. A direction if one is wanted: the templates that carry a Date or baseline stamp say 'paste from `date -u`' at the field.

### L10 — override — 2 facts
**Theme:** the operator bounds the wrap to P1 and runs the rest in a fresh window.
**Facts:**
- L660 `2026-10-07T20:34:14Z-a` #0 · refusal-detection · wrap-session/report · process / overridden — the operator bounded this wrap to P1 for window size (43.8 percent at invocation, the last wrap grew its window by 46 points); P2 to P7 run in a fresh window from report.md, so the report's Decisions and corrections section was written as the only carrier of this window's corrections
- L694 `2026-10-07T22:39:30Z-a` #0 · driver-command-spans · wrap-session/report · process / overridden — the operator directed this wrap to run P1 only and stop, the remaining phases to run in a fresh window from report.md; Setup's background code-graph refresh was still fired here and finished, though this window reaches no P4

L660 #0 gives the reason as window size (43.8 percent at invocation, 'the last wrap grew its window by 46 points'); L694 #0 records the same direction at the next chunk without restating a reason. Downstream in the same chunks: curation consumed `conversation` as thin twice (L671, L699) and ran `ok-degraded` once (L671 #0, candidates taken from the report alone); L641 #0 clipped tool output to hold a window at 85 percent; `ambiguity.review-cycles` L654 records a review prompt issued across three context windows.
**Level hypothesis:** The wrap is written as one window's work and, on this project's last two chunks, no longer fits one; the operator re-cuts it by hand each time, and curation then runs without the conversation it is designed to read.
**Proposal:** A direction: a sanctioned two-window wrap — the report written as the carrier, a stated resume point, and curation's inputs defined for a resumed window — so the split stops being an override and its cost to curation is designed for.

### L11 — override — 2 facts
**Theme:** an escalation-graded proposal is resolved on a word recorded beforehand, without the halt.
**Facts:**
- L533 `2026-10-07T07:18:59Z-a` #0 · driver-session · wrap-session/reconcile · process / overridden — three escalations (the socket as a boundary widening; ids and names at debug and trace under two escalate-severity detectors; the unmet quiet criterion) took no HALT: each was resolved on a word recorded before or at the wrap (inputs#I1, the wrap argument's item 5) and recorded as escalated-and-resolved
- L695 `2026-10-07T23:06:24Z-a` #0 · driver-command-spans · wrap-session/reconcile · process / overridden — the boundary-widening escalation for the sink's closed-span line was resolved by the operator's direction at the wrap's invocation (keep PROVISIONAL), so no halt was raised for it; one halt was raised for a second founder-ratified clause the direction did not name

Correlates: the untyped L559 (six proposals arrived at `escalate` because the detector copies its drift-base severity onto every proposal it files; applied as routine) and `ambiguity.playbook-no-match` L667. The two halts that did fire at reconcile (L664, L695) each cost one dialogue round. Lookback: Epoch 1 L99 records the same detector, the same severity and the same applied-without-halt.
**Level hypothesis:** A detector's severity is a property of the detector, not of the proposal, so 'escalate = halt and ask' fires on amendments that narrow or merely restate; the operator pre-empts it with a word at the wrap's invocation. The rule's calibration, not the project, appears to be what is being corrected.
**Proposal:** A direction: severity judged per proposal (or a drift-base rule that an amendment already named in the P5-approved expected amendments is routine), so the halt is kept for the cases that need the founder.

### L12 — override — 3 facts
**Theme:** a lean P4 took on its own is overridden at the P5 review.
**Facts:**
- L573 `2026-10-07T10:39:07Z-a` #0 · settle-detection · phase/validate · product-logic / overridden — the plan proposed claiming v010-10 on an in-process Session::act step; the operator review overrode it: no claim, test 1 kept as a tests criterion, a ledger note written, the driver leg carried for route-resolve; the review prompt was issued as text both times
- L598 `2026-10-07T12:06:38Z-a` #0 · command-and-refusal-schema · phase/plan · product-logic / overridden — the recommended time option was offered with a bare step count; the operator took the option and directed the unit to be weighed against milliseconds so the schema itself states it, and added a refusal cause for an app with no time seam — the plan takes milliseconds, an advanced_ms result field and an eighth cause
- L649 `2026-10-07T16:46:22Z-a` #0 · refusal-detection · phase/plan · product-logic / overridden — the operator rejected the first plan's lean at the P5 review — scroll moved the viewport only and the nested case was a stated limit with no owner — and directed the nested scroll, the stand proof and a result that says when scroll could not; P4 was re-run on it

Three chunks, three different product decisions (a capability claim, a unit of time, a scroll's reach). Typed correlates: P3's 524 and 653 (review-caught defects, each a return to P4), `contract.matrix-claim` L574. Lookback: Epoch 3 L410 and L441 are the same capability-claim override.
**Level hypothesis:** The review is working as designed; what recurs is that the plan arrives with the lean already taken as decisive and the whole plan built on it, so a rejected lean costs a re-synthesis — twice in this epoch (driver-session and refusal-detection, the repeated `phase/plan` runs).
**Proposal:** A direction: a lean that claims a capability or widens a boundary goes to the operator as a P4 question before synthesis rather than as a caveat on the review card (L574: the card listed three caveats and still recommended the claim).

### L13 — chronic-degrade — 1 fact
**Theme:** `tooling.host-shell` recurs once per epoch and never halts.
**Facts:**
- L660 `2026-10-07T20:34:14Z-a` #1 · refusal-detection · wrap-session/report · environment / workaround — a CI job-log parser sent as an inline heredoc lost its backslash pair to the Bash transport and read every target as not found; it was moved to a script file written by the Write tool

The in-epoch friction is L663 (two job-log parses read 'not found': a regex backslash pair collapsed in an inline heredoc, then colour codes as literal text), with P8's 674. Lookback: Epoch 2 L194 and Epoch 3 L382 are a different mechanism under the same type (the shell's grep refusing a bounded repetition and printing nothing). Two in-epoch facts read against the host rule: L655 #1 and L688 #0 record backslash-bearing heredoc payloads that landed as written.
**Level hypothesis:** Three epochs, one record each, two mechanisms; each read as an empty or negative result before it was recognised as the shell. The host rule on the backslash collapse has one supporting and two contrary readings inside this epoch.
**Proposal:** A direction for the founder's judgment only: the host rule's collapse claim wants a controlled measurement before it is either kept or dropped, since the ledger now holds both readings.

## Playbook-extension candidates (untyped patterns, F-4)

14 untyped records, read and clustered: 1 cluster at the F-4 threshold; the rest are in the appendix. The two untyped groups that reach n = 3 by step (`implement/code`, `wrap-session/reconcile`) each split into unrelated events and are not candidates as groups.

### U1 — wrap-session/report (and one take-up) — 2 cases in-epoch, recurring from Epoch 3 → proposed type `tooling.long-line-misread`
**Cluster:**
- L554 `2026-10-07T08:27:21Z-b` · sink-target-allowlist · wrap-session/report · retries 1 — the first site sweep for the expected amendments used grep -c per master: it counts lines, the masters hold multi-KB single-line bullets, so it read 2 where there were 3 citations and 6 where there were 8; the call also exited 1 on a filter with no match; the sweep was re-run occurrence-level before any count was written
- L583 `2026-10-07T11:08:36Z-b` · settle-detection · wrap-session/report · — — report-template asks for occurrence-level site counts with a disposition per hit and an old-to-new coordinate map for every moved citation; no shipped tool serves either at P1 (cascade.py sweep runs after the apply, from a patterns file), so two scratch scripts were written - a site counter and a line mapper - as the previous wrap's report also cites a hand-made sites.py
- previous epoch: L376 `2026-10-06T21:34:26Z-b` · phase/take-up — Re-verifying the CARRY's two architecture coordinates with a line-matching search printed two whole single-line entries of architecture.md (lines 91 and 134, the second several KB) into the window; the coordinates were confirmed from that one read, no redo

The membership is a judgment: L554 is a line count under-reading occurrences, L583 the missing occurrence-level tool, L376 a line-matching search flooding the window — one obstacle (a master bullet is a single multi-KB line) met by three line-oriented reads. Typed neighbours that did not cover it: `contract.detector-fact-gap` (L632, L662) names the template gap, not the misread.
→ **draft criteria line:** `tooling.long-line-misread` — a line-oriented read (a line count, a line-matching search, a line-numbered window) over a document whose entries are single multi-KB lines under-counted occurrences, missed a site or flooded the window; record the document, the tool form and the occurrence-level re-read that replaced it. The cluster crosses steps, so the Universal list is as plausible an owner as the report playbook.

## Below threshold — no action

**Typed groups (n < 3, no halt or soft-exit impact)** — count per epoch 1 / 2 / 3 / 4 in brackets:
- phase/plan/input.research-thin — n 2 · weight 4 · L520, L650 [0 / 0 / 0 / 2]
- implement/code/tooling.hook-friction — n 2 · weight 3 · L550, L579 [2 / 2 / 0 / 2]
- implement/fix-loop/contract.test-expectation — n 2 · weight 3 · L477, L692 [0 / 0 / 0 / 2]
- phase/plan/retry.synthesis-rework — n 2 · weight 3 · L572, L599 [2 / 0 / 1 / 2]
- phase/validate/ambiguity.review-cycles — n 1 · weight 5 · L654 [0 / 0 / 1 / 1]
- phase/research/contract.narrow-basis-claim — n 2 · weight 2 · L570, L622 [4 / 0 / 5 / 5] — counted in P4 by type
- wrap-session/report/contract.detector-fact-gap — n 2 · weight 2 · L632, L662 [0 / 0 / 0 / 2]
- phase/take-up/contract.narrow-basis-claim — n 1 · weight 3 · L644 [4 / 0 / 5 / 5] — counted in P4 by type
- phase/validate/contract.matrix-claim — n 1 · weight 3 · L574 [0 / 0 / 2 / 1]
- wrap-session/report/tooling.host-shell — n 1 · weight 3 · L663 [0 / 1 / 1 / 1]
- phase/take-up/input.carry-context-gap — n 2 · weight 1 · L618, L682 [0 / 0 / 0 / 2]
- phase/take-up/input.working-entry-thin — n 2 · weight 1 · L498, L515 [2 / 1 / 0 / 2]
- implement/smoke/contract.vacuous-check-found — n 1 · weight 2 · L630 [1 / 0 / 0 / 1]
- phase/research/contract.instrument-validity — n 1 · weight 2 · L470 [3 / 1 / 0 / 1]
- implement/fix-loop/contract.spec-reality-gap — n 1 · weight 1 · L530 [1 / 2 / 0 / 1]
- new-session/orientation/contract.jointly-contradictory-instructions — n 1 · weight 1 · L496 [0 / 0 / 0 / 1]
- phase/plan/contract.skill-reference-drift — n 1 · weight 1 · L521 [0 / 1 / 2 / 4] — counted in P12 by type
- wrap-session/gates/contract.narrow-basis-claim — n 1 · weight 1 · L493 [4 / 0 / 5 / 5] — counted in P4 by type
- wrap-session/reconcile/ambiguity.playbook-no-match — n 1 · weight 1 · L667 [1 / 1 / 0 / 1]
- wrap-session/report/input.implement-outcome-unsettled — n 1 · weight 1 · L661 [0 / 1 / 2 / 1]
- wrap-session/report/recall.change-reconstruction — n 1 · weight 1 · L486 [0 / 1 / 0 / 1]
- wrap-session/route-resolve/contract.narrow-basis-claim — n 1 · weight 1 · L511 [4 / 0 / 5 / 5] — counted in P4 by type
- wrap-session/route-resolve/contract.no-sanctioned-channel — n 1 · weight 1 · L613 [0 / 0 / 2 / 1]

Two of these sit at a step that halted — `contract.narrow-basis-claim` L644 at take-up L643 (with the untyped L645) and `ambiguity.playbook-no-match` L667 at reconcile L664; the halt is on the step record, not in the friction's own impact, so the n >= 2 arm does not apply.

**Untyped clusters below F-4:**
- a recorded learning recurred at the acting step (per-crate clippy) — L603, L690 — n = 2 in-epoch; no untyped member in the previous epoch (Epochs 1-2 hold four untyped recurrences of the Bash-guard learning: L156, L185, L211, L238)
- a time stamp estimated ahead of the clock, refused by the stamp hook — L651 — n = 1 in-epoch (Epoch 2 holds four); see level candidate L9
- tracked evolve trail outside the session-state contract's list — L495 — n = 1 untyped; the same event is typed contract.skill-reference-drift three times later in the epoch (P12)
- detectors read the tree against the prompt's ban — L534 — n = 1 untyped; era — the reconcile record's rejected-for-source count deploys 2026-10-08 (see P10)
- sidecar entry cap of 3000 B against a many-edit entry — L587 — n = 1 untyped; the same cap is hit twice more inside contract.in-pass-correction (P1: 637, 670)
- a detector copies its drift-base severity onto every proposal — L559 — n = 1; zero-cost record; see level candidate L11 (Epoch 1 L99 records the same detector and severity)
- route-resolve has no delegated-disposition form — L464 — n = 1 untyped; see P14
- a duplication detector's result depends on its scan roots — L471 — n = 1
- a Tier-1 learning's wording gives `gh api` a flag it does not take — L645 — n = 1 (the learning is CLAUDE.md:136 by the record's own citation)
- cargo's stdout and stderr read as two streams; a nine-mutation run repeated — L657 — n = 1
- `gate.py hygiene` read clean over a run dir holding two host-root paths — L678 — n = 1 (deviation fact L677 #0 records the grep that found them); cause not established by the record

**Deviation themes below threshold:**
- the friction ledger read against the never-read rule (to copy the envelope's version form; to classify a dirty tree) — L564 #0 — 1 workaround fact + the `unresolved` L513 #0 + `contract.jointly-contradictory-instructions` L496; Epoch 3 holds L402 #0 and L442 #2 — see Mechanism health (version spellings)
- a temporary print added to a check that prints nothing by rule, then restored by checksum — L688 #1, L691 #0 — 2 facts, one chunk, no prior epoch
- the per-crate clippy form run instead of the documented lint leg — L526 #1, L601 #0 — 2 facts, first seen this epoch — carried by P8
- a gate entry that cannot read green on a correct tree, run by hand — L476 #0 — 1 fact (`ok-degraded` fix-loop; `contract.test-expectation` L477); Epoch 1 L89 has the same form from a different cause
- a wrap resumed in a fresh window takes its curation candidates from the report — L671 #0 — 1 fact — carried by L10

**Single deviation facts (workaround / removed-cause), one line each:**
- L467 `2026-10-07T03:02:42Z-a` #0 · audit-corrections · phase/distill · process / workaround — the three check-3 retries were delivered by resuming each distiller with SendMessage instead of a fresh spawn; the file was moved aside to its raw twin first and each retry ran inside its own snapshot pair, as the re-spawn rule states
- L480 `2026-10-07T03:42:24Z-a` #0 · audit-corrections · phase/plan · process / workaround — P4's read of the 7 extracts and their history files was not made in this run: the revision changes one entry's addresses and reaches no master section, so the approved plan's synthesis was carried and the plan's Provenance says the take-up's extracts were not re-read
- L503 `2026-10-07T04:37:22Z-a` #0 · upstream-sync-driver-core · phase/validate · process / removed-cause — the control for the accessibility-files entry was minted in the run dir as a scratch copy of raw cargo Running lines; it was deleted after its reading, since raw gate output is not committable evidence, and only its verdict is recorded in the entry's baseline
- L504 `2026-10-07T04:39:53Z-a` #0 · upstream-sync-driver-core · implement/code · process / workaround — the plan orders its only write (step 3, evidence/sync.md) after the gate block (steps 1-2), while the skill body orders P1 write then P2 gates; the plan's order was followed, so this code checkpoint ran after the gate block instead of before it
- L510 `2026-10-07T04:46:13Z-a` #0 · upstream-sync-driver-core · wrap-session/route-resolve · process / workaround — the measurement behind the CARRY arrived at P5, after the fan-out had read the report; one dated line was appended to the report's Decisions and corrections at P5 so the chunk's record carries it, after a 0-hit search showed no master states the corrected claim
- L514 `2026-10-07T04:52:40Z-a` #0 · driver-session · phase/take-up · process / workaround — Setup's read of evolve-system.md was replaced by a cmp against the byte-synced new-session copy read earlier in this window; cmp read identical
- L519 `2026-10-07T05:17:31Z-a` #0 · driver-session · phase/plan · environment / workaround — the socket control's first run failed in the session scratchpad: its path exceeds the AF_UNIX bound; re-run in a short directory under target/tmp, removed afterwards
- L523 `2026-10-07T05:31:15Z-a` #2 · driver-session · phase/validate · process / workaround — after the re-synthesis the seven entries whose run text did not change kept their first-pass baselines; only the six added or changed entries were baseline-run again
- L523 `2026-10-07T05:31:15Z-a` #3 · driver-session · phase/validate · process / workaround — the approval word arrived with a direction to reword the review's authority line; the edit was made after the word and the whole mechanical set re-run before the ledger call, not before the word
- L546 `2026-10-07T07:52:54Z-a` #0 · sink-target-allowlist · phase/plan · process / workaround — the by-level probe research ran from the session scratchpad was rewritten as chunks/{marker}/evidence/by-level.py so implement can take the after-readings with the same instrument; the phase letter names scope.md, research.md and plan.md as its chunk writes, not an evidence file; the script windowed mode is untested and the plan says so
- L548 `2026-10-07T08:06:55Z-a` #0 · sink-target-allowlist · implement/code · process / workaround — the five outside_target unit tests are step 6, after the fix of step 5, so they could not be seen red in plan order; their control was taken by temporarily making is_outside_target return false (5 of 5 failed), then restoring format.rs byte-identical (cmp) and re-running green
- L626 `2026-10-07T13:54:47Z-a` #0 · act-by-id · implement/code · product-logic / removed-cause — plan mutation 4 (advanced_ms returned as asked) does not isolate the .min(ms) clamp and no listed check drove a step that over-reports; added stand_act_timer's an_advance_reports_no_more_than_it_was_asked and a fifth mutation control
- L629 `2026-10-07T14:00:27Z-a` #0 · act-by-id · implement/smoke · process / workaround — no listed smoke boots escher-session on the timer task (host_binary boots counter, host_log boots crud), so the edited branch had no real boot; drove one by hand with a scratch script speaking the socket's hello and stop lines
- L639 `2026-10-07T14:58:48Z-a` #0 · act-by-id · wrap-session/route-resolve · process / workaround — the ruling that mints the entry gave no placement and said to place it at route-resolve; the entry was placed by dependency, ahead of the first work that needs it, without a halt, and the placement and its reason are written in the entry's own carry
- L652 `2026-10-07T18:49:37Z-a` #1 · refusal-detection · phase/validate · process / workaround — the approving word carried an instruction for the chunk's report (flag the engine fix as upstreamable); plan.md was not edited after the word, so the instruction is held verbatim in inputs#I4 and stated in this run's report
- L655 `2026-10-07T19:11:24Z-a` #0 · refusal-detection · implement/code · product-logic / workaround — a scrolled box's own snapshot bounds read shifted by its own scroll offset (get_client_bounding_rect, a file the chunk may not edit) - the stand check reads the list's box from bounds taken before the scroll and from the engine reader, instead of the plan's bounds-after wording; recorded in evidence/stand-census.md
- L655 `2026-10-07T19:11:24Z-a` #2 · refusal-detection · implement/code · process / workaround — a splice command for execute.rs was fired before its input file was written and failed before any write; the file was then written whole with the Write tool
- L677 `2026-10-07T21:13:55Z-a` #0 · refusal-detection · wrap-session/gates · process / workaround — the hygiene read printed clean over a run dir whose fanout-results.md held two lines spelling the host's root path (two detector returns' basis fields); a grep run beside the tool found them and the record was reassembled with the repository root prefix dropped
- L704 `2026-10-07T23:14:27Z-a` #0 · driver-command-spans · wrap-session/gates · process / workaround — health.py ran after the commit for the console's size line and left its trail untracked in the committed run dir; left for the next wrap's sweep with the trailing friction append

**Form signatures with nothing to report:** deferred-forever — 0 (2 deferred facts, both routed and closed in the same wrap). `ok-degraded` — 2 in-epoch (L476, L671), against 2 / 3 / 0 in Epochs 1-3; the fix-loop one shares its form with Epoch 1 L89 from a different cause, and the curation one is carried by L10.
