# Codebase Research — 2026-10-07-change-tracking-and-diff

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 12 · code-graph queries 8 (rust plane, trace `tree-query-2026-10-07-change-tracking-and-diff.json` in the phase run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 addition read, 0 applied (it concerns a `timeout`-bounded boot smoke; this chunk plans none) · `.claude/rules/testing.md` — 2 additions, 1 applied (never drive a check with a deleting or platform-bound key: the diff's steps are clicks, Tab and typed characters) · `.claude/rules/a11y.md` — 1 addition applied (on the dev host AccessKit's `update_if_active` never runs the tree build in a windowed boot, so the shell's refresh cannot be proven by a windowed run here)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `packages/blitz-dom/src/document.rs` (:96-190, :320-330, :486-516, :780-803, :854-866, :1000-1014, :1420-1440, :1536-1560, :1640-1712, :2237-2254) — the `Document` trait, the set and the flag, the constructor's root creation, `create_node`, the style-snapshot helpers every element-state change goes through, the focus setters, the viewport-relative bounds reader
- `packages/blitz-dom/src/mutator.rs` (:40-94, :196-232, :266-318, :392-420, :530-560, :868-896, :1040-1062, :1272-1292, :1360-1400) — the mutator's fields, drop and flush, its text / attribute / removal writers, the unit-test shape
- `packages/blitz-dom/src/events/keyboard.rs` (:96-135) — where a typed character lands
- `packages/blitz-dom/src/events/pointer.rs` (:530-552, :640-692, :800-820) — pointer-down focus, checkbox and radio toggles, the click that clears focus
- `packages/blitz-dom/src/layout/construct.rs` (:136-146, :706-714) — layout's own calls to `create_node`
- `packages/blitz-shell/src/window.rs` (:360-392, every `accessibility` site by grep) and `packages/blitz-shell/src/accessibility.rs` (full) — the flag's caller and what it gates
- `packages/dioxus-native-dom/src/lib.rs` (full) — module list, feature gates, re-exports, the crate doc's feature list
- `packages/dioxus-native-dom/src/snapshot.rs` (:1-250) — the model, its derives, its readers, the unit-test `build` helper
- `packages/dioxus-native-dom/src/dioxus_document.rs` (`poll`, `accessibility_tree`) — where vdom writes land, what a tree build costs
- `packages/blitz-test-harness/src/harness.rs` (full) and `input.rs` / `inspect.rs` (signatures) — what a step is
- `examples/seven_guis/src/stand.rs` (full) — the boot surface
- `tests/blitz-tests/tests/stand_snapshot_state.rs` (:1-135) and `stand_snapshot_text.rs` (:118-160, :362-400) — helper, fixture and no-content-in-messages shapes

## Graph impact (from the code-graph query; rust plane, lines cited are the editor's)
- **`has_changes`** — 1 caller: `View::poll` @ `packages/blitz-shell/src/window.rs:378`. Its signature does not change; only what it answers.
- **`changed_nodes`** (field) — 3 references besides its definition (`document.rs:327`): the initialiser `:491`, the insert in `create_node` `:863`, the read in `has_changes` `:1010`. No reader drains it; no crate outside blitz-dom names it.
- **`create_node`** — 7 non-test callers: `BaseDocument::new` `document.rs:514` (the root node), `create_text_node` `:1016`, `deep_clone_node` `:1038`, layout's `create_anonymous_block` `layout/construct.rs:142` and `flush_pseudo_elements` `:712`, the mutator's `create_comment_node` `mutator.rs:135` and `create_element` `:148`. Two of the seven are layout, not mutation.
- **`snapshot_node_and`** (the element-state change helper) — 11 non-test call sites: focus (`clear_focus` `document.rs:1663`, `set_focus_to` `:1688`, `:1694`), active (`:1727`, `:1742`), hover (`:1882`, `:1885`, `:1911`), radio (`toggle_radio` `:700`), checkbox and radio clicks (`events/pointer.rs:647`, `:671`). `snapshot_node` (the attribute-change helper) — 2: `set_attribute` `mutator.rs:271`, `clear_attribute` `:399`.
- **`apply_generated_text_input_event`** — 2 callers: `events/keyboard.rs:97`, `events/ime.rs:24`. The one place a text control's editor change is announced.
- **`DioxusDocument::snapshot`** — 58 call sites, every one in a unit test or a `tests/blitz-tests` file; no library, binary or script calls it. A diff over two snapshots adds no caller threading.
- **`accessibility_tree`** — 3 callers: the shell's `update_tree` `blitz-shell/src/accessibility.rs:45`, `unkeyed_actionable` `actionable.rs:57`, `snapshot` `snapshot.rs:84`.
- **`update_tree`** — 2 callers: `View::poll` `window.rs:380` (behind the flag) and `View::build_accessibility_tree` `window.rs:523` (on `InitialTreeRequested`, `application.rs:81-82`).
- **Name collisions** — `diff`, `SnapshotDiff`, `snapshot_diff`, `take_changed_nodes`, `drain_changed_nodes`, `NodeChange`, `Change`: 0 rows on `symbol` — every candidate name is free.
- **Crate edges** — blitz-dom has 16 inbound crates, dioxus-native-dom 7. An additive public item on either breaks none; a changed answer from `has_changes` reaches blitz-shell only.

## Patterns detected
- **The flag is constant, not merely inverted** (`document.rs:514`, `:863`, `:1009-1010`): the constructor creates the root node through `create_node`, which inserts into `changed_nodes`; nothing removes from the set; `has_changes()` returns `changed_nodes.is_empty()`. So it reads `false` for every document from construction on, and the shell's `if has_changes { update_tree }` (`window.rs:378-381`) has never run. The platform accessibility tree is built once, on AccessKit's `InitialTreeRequested`, and never refreshed. By code read; no run measures it, and the dev host cannot (the a11y rule's addition: the AT-SPI bus reads inactive, so `update_if_active` skips the build).
- **The set is written by layout** (`layout/construct.rs:142`, `:712`): anonymous blocks and pseudo-element nodes are created through the same `create_node`. A set that keeps that write site reads "changed" after a resolve that only rebuilt boxes, and may differ between the two layout modes — non-incremental mode rebuilds boxes each resolve (architecture §Cross-cutting Patterns → Invalidation and state integrity). Not measured: whether an idle pump in non-incremental mode in fact creates nodes.
- **The mutator already tracks "did a rendered mutation occur"** (`mutator.rs:62`, `:69-76`): a per-mutator bool set at 15 write sites (`grep -n 'mutations_occurred |=' packages/blitz-dom/src/mutator.rs`), each guarded by `node_is_in_document`, read once at `Drop` to request a redraw. Those 15 sites are exactly where a node-level mark would sit.
- **Element-state changes share one helper** (`document.rs:1540-1551`): focus, hover, active and checked all go through `snapshot_node_and(node_id, ElementState, cb)`; the state bits name which kind. Hover and active reach it on every pointer move and press — a mark there that did not select on the bits would make every click read as a change.
- **Writes that change a snapshot reading and bypass the mutator**: focus (`set_focus_to` / `clear_focus`, `document.rs:1660-1703` — `focused` on two nodes), checked (`events/pointer.rs:647-675`, `toggle_radio` `document.rs:686-703`), a text control's editor text (applied in the event handler, announced at `events/keyboard.rs:104-125`), scroll (`get_client_bounding_rect` subtracts `viewport_scroll()`, `document.rs:2249-2250` — a viewport scroll moves every node's bounds), a viewport resize.
- **A reading changes on nodes nothing wrote**: a node's `name` is its label, else the joined names of its `labelled_by` targets, a text run's name being its value (`snapshot.rs:160-179`) — a text node's edit renames its parent element and any input a `<label>` binds; `bounds` come from layout (`snapshot.rs:127`). So no per-node engine set equals "the nodes whose snapshot changed"; the equality the acceptance needs — the diff names node N **iff** N's reading differs between the two snapshots, or N is in one and not the other — is decided by comparing two `Snapshot` values, which `PartialEq` on `Snapshot` / `SnapshotNode` / `NodeState` (`snapshot.rs:17`, `:24`, `:45`) and the by-id reader `Snapshot::get` (`snapshot.rs:74-76`) support with no second reading of any control.
- **Equal snapshots are provable today**: `stand_snapshot::snapshot_is_deterministic` reads two snapshots of one screen equal, in both layout modes (called at `stand_snapshot.rs:398`, `:403`). A comparison of two readings with no step between them is therefore empty by construction.
- **The engine-touched, snapshot-unchanged case exists**: `set_node_text` writes only when the text differs (`mutator.rs:208-210`); `set_attribute` snapshots, damages and flags without comparing (`mutator.rs:268-316`). An attribute re-written to its own value touches the engine and changes no reading; so does a further character typed into a non-empty password input (the value reads `MASKED_VALUE` before and after, `snapshot.rs:207-210`).
- **A click that matches nothing clears focus** (`events/pointer.rs:813-817`): a click on a control the click handler does not match runs `clear_focus`. At boot nothing is focused, so the first click changes no `focused` reading; after a Tab press it can. Which controls match is not measured here — the stand check reads it from the diff rather than assuming it.
- **In-file fixtures boot with the stand's options** (`stand_snapshot_text.rs:387-389`): `Harness::from_vdom(VirtualDom::new(fixture), stand::options(incremental))` — the stand's pinned viewport and bundled font, not the harness default. The layouts extract's "harness default viewport" and the tests extract's "the stand's options" differ only in the font context; the existing fixtures settle it for the stand's.
- **Stand checks never format content into a message** (`stand_snapshot_text.rs:7`, `:117`, `:165-262`): content lives in `text`-named locals, assertions carry mode and id labels, nothing is printed. `assert_eq!` over two content values would print both on failure — the existing check uses `assert!(a == b, "{mode}: …")` for content.

## Conventions to follow
- **Mutations through the mutator; marks beside the flag it already sets** — `DocumentMutator` (`mutator.rs:46-76`); never through `DocumentMutator::doc` from outside (CLAUDE.md invariant).
- **Stale ids read through `get_node`** — the snapshot's own reader does (`snapshot.rs:125-126`); a drained id may name a dropped node.
- **Feature gate and re-export** — a snapshot-level module is `#[cfg(feature = "accessibility")] mod …;` with a gated `pub use` and a line in the crate doc's feature list (`lib.rs:7-9`, `:20-23`, `:30-33`); module crate-private, types re-exported.
- **Tracing** — engine call sites sit under `#[cfg(feature = "tracing")]` (`document.rs:1681-1682`); the touched spans of `mutator.rs` hold 5 such sites (`:1147-1216`, image and iframe loading), `window.rs` 1 (`:835`), `snapshot.rs` and `snapshot_text.rs` none (`grep -n 'tracing::'`). The chunk needs none.
- **Unit tests** — blitz-dom: `BaseDocument::new(DocumentConfig::default())`, nodes through `create_node` / `document.mutate()` (`mutator.rs:1379-1400`). dioxus-native-dom: `build(app)` = `DioxusDocument::new` + `initial_build` + `resolve(0.0)` (`snapshot.rs:219-224`).
- **Stand checks** — `stand::boot(task, stand::options(incremental))` inside `for incremental in [false, true]`, input through the harness helpers, `#[track_caller]` on helpers (`stand_snapshot_state.rs:24-95`).
- **Public items documented** — the rustdoc gate runs `-D warnings` over every workspace crate; a new `pub fn` on `BaseDocument` and a new public type in dioxus-native-dom each carry `///`.

## New files to create
- `packages/dioxus-native-dom/src/snapshot_diff.rs` — the diff over two snapshots and its unit tests
- `tests/blitz-tests/tests/stand_diff.rs` — the stand check: real steps on the lean tasks and the in-file fixtures, both layout modes

## Files to modify
- `packages/blitz-dom/src/document.rs` — the flag's answer, a drain, the set's write sites at node creation and at the focus and checked state changes
- `packages/blitz-dom/src/mutator.rs` — the set's write sites at the mutation writers
- `packages/blitz-dom/src/events/keyboard.rs` — the set's write site at a text control's editor change
- `packages/blitz-shell/src/window.rs` — the flag's caller: what the poll does once the flag is truthful and who drains
- `packages/dioxus-native-dom/src/lib.rs` — the gated `mod` and `pub use` lines and the crate doc's feature list

  The list is P4's fork as decided (the operator, 2026-10-07, at P4's question round): marks at the mutation writers, at focus and checked changes and at a typed character — so `packages/blitz-dom/src/scrolling.rs` is not written — and the diff is a pure function of two snapshots with no entry point of its own on `DioxusDocument`, so `dioxus_document.rs` is not written either. Sweeps: `has_changes` — 6 hits · 3 changed (the definition `document.rs:1009`, its caller's two lines `window.rs:378-379`) · 3 no-change (a local of the same name in `poll_subdocuments`, `document.rs:789`, `:799`, `:801`) — `grep -rn 'has_changes' --include=*.rs packages apps examples tests wpt`. `changed_nodes` — 4 hits · 3 changed or kept as the plan decides · 1 no-change (the initialiser) — same grep. `.claude/docs/services/blitz-dom.md:19` names the set as "the natural seed for snapshot diffs" — a derived doc, the wrap cascade's to re-derive.

## Open questions
- How far does the set's marking reach — every engine write an agent can read (mutations, focus, checked, typed text, scroll), so that the flag reading false means the snapshot did not change; or the mutation writers only, with a step's change flag read off the diff? → blocks: plan-decision
- What does the shell's poll do once the flag is truthful — refresh the platform tree when the set is non-empty and then drain it (the a11y plan's recorded intent, never yet the behaviour), with no windowed proof available on this host? → blocks: plan-decision
- Does this chunk give the diff a text form, or leave it a Rust value until the driver needs one? → blocks: plan-decision
