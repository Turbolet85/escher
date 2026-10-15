group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:18   class: stale
  leaf-text: «allowlists its verb and the task `counter` (else exit 2, before any precondition or `claude` call)»
  source: .andromeda/security-plan.md:75 «a task outside the allowlist `stand` (`TASKS=(stand)` since 2026-10-15-cold-agent-test; `run counter` now a usage error) prints usage to stderr with empty stdout and exits 2 before any precondition»
  replace-with: «allowlists its verb and the task `stand` (since 2026-10-15-cold-agent-test; `run counter` now a usage error; else exit 2, before any precondition or `claude` call), requires an executable `target/debug/escher-session` (else exit 3; the pipe builds nothing) and hands the session only the fixed prompt `STAND_PROMPT`, naming no element id and no app slug»

- leaf: .claude/docs/security-summary.md:19   class: retired
  leaf-text: «its only tools are a stdlib stdio MCP stub that refuses an unknown id, a disabled button, a press on the display and a malformed argument (`isError`, no state change) and logs no argument value»
  source: .andromeda/security-plan.md:292 «since 2026-10-15-cold-agent-test the pipe drives the driver's own MCP server (`--allowedTools mcp__escher`, `--model claude-opus-5-5`, `--max-budget-usd 3`, `claude` 2.1.296)»
  replace-with: «since 2026-10-15-cold-agent-test its tools are the driver's own MCP server (`--allowedTools mcp__escher`, `--model claude-opus-5-5`, `--max-budget-usd 3`; `scripts/cold_agent_stub.py` deleted), its one live run carrying the stand's screen — ids, names and values in snapshot texts and diffs — to the model provider; the reply returns as the agent-written check, a new input class (the founder's «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)», 2026-10-15): only its last fenced json block, at most 65,536 bytes, a JSON array of at most 400 steps with closed keys and `tool` one of the eleven verbs, else `no-check` / `check-failed` and never replayed; a valid check replayed in full, then ablated, on two fresh `escher-session mcp` children (stderr to `/dev/null`), every step validated again by the server, 30 s per reply, each child killed and reaped; `check.json` never printed»

- leaf: .claude/docs/security-summary.md:22   class: omission
  leaf-text: «in the one live trial that content reached the model provider through the client»
  source: .andromeda/security-plan.md:28 «Since 2026-10-15-cold-agent-test the cold-agent pipe's one live run is a second carriage: through the `claude` client and the operator's own login it carried the stand's screen»
  replace-with: «in the one live trial that content reached the model provider through the client, and since 2026-10-15-cold-agent-test the cold-agent pipe's one live run is a second carriage — the stand's screen (element ids, accessible names and control values in snapshot texts and diffs, 43 tool calls, no `screenshot`) to the model provider, the model's reply returning as the agent-written check the pipe parses and replays (a boundary widening on the founder's «Один запуск, до $3 (Рекомендую)» and «JSON-проверка с повтором (Рекомендую)», 2026-10-15)»

- leaf: .claude/docs/security-summary.md:36   class: stale
  leaf-text: «`target/cold-agent/transcript.jsonl` — model output, tool I/O, host paths | raw by design, never printed, gitignored; events and verdict carry counts and identities only; the one committed copy host-path-masked»
  source: .andromeda/security-plan.md:403 «each committed copy (the live runs' evidence) has its host paths masked — the session dir and the home prefix»
  replace-with: «`target/cold-agent/transcript.jsonl` — model output, tool I/O (since 2026-10-15-cold-agent-test the stand's screen), host paths; `target/cold-agent/server.log` (escher's sink at `RUST_LOG=info`); `target/cold-agent/check.json` (the agent's check as parsed) | raw by design, never printed, gitignored; events and verdict carry counts, booleans and closed words only — no tool argument, tool answer, reply or check; `server.log` counted by span name, never printed or copied; the replay children's stderr to `/dev/null`; each committed copy (the live runs' evidence) host-path-masked — the session dir and the home prefix»

- leaf: .claude/rules/security.md:23   class: retired
  leaf-text: «its stdio-only MCP stub (no port), and the model provider reached on the live run only»
  source: .andromeda/security-plan.md:292 «since 2026-10-15-cold-agent-test the pipe drives the driver's own MCP server (`--allowedTools mcp__escher`, `--model claude-opus-5-5`, `--max-budget-usd 3`, `claude` 2.1.296)»
  replace-with: «the driver's own MCP server as its tools (`mcp__escher`, stdio, no port — the stub deleted since 2026-10-15-cold-agent-test), and the model provider reached on the live run only — since 2026-10-15-cold-agent-test a second carriage of the stand's screen (ids, names, values in snapshot texts and diffs) to it, a boundary widening on the founder's «Один запуск, до $3 (Рекомендую)» (2026-10-15)»

- leaf: .claude/rules/security.md:23   class: retired
  leaf-text: «The stub validates every tool argument and refuses with a named cause; a committed transcript is host-path-masked»
  source: .andromeda/security-plan.md:78 «`extract_check` takes only the reply's last fenced json block, at most 65,536 bytes (`MAX_CHECK_BYTES`), parsed by python `json` and required to be a JSON array»
  replace-with: «The model's reply is untrusted input, the agent-written check (the founder's «JSON-проверка с повтором (Рекомендую)», 2026-10-15): only its last fenced json block, at most 65,536 bytes, a JSON array of at most 400 steps with closed keys and `tool` one of the eleven verbs, else `no-check` / `check-failed` and never replayed; a valid check is replayed on fresh `escher-session mcp` children, every step validated again by the server, 30 s per reply, each child killed and reaped; `check.json`, `server.log` and the transcript are never printed; each committed transcript is host-path-masked»

- leaf: .claude/rules/security.md:9   class: omission
  leaf-text: «and `RUST_LOG` (escher-telemetry's filter) only»
  source: .andromeda/security-plan.md:314 «`env!("CARGO_BIN_EXE_escher-session")` and `env!("CARGO_TARGET_TMPDIR")` at build time in the session checks»
  replace-with: «`RUST_LOG` (escher-telemetry's filter) and, at build time in the session checks, `CARGO_BIN_EXE_escher-session` and `CARGO_TARGET_TMPDIR` only»
