group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «no driver schema, span field or log line moving (2026-10-25-cold-agent-bar). The driver»
  source: .andromeda/obs-plan-detail/log-coverage.md:125 «- observed absent — a print, log site, span, field, env read or subscriber in the private items that factor the lint's findings walk and the two table-asserting tests (escher-driver `lint.rs`'s»
  replace-with: «no driver schema, span field or log line moving (2026-10-25-cold-agent-bar); the private items that factor the lint's findings walk and the two table-asserting tests — escher-driver `lint.rs`'s `readings` and `placement`, `mcp_tests.rs`'s `#[track_caller]` `assert_described`, `assert_schema` and `assert_property`, `stand_requirement_sweep`'s `#[track_caller]` `assert_0_2_0_row` and `assert_0_2_0_witness` and its target predicate `inside_stand_repeat` — hold no print, log site, span, field, env read or subscriber, the helpers' assertion messages naming a row and an index only, `mcp_tests.rs`'s env reads the 2 compile-time `env!` alone (`CARGO_MANIFEST_DIR`, `CARGO_PKG_VERSION`), escher-driver holding 2 `info_span!` sites, no event and no `#[instrument]` (2026-10-26-codebase-health). The driver»
