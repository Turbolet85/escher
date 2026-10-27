# Fan-out results — 2026-10-26-codebase-health · wrap 2026-10-27T02-06-43

Stage 1 (`stage1` + `stage1-r1`): 64 of 64 pass; `check` `proposals: architecture 0 · security-plan 0 · design-system 0 ·
layout-templates 0 · test-plan 0 · obs-plan 0 · a11y-plan 0`. Classify (`classify` + `classify-r1`): 74 of 74 pass. Strong:
6 of 6 pass. New-detail: 1 of 1 pass. Draw: `sample: yes · wrap · 2026-10-26-codebase-health · draw 0 of 10`.

## Verdicts
- architecture: 0 proposals · fanout-architecture.md · 7 cascade proposals · fanout-architecture--cascade.md · 6 facts ·
  6 placed · 0 no home · (sampler) 0 lost — note: D-arch-resources and D-arch-decisions hold; the disproved sweep-reach
  sentences sit in `existing-scopes.md:115` and `standard-contracts.md:273`, no body line restating them.
- security-plan: 0 proposals · fanout-security-plan.md · 0 cascade proposals · fanout-security-plan--cascade.md · 0 facts
  · 0 placed · 0 no home · (sampler) 0 lost — note: D-security-input · auth · deps clean; the `security-plan.md:295`
  move is the citation sweep's.
- design-system: 0 proposals · fanout-design-system.md · no strong reader (pick `yes 0`) · 0 facts · (sampler) 0 lost —
  note: D-design-tokens holds; `color-palette.md:107` `hover 152` a token-hazard no-change.
- layout-templates: 0 proposals · fanout-layout-templates.md (whole) — note: D-layout-surface holds; `layout-templates.md:54`
  is `stand_lint.rs` citations.
- test-plan: 0 proposals · fanout-test-plan.md · 8 cascade proposals · fanout-test-plan--cascade-g1.md (2) ·
  fanout-test-plan--cascade-g2.md (6) · 5 facts · 5 placed · 0 no home · (sampler) 0 lost — note: no test-plan body
  drift; the moved facts are the cascade's and the citation sweep's.
- obs-plan: 0 proposals · fanout-obs-plan.md · 1 cascade proposal · fanout-obs-plan--cascade.md ·
  fanout-obs-plan--cascade-new-log-coverage.md (2) · 3 facts · 3 placed (by pass 2) · 0 no home · (sampler) 1 lost,
  beside a placed fact (log-coverage#9.4, beside F3) — note: no detector fires; `mcp_tests.rs`'s census reading is new
  detail in log-coverage, not `obs-plan.md:93`.
- a11y-plan: 0 proposals · fanout-a11y-plan.md · 3 cascade proposals · fanout-a11y-plan--cascade.md · 2 facts · 2 placed
  · 0 no home · (sampler) 0 lost — note: D-a11y-surface · obs-schema clean; the disproved membership clause stands only in
  `a11y-plan-detail/ci-integration.md:9`.

No fix was made to any file; no raw twin but the re-spawns' `.raw-*` move-asides.

## Dispositions
Rejected for a source the report does not carry: 0. Check 3: the scope record holds 0 lines, no divergence. Check 5: every
plan entry covered — the citation spans by the sweep (8 re-pointed, `citation-dispositions.md`), test-plan §10's coverage
reading by fanout-test-plan--cascade-g2.md 4, `mcp_tests.rs` in the census by fanout-obs-plan--cascade.md 1; the
`not carried` entries need none; no `claim false` row. Check 6: the three disproved sentences end matched —
`existing-scopes.md:115` by fanout-architecture--cascade.md 6, `standard-contracts.md:273` by 5,
`a11y-plan-detail/ci-integration.md:9` by fanout-a11y-plan--cascade.md 1; the playbook's disproved-claim rule governs
(the leg check unchanged by the chunk, report Spec claims disproved), its CARRY placed at P5 on «Stand requirement sweep» (Q3). Escalations Q1–Q3 resolved by the operator's dialog answers, `escalation-answers.md`.

fanout-architecture--cascade.md
1. apply — check 1: Accurate this-chunk addition (the dev-host complexity reading is this chunk's plan entry; report Dev-tool versions, Harness / gate surface); check 7: architecture-newgroup-stack-and-technologies.md · distinct
2. escalate → Q2 — check 3: it states the audit's ceiling of 15 as a convention «code is now held to», while the founder's M1 ruling (intent §1) brings escher's three functions under it and 85 functions still read over it; check 7: architecture-newgroup-conventions.md · distinct · resolved: reject (Q2)
3. escalate → Q2 — rides with 2 (how the reading counts nested spaces, the same fact as fanout-test-plan--cascade-g2.md 6); check 7: architecture-newgroup-conventions.md · distinct · resolved: reject (Q2)
4. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, both tests' helpers); check 7: architecture-newgroup-conventions.md · distinct · as: state what the two tests do, not a rule for every test: «; since 2026-10-26-codebase-health escher-driver's MCP listing test and blitz-tests' 0.2.0 requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private `#[track_caller]` helpers whose messages name the row and index only (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)»
5. apply — check 6: disproved claim `standard-contracts.md:273`, the playbook's disproved-claim rule; check 7: architecture-newgroup-standard-contracts.md · distinct
6. apply — check 6: disproved claim `existing-scopes.md:115`, the playbook's disproved-claim rule (D-cascade, no check 7)
7. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, `inside_stand_repeat`); check 7: architecture-newgroup-existing-scopes.md · distinct

fanout-test-plan--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, the sweep test's helpers); check 7: test-plan-newgroup-test-scope-summary.md · distinct
2. apply — check 6: the measured reach limit beside `test-scope-summary.md:120`, which the report rules exact; the playbook's disproved-claim rule; check 7: test-plan-newgroup-test-scope-summary.md · distinct · as: replace «its owner placed at that chunk's wrap P5» with «its fix owned by a `CARRY:` on the 0.3.0 route's «Stand requirement sweep» entry, placed at that chunk's wrap on the operator's word (Q3)»

fanout-test-plan--cascade-g2.md
1. apply — check 1: Accurate this-chunk addition (report Symbols / APIs, the listing test's helpers); check 7: test-plan-newgroup-unit-test-strategy.md · distinct
2. apply — check 1: the report's Cross-project / external claims carries CI#40434241601's six job tallies · as: keep the tallies and «every job equal to the base's CI#40389488295 (`19f9ff91ca`), the chunk adding no test»; drop the clause on the moves since CI#40134371519 (differences the report does not carry)
3. apply — check 1: the report's Outcome reads `ci-leg.sh coverage` green at implement, so «not re-run locally since» is false
4. apply — check 5: the plan's expected amendment (test-plan §10, escher-driver's coverage reading), report Counts / qualifiers moved
5. apply — check 1: Accurate this-chunk addition (report Counts / qualifiers moved, Dev-tool versions, Harness / gate surface, Deviations); check 7: test-plan-newgroup-quality-gates-coverage-targets.md · distinct · as: name it the code audit's cognitive-complexity ceiling of 15, read on the dev host with rust-code-analysis-cli 0.0.25 (no lockfile), test functions counted; escher's three functions over it brought under it since 2026-10-26-codebase-health, the engine's `clear_interaction_state_for_removed_node` left as is; no CI leg and no ceiling on every function, 85 reading over it at `3c04042ed` (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md) — no word, ruling or earlier reading in the body (provenance is the sidecar's)
6. apply-merged → fanout-test-plan--cascade-g2.md 5 — one record for the reading and how it sums; check 7: test-plan-newgroup-quality-gates-coverage-targets.md · distinct

fanout-a11y-plan--cascade.md
1. apply — check 6: disproved claim `a11y-plan-detail/ci-integration.md:9`, the playbook's disproved-claim rule (D-cascade, no check 7) · as: replace «their membership held by `stand_requirement_sweep`'s leg check» with «their names also read by `stand_requirement_sweep`'s leg check, which reads `ci-leg.sh` as one whole text by `contains` and so cannot see one dropped from the stand-repeat list alone while the a11y leg still names it — only both sites removed read red — so the list's membership is held by the two checks jointly (as measured at escher-0.3.0/chunks/2026-10-26-codebase-health/report.md)»
2. escalate → Q1 — check 7: a11y-plan-newgroup-ci-integration.md · contradiction (line 9's clause, the one 1 replaces) · resolved: reject (Q1) — merged: its fact rides fanout-a11y-plan--cascade.md 1's direction
3. escalate → Q1 — check 7: a11y-plan-newgroup-ci-integration.md · contradiction (line 9's clause, the one 1 replaces) · resolved: reject (Q1) — merged: its fact rides fanout-a11y-plan--cascade.md 1's direction

fanout-obs-plan--cascade.md
1. apply — check 1: Accurate this-chunk addition (report Counts / qualifiers moved, the census bullet; Coverage of new surfaces); check 7: obs-plan-newgroup-log-coverage.md · distinct · as: one record for the factoring's private items — `lint.rs`'s `readings` and `placement`, `mcp_tests.rs`'s `assert_described` · `assert_schema` · `assert_property`, `stand_requirement_sweep`'s `assert_0_2_0_row` · `assert_0_2_0_witness` · `inside_stand_repeat` — silent, messages naming a row and an index only; searched: the silence census over the three files reading 0, `mcp_tests.rs`'s first census, its env reads the 2 compile-time `env!` alone, escher-driver still 2 `info_span!`, no event or `#[instrument]`

fanout-obs-plan--cascade-new-log-coverage.md
1. apply-merged → fanout-obs-plan--cascade.md 1 — check 7: obs-plan-newgroup-log-coverage.md · distinct (`mcp_tests.rs`'s census and `env!` reads)
2. apply-merged → fanout-obs-plan--cascade.md 1 — check 7: obs-plan-newgroup-log-coverage.md · distinct (`lint.rs`'s helpers)

(sampler) cascade-compare-obs-plan.md
S1. reject — check 7: obs-plan-newgroup-log-coverage.md · twin · fanout-obs-plan--cascade.md 1 (sampler)
