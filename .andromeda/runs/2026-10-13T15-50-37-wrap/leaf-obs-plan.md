group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the engine check `password_painted_masked` installs none, and»
  source: .andromeda/obs-plan.md:70 «for the five upstream integration targets that arrived with the merge of upstream/main `434e0661` at 2026-10-13-upstream-sync-polish-and-ship»
  replace-with: «the engine check `password_painted_masked` installs none, the five upstream integration targets that arrived with the merge of upstream/main `434e0661` at 2026-10-13-upstream-sync-polish-and-ship (`anonymous_block_restyle`, `button_alignment`, `checked_attribute`, `pointer_leave`, `ua_stylesheet_order`) each install no subscriber, read no env var and print nothing, and»
