group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 971 · 0 · 12 over 187 result lines at 2026-10-17-upstream-sync-engine-debts»
  source: .andromeda/test-plan.md:331 «re-counted at 2026-10-18-upstream-sync-determinism-and-traces: 192 result lines, 991 passed · 0 failed · 12 ignored, from 187 · 971»
  replace-with: «`cargo test --workspace` 991 · 0 · 12 over 192 result lines at 2026-10-18-upstream-sync-determinism-and-traces (from 187 · 971: 2026-10-18-engine-debts' +5 result lines and +19 passed, its 192 · 990 never written in the plan, and this chunk's +1 passed in no new result line — blitz-dom's lib 97 → 98, upstream's `hovering_below_content_reports_default_cursor`; timings not re-measured); 971 · 0 · 12 over 187 result lines at 2026-10-17-upstream-sync-engine-debts»
