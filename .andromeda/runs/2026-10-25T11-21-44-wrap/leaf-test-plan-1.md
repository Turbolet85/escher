group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «the cold-agent pipe under a `claude` shim and a fake `target/debug/escher-session`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:8 «the four of `CapabilityStatementTest`, which pin `docs/capabilities.md`'s verb and cause names and its five counts, and README.md's count of the MCP tools, to the driver schema's tables»
  replace-with: «the cold-agent pipe under a `claude` shim and a fake `target/debug/escher-session`, and `test_capability_statement.py` (`CapabilityStatementTest`, 4) `docs/capabilities.md`'s verb and cause names and its five counts, and README.md's count of the MCP tools, to the driver schema's tables — only names and counts, so a check renamed or moved changes the statement's `## Where each claim is proven` by hand»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «(100 CI-script tests in seven files — 94 until 2026-10-15-cold-agent-test»
  source: .andromeda/test-plan-detail/test-scope-summary.md:8 «as the fast `ci-scripts` leg — 104 tests in eight test files (§4; 100 in seven until test_capability_statement.py added its 4; 94 until 2026-10-15-cold-agent-test»
  replace-with: «(104 CI-script tests in eight files — 100 in seven until test_capability_statement.py added its 4; 94 until 2026-10-15-cold-agent-test»
