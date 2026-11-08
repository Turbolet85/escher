# Fan-out results — 2026-11-07-ui-graph (wrap 2026-11-08T02-04-13, resumed)

Batches: cites 12/12 · stage1 13/13 (doc 7 · task 6) · classify 158 (156 pass, 2 re-spawned once in classify-r1, 2/2
pass, no dead draw) · strong 20/20 · newdetail 3/3 · newgroup 18/18. Sampler: off (`cascade-sample.txt`, not forced).
Doc agents' own proposals (`check`'s `proposals:` line): architecture 3 · security-plan 0 · design-system 0 ·
layout-templates 2 · test-plan 13 · obs-plan 1 · a11y-plan 1.

## Verdict lines
- architecture: 3 proposals · fanout-architecture.md · 64 cascade proposals · fanout-architecture--cascade-g1.md ·
  fanout-architecture--cascade-g2.md · fanout-architecture--cascade-g3.md · fanout-architecture--cascade-g4.md ·
  fanout-architecture--cascade-g5.md · fanout-architecture--cascade-g6.md · 10 facts · 10 placed · 0 no home
- security-plan: 0 proposals · fanout-security-plan.md · 31 cascade proposals · fanout-security-plan--cascade-g1.md ·
  fanout-security-plan--cascade-g2.md · fanout-security-plan--cascade-g3.md · fanout-security-plan--cascade-g4.md ·
  9 facts · 9 placed · 0 no home
- design-system: 0 proposals · fanout-design-system.md · 2 cascade proposals · fanout-design-system--cascade.md ·
  fanout-design-system--cascade-new-surface.md · 3 facts · 2 placed · 1 no home
- layout-templates: 2 proposals · fanout-layout-templates.md (whole — no category file)
- test-plan: 13 proposals · fanout-test-plan.md · 44 cascade proposals · fanout-test-plan--cascade-g1.md ·
  fanout-test-plan--cascade-g2.md · fanout-test-plan--cascade-g3.md · fanout-test-plan--cascade-g4.md ·
  fanout-test-plan--cascade-g5.md · fanout-test-plan--cascade-new-unit-test-strategy.md ·
  fanout-test-plan--cascade-new-test-scope-summary.md · 13 facts · 13 placed · 0 no home
- obs-plan: 1 proposal · fanout-obs-plan.md · 12 cascade proposals · fanout-obs-plan--cascade-g1.md ·
  fanout-obs-plan--cascade-g2.md · 7 facts · 7 placed · 0 no home
- a11y-plan: 1 proposal · fanout-a11y-plan.md · 14 cascade proposals · fanout-a11y-plan--cascade-g1.md ·
  fanout-a11y-plan--cascade-g2.md · 6 facts · 6 placed · 0 no home

Remarks carried: architecture's — the body fact fct-tev7ieys (no log or event carries the snapshot, its text or a diff)
does not name the graph; the obs outcome measures the graph logged nowhere (no proposal; the cascade sweep's row).
design-system F2 no home: the snapshot's `clickable` and the `CLICKABLE` table belong to a11y-plan §7 and architecture's
Dioxus DOM bridge (design-system-newdetail-surface.md).

Main's measurements this Validate (read on the work tree, each against `be4180b8`): `mod common;` declared by 22
blitz-tests files (21 at base) and `mod session_common;` by 33 (32), `stand_graph.rs` declaring both (lines 23-24);
159 `.rs` files directly under `tests/blitz-tests/tests/` (158); `#[test]` in `examples/seven_guis/tests/cli_commands.rs`
17 (16), its last two a per-OS pair, so 16 per platform (15); `mcp_server.rs` 7 (6); `stand_mcp_tools.rs` 7 (6);
`packages/escher-driver/src/cli_tests.rs:437` `assert_eq!(rows.len(), 101)` (100); `Command` enumerates its variants,
no count.

## Dispositions

fanout-architecture.md
1. apply — check 1: rul-ipb3atbm, the report's Schema / config (VERBS 12 → 13, `graph` the ninth instance verb) · as: the body line reads thirteen verbs, the rest of the claim unchanged; what `graph`'s answer carries is Q1's (fanout-architecture--cascade-g1.md 4)
2. apply — check 1: rul-ipb3atbm for the graph reader, the registry and its dispatch, rul-cepujoy6 for `clickable`; check 5: the plan's entry «→ Dioxus DOM bridge», the reserved form the founder's fork 2 (inputs#I4)
3. escalate → Q1 — check 1: rul-izni57qv, the UI graph's names joining what crosses (plan entry, inputs#I4, inputs#I5, inputs#I9) · resolved: apply (Q1)
4. apply — check 5: citation row fct-jzxi3spj `claim false` (raised by main) · §Occupied Resources (occupied-resources, fct-jzxi3spj) · change: the log checks' shared-module list adds `graph_needles` (each served component and registered state name, read by `host_log` and `mcp_log` over their one `graph` call); its citation's range re-read

fanout-architecture--cascade-g1.md
1. apply — check 1: rul-ipb3atbm (Symbols / APIs, the snapshot)
2. apply — check 1 + check 5: the plan's entry names «its dispatch»; the report reads it inert without a registry
3. apply — check 1: rul-ipb3atbm; the row's «not reachable without a fork-level change» is untrue for a registered state
4. escalate → Q1 — check 1: rul-izni57qv (Established Decisions → Driver session, the graph's names crossing) · resolved: apply (Q1)
5. apply-merged → fanout-architecture.md 3 — the same body fact fct-nb3g6bqn
6. escalate → Q1 — check 1: rul-izni57qv (a `graph` answer at rest in a kept check) · resolved: apply (Q1)
7. apply — check 1: rul-ipb3atbm (Crates / modules)
8. apply — check 1: rul-ipb3atbm
9. apply — check 7: architecture-newgroup-conventions.md · distinct; rul-ipb3atbm

fanout-architecture--cascade-g2.md
1. escalate → Q1 — check 1: rul-izni57qv (the validated verb set admits `graph`) · resolved: apply (Q1)
2. apply — check 1: rul-cepujoy6 (`clickable`, a closed bool inside the ratified carrier)
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm (describe's row) — dependent of 1, applied with it
5. apply — check 1: rul-ipb3atbm · as: `Command` gains `Graph`, validated from `("graph", [])`, after `Lint`; no variant count is written (the body enumerates the variants)
6. apply — check 1: rul-ipb3atbm
7. apply — check 1: rul-ipb3atbm
8. apply — check 1: rul-ipb3atbm
9. apply — check 1: rul-ipb3atbm (the MCP listing) — dependent of 1, applied with it
10. apply — check 1: rul-cepujoy6
11. apply — check 1: rul-fxlxu7mw (the helps, the program's own `'static` text)
12. escalate → Q1 — check 7: architecture-newgroup-standard-contracts.md · distinct; check 1: rul-izni57qv (the graph answer's written form) · resolved: apply (Q1)
13. apply — check 7: architecture-newgroup-standard-contracts.md · distinct; the limit and its owner `res-7cc5bi4k`, target next (inputs#I8, inputs#I9)
14. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
15. apply — check 7: architecture-newgroup-standard-contracts.md · distinct

fanout-architecture--cascade-g3.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. reject — check 7: architecture-newgroup-standard-contracts.md · contradiction · fct-t7zq5feb · rewritten by fanout-architecture--cascade-g2.md 1; the same F5 fact as fanout-architecture--cascade-g2.md 12, stated once
5. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g2.md 13
6. apply — check 1: rul-ipb3atbm (Symbols / APIs, the re-export through dioxus-native)
7. apply — check 1: rul-cepujoy6 · as: also that `differing` compares `clickable`, so a changed node's `differs` can name it (citation row fct-kllkxz3d)
8. apply — check 5: the plan's entry, the founder's fork 2 (inputs#I4)
9. apply — check 5: the plan's entry «its dispatch»
10. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
11. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
12. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
13. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g2.md 15
14. reject — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g2.md 14
15. apply — check 1: rul-ipb3atbm
16. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
17. apply — check 1: rul-ipb3atbm
18. apply — check 1: rul-ipb3atbm
19. apply-merged → fanout-architecture.md 1 — the same body fact fct-ksgwc2yv
20. apply-merged → fanout-architecture.md 2 — the same body fact fct-srb7slak

fanout-architecture--cascade-g4.md
1. apply — check 1: rul-ipb3atbm (Harness / gate surface)
2. apply — check 1: rul-ipb3atbm

fanout-architecture--cascade-g5.md
1. escalate → Q1 — check 1: rul-izni57qv (Occupied Resources, both carriers and a kept check take the graph out) · resolved: apply (Q1)
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm
5. apply — check 5: the plan's entry «its dispatch»
6. apply — check 7: architecture-newgroup-cross-cutting-patterns.md · distinct; check 6: the report's disproved re-subscribe claim, disposed here
7. apply — check 1: rul-ipb3atbm

fanout-architecture--cascade-g6.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm
5. apply — check 1: rul-ipb3atbm · as: the reader count 22 (21 until this chunk) is main's measurement (Main's measurements above)
6. apply — check 1: rul-ipb3atbm
7. apply — check 1: rul-ipb3atbm
8. apply — check 1: rul-ipb3atbm
9. apply — check 1: rul-ipb3atbm
10. apply — check 1: rul-ipb3atbm (pattern `cells-parts`)
11. apply — check 1: rul-ipb3atbm · as: 22 and 33 readers (21 and 32 until this chunk), main's measurement

fanout-security-plan.md
(@none — no proposal)

fanout-security-plan--cascade-g1.md
1. escalate → Q1 — check 7: security-plan-newgroup-threat-model-summary.md · distinct; check 1: rul-izni57qv · resolved: apply (Q1)
2. apply — check 1: rul-cepujoy6
3. escalate → Q1 — check 1: rul-izni57qv (the graph's carriage to the MCP client) · resolved: apply (Q1)
4. escalate → Q1 — check 1: rul-izni57qv (the carried-onward set gains the UI graph) · resolved: apply (Q1)
5. escalate → Q1 — check 1: rul-izni57qv (body socket line) · resolved: apply (Q1)
6. escalate → Q1 — check 1: rul-izni57qv (body MCP line) · resolved: apply (Q1)

fanout-security-plan--cascade-g2.md
1. apply — check 7: security-plan-newgroup-input-validation.md · contradiction · fct-tbmm2cwc · its own line rewritten (by 2's companion, this proposal)
2. apply — check 7: security-plan-newgroup-input-validation.md · contradiction · fct-tbmm2cwc · rewritten by fanout-security-plan--cascade-g2.md 1
3. escalate → Q1 — check 1: rul-izni57qv (the socket's carried set gains the UI graph) · resolved: apply (Q1)
4. apply — check 7: security-plan-newgroup-input-validation.md · contradiction · fct-7jixhrbw · rewritten by fanout-security-plan--cascade-g2.md 3; check 1: rul-cepujoy6
5. apply — check 1: rul-ipb3atbm
6. apply — check 1: rul-ipb3atbm
7. apply — check 1: rul-ipb3atbm · as: drop «its Debug printing the graph's ids and names» (the report states none)
8. escalate → Q1 — check 7: security-plan-newgroup-input-validation.md · contradiction · fct-awfxp663 · rewritten by fanout-security-plan--cascade-g2.md 7; check 1: rul-izni57qv (the verb's answer content) · resolved: apply (Q1)
9. escalate → Q1 — check 1: rul-izni57qv (the command line answers the graph) · resolved: apply (Q1)
10. apply — check 1: rul-ipb3atbm
11. apply — check 1: rul-ipb3atbm
12. apply — check 7: security-plan-newgroup-input-validation.md · distinct; check 5: fork 2 (inputs#I4)
13. escalate → Q1 — check 1: rul-izni57qv · resolved: apply (Q1) · as: «a further reader of the id» — no ordinal (the report counts none)
14. apply — check 1: rul-ipb3atbm
15. escalate → Q1 — check 1: rul-izni57qv (body socket broad line) · resolved: apply (Q1)

fanout-security-plan--cascade-g3.md
1. reject — check 7: security-plan-newgroup-input-validation.md · contradiction · fct-7jixhrbw · the same fact as fanout-security-plan--cascade-g2.md 4, stated once there
2. apply — check 7: security-plan-newgroup-input-validation.md · distinct; check 5: the plan's entry «its dispatch»
3. apply — check 7: security-plan-newgroup-input-validation.md · distinct
4. apply-merged → fanout-security-plan--cascade-g2.md 14 — the same body fact fct-cvyxuuhn

fanout-security-plan--cascade-g4.md
1. escalate → Q1 — check 1: rul-izni57qv (a recorded `graph` answer at rest) · resolved: apply (Q1)
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. apply — check 7: security-plan-newgroup-api-security.md · distinct; the limit and its owner (inputs#I8, inputs#I9)
5. apply — check 1: rul-fxlxu7mw
6. apply — check 7: security-plan-newgroup-logging-monitoring.md · distinct

fanout-design-system.md
(@none — no proposal)

fanout-design-system--cascade.md
1. escalate → Q1 — check 7: design-system-newgroup-surface.md · distinct; check 1: rul-izni57qv (the graph answer line on the cli surface) · resolved: apply (Q1)
2. apply — check 7: design-system-newgroup-surface.md · distinct; a `pending` group with its owner `res-7cc5bi4k` (inputs#I9)

fanout-design-system--cascade-new-surface.md
(@none — F2 no home)

fanout-layout-templates.md
1. escalate → Q1 — check 1: rul-izni57qv (the cli screen's graph answer) · resolved: apply (Q1)
2. apply — check 1: rul-ipb3atbm (the MCP listing thirteen) — dependent of 1, applied with it

fanout-test-plan.md
1. apply — check 1: rul-ipb3atbm (Harness / gate surface)
2. apply — check 1: rul-ipb3atbm — dependent of 1
3. apply — check 1: rul-ipb3atbm (`run-stand`, `stand-files`)
4. apply — check 1: rul-ipb3atbm; the tests ↔ obs bind (§3 ↔ obs §4 / §6)
5. apply — check 1: rul-ipb3atbm · as: the unit-scope list reads «the snapshot builder, its text form, its parts and its diff, the stable element id and its reserved virtual form, the actionable-key check, the bridge and its dispatch record, the UI graph reader, the signal registry, the mutation writer and the document's events», the rest of the line kept
6. apply — check 1: rul-ipb3atbm · as: the helper line's «no … `cfg(test)` shim» clause (fct-i47d76zr) is rewritten to name `DioxusDocument::dispatch_to_vdom`
7. apply — check 1: rul-ipb3atbm
8. apply — check 1: rul-ipb3atbm
9. apply — check 1: rul-ipb3atbm · as: one broad clause; the detail is fanout-test-plan--cascade-g4.md 6's group
10. apply — check 1: rul-ipb3atbm
11. apply — check 1: rul-ipb3atbm
12. apply — check 1: rul-ipb3atbm · as: and `cli_commands` 16 (15 until this chunk), main's measurement
13. escalate → Q1 — check 1: rul-izni57qv (session-wire, the graph's names cross the socket) · resolved: apply (Q1)
14. apply — check 5: citation row fct-nks3g3e7 `claim false` (raised by main) · §1 Test Scope Summary (test-scope-summary, fct-nks3g3e7) · change: `cli_tests.rs`' line-reading rows read 101 (100 until this chunk added `graph`'s row), measured at `packages/escher-driver/src/cli_tests.rs:437`; «64 rows» as the body states is stale
15. apply — check 5: citation rows fct-csryo2zo and fct-gtzxj2kr `claim false` (raised by main) · §4 / §6 (unit-test-strategy fct-klxgwgcy, e2e-test-strategy fct-csryo2zo and fct-gtzxj2kr, contracts/session-lifecycle fct-egnkfdyv) · change: `cli_commands` reads 16 tests (15 until this chunk), `mcp_server` 7 (6 until), main's measurement

fanout-test-plan--cascade-g1.md
1. apply — check 1: rul-ipb3atbm (16 tests, main's measurement agrees)
2. apply — check 1: rul-ipb3atbm (7 tests, main's measurement agrees)
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm (`describe-row`)
5. apply — check 1: rul-ipb3atbm
6. apply — check 1: rul-ipb3atbm
7. apply — check 1: rul-ipb3atbm (`dnd-units` 83 → 100)
8. apply — check 1: rul-ipb3atbm
9. apply — check 1: rul-ipb3atbm
10. apply — check 1: rul-ipb3atbm
11. apply-merged → fanout-test-plan.md 5 — the same body fact fct-jydhmppz
12. apply — check 1: rul-ipb3atbm

fanout-test-plan--cascade-g2.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct
3. apply — check 1: rul-ipb3atbm (7, main's measurement agrees)
4. apply — check 1: rul-ipb3atbm
5. apply — check 1: rul-ipb3atbm (`cells-parts`)
6. apply-merged → fanout-test-plan.md 5 — the same body fact fct-jydhmppz

fanout-test-plan--cascade-g3.md
(@none — no proposal)

fanout-test-plan--cascade-g4.md
1. apply — check 1: rul-ipb3atbm (159 files, 158 targets, 33 and 22 readers — main's measurement agrees)
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm
5. apply — check 1: rul-ipb3atbm
6. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
7. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
8. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
9. apply — check 1: rul-ipb3atbm (one `graph` call each, one command line per call)
10. apply — check 1: rul-ipb3atbm

fanout-test-plan--cascade-g5.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
3. apply — check 1: rul-ipb3atbm · as: and `mcp_server` 7 tests (6 until this chunk)
4. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct
5. apply — check 1: rul-ipb3atbm
6. apply — check 1: rul-ipb3atbm
7. apply — check 7: test-plan-newgroup-test-data-fixtures.md · distinct; check 6: the disproved size forecast, its measured figure placed here
8. apply — check 7: test-plan-newgroup-test-data-fixtures.md · distinct
9. apply — check 1: rul-ipb3atbm (`stand-files`)
10. apply — check 1: rul-ipb3atbm

fanout-test-plan--cascade-new-unit-test-strategy.md
1. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct; check 6: the gone-node resolution, disposed here
2. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
3. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
4. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct
5. apply — check 7: test-plan-newgroup-unit-test-strategy.md · distinct

fanout-test-plan--cascade-new-test-scope-summary.md
1. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct · as: no provenance in the body — drop «owner: the operator's word at P5 (inputs#I6)»; the limit is stated as a stated limit, the word goes to the sidecar

fanout-obs-plan.md
1. apply — check 1: rul-ipb3atbm (the answer never logged; the obs outcome)

fanout-obs-plan--cascade-g1.md
1. apply — check 7: obs-plan-newgroup-span-trace-coverage.md · distinct
2. apply — check 1: rul-ipb3atbm
3. apply — check 1: rul-ipb3atbm
4. apply — check 1: rul-ipb3atbm
5. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
6. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct
7. apply — check 7: obs-plan-newgroup-log-coverage.md · distinct

fanout-obs-plan--cascade-g2.md
1. apply — check 1: rul-ipb3atbm
2. escalate → Q1 — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct; check 1: rul-izni57qv (what the graph answer carries) · resolved: apply (Q1)
3. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct; rul-cepujoy6
4. apply — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct
5. escalate → Q1 — check 1: rul-izni57qv (a `graph` answer at rest in a recording directory) · resolved: apply (Q1)

fanout-a11y-plan.md
1. apply — check 1: the Retired bullet's item (`D-retired`), routine

fanout-a11y-plan--cascade-g1.md
1. apply — check 1: rul-ipb3atbm
2. apply — check 7: a11y-plan-newgroup-keyboard-navigation.md · distinct

fanout-a11y-plan--cascade-g2.md
1. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
2. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · rewritten by fanout-a11y-plan--cascade-g2.md 5
3. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
4. apply — check 1: rul-ipb3atbm
5. apply — check 1: rul-ipb3atbm (`click-field`; residual `res-gvelrxja`'s disposition met)
6. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
7. apply — check 1: rul-ipb3atbm
8. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · rewritten by fanout-a11y-plan--cascade-g2.md 5
9. apply — check 1: rul-ipb3atbm
10. apply — check 1: rul-ipb3atbm
11. apply-merged → fanout-a11y-plan.md 1 — the same body fact fct-77ewqepn
12. apply — check 1: rul-ipb3atbm

## Checks 3, 5, 6 — the chunk-level readings
- check 3 (intent-consistency): the scope record's five lines (in-intent 1, companion 4, widening 0) each serve their step; the operator's fixes inputs#I7 and inputs#I8 are justified divergences on his word; no line escalates.
- check 5 (expected amendments): every plan entry has proposals — Driver session (architecture g2, g3), Dioxus DOM bridge (architecture 2, g3 7-16), the widening (Q1), the reserved form (architecture g3 8, security g2 12), obs §4 / §6 (obs g1), test-plan counts (test-plan, g1-g5), a11y §7 (a11y 1, g2), design-system / layout-templates cli (Q1, layout 2, design-system 2). The citation sweep's 32 `claim false` rows are matched by proposals but for fct-jzxi3spj and fct-nks3g3e7's rows count, raised as architecture 4 and test-plan 14, and the `cli_commands` / `mcp_server` counts, raised as test-plan 15.
- check 6 (disproved claims): the size forecast → test-plan g5 7 and architecture g2 13 (the measured figure); no re-subscribe → architecture g5 6; the resolution rule → test-plan new-unit-test-strategy 1 and architecture g3 11; the confirmed Cells prediction → the `cells-parts` proposals.

Rejected for a source the report does not carry: 0 (the counts the report does not state were measured by main and given as directions).
