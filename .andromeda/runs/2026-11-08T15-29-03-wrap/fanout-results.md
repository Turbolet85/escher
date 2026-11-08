# Fan-out results — 2026-11-08-what-can-i-do-here (wrap 2026-11-08T15-29-03, resumed)

Batches: cites 12 (10 pass, 2 re-spawned once in cites-r1, 2/2 pass) · stage1 13/13 (doc 7 · task 6) · classify 164
(163 pass, 1 re-spawned once in classify-r1, 1/1 pass, no dead draw) · strong 20/20 · newdetail 1/1 · newgroup 16/16.
Sampler: off (`cascade-sample.txt`, not forced).
Doc agents' own proposals (`check`'s `proposals:` line): architecture 3 · security-plan 4 · design-system 0 ·
layout-templates 2 · test-plan 0 · obs-plan 0 · a11y-plan 1.

## Verdict lines
- architecture: 3 proposals · fanout-architecture.md · 50 cascade proposals · fanout-architecture--cascade-g1.md ·
  fanout-architecture--cascade-g2.md · fanout-architecture--cascade-g3.md · fanout-architecture--cascade-g4.md ·
  fanout-architecture--cascade-g5.md · fanout-architecture--cascade-g6.md · fanout-architecture--cascade-g7.md ·
  7 facts · 7 placed · 0 no home
- security-plan: 4 proposals · fanout-security-plan.md · 29 cascade proposals · fanout-security-plan--cascade-g1.md ·
  fanout-security-plan--cascade-g2.md · fanout-security-plan--cascade-g3.md · fanout-security-plan--cascade-g4.md ·
  9 facts · 9 placed · 0 no home
- design-system: 0 proposals · fanout-design-system.md · 3 cascade proposals · fanout-design-system--cascade.md ·
  3 facts · 3 placed · 0 no home
- layout-templates: 2 proposals · fanout-layout-templates.md (whole — no category file)
- test-plan: 0 proposals · fanout-test-plan.md · 42 cascade proposals · fanout-test-plan--cascade-g1.md ·
  fanout-test-plan--cascade-g2.md · fanout-test-plan--cascade-g3.md · fanout-test-plan--cascade-g4.md ·
  fanout-test-plan--cascade-g5.md · fanout-test-plan--cascade-new-unit-test-strategy.md · 11 facts · 11 placed (1 by
  the pass) · 0 no home
- obs-plan: 0 proposals · fanout-obs-plan.md · 11 cascade proposals · fanout-obs-plan--cascade.md · 4 facts · 4 placed
  · 0 no home
- a11y-plan: 1 proposal · fanout-a11y-plan.md · 13 cascade proposals · fanout-a11y-plan--cascade-g1.md ·
  fanout-a11y-plan--cascade-g2.md · 9 facts · 9 placed · 0 no home

Remarks carried: architecture g7's — blitz-tests' `tests/common/mod.rs` declarer count (22) does not move in the report
though `stand_actions` declares `mod common;` (raised below, main's measurement). Security g1's — proposal 3 rests on
`docs/capabilities.md`'s «What an MCP client carries onward» naming the actions answer: the report's New text lists
`docs/capabilities.md` 372-373, and line 373 reads «an actions answer: ids, owner ids, verb names, closed cause words and
booleans, never a value or typed text». Architecture g2's and security g3's — is_select's caller asked to be confirmed
(main's measurement below). Obs's — body facts fct-icom2cq7 and fct-pzih6tk6 and test-plan §3's fct-glytb3w3 /
fct-6bs5asmb / fct-63656bwh are the cascade sweep's rows.

Main's measurements this Validate (read on the work tree, each against `7e2e8be6`): `mod common;` declared by 23
blitz-tests files (22 at base), `stand_actions.rs` declaring it at line 26 (`git grep -l '^mod common;'
tests/blitz-tests/tests/`); `packages/escher-driver/src/cli_tests.rs:438` `assert_eq!(rows.len(), 102)`, `actions`'s row
at 149; `is_select` called once in escher-driver, `execute.rs:443`, inside `target_of` (execute.rs:435), which `locate`
(execute.rs:423) ends in and `Session::actions` calls; `snapshot`'s `text` help at `schema.rs:203-207` reads «the screen
as one text, or the part of it asked for: one line per element, nested by indent, a part's lines as they read in the
whole text; a line's state words, each written only where the element states it, are enabled or disabled, checked or
unchecked, value=, selected or unselected, pressed or unpressed, invalid, focused and clickable; a mixed toggle reads
unpressed, and a grammar or spelling error reads invalid».

## Dispositions

fanout-architecture.md
1. apply — check 1: rul-ipb3atbm, the report's Schema / config (VERBS 13 → 14) · as: the body line's «thirteen verbs» reads «fourteen verbs», nothing else added to the broad line; what `actions` is and answers is the Driver session detail's (fanout-architecture--cascade-g3.md 2, 12)
2. apply — check 1: rul-ipb3atbm (Symbols / APIs, `actionable_nodes` and the one rule); check 5: the plan's entry «→ Dioxus DOM bridge»
3. escalate → Q1 — check 1: rul-izni57qv, the actions answer joining what crosses (plan entry, inputs#I4, inputs#I6) · resolved: apply (Q1)

fanout-architecture--cascade-g1.md
1. escalate → Q1 — check 7: architecture-newgroup-established-decisions.md · distinct; check 1: rul-izni57qv (Established Decisions → Driver session, the actions answer's crossing) · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date; the word and its relays go in the sidecar entry
2. escalate → Q1 — check 1: rul-izni57qv (an actions answer at rest in a kept check) · resolved: apply (Q1)
3. apply-merged → fanout-architecture.md 3 — the same body fact fct-nb3g6bqn
4. apply — check 1: rul-ipb3atbm (Counts [sc-readers], Symbols / APIs `ACTIONABLE_AT_BOOT`)

fanout-architecture--cascade-g2.md
1. apply — check 1: rul-ipb3atbm (Symbols / APIs, `locate` ends in `target_of`) · as: «`is_select`, the driver's select test, read within the workspace by execute.rs's crate-private `target_of` alone (execute.rs:443), which `locate` ends in and `Session::actions` also calls» — main's measurement; the rest of the entity unchanged

fanout-architecture--cascade-g3.md
1. apply — check 1: rul-ipb3atbm (Crates / modules: re-exports 57 → 59, private modules 17 → 18); check 5: citation row fct-t7zq5feb `claim false`
2. escalate → Q1 — check 1: rul-izni57qv (the validated verb set admits `actions`) · resolved: apply (Q1)
3. apply — check 1: rul-cepujoy6 / rul-ipb3atbm (`FieldKind` 12 → 13)
4. apply — check 1: rul-ipb3atbm (describe's row)
5. apply — check 1: rul-ipb3atbm · as: `Command` gains `Actions`, validated from `("actions", [])`, after `Graph`; no variant count is written
6. apply — check 1: rul-ipb3atbm
7. apply — check 1: rul-ipb3atbm
8. apply — check 1: rul-ipb3atbm (the command span, `verb` alone)
9. apply — check 1: rul-ipb3atbm (the MCP listing) — dependent of 2, applied with it
10. apply — check 1: rul-ipb3atbm
11. apply — check 1: rul-ipb3atbm (the written form, the content Q1's)
12. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
13. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
14. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
15. apply — check 7: architecture-newgroup-standard-contracts.md · distinct; the limit and its owner `res-7cc5bi4k`, target next (inputs#I5)
16. apply — check 1: rul-ipb3atbm (Counts [instance])
17. apply — check 1: rul-ipb3atbm — dependent of 16
18. apply-merged → fanout-architecture.md 1 — the same body fact fct-ksgwc2yv
19. apply — check 5: citation row fct-t7zq5feb@c23276 `claim false` (raised by main) · §Standard Contracts (standard-contracts, fct-t7zq5feb) · change: the quoted help of `snapshot`'s `text` reads as `schema.rs:203-207` holds it (Main's measurements above), its citation's range re-read to 202-207

fanout-architecture--cascade-g4.md
1. apply — check 1: rul-ipb3atbm (VERBS 13 → 14)
2. apply — check 1: rul-ipb3atbm (record_result's no-step outcomes, the arch outcome)
3. reject — check 7: architecture-newgroup-standard-contracts.md · contradiction · fct-t7zq5feb · rewritten by fanout-architecture--cascade-g3.md 2; the same F1 fact as fanout-architecture--cascade-g3.md 12, stated once there
4. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g3.md 13
5. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g3.md 14
6. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g3.md 15
7. apply — check 1: the Retired bullet's item (`D-retired`), routine
8. apply — check 1: rul-cepujoy6 (`keyed`, a closed bool inside the answer); check 5: the plan's entry; the PROVISIONAL mark kept (the overseer's call, inputs#I4)
9. apply — check 1: rul-ipb3atbm
10. apply — check 7: architecture-newgroup-standard-contracts.md · contradiction · fct-kllkxz3d · rewritten by fanout-architecture--cascade-g4.md 7
11. apply — check 7: architecture-newgroup-standard-contracts.md · distinct; the PROVISIONAL mark kept
12. apply — check 1: rul-ipb3atbm (the command line's verbs) — dependent of fanout-architecture--cascade-g3.md 2
13. apply-merged → fanout-architecture.md 1 — the same body fact fct-ksgwc2yv

fanout-architecture--cascade-g5.md
1. apply — check 1: rul-ipb3atbm (Counts [cold-tools])
2. apply-merged → fanout-architecture.md 1 — the same body fact fct-ksgwc2yv

fanout-architecture--cascade-g6.md
1. escalate → Q1 — check 1: rul-izni57qv (Occupied Resources, both carriers and a kept check take the actions answer out) · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date
2. apply — check 1: rul-ipb3atbm; check 5: citation row fct-jzxi3spj `claim false`
3. apply — check 1: rul-ipb3atbm (the obs outcome)
4. apply — check 1: rul-ipb3atbm

fanout-architecture--cascade-g7.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 1: rul-ipb3atbm; check 5: citation row fct-mzrxx2a3 `claim false`
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm
5. apply — check 1: rul-ipb3atbm
6. apply — check 1: rul-ipb3atbm (Deviations, the shared helper)
7. apply — check 1: rul-ipb3atbm
8. apply — check 1: rul-ipb3atbm · as: also `tests/common/mod.rs` read «by 23» (22 until this chunk), main's measurement
9. apply — check 5: the report's Counts row omits it (raised by main) · §Existing Scopes (existing-scopes, fct-4zmppehn) · change: blitz-tests' `tests/common/mod.rs` «read through `mod common;` by 23 stand checks» (22 until this chunk, `stand_actions` the twenty-third), main's measurement

fanout-security-plan.md
1. escalate → Q1 — check 1: rul-izni57qv (the validated table admits `actions`) · resolved: apply (Q1)
2. escalate → Q1 — check 1: rul-izni57qv (the socket's replies carry the actions answer) · resolved: apply (Q1)
3. escalate → Q1 — check 1: rul-izni57qv (Threat Model, the socket vector) — dependent of 2 · resolved: apply (Q1)
4. escalate → Q1 — check 1: rul-izni57qv (Threat Model, the MCP vector) — dependent of 2 · resolved: apply (Q1)

fanout-security-plan--cascade-g1.md
1. escalate → Q1 — check 7: security-plan-newgroup-threat-model-summary.md · distinct; check 1: rul-izni57qv · resolved: apply (Q1)
2. escalate → Q1 — check 7: security-plan-newgroup-threat-model-summary.md · distinct; check 1: rul-izni57qv · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date
3. escalate → Q1 — check 1: rul-izni57qv (the carried-onward set gains the actions answer; the report's New text `docs/capabilities.md` 372-373) · resolved: apply (Q1)
4. apply-merged → fanout-security-plan.md 3 — the same body fact fct-z5ttw3ol
5. apply-merged → fanout-security-plan.md 4 — the same body fact fct-dgkl4a64

fanout-security-plan--cascade-g2.md
1. apply — check 1: rul-ipb3atbm (Counts [cold-tools])
2. apply — check 1: rul-ipb3atbm (Counts [cold-tools], New text `test_cold_agent.py` 144-145); check 5: citation row fct-pbvi5fsv `claim false`
3. escalate → Q1 — check 1: rul-izni57qv (the socket's carried set gains the actions answer) · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date; the `keyed` field's PROVISIONAL mark kept
4. escalate → Q1 — check 1: rul-izni57qv (an `actions` step kept with its answer whole) · resolved: apply (Q1)
5. apply — check 1: rul-ipb3atbm (Counts [instance])
6. escalate → Q1 — check 1: rul-izni57qv (the validated table admits `actions`); check 5: citation row fct-awfxp663 `claim false` · resolved: apply (Q1)
7. apply — check 7: security-plan-newgroup-input-validation.md · distinct
8. apply — check 7: security-plan-newgroup-input-validation.md · distinct
9. apply — check 7: security-plan-newgroup-input-validation.md · distinct
10. escalate → Q1 — check 1: rul-izni57qv (the command line answers the actions answer) · resolved: apply (Q1)
11. escalate → Q1 — check 1: rul-izni57qv (the MCP tool answers the actions answer); check 5: citation row fct-f4eyvu3v `claim false` · resolved: apply (Q1)
12. apply — check 7: security-plan-newgroup-input-validation.md · distinct
13. escalate → Q1 — check 1: rul-izni57qv; check 5: citation row fct-d2yejoup `claim false` · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date; `keyed` provisional kept
14. apply — check 1: rul-ipb3atbm (Symbols / APIs, the record of ids)
15. apply-merged → fanout-security-plan.md 1 — the same body fact fct-cvyxuuhn
16. apply-merged → fanout-security-plan.md 2 — the same body fact fct-6hwcr6nz

fanout-security-plan--cascade-g3.md
1. apply — check 1: rul-ipb3atbm · as: «`is_select`'s caller, execute.rs's crate-private `target_of` alone (execute.rs:443), which `locate` ends in and `Session::actions` also calls» — main's measurement
2. reject — check 7: security-plan-newgroup-input-validation.md · twin → fanout-security-plan--cascade-g2.md 12

fanout-security-plan--cascade-g4.md
1. escalate → Q1 — check 7: security-plan-newgroup-data-protection.md · distinct; check 1: rul-izni57qv (a recorded `actions` answer at rest) · resolved: apply (Q1) · as: the body says `ratified`, bare — no founder words, name or date; drop «and v030-06 is verified only once it passes» (a ledger fact); keep that the replay of a kept `actions` step is exercised by no test yet, owed on «Handler coverage»
2. escalate → Q1 — check 1: rul-izni57qv (MCP tools fourteen, the `actions` tool) · resolved: apply (Q1)
3. apply — check 1: rul-ipb3atbm
4. apply — check 7: security-plan-newgroup-api-security.md · distinct; the limit and its owner (inputs#I5)
5. apply — check 1: rul-ipb3atbm (the security outcome)
6. apply — check 7: security-plan-newgroup-logging-monitoring.md · distinct

fanout-design-system.md
(@none — no proposal)

fanout-design-system--cascade.md
1. escalate → Q1 — check 7: design-system-newgroup-surface.md · distinct; check 1: rul-izni57qv (the actions answer line on the cli surface) · resolved: apply (Q1)
2. apply — check 7: design-system-newgroup-surface.md · distinct
3. apply — check 7: design-system-newgroup-surface.md · distinct; a `pending` group with its owner `res-7cc5bi4k` (inputs#I5)

fanout-layout-templates.md
1. escalate → Q1 — check 1: rul-izni57qv (the cli screen's actions answer) · resolved: apply (Q1)
2. apply — check 1: rul-ipb3atbm (the MCP listing fourteen) — dependent of 1, applied with it

fanout-test-plan.md
(@none — no proposal)

fanout-test-plan--cascade-g1.md
1. apply — check 1: rul-ipb3atbm (Counts [needles])
2. apply — check 1: rul-ipb3atbm (Counts [cli-tests]); check 5: citation rows fct-k5y3m6cu `claim false`
3. apply — check 1: rul-ipb3atbm (Counts [mcps-tests]); check 5: citation rows fct-k5y3m6cu `claim false`
4. apply — check 1: rul-ipb3atbm (Counts [unit-counts])
5. apply — check 1: rul-ipb3atbm; check 5: citation row fct-nks3g3e7 (schema_tests) `claim false`
6. apply — check 1: rul-ipb3atbm; check 5: citation row fct-nks3g3e7 (mcp_tests) `claim false`
7. apply — check 1: rul-ipb3atbm; check 5: citation row fct-nks3g3e7 (describe) `claim false`
8. apply — check 1: rul-ipb3atbm (Coverage of new surfaces)
9. apply — check 1: rul-ipb3atbm (Counts [unit-counts])
10. apply — check 1: rul-ipb3atbm; check 5: citation row fct-i5a6vv3t `claim false`
11. apply — check 1: rul-ipb3atbm
12. apply — check 5: citation row fct-nks3g3e7 (cli_tests) `claim false` (raised by main) · §1 Test Scope Summary (test-scope-summary, fct-nks3g3e7) · change: `cli_tests.rs`' line-reading rows read 102, `actions`'s among them (101 until this chunk), measured at `packages/escher-driver/src/cli_tests.rs:438`

fanout-test-plan--cascade-g2.md
1. apply — check 1: rul-ipb3atbm (Counts [smt-tests])
2. apply — check 1: rul-ipb3atbm (Counts [span-calls])
3. apply — check 1: rul-ipb3atbm (Changes, Outcome)

fanout-test-plan--cascade-g3.md
1. apply — check 1: rul-ipb3atbm (Deviations, the `can_paint_text` order)
2. apply — check 1: rul-ipb3atbm (Counts [sc-readers], [fx-readers]) · as: also `tests/common/mod.rs` read through `mod common;` by 23 `stand_*.rs` checks (22 until this chunk), main's measurement
3. apply — check 1: rul-ipb3atbm; check 5: citation row fct-klxgwgcy `claim false`
4. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
5. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
6. apply — check 1: rul-ipb3atbm; check 5: citation row fct-fghufsse `claim false`
7. apply — check 1: rul-ipb3atbm (Counts [renamed-tests])
8. apply — check 1: rul-ipb3atbm
9. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
10. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
11. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
12. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct; the CARRY on «Handler coverage» the operator's word (inputs#I6) · as: drop «and v030-06 reads verified only once that test passes» (a ledger fact); keep the owed test and its CARRY on «Handler coverage»
13. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
14. apply — check 1: rul-ipb3atbm (Deviations, the shared helper)
15. apply — check 1: rul-ipb3atbm (Counts [host-calls], [needles])
16. apply — check 1: rul-ipb3atbm

fanout-test-plan--cascade-g4.md
1. apply — check 1: rul-ipb3atbm; check 5: citation row fct-csryo2zo `claim false`
2. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
3. apply — check 1: rul-ipb3atbm; check 5: citation rows fct-gtzxj2kr `claim false` · as: `stand_mcp_tools` reads 8 (7 until this chunk, the report's Counts [smt-tests])
4. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
5. apply — check 1: rul-ipb3atbm (Decisions, the load page)
6. apply — check 7: test-plan-newgroup-test-data-fixtures.md · distinct
7. apply — check 1: rul-ipb3atbm (Counts [stand-tally], Harness / gate surface)
8. apply — check 1: rul-ipb3atbm (Outcome basis, 223 result lines)

fanout-test-plan--cascade-g5.md
1. apply — check 1: rul-ipb3atbm (Outcome, legs)
2. apply — check 1: rul-ipb3atbm (Counts [stand-tally])
3. apply — check 1: rul-ipb3atbm (Counts [renamed-tests])

fanout-test-plan--cascade-new-unit-test-strategy.md
1. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct

fanout-obs-plan.md
(@none — no proposal)

fanout-obs-plan--cascade.md
1. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
2. apply — check 1: rul-ipb3atbm (Coverage of new surfaces)
3. apply — check 1: rul-ipb3atbm (VERBS 13 → 14; the security outcome)
4. apply — check 1: rul-ipb3atbm (Counts [cold-tools])
5. apply — check 1: rul-ipb3atbm (Counts [instance]; the obs outcome)
6. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
7. apply — check 1: rul-ipb3atbm (Counts [needles])
8. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
9. apply — check 1: rul-ipb3atbm (the obs outcome)
10. escalate → Q1 — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct; check 1: rul-izni57qv (what the actions answer carries, kept whole in a recorded check) · resolved: apply (Q1)
11. escalate → Q1 — check 1: rul-izni57qv (an `actions` answer at rest in a recording directory) · resolved: apply (Q1)

fanout-a11y-plan.md
1. apply — check 1: the Retired bullet's item (`D-retired`), routine

fanout-a11y-plan--cascade-g1.md
1. apply — check 1: rul-ipb3atbm

fanout-a11y-plan--cascade-g2.md
1. apply — check 1: the Retired bullet's item (`D-retired`), routine; check 5: citation row fct-pfylw46r `claim false`
2. apply-merged → fanout-a11y-plan.md 1 — the same body fact fct-77ewqepn
3. apply — check 1: rul-ipb3atbm
4. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
5. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · rewritten by fanout-a11y-plan--cascade-g2.md 1
6. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct; the PROVISIONAL mark kept
7. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
8. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
9. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
10. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · rewritten by fanout-a11y-plan--cascade-g2.md 1 · as: «an inert or hidden element is not listed by `actionable_nodes`», whose domain is the UI graph's nodes; the hidden control `unkeyed_actionable` still holds stays as fanout-a11y-plan--cascade-g2.md 1 rewrites it
11. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · rewritten by fanout-a11y-plan--cascade-g2.md 1
12. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct

## Checks 3, 5, 6 — the chunk-level readings
- check 3 (intent-consistency): the scope record's one line (companion `stand_requirement_sweep.rs`, serving `schema_tests.rs`) serves its step; inputs#I6 (the replay test owed, a CARRY on «Handler coverage») narrows the report's routed criterion and is the operator's word; no line escalates.
- check 5 (expected amendments): every plan entry has proposals — Driver session (architecture 1, g3, g4 1-2), Established Decisions (Q1), Dioxus DOM bridge (architecture 2, g4 7-11), security §Input Validation / §Threat Model (Q1, security g2, g4), test-plan §1 §3 §4 §5 (test-plan g1-g5), obs §4 §6 §8 (obs cascade), a11y §7 (a11y 1, g2), design-system §Surface: cli (Q1, design-system 2-3), layout-templates (Q1, layout 2). The citation sweep's 22 `claim false` rows are matched by proposals but for fct-t7zq5feb@c23276 (the snapshot help quote) and fct-nks3g3e7's cli_tests row count, raised as architecture g3 19 and test-plan g1 12; the common declarer count the report omits is raised as architecture g7 9 with directions on architecture g7 8 and test-plan g3 2.
- check 6 (disproved claims): none in the report.

Rejected for a source the report does not carry: 0 (the facts the report does not state were measured by main and given as directions or raised items).
Rejected otherwise: 5 (check 7 twins and one restatement: architecture g4 3-6, security g3 2).
