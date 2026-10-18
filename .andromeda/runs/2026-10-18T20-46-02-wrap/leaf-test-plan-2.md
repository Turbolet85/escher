group: test-plan-2
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/verification-harness.md:31   class: stale
  leaf-text: «never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle` (PROVISIONAL)»
  source: .andromeda/test-plan.md:89 «`screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified by the founder, 2026-10-18)»
  replace-with: «never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle` (ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/verification-harness.md:36   class: stale
  leaf-text: «and `advanced_ms` reads exactly `ms`, PROVISIONAL)»
  source: .andromeda/test-plan.md:287 «`advanced_ms` is exactly `ms`, the clock's move (ratified by the founder, 2026-10-18)»
  replace-with: «and `advanced_ms` reads exactly `ms`, ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/verification-harness.md:32   class: stale
  leaf-text: «`Render` or `Layout` when still busy after 64 passes (`SETTLE_PASS_LIMIT`), `Loads` at once for a load in flight»
  source: .andromeda/test-plan.md:90 «`SETTLE_PASS_LIMIT` passes with none quiet return `Render`, `Loads` or `Layout`, read off the last pass»
  replace-with: «`Loads` at once after a quiet pass for a load in flight, and `Render`, `Loads` or `Layout`, read off the last pass, when none of 64 passes (`SETTLE_PASS_LIMIT`) is quiet»
