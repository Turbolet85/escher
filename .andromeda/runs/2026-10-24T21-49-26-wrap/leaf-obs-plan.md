group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`packages/escher-telemetry/` unchanged and `host_log` and `mcp_log` passing under both builds.»
  source: .andromeda/obs-plan-detail/log-coverage.md:109 «a print, an env read, a subscriber or a listener in the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`»
  replace-with: «`packages/escher-telemetry/` unchanged and `host_log` and `mcp_log` passing under both builds; the eight checks the `a11y` leg runs beside its six earlier files (`accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction`, `stand_act_select`) and the two shared modules they read (`tests/common/mod.rs`, `tests/fixtures/mod.rs`) hold no print, env read, subscriber or listener — the silence census over those ten files reading 0, `packages/` unchanged (2026-10-24-a11y-re-read).»
