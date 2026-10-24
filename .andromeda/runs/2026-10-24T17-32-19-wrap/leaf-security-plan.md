group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:32   class: omission
  leaf-text: «(measured on the `escher-session` host and its `mcp` role while they run calls: 0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace`, under both builds)»
  source: .andromeda/security-plan-detail/logging-monitoring.md:14 «through the `escher-session` host's own sink it is unmeasured, the host booting only the four lean tasks and none paging, its measurement owned by a `.andromeda/residuals.md` line (target: next)»
  replace-with: «(measured on the `escher-session` host and its `mcp` role while they run calls, and on a recording session's clients: 0 id needles, 0 name needles and 0 occurrences of a typed sentinel at `trace`, under both builds; a paged part read is measured in process only — through the host's own sink it is unmeasured, owned by a `.andromeda/residuals.md` line (target: next))»
