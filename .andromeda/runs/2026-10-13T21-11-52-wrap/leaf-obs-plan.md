group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «its readings going into assertions only, the five upstream integration targets»
  source: .andromeda/obs-plan.md:70 «engine check `range_input_interaction` (since 2026-10-13-range-input-interaction), which drives `blitz_test_harness::Harness` on its own parsed pages in both layout modes and installs no subscriber»
  replace-with: «its readings going into assertions only, the engine check `range_input_interaction` (since 2026-10-13-range-input-interaction), driving `blitz_test_harness::Harness` on its own parsed pages in both layout modes, installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the five upstream integration targets»
