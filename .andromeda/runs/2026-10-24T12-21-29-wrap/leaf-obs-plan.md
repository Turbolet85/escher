group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`TextPart` hold no log, print or failure path, adding no log site, span, field, log field or env read. The driver's»
  source: .andromeda/obs-plan.md:96 «The upstream take read at `f9cbb2756` is silent: upstream's target `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing, its readings going into assertions only;»
  replace-with: «`TextPart` hold no log, print or failure path, adding no log site, span, field, log field or env read; the upstream take read at `f9cbb2756` is silent too — upstream's target `hit_test_dropped_nodes` installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the taken commits add no tracing site over `packages/` and no wall-clock read in blitz-dom outside `clock.rs`, and the process-wide `PARALLEL_TRAVERSAL_LOCK`, `Node::out_of_flow`, `ElementData::classes`, the dropped-node hit skip and a replaced element's `aspect-ratio` height add no log site, span, field, log field or env read, `packages/escher-telemetry/` unchanged and `host_log` and `mcp_log` passing under both builds. The driver's»
