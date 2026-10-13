# Codebase Research — 2026-10-13-upstream-sync-polish-and-ship

## Scope
- **Depth:** deep · **Reads:** 31 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition applied (the `timeout` exit-124 rule: no boot smoke here is bounded by `timeout`); `.claude/rules/testing.md` — read in full, 7 Session Additions applied (merged-stream attribution of `Running tests/{name}.rs` lines, `--no-fail-fast` before counting a red run, the table-and-builder search before calling a check unaffected)
- **Platform issues consulted:** none — no runner-only bullet: Setup 5a read no red and no `not green` (`a70313687` in progress, `495067f8` green 16/16)
- **External inputs:** `inputs#I1` — the orchestrator's git measurement at take-up: `upstream/main` at `434e06615`, 16 commits past `7832c177`, the trial merge's two conflicts and our surface re-measured · `inputs#I2` — the operator relays the founder: no mutation runs; pipeline tool output redirected to a file and read whole

## Files inspected
- `packages/dioxus-native-dom/src/mutation_writer.rs` (upstream's diff since `7832c177`, full; HEAD 1-40, 400-445) — #1185 replaces `docm.replace_node_with` / `docm.remove_node` in the three `WriteMutations` arms with `remove_and_drop_node` / `replace_and_drop_node`, which drop the node and clear its `element_id_mapping` / `node_id_mapping` entries; #1186 builds every attribute name as `qual_name(local_name, Some(ns.unwrap_or("")))`; #1188's `is_bool_attr` is a `matches!` over 28 names — ours (`BOOLEAN_ATTRIBUTES`, 27) plus `inert` (both lists read whole) — and upstream adds two unit tests of it in the same file.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (upstream's diff, full; HEAD tail; trial-merged tree `2481e052` tail) — upstream drops `DEFAULT_CSS` and `doc.add_user_agent_stylesheet(DEFAULT_CSS)` from `DioxusDocument::new` and appends four tests inside `mod tests`; our four hunks (the import at :18, the id walk at :185, the `accessibility_tree` override at :296, the `#[path = "dioxus_document_tests.rs"] mod bridge_tests;` declaration at the file's end) meet none of upstream's — in the trial merge `bridge_tests` stands at the merged file's end (:780-782).
- `packages/dioxus-native-dom/src/lib.rs` (the conflict region in `2481e052`, :32-42) — one `pub use events::{…}` line: ours keeps three `#[cfg(feature = "accessibility")]` re-exports below it, upstream adds `synthetic_form_event` to it.
- `packages/dioxus-native-dom/src/element_id.rs` (HEAD :396-431, `a_removed_element_reads_no_id`) — asserts a node a re-render removes still resolves, detached (`get_node(removed).map(|n| n.parent) == Some(None)`, :421-422), then drops it through `remove_node_if_unparented` (:425-428).
- `packages/dioxus-native-dom/src/snapshot.rs` (HEAD :520-560) — a unit test writes `disabled` through `crate::qual_name("disabled", None)` (the HTML namespace) and reads it back by local name.
- `packages/blitz-dom/src/document.rs` (upstream's diff, full) — #1189: `UserAgentStylesheet` (parsed once, a `LazyLock` default), `BaseDocument::ua_stylesheets` removed, `add_user_agent_stylesheet(UserAgentStylesheet)` / `remove_user_agent_stylesheet(&UserAgentStylesheet)` replace the `&str` forms; #1176/#1177: the cursor no longer set inside `set_hover_to` / `clear_hover`, a new `update_cursor` after the event.
- `packages/blitz-dom/src/config.rs` (upstream's diff, full) — `DocumentConfig.ua_stylesheets` becomes `Vec<UserAgentStylesheet>`, defaulting to `vec![UserAgentStylesheet::default_sheet()]` through a hand-written `Default`.
- `packages/blitz-dom/src/node/node.rs` (upstream's diff, full) — #1175: `display_style`, `taffy_display` and `is_column_flex_container` treat a button input as a flow root; `IS_BUTTON` / `IS_BUTTON_INPUT` style flags.
- `packages/blitz-dom/src/node/element.rs` (HEAD :467-475) — `attr` / `attr_parsed` match `attr.name.local` only, never the namespace; `node.rs:895` and `:1097` delegate the same way.
- `packages/blitz-dom/src/stylo.rs` (HEAD :441-458, `attr_matches`) — an attribute selector matches when the local name is equal AND, for `NamespaceConstraint::Specific(ns)`, `attr.name.ns == ns`.
- `packages/blitz-dom/src/mutator.rs` (upstream's diff) — #1187: removing an `input`'s `checked` sets `set_checkbox_input_checked(false)`.
- `packages/blitz-dom/src/events/driver.rs` (upstream's diff) — `UiEvent::PointerLeave` dispatches through `handle_pointer_leave` and ends hover; no mutator call.
- `packages/blitz-paint/src/render.rs` (upstream's diff, full) — #1194/#1195 change `ElementCx`'s `svg` field type, `draw_svg` and `draw_image`, and add `with_replaced_content_clip`.
- `packages/blitz-shell/src/window.rs` (upstream's diff; HEAD :366-392) — upstream sends `UiEvent::PointerLeave` from the `PointerLeft` arm (base :729); our refresh lives in `View::poll` (:370-388), which takes the changed set and rebuilds the tree only when it is non-empty.
- `packages/blitz-traits/src/events.rs` (upstream's diff) — `UiEvent` gains the `PointerLeave` variant.
- `packages/blitz-vibey-script/src/document.rs` (ours and upstream's diffs) — ours rewrites three doc-comment lines (:110, :122, :255); upstream deletes the `DEFAULT_CSS` block at :55-63 and its import.
- `packages/blitz-test-harness/src/harness.rs` (HEAD :50-70) — the harness builds its `DocumentConfig` with `..Default::default()`, so `ua_stylesheets` takes upstream's new default.
- `tests/blitz-tests/tests/{anonymous_block_restyle,button_alignment,checked_attribute,pointer_leave,ua_stylesheet_order}.rs` (at `upstream/main`, their `use` / `mod` / `#[test]` lines) — no `super::`, `crate::` or `mod` line; they import blitz crates, `blitz_test_harness::{Harness, HarnessOptions, pointer_event}` and `cursor_icon` only.
- `tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs` (HEAD :1-45) — the falsy check of ours, its doc citing Dioxus 0.7.10's list.
- `Cargo.toml` / `tests/blitz-tests/Cargo.toml` / `Cargo.lock` (at `7832c177`, HEAD, `upstream/main`, the trial merge) — see Patterns.
- `README.md` (HEAD :40-55) — :48 names "the ten driver commands as tools" and "Its screenshot is still to come".
- `packages/blitz-dom/assets/default.css` (at `upstream/main`, its attribute selectors) and the lean tasks' typed attributes.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T14-26-44-phase/tree-query-2026-10-13-upstream-sync-polish-and-ship.json`, the rust plane regenerated at the query, 7748 nodes)
- **add_user_agent_stylesheet** — 8 callers: `BaseDocument::new` @ packages/blitz-dom/src/document.rs:524 and :527, five `hover_invalidation_tests` @ document.rs:2995 · 3082 · 3151 · 3230 · 3286, `DioxusDocument::new` @ packages/dioxus-native-dom/src/dioxus_document.rs:98 — every one in a file upstream's delta edits, and upstream rewrites each; no escher-owned caller.
- **ua_stylesheets** (refs) — 14 references, all in `apps/browser/src/{document_loader,toolbar}.rs`, `apps/readme/src/{main,readme_application}.rs`, `packages/blitz-dom/src/{document,iframe}.rs`, `packages/blitz-html/src/html_document.rs`, `packages/blitz-vibey-script/src/document.rs`, `packages/blitz/src/lib.rs` — every one a file upstream's delta edits; none in escher's crates, the harness, the stand or our checks.
- **take_changed_nodes** — 19 call sites: 15 in blitz-dom's `changed_set_tests` (document.rs:3460-3634, among them `changed_set_drained_id_of_a_dropped_node_reads_none` @ :3627, :3634), `View::poll` @ packages/blitz-shell/src/window.rs:376 (emptiness only), and three checks of ours that drain and discard (`stand_act_screenshot.rs:283`, `stand_diff.rs:103`, `stand_settle.rs:92`). No consumer indexes a drained id, so a dropped node in the set reads nowhere.
- **BOOLEAN_ATTRIBUTES** — 1 reference, `set_attribute_inner` @ packages/dioxus-native-dom/src/mutation_writer.rs:439 — the conflicted line; the constant leaves with it.
- **qual_name** (dioxus-native-dom's) — 12 call sites in `dioxus_document.rs` (:8, :114-130, :172, :177), `mutation_writer.rs` (:2, :382, :433) and one unit test `snapshot.rs:543`; upstream changes only the argument at `set_attribute_inner`, not the function.
- **remove_node** (`DocumentMutator`) — 19 callers; the bridge's own @ mutation_writer.rs:225 is the one #1185 replaces; the rest (blitz-html's sink, blitz-vibey-script's DOM, `incremental_oracle`, `oof_dynamic_cb`) are untouched by the delta.
- **crate_edges into dioxus-native-dom** — escher-driver, seven_guis, blitz-test-harness and blitz-tests read it (beside the examples and apps): the #1185/#1186/#1188 bridge changes reach every escher surface through the one crate.

## Patterns detected
- **The lock moves narrowly** (`Cargo.lock` at `7832c177` vs `upstream/main`, name@version by `awk` over `name`/`version` lines): upstream's delta changes the eleven in-repo packages 0.3.0-beta.2 → 0.3.0-beta.3 and `taffy` 0.14.0 (git `7d33901c`) → 0.15.0 (registry); no third-party package enters or leaves. The root manifest's `taffy` entry keeps its eight features byte for byte (`std flexbox flexbox_balance grid block_layout content_size calc detailed_layout_info`), only its source moving. `members` is unchanged by upstream (`diff` of the two `members` arrays empty).
- **The merged lock does not resolve as merged** (`git show 2481e052:Cargo.lock`): it lists `escher-driver@0.3.0-beta.2` and `escher-telemetry@0.3.0-beta.2` beside upstream's beta.3 set, while both crates take `version.workspace = true` (`packages/escher-driver/Cargo.toml:5`, `packages/escher-telemetry/Cargo.toml:5`) and the workspace version becomes 0.3.0-beta.3 — so a `--locked` build refuses the textually merged lock until those two entries read beta.3. `seven_guis` keeps its own `0.1.0` (`examples/seven_guis/Cargo.toml:3`). No conflict marker in the merged lock (count 0).
- **Attribute reads by name ignore the namespace** (element.rs:467-475, node.rs:895, :1097): `element_id`'s author key, the accessibility builder's `aria-selected` · `aria-pressed` · `aria-invalid` reads and every `attr(...)` read go by local name, so #1186's namespace change moves none of them.
- **CSS attribute selectors are namespace-exact** (stylo.rs:451-455): an unprefixed `[name…]` selector matches only an attribute in the empty namespace; Dioxus attributes were written in the HTML namespace (`qual_name`'s default, dioxus-native-dom lib.rs:44-50), so before #1186 no unprefixed attribute selector matched a Dioxus element, and after it they do. The UA sheet's unprefixed attribute selectors (default.css at `upstream/main`: `[dir=…]`, `[popover]`, `a[href]`, `input[type=…]` for tel · hidden · checkbox · radio · submit · reset · button · file · image, the `table[…]` family, `details[open]`, `dialog[open]`, `audio[controls]`) start matching on the stand.
- **Hunks of the two sides lie apart in every shared engine file** (`git diff -U0` hunk headers, base line numbers): `render.rs` ours :901-1041 (the mask in the text-input path), upstream :655-711 and :1085-1216 (SVG and image object-fit); `document.rs` ours :150, :316, :850, :1000-1002, :1544, :2234-2359 (the bounds reader), :3330, upstream :42-515, :1128-1179, :1854-2163, :2911-3205; `node/node.rs` ours :131, :1358-1439, :1660 (`hit_inner`, the position reader), upstream :473-506, :1122-1197 (button display); `dioxus_document.rs` ours :17, :182, :259, :412, upstream :7-13, :92-98, :366-369, :409+.
- **A removed Dioxus node is dropped, not detached** (#1185, mutation_writer.rs at `upstream/main` :112-130): the node and its descendants leave the slab at removal and their id mappings are cleared — where before a removed node stayed resolvable with `parent == None` until its ElementId was reused.
- **The default UA sheet is added once** (document.rs and config.rs at `upstream/main`): `BaseDocument::new` appends `config.ua_stylesheets` (default: the default sheet) and `add_user_agent_stylesheet` refuses a sheet already present; before, a `DioxusDocument` received `DEFAULT_CSS` twice (once from `BaseDocument::new` for a `None` config, once from `DioxusDocument::new`).

## Conventions to follow
- **A merge is one merge commit** on the build branch, never a rebase (`f00b0216`, `9462a7e4`; scope).
- **Conflict resolutions and gate fixes are named and minimal**; an edit to upstream-owned code beyond a conflict is recorded non-additive (intent §Principles; scope).
- **Integration tests are files of their own** directly under `tests/blitz-tests/tests/`, auto-discovered, with no manifest line (`tests/blitz-tests/Cargo.toml` comment at the `[[test]] all` table; testing.md).
- **A moved measured number is re-measured and recorded, never loosened** (scope; the layouts and tests extracts).
- **Counting a cargo run**: one merged stream (`2>&1`), `--no-fail-fast` before counting a red run (testing.md Session Additions 2026-10-09, 2026-10-10).

## New files to create
- `escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/evidence/` — the chunk's evidence: the merge record, the lock review, the counts, the moved readings, the CI run

## Files to modify
- `.github/scripts/test_wpt_area_changes.py` — upstream's, by the merge
- `.github/scripts/wpt_area_changes.py` — upstream's, by the merge
- `Cargo.lock` — the merge, then the two escher workspace entries to 0.3.0-beta.3
- `Cargo.toml` — upstream's, by the merge
- `README.md` — eleven driver verbs and the built screenshot
- `apps/browser/src/document_loader.rs` — upstream's, by the merge
- `apps/browser/src/toolbar.rs` — upstream's, by the merge
- `apps/readme/src/main.rs` — upstream's, by the merge
- `apps/readme/src/readme_application.rs` — upstream's, by the merge
- `packages/blitz-dom/assets/default.css` — upstream's, by the merge
- `packages/blitz-dom/src/config.rs` — upstream's, by the merge
- `packages/blitz-dom/src/document.rs` — upstream's, by the merge
- `packages/blitz-dom/src/events/driver.rs` — upstream's, by the merge
- `packages/blitz-dom/src/events/mod.rs` — upstream's, by the merge
- `packages/blitz-dom/src/iframe.rs` — upstream's, by the merge
- `packages/blitz-dom/src/layout/construct.rs` — upstream's, by the merge
- `packages/blitz-dom/src/layout/damage.rs` — upstream's, by the merge
- `packages/blitz-dom/src/layout/inline.rs` — upstream's, by the merge
- `packages/blitz-dom/src/layout/table.rs` — upstream's, by the merge
- `packages/blitz-dom/src/lib.rs` — upstream's, by the merge
- `packages/blitz-dom/src/mutator.rs` — upstream's, by the merge
- `packages/blitz-dom/src/node/element.rs` — upstream's, by the merge
- `packages/blitz-dom/src/node/node.rs` — upstream's, by the merge
- `packages/blitz-dom/src/node/svg.rs` — upstream's, by the merge
- `packages/blitz-html/src/html_document.rs` — upstream's, by the merge
- `packages/blitz-paint/src/render.rs` — upstream's, by the merge
- `packages/blitz-paint/src/render/background.rs` — upstream's, by the merge
- `packages/blitz-shell/src/window.rs` — upstream's, by the merge
- `packages/blitz-traits/src/events.rs` — upstream's, by the merge
- `packages/blitz-vibey-script/Cargo.toml` — upstream's, by the merge
- `packages/blitz-vibey-script/src/document.rs` — upstream's, by the merge
- `packages/blitz/src/lib.rs` — upstream's, by the merge
- `packages/dioxus-native-dom/src/dioxus_document.rs` — upstream's, by the merge
- `packages/dioxus-native-dom/src/element_id.rs` — `a_removed_element_reads_no_id` to the dropped-node lifetime
- `packages/dioxus-native-dom/src/events.rs` — upstream's, by the merge
- `packages/dioxus-native-dom/src/lib.rs` — the conflict: both sides' re-exports
- `packages/dioxus-native-dom/src/mutation_writer.rs` — the conflict: upstream's `is_bool_attr`, our constant dropped
- `packages/stylo_taffy/src/convert.rs` — upstream's, by the merge
- `packages/stylo_taffy/src/wrapper.rs` — upstream's, by the merge
- `tests/blitz-tests/Cargo.toml` — upstream's `cursor-icon` dev-dependency, by the merge
- `tests/blitz-tests/tests/all.rs` — upstream's, by the merge (still `test = false`)
- `tests/blitz-tests/tests/anonymous_block_restyle.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/button_alignment.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/checked_attribute.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/pointer_leave.rs` — upstream's new file, by the merge
- `tests/blitz-tests/tests/ua_stylesheet_order.rs` — upstream's new file, by the merge
- `wpt/WPT_COMMIT` — upstream's, by the merge

## Sweeps
- `add_user_agent_stylesheet` (graph `calls`, rust plane): 8 sites · 0 changed by us · 8 no-change (each in a file upstream's delta rewrites)
- `ua_stylesheets` (graph `refs`, rust plane): 14 references · 0 changed by us · 14 no-change (each in a file upstream's delta rewrites)
- `DocumentConfig {` over escher-owned sources (`git grep` over packages/escher-driver, packages/blitz-test-harness, examples/seven_guis/src, the two shared test modules): 2 hits · 0 changed · 2 no-change (`harness.rs:56`, `settle.rs:435` — both close with `..Default::default()`)
- `Some(None)` over packages/dioxus-native-dom/src, packages/escher-driver/src, `stand_*.rs`, `dioxus_*.rs`: 2 hits · 1 changed (`element_id.rs:422`, the detached-node premise #1185 ends) · 1 no-change (`mcp.rs:276`, a JSON-RPC id)
- `detached` / `remove_node_if_unparented` / `only detaches` over packages/dioxus-native-dom/src, `stand_id_persistence.rs`, packages/escher-driver/src: 4 hits · 1 changed (`element_id.rs:421-428`) · 3 no-change (`dioxus_document.rs:208`'s doc still true; `mutation_writer.rs:162`, upstream's own call)
- `0\.3\.0-beta\.2` over the tree outside manifests, `Cargo.lock`, `.andromeda/` and `escher-0.1.0/`: 2 hits · 0 changed · 2 no-change (`.claude/docs/stack.md:7` a derived doc the wrap's cascade re-derives; `.github/scripts/test_agent_run.py:19` a cargo-shim fixture line no assertion compares with the real version)
- `service.version` over `tests/`, `examples/seven_guis/tests/`, packages/escher-telemetry/src: 6 hits · 0 changed · 6 no-change (`host_binary.rs:99` asserts the key's presence, not a value; `format.rs:590-592` reads `env!("CARGO_PKG_VERSION")`)
- upstream's five new test files, `env::var|env!|option_env!` · `println!|eprintln!|print!` · `tracing_subscriber|set_global_default|escher_telemetry`: 0 · 0 · 0 in each
- upstream's Rust delta (`git diff 7832c177 upstream/main -- '*.rs'`), added lines with `TcpListener|UdpSocket|UnixListener|bind(|listen(|env::var|std::env`: 0; added log or print sites under packages/ (`tracing::|trace!|debug!|info!|warn!|error!|println!|eprintln!|log::`): 0 — control: the same diff's added `fn` lines read 43
- upstream's delta under `.github/workflows/`: 0 files; under `.github/scripts/`: 2 files, `test_wpt_area_changes.py` +1 test (`def test_` lines: 78 at HEAD, 79 in the trial merge)
- `tests/blitz-tests/tests/*.rs` at the top level (`git ls-tree --name-only`): 113 at HEAD (112 targets and the unbuilt `all.rs`), 118 in the trial merge

## Open questions
- none
