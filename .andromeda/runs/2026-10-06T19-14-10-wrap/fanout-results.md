# Fan-out results — 2026-10-06-snapshot-model

Seven doc-agents, one batch, the prompt of `amendment-flow.md` sent verbatim (15 detectors over 7 docs: architecture 2 ·
security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2). Entity probe on
every return: 0 HTML entities (`&lt;` `&gt;` `&amp;`), no decode needed.

## Verdict lines
- **architecture** — 6 proposals (1 primary + 5 `dependent-of`). Stripped: trailing notes on D-arch-decisions (no drift) and four sites read and left alone (architecture.md:9 · :105 · :120 · :207 · :259 · the two `454` line citations).
- **security-plan** — 2 proposals (1 primary + 1 `dependent-of`). Stripped: trailing notes on D-security-auth and D-security-deps (no drift), and a sweep note (0 `lib.rs` citations, no test count stated in the doc).
- **design-system** — `proposals: []`. Stripped: a trailing note (tokens n/a; 0 `dioxus-native-dom/src/lib.rs` citations). Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: a trailing note (no surface added). Raw twin: `.raw-fanout-layout-templates.md`.
- **test-plan** — 5 proposals (4 primary-class + 1 `dependent-of`). Stripped: a leading verdict block (D-tests-framework and D-tests-obs-harness no drift; a not-proposed note on the unit-test fixture's `resolve(0.0)`; a staleness note on test-plan.md:23-24).
- **obs-plan** — `proposals: []`. Stripped: a trailing note (no instrumentation requirement unmet, no logger, no PII; 0 `lib.rs` citations). Raw twin: `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripped: a trailing note, which names the plan's expected §7 amendment as outside both detectors. Raw twin: `.raw-fanout-a11y-plan.md`.

## The re-derivation tell (amendment-flow §Validate, the pre-check)
A proposal whose `change` or `basis` cites a source location the report does not carry is rejected before the six
checks, and its fact enters through the orchestrator. The report carries `lib.rs` line numbers (the line map) and no
line of `snapshot.rs`, `stand_snapshot.rs` or `element_id.rs`. Proposals marked **tell** below cite such lines. Each
tell's fact is real and this chunk's, so each is re-raised by the orchestrator (check 5 for a plan-listed entry,
cascade step 2 for a duplicate site) and applied from the orchestrator's own read of the source.

## architecture — parsed list and dispositions
1. D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge · register the snapshot model (`DioxusDocument::snapshot`, `Snapshot`, `SnapshotNode`, `NodeState`, their field sources, membership rule, no id-bearing field, no wire form) after the accessibility-tree-identity clause.
   - **tell** (cites `snapshot.rs:14-53`, `:55-90`, `stand_snapshot.rs:1-5`) → rejected as a proposal; **re-raised by the orchestrator, check 5** (expected amendment 1) → playbook *Accurate this-chunk addition* → routine → **apply**.
2. D-arch-resources · dependent · §Standard Contracts → Dioxus DOM bridge · the feature clause "gets the override only by naming the feature" also covers the snapshot model.
   - rejected with its primary; **re-raised, cascade step 2** (a duplicate site of the registered claim) → routine → **apply**.
3. D-arch-resources · dependent · §Existing Scopes → dioxus-native-dom · the row lists the `snapshot` module; citation `lib.rs:12-16` → `13-19`.
   - rejected with its primary; **re-raised, check 5** (expected amendments 1 and 5) → routine → **apply**.
4. D-arch-resources · dependent · §Cross-cutting Patterns → Config management · "gates its `accessibility_tree` override" also names the `snapshot` module.
   - rejected with its primary; **re-raised, cascade step 2** → routine → **apply**.
5. D-arch-resources · dependent · §Conventions → Feature gating · citation `lib.rs:5-10` → `5-11`.
   - rejected with its primary; **re-raised, check 5** (expected amendment 5) → routine → **apply**.
6. D-arch-resources · dependent · §Conventions → Feature gating · citations `lib.rs:33-56` → `38-61`, `lib.rs:47-50` → `52-55`.
   - rejected with its primary; **re-raised, check 5** (expected amendment 5) → routine → **apply**.
- Left alone, read: architecture.md:9 `lib.rs:3` and :105 `lib.rs:1` (unchanged lines) · :120 (still true) · :207 (describes README.md, untouched, still reading "snapshot … planned") · :259 (the blitz-tests row is not an exhaustive file list) — **no change**.

## security-plan — parsed list and dispositions
1. D-security-input · lowered to warning by the detector · §Input Validation → `id` (stable element id) row · record `DioxusDocument::snapshot` as a second, in-process reader of `author_id` with no crossing; every existing clause kept.
   - **tell** (cites `snapshot.rs:76`, `:117-130`, a census grep of the source) → rejected as a proposal; **re-raised by the orchestrator, check 5** (expected amendment 2). Playbook: *Boundary widening* read by its subject — no crossing is added (no wire form, no log, event, socket or file; both probes read 0), no input class is admitted (the method takes no argument), so the rule's subject is absent → *Accurate this-chunk addition* → routine → **apply**. The plan's lean ("not a boundary widening", an in-process reader of already-admitted data) was approved at the plan's P5 review.
2. D-security-input · dependent · §Input Validation → `aria-label` · `<label for>` (accessible names) row · record the snapshot as an in-process reader of the same names.
   - rejected with its primary; **re-raised, cascade step 2** (the sibling site of the same mechanism) → routine → **apply**, names only. The detector's side note on `NodeState.value` (a text control's text, in-process) has no row here and retires no claim: routed to P5 as a `CARRY` on "Compact snapshot serialization", the first wire form.

## test-plan — parsed list and dispositions
1. D-tests-coverage · warning · §1 → Coverage scope → tests/blitz-tests · the stand coverage sentence gains the snapshot checks and a `stand_snapshot.rs:1` citation.
   - **tell** (cites `stand_snapshot.rs:1-5`, `:156-410`) → rejected as a proposal; **re-raised, check 5** (expected amendment 3) → routine → **apply**.
2. D-tests-coverage · warning · §3 → Agent-run contract → Proof · chain link `run stand` 33 `ok` (+8 `stand_snapshot`).
   - rests on the report and the doc → playbook *Accurate this-chunk addition* → routine → **apply**.
3. D-tests-coverage · warning · §9 → Local baseline · chain link 125 result lines, 471 · 0 · 5.
   - rests on the report and the doc → routine → **apply**.
4. D-tests-coverage · warning · §1 → Coverage scope → dioxus-native-dom · "three unit tests" → 18 in four files.
   - **tell** (cites `grep -c '#[test]'` over the source, `snapshot.rs:195-347`, `element_id.rs:243-410`) → rejected as a proposal; **re-raised by the orchestrator** on its own measurement (`grep -c '#\[test\]' packages/dioxus-native-dom/src/*.rs`: dioxus_document 1 · events 2 · element_id 6 · snapshot 9 = 18, agreeing with the report's 9 → 18) → routine → **apply**. Six of the 18 predate this chunk (2026-10-06-stable-element-ids); the count moved here, so the line is brought to current truth here.
5. D-tests-coverage · dependent of 4 · §1 → Coverage scope → dioxus-native and stylo_taffy · the search parenthetical no longer says the matches sit in two files only.
   - rejected with its primary; **re-raised, cascade step 2** → routine → **apply**.
- Not proposed, raised by the orchestrator: §3 → Crate-local test helpers → dioxus-native-dom does not describe the snapshot unit tests' fixture (`resolve(0.0)` after `initial_build`). The report carries it under Deviations (justified). Check 3: a justified divergence → the body is brought to current truth → routine → **apply**.

## a11y-plan — orchestrator-raised (check 5, expected amendments 4 and 5)
- §7 Screen Reader Support → Accessibility tree output · the snapshot is a consumer of the tree's role, name and focus → routine → **apply**.
- §2 (a11y-plan.md:55) · "gates its own `accessibility_tree` override" also names the snapshot model; citation `lib.rs:7` → `7-8` → routine (cascade step 2, the same duplicate claim architecture.md:189 carried) → **apply**.
- §7 → Platform adapter (a11y-plan.md:280) · citation `lib.rs:7` → `7-8` → routine → **apply**.
- §8 (a11y-plan.md:313), raised from the cascade sweep's `stand-coverage` row · "no test asserts a Dioxus control's focusability or Tab order" is false: `stand_accessibility_ids.rs` already asserted the focus sequence, and this chunk's `state_reads_the_engine` asserts `focused` after one move. A single stale sentence in one master, contradicted by a line of the same master (269) and by this chunk's own test → routine → **apply** (the sentence now names both checks).

## Check summary
- Check 1 (playbook): every applied amendment matches *Accurate this-chunk addition*; no escalate verdict; no two-rule collision.
- Check 2 (cross-contradiction): none — no two proposals edit one section in opposing directions.
- Check 3 (intent-consistency): the report's four deviations are justified and stay inside the chunk's intent; the scope record holds no line.
- Check 4 (absence needs evidence): the "0 `lib.rs` citations in design-system and obs-plan" claim rests on the report's grep and was re-read independently by both detectors.
- Check 5 (expected amendments): entries 1–4 applied; entry 5 applied for architecture and a11y-plan, not carried for design-system and obs-plan (0 citations).
- Check 6 (disproved claims): the report lists none.
- Escalations: 0.
