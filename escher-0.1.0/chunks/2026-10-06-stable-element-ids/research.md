# Codebase Research — 2026-10-06-stable-element-ids

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (5 452 B, one file, no over-cap extraction): 1 Session Addition (the boot-smoke `timeout` exit-124 note), 0 applied — this chunk names no bounded boot smoke; its live leg is `agent-run.sh run stand`, whose exit grammar and event shape the rule's "5-command contract" section states (atom source for P4: that section, lines "Exit grammar" and "Stdout is JSON lines only").
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a's run `CI#37438837787` was in progress, nothing folded) and no CI-reading entry outside the operator leg.
- **External inputs:** none — every fact this chunk turns on lives in this repository or in the locked dependency `dioxus-core 0.7.10` (`Cargo.lock:2441-2443`), whose registry source is reproducible from the lockfile at that version and is cited by version + file:line below, not snapshotted (it is neither a sibling repository, a relay nor a memory — inputs-contract §The folder's three kinds).

## Files inspected
- `packages/dioxus-native-dom/src/mutation_writer.rs` (1-140, fn list) — `WriteMutations` receives templates, `ElementId`s and template-relative byte paths (`assign_node_id(path: &'static [u8], id)` at :122, `load_template(template, index, id)` at :290); no method carries a Dioxus `key` or a component name. `DioxusState` (:13-26) holds `node_id_mapping: Vec<Option<NodeId>>` (ElementId → NodeId) and `element_id_mapping` (reverse); `try_element_to_node_id` (:47-49) is the non-panicking lookup, `element_to_node_id` (:42-44) unwraps.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (60-160, fn list) — `DioxusDocument` has `pub vdom: VirtualDom` and `pub vdom_state: DioxusState` (:67-80). `new` (:84-142) builds the skeleton `html > head + body > main#main` with `doc.mutate()` OUTSIDE the vdom; `main` carries the attribute `id="main"`; the vdom mounts into `main` (`DioxusState::create(main_element_id)`, :133). Inline `#[cfg(test)] mod tests` at :366 with `keyed_nodes_do_not_crash` at :378.
- `packages/dioxus-native-dom/src/lib.rs` (full exports) — private modules `dioxus_document`, `events`, `mutation_writer`, `write_once_attr`; public re-exports `DioxusDocument`, `NodeHandle`, `synthetic_click_event`, `NodeId`, `DocumentConfig`. `trace!` (:33-) expands to `#[cfg(feature = "tracing")] tracing::debug!` — gated.
- `packages/blitz-test-harness/src/harness.rs` (1-140) — `Harness<D: Document> { pub doc: D, time }`; `Harness<DioxusDocument>::from_vdom` builds, `initial_build`s and pumps; `base()` gives `DocGuard` read access. A check holding `Harness<DioxusDocument>` reaches `harness.doc.vdom` / any `DioxusDocument` method directly.
- `packages/blitz-test-harness/src/inspect.rs` (fn list) — `query`, `node`, `query_all`, `layout_rect(_of)`, `text_content`, `attr`, `hit(_node)`, `focused`, `hovered`, `dom_string` — no id-like read.
- `packages/blitz-dom/src/node/node.rs:1021` `element_data()` and `packages/blitz-dom/src/document.rs:598` `get_node(NodeId) -> Option<&Node>` — the stale-safe DOM reads a walk needs.
- `~/.cargo/registry/src/…/dioxus-core-0.7.10/src/nodes.rs` (15-60, 175-240, 420-450, 490-625) — `VNodeMount` (crate-private) holds `root_ids` per template root and `mounted_dynamic_nodes` (a ScopeId for a component). Public: `VNodeInner.key: Option<String>` (:49, "the key given to the root of this template; in fragments the key of the first child"), `VNodeInner.template` / `dynamic_nodes`, `VNode::dynamic_root(idx)` (:178), `VNode::mounted_root(root_idx, &VirtualDom) -> Option<ElementId>` (:206), `VComponent.name: &'static str` (:533, pub), `VComponent::mounted_scope(idx, &VNode, &VirtualDom) -> Option<&ScopeState>` (:601), `DynamicNode::{Component, Text, Placeholder, Fragment}` (:490-).
- `~/.cargo/registry/src/…/dioxus-core-0.7.10/src/diff/node.rs:833-844` — `load_template_root` sets the mounted root id for every STATIC template root (`set_mounted_root_node`) and emits `load_template`; a static element nested INSIDE a template root gets an ElementId only when it carries a dynamic attribute or listener (`assign_static_node_as_dynamic`, :856).
- `~/.cargo/registry/src/…/dioxus-core-0.7.10/src/virtual_dom.rs:286-297, 336-345` — the root component is named `"root"` (`VComponent { name: "root", … }`); `VirtualDom::base_scope()` and `get_scope(ScopeId)` are public; `ScopeState::root_node()` is public (`scopes.rs:89`); the scope's own `name` is `pub(crate)` (`scope_context.rs:46`).
- `examples/seven_guis/src/app.rs` (1-30, 82-112, 153-200, CSS) — `task_in_shell` (:82) renders `TaskShell { title, on_back, <Task> {} }`; `TaskShell` (:153, `#[component]`) renders `style { }` + `div#task-shell > header#task-header > (button#back-btn, h1#task-title, div#task-header-spacer)` + `main#task-body > {children}`. Shell CSS selects by id (`#task-shell`, `#task-header`, `#back-btn`, `#task-title`, `#task-header-spacer`, `#task-body`, :282-324); home CSS by `#home*` / `#task-grid`.
- `examples/seven_guis/src/stand.rs` (fn list, 95-103) — `stand_root` (a plain fn component behind `VirtualDom::new_with_props`, so the vdom names it `"root"`) provides the timer ticks context and returns `task_in_shell(…)`.
- `examples/seven_guis/src/tasks/counter.rs` (full) — `Counter` (`#[component]`): `div.counter-root > style + div.counter-card > (p.counter-display, button.counter-btn)`; no `id:`, no `key:`.
- `examples/seven_guis/src/tasks/crud.rs` (40-80) — the list renders `for (i, person) in people_snap.iter().enumerate() { if … { div.list-item {…} } }` — unkeyed; a filtered-out row is a placeholder in the fragment, so an item's fragment position equals its `people` index `i`.
- `tests/blitz-tests/tests/stand_boot.rs` (1-30, selectors) — stand checks compare two boots' `dom_string()` (:78-90) and query the chrome by id selectors; none pins a literal `dom_string`.
- `scripts/cold_agent_stub.py` (grep) — the stub models its own toy counter (`count`, `inc`, `dec`, `reset`); it never boots or reads the stand DOM.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-06T08-52-21-phase/tree-query-2026-10-06-stable-element-ids.json`)
- **try_element_to_node_id** — 3 callers: `DioxusState::element_to_node_id @ packages/dioxus-native-dom/src/mutation_writer.rs:43`, `MutationWriter::assign_node_id @ packages/dioxus-native-dom/src/mutation_writer.rs:126`, `assert_no_stale_mappings @ tests/blitz-tests/tests/stale_node_mapping.rs:80` — the id walk becomes a 4th, read-only caller; no signature changes.
- **crate edges of dioxus-native-dom** — inbound from `blitz-examples`, `browser`, `blitz-test-harness`, `wgpu_texture`, `seven_guis`, `blitz-tests`, `dioxus-native`; outbound to `blitz-dom`, `blitz-traits`. The change is ADDITIVE (new pub items, no changed signature), so the 7 inbound crates have zero blast radius; `blitz-tests → dioxus-native-dom` already exists, so the stand check needs no manifest edge.
- **name collision** (`element_id_of`, `stable_id`, `StableId`, `ElementKey`, `element_key`) — 0 rows: free.

## Patterns detected
- **Non-panicking id resolution** (`mutation_writer.rs:47-49`): `node_id_mapping.get(i).copied().flatten()` — the shape the id walk uses for every ElementId → NodeId hop; the stale-mapping guard test (`stale_node_mapping.rs`) pins that mappings never alias.
- **Structural position as identity** (arch §Established Decisions → [Selection, animation, pseudo-elements]; the anonymous-block endpoint stores parent + sibling index): the in-tree precedent for a path built from parent and sibling position rather than a slot id.
- **Template ownership** (dioxus-core `diff/node.rs:833-856`): only static template ROOTS and dynamically-touched nodes get ElementIds, so a vdom walk alone reaches the template-root elements; the static elements nested under a root are reached by walking the DOM below that root.

## Conventions to follow
- **Mutations through the mutator**: CLAUDE.md invariant; a computed read writes nothing, so it needs no `doc.mutate()` (the lean P4 states).
- **Feature-gated tracing**: `trace!` in `dioxus-native-dom/src/lib.rs:33` is `#[cfg(feature = "tracing")]`; no new log site is needed.
- **One behaviour per integration file, `//!` doc first, `#[track_caller]` helpers, `for incremental in [false, true]`**: `tests/blitz-tests/tests/stand_boot.rs:1-3, 78-90`.
- **Inline unit tests in dioxus-native-dom**: `dioxus_document.rs:366-378` (`DioxusDocument` over `DocumentConfig::default()`, `initial_build`, then poll) — the shape for keyed-list / duplicate-key / stale-id unit cases.

## New files to create
- `packages/dioxus-native-dom/src/element_id.rs` — the stable-id derivation: vdom walk from the base scope (component names, keys, template roots) joined with a DOM walk below each template root; the id grammar; the uniqueness rule; inline unit tests.
- `tests/blitz-tests/tests/stand_element_ids.rs` — the per-task proof check: every element of each lean task, both layout modes, reads one id; keyed read the key, the rest the component path; ids unique.

## Files to modify
- `packages/dioxus-native-dom/src/lib.rs` — `mod element_id;` and re-export of the public id read type(s).
- `packages/dioxus-native-dom/src/dioxus_document.rs` — the public read surface on `DioxusDocument` (an id for a `NodeId`; every element's id in document order).
- `examples/seven_guis/src/tasks/counter.rs` — author keys on the task's controls (P4 fork; listed for both branches).
- `examples/seven_guis/src/tasks/flight_booker.rs` — author keys on the task's controls (P4 fork; listed for both branches).
- `examples/seven_guis/src/tasks/timer.rs` — author keys on the task's controls (P4 fork; listed for both branches).
- `examples/seven_guis/src/tasks/crud.rs` — author keys on the task's controls (P4 fork; listed for both branches).

## Open questions
- What is "the author's key": the HTML `id` attribute (document-unique, already on the TaskShell chrome, written to the DOM), the Dioxus `key` (exists only on list-item VNode roots, sibling-unique, never reaches the DOM), or both in a fixed order? → blocks: plan-decision
- Do the four lean tasks' controls get author keys in this chunk (the chrome is keyed on every task, so the keyed branch of the acceptance is non-vacuous either way)? → blocks: plan-decision
