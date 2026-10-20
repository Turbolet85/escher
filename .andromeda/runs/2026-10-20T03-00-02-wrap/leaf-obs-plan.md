group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «The settle loop `Harness::settle` and `Session::act` carry no span of their own»
  source: .andromeda/obs-plan.md:120 «a field of its own in the driver's load wait over it (`settle_waiting`, bounded by `LOAD_WAIT`), which runs inside the call's `command` span»
  replace-with: «The settle loop `Harness::settle`, the driver's load wait over it (`settle_waiting`, bounded by `LOAD_WAIT`) — which runs inside the call's `command` span and leaves its eight fields as they are — and `Session::act` carry no span of their own»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and nothing logs, fields or prints one; the upstream focus checks»
  source: .andromeda/obs-plan.md:81 «the load wait it pins (`settle_waiting`, `LOAD_WAIT`) adds no log site, span, field, log field or env read, escher-driver keeping its 2 `info_span!` sites and 0 events»
  replace-with: «and nothing logs, fields or prints one; the load-wait check `stand_load_wait` (a `fixtures` and `session_common` reader, both layout modes) and its in-file stand-in `LateNet`, which answers each request from its own thread after 200 ms, install no subscriber, read no env var, print nothing and use no `blitz_net`, their readings going into assertions only, and the load wait it pins (`settle_waiting`, `LOAD_WAIT`) adds no log site, span, field, log field or env read, escher-driver keeping its 2 `info_span!` sites and 0 events; the upstream focus checks»
- leaf: .claude/rules/observability.md:24   class: omission
  leaf-text: «the settle loop `Harness::settle`, `Session::act` and, since 2026-10-12-headless-screenshot,»
  source: .andromeda/obs-plan.md:120 «a field of its own in the driver's load wait over it (`settle_waiting`, bounded by `LOAD_WAIT`), which runs inside the call's `command` span»
  replace-with: «the settle loop `Harness::settle`, the driver's load wait over it (`settle_waiting`, bounded by `LOAD_WAIT`, run inside the call's `command` span, its eight fields as they are), `Session::act` and, since 2026-10-12-headless-screenshot,»
