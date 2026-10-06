# Codebase Research — 2026-10-06-accessibility-tree-identity

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs/Greps:** 17 · **Code-graph queries:** 3 (trace `.andromeda/runs/2026-10-06T11-54-50-phase/tree-query-2026-10-06-accessibility-tree-identity.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full (42 lines), with its 1 Session Addition applied: a `timeout`-bounded boot smoke never reads green under the gate tool. That addition is not relevant here, because the chunk's live leg is `run stand` and runs no boot smoke. `.claude/rules/a11y.md` was auto-loaded and read in full (0 Session Additions).
- **Platform issues consulted:** none. This chunk has no runner-only bullet: Setup's CI read was `in progress`, with no red to fold.
- **External inputs:** none. Every fact this chunk turns on lives in this repository or in the pinned accesskit 0.25.0 crate source under the cargo registry. That source is a dependency's behaviour, not an input.

## Files inspected
- `packages/blitz-dom/src/accessibility.rs` (full, 255 lines) — `build_accessibility_tree` (6-44) walks the whole document with `self.visit` every time it is called. It keys every AccessKit node `NodeId(node.id.as_u64())` (51), gives the document root `Role::Window` (54-55), and roles elements from `role` attr → HTML-AAM (`role_from_element_data`, 165-255) → `Unknown` (62-65). It sets `html_tag` (68) and `hidden` for `aria-hidden="true"` (71-73), and makes text nodes `TextRun` with `value` and `push_labelled_by` on the parent (74-78). It sets **no** `label`, `author_id` or `html_id`, reads no `aria-label`, `aria-labelledby` or `<label>` association, and never reads `changed_nodes` (`grep -c changed_nodes packages/blitz-dom/src/accessibility.rs` → 0). The tree also carries a synthetic `Window` node `NodeId(u64::MAX)` (35-37).
- `packages/dioxus-native-dom/src/dioxus_document.rs` (full) — `DioxusDocument { inner: Rc<RefCell<BaseDocument>>, vdom: VirtualDom, vdom_state }` (67-80). The vdom is owned by the wrapper, not by `BaseDocument`. `element_id` / `element_ids` borrow `inner` immutably (198-206). `impl Document for DioxusDocument` (232-285) overrides `inner`, `inner_mut`, `poll`, `handle_ui_event` and `id`.
- `packages/blitz-dom/src/document.rs` (129-175, 620-660) — trait `Document: Any + 'static` with `inner` / `inner_mut` and defaulted `handle_ui_event` / `poll` / `id` (129-151). It is implemented by `PlainDocument` (154), `BaseDocument` (163) and `Rc<RefCell<BaseDocument>>` (172). `label_bound_input_element(label_node_id)` (627-660) already resolves a `<label>` to its `for`-target input or its first nested input. It is used for click activation, and the tree build does not use it.
- `packages/blitz-dom/src/config.rs` (full) — `DocumentConfig` carries net / navigation / shell / html-parser providers, with nothing that could reach a VirtualDom.
- `packages/blitz-shell/src/accessibility.rs` (full) — `AccessibilityState::update_tree(&mut self, doc: &BaseDocument)` calls `doc.build_accessibility_tree()` inside `adapter.update_if_active` (44-48).
- `packages/blitz-shell/src/window.rs` (370-384, 518-525, 44-68) — the view holds `doc: Box<dyn Document>`. `poll` rebuilds the whole tree from `self.doc.inner()` when `has_changes()` (376-381). `build_accessibility_tree` rebuilds from `self.doc.inner()` on `InitialTreeRequested` (521-524, called from application.rs:82).
- `packages/dioxus-native/src/lib.rs` (214-237) — the windowed app boxes a `DioxusDocument` as `Box<dyn Document>` for the shell. So the platform tree an AT reads is built from the inner `BaseDocument`, which today has no access to the id.
- `packages/blitz-test-harness/src/harness.rs` (1-80, 123-126) — `Harness { pub doc: D }`. `pump` = `doc.poll(None)` then `doc.inner_mut().resolve(time)`. A test can build the tree from `harness.doc` directly, so no harness accessor is required.
- `~/.cargo/registry/…/accesskit-0.25.0/src/lib.rs` (2104-2140, 2057) — the pinned accesskit (`Cargo.lock`: `accesskit 0.25.0`; root `Cargo.toml:141` `accesskit = "0.25"`) has string properties `author_id` ("A way for application authors to identify this node for automated testing purposes. The value must be unique among this node's siblings", 2127-2129), `html_id` (2135-2140), `label` (2104) and `labelled_by` (2057).
- `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud}.rs` (markup bodies) and `examples/seven_guis/src/app.rs` (82-172) — the control inventory below.
- `tests/blitz-tests/tests/stand_element_ids.rs` (1-40) and `tests/blitz-tests/tests/accessibility_roles.rs` (imports) — the stand-check shape: `stand::boot(task, stand::options(incremental))`, per-task key tables, `use accesskit::{NodeId, Role}`. `tests/blitz-tests/Cargo.toml:32` already depends on `accesskit`.
- `packages/{blitz-dom,blitz-shell,dioxus-native-dom,dioxus-native}/Cargo.toml` (features) — blitz-dom: `accessibility = ["accesskit"]` with accesskit optional (27, 65). blitz-shell: `accessibility = ["dep:accesskit", "dep:accesskit_xplat", "blitz-dom/accessibility"]` (15-18, 37-38). dioxus-native-dom: `accessibility = ["blitz-dom/accessibility"]` (18), with **no** accesskit dependency. blitz-dom's `lib.rs` re-exports no accesskit (`grep -n accesskit packages/blitz-dom/src/lib.rs` → 0 hits; the module is gated at 83-84).

## Graph impact (from the code-graph query)
- **`build_accessibility_tree`** (blitz-dom inherent) — 10 callers. `AccessibilityState::update_tree` at blitz-shell/src/accessibility.rs:46. The `View::build_accessibility_tree` wrapper chain at application.rs:81. Six `accessibility_hidden` tests (13, 33, 54, 75, 96, 125). `accessibility_roles::{unknown_tags, assert_role}` (28, 56). The inherent method keeps its signature, so none of these change.
- **`update_tree`** (blitz-shell) — 2 callers: `View::poll` (window.rs:379) and `View::build_accessibility_tree` (window.rs:523). If its parameter changes from `&BaseDocument` to the document trait object, both call sites are the whole boundary.
- **`element_ids`** — 10 callers, all in dioxus-native-dom and blitz-tests. The chunk reads it and does not change it.
- **Crate edges** — the change sits on existing edges (dioxus-native-dom → blitz-dom, blitz-shell → blitz-dom). blitz-dom gains no outbound edge.

## Patterns detected
- **Wrapper overrides a defaulted `Document` method** (document.rs:133-150; dioxus_document.rs:245-284): `poll` and `handle_ui_event` are defaulted on the trait and overridden by `DioxusDocument` with vdom-aware behaviour. A defaulted, `accessibility`-gated tree method on the trait is the same seam. It breaks none of the six implementors (`grep -rn 'impl Document for'`: PlainDocument, BaseDocument, `Rc<RefCell<BaseDocument>>`, HtmlDocument, DioxusDocument, ScriptDocument; `DioxusNativeDocument` implements `dioxus_document::Document`, a different trait).
- **Optional accesskit per crate behind `accessibility`** (blitz-shell/Cargo.toml:15-17, 37): a crate that names accesskit types declares `accesskit = { workspace = true, optional = true }` and enables it from its `accessibility` feature. dioxus-native-dom would follow this to name `TreeUpdate`. The dependency is already in the lock graph, so only dioxus-native-dom's dependency list in `Cargo.lock` changes.
- **Label→control resolution exists** (document.rs:627-660): `label_bound_input_element` resolves `for` → input by `id`, else the first nested input. It returns only `input`s and indexes `self.nodes[label_node_id]` directly, which is safe on a live node during a visit.
- **Name today = text children** (accessibility.rs:74-78): a control's only name source is `labelled_by` on its direct `TextRun` children.

**The stand's controls and their measured state** (role from accessibility.rs:165-255; name source from 74-78):

| id | element | role | name today |
|---|---|---|---|
| `back-btn` | button "← Back" | Button | TextRun |
| `counter-increment` | button "Count" | Button | TextRun |
| `flight-one-way` · `flight-return` · `flight-book` | button | Button | TextRun |
| `flight-start` · `flight-return-date` | input (text) | TextInput | **none** (no label in markup) |
| `timer-duration` | input range | Slider | **none** (sibling `label "Duration: "`, no `for`) |
| `timer-reset` | button "Reset" | Button | TextRun |
| `crud-filter` · `crud-name` · `crud-surname` | input (text) | TextInput | **none** (sibling `label`s, no `for`) |
| `crud-create` · `crud-update` · `crud-delete` | button | Button | TextRun |

There are 15 interactive controls: 9 buttons named, 6 inputs nameless. Derivation: `grep -n 'input\|button' examples/seven_guis/src/tasks/{flight_booker,timer,crud}.rs`, counter.rs:11-18 and app.rs:157-163, read row by row. Non-controls: `counter-value` / `flight-booked` / `timer-elapsed` are `p` (Paragraph). `timer-progress` is a `div` (GenericContainer), not a `<progress>`. CRUD rows are clickable `div`s (GenericContainer) named by their text. The lean four have **no** `select` and **no** `<progress>` element.

## Conventions to follow
- **Feature-gated accessibility code**: blitz-dom gates the module with `#[cfg(feature = "accessibility")]` (lib.rs:83-84), and the shell gates every call site (window.rs:376, 521). A new trait method and its override carry the same gate.
- **Stand checks**: a `//!` header, `stand::boot` / `boot_timer` with `stand::options(incremental)`, per-task key tables, `for incremental in [false, true]` (stand_element_ids.rs:1-40).
- **Spec citations in the tree build**: comments link the spec they implement (accessibility.rs:60-61, 70, 166). An accessible-name addition cites accname-1.2 and HTML-AAM.
- **Never index with a possibly stale id**: the override maps tree `NodeId(u64)` back through `element_ids()` pairs (`NodeId::as_u64` / `from_u64`, blitz-traits/src/node_id.rs:20-32) and does not index the slab.

## New files to create
- `tests/blitz-tests/tests/stand_accessibility_ids.rs`
- `tests/blitz-tests/tests/accessibility_names.rs`
  Added at P4 once the name-source fork was answered (the operator, 2026-10-06, P4: `aria-label` plus `<label>` association). The engine's name sources are a behaviour of their own, proved on plain HTML documents. This file also proves that a non-Dioxus tree carries no stable id.

## Files to modify
- `packages/blitz-dom/src/document.rs`
- `packages/blitz-dom/src/accessibility.rs`
- `packages/dioxus-native-dom/src/dioxus_document.rs`
- `packages/dioxus-native-dom/Cargo.toml`
- `Cargo.lock`
- `packages/blitz-shell/src/accessibility.rs`
- `packages/blitz-shell/src/window.rs`
- `examples/seven_guis/src/tasks/flight_booker.rs`
- `examples/seven_guis/src/tasks/timer.rs`
- `examples/seven_guis/src/tasks/crud.rs`

  Caller threading: the inherent `BaseDocument::build_accessibility_tree` keeps its signature, so its 10 callers are untouched. `update_tree`'s 2 callers (window.rs:379, 523) are both in `window.rs`, which is listed. Companion sweep for `build_accessibility_tree`: `grep -rn build_accessibility_tree --include=*.rs packages apps examples wpt tests` → 10 hits. These are the 10 graph callers above plus the definition, with no other hit. Each is `no change`, because the inherent method is unchanged. Seam: `accesskit` is a normal (optional) dependency of blitz-dom and blitz-shell and a dependency of blitz-tests (Cargo.toml:32). dioxus-native-dom has none, hence its manifest and the lock entry. The markup files are listed for both naming branches (`aria-label` or `<label for>`). Each branch writes the same three files.

## Open questions
- Where does the id cross into the tree? The candidates are a defaulted `Document` trait method that `DioxusDocument` overrides (reaching the shell and the harness), a `DioxusDocument`-only inherent post-pass (reaching tests but not the windowed platform tree, window.rs:376-381), or the DOM `id` attribute (contradicts arch's "written to neither the DOM nor the vdom"). A `DocumentConfig` provider cannot reach the vdom, which `DioxusDocument` owns (dioxus_document.rs:67-70). → blocks: plan-decision
- Do names come from `aria-label` alone, or also from `<label for>` / nested-label association through `label_bound_input_element`? → blocks: plan-decision
