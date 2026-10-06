# Fan-out results — 2026-10-06-id-persistence

The 7 doc-agents were sent in one batch. Prompts were built from `amendment-flow.md`'s template. The detector counts
sum to the drift-base `doc:` names (15 = 15).

## Verdicts
- architecture — 1 proposal (D-arch-resources); D-arch-decisions no drift. Stripped: a note that the Dioxus DOM bridge expected amendment sits outside both detectors and goes through the expected-amendments channel.
- security-plan — 1 proposal (D-security-input, severity escalate); D-security-auth and D-security-deps no drift.
- design-system — `proposals: []` (D-design-tokens: no new UI element, tokens n/a).
- layout-templates — 1 proposal (D-layout-surface).
- test-plan — 3 proposals (D-tests-coverage ×3); D-tests-framework and D-tests-obs-harness no drift.
- obs-plan — 1 proposal (D-obs-stack); D-obs-instrumentation and D-obs-pii no drift.
- a11y-plan — `proposals: []` (D-a11y-surface: no new interactive element; D-a11y-obs-schema: no schema change).

No return was HTML-escaped (no `&lt;` `&gt;` `&amp;` in any value). Entity probe: entities=0. No raw twin was warranted.

## Proposals and dispositions

### architecture
1. D-arch-resources · warning · §Occupied Resources → Process-wide state and threads. The change registers two re-exec spawns (`telemetry_stdout_silent`, `stand_id_persistence`).
   - **REJECTED** (re-derivation tell): its change cites `tests/blitz-tests/tests/telemetry_stdout_silent.rs:10-14` and `tests/blitz-tests/tests/stand_id_persistence.rs:275-283`, which the report does not carry.
   - Its fact is real and in the report (Symbols / APIs → process spawn; Expected amendments 2). It is **raised by the orchestrator as R2** below.

### security-plan
1. D-security-input · escalate · §Input Validation → Markup attributes | `id`. The change records that a keyed row id carries its Dioxus key and that the CRUD row key is a model-assigned `u64`.
   - **APPLY** (check 1). The P5-approved expected amendment names this change itself ("the CRUD row Dioxus key is a model-assigned `u64`, carrying no pointer or process-local value"). Playbook "Accurate this-chunk addition" also holds.
   - Not a boundary widening: no new input class crosses the `id` surface. The Dioxus key component `{tag}[{key}]` was already in the grammar (stable-element-ids). This chunk changes which model value the app feeds it.
   - The detector's escalate severity is its default; check 1 decides.

### layout-templates
1. D-layout-surface · warning · §Surface: desktop-native → Primary screens. The change: the CRUD row key `{i}` → `{person.id}`, and the citation `crud.rs:41-119` → `46-125`.
   - **APPLY** (check 1: expected amendment 3 names it; playbook "Accurate this-chunk addition"). The citation re-point is from the report's line map.

### test-plan
1. D-tests-coverage · §1 Test Scope Summary → tests/blitz-tests coverage — `stand_id_persistence` and its persistence facts. **APPLY** (expected amendment 4; "Accurate this-chunk addition").
2. D-tests-coverage · §3 Proof — `run stand` 20 ok, appended as a re-count. **APPLY** (expected amendment 4).
3. D-tests-coverage · §9 Local baseline — 444 · 0 · 5 / 122, appended as a re-count. **APPLY** (expected amendment 4).

### obs-plan
1. D-obs-stack · warning · §3 Logging stack, headless-stand bullet. The change scopes "no `println!` … in it or its checks" to allow the re-exec child's stdout ids.
   - **REJECTED** (re-derivation tell): its basis cites `tests/blitz-tests/tests/stand_id_persistence.rs:315,318`, which the report does not carry.
   - Its fact is in the report ("The child prints its pid and its ids per task and mode"; Schema / config: "The child's stdout carries ids only into the parent's assertion; nothing is logged"), and obs-plan.md:68 states "no `println!` … in it or its checks". It is **raised by the orchestrator as R3** below.

## Orchestrator raises (check 5 — expected amendments; check 6 — disproved claims)
- **R1** · architecture §Standard Contracts → Dioxus DOM bridge (line 134) — persistence proven on the stand (re-render, remount with fresh `NodeId`s bar the four-element document skeleton, a new process). The CRUD row key is the person id, placed on the `for` item so the list diffs by key. **APPLY**, routine: expected amendment 1, substantiated by the report (Symbols / APIs → persistence as proven, CRUD row id, keyed-list fact).
- **R2** · architecture §Occupied Resources → Process-wide state and threads (line 148) — register both test-binary re-exec spawns.
  - **APPLY.** Expected amendment 2 names the change. The report carries `telemetry_stdout_silent.rs:10-11`, the argv, the captured output, and that `stand_id_persistence` sets or removes no env var.
  - Boundary-widening subject: the operator's own ruling at the P5 review (2026-10-06) is "not a widening; register both re-exec spawns under arch §Occupied Resources → Process-wide state at the wrap" (plan.md §Provenance and Expected amendments). It is recorded in the sidecar.
- **R3** · obs-plan §3 Logging stack, headless-stand bullet (line 68) — "no `println!` … in it or its checks" is false for `stand_id_persistence`'s re-exec child. **APPLY**, routine (Accurate this-chunk addition: the named test is this chunk's; the invariant "a headless boot has no escher sink" still holds).

## Validate checks
1. Playbook — the applies above match "Accurate this-chunk addition" (and the expected-amendment direction for S1 / R1 / R2). No two rules collide.
2. Cross-contradiction — none: every proposal edits a distinct section.
3. Intent-consistency — the deviations (row loop restructured, entry 3 amended, remount scope, CRUD order) carry the operator's word or a measured justification (report §Deviations). The scope record is empty (`gate.py scope` clean). The remount-acceptance divergence is matrix-linked → P7.3 escalation (not P2).
4. Absence needs evidence — test-plan's sweep claims ('17 `ok`' only at :111, '441' only at :314) are re-checked by the cascade sweep below. layout's `{i}`-only-at-:10 sweep is re-checked the same way.
5. Expected amendments — all five entries are covered:
   - 1 → R1
   - 2 → R2
   - 3 → layout D-layout-surface
   - 4 → test-plan ×3
   - 5 → security D-security-input
6. Disproved claims —
   - 1 (plan's index-key note) is plan-only → DISPOSED: report + curation hazard; R1 states the keyed-list fact as current truth.
   - 2 (matrix v010-02 remount clause) → DISPOSED to P7.3: a post-claim-disproof escalation (refine with evidence or un-claim).
   - 3 (plan entry 3 whitespace) → DISPOSED: amended in plan.md on the operator's direction before this wrap.

Escalations at P2: 0. The detector-severity escalate on D-security-input was resolved by check 1 (expected-amendment direction; not a widening).
