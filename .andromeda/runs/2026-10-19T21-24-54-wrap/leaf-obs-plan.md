group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`host_log` and `mcp_log` passing under both builds.»
  source: .andromeda/obs-plan.md:78 «- The out-of-line escher-driver test files `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` and the seven_guis shared module `examples/seven_guis/tests/common/mod.rs` are silent»
  replace-with: «`host_log` and `mcp_log` passing under both builds; the out-of-line escher-driver test files `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` and the seven_guis shared module `examples/seven_guis/tests/common/mod.rs` are silent — the module states once, for `host_log` and `mcp_log` alone, the log checks' readers `Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences` and the unix-only `recorded_needles`, their assertion messages naming a kind and a count, never a needle or a captured line — 0 print, tracing and subscriber sites over the four files, escher-driver still holding its 2 `info_span!` sites and the env-read set unchanged.»
