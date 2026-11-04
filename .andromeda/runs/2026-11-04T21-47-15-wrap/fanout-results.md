# Fan-out results — 2026-11-04-dioxus-runtime-probe (wrap 2026-11-04T21-47-15, resumed)

Batches: cites 6/6 · stage1 64/64 · classify 158/158 (yes 52 of 1352, no dead draw) · strong 8/8 · newdetail 1/1 — every `fanout.py check` PASS, index and head unchanged. Sampler: forced by the operator (inputs#I5, inputs#I6).

architecture: 0 proposals · fanout-architecture.md · 13 cascade proposals · fanout-architecture--cascade-g1.md (12) · fanout-architecture--cascade-g2.md (1) · 8 facts · 8 placed · 0 no home · (sampler) 3 lost
security-plan: 0 proposals · fanout-security-plan.md · 10 cascade proposals · fanout-security-plan--cascade.md · 3 facts · 3 placed · 0 no home · (sampler) 0 lost
design-system: 0 proposals · fanout-design-system.md · 0 cascade proposals · 0 facts · 0 placed · 0 no home · (sampler) 0 lost
layout-templates: 0 proposals · fanout-layout-templates.md (whole, no cascade)
test-plan: 0 proposals · fanout-test-plan.md · 13 cascade proposals · fanout-test-plan--cascade-g1.md (3) · fanout-test-plan--cascade-g2.md (2) · fanout-test-plan--cascade-g3.md (6) · fanout-test-plan--cascade-new-integration-test-strategy.md (2) · 7 facts · 7 placed (2 by pass) · 0 no home · (sampler) 5 lost
obs-plan: 0 proposals · fanout-obs-plan.md · 1 cascade proposal · fanout-obs-plan--cascade.md · 1 fact · 1 placed · 0 no home · (sampler) 2 lost
a11y-plan: 0 proposals · fanout-a11y-plan.md · 17 cascade proposals · fanout-a11y-plan--cascade.md · 4 facts · 4 placed · 0 no home · (sampler) 1 lost

Remarks (substance):
- architecture g1: `.claude/rules/security.md:26` and `.claude/docs/security-summary.md:42` name three zip IDs and pulldown-cmark 0.13.5 (leaf rows); they and CLAUDE.md name `.andromeda/residuals.md`.
- security-plan: the report counts eight `deny.toml` ignores after RUSTSEC-2026-0429 joins; `deny.toml:21-29` holds nine (the fact's «Eight ignores stand» was eight before the add) — the orchestrator settles the count.
- test-plan g3: ci-integration fct-qjdo4c7k's per-OS tallies were not re-read from CI#41914044942; they stay unwritten until a reading exists.
- obs-plan: whether the silence row lands in log-coverage alone or in a §3 body block too is the orchestrator's.
- a11y-plan: the `.andromeda/residuals.md` path also stands in CLAUDE.md, `.claude/rules/security.md` and other masters.

## Dispositions (Validate)

Standing directions for every applied text below: the body takes no provenance — no founder's word, no `inputs#I…`, no «on the founder's word» (the sidecar entry carries it); a routing reads «routed to the route entry «MountedData buffered render»» (the entry P5 inserts on card row 1, inputs#I3). The `.andromeda/residuals.md` family (≈30 master sites, CLAUDE.md, `.claude/rules/security.md`, the docs leaves) is pre-existing since the bases door `031e983d9`, not this chunk's work: a fact this pass rewrites for this chunk's substance names its owner as «a `residual` row in the store» (fct-a4qh6ioi, fct-r6zswm2d); a proposal that only renames that path is rejected under rule rul-zokdndgi and the family is routed whole to a `CARRY:` at P5.

fanout-architecture.md
(no proposals)

fanout-architecture--cascade-g1.md
1. apply — check 1: Accurate this-chunk addition (rul-ipb3atbm); check 5: settles citation-dispositions row fct-kjoxjobc `Cargo.lock:6505` claim false · as: the rdme row reads «resolved 0.13.6, past RUSTSEC-2026-0414 and RUSTSEC-2026-0433», the rest standing; no «since …» clause
2. apply — check 5: expected amendment (architecture §Established Decisions); check 1 rul-ipb3atbm · as: the hazard row re-measured at HEAD (poll's borrow across `render_immediate`, `handle_task_wakeup`, `set_focus` through `doc_mut`, both layout modes), pinned by `dioxus_runtime_probe` (its pins stated in test-plan §6 E2E Test Strategy), its fix routed to the route entry «MountedData buffered render» (a port of upstream `286874cc1`'s buffered render onto the 0.7 bridge); no engine change until then; Cells still calls its handles synchronously; no provenance
3. reject — stated once: the pins' home is test-plan e2e-test-strategy (fanout-test-plan--cascade-g3.md 3); 2 names the pin · check 7: architecture-newgroup-established-decisions.md · distinct
4. apply — check 7: architecture-newgroup-established-decisions.md · distinct
5. apply — check 7: architecture-newgroup-established-decisions.md · distinct
6. reject — owed clause: the founder's three answers are «The UI graph»'s, carried by that route entry as a dated note at P5 on the operator's word (inputs#I6); the body holds current truth · check 7: architecture-newgroup-established-decisions.md · distinct
7. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
8. apply — check 7: architecture-newgroup-standard-contracts.md · distinct
9. apply — check 1 rul-ipb3atbm (report Counts, pattern listener)
10. reject — stated once: the audit's published-advisory limit lands in security-plan Dependency Security (fanout-security-plan--cascade.md 4)
11. apply — check 7: architecture-newgroup-project-intent.md · distinct · as: one clause after `286874cc1` among the left-out commits: its buffered render, the MountedData hazard's fix, is to be ported onto the Dioxus 0.7 bridge by the route entry «MountedData buffered render», while the 0.8 move itself still waits; the port's shape (its buffer, the two callers) rides that entry, not the body; no provenance
12. reject — rul-zokdndgi: a rename of the `.andromeda/residuals.md` path alone, the family routed whole to a CARRY at P5

fanout-architecture--cascade-g2.md
1. apply — check 7: architecture-newgroup-existing-scopes.md · distinct

(sampler) cascade-compare-architecture.md
S1. reject (sampler) — stated once: the over-report lands in standard-contracts (fanout-architecture--cascade-g1.md 7, 8) · check 7: architecture-newgroup-cross-cutting-patterns.md · distinct
S2. reject (sampler) — stated once: the hazard record is §Established Decisions' (fanout-architecture--cascade-g1.md 2) · check 7: architecture-newgroup-cross-cutting-patterns.md · distinct
S3. reject (sampler) — check 7: architecture-newgroup-standard-contracts.md · twin → fanout-architecture--cascade-g1.md 7

fanout-security-plan.md
(no proposals)

fanout-security-plan--cascade.md
1. apply — check 5: settles citation-dispositions rows fct-a4qh6ioi `deny.toml:1-29` (the count) and `.andromeda/residuals.md:29` · `:30` claim false; check 1 rul-ipb3atbm · as: «Nine ignores stand.» (`deny.toml:21-29` holds nine IDs; the report's «eight» is its own miscount); the zip passage names four IDs — RUSTSEC-2026-0367, 0368, 0369 and RUSTSEC-2026-0429 (`ZipArchive::extract` aborting the process on a symlink entry with a large declared size), each fixed only in zip 9.0 — `anyrender_serialize 0.8.0`'s `deserialize` opening a `ZipArchive` and reading entries `by_name`, never calling `extract` (deny.toml:23-26); 0429 an external decay too: with RUSTSEC-2026-0433 it turned the Dependency audit red (CI#41911351081) over a lock the green CI#41346357110 read, and the ignore with the pulldown-cmark update read green (CI#41914044942); the zip ignores owned by the store's `residual` row `res-h7acz56c`, the zbus-family ignores by their `residual` row, the paste ignore and the memmap2 limit by theirs — each «a `residual` row in the store (target: next)», the `.andromeda/residuals.md:29` and `:30` citations removed; standing text otherwise kept; no new founder's word
2. apply — check 1 rul-ipb3atbm · as: one sentence after RUSTSEC-2026-0414's: the lock resolves pulldown-cmark 0.13.6 by a `Cargo.lock`-only update, semver-compatible, no coupled pin, reached only through `rdme`, fixing RUSTSEC-2026-0433 (quadratic parsing of nested image wikilinks under `ENABLE_WIKILINKS`, which `rdme` never sets); `advisories ok` and `licenses ok` locally and in CI#41914044942 — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md
3. apply-merged → fanout-security-plan--cascade.md 1 — the paste, zbus and memmap2 owners in the same fact
4. apply — check 7: security-plan-newgroup-dependency-security.md · distinct · the home of the audit's published-advisory limit
5. apply-merged → fanout-security-plan--cascade.md 1 — check 7: security-plan-newgroup-dependency-security.md · contradiction · fct-a4qh6ioi · rewritten by fanout-security-plan--cascade.md 1
6. apply — check 5: expected amendment (security-plan §Error Handling); check 1 rul-ipb3atbm · as: as proposed, plus one clause: pinned by `dioxus_runtime_probe`'s two `should_panic` tests, one per layout mode, its fix routed to the route entry «MountedData buffered render»; no provenance
7. apply-merged → fanout-security-plan--cascade.md 6 — check 7: security-plan-newgroup-error-handling.md · distinct
8. reject — rul-zokdndgi: a rename of the `.andromeda/residuals.md` path alone, the family routed to a CARRY at P5
9. reject — rul-zokdndgi (dependent of 8)
10. reject — rul-zokdndgi (dependent of 8)

fanout-design-system.md
(no proposals)

fanout-layout-templates.md
(no proposals)

fanout-test-plan.md
1. apply — check 5: expected amendment (test-plan §3 Test Harness Contract, fct-glytb3w3, pattern run-all; no detector proposed it) · main raised · §3. Test Harness Contract · change: the Proof's newest reading — `run stand` passed 237 · failed 0 · ignored 7 over 51 files, unmoved, and `run all` passed 743 · failed 0 · ignored 12, `dioxus_runtime_probe`, with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md

fanout-test-plan--cascade-g1.md
1. apply — check 7: test-plan-newgroup-test-scope-summary.md · distinct
2. apply — check 1 rul-ipb3atbm (sweep: fct-e2zfwpza changed) · as: «the witness of the route entry «MountedData buffered render»», no provenance
3. reject — owed clause: the port's planned shape rides the route entry «MountedData buffered render» (P5), the body naming the witness (2) · check 7: test-plan-newgroup-test-scope-summary.md · distinct

fanout-test-plan--cascade-g2.md
1. reject — check 7: test-plan-newgroup-test-scope-summary.md · twin → fanout-test-plan--cascade-g1.md 3
2. reject — check 7: test-plan-newgroup-test-scope-summary.md · twin → fanout-test-plan--cascade-g1.md 1

fanout-test-plan--cascade-g3.md
1. apply — check 5: expected amendment (test-plan §2 Test Strategy, pattern targets)
2. apply — check 5: expected amendment (test-plan, pattern mounted) · as: no provenance; «routed to the route entry «MountedData buffered render»»
3. apply — check 7: test-plan-newgroup-e2e-test-strategy.md · distinct · the pins' one home
4. apply — check 7: test-plan-newgroup-test-data-fixtures.md · distinct
5. apply — check 7: test-plan-newgroup-ci-integration.md · distinct · as: the CI record alone (CI#41911351081 red on Dependency audit at `53ae48a65`, the two advisories, CI#41914044942 green 17/17 at `9b3b29714`), the audit's published-advisory limit and its local reproduction pointed to security-plan Dependency Security, not restated
6. reject — not a re-count of the fact's reading: fct-kjlei6et counts the `test` leg's result lines, the 221 counts `fast`'s across its legs — another figure, under no gate the fact names

fanout-test-plan--cascade-new-integration-test-strategy.md
1. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct
2. apply — check 7: test-plan-newgroup-integration-test-strategy.md · distinct

(sampler) cascade-compare-test-plan.md
S1. apply (sampler) — check 7: test-plan-newgroup-ci-integration.md · distinct · the file states each target's leg reach in this shape
S2. reject (sampler) — check 7: test-plan-newgroup-integration-test-strategy.md · twin → fanout-test-plan--cascade-new-integration-test-strategy.md 1
S3. reject (sampler) — check 7: test-plan-newgroup-test-data-fixtures.md · twin → fanout-test-plan--cascade-g3.md 4
S4. reject (sampler) — check 7: test-plan-newgroup-test-scope-summary.md · twin → fanout-test-plan--cascade-g1.md 1
S5. reject (sampler) — stated once: the pins stand in e2e-test-strategy (g3 3) and the target in test-scope-summary (g1 1) · check 7: test-plan-newgroup-test-strategy.md · distinct

fanout-obs-plan.md
(no proposals)

fanout-obs-plan--cascade.md
1. apply — check 5: expected amendment (obs-plan silence row); check 7: obs-plan-newgroup-log-coverage.md · distinct · log-coverage alone, as the analog fct-x7cwmno3

(sampler) cascade-compare-obs-plan.md
S1. reject (sampler) — check 7: obs-plan-newgroup-log-coverage.md · twin → fanout-obs-plan--cascade.md 1
S2. apply (sampler) — check 7: obs-plan-newgroup-pii-scrubbing-compliance.md · distinct

fanout-a11y-plan.md
(no proposals)

fanout-a11y-plan--cascade.md
1. apply — check 1 rul-ipb3atbm (sweep: fct-r6zswm2d changed) · as: the re-read on pulldown-cmark 0.13.6 as proposed; the adapter's owner «a `residual` row in the store», this fact being rewritten
2. reject — rul-zokdndgi: a path rename alone, the family routed to a CARRY at P5
3. reject — rul-zokdndgi
4. reject — rul-zokdndgi
5. reject — rul-zokdndgi
6. reject — rul-zokdndgi
7. reject — rul-zokdndgi
8. reject — rul-zokdndgi
9. apply — check 5: the MountedData record's a11y site (sweep: fct-gmhtpdel changed) · as: no provenance; «routed to the route entry «MountedData buffered render»»
10. reject — stated once: the pins' home is test-plan e2e-test-strategy (fanout-test-plan--cascade-g3.md 3), the target's legs test-plan's; 9 names the pin · check 7: a11y-plan-newgroup-keyboard-navigation.md · distinct
11. reject — owed clause: the founder's answers are «The UI graph»'s route note (P5, inputs#I6) · check 7: a11y-plan-newgroup-screen-reader-support.md · contradiction · fct-pfylw46r · moot, the proposal rejected; the standing line rewritten by fanout-a11y-plan--cascade.md 12
12. apply — check 1 rul-ipb3atbm · as: the listener-mark clause gains its measured limit (a mark standing on an element whose listener a re-render removed, so the check over-includes, conservative; a `mounted` listener writes none — read, not run); «that reading owned by a `.andromeda/residuals.md` line (target: next)» becomes «that reading measured by link 2, its snapshot field owed to the route entry «The UI graph»»; the founder's C3 words and dates standing as written
13. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
14. apply — check 7: a11y-plan-newgroup-screen-reader-support.md · distinct
15. apply — check 5: expected amendment (a11y-plan §7) · as: the body line's tail reads «no snapshot field says an element reacts to a click — that reading measured by link 2, its field owed to the route entry «The UI graph»; the listener mark over-includes an element whose listener was removed (conservative)»; no provenance, no residual clause
16. reject — rul-zokdndgi
17. reject — rul-zokdndgi

(sampler) cascade-compare-a11y-plan.md
S1. apply (sampler) — check 7: a11y-plan-newgroup-ci-integration.md · distinct

Checks 2–6: no cross-contradiction left (the two check-7 contradictions settled above); check 3 — the scope record's two `widening` lines (`Cargo.lock`, `deny.toml`) carry the operator's word (inputs#I4), the justified branch, and the report's UNMET criterion is settled by the operator's word at this resume (inputs#I6: the plan's-wording arm, P7.1); check 4 — no absence claim applied without its sweep; check 5 — every expected amendment and every `claim false` row dispositioned above; check 6 — no disproved claim. Escalations: none.
