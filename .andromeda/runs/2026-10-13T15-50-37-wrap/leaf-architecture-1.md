group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/conventions.md:19   class: omission
  leaf-text: «App and example crates, blitz-test-harness, escher-telemetry and escher-driver set `publish = false`»
  source: .andromeda/architecture.md:224 «app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`»
  replace-with: «App and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`»

- leaf: .claude/docs/gotchas.md:7   class: stale
  leaf-text: «taffy is a git dep pinned by `rev` and parley a registry version (0.12)»
  source: .andromeda/architecture.md:73 «taffy is a registry version, "0.15" (resolved 0.15.0, its eight features unchanged), since 2026-10-13-upstream-sync-polish-and-ship — a git dependency pinned by `rev` until then»
  replace-with: «taffy is a registry version (0.15) since 2026-10-13-upstream-sync-polish-and-ship — a git dep pinned by `rev` until then, so no git dependency is left in the manifest — and parley a registry version (0.12)»

- leaf: .claude/docs/gotchas.md:16   class: stale
  leaf-text: «`node_id_mapping` entries are deliberately not cleared on removal so dioxus-core can reuse detached nodes.»
  source: .andromeda/architecture.md:91 «a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves»
  replace-with: «Since 2026-10-13-upstream-sync-polish-and-ship (upstream #1185) a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves; until then the mappings were not cleared so dioxus-core could reuse detached nodes.»

- leaf: .claude/docs/services/blitz-dom.md:12   class: stale
  leaf-text: «Taffy (git rev) via `stylo_taffy`»
  source: .andromeda/architecture.md:48 «taffy "0.15" from the crates.io registry (resolved 0.15.0; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship)»
  replace-with: «Taffy 0.15 (crates.io registry since 2026-10-13-upstream-sync-polish-and-ship, a git rev until then) via `stylo_taffy`»

- leaf: .claude/docs/services/blitz-dom.md:25   class: stale
  leaf-text: «`node_id_mapping` is not cleared on removal.»
  source: .andromeda/architecture.md:91 «a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves»
  replace-with: «a node the Dioxus bridge removes is dropped with its descendants and its `node_id_mapping` entries cleared (upstream #1185, since 2026-10-13-upstream-sync-polish-and-ship), so it no longer resolves.»

- leaf: .claude/docs/services/dioxus-native-dom.md:23   class: stale
  leaf-text: «one of the 27 boolean attributes of `BOOLEAN_ATTRIBUTES` (`mutation_writer.rs` — the `isBoolAttr` list of Dioxus 0.7.10's web interpreter: `checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and 18 more)»
  source: .andromeda/architecture.md:136 «of one of the 28 names of upstream's private predicate `is_bool_attr`, a closed `matches!` set»
  replace-with: «one of the 28 names of upstream's private predicate `is_bool_attr` (`mutation_writer.rs`, a closed `matches!` set: `checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus`, `inert` and 18 more — escher's own 27-name `BOOLEAN_ATTRIBUTES` list retired for it at 2026-10-13-upstream-sync-polish-and-ship, upstream #1188; a framework-written `inert: false` is now cleared)»

- leaf: .claude/docs/services/dioxus-native-dom.md:24   class: stale
  leaf-text: «`<html><head></head><body><main id="main"></main></body></html>` with `DEFAULT_CSS`;»
  source: .andromeda/architecture.md:98 «`DioxusDocument::new` adds no stylesheet of its own: the default user-agent sheet reaches it once, through `DocumentConfig::default()`, and a config that sets `ua_stylesheets` is used as given»
  replace-with: «`<html><head></head><body><main id="main"></main></body></html>`; `DioxusDocument::new` adds no stylesheet of its own — the default user-agent sheet arrives once through `DocumentConfig::default()`, and a config that sets `ua_stylesheets` is used as given (upstream #1189, since 2026-10-13-upstream-sync-polish-and-ship; until then `DEFAULT_CSS` was always added);»

- leaf: .claude/docs/services/dioxus-native-dom.md:33   class: stale
  leaf-text: «A node a re-render removes is only DETACHED (parent `None`) and keeps resolving until its ElementId is reassigned; `element_id` reads `None` for it, but `get_node` still returns it.»
  source: .andromeda/architecture.md:136 «a node a re-render removes is dropped with its descendants by the bridge's crate-private `remove_and_drop_node` / `replace_and_drop_node`, their `element_id_mapping` / `node_id_mapping` entries cleared»
  replace-with: «A node a re-render removes is dropped with its descendants by the crate-private `remove_and_drop_node` / `replace_and_drop_node`, its `element_id_mapping` / `node_id_mapping` entries cleared, so it no longer resolves — `get_node` `None`, `element_id` `None`, no pair in `element_ids()` (upstream #1185, since 2026-10-13-upstream-sync-polish-and-ship; until then it was only DETACHED and kept resolving until its ElementId was reassigned).»

- leaf: .claude/docs/services/dioxus-native-dom.md:35   class: stale
  leaf-text: «An attribute outside the 27-name boolean list»
  source: .andromeda/architecture.md:136 «of one of the 28 names of upstream's private predicate `is_bool_attr`, a closed `matches!` set»
  replace-with: «An attribute outside the 28-name `is_bool_attr` set»

- leaf: .claude/docs/services/dioxus-native-dom.md:41   class: omission
  leaf-text: «`dioxus_falsy_boolean_attrs.rs` the 27-name list.»
  source: .andromeda/architecture.md:263 «(the bridge's predicate upstream's 28-name `is_bool_attr` since 2026-10-13-upstream-sync-polish-and-ship; whether a check reads a falsy `inert` is not measured here)»
  replace-with: «`dioxus_falsy_boolean_attrs.rs` the 27 names of its own list (the bridge's predicate is upstream's 28-name `is_bool_attr` since 2026-10-13-upstream-sync-polish-and-ship; whether a check reads a falsy `inert` is not measured).»
