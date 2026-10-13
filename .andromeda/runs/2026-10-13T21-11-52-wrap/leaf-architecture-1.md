group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:19   class: stale
  leaf-text: «by a text control's input — the last two wherever the node is — and NOT by node creation»
  source: .andromeda/architecture.md:197 «and, since 2026-10-13-range-input-interaction, `BaseDocument::apply_range_move`, which marks a range input where a key or a pointerdown changed its value and nothing for a move that changes nothing»
  replace-with: «by a text control's input — the last two wherever the node is — and, since 2026-10-13-range-input-interaction, by `BaseDocument::apply_range_move`, which marks a range input where a key or a pointerdown changed its value and nothing for a move that changes nothing — and NOT by node creation»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «`BoundingRect`, CSSOM View accessors, `Widget` trait, and since 2026-10-12-headless-screenshot»
  source: .andromeda/architecture.md:126 «a range input's current value `RangeInputData`, its one public method `value() -> f64`, exported from `blitz_dom::node` and not re-exported from the crate root»
  replace-with: «`BoundingRect`, CSSOM View accessors, `Widget` trait, since 2026-10-13-range-input-interaction a range input's current value — `SpecialElementData::RangeInput` (a source break for an external exhaustive match, `SpecialElementData` not being `#[non_exhaustive]`), `RangeInputData` (its one public method `value() -> f64`, exported from `blitz_dom::node` and not re-exported from the crate root) and the public `ElementData` accessors `range_input_data()`, `range_input_data_mut()` and `range_input_value() -> Option<String>` (the formatted current value), `RangeRules`, `RangeMove` and the range helpers crate-internal — and since 2026-10-12-headless-screenshot»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «its one caller is blitz-paint's password mask).»
  source: .andromeda/architecture.md:87 «a fifth direct engine change for every document stands beside the three this decision counts and the password paint under [Painting] since 2026-10-13-range-input-interaction»
  replace-with: «its one caller is blitz-paint's password mask). Since 2026-10-13-range-input-interaction the range interaction model, a fifth direct engine change for every document (ratified by the founder, flagged upstreamable): an `input` whose `type` reads `range` (ASCII case-insensitive) holds its current value apart from its `value` attribute, its rules in `node/range.rs` against the WHATWG range state (`min` 0, `max` 100 and `step` 1 by default, every value sanitized to `[min, max]` and the nearest grid value); focused and with no `disabled` attribute, `ArrowRight`/`ArrowUp` step it up, `ArrowLeft`/`ArrowDown` down, `Home`/`End` take it to its minimum/maximum, every other key does nothing; a main-button pointerdown sets the value from the hit's fraction of the content box and a click focuses it and changes no value; each change fires one `input` event through the crate-internal `BaseDocument::apply_range_move` and a move that changes nothing fires none; a `value` write re-seeds the current value and a `min`, `max` or `step` write or removal re-sanitizes it (a departure from the spec's dirty-value flag, so a Dioxus re-render keeps control); form submission sends the formatted current value.»

- leaf: .claude/docs/services/dioxus-native-dom.md:17   class: stale
  leaf-text: «`value` is the editor's text else the `value` attribute — and a password»
  source: .andromeda/architecture.md:136 «`value` the editor text, else — for a range `input`, since 2026-10-13-range-input-interaction — its current value `range_input_value()`»
  replace-with: «`value` is the editor's text, else — for a range `input`, since 2026-10-13-range-input-interaction — its current value `range_input_value()` (the formatted value a key or a pointerdown moved, while its `value` attribute keeps what the markup wrote), else the `value` attribute — and a password»
