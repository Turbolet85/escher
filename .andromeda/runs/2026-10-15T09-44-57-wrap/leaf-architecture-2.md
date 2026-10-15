group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:43   class: retired
  leaf-text: «read at 2.1.288 on the dev host; a `run` precondition, not a CI requirement — CI runs only the pipe's shim tests) and a stdlib python3 MCP stub over stdio, no package added.»
  source: .andromeda/architecture.md:66 «its MCP server, since 2026-10-15-cold-agent-test, the workspace's own `escher-session mcp` (a `run` precondition built by the operator, never by the pipe; the stub retired with that chunk)»
  replace-with: «read at 2.1.288 and, at 2026-10-15-cold-agent-test, at 2.1.296 on the dev host; a `run` precondition, not a CI requirement — CI runs only the pipe's shim tests), its MCP server since 2026-10-15-cold-agent-test the workspace's own `escher-session mcp` (a `run` precondition built by the operator, never by the pipe; the stub retired with that chunk), and its judge stdlib python3 (`datetime json os queue re subprocess sys threading`), no package added.»

- leaf: .claude/docs/stack.md:50   class: stale
  leaf-text: «The fork's Actions cache sat at ≈ 9.7 GB of its 10 GB budget after the first build-branch run.»
  source: .andromeda/architecture.md:155 «after the first run on a re-keyed lockfile it held 12 entries, 10 723 071 252 B — over the budget»
  replace-with: «The fork's Actions cache (10 GB per-repository budget, least-recently-used eviction) held 12 entries, 10 723 071 252 B — over the budget — after the first run on a re-keyed lockfile (CI run 37386253475, 2026-10-05).»

- leaf: .claude/docs/services/escher-driver.md:29   class: omission
  leaf-text: «and by blitz-tests (dev-dependency), whose thirteen `stand_act_*` checks drive the schema through `Session::run`.»
  source: .andromeda/architecture.md:141 «with the fixed prompt `STAND_PROMPT` and only the escher server's eleven tools — the driver's own `escher-session mcp` (`--tools ""`, `--strict-mcp-config`, `--allowedTools mcp__escher`»
  replace-with: «and by blitz-tests (dev-dependency), whose thirteen `stand_act_*` checks drive the schema through `Session::run`; since 2026-10-15-cold-agent-test the binary's `mcp` role is also the cold-agent pipe's MCP server (`bash scripts/cold-agent.sh run stand`: one isolated `claude -p` session allowed `mcp__escher` alone, the agent's JSON check replayed on fresh `escher-session mcp` children), a `run` precondition the operator builds, never the pipe.»

- leaf: .claude/docs/services/escher-driver.md:32   class: omission
  leaf-text: «the next surface that carries any of it is a new crossing question.»
  source: .andromeda/architecture.md:147 «since 2026-10-15-cold-agent-test that run carries the stand's screen — element ids, accessible names and control values in snapshot texts and diffs, from the driver's own MCP server — one run»
  replace-with: «the next surface that carries any of it is a new crossing question — since 2026-10-15-cold-agent-test the operator-driven live cold-agent run carries the stand's screen (element ids, accessible names and control values in snapshot texts and diffs, from this MCP server) on to the model provider, one run, on the founder's «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)» (2026-10-15).»
