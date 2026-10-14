group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:43   class: stale
  leaf-text: «and 5 on the workspace-built host, one `blitz_dom::document` INFO line more»
  source: .andromeda/obs-plan.md:296 «the workspace-built host writes 6 lines, 978 B — the `blitz_dom::document` `focussed node` INFO line now printing twice where it printed once»
  replace-with: «and 6 on the workspace-built host, the `blitz_dom::document` INFO line printing twice (re-measured at 2026-10-14-stand-requirement-sweep)»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «(9 stderr lines built alone, 10 with the workspace)»
  source: .andromeda/obs-plan.md:181 «re-measured at 2026-10-14-stand-requirement-sweep, 9 built alone and 11 built with the workspace — the `blitz_dom::document` INFO line now printing twice — the 5 `tool_call` and 3 `command` lines unchanged»
  replace-with: «(9 stderr lines built alone, 11 with the workspace, the `blitz_dom::document` INFO line printing twice since 2026-10-14-stand-requirement-sweep)»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «prints one engine line, `blitz_dom::document` at INFO, from `info` up and none more at `debug` or `trace`»
  source: .andromeda/obs-plan.md:42 «re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none»
  replace-with: «prints one engine line, `blitz_dom::document` at INFO, twice from `info` up (re-measured at 2026-10-14-stand-requirement-sweep) and none more at `debug` or `trace`»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the engine check `range_input_interaction` (since 2026-10-13-range-input-interaction)»
  source: .andromeda/obs-plan.md:70 «and the engine check `password_caret_click` (its own parsed page, declaring no shared module) each install no subscriber, read no env var and print nothing, their readings going into assertions only»
  replace-with: «`stand_requirement_sweep` and `stand_non_lean_tasks` (since 2026-10-14-stand-requirement-sweep, in process in both layout modes) and the engine check `password_caret_click` (its own parsed page) each install no subscriber, read no env var and print nothing, their readings going into assertions only, the engine check `range_input_interaction` (since 2026-10-13-range-input-interaction)»

- leaf: .claude/rules/observability.md:36   class: stale
  leaf-text: «one engine line, `blitz_dom::document` at INFO, from `info` up under the workspace build»
  source: .andromeda/obs-plan.md:42 «re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none»
  replace-with: «one engine line, `blitz_dom::document` at INFO, printed twice from `info` up under the workspace build (re-measured at 2026-10-14-stand-requirement-sweep)»
