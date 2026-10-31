group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»
- leaf: .claude/docs/services/escher-telemetry.md:33   class: omission
  leaf-text: «prints its one engine line, `blitz_dom::document` at INFO, twice from `info` up»
  source: .andromeda/obs-plan-detail/log-coverage.md:71 «info: focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name»
  replace-with: «prints its one engine line, `blitz_dom::document` `focussed node` at INFO, once per focus move, twice on the measured snapshot · click · type flow, from `info` up»
