group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`BoundingRect`, CSSOM View accessors, `Widget` trait,»
  source: .andromeda/architecture-detail/standard-contracts.md:79 «- `ElementData::flush_open_state(&mut self)`, public: it sets `ElementState::OPEN` on a `details` or `dialog` carrying an `open` attribute and clears it on any other;»
  replace-with: «`BoundingRect`, CSSOM View accessors, `Widget` trait, the public `ElementData::flush_open_state(&mut self)` (it sets `ElementState::OPEN` on a `details` or `dialog` carrying an `open` attribute and clears it on any other; `ElementData::new` calls it, and so do `DocumentMutator::set_attribute` — its arm a second independent `if` after the range re-sanitize arm — and `clear_attribute` on an `open` write, and Stylo's `NonTSPseudoClass::Open` reads that state, so `:open` matches such an element in every document — an upstream engine change, not counted among escher's direct ones),»

- leaf: .claude/docs/services/blitz-dom.md:21   class: omission
  leaf-text: «Layout runs over a crate-private `LayoutPassState`, which wraps the document for one pass and carries Taffy's tree traits; `BaseDocument` implements none of them.»
  source: .andromeda/architecture-detail/established-decisions.md:99 «- Every box-generating `::before` / `::after` votes in flow classification (`classify_flow_children`)»
  replace-with: «Layout runs over a crate-private `LayoutPassState`, which wraps the document for one pass and carries Taffy's tree traits; `BaseDocument` implements none of them. Every box-generating `::before` / `::after` votes in flow classification (`classify_flow_children`) — an inline-block whose only child is a pseudo exports its line baseline and bottom-aligns, and a grid row grows by the baseline shim of pseudo-only items (`pseudo_element_flow_classification`, which does not assert both layout modes) — and a text input already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor`: two upstream engine changes for every document, not counted among escher's direct ones.»

- leaf: .claude/docs/services/blitz-shell.md:17   class: omission
  leaf-text: «Shell events flow through an mpsc channel drained in `proxy_wake_up`; every window event is followed by a Poll.»
  source: .andromeda/architecture-detail/standard-contracts.md:104 «- `BlitzShellProvider` holds its window, the shell's `BlitzShellProxy` and its document's id, and its `request_redraw` sends `BlitzShellEvent::RequestRedraw { doc_id }` through that proxy»
  replace-with: «Shell events flow through an mpsc channel drained in `proxy_wake_up`; every window event is followed by a Poll. `BlitzShellProvider` holds its window, the shell's `BlitzShellProxy` and its document's id, and its `request_redraw` sends `BlitzShellEvent::RequestRedraw { doc_id }` through that proxy, so the windowed shell's redraw requests travel through its event loop — an upstream engine change for every document, held by the windowed `seven_guis_native` smoke boot and `focus_redraw`.»
