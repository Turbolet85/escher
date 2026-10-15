group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:17   class: retired
  leaf-text: «the stdlib MCP stub appends `{seq, tool, outcome, cause}` per call to `calls.jsonl`; no `tracing`, OTel or env read.»
  source: .andromeda/obs-plan.md:87 «Since 2026-10-15-cold-agent-test (the stub `scripts/cold_agent_stub.py` and its call log gone) its per-run `mcp.json` names one server, `escher`, spawned as»
  replace-with: «the pipe itself installs no `tracing` subscriber, OTel setup or third-party logger and reads no env var; since 2026-10-15-cold-agent-test (the stub and its call log gone) its per-run `mcp.json` spawns `escher-session mcp` with `RUST_LOG=info` on that child only, so the child's escher sink writes its closed `tool_call` / `command` span lines to `target/cold-agent/server.log`, which the pipe reads only to count ` span="tool_call" ` lines and their ` ending="refused" ` / ` ending="error" `, never printing or copying it; the agent's check goes to `check.json`, never printed.»

- leaf: .claude/docs/obs-summary.md:45   class: retired
  leaf-text: «no transcript text, tool argument, tool result or reply reaches one; the stub never logs an argument value.»
  source: .andromeda/obs-plan.md:303 «the stub's call log is retired with the stub (2026-10-15-cold-agent-test). `target/cold-agent/server.log` is escher's sink output (the allowlist formatter's redaction), counted by span name»
  replace-with: «no transcript text, tool argument, tool answer, reply or check reaches one; the stub's call log is retired with the stub (2026-10-15-cold-agent-test). `target/cold-agent/server.log` is escher's sink output (the allowlist formatter's redaction), counted by span name and never printed or copied; `check.json` holds the agent's check — stand fixture ids and texts — never printed.»

- leaf: .claude/docs/obs-summary.md:45   class: omission
  leaf-text: «raw by design (model output, tool I/O, host paths, the client's socket path, a rate-limit line)»
  source: .andromeda/obs-plan.md:303 «since 2026-10-15-cold-agent-test the stand's screen from the escher server (element ids, accessible names and control values in snapshot texts and diffs), carried to the model provider»
  replace-with: «raw by design (model output and tool I/O — since 2026-10-15-cold-agent-test the stand's screen from the escher server, element ids, accessible names and control values in snapshot texts and diffs, carried to the model provider on the founder's «Один запуск, до $3 (Рекомендую)» (2026-10-15) — host paths, the client's socket path, a rate-limit line)»

- leaf: .claude/docs/obs-summary.md:45   class: stale
  leaf-text: «the one committed copy (live evidence) host-path-masked»
  source: .andromeda/obs-plan.md:303 «each committed copy (the live runs' evidence) masks the session dir and the home prefix»
  replace-with: «each committed copy (the live runs' evidence) masks the session dir and the home prefix»

- leaf: .claude/rules/observability.md:26   class: stale
  leaf-text: «counts and identities only, never transcript text, tool arguments or results»
  source: .andromeda/obs-plan.md:159 «every one a count, a boolean or a closed word, no transcript text, tool argument, tool answer, reply or check»
  replace-with: «counts, booleans and closed words only, never transcript text, a tool argument, a tool answer, a reply or the agent's check»

- leaf: .claude/rules/observability.md:26   class: omission
  leaf-text: «its raw session transcript stays in `target/cold-agent/transcript.jsonl`, never printed (test-plan §3).»
  source: .andromeda/obs-plan.md:87 «the child installs escher's own sink (escher-telemetry, §3 above) at `info` and its closed `tool_call` / `command` span lines land in `target/cold-agent/server.log`»
  replace-with: «its raw session transcript stays in `target/cold-agent/transcript.jsonl` and the agent's check in `check.json`, neither printed; since 2026-10-15-cold-agent-test its one MCP server is `escher-session mcp` spawned with `RUST_LOG=info` on that child only, so escher's sink writes its closed `tool_call` / `command` span lines to `target/cold-agent/server.log`, which the pipe reads only to count ` span="tool_call" ` lines and their ` ending="refused" ` / ` ending="error" `, never printing or copying it (test-plan §3).»
