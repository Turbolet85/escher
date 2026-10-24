# Fan-out results — 2026-10-24-upstream-sync-polish-and-ship (wrap P2)

Batch 1 (7 doc agents · 6 task writers · 51 category detectors): before `tree 060903c220c898e820472f5bba302d529c6388da index d7bdabcc572804eb9747017b0f4adff395f9937b head 34f6f051b81460d82837f9433dbe1faebaf49e56`; after: 64 `A` (one per agent) + `M detail-2026-10-24-upstream-sync-polish-and-ship.json` (the tool's trail); index and head unchanged. Re-spawn pair (2 design-system category detectors, [DETECTORS] `D-new-detail`): before `tree e1d4f3be…`, after 2 `A` + the trail; raw returns kept as `.raw-fanout-design-system--{surface,depth-strategy}.md`. Classifier pair (73, Haiku · max): before `tree 6a1d83483a3f41ec859e85620e19c0ef5d094f93`, after `tree b9c0317028919fa9a5ce4061f7ec00301e1393e6`, 73 `A`, index and head unchanged; one spawn refused at the harness's 20-subagent limit (stopped, resumed on the operator's word at ≤ 16, inputs#I9); `answered` 73/73 PASS, headers 73/73. Strong-reader pair (11): before `tree 65bd678cbfcf89cea45e6da25a77757ffaba4957`, after `tree ca996c0633865381ab552eddf6bf23ae4a326410`, 22 `A`; `answered --kind account` 11/11 PASS; HEADER · SPLIT · OWN_FILE · PROPOSED_ARE_PICKED · NO_ESCAPES 11/11 ok.

Disposition words: **apply** · **apply (merged → X)** (one apply carries several proposals of one fact) · **reject (dup → X)** (the fact stated once, at X — amendment-flow §Apply, a fact found standing elsewhere in this pass is stated once) · **reject**. The check that decided each is named.

## architecture: 10 proposals · fanout-architecture.md · 39 cascade proposals · fanout-architecture--cascade-g{1..4}.md · (sampler) 9 lost proposals (6 entities)
sampler: E carried 37 · the cascade lost 6 (conventions#2.1, established-decisions#10.1, occupied-resources#6.1, standard-contracts#2.2, standard-contracts#6.2, standard-contracts#9.2)
note (doc agent): category-file standing groups and the `layout/inline` paths left to the cascade and the citation sweep.
- 1 D-arch-resources §Occupied Resources → Environment variables (MULTI_DOC_*) — apply (merged → cascade-g3 12, body cascade-g3 13) · check 1 Accurate this-chunk addition + the plan's Expected amendments entry naming the registration; an env read of an upstream example, no carrier or crossing
- 2 D-arch-resources §Occupied Resources → Process-wide state (PARALLEL_TRAVERSAL_LOCK) — apply (merged → cascade-g3 9, placed as the (sampler) occupied-resources#6.1 record) · check 1 Accurate addition + plan entry
- 3 D-arch-resources §Occupied Resources → Process-wide state (mimalloc allocator) — apply (merged → cascade-g3 10, the (sampler) record) · check 1 + plan entry
- 4 D-arch-resources §Standard Contracts → Nodes and widgets (Node::out_of_flow, ElementData::classes) — apply (merged → cascade-g3 4, placed as (sampler) standard-contracts#9.2 bullets) · check 1: Accurate addition; Registry over-reach fails its category-grain precondition (the group registers public symbols one by one, e.g. `ElementData::is_disabled`, standard-contracts.md:74), so no collision
- 5 D-retired §Established Decisions → [Style threading] (body :44) — apply · check 1 D-retired (the report's Retired names the two-`Parallel`-documents warning) + plan entry; settles citation rows config.rs:53-55 and :11-21 (claim false)
- 6 D-arch-decisions §Stack (body :21 flake drift) — apply (merged with cascade-g1 9) · check 1 + plan entry
- 7 D-arch-decisions §Stack (body: mimalloc) — apply · check 1 + plan entry («body :21: mimalloc by default»)
- 8 D-arch-decisions §Established Decisions → [Layout] replaced element — apply (merged → cascade-g2 5, established-decisions.md:84) · check 1 engine change for every document on the founder's word (inputs#I4) + plan entry
- 9 D-arch-decisions §Established Decisions → [Scrolling and input] hit skip — reject (dup → cascade-g2 7 [DOM semantics] for the decision, cascade-g3 1 for the contract) · stated once
- 10 D-arch-decisions §Project Intent (body :286) — apply · check 1 + plan entry
- cascade-g1 1 design-philosophy.md:53 `Node` carries only tree structure — apply · check 1 + plan entry (§Design Philosophy :53)
- cascade-g1 2 design-philosophy.md:48 hit skip — reject (dup → cascade-g3 1 / cascade-g2 7)
- cascade-g1 3 design-philosophy.md:62 multi_doc — reject (dup → cascade-g4 4 existing-scopes, cascade-g3 6 CLIs)
- cascade-g1 4 stack-and-technologies.md:39 tokio resolved 1.54.1 — apply · check 1 (a lockfile reading, the entity's «resolved» shape)
- cascade-g1 5 stack-and-technologies.md:55 lock + thin-vec — reject (dup → occupied-resources record, body :44)
- cascade-g1 6 stack-and-technologies.md:60 mimalloc default — apply · plan entry
- cascade-g1 7 stack-and-technologies.md:66 WPT runner mimalloc — apply · check 1
- cascade-g1 8 stack-and-technologies.md:69 flake 1.91.0 — apply · plan entry
- cascade-g1 9 architecture.md:21 flake drift — apply (merged with doc 6)
- cascade-g2 1 established-decisions.md:32 [Default features] browser `mimalloc` — apply · check 1
- cascade-g2 2 established-decisions.md:75 CARRY owner → residuals line — apply · check 5 (report-raised entry «The CARRY 3–5 owners», P5 card row 2, inputs#I5)
- cascade-g2 3 established-decisions.md:76 — apply (dependent of g2 2)
- cascade-g2 4 established-decisions.md:78 — apply (dependent of g2 2)
- cascade-g2 5 established-decisions.md:84 [Layout] replaced — apply (merged with doc 8)
- cascade-g2 6 established-decisions.md:85 [Layout] out_of_flow — reject (dup → cascade-g1 1, Standard Contracts bullets)
- cascade-g2 7 established-decisions.md:92 [DOM semantics] hit skip — apply · check 1 engine change on the founder's word (inputs#I4)
- cascade-g2 8 established-decisions.md:131 [Dioxus integration] Parallel — reject (dup → doc 5 body :44)
- cascade-g2 9 established-decisions.md:137 [Browser app] — reject (dup → g2 1, cascade-g1 6, doc 5)
- cascade-g2 10 established-decisions.md:143 [WPT runner] mimalloc — reject (dup → cascade-g1 7, occupied-resources record)
- cascade-g3 1 standard-contracts.md:45 hit walk skips dropped nodes — apply (merged with (sampler) standard-contracts#6.2) · check 1
- cascade-g3 2 architecture.md:143 broad line — apply (dependent of g3 1)
- cascade-g3 3 standard-contracts.md:60 replaced — reject (dup → cascade-g2 5)
- cascade-g3 4 standard-contracts.md:67 out_of_flow + classes — apply (merged with doc 4)
- cascade-g3 5 standard-contracts.md:192 launch Parallel — reject (dup → doc 5)
- cascade-g3 6 standard-contracts.md:229 CLIs multi_doc — apply · check 1
- cascade-g3 7 architecture.md:187 CLIs broad line — apply (dependent of g3 6)
- cascade-g3 8 standard-contracts.md:237 browser Parallel — reject (dup → doc 5)
- cascade-g3 9 occupied-resources.md:38 lock — apply (merged with doc 2)
- cascade-g3 10 occupied-resources.md:38 mimalloc — apply (merged with doc 3)
- cascade-g3 11 architecture.md:211 — apply (dependent)
- cascade-g3 12 occupied-resources.md:55 env vars — apply (merged with doc 1)
- cascade-g3 13 architecture.md:216 — apply (dependent)
- cascade-g4 1 project-intent.md:9 — apply (merged with doc 10)
- cascade-g4 2 existing-scopes.md:46 `inline` a directory — apply · plan entry
- cascade-g4 3 existing-scopes.md:103 blitz-tests `hit_test_dropped_nodes` — apply · check 1
- cascade-g4 4 existing-scopes.md:123 root examples `multi_doc` — apply · plan entry
- cascade-g4 5 cross-cutting-patterns.md:15 multi_doc timing — apply · check 1
- cascade-g4 6 cross-cutting-patterns.md:33 hit skip — reject (dup → g3 1 / g2 7)
- cascade-g4 7 cross-cutting-patterns.md:33 classes/out_of_flow — reject (dup → g1 1, Standard Contracts)
- (sampler) conventions#2.1 :15 WPT runner's direct `mimalloc = "0.1.48"` an exception to `[workspace.dependencies]` — apply (sampler) · check 1, read against the file at apply
- (sampler) established-decisions#10.1 :67 range merge surface met, `RangeInputData` kept — apply (sampler) · check 1
- (sampler) established-decisions#10.1 :69 keyboard merge surface met additively — apply (sampler, dependent)
- (sampler) occupied-resources#6.1 :49 lock record — apply (sampler; merged with doc 2 — its placement taken)
- (sampler) occupied-resources#6.1 :49 mimalloc record — apply (sampler; merged with doc 3)
- (sampler) standard-contracts#2.2 :17 StyleThreading contract — reject (sampler; dup → doc 5)
- (sampler) standard-contracts#6.2 :54 hit walk — apply (sampler; merged → cascade-g3 1)
- (sampler) standard-contracts#9.2 :75 out_of_flow — apply (sampler; merged with doc 4)
- (sampler) standard-contracts#9.2 :75 classes — apply (sampler; merged with doc 4)
- raised 11 §Cross-cutting Patterns (body :276) `layout/inline.rs:236` · `:877-880` → `layout/inline/layout.rs` — apply · check 5 (citation rows `claim false`, the path gone)
- raised 12 category files `layout/inline.rs` paths (conventions.md:27 ×2, :46; established-decisions.md:57, :85 ×2) — apply · check 5 (the report's `inline` pattern: «changed (path re-points)»)

## security-plan: 8 proposals · fanout-security-plan.md · 7 cascade proposals · fanout-security-plan--cascade.md · (sampler) 6 lost proposals (4 entities)
sampler: E carried 10 · the cascade lost 4 (dependency-security#1.0, error-handling#3.1, input-validation#1.4, secret-management#1.2)
note (doc agent): `security-plan.md:363` inline.rs left to the sweep; Graceful degradation not proposed.
- 1 D-security-deps Pinning (body :234) — apply (merged → cascade 5) · check 1 + plan entry
- 2 D-security-deps the 2026-10-24 sync record — apply (merged → cascade 4) · plan entry
- 3 D-security-deps Audit tool RUSTSEC-2026-0386 — apply (merged with (sampler) dependency-security#1.0) · plan entry; detector graded escalate on no violation → the «graded escalate, invariant holding, plan names it» rule
- 4 D-security-deps body bullet mimalloc — reject (dup → 2)
- 5 D-security-auth Environment values read (body :294) — apply · plan entry; non-secret env reads of an upstream example
- 6 D-security-input new Input Validation entity — reject (dup → cascade 1 + 7)
- 7 D-security-input body :53 examples CLI summary — apply · check 1
- 8 D-security-input Error Handling body :369 · :370 — apply · plan entry
- cascade 1 input-validation.md:13 multi_doc parse-with-default — apply · check 1
- cascade 2 input-validation.md:245 ratio guard covers the natural-height coercion — apply · plan entry (:245 kept)
- cascade 3 input-validation.md:350 attr_test unwrap — reject (dup → 8)
- cascade 4 dependency-security.md:20 sync record — apply (merged with 2)
- cascade 5 security-plan.md:234 — apply (merged with 1)
- cascade 6 secret-management.md:22 — reject (dup → 5)
- cascade 7 logging-monitoring.md:15 multi_doc println — apply · check 1
- (sampler) dependency-security#1.0 :10 tokio record — apply (sampler; merged with 3)
- (sampler) error-handling#3.1 :26 PARALLEL lock no panic path — apply (sampler) · check 1, a panic path removed (the report's Retired)
- (sampler) error-handling#3.1 :26 multi_doc panic — reject (sampler; dup → 8)
- (sampler) error-handling#3.1 :26 attr_test unwrap — reject (sampler; dup → 8)
- (sampler) input-validation#1.4 :19 multi_doc entity — reject (sampler; dup → cascade 1 + 7)
- (sampler) secret-management#1.2 :15 env reads — reject (sampler; dup → 5)
- raised 9 §Error Handling (body :363) `layout/inline.rs:732` → `layout/inline/layout.rs:732` — apply · check 5 (citation row `claim false`, the path gone)
- raised 10 input-validation.md:248 `layout/inline.rs:470-478` path — apply · check 5 (the `inline` pattern)

## design-system: 0 proposals · fanout-design-system.md · 1 cascade proposal · fanout-design-system--cascade.md · (sampler) 0 lost
sampler: E carried 1 · the cascade lost 0
note: D-design-tokens holds; the flake citations are the sweep's.
- cascade 1 depth-strategy.md:26 the hit walk skips dropped nodes — apply, as a short clause pointing to architecture's Standard Contracts · check 1

## layout-templates: 1 proposal · fanout-layout-templates.md
- 1 D-layout-surface §Surface: cli `multi_doc` beside `paint_bench` — apply · check 1

## test-plan: 3 proposals · fanout-test-plan.md · 8 cascade proposals · fanout-test-plan--cascade-g{1..3}.md · (sampler) 4 lost proposals (4 entities)
sampler: E carried 10 · the cascade lost 4 (integration-test-strategy#1.1, integration-test-strategy#3.1, quality-gates-coverage-targets#2.1, test-scope-summary#3.2)
note (doc agent): the tallies in ci-integration.md are the cascade's.
- 1 D-tests-coverage §3 Agent-run Proof re-count link (body :176) — apply · plan entry
- 2 D-tests-coverage blitz-tests `hit_test_dropped_nodes` — apply (merged → cascade-g2 1)
- 3 D-tests-coverage `PARALLEL_TRAVERSAL_LOCK` read by no headless check — apply (merged → cascade-g1 3)
- cascade-g1 1 test-scope-summary.md:19 multi_doc holds no test — apply · check 1
- cascade-g1 2 test-scope-summary.md:22 hit testing past dropped nodes covered — apply · check 1
- cascade-g1 3 test-scope-summary.md:22 the lock asserted by no check — apply · check 1 (merged with doc 3)
- cascade-g1 4 test-plan.md:19 broad line — apply (dependent)
- cascade-g2 1 test-scope-summary.md:97 upstream's `hit_test_dropped_nodes` 1 — apply (merged with doc 2) · plan entry
- cascade-g3 1 test-strategy.md:48 145 files / 144 targets — apply · plan entry (§2 144 → 145)
- cascade-g3 2 ci-integration.md:40 per-OS tallies — apply · plan entry (§9)
- cascade-g3 3 ci-integration.md:59 local re-count — apply · plan entry (§9)
- (sampler) integration-test-strategy#1.1 :10 hit-test boundary record — reject (sampler; dup → cascade-g1 2, cascade-g2 1)
- (sampler) integration-test-strategy#3.1 :28 lock absence — reject (sampler; dup → cascade-g1 3)
- (sampler) quality-gates-coverage-targets#2.1 :14 keyboard_compat re-read — apply (sampler) · check 1 (gate entries 14, 15; the exclusion stands)
- (sampler) test-scope-summary#3.2 :29 out_of_flow / classes no unit test — apply (sampler) · check 1

## obs-plan: 3 proposals · fanout-obs-plan.md · 2 cascade proposals · fanout-obs-plan--cascade.md · (sampler) 1 lost proposal (1 entity)
sampler: E carried 3 · the cascade lost 1 (pii-scrubbing-compliance#1.1)
note (cascade): the browser's `document_loader.rs` / `toolbar.rs` cites may have moved — the category-file citation question.
- 1 D-obs-stack §3 silence census (the body's `absence · logs` block; `file: new` resolves to the body) — apply · plan entry
- 2 D-obs-stack §6 examples emitter multi_doc — apply (merged → cascade 2)
- 3 D-obs-stack §1 examples scope — apply · check 1
- cascade 1 metric-coverage.md:16 multi_doc row — apply · check 1
- cascade 2 log-coverage.md:33 multi_doc println cite — apply (merged with 2)
- (sampler) pii-scrubbing-compliance#1.1 :11 sink allowlist re-check — apply (sampler) · plan acceptance «sink allowlist re-checked»

## a11y-plan: 0 proposals · fanout-a11y-plan.md · 1 cascade proposal · fanout-a11y-plan--cascade.md · (sampler) 1 lost proposal (1 entity)
sampler: E carried 1 · the cascade lost 1 (a11y-scope-summary#1.9)
- cascade 1 visual-design-verification.md:75 hit skips dropped nodes — apply, a short clause pointing to architecture's Standard Contracts · check 1
- (sampler) a11y-scope-summary#1.9 :18 accesskit re-read — apply (sampler) · check 1 (gate entry 34)

## Checks over the whole set
- Check 2 (cross-contradiction): none — every pair on one section edits it the same way; merged above.
- Check 3 (intent): scope record none; the take is the founder's ratified take (inputs#I4).
- Check 4 (absence): the report's «Sweep hazard» — the pre-pass sweep reads master bodies only — is confirmed by measurement: `cites.py` and `cascade.py` read no `.andromeda/*-detail/` file; 402 citations in category files name a file this take changed (a scratch count over `git diff --name-only a356f9086 HEAD`). Raised to the operator (halt question 1).
- Check 5 (expected amendments): every plan entry is matched above; the CARRY 3–5 owners raised; citation rows `claim false` raised (11, 9).
- Check 6 (disproved claims): CARRY 4 forecast — no master site, a plan forecast (disposed: no master site); inputs#I3 «twelve» — no master site (disposed).
- Check 7: every category-file addition above is read against its target file whole before apply.
- Boundary widening: none — the three engine changes for every document carry the founder's own word (inputs#I4, the playbook's «An engine behaviour change for every document», routine); no new carrier, listener, verb or crossing; the `MULTI_DOC_*` reads are an upstream example's, registered.
- No fitting category: 0.
