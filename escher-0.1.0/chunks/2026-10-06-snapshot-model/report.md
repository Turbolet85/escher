# Report — 2026-10-06-snapshot-model

**Chunk:** Snapshot model — screen as a tree of id, role, name, state, bounds (v010-04)
**Date:** 2026-10-06T19:17Z
**Commits:** `910d1237` chore(2026-10-06-snapshot-model): operator pre-CI commit, for the run this chunk's verdict reads (the only commit since `last_wrap` 2026-10-06T18:13:35Z / `ee88e85e`; basis `git log --format='%h %s' ee88e85e..HEAD`)

## Changes (structured — detectors read this)
- **Files** (basis `git diff --name-status ee88e85e` filtered to non-audit paths; `gate.py scope` at this wrap reads `clean — changed 3 · listed 3 · recorded 0`):
  - new: `packages/dioxus-native-dom/src/snapshot.rs` · `tests/blitz-tests/tests/stand_snapshot.rs`
  - modified: `packages/dioxus-native-dom/src/lib.rs`
  - chunk evidence: `escher-0.1.0/chunks/2026-10-06-snapshot-model/evidence/operator-pass.md`
  - untouched, asserted by the plan's `git diff --quiet ee88e85e… -- …` gate (green): every manifest, `Cargo.lock`, `deny.toml`, `packages/blitz-dom`, `packages/blitz-traits`, `packages/blitz-shell`, `packages/blitz-test-harness`, `packages/dioxus-native`, `packages/dioxus-native-dom/src/{dioxus_document,element_id}.rs`, `examples`, `tests/blitz-tests/Cargo.toml`, `scripts`, `.github`.
- **Symbols / APIs** (basis `packages/dioxus-native-dom/src/snapshot.rs@910d1237`; all new, all public, all compiled only under dioxus-native-dom's `accessibility` feature because `lib.rs` gates `mod snapshot` and its re-export with `#[cfg(feature = "accessibility")]`):
  - `DioxusDocument::snapshot(&self) -> Snapshot` — an inherent method, read-only (no mutation, no `DocumentMutator`). It builds from `Document::accessibility_tree` (the `DioxusDocument` override that carries `author_id`) and walks the tree's `children` lists from `TreeInfo.root`.
  - `Snapshot { pub roots: Vec<SnapshotNode> }` with `nodes()` (a pre-order iterator over every node) and `get(&str) -> Option<&SnapshotNode>` (lookup by stable element id).
  - `SnapshotNode { pub id: String, pub role: accesskit::Role, pub name: String, pub state: NodeState, pub bounds: blitz_dom::BoundingRect, pub children: Vec<SnapshotNode> }`.
  - `NodeState { pub enabled: Option<bool>, pub checked: Option<bool>, pub value: Option<String>, pub focused: bool }`.
  - Derives: `Debug`, `Clone`, `PartialEq` on all three; `NodeState` also `Default`.
  - Re-exports: `dioxus_native_dom::{NodeState, Snapshot, SnapshotNode}` (lib.rs:23-24), and through dioxus-native's existing `pub use dioxus_native_dom::*` (packages/dioxus-native/src/lib.rs:27, unchanged) when its `accessibility` feature is on.
  - Field sources, one existing reader each, none re-derived:
    - **membership** — a tree node carrying an `author_id` whose element resolves through `BaseDocument::get_node(NodeId::from_u64(tree id))` (an `Option`, never an index) and reads a rect. Every other node (the `Window`, the document root, `TextRun`s) is not a node; its kept descendants attach to the nearest kept ancestor. Generic containers are kept (the operator's P4 answer, "Every kept element"). What the accessibility tree excludes (`display:none`, `visibility:hidden`, their subtrees, `<head>` among them) never appears.
    - **id** — the node's `author_id`, verbatim. **role** — the node's `role()`.
    - **name** — the node's non-empty `label`, else the concatenated names of its `labelled_by` targets in order (a `TextRun` contributes its `value`), trimmed; `""` with no name source. A visited set guards cycles.
    - **bounds** — `BaseDocument::get_client_bounding_rect` of the element: viewport-relative CSS px snapped to 1/64 px, the union of the inline-fragment rects for a non-atomic inline; reported as computed, not clamped.
    - **state** — `enabled`: `Some(!has_attr("disabled"))` when `ElementData::can_be_disabled()`, else `None` (the presence reading). `checked`: `ElementData::checkbox_input_checked()`. `value`: for a `textarea` and for an `input` whose `type` is not checkbox, radio, button, submit, reset or hidden — `text_input_data().editor.text()`, else the `value` attribute; `None` for every other element. `focused`: the node's tree id equals the `TreeUpdate`'s `focus` (never `get_focussed_node_id`, which falls back to the root element).
  - No field holds a `NodeId`, an AccessKit tree id, an `ElementId`, a `ScopeId` or a pointer.
  - Remaining-caller facts: the chunk CALLS `Document::accessibility_tree`, `BaseDocument::get_node`, `BaseDocument::get_client_bounding_rect`, `ElementData::{can_be_disabled, has_attr, checkbox_input_checked, text_input_data, attr}` and changes none of their signatures or bodies; their other callers are untouched. The snapshot's only callers are its own unit tests and `stand_snapshot.rs`; no driver, CLI or MCP command exposes it yet.
  - No IPC method, endpoint, event, socket, port, env var, listener or process-wide state.
- **Crates / modules:** one new module, `snapshot`, in the existing crate dioxus-native-dom (crate-private module, public types re-exported; `#[cfg(feature = "accessibility")]`). dioxus-native-dom's module list at lib.rs:13-19 now reads dioxus_document, element_id, events, mutation_writer, snapshot (accessibility-gated), write_once_attr. No crate added, removed or re-edged.
- **Dependencies:** none — no manifest and no lockfile change (the `git diff --quiet` gate). `accesskit` was already an optional dependency behind `accessibility`.
- **Schema / config:** none. No config key, feature flag, scrub set or serialization shape. The snapshot has no wire form: its serialization is the "Compact snapshot serialization" route entry's.
- **Spec-master edits:** none by this chunk.
- **Counts / qualifiers moved:**
  - Workspace test baseline (`cargo test --workspace`, the `test` leg): 454 passed · 0 failed · 5 ignored → **471 passed · 0 failed · 5 ignored over 125 result lines** (+9 `dioxus-native-dom` `snapshot::tests` unit tests in the crate's existing lib result line, +8 `stand_snapshot` stand checks in one new result line). Basis: `grep -E '^test result' target/ci-logs/test.log` summed, read after `bash .github/scripts/ci-leg.sh fast` at /implement and again on the pre-CI commit `910d1237` (evidence/operator-pass.md). Stated at test-plan.md:314 (the §9 local-baseline chain, last link 454 · 0 · 5 with "result lines … not re-measured") and its leaf `.claude/docs/tests-summary.md:28` (grep `454`: test-plan 1 hit, tests-summary 1 hit; the two architecture.md hits, lines 85 and 178, are `document.rs:454`-style line citations, not this count).
  - Stand `ok` events (`bash scripts/agent-run.sh run stand`): 25 → **33** (+8 `stand_snapshot`, picked up by its `stand_` prefix with no script change). Basis: the plan's `agent-run.sh logs | python3 …` gate, `last line 33`, green. Stated at test-plan.md:111 (the §3 Proof chain, last link 25).
  - dioxus-native-dom lib unit tests: 9 → 18 (basis: the plan's baseline read `0 passed; 9 filtered out` under the `snapshot::` filter on the untouched tree; the same filter now reads `9 passed; 9 filtered out`).
  - Unchanged, re-read: `Ran 64 tests` (the `ci-scripts` leg); the `a11y` leg's three result lines 6 + 6 + 3.
  - Line citations into `packages/dioxus-native-dom/src/lib.rs` shift. Measured line map old → new (basis `git diff ee88e85e -- packages/dioxus-native-dom/src/lib.rs` and `nl -ba` of the new file): 1-7 unchanged (the `accessibility` feature line stays at 7 and now continues on a new line 8); old 8-15 → 9-16; two inserted lines 17-18 (`#[cfg(feature = "accessibility")]` · `mod snapshot;`); old 16-19 → 19-22; two inserted lines 23-24 (the gated `pub use snapshot::…`); old 20 and below → +5. Citing sites (grep `dioxus-native-dom/src/lib.rs` over the seven masters and `.andromeda/registries/*.toml`: architecture 4 lines carrying 6 citations, a11y-plan 2 lines carrying 2, the other five masters and the registries 0):
    - architecture.md:9 `lib.rs:3` — unchanged.
    - architecture.md:105 `lib.rs:1` — unchanged.
    - architecture.md:110 `lib.rs:33-56` → `38-61` (the `trace!` macro) · `lib.rs:47-50` → `52-55` · `lib.rs:5-10` → `5-11` (the feature list).
    - architecture.md:251 `lib.rs:12-16` → `13-19` (the module list, which now includes `snapshot`).
    - a11y-plan.md:55 `lib.rs:7` and a11y-plan.md:280 `lib.rs:7` — the line is still 7; the feature's doc text now spans 7-8 and names the snapshot model.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none changed. `scripts/agent-run.sh` is byte-identical; `run stand` selects `stand_snapshot.rs` by its `stand_` prefix. The `a11y` CI leg keeps its three files; `stand_snapshot` rides the workspace `test` leg.
- **Cross-project / external claims:**
  - CI: run `CI#37516167752` on `910d1237e4e2` (repo Turbolet85/escher), `verdict: green · checks 16/16 · wall 515 s` — read by `ci.py conclusion --sha HEAD --wait 1800`, recorded in evidence/operator-pass.md. The verdict was taken on the pre-CI commit's tree; this wrap's commit adds specs, the report and bookkeeping to it, no source.
  - Inputs: `inputs.py verify` reads `inputs: absent — no external input snapshotted`; the chunk read no external input live.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. (One PLAN claim did not hold and is dispositioned under Expected amendments: the plan's list names design-system and obs-plan as carrying `lib.rs` citations; the grep finds 0 in both.)
- **Expected amendments (from plan):**
  - *architecture §Standard Contracts — register the snapshot contract beside the Dioxus DOM bridge entry; update §Existing Scopes for dioxus-native-dom's new module* — **carried**: Symbols / APIs and Crates / modules above. Sites: grep `Dioxus DOM bridge` → architecture.md:134 (the §Standard Contracts entry); grep `dioxus-native-dom | packages/dioxus-native-dom` → architecture.md:251 (the §Existing Scopes row, which lists the modules and cites `lib.rs:12-16`).
  - *security-plan §Input Validation `id` row — re-open: the snapshot is an in-process reader with no second crossing* — **carried**: Symbols / APIs (membership · id) and Coverage below. Site: security-plan.md:107 (grep `stable element id` in the `Markup attributes` table, 1 hit). The row today says the id "leaves the process only through the platform accessibility adapter"; the snapshot adds an in-process consumer of the same `author_id` and no crossing: no log, event, socket, file or serialized form carries it (the two probes under Outcome read 0).
  - *test-plan §1 · §9 — the stand coverage line and the re-measured local baseline / stand `ok` count* — **carried**: Counts / qualifiers moved. Sites: test-plan.md:25 (§1, the stand-checks coverage sentence — grep `fresh process`, 1 hit), test-plan.md:111 (§3 Proof chain), test-plan.md:314 (§9 baseline chain).
  - *a11y-plan §7 — the snapshot reads role, name and focus from the accessibility tree, a consumer of §7's output* — **carried**: Symbols / APIs (field sources). Site: a11y-plan.md §7 Screen Reader Support (line 263 onward); a11y-plan.md:55 and :280 carry the feature-doc citation.
  - *architecture · a11y-plan · design-system · obs-plan — every `file:line` citation into `packages/dioxus-native-dom/src/lib.rs` past the inserted lines re-pointed by a measured line map* — **carried for architecture and a11y-plan** (the map and the six + two citations under Counts / qualifiers moved); **not carried for design-system and obs-plan**: the grep reads 0 `lib.rs` citations in either (their dioxus-native-dom citations point at `events.rs`, `dioxus_document.rs`, `mutation_writer.rs` and `Cargo.toml`, none of which this chunk touched).
- **Coverage of new surfaces:**
  - `DioxusDocument::snapshot` (an in-process read API; not an external-input surface, not a UI element) → validation n/a (it takes no argument; it reads data the document already admitted, resolves every tree id through `get_node` and omits what does not resolve, and its name resolver is cycle-guarded) · instrumentation n/a (no call site by decision: the plan's Constraints leave spans to the driver, and the route entry "Driver command spans" owns them; the census gate over both new files reads `0` for tracing, log, print, env and listener tokens) · PII n/a (names and values are user content and reach no log and no event: the census reads 0, and the agent-run event-key probe reads 0 content-named keys on a run holding `stand_snapshot` events) · tests unit 9 + integ 8 · a11y n/a (no interactive element added; the snapshot is a consumer of the accessibility tree, and the `a11y` leg stays green at 6 + 6 + 3) · tokens n/a.

## Deviations from intent
- **An unresolved node's subtree (plan step 5).** The plan says a node whose element no longer resolves "is omitted". The code omits that node and attaches its kept descendants to the nearest kept ancestor, the same rule membership applies to every non-kept node. Justification: the plan did not say what happens to the subtree, and dropping resolvable descendants would lose nodes the membership rule keeps. It cannot occur on a consistent document; `ids_are_the_accessibility_tree_ids` proves none is lost on the stand.
- **"No NodeId-derived token" (plan step 10).** The check asserts no id contains its own element's `NodeId` Debug form (`NodeId({index}v{version})`) or the text `NodeId`. Justification: a raw slot number would falsely match ordinary sibling indices such as `div:3`.
- **Unit-test fixture.** The unit tests call `doc.inner_mut().resolve(0.0)` after `initial_build()`. Justification: without a resolve no style exists, so `display:none` is not observable; no earlier unit test in the crate needed it.
- **`NodeState` derives `Default`** beyond the plan's `Debug`, `Clone`, `PartialEq`. Additive; no caller relies on it.
- scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- **Operator direction, the operator pass.** The operator directed the agent to drive the three `leg = 'operator'` entries by hand in plan order, the pre-CI commit included, and to stop on any red (2026-10-06). All three read green; record in evidence/operator-pass.md.
- **Operator intent rulings carried into this wrap (the founder, 2026-10-06, given as this wrap's directive):**
  1. Additivity is a preference, not a goal: where staying additive would force contorted logic, the upstream code is changed directly; the route's sync lines read "mostly additive".
  2. The stand is the proof, plus minimal fixtures beside it for cases the 7GUIs tasks lack (an element covered by another), so that everything built is validated.
  3. Element ids must stay stable across edits of the app's code, not only across re-render, remount and restart: a requirement of its own, with its own route entry ahead of the diff and driver work.
  4. The cold-agent test's wrong-call bar starts in 0.2.0; 0.1.0's run is the baseline that shows which values are reasonable.
  These are not this chunk's Changes. They are applied at this wrap's P5 (route, intent, requirements, ledger), and none retires a claim a spec master states: grep over the seven masters and the registries reads 0 hits for `additive`, and the masters already record `wrong_calls` as "recorded, never deciding" (test-plan.md:119).
- **Sweep hazard.** grep `454` over the masters returns two architecture.md hits that are `…/document.rs:…454…`-style line citations, not the workspace test count; a count sweep must read each hit.
- **Slip caught at compile time.** `Harness<DioxusDocument>` has no `snapshot`; the document's method is reached as `harness.doc.snapshot()`.

## Outcome
Acceptance criteria, each re-asserted against the diff:
- (arch · a11y) ids equal `element_id` and the tree's `author_id`, set-equal and pairwise distinct, on every lean task in both layout modes — **met** (`ids_are_the_accessibility_tree_ids`).
- (a11y) role and name equal the AccessKit node's; the 15 controls carry their roles and the six inputs their names — **met** (`role_and_name_match_the_accessibility_tree`).
- (layouts · design) the tree follows the TaskShell; every control rendered at boot has a non-zero finite rect inside 800×600 equal to `get_client_bounding_rect` — **met** (`tree_follows_the_task_shell` · `controls_lie_inside_the_viewport`).
- (a11y) state reads the engine: nothing focused at boot, exactly `back-btn` after one focus move, `flight-return-date` `enabled: Some(false)`, `flight-start`'s value equal to its editor text — **met** (`state_reads_the_engine`).
- (arch · tests) identical for `incremental` false and true and across two reads; follows a re-render — **met** (`snapshot_is_deterministic` · `snapshot_follows_a_rerender`).
- (tests) the builder's unit tests pass — **met** (9 passed).
- (security · arch) no `NodeId`/`ElementId`/`ScopeId`/pointer in the public model; the crate builds without `accessibility`; no manifest, lockfile, engine, harness, stand or task file touched — **met** (the type definitions in Changes; the build gate; the `git diff --quiet` gate).
- (obs · security) no log, print, env or listener site in the new files; no content-named key in any agent-run event — **met** (both probes read 0).
- (tests) `run stand` exits 0 and the stand `ok` count rises from 25 — **met** (33); `fast`, `doc`, `a11y` pass — **met**.
- (CI) the pushed HEAD reads `verdict: green` — **met** (`CI#37516167752` on `910d1237e4e2`).
- No matrix capability claimed — **met** (`matrix.py show --chunk 2026-10-06-snapshot-model`: claimed 0). v010-04 and v010-03 stay pooled.

Gates, by `run`, in block order (the gate tool's words; /implement's run, `entries 17 · green 14 · red 0 · not-run 3`):
- `cargo test -p dioxus-native-dom --locked --lib snapshot::` — green · exit 0 · `lacks FAILED` · `lacks running 0 tests`.
- `cargo test -p blitz-tests --locked --test stand_snapshot` — green · exit 0 · `contains 8 passed; 0 failed`.
- `cargo test -p blitz-tests --locked --test stand_accessibility_ids --test stand_element_ids … --test focusability_updates` — green · exit 0 · `lacks FAILED`.
- `cargo build -p dioxus-native-dom --no-default-features --locked` — green · exit 0.
- `git diff --quiet ee88e85e… -- Cargo.toml Cargo.lock deny.toml packages/blitz-dom …` — green · exit 0.
- `test -f packages/dioxus-native-dom/src/snapshot.rs && test -f … | grep -c -E 'tracing|…'` — green · exit 1 · `last line 0`.
- `bash scripts/agent-run.sh boot` — green · exit 0 · `contains "ready"`.
- `bash scripts/agent-run.sh run stand` — green · exit 0 · `contains "run.end"` · artifact fresh.
- `bash scripts/agent-run.sh logs | python3 -c '… startswith("stand_") …'` — green · exit 0 · `last line 33`.
- `bash scripts/agent-run.sh logs | python3 -c '… bad={…} …'` — green · exit 0 · `last line 0`.
- `bash scripts/agent-run.sh cleanup` — green · exit 0.
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (471 · 0 · 5; `Ran 64 tests`).
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0.
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0 (6 + 6 + 3).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, driven by hand: exit 0 · `contains hygiene: clean` (fired twice; evidence/operator-pass.md).
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — `leg = 'operator'`, driven by hand: exit 0 · push `ee88e85e..910d1237`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`, driven by hand: exit 0 · `contains verdict: green`.
- No deferral: the Rust delta is real, so every gate ran. Smoke: skipped — no boot-path or UI-surface change; the agent-run boot · run · logs · cleanup pairing ran as gates.

Watches: none folded.

Outcome basis: the operator pass ran, so the verdicts rest on its final state — the single commit `910d1237` and its CI run `CI#37516167752`, recorded in evidence/operator-pass.md; implement's P4 report (held in this session's conversation) is the basis for the deviations and the census. One operator directive sits between implement and this report (the operator pass, above); it changed no source.

Process hygiene: implement's census — gate entries (cargo, test binaries, agent-run) started by its run, all `terminated`, measured against `ps` with `target/agent-run/` removed by `cleanup`. The operator pass started the `fast` leg and one `ci.py` poll, both exited. Re-measured at this wrap: see the P7 console report.
