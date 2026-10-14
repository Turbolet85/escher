group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: stale
  leaf-text: «the session host then prints one engine line, `blitz_dom::document` at INFO, from `info` up»
  source: .andromeda/obs-plan.md:42 «re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none»
  replace-with: «the session host then prints its one engine line, `blitz_dom::document` at INFO, twice from `info` up — `trace` adding no line, the host built alone printing none (re-measured at 2026-10-14-stand-requirement-sweep; once before)»
