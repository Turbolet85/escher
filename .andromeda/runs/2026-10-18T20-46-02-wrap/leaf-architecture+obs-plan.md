group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:18   class: omission
  leaf-text: «a bridged `log` record printed under its `log.target`.»
  source: .andromeda/architecture.md:174 «newlines escaped, string values Debug-quoted, a bridged `log` record printed under its `log.target` with that field omitted (packages/escher-telemetry/src/format.rs:134-286)»
  replace-with: «a bridged `log` record printed under its `log.target`, that field itself omitted.»
