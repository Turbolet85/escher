group: masters
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:17   class: retired
  leaf-text: «the cold-agent run pipe and its stub, the code-graph tooling»
  source: .andromeda/architecture.md:175 «scripts/               agent-run.{sh,ps1} (test contract) · cold-agent.{sh,ps1} (cold-agent pipe) · code-graph tooling»
  replace-with: «the cold-agent run pipe, the code-graph tooling»

- leaf: CLAUDE.md:68   class: retired
  leaf-text: «| Cold-agent run pipe (isolated agent session · stub MCP tool · verdict) | `.andromeda/test-plan.md` §3 · `scripts/cold-agent.sh` · `scripts/cold_agent_stub.py` (contract tests `.github/scripts/test_cold_agent.py`) |»
  source: .andromeda/test-plan.md:123 «- **The server and the check's judge (since 2026-10-15-cold-agent-test, which deleted the stub `scripts/cold_agent_stub.py`):** the agent talks to the driver's own MCP surface, `escher-session mcp`»
  replace-with: «| Cold-agent run pipe (isolated agent session · the driver's own MCP server `escher-session mcp` · the agent's replayed check · verdict) | `.andromeda/test-plan.md` §3 · `scripts/cold-agent.sh` (contract tests `.github/scripts/test_cold_agent.py`) |»
