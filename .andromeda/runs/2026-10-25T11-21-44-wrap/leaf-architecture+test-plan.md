group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:31   class: omission
  leaf-text: «`test_ci_workflows.py`, `test_agent_run.py` and `test_cold_agent.py` among them»
  source: .andromeda/test-plan.md:11 «the capability statement's pin over `docs/capabilities.md` and README.md, and the workflow invariant that every package install of ci.yml is bounded»
  replace-with: «`test_ci_workflows.py`, `test_agent_run.py`, `test_cold_agent.py` and `test_capability_statement.py` (the capability statement's pin over `docs/capabilities.md` and README.md) among them»
