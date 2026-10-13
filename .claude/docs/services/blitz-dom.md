# blitz-dom

_Crate notes. Primary source: `.andromeda/architecture.md` (§Standard Contracts, §Existing Scopes, §Infrastructure Patterns)._

## Responsibility
The headless DOM (`BaseDocument`): node tree, CSS parsing and resolution through Stylo, damage, box construction, Taffy/Parley layout, paint-tree topology, event dispatch, scrolling, selection, CSSOM, forms, iframes and the AccessKit tree. It does NOT parse HTML (blitz-html), fetch (blitz-net), paint (blitz-paint) or own windows (blitz-shell). Driven by external code; most users go through a wrapper (`HtmlDocument`, `DioxusDocument`, `ScriptDocument`).

## Key integrations

### Consumes from
- Provider traits from blitz-traits via `DocumentConfig`: `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (`Dummy*` defaults).
- Stylo 0.22, Taffy 0.15 (crates.io registry since 2026-10-13-upstream-sync-polish-and-ship, a git rev until then) via `stylo_taffy`, Parley 0.12 (registry), ICU4X 2.3, accesskit (feature `accessibility`).

### Publishes to
- `Document` trait (`inner`, `inner_mut`, `handle_ui_event`, `poll`, `id`, and under `accessibility` a default `accessibility_tree` a wrapper may override), `BaseDocument`, `DocumentMutator`, `EventDriver`/`EventHandler`, query-selector API, CSSOM, scroll API (`scroll_into_view` scrolls every scrolling box that holds its target, innermost first, then the viewport — a nested box at once, the requested behaviour for the viewport alone; `visible_region` reads the part of the viewport an element can be seen through — never a hit), the hit walk (it tests a node's own area where the box stands and stops at the padding box of a box that clips by `overflow`, the root element excepted; it still reaches content clipped by `contain: paint`), `Node::absolute_position` (for a box that is itself scrolled, its position less its own scroll offset — measured and pinned, not fixed, while `get_client_bounding_rect` reads where the box stands), `build_accessibility_tree` (names from text children, a trimmed-non-empty `aria-label` and `<label>` association; since 2026-10-10-snapshot-states three states read by value on every element, whatever its role — `aria-selected` → `selected`, `aria-pressed` → `toggled`, `aria-invalid` → `invalid`, each a closed-set match on the trimmed lower-case value, an unrecognized `aria-invalid` reading `Invalid::True`, `aria-checked` unread, no log site and no panic path — a direct engine change for every document, ratified by the founder and flagged upstreamable; check `accessibility_states`), `BoundingRect`, CSSOM View accessors, `Widget` trait, and since 2026-10-12-headless-screenshot two public read-only methods: `BaseDocument::can_paint_text(&self) -> bool` (whether the font context resolves a family for the `sans-serif` generic, through fontique's `generic_families`) and `BaseDocument::layout_as_text_input(&self, node_id, text) -> Option<parley::Layout<TextBrush>>` (`text` laid out with a text input's editor styles, scale and direction in the document's own fonts, one unbroken line, a fresh `LayoutContext` per call, `None` for a node that is no text input — its one caller is blitz-paint's password mask).

## Internal conventions
- `resolve` order: messages → critical-resource gate → scroll animation → device changes → stylist → damage → layout-children construction → deferred tasks → style images → layout → transforms → paint tree → clear damage → hover refresh → sub-documents.
- The changed set (`changed_nodes`): nodes written since the last drain. `has_changes()` reads true while it is non-empty; `take_changed_nodes()` returns it and leaves it empty (a returned id may name a dropped node — read it through `get_node`). It is marked by the 15 mutation-flag sites of `DocumentMutator` (in-document nodes only), by a focus or checked change (`snapshot_node_and` with `FOCUS` / `CHECKED`) and by a text control's input — the last two wherever the node is — and NOT by node creation, hover, active, scroll, resize, resource loads, animation or layout, so an idle `resolve` marks nothing in either layout mode. It is engine-touched, not a diff: the snapshot diff (`Snapshot::diff` in dioxus-native-dom) compares two snapshots and never reads it. Ratified by the founder, 2026-10-07 (it changes behaviour for every Blitz document); the windowed shell is its one non-test drainer.
- Default features: svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu. `writing-mode` (vertical writing modes) is opt-in: a workspace-wide build turns it on, a `-p blitz-tests` build does not, and it selects which of two bodies of the bounds reader `physical_unrounded_geometry` is compiled (test-plan §9 → Engine features by runner).
- Layout runs over a crate-private `LayoutPassState`, which wraps the document for one pass and carries Taffy's tree traits; `BaseDocument` implements none of them.

## Crate-specific gotchas
- `StyleThreading::Parallel` panics with two concurrent documents; `LAYOUT_CTX` is thread-local.
- Stale `NodeId` indexing panics; a node the Dioxus bridge removes is dropped with its descendants and its `node_id_mapping` entries cleared (upstream #1185, since 2026-10-13-upstream-sync-polish-and-ship), so it no longer resolves.
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
