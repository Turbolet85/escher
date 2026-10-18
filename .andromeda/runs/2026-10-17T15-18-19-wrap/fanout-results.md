# Fan-out results — 2026-10-17-upstream-sync-engine-debts

Batch: one snapshot pair, before `tree 1004cac1eb26389d88c3813ffe9a5e4607441005 index 4c1dd23ac2904a7c75e239cadf8c0d33e6d50f71 head ae8c8daf89778a43c8aa71ba55661cdc146adda4`, after `tree 9fb8d36296873c5940e1fa199601b75b707885dd`, index and head unchanged; `git diff-tree` printed exactly the seven `A` lines, one per agent. No file needed a fix (no text before `proposals:`, no entity), so no `.raw-` twin. Every receipt count equals its file's `- detector:` count. 0 proposals rejected for a source the report does not carry.

## Verdicts
- architecture: 23 proposals · fanout-architecture.md — note: util.rs:7-10 `SUPER` on macOS not proposed (raised below as architecture 24).
- security-plan: 8 proposals · fanout-security-plan.md — note: D-security-auth no drift; vibey-script bindings add no input boundary.
- design-system: 0 proposals · fanout-design-system.md — note: the bump §Surface: cli lines are the plan's expected amendment (raised below).
- layout-templates: 0 proposals · fanout-layout-templates.md — note: the bump §Surface: cli line is the plan's expected amendment (raised below).
- test-plan: 12 proposals · fanout-test-plan.md — note: D-tests-obs-harness holds.
- obs-plan: 0 proposals · fanout-obs-plan.md — note: §9's coverage-report row does not say the leg excludes `keyboard_compat.rs` (raised below).
- a11y-plan: 6 proposals · fanout-a11y-plan.md — note: D-a11y-obs-schema no drift.

## Dispositions (check that decided)

### architecture
1. §Occupied Resources → Names, bump leaves — apply (check 1: Accurate this-chunk addition; check 5: plan list)
2. §Stack → Language/runtime, bump leaves the edition-2024 list — apply (check 1, dependent of 1)
3. §Stack → Release tooling row leaves — apply (check 1; check 5)
4. §Established Decisions → [Release versioning], the bump clause — apply (check 1; citation rows `apps/bump/src/main.rs:5-23`, `:95-110` claim false, settled here)
5. §Standard Contracts → CLIs, bump's arguments — apply (check 1)
6. §Infrastructure Patterns → tree `bump/` — apply (check 1; check 5)
7. §Infrastructure Patterns → Deployment model, bump's toml_edit rewrite — apply (check 1)
8. §Project Intent → Apps and demonstrators, bump — apply (check 1)
9. §Project Intent → Growth model, the versioning citation — apply (check 1)
10. §Inherited Defaults → Publishability, bump — apply (check 1)
11. §Existing Scopes, the bump row — apply (check 1; check 5)
12. §Standard Contracts → Mutation, query and CSSOM, the six merged reads — escalate → the answer card (check 1: the description text and `aria-labelledby` names are the widening's subject)
13. §Standard Contracts → Mutation, query and CSSOM, the adapter's content classes gain description — escalate → the answer card (check 1: Boundary widening by upstream merge)
14. §Standard Contracts → Dioxus DOM bridge, the snapshot `name` gains the `aria-labelledby` source — escalate → the answer card (check 1: the name crossing)
15. §Standard Contracts → Test harness, the re-export gains `NamedKey` — apply (check 1; check 5)
16. §Established Decisions → [Driver session], the key types through the harness — apply (check 1)
17. §Existing Scopes → dioxus-native-dom, `keyboard_compat` — apply (check 1; the report's beyond-the-list entry)
18. §Standard Contracts → CI contracts, the coverage exclusion — apply (check 1; check 5)
19. §Stack → Windowing and input, keyboard-types "0.8" — apply (check 1; check 5)
20. §Stack → Parallelism and misc, test-that "0.6.0" — apply (check 1; check 5)
21. §Stack → Workspace package, 0.3.0-beta.4 — apply (check 1; check 5)
22. §Inherited Defaults → Framework, `=0.3.0-beta.4` — apply (check 1, dependent of 21)
23. §Project Intent → Upstream sync, `23b9b26a9` / `3eda07bc` — apply (check 1; check 5)
24. (raised by main, check 5 — citation row `architecture.md:87@c10896 util.rs:7-10` claim false) §Established Decisions → the interaction constants: the platform action modifier reads `META` on macOS, `CONTROL` elsewhere (util.rs:8 is the report's new text, upstream's keyboard-types 0.8 move) — apply (check 1: Accurate this-chunk addition)

### security-plan
1. §Input Validation → `aria-label` · `<label for>` row gains `aria-labelledby` — escalate → the answer card (check 1: inside the ratified name crossing on the founder's answer)
2. §Input Validation → new `aria-describedby` · `aria-description` row — escalate → the answer card (check 1: Boundary widening by upstream merge)
3. §Input Validation → `aria-expanded` · `aria-disabled` · `aria-level` closed values — apply (check 1: A field inside a ratified carrier — closed values on the ratified adapter crossing, no new carrier, verb, exit or log field)
4. §Input Validation → `id` row, the adapter's content classes gain descriptions — escalate → the answer card (dependent of 2)
5. §Input Validation → the `CLI arguments (bump)` row leaves — apply (check 1: a detector-severity escalate whose invariant holds, the plan's list naming it)
6. §Error Handling → bump's `bail!` — apply (as 5, dependent)
7. §Dependency Security → Audit tool, ignores 5 → 8 — apply (check 1: External decay, routine — ID-scoped reasoned ignores with an owner, `.andromeda/residuals.md:30`; check 3: the scope record's `deny.toml` widening on the operator's word)
8. §Dependency Security → Pinning, the 2026-10-17 sync record — apply (check 1: a detector-severity escalate whose invariant holds — no ban list — the plan's list naming it; its "`deny.toml` unchanged" superseded by 7)
9. (raised by main, check 5 — citation row `security-plan.md:371 convert_events.rs:398-399` claim false) §Error Handling: the KeyboardBacklightToggle / unlisted-code `todo!()` clause leaves — the physical-key converter now passes winit's `Code` through — apply (check 1: Accurate this-chunk addition)

### design-system
1. (raised by main, check 5) §Surface: cli → Platform: the `bump` app and its citation leave — apply (check 1)
2. (raised by main, check 5) §Surface: cli → the "bump prints …" pattern leaves — apply (check 1)

### layout-templates
1. (raised by main, check 5) §Surface: cli → the `bump` line leaves — apply (check 1)

### test-plan
1. §1 tests/blitz-tests, the two merged checks — apply (check 1; check 5)
2. §1 tests/blitz-tests, observed absent: a test of the merged `aria-expanded` · `aria-disabled` · `aria-level` reads — apply (check 4: main's own search, `aria-level|aria-expanded|aria-disabled` over `packages` and `tests` `.rs` files, hits only `packages/blitz-dom/src/accessibility.rs` itself, no test module there; the report's Coverage line «tests: upstream's (#1014; …)» is false on this point)
3. §10 Coverage floors, the exclusion — apply (check 1; check 5)
4. §9 Coverage, the two `cargo llvm-cov` lines — apply (dependent of 3)
5. §2 Directory pattern, 127 files / 126 targets and 137 executables — apply (check 6: the disproved `test-plan.md:66` count, settled here; the resume directive inputs#I4 (c))
6. §9 Per-OS test tallies — apply (check 1; check 5), the re-derived text holding only the report's figures
7. §9 tally history — apply (check 1; check 5)
8. §3 Agent-run contract → Proof — apply (check 1)
9. §1 apps, bump leaves — apply (check 1)
10. §3 Harness Core, the re-export gains `NamedKey` — apply (check 1; citation row `test-plan.md:88@c1977 lib.rs:32` claim false, settled here)
11. §3 Third-party test frameworks, test-that 0.6.0 — apply (check 1: the report's Dependencies bump and the two merged files' `verify_that!`)
12. §3 Rust tests, `verify_that!` beside `assert!` — apply (dependent of 11)

### obs-plan
1. (raised by main, check 5) §9 the Coverage report row: the leg's lcov excludes `keyboard_compat.rs` by filename — apply (check 1; the tests↔obs bind kept with test-plan 3)

### a11y-plan
1. §2 Role derivation, upstream's six reads — escalate → the answer card (records the description to the adapter and the `aria-labelledby` names)
2. §7 snapshot `name` gains the `aria-labelledby` source — escalate → the answer card
3. §7 the `aria-label` bullet: `aria-labelledby` outranks it — escalate → the answer card
4. §7 the `<label>` bullet: `aria-labelledby` names the input instead — escalate → the answer card
5. §7 the snapshot model: the tree's disabled from `aria-disabled` — apply (check 1: A field inside a ratified carrier)
6. §3 the two merged checks — apply (check 1)

## Check 3 — intent-consistency
- The scope record's one line, `deny.toml` · widening, carries the operator's word — justified.
- The (security) criterion «audit … with `deny.toml` unchanged» is UNMET as written: resolved on the operator's resume directive, inputs#I4 (a) — the letter unchanged, nothing reverted, resting on `scope-record.md` (the `deny.toml` widening, `ae8c8daf`) and the External decay cause. No amendment of intent.

## Check 6 — disproved claims
- `test-plan.md:66`: test-plan 5. The two plan figures (188 → 187; 9 → 10): disposed in the report's Deviations, no master states them.
- The citation sweep's 32 `claim false` rows: settled by architecture 1–11, 15, 17, 19–22, 24; security-plan 5, 6, 9; design-system 1–2; layout-templates 1; test-plan 10.

## The answer card's outcome
The ten card proposals (architecture 12, 13, 14 · security-plan 1, 2, 4 · a11y-plan 1, 2, 3, 4) — escalate → resolved: confirmed by the founder's «Подтвердить все 10 (Рекомендую)», relayed by the overseer (inputs#I7, `answer-card.md`); applied.

## Phase 2 summary
56 amendments applied (architecture 24 · security-plan 9 · design-system 2 · layout-templates 1 · test-plan 12 · obs-plan 1 · a11y-plan 6 — 49 proposed, 7 raised by main) in 15 sidecar entries · 1 escalation resolved (the answer card, its ten proposals) · 0 proposals rejected for a source the report does not carry · citations 266 re-pointed · 31 changed · 105 stretched.
