group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/gotchas.md:41   class: stale
  leaf-text: «`:focus-visible`, `:focus-within`, `:valid`, `:invalid`, `:required`, `:target` always match false;»
  source: .andromeda/architecture.md:85 «while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (packages/blitz-dom/src/stylo.rs:477-479; PROVISIONAL, [DOM semantics])»
  replace-with: «`:focus-visible`, `:valid`, `:invalid`, `:required`, `:target` always match false, while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (PROVISIONAL);»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «its fix owned by a `.andromeda/residuals.md` line (target: next).»
  source: .andromeda/architecture.md:103 «Focus keeps `:focus-within` on its ancestors and asks for a frame (PROVISIONAL): `BaseDocument::set_focus_to` returns `false` for a node not in the document, before its `Focussed node` INFO line»
  replace-with: «its fix owned by a `.andromeda/residuals.md` line (target: next). Focus keeps `:focus-within` on its ancestors and asks for a frame (PROVISIONAL; upstream #1226): `BaseDocument::set_focus_to` returns `false` for a node not in the document, moves the `FOCUS_WITHIN` element state from the old focus's element ancestors to the new focus's (the shared prefix keeping it) and requests a redraw; `clear_focus` removes it from the cleared node's element ancestors and requests a redraw; a removed focus clears its chain, and `DocumentMutator::add_children_to_parent` carries the state with a moved subtree that holds the focus, or only clears the old chain when the focus left with it; the Tab key asks for no redraw of its own.»

- leaf: .claude/docs/services/blitz-dom.md:19   class: omission
  leaf-text: «by a focus or checked change (`snapshot_node_and` with `FOCUS` / `CHECKED`)»
  source: .andromeda/architecture.md:103 «The `FOCUS_WITHIN` writes go through `snapshot_node_and` and mark no changed-set entry, so a focus move still marks exactly the node that lost focus and the node that gained it»
  replace-with: «by a focus or checked change (`snapshot_node_and` with `FOCUS` / `CHECKED`; the `FOCUS_WITHIN` writes on the focus's ancestors go through `snapshot_node_and` and mark no entry, so a focus move still marks exactly the node that lost focus and the node that gained it — PROVISIONAL)»
