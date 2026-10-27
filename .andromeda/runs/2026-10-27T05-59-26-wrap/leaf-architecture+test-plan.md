group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:21   class: stale
  leaf-text: «carry none; the stand's load page, whose `LoadNet` overrides no `is_noop`, carries the counter, so its one stylesheet fetch reads as a load in flight until it is answered and are read through `has_pending_critical_resources()` alone.»
  source: .andromeda/architecture-detail/standard-contracts.md:108 «an offline harness (every stand boot but `boot_load`'s) and a document passed to `wrap` carry no counter and are read through `has_pending_critical_resources()` alone»
  replace-with: «carry none and are read through `has_pending_critical_resources()` alone; the stand's load page, whose `LoadNet` overrides no `is_noop`, carries the counter, so its one stylesheet fetch reads as a load in flight until it is answered.»

- leaf: .claude/docs/services/blitz-test-harness.md:15   class: omission
  leaf-text: «`Busy` one of `Render` · `Layout` (still busy after `SETTLE_PASS_LIMIT` = 64 passes) · `Loads` (a load in flight, reported at once)»
  source: .andromeda/architecture-detail/standard-contracts.md:107 «`SETTLE_PASS_LIMIT` (64) passes with none quiet return `NotSettled` with the class read off the last pass — `Render` when its poll answered `true`, else `Loads` when a load finished in it, else `Layout`»
  replace-with: «`Busy` one of `Render` · `Layout` · `Loads` — a load in flight after a quiet pass reported at once as `Loads`, and after `SETTLE_PASS_LIMIT` = 64 passes with none quiet the class read off the last pass (`Render` when its poll answered `true`, else `Loads` when a load finished in it, else `Layout`)»
