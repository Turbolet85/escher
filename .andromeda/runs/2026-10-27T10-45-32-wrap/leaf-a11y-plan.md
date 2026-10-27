group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/rules/a11y.md:34   class: omission
  leaf-text: «while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state.»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:28 «so `:open` matches it, in every document; activating a `details`'s first `summary` (above) toggles `open` through the mutator and so flips `:open`; no test asserts `:open`»
  replace-with: «while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state, and `:open` matches a `details` or `dialog` carrying `open` by the `OPEN` element state (`ElementData::flush_open_state`, at element construction and on every `open` write or removal through the mutator; a `details`'s first `summary` activated flipping it), in every document — no test asserts `:open`.»

- leaf: .claude/docs/a11y-summary.md:23   class: omission
  leaf-text: «by the `FOCUS_WITHIN` element state, while `:focus-visible` never matches.»
  source: .andromeda/a11y-plan-detail/keyboard-navigation.md:28 «so `:open` matches it, in every document; activating a `details`'s first `summary` (above) toggles `open` through the mutator and so flips `:open`; no test asserts `:open`»
  replace-with: «by the `FOCUS_WITHIN` element state, while `:focus-visible` never matches; `:open` matches a `details` or `dialog` carrying `open` by the `OPEN` element state, in every document, flipped when a `details`'s first `summary` toggles it through the mutator — no test asserts `:open`.»
