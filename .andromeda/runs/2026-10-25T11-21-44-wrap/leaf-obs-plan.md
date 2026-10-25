group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:32   class: stale
  leaf-text: «snapshot text and diffs at rest, and no log or event line»
  source: .andromeda/obs-plan-detail/ci-integration.md:16 «so the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest, and no log or event line»
  replace-with: «snapshot text, diffs and a lint's findings at rest, and no log or event line»

- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «snapshot text and diffs at rest — a file»
  source: .andromeda/obs-plan-detail/ci-integration.md:16 «so the screens' ids, names, values, snapshot text, diffs and a lint's findings at rest, and no log or event line»
  replace-with: «snapshot text, diffs and a lint's findings at rest — a file»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «each test under its own because `tracing`'s callsite interest is process-global.»
  source: .andromeda/obs-plan-detail/log-coverage.md:49 «its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand»
  replace-with: «each test under its own because `tracing`'s callsite interest is process-global. The published statement `docs/capabilities.md` restates both spans (closed-word, bool and count fields, stderr only, no driver line at the default `warn`) and the scrub's limits; its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand.»

- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «nor a part's number or the part count.»
  source: .andromeda/obs-plan-detail/log-coverage.md:49 «its pin `.github/scripts/test_capability_statement.py` reads none of these claims, so a change to the span set or to the scrub's limits is made in that document by hand»
  replace-with: «nor a part's number or the part count. `docs/capabilities.md` restates both spans and the scrub's limits, and its pin `.github/scripts/test_capability_statement.py` reads none of these claims — a change to the span set or to the scrub's limits is made in that document by hand.»
