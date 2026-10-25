# Fan-out results — 2026-10-25-capability-statement (wrap, resumed at P2)

Run: a forced sample (inputs#I7, the operator's word «forced sample»; `cascade-sample.txt`: `sample: yes · wrap · draw 8 of 10 · FORCED by the operator`).
Batches, each one Workflow, each `fanout.py check` verdict as printed:
- stage1 — `VERDICT: 62 of 64 pass` (doc 7/7 · task 6/6 · catdet 49/51, failed OWN_FILE 2: fanout-security-plan--error-handling, fanout-security-plan--input-validation); stage1-r1 (the two re-spawned, OWN_FILE) — `VERDICT: 2 of 2 pass`; their first files kept as `.raw-fanout-security-plan--{error-handling,input-validation}.md`
- classify — `VERDICT: 73 of 73 pass` · yes 21 of 971
- strong — `VERDICT: 8 of 8 pass`
- newdetail — `VERDICT: 1 of 1 pass` (test-plan · test-strategy · F4)
Every batch `[PATH_CHECK] PASS · index and head unchanged`. No fix by script: no twin but the two re-spawn move-asides.

## Verdicts
- architecture: 2 proposals · fanout-architecture.md · 6 cascade proposals · fanout-architecture--cascade-g1.md · fanout-architecture--cascade-g2.md · fanout-architecture--cascade-g3.md · 3 facts · 2 placed · 0 no home · (the doc agent's 1) · (sampler) 0 lost
  - note: D-arch-resources and D-arch-decisions find nothing unregistered; the cascade's category sites left to it
  1. §Infrastructure Patterns → Directory structure, a `docs/` row — apply · check 1: the plan's Expected amendment (docsdir) names the change; the applied text also carries the entry's «an upstream change under `docs/` never conflicts with it» (the proposal's change left it out); body section (no category file), check 7 n/a
  2. §Project Intent → Front page (body `architecture.md:292`), README links `docs/capabilities.md`; «no build, doc or test step reads it» false — the pin's fourth case reads README.md (report Symbols / APIs) — apply · check 1: the plan's Front page entry + Accurate this-chunk addition
  - cascade-g1 1. standard-contracts.md:27 AT owner `re-carried:0.2.0` — apply · check 1: the plan's atowner entry (arch §Standard Contracts → Mutation, query and CSSOM); the report lists the site
  - cascade-g1 2. standard-contracts.md:149 «the nine causes» → eight — apply · check 6: the report's causes9 disproved claim, substantiated (`CAUSES: [Cause; 8]`)
  - cascade-g2 1. standard-contracts.md:261 seven → eight test files — apply · check 1: Accurate this-chunk addition (report ciscripts, the site listed)
  - cascade-g2 2. standard-contracts.md CI contracts, the capability-statement pin's contract (F3) — apply · check 7: read whole (check7-architecture-standard-contracts, lines 1–264 to END): the CI contracts entity holds one contract per CI script (agent-run, cold-agent, blitz-tests targets, coverage floor, apt-install) and none for this one — distinct
  - cascade-g3 1. project-intent.md:21 readme12 (eleven → twelve, the link) — apply · check 1: the plan's Front page entry; merged with item 3 on the same line
  - cascade-g3 2. project-intent.md, a Capability statement entity (F1) — apply · check 7: read whole (check7-architecture-project-intent, END): no entity names the statement — distinct
  3. (raised) §Project Intent detail, `project-intent.md:21` «No build, doc or test step reads it» — the same false clause as item 2 standing in the detail — apply · cascade step 2 (a fact standing elsewhere is stated once in this pass); rides cascade-g3 1
- security-plan: 0 proposals · fanout-security-plan.md · 7 cascade proposals · fanout-security-plan--cascade.md · 2 facts · 2 placed · 0 no home · (sampler) 1 lost
  - note: no detector fires; flags the body's «local stdio» vector (no diffs, no lint findings) and «local IPC» vector (no lint findings)
  - cascade 1. threat-model-summary.md:34 boundary-mcp-stdio carried list + an acting call's diff and a lint's findings — apply · check 1: the plan's first Expected amendment names the change itself («names a lint's findings and the diffs … as §Input Validation and arch §Established Decisions → Driver session already do») — a reconciliation of crossings ratified 2026-10-10 and at 2026-10-19-visual-lint, no new class crossing
  - cascade 2. security-plan.md:27 body «local stdio» vector, the same carried list — apply · dependent-of cascade 1 (one claim, its broad line)
  - cascade 3. threat-model-summary.md boundary-mcp-stdio, docs/capabilities.md states this crossing (F1) — apply · check 1: Accurate this-chunk addition (no carrier added, guard clean); check 7: read whole (check7-security-plan-threat-model-summary, END) — distinct
  - cascade 4. input-validation.md:217 AT owner `re-carried:0.2.0` — apply · check 1: the plan's atowner entry (security §Input Validation); the report lists the site
  - cascade 5. input-validation.md:194 AT owner `re-carried:0.2.0` — apply · check 4: read at offset 1172 (`that witness owned by a .andromeda/residuals.md line (target: next)`, the adapter witness) — the report's atowner fact at a site its rows do not list
  - cascade 6. secret-management.md, the statement and pin hold no credential (F2) — apply, re-targeted · check 7: read whole (check7-security-plan-secret-management, END): the file's searches stand in its `absence · secrets` group, so the fact lands there as a `searched:` row, not in the Development row the proposal named
  - cascade 7. error-handling.md:9 «the cause set (nine)» → eight — apply · check 1: the plan's Error Handling entry
  8. (sampler) threat-model-summary.md:26 boundary-session-socket answer list + a lint's findings — escalate · check 1: no playbook rule governs, no direction names it; its subject is what crosses a hardened boundary (the widening class judged by subject) — the visual-lint crossing «onto every ratified carrier, a ratified one» stands at `input-validation.md:54`, so the recommended resolution is apply as a reconciliation
  9. (raised) security-plan.md:25 body «local IPC» vector — its carried list names no lint finding — escalate with item 8 (one claim, its broad line)
  - 8 and 9 resolved at the halt: apply as a reconciliation — the founder's own word «Применить как сверку (Рекомендую)», by dialog in the overseer session (inputs#I8, `relay-5.md`)
  11. (raised at cascade step 2) eight further carried-set sites the sweep's `carriedset` pattern found stating the socket, MCP, command-line or at-rest list without a lint's findings — arch `established-decisions.md:172`, `security-plan.md:65`, `data-protection.md:8`, `input-validation.md:38` · `:65` · `:70`, obs `ci-integration.md:16`, the test-plan key file `session-lifecycle.md:7` — apply · the same reconciliation as 8 and 9; a fact the sweep finds standing elsewhere is fixed in this pass (`replace-1.md`); confirmed after the pass by the founder's own word «Да, и эти 8 (Рекомендую)», by dialog in the overseer session, relayed by the operator (inputs#I9, `relay-6.md`) — his reconciliation covers all ten carried-set sites
  10. (raised) error-handling.md:8 «the rest, seven since 2026-10-12-headless-screenshot, by the executor» — the row's own closed eight less `validate`'s two is six, as arch `standard-contracts.md:148` states («the executor returns the other six») — apply · check 6 (sibling of causes9 on the same entity), routine
- design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals (yes 0, no strong reader) · 0 facts · (sampler) 0 lost
  - note: D-design-tokens holds — both new surfaces `tokens n/a`
- layout-templates: 0 proposals · fanout-layout-templates.md (whole master, no cascade)
  - note: D-layout-surface unfired — no UI surface or region added
- test-plan: 1 proposal · fanout-test-plan.md · 10 cascade proposals · fanout-test-plan--cascade-g1.md · fanout-test-plan--cascade-g2.md · fanout-test-plan--cascade-new-test-strategy.md · 4 facts · 4 placed (by pass 1) · 0 no home · (sampler) 2 lost
  - note: flags test-scope-summary.md:98's AT owner for the cascade; the ciscripts rows category-only
  1. test-plan.md:48 AT owner `re-carried:0.2.0` — apply · check 1: the plan's atowner entry (test-plan §1); one edit with cascade-g1 4
  - cascade-g1 1. test-scope-summary.md:8 100 → 104 tests, seven → eight files — apply · check 1: Accurate this-chunk addition (ciscripts, site listed); one edit with cascade-g1 2
  - cascade-g1 2. test-scope-summary.md:8 the pin among the CI Python scripts (F3) — apply · check 7: read whole (check7-test-plan-test-scope-summary, END): the row names each CI-script contract test and none for this one — distinct
  - cascade-g1 3. test-plan.md:11 §1 body, the pin among what the leg tests — apply · check 1: the plan's test-plan §4 entry's fact (the pin joins the CI scripts), Accurate this-chunk addition
  - cascade-g1 4. test-plan.md:48 — the same edit as item 1 (cross-contradiction check 2: same direction, one apply)
  - cascade-g1 5. test-scope-summary.md:98 AT owner `re-carried:0.2.0` — apply · check 4: read (line 98, «a gap owned since 2026-10-14-stand-a11y-assertions by a `.andromeda/residuals.md` line (target: next)»), the report's atowner fact at a site its rows do not list
  - cascade-g2 1. unit-test-strategy.md:29 100 over seven → 104 over eight, the per-file list +4 (sum 104) — apply · check 1: Accurate this-chunk addition (ciscripts @1002/@1017 changed, @1303 kept)
  - cascade-g2 2. unit-test-strategy.md, a Capability statement pin entity (F1) — apply, merged with cascade-g2 3 and (sampler) unit-test-strategy#5.1 into one group · check 7: read whole (check7-test-plan-unit-test-strategy, END): one block per CI-script test file (blitz-tests targets, cold-agent pipe), none for this one — distinct
  - cascade-g2 3. unit-test-strategy.md, the pin's reading discipline (F2) — merged into cascade-g2 2
  - cascade-g2 4. test-plan.md §4 body, a Capability statement kind — apply · check 1: the plan's test-plan §4 entry names the change
  - cascade-new-test-strategy 1. test-strategy.md, the statement's claim-to-check section (F4) — apply · check 7: read whole (check7-test-plan-test-strategy, END) — distinct; Accurate this-chunk addition (the statement's names and counts alone pinned)
  2. (sampler) test-data-fixtures.md:28 a data-seeds row for repository text read by path — apply · check 7: read whole (check7-test-plan-test-data-fixtures, END): the seeds table holds no row for a test whose data is the committed text — distinct
  3. (sampler) unit-test-strategy#5.1 — merged into cascade-g2 2 (a duplicate of the same new entity)
- obs-plan: 0 proposals · fanout-obs-plan.md · 2 cascade proposals · fanout-obs-plan--cascade.md · 2 facts · 2 placed · 0 no home · (sampler) 0 lost
  - note: D-obs-instrumentation, D-obs-stack, D-obs-pii clean
  - cascade 1. log-coverage.md escher-driver emitter, the statement restates its spans and the scrub's limits, no pin reads them (F2) — apply · check 7: read whole (check7-obs-plan-log-coverage, END) — distinct; Accurate this-chunk addition
  - cascade 2. log-coverage.md absence group, the pin recorded silent (F1) — reject · check 4: an `observed absent` row whose own `searched:` says no silence census was fired over the file — an absence with no search behind it
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 7 cascade proposals · fanout-a11y-plan--cascade.md · 1 fact · 1 placed · 0 no home · (sampler) 0 lost
  - note: no detector fires; the atowner body rows are the cascade's
  - cascade 1. a11y-scope-summary.md:17 AT owner — apply · check 1: the plan's atowner entry; the report lists the site
  - cascade 2. a11y-strategy.md:9 AT owner — apply · same
  - cascade 3. a11y-strategy.md:21 AT owner (the checked-state fix's owner kept) — apply · same
  - cascade 4. a11y-plan.md:9 — apply · same (dependent-of cascade 1)
  - cascade 5. a11y-plan.md:27 — apply · same (dependent-of cascade 2)
  - cascade 6. a11y-plan.md:35 — apply, the checked-state fix keeping «(target: next)» · same (dependent-of cascade 3)
  - cascade 7. a11y-scope-summary.md, the statement publishes the a11y reach, no pin reads it (F1) — apply · check 7: read whole (check7-a11y-plan-a11y-scope-summary, END) — distinct; Accurate this-chunk addition

## Totals
Proposals 38 (doc agents 3 · cascade 32 · sampler 3) + raised by main 3 = 41: apply 38 (35 edits once the three merges fold: test-plan 1 = cascade-g1 4, cascade-g2 3 and (sampler) 3 into cascade-g2 2) · escalate 2 (security 8, 9) · reject 1 (obs cascade 2, check 4) · rejected for a source the report does not carry 0.
Sampler lines: architecture `sampler: E carried 6 · the cascade lost 0` · security-plan `sampler: E carried 5 · the cascade lost 1 (threat-model-summary#3.1)` · design-system `sampler: E carried 0 · the cascade lost 0` · test-plan `sampler: E carried 6 · the cascade lost 2 (test-data-fixtures#3.7, unit-test-strategy#5.1)` · obs-plan `sampler: E carried 0 · the cascade lost 0` · a11y-plan `sampler: E carried 3 · the cascade lost 0`.
Expected amendments (check 5): all six entries matched by a proposal (mcpcarry · causes9 · atowner · readme12 · docsdir · ciscripts/§4); `matrix#v020-10` — no master site. Disproved claims (check 6): causes9, atowner, mcpcarry each matched; the generic owner phrase at `input-validation.md:197` — no change, as the report carries it. Citation dispositions: no `claim false` row.
