# Fan-out results — 2026-10-07-change-tracking-and-diff

Seven Explore doc-agents, one batch, the prompt of `amendment-flow.md` sent verbatim with `{doc}` · `{doc_path}` · `{contracts_line}` · `{detectors_yaml}` · `{report_path}` substituted (the contracts line for test-plan, obs-plan and a11y-plan; dropped for architecture — `NOT MIGRATED` — and for the three docs with no keyed-contract section). Detector count: architecture 2 · security-plan 3 · design-system 1 · layout-templates 1 · test-plan 3 · obs-plan 3 · a11y-plan 2 = 15 = the `doc:` names over drift-base. Entity probe on the seven returns as received: 0 of `&lt;` `&gt;` `&amp;` (the returns carry `<label>`, `&self` and `->` raw).

## Verdict lines
- **architecture** — 8 proposals (D-arch-resources ×8, 3 of them `dependent-of`; D-arch-decisions: no drift). Stripped: three leading comment lines — no dependency or locked decision contradicted; no Occupied Resources entry needed; the pure line-map re-points are named as outside both invariants.
- **security-plan** — 6 proposals (D-security-input ×6, 4 of them `dependent-of`; D-security-deps and D-security-auth hold). Stripped: a leading comment block — the detector's escalate condition (an unvalidated external-input boundary) is not met, the six are additive and filed `warning`; 11 of its 17 citations move by the line map, not re-measured by it; the file-input half of the mask is by construction, not run.
- **design-system** — `proposals: []`. Stripped: two comment lines (no `hardcoded✗` flag; 7 `document.rs` citations move by the line map, outside its invariant) — twin `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: four comment lines (no surface or region added; its 3 `window.rs` citations do not move) — twin `.raw-fanout-layout-templates.md`.
- **test-plan** — 15 proposals (D-tests-coverage ×15, 12 of them `dependent-of`; D-tests-framework and D-tests-obs-harness: no drift). Stripped: a leading comment block — the coverage invariant itself holds; the one untested path (`View::poll`'s gated refresh) is `unrunnable-here` and owed on the route; it read each new line number in the source after computing it from the report's map.
- **obs-plan** — `proposals: []`. Stripped: a comment block (all three detectors hold; 14 of its 16 citations move, listed old → new) — twin `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripped: a comment block (no interactive product UI element, no schema change; the four a11y-plan edits the plan expects are named as outside both invariants) — twin `.raw-fanout-a11y-plan.md`.

## architecture — the parsed list, with dispositions

A1 · D-arch-resources · warning · §Standard Contracts → Document core (blitz-dom)
change: Add to the contract: `BaseDocument::has_changes(&self) -> bool` reads true while the changed set (`changed_nodes`, a `pub(crate)` `HashSet<NodeId>`) is non-empty — from the first tracked write until the next drain — and the new public `BaseDocument::take_changed_nodes(&mut self) -> HashSet<NodeId>` returns the set and leaves it empty; a returned id may name a node dropped since it was written, so it is read through `get_node`, never by indexing; `BaseDocument::create_node` no longer marks, so node creation puts nothing in the set; `has_changes` has no caller outside tests — the shell drains instead; PROVISIONAL, half (a) (packages/blitz-dom/src/document.rs:326-329; :856-862; :1005-1018).
**disposition: APPLY — check 1 routine (Accurate this-chunk addition; the plan's P5-approved expected amendment names the change), check 5 entry 1 matched. The applied text is re-derived from the report; its citations are the report's (`:329`, `:856-862`, `:1009`, `:1016`) — the proposal's `:1005-1018` is a range the report does not carry and is not used.**

A2 · D-arch-resources · warning · §Standard Contracts → Mutation, query and CSSOM
change: Replace "from `View::poll` on `has_changes()` and from `View::build_accessibility_tree` on `InitialTreeRequested`" with: from `View::poll` — on a poll that reported work it takes the changed set (`take_changed_nodes`, outside the `accessibility` cfg) and, under `accessibility`, calls `update_tree` when the taken set was non-empty — and from `View::build_accessibility_tree` on `InitialTreeRequested`; so the platform accessibility adapter receives the tree on change as well as on its initial request (the same crossing, the same content classes, more often); `has_changes()` is no longer what the poll reads, and before this chunk the poll-time refresh never ran; PROVISIONAL, half (b); no windowed witness on the dev host; the citation `window.rs:376-382` keeps its numbers.
**disposition: ESCALATE — check 1: the playbook's Boundary widening rule (never routine, whatever the plan or a direction says). Group W (A2 · A3 · S2 · S3 · the orchestrator's a11y-plan raise O1). Resolved below.**

A3 · D-arch-resources · warning · §Cross-cutting Patterns → Invalidation and state integrity · dependent-of D-arch-resources (A2)
change: Replace "the accessibility tree is rebuilt on poll when the document changes (packages/blitz-shell/src/window.rs:372-390)" with: under `accessibility` the shell rebuilds the platform accessibility tree on a poll that reported work when the changed set that poll drained was non-empty — not on every document change — and the set is drained on every such poll with or without the feature (`window.rs:372-390`) — PROVISIONAL, half (b).
**disposition: ESCALATE with A2 (group W) — a second statement of the same claim, found by the detector; the report's site list had it only as a moved-text citation.**

A4 · D-arch-resources · warning · §Cross-cutting Patterns → Invalidation and state integrity
change: Add what writes the changed set — (a) the 15 mutation-flag sites of `DocumentMutator` (the node itself at twelve sites, the parent in `remove_and_drop_all_children` and `add_children_to_parent`, the moved child in `add_children_to_parent`), each only when its own in-document condition holds; (b) `snapshot_node_and` when its `ElementState` argument intersects `FOCUS` or `CHECKED`; (c) `apply_generated_text_input_event`'s `Input` arm; (b) and (c) not limited to in-document nodes; NOT written by node creation, hover, active, scroll, a viewport resize, a resource load, an animation or layout; engine-touched, a superset of snapshot-changed; PROVISIONAL, half (a); re-point `document.rs:1529-1550` → `:1536-1561`.
**disposition: REJECTED as returned (its basis and change cite `packages/blitz-dom/src/document.rs:1553-1559` and `events/keyboard.rs:121`, locations the report does not carry — the re-derivation tell); its fact is raised by the orchestrator under check 5 entry 4 → APPLY, routine, re-derived from the report (the mark sites are named by function, cited by the ranges the report carries).**

A5 · D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge
change: Add a clause after the `Snapshot::to_text` clause registering `Snapshot::diff(&self, after: &Snapshot) -> SnapshotDiff`, `SnapshotDiff { added, removed, changed }` with `is_empty()`, `DiffNode { id, parent, role, name, state, bounds }` (the after-reading, flat), their derives and re-export; the added / removed / changed rules, the orders, the duplicate-id rule; values from the two snapshots only (`MASKED_VALUE`), no engine id or pointer; not derived from the engine's changed set; no text form, `Display`, JSON, parser, `DioxusDocument` entry point or `Document` trait method; returned value only, callers are tests, the crossing answer carried PROVISIONAL; equal across the two layout modes on the stand.
**disposition: REJECTED as returned (its change cites `snapshot_diff.rs:11-47`, a range the report does not carry); raised under check 5 entry 3 → APPLY, routine, re-derived (citations `snapshot_diff.rs:60`, `lib.rs:35-36`, `stand_diff.rs:1-10`).**

A6 · D-arch-resources · warning · §Existing Scopes → dioxus-native-dom
change: The module list gains, between `snapshot` and `snapshot_text`: snapshot_diff (crate-private, compiled only under `accessibility`: the snapshot's diff `Snapshot::diff`, its `SnapshotDiff` and `DiffNode` types re-exported); the row's citation `lib.rs:14-24` → `:15-27`.
**disposition: APPLY — check 1 routine (Accurate this-chunk addition), check 5 entry 3 matched.**

A7 · D-arch-resources · warning · §Cross-cutting Patterns → Config management · dependent-of D-arch-resources (A6)
change: "its `snapshot`, `snapshot_text` and `actionable` modules" becomes "its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules".
**disposition: APPLY with A6.**

A8 · D-arch-resources · warning · §Standard Contracts → Dioxus DOM bridge · dependent-of D-arch-resources (A6)
change: In the feature-forwarding sentence, "gets the override, the snapshot model, its text form and the actionable-key check only by naming the feature" gains "its diff".
**disposition: APPLY with A6 — the sentence read by offset on architecture.md:134 before the edit (check 4).**

## security-plan — the parsed list, with dispositions

S1 · D-security-input · warning · §Input Validation → `id` (stable element id)
change: After the `unkeyed_actionable` reader, add `Snapshot::diff` as a further in-process reader of the id through the snapshot model only — pure, no document access, no log; ids copied into `DiffNode.id`, `DiffNode.parent` and `SnapshotDiff.removed`; no engine id or pointer; no text form; returned value only (PROVISIONAL, as the snapshot text) — so the platform adapter still stays the id's only exit.
**disposition: REJECTED as returned (basis cites `snapshot_diff.rs:13`, `:34` and `lib.rs:36`, not carried by the report); raised under check 5 entry 5 → APPLY, routine, re-derived. No crossing is added, so the Boundary widening rule does not match this half.**

S2 · D-security-input · warning · §Input Validation → `id` (stable element id)
change: Where the row says the id "leaves the process only through the platform accessibility adapter when an assistive technology is active", add that the adapter now receives the tree on change as well as on its initial request — `View::poll` takes the changed set and, under `accessibility`, rebuilds the platform tree when the taken set was non-empty; same crossing, same content classes, more often; before this chunk the poll-time refresh never ran (PROVISIONAL: the operator's "refresh, then drain", 2026-10-07; no windowed witness).
**disposition: ESCALATE (group W).**

S3 · D-security-input · warning · §Input Validation → `aria-label` · `<label for>` (accessible names) · dependent-of D-security-input (S2)
change: Where the row says the names "reach the platform accessibility adapter like any text run", add that the adapter now receives them on change as well as on its initial request (PROVISIONAL, as the `id` row records; no windowed witness).
**disposition: ESCALATE with S2 (group W).**

S4 · D-security-input · warning · §Input Validation → accessible names · dependent-of D-security-input (S1)
change: After the `Snapshot::to_text` sentence, add that `Snapshot::diff` reads the same resolved names from the two snapshots into `DiffNode.name` — no new input class and no crossing (PROVISIONAL, as the `id` row records).
**disposition: REJECTED as returned (basis `snapshot_diff.rs:34-60`); raised under check 5 entry 5 → APPLY, routine, re-derived.**

S5 · D-security-input · warning · §Input Validation → password and file `input` value · dependent-of D-security-input (S1)
change: After the `Snapshot::to_text` sentence, add that `Snapshot::diff` takes its values from the two snapshots only, so a password's or file input's value in a diff entry is `MASKED_VALUE` — measured for a password input; the file-input case by the same construction, not separately run; no input class, no crossing (PROVISIONAL, as the `id` row records).
**disposition: REJECTED as returned (basis `stand_diff.rs:615-633`, `:605`); raised under check 5 entry 5 → APPLY, routine, re-derived — its hedge (password measured, file input by construction) is the report's own and is kept.**

S6 · D-security-input · warning · §Data Protection → Local user data handled · dependent-of D-security-input (S1)
change: Extend "so neither the snapshot nor its text (`Snapshot::to_text`) holds it" to also name a diff of two snapshots — by construction; this chunk ran no file-input diff.
**disposition: APPLY — check 1 routine (Accurate this-chunk addition; a fourth statement of the reader list, outside the section the plan named). The bullet read on its line before the edit.**

## test-plan — the parsed list, with dispositions

T1 · §1 Coverage scope → blitz-dom (test-plan.md:11) — 32 `#[test]` functions: 16 in document.rs, 12 in mutator.rs, 4 in net.rs; `document.rs:2765` → `:2776`, `mutator.rs:1352` → `:1397`. **APPLY — routine; the two citations by the line map.**
T2 · §1 → blitz-dom covered behaviours (:12) · dependent — `mutator.rs:1352-1799` → `:1397-1844`; `document.rs:2749-3308` → `:2760-3319`; add the changed-set clause citing `document.rs:3321-3552`. **APPLY — routine; the split form is taken over the report's hand-read union `:2760-3552` (the old list keeps citing only the modules that hold it).**
T3 · §1 → dioxus-native-dom (:23) — "49 unit tests in seven files"; append the eight `Snapshot::diff` tests, `snapshot_diff.rs:141-352`. **APPLY — routine.**
T4 · §1 → dioxus-native and stylo_taffy (:24) · dependent — the test-file list gains `snapshot_diff.rs`. **APPLY — routine; a site the report's searches did not list, read on its line before the edit.**
T5 · §1 → tests/blitz-tests (:25) — the stand narrative gains the diff clause; `stand_diff.rs:1` joins the citation list. **APPLY — routine.**
T6 · §3 Agent-run contract → Proof (:111) · dependent — append the re-count: `run stand` 63 `ok` (+9 `stand_diff`). **APPLY — routine.**
T7 · §9 Local baseline (:314) · dependent — append the re-count: 131 result lines, 542 · 0 · 5. **APPLY — routine.**
T8 · §2 blitz-dom pipeline tests (:44) — five citations re-pointed. T9 · §4 (:142) — one. T10 · §5 (:167) — two. T11 · §7 Style fixtures (:225) — two. T12 · §7 Viewports (:226) — two. T13 · §7 Font payload (:227) — one. T14 · §8 Shell fakes (:257) — two. T15 · §8 Real dependencies (:279) — one. **APPLY, all eight — subsumed by the orchestrator's line-map amendment O2; each proposed range equals the measured map's.**

## Raised by the orchestrator (check 5 — the plan's expected amendments no detector proposed)

O1 · a11y-plan §2 Accessibility tree lifecycle (a11y-plan.md:36) — the refresh fires on a poll that reported work when the changed set it took was non-empty, the set drained with or without the feature; before this chunk it never fired; PROVISIONAL; no windowed witness. **ESCALATE (group W).**
O2 · the measured line map — 119 of 181 citations re-pointed across architecture (50) · security-plan (11) · design-system (7) · test-plan (20) · obs-plan (14) · a11y-plan (17); layout-templates 0. **APPLY — the plan's expected amendment names the change; routine. Applied first, before any semantic amendment.**
O3 · a11y-plan §2 (a11y-plan.md:37) — the `changed_nodes` sentence restated to what the field's doc now says; `:326-327` → `:326-329`. **APPLY — routine (check 6: a disproved claim).**
O4 · a11y-plan §2 Feature exposure (:55) — the feature also gates the snapshot's diff; "all four of which its crate doc names". **APPLY — routine.**
O5 · a11y-plan §7 Accessibility tree output (:270) — the diff as one more reader of the snapshot model. **APPLY — routine.**

## The other checks
- **2 cross-contradiction:** none — A3 and A4 edit one entry in the same direction.
- **3 intent-consistency:** the report's twelve deviations are in-intent details or carry the operator's word (5, 6, 12); the scope record holds no line; no divergence from the route entry or the acceptance criteria.
- **4 absence needs evidence:** every "0 hits" a proposal states is the report's own search; this pass's caught-all claim is `cascade-dispositions.md`, written from the sweep.
- **5 expected amendments:** entries 1–8 of the plan's list are each matched above (A1 · A2+A3 · A5+A6+A7+A8 · A4 · S1–S5 · O1+O3+O4+O5 · T1–T7 · O2); the two route directions are P5's; the ledger line needs no write.
- **6 disproved claims:** the never-run refresh → group W; the `changed_nodes` sentence → O3; research's unmeasured idle pump and the plan's step-4 qualification → the report is their record (both artifacts are immutable), and A4's text states the idle-resolve fact; the no-namespace fixture convention → P3 curation, and its measured consequence → P5; CLAUDE.md and `services/blitz-dom.md` → the cascade's re-derivation.

## Escalation — group W (the shell's refresh on change)
Presented to the operator at this wrap (one question, three options: record as PROVISIONAL · ratify outright · stop the wrap). **Resolved: record as PROVISIONAL** — the operator, 2026-10-07, at this wrap's P2 escalation, in the session. A2 · A3 · S2 · S3 · O1 → APPLY: the five sites state the new behaviour, marked PROVISIONAL pending the founder at the Epoch 3 boundary (one item with the engine flag change), and say no windowed run witnesses it on the dev host. No playbook rule is proposed: Boundary widening is the never-routine class.
