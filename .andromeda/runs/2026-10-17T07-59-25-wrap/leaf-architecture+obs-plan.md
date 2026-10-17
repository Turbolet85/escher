group: architecture+obs-plan
read: .claude/docs/services/escher-telemetry.md · 46 · «- `.claude/rules/observability.md` · `.claude/docs/obs-summa»

- leaf: .claude/docs/services/escher-telemetry.md:33   class: stale
  leaf-text: «nothing at the default level, the install line alone at `info`, `debug` and `trace`»
  source: .andromeda/obs-plan.md:296 «the `escher-session` binary (the CRUD task, one `hello`, then `stop`) 0 → 0 lines unset (the default `warn`), 1 → 1 at `info`»
  replace-with: «nothing at the default level and the install line alone at `info`, `debug` and `trace` on a host that runs no call (one `hello`, then `stop`) and on the windowed stand left on Home»
