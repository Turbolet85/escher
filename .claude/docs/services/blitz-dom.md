# blitz-dom

_Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts, §Existing Scopes, §Infrastructure Patterns)._

## Responsibility
The headless DOM (`BaseDocument`): node tree, CSS parsing and resolution through Stylo, damage, box construction, Taffy/Parley layout, paint-tree topology, event dispatch, scrolling, selection, CSSOM, forms, iframes and the AccessKit tree. It does NOT parse HTML (blitz-html), fetch (blitz-net), paint (blitz-paint) or own windows (blitz-shell). Driven by external code; most users go through a wrapper (`HtmlDocument`, `DioxusDocument`, `ScriptDocument`).

## Key integrations

### Consumes from
- Provider traits from blitz-traits via `DocumentConfig`: `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (`Dummy*` defaults).
- Stylo 0.22, Taffy (git rev) via `stylo_taffy`, Parley (git rev), accesskit (feature `accessibility`).

### Publishes to
- `Document` trait (`inner`, `inner_mut`, `handle_ui_event`, `poll`, `id`), `BaseDocument`, `DocumentMutator`, `EventDriver`/`EventHandler`, query-selector API, CSSOM, scroll API, `build_accessibility_tree`, `BoundingRect`, CSSOM View accessors, `Widget` trait.

## Internal conventions
- `resolve` order: messages → critical-resource gate → scroll animation → device changes → stylist → damage → layout-children construction → deferred tasks → style images → layout → transforms → paint tree → clear damage → hover refresh → sub-documents.
- `changed_nodes` is the set of changed nodes for accessibility updates — the natural seed for snapshot diffs.
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
