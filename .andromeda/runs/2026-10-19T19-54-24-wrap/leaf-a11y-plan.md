group: a11y-plan
read: .claude/docs/a11y-summary.md · 45 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 46 · «- 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable»

- leaf: .claude/docs/a11y-summary.md:23   class: stale
  leaf-text: «`:focus-visible` / `:focus-within` never match.»
  source: .andromeda/a11y-plan.md:155 «`:focus` matches the focus element state and `:focus-within` the focused element and its ancestors by the `FOCUS_WITHIN` element state (PROVISIONAL), while `:focus-visible` never matches»
  replace-with: «`:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (PROVISIONAL), while `:focus-visible` never matches.»

- leaf: .claude/rules/a11y.md:34   class: stale
  leaf-text: «- `:focus-visible` and `:focus-within` never match — a stand style that relies on them shows no focus ring.»
  source: .andromeda/a11y-plan.md:155 «`:focus` matches the focus element state and `:focus-within` the focused element and its ancestors by the `FOCUS_WITHIN` element state (PROVISIONAL), while `:focus-visible` never matches»
  replace-with: «- `:focus-visible` never matches — a stand style that relies on it shows no focus ring — while `:focus-within` matches the focused element and its ancestors by the `FOCUS_WITHIN` element state (PROVISIONAL).»
