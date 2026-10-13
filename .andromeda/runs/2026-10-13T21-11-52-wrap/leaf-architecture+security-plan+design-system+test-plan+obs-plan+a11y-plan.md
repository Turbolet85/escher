group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «`aria-checked` unread), and the changed set (»
  source: .andromeda/architecture.md:87 «a fifth direct engine change for every document stands beside the three this decision counts and the password paint under [Painting] since 2026-10-13-range-input-interaction»
  replace-with: «`aria-checked` unread), a range input's interaction model (an `input` whose `type` reads `range` holds a current value apart from its `value` attribute — `RangeInputData`, its rules in `node/range.rs`, every value sanitized to `[min, max]` on its step grid; focused and with no `disabled` attribute, the arrows step it and `Home` · `End` take it to its minimum and maximum, a main-button pointerdown sets it from the hit's fraction of the content box and a click focuses it without moving it; each change fires one `input` event, a move that changes nothing none; a `value` write re-seeds it and a `min`, `max` or `step` write re-sanitizes it — a direct engine change for every document, ratified by the founder, 2026-10-13), and the changed set (»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «marked by mutations of in-document nodes, focus and checked changes and typed text, not by node creation, hover or layout»
  source: .andromeda/architecture.md:197 «and, since 2026-10-13-range-input-interaction, `BaseDocument::apply_range_move`, which marks a range input where a key or a pointerdown changed its value and nothing for a move that changes nothing»
  replace-with: «marked by mutations of in-document nodes, focus and checked changes, typed text and a range input's moved value (`apply_range_move`, nothing for a move that changes nothing), not by node creation, hover or layout»

- leaf: CLAUDE.md:33   class: omission
  leaf-text: «a password or file input's value reads the fixed mask `MASKED_VALUE`;»
  source: .andromeda/architecture.md:136 «`value` the editor text, else — for a range `input`, since 2026-10-13-range-input-interaction — its current value `range_input_value()`, the formatted value a key or a pointerdown moved»
  replace-with: «a password or file input's value reads the fixed mask `MASKED_VALUE`, a range input's its current value `range_input_value()` — the formatted value a key or a pointerdown moved, while its `value` attribute keeps what the markup wrote;»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «proven on the four lean stand tasks in both layout modes — a range input, which no verb can move, is measured and owned by a route entry)»
  source: .andromeda/architecture.md:263 «the range input moved by the driver's `click` and keys — since 2026-10-13-range-input-interaction, its value changes named by the diff on the slider (`value`) and on its display (`name`)»
  replace-with: «proven on the four lean stand tasks in both layout modes — the Timer's range input moved by `click` and the arrow, `Home` and `End` keys since the range interaction model, a direct engine change for every document ratified by the founder (2026-10-13), the diff naming the slider's `value` and its display's `name`)»

- leaf: CLAUDE.md:75   class: omission
  leaf-text: «| DOM entry point · config | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` |»
  source: .andromeda/architecture.md:245 «Attributes, custom widgets, element data, node tree, range input data (the private module `range`, since 2026-10-13-range-input-interaction, `RangeInputData` exported from it;»
  replace-with: «| DOM entry point · config · range input (current value · rules · key and pointer default actions) | `packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` · range input `packages/blitz-dom/src/node/range.rs` (unit tests inline) and `packages/blitz-dom/src/events/{keyboard,pointer}.rs` · check `tests/blitz-tests/tests/range_input_interaction.rs` |»
