# blitz-dom

_Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts, §Existing Scopes, §Infrastructure Patterns)._

## Responsibility
The headless DOM (`BaseDocument`): node tree, CSS parsing and resolution through Stylo, damage, box construction, Taffy/Parley layout, paint-tree topology, event dispatch, scrolling, selection, CSSOM, forms, iframes and the AccessKit tree. It does NOT parse HTML (blitz-html), fetch (blitz-net), paint (blitz-paint) or own windows (blitz-shell). Driven by external code; most users go through a wrapper (`HtmlDocument`, `DioxusDocument`, `ScriptDocument`).

## Key integrations

### Consumes from
- Provider traits from blitz-traits via `DocumentConfig`: `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (`Dummy*` defaults).
- Stylo 0.22, Taffy (git rev) via `stylo_taffy`, Parley (git rev), accesskit (feature `accessibility`).

### Publishes to
- `Document` trait (`inner`, `inner_mut`, `handle_ui_event`, `poll`, `id`, and under `accessibility` a default `accessibility_tree` a wrapper may override), `BaseDocument`, `DocumentMutator`, `EventDriver`/`EventHandler`, query-selector API, CSSOM, scroll API, `build_accessibility_tree` (names from text children, a trimmed-non-empty `aria-label` and `<label>` association), `BoundingRect`, CSSOM View accessors, `Widget` trait.

## Internal conventions
- `resolve` order: messages → critical-resource gate → scroll animation → device changes → stylist → damage → layout-children construction → deferred tasks → style images → layout → transforms → paint tree → clear damage → hover refresh → sub-documents.
- The changed set (`changed_nodes`): nodes written since the last drain. `has_changes()` reads true while it is non-empty; `take_changed_nodes()` returns it and leaves it empty (a returned id may name a dropped node — read it through `get_node`). It is marked by the 15 mutation-flag sites of `DocumentMutator` (in-document nodes only), by a focus or checked change (`snapshot_node_and` with `FOCUS` / `CHECKED`) and by a text control's input — the last two wherever the node is — and NOT by node creation, hover, active, scroll, resize, resource loads, animation or layout, so an idle `resolve` marks nothing in either layout mode. It is engine-touched, not a diff: the snapshot diff (`Snapshot::diff` in dioxus-native-dom) compares two snapshots and never reads it. Provisional, pending the founder's word (it changes behaviour for every Blitz document); the windowed shell is its one non-test drainer.
- Default features: svg, woff, accessibility, system-fonts, file-input, custom-widget.

## Crate-specific gotchas
- `StyleThreading::Parallel` panics with two concurrent documents; `LAYOUT_CTX` is thread-local.
- Stale `NodeId` indexing panics; `node_id_mapping` is not cleared on removal.
- Interaction state references DOM nodes only and is retargeted before a node is freed.
See `.claude/docs/gotchas.md`.

## Entry points for modification
- **Document core:** `src/document.rs`, `src/config.rs`, `src/resolve.rs`
- **Mutation:** `src/mutator.rs` · **Tree:** `src/tree.rs`, `src/traversal.rs`
- **Events:** `src/events/{driver,focus,keyboard,pointer,ime}.rs`
- **Layout:** `src/layout/{mod,construct,damage,inline,paint_tree,table,replaced}.rs`
- **Accessibility:** `src/accessibility.rs` · **Nodes:** `src/node/`
- **Tests:** in-file `#[cfg(test)]` (document.rs, mutator.rs, net.rs, …) + `tests/blitz-tests/tests/`

## Testing this crate
- **Unit:** `cargo test -p blitz-dom` (SVG tests need the `svg` feature)
- **Integration:** `cargo test -p blitz-tests`
- **Fixtures:** DOM built by hand through `DocumentMutator` (blitz-html would be a circular dev-dependency)

## References
- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `.claude/docs/conventions.md`
