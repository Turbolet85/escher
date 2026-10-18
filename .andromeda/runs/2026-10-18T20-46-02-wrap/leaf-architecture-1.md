group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «the document's clock (PROVISIONAL: `DocumentConfig.clock: Option<Arc<dyn Clock>>`»
  source: .andromeda/architecture.md:139 «- The document's clock (ratified by the founder, 2026-10-18): `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`»
  replace-with: «the document's clock (ratified by the founder, 2026-10-18: `DocumentConfig.clock: Option<Arc<dyn Clock>>`»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «a computed `contain` that includes `paint` (that half PROVISIONAL)»
  source: .andromeda/architecture.md:145 «the `contain: paint` half ratified by the founder (2026-10-18)»
  replace-with: «a computed `contain` that includes `paint` (that half ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`center_of` among them — moving with it (PROVISIONAL))»
  source: .andromeda/architecture.md:145 «`scroll_into_view` and the test harness's `layout_rect`, `layout_rect_of` and `center_of`, which read such a box where it stands (ratified by the founder, 2026-10-18)»
  replace-with: «`center_of` among them — moving with it (ratified by the founder, 2026-10-18))»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «as CSSOM View's `getBoundingClientRect` does (PROVISIONAL)»
  source: .andromeda/architecture.md:149 «- The client rect under transforms (ratified by the founder, 2026-10-18): `BaseDocument::get_client_bounding_rect` and `node_client_rects` read an element where its transforms move it»
  replace-with: «as CSSOM View's `getBoundingClientRect` does (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «(the toggle read per role PROVISIONAL)»
  source: .andromeda/architecture.md:143 «the toggle read per role ratified by the founder (2026-10-18)»
  replace-with: «(the toggle read per role ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`disabled` is read by presence on a form control alone (PROVISIONAL)»
  source: .andromeda/architecture.md:98 «`disabled` is read one way, by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18)»
  replace-with: «`disabled` is read by presence on a form control alone (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: stale
  leaf-text: «`inert` is honoured by its presence on an element or an ancestor (PROVISIONAL)»
  source: .andromeda/architecture.md:102 «- `inert` is honoured by its presence on the element or an ancestor element (ratified by the founder, 2026-10-18)»
  replace-with: «`inert` is honoured by its presence on an element or an ancestor (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/blitz-paint.md:18   class: stale
  leaf-text: «a double click selects the whole value (PROVISIONAL; `password_caret_click`)»
  source: .andromeda/architecture.md:164 «one click at one offset into two values of one length lands on one index, and a double click selects the whole value (ratified by the founder, 2026-10-18)»
  replace-with: «a double click selects the whole value (ratified by the founder, 2026-10-18; `password_caret_click`)»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «an element under a transform read where its transforms move it (PROVISIONAL)»
  source: .andromeda/architecture.md:178 «an element under a transform read where its transforms move it (ratified by the founder, 2026-10-18; the Scrolling, selection, tree contract)»
  replace-with: «an element under a transform read where its transforms move it (ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «a mixed one `Some(false)` (PROVISIONAL)»
  source: .andromeda/architecture.md:178 «a mixed one `Some(false)` (ratified by the founder, 2026-10-18)»
  replace-with: «a mixed one `Some(false)` (ratified by the founder, 2026-10-18)»
