group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:32   class: omission
  leaf-text: «(`target/coverage/`, line counts, no user data)»
  source: .andromeda/obs-plan.md:329 «line counts of the workspace's public source, no user data (§8), `keyboard_compat.rs` excluded by filename since 2026-10-17-upstream-sync-engine-debts (test-plan §10 Coverage floors)»
  replace-with: «(`target/coverage/`, line counts, no user data, `keyboard_compat.rs` excluded by filename since 2026-10-17-upstream-sync-engine-debts)»
