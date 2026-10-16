group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «each change fires one `input` event through the crate-internal `BaseDocument::apply_range_move` and a move that changes nothing fires none;»
  source: .andromeda/architecture.md:123 «the bodies of their range and button arms are private named functions, with no behaviour changed for any document»
  replace-with: «each change fires one `input` event through the crate-internal `BaseDocument::apply_range_move` and a move that changes nothing fires none (since 2026-10-16-handler-splits-and-readme the range arms' bodies are private named functions — `press_range_input_key` for the keys, `range_track_fraction` and `press_range_input` for the pointerdown — still writing only through `apply_range_move`, `handle_key_or_input_event` and `handle_pointerdown` keeping their signatures, no behaviour changed for any document);»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «before it submits a submit button's form owner, a click on any other element still clearing focus;»
  source: .andromeda/architecture.md:123 «`click_button` for the button arm's focus and submit (packages/blitz-dom/src/events/pointer.rs:890-907)»
  replace-with: «before it submits a submit button's form owner — since 2026-10-16-handler-splits-and-readme in the private `click_button`, `handle_click` keeping its signature, no behaviour changed — a click on any other element still clearing focus;»
