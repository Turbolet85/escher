# Codebase Research — 2026-10-06-snapshot-model

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 13 · **Graph queries:** 3 (rust plane, `db_state` fresh)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (5 449 B, 1 Session Addition — the `timeout`-124 boot-smoke trap; not applicable: no boot smoke in this chunk) · `.claude/rules/testing.md` and `.claude/rules/a11y.md` (auto-loaded on the test reads, read in full)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `packages/blitz-dom/src/accessibility.rs` (full) — `build_accessibility_tree` walks `self.visit` in document order; a node hidden by `display:none` / `visibility:hidden` (and its subtree) is skipped (13-21, 114-124); each node's AccessKit id is `NodeId(node.id.as_u64())` (69) and it is pushed as a child of its parent, so the `TreeUpdate`'s `children` lists carry document order (106); role = `role` attr → HTML-AAM mapping → `Unknown` (80-85); `aria-label` → `label` (89-94); `aria-hidden="true"` → `set_hidden`, node kept (97-99); text nodes → `TextRun` with `value` = text, pushed onto the parent's `labelled_by` (100-104); a bound `<label>` is pushed onto its input's `labelled_by` after the walk (40-47); `focus` = `focus_node_id` else the window id `u64::MAX` (60).
- `packages/dioxus-native-dom/src/dioxus_document.rs` (186-305) — `element_id` / `element_ids` (200-208, pre-order, distinct) and the `#[cfg(feature = "accessibility")]` `Document::accessibility_tree` override that sets `author_id` from `element_ids()` on element nodes only (289-304).
- `packages/dioxus-native-dom/src/lib.rs` (full) — module list and `pub use` surface (12-22); the crate re-exports `blitz_dom::NodeId`; `dioxus-native` re-exports `dioxus_native_dom::*` (packages/dioxus-native/src/lib.rs:27).
- `packages/dioxus-native-dom/Cargo.toml` (full) — `accessibility = ["blitz-dom/accessibility", "dep:accesskit"]`, a default feature (13, 17); `accesskit` optional (38).
- `packages/blitz-dom/src/document.rs` (150-162, 610-618, 2225-2345, 2700-2730) — `Document::accessibility_tree` default (158-161); `get_focussed_node_id` falls back to the ROOT ELEMENT when nothing is focused (616-618) — not a "focused" reading; `get_client_bounding_rect` (2238-2256): viewport-relative CSS px (`- viewport_scroll()`), snapped to 1/64 px, and for a non-atomic inline element the UNION of `inline_fragment_rects` (2241-2243); `BoundingRect { x, y, width, height: f64 }` with `PartialEq` (2708-2714).
- `packages/blitz-dom/src/node/element.rs` (425-490, 558-610) — `DISABLED`/`ENABLED` element state set from `disabled` PRESENCE for `can_be_disabled()` elements (445-451); `can_be_disabled` = button · input · select · textarea (476-478, pub); `checkbox_input_checked() -> Option<bool>` (588-593, pub); `text_input_data()` (558, pub).
- `packages/blitz-dom/src/node/text.rs` (55-80) — `TextInputData.editor` is a pub `parley::PlainEditor`; its `text()` is the typed value.
- `packages/blitz-dom/src/form.rs` (300-320) — form submission reads a control's value as `text_input_data().editor.text()` else the `value` attribute (316-319): the engine's one existing value reader.
- `packages/blitz-dom/assets/default.css` (850-858) — `head` (with base, link, meta …) is `display: none`, so `<head>` and its children never reach the accessibility tree.
- `packages/blitz-test-harness/src/inspect.rs` (45-140) — `layout_rect_of` = `absolute_position` + `final_layout` (page coordinates, unsnapped); `focused()` = `get_focussed_node_id` (root fallback); `dom_string()`.
- `examples/seven_guis/src/stand.rs` (full) — `boot(LeanTask, options(incremental))`, `boot_timer`, pinned 800×600 · scale 1.0 · Light · bundled DejaVu Sans · offline; read only, not touched.
- `examples/seven_guis/src/tasks/*.rs` + `app.rs` (`id:` grep) — the stand's author ids: `back-btn`, `task-title`, `task-header`, `task-header-spacer`, `task-shell`, `task-body`; counter 2 (`counter-value`, `counter-increment`); flight booker 6 (`flight-one-way`, `flight-return`, `flight-start`, `flight-return-date`, `flight-book`, `flight-booked` — the last rendered only once a booking is made, flight_booker.rs:109-110); timer 5 (`timer-progress`, `timer-elapsed`, `timer-duration`, `timer-duration-value`, `timer-reset`); CRUD 7 (`crud-filter`, `crud-list`, `crud-name`, `crud-surname`, `crud-create`, `crud-update`, `crud-delete`; rows keyed `div[{person.id}]`). Re-derived: `grep -n 'id:' examples/seven_guis/src/tasks/*.rs examples/seven_guis/src/app.rs`.
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` (full) — the precedent check: `controls(task)` (the 15 controls with roles), `INPUT_NAMES`, the `name()` resolver (label → TextRun value → joined `labelled_by` names, 100-114), `assert_carried`, both layout modes.
- `tests/blitz-tests/Cargo.toml` (17-22) — blitz-tests already names `dioxus-native-dom` with `features = ["accessibility"]` and depends on `seven_guis`: no manifest change is needed for the proof.

## Graph impact (rust plane, trace `.andromeda/runs/2026-10-06T18-16-44-phase/tree-query-2026-10-06-snapshot-model.json`)
- **accessibility_tree / build_accessibility_tree / element_ids / get_client_bounding_rect / inline_fragment_rects** — 45 caller rows (query 1). This chunk CALLS these and changes none of their signatures: the shell (`blitz-shell/src/accessibility.rs:45`, `application.rs:82`), the override (`dioxus_document.rs:292-294`), the a11y tests, `element_id.rs` unit tests, the stand checks; `get_client_bounding_rect` is already the engine's one bounding-box reader — Dioxus `onmounted` `get_client_rect` (`dioxus-native-dom/src/events.rs:212`), script `getBoundingClientRect` (`blitz-vibey-script/src/dom/element.rs:1320`), `node_client_rects` (`document.rs:2315`), the debug overlay. Additive change: zero blast radius.
- **`snapshot` name collision** (query 2, `lower(name) LIKE '%snapshot%'`, 17 rows) — every blitz-dom hit is Stylo's element-snapshot invalidation (`snapshot_node`, `has_snapshot`, `SnapshotMap` …, stylo.rs / node.rs / document.rs); one `snapshot` fn in `apps/browser/src/fps_overlay.rs`. None in dioxus-native-dom: a `snapshot` module and method there collide with nothing, and keeping the model OUT of blitz-dom avoids overloading the Stylo term.
- **dioxus-native-dom crate edges** (query 3, 9 rows) — inbound from blitz-examples, browser, blitz-test-harness, wgpu_texture, seven_guis, blitz-tests, dioxus-native; outbound to blitz-dom, blitz-traits. A new pub module there adds no edge.

## Patterns detected
- **Wrapper enriches the base tree** (dioxus_document.rs:289-304): the stable id exists only in dioxus-native-dom, so anything carrying it is built there on top of blitz-dom's output. The snapshot follows the same seam.
- **One reader per fact**: role/name live in the AccessKit build (accessibility.rs), bounds in `get_client_bounding_rect` (document.rs:2238), a control's value in the form reader rule (form.rs:316-319), disabled in the `DISABLED` state set from presence (element.rs:445-451). Every snapshot field has an existing reader to call; none needs a new derivation.
- **Name model is AccessKit's, resolved by the consumer**: blitz-dom never writes a computed name — an element's name is its `label`, else the concatenation of its `labelled_by` targets (TextRuns' `value`, bound `<label>`s' own names). `stand_accessibility_ids.rs:100-114` resolves it that way.
- **Stand checks shape** (stand_accessibility_ids.rs): `LeanTask::ALL × [false, true]`, `#[track_caller]` helpers, assert the fixture first, non-vacuous counts.

## Conventions to follow
- **Feature gating**: accessibility-only code is `#[cfg(feature = "accessibility")]` (dioxus_document.rs:290); the crate builds with `--no-default-features` (the prior chunk's build gate).
- **Public docs**: `///` on every pub item; `cargo doc --workspace --no-deps --locked` under `-D warnings` (architecture §Conventions → Documentation).
- **No telemetry**: the override and `element_id` have no tracing call site; the obs history leaves spans to the driver (Epoch 4) — the builder adds none.
- **Stale ids**: resolve a `NodeId` from a tree id with `get_node` (returns `Option`) — never index (CLAUDE.md invariant; security-plan §Error Handling).

## New files to create
- `packages/dioxus-native-dom/src/snapshot.rs` — the snapshot types and the builder (with inline `#[cfg(test)] mod tests`)
- `tests/blitz-tests/tests/stand_snapshot.rs` — the stand proof

## Files to modify
- `packages/dioxus-native-dom/src/lib.rs` — declare the `snapshot` module and re-export its public types

## Open questions
- Which nodes the snapshot contains (every element the accessibility tree keeps vs only semantic ones) → blocks: plan-decision (P4 fork; the rest of the model is fixed by the readers above).
