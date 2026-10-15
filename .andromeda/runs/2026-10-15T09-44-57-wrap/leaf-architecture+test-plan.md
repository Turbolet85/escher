group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:40   class: stale
  leaf-text: «`bash scripts/cold-agent.sh run counter` — one isolated `claude -p` session»
  source: .andromeda/test-plan.md:120 «the one task `stand` — the four lean apps in one client session (`TASKS=(stand)`; `counter` no task since 2026-10-15-cold-agent-test, so `run counter` is a usage error)»
  replace-with: «`bash scripts/cold-agent.sh run stand` — the four lean apps, one at a time, in one isolated `claude -p` session (`run counter` is a usage error since 2026-10-15-cold-agent-test)»

- leaf: .claude/docs/commands.md:40   class: retired
  leaf-text: «given only the stub's MCP tools»
  source: .andromeda/architecture.md:141 «only the escher server's eleven tools — the driver's own `escher-session mcp`»
  replace-with: «given only the escher server's eleven tools (the driver's own `escher-session mcp`; the stub retired at 2026-10-15-cold-agent-test), the agent's last fenced JSON check then replayed on a fresh `escher-session mcp` child and, ablated, on a second»

- leaf: .claude/docs/commands.md:40   class: omission
  leaf-text: «3 when `claude` or `python3` is not on PATH)»
  source: .andromeda/test-plan.md:121 «since 2026-10-15-cold-agent-test `target/debug/escher-session` absent or not executable, with one stderr line naming `cargo build --locked -p seven_guis --bin escher-session` — the pipe builds nothing»
  replace-with: «3 when `claude` or `python3` is not on PATH or `target/debug/escher-session` is absent or not executable — build it first with `cargo build --locked -p seven_guis --bin escher-session`, the pipe builds nothing)»
