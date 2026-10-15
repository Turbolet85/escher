# security extract

## Relevance
relevant — the chunk rewires a registered crossing (the cold-agent pipe's spawned `claude` client and its stdio MCP server), moves its CLI allowlist, may replace its call log, runs the live login once more and may reword a refusal's text.

## Constraints
- `scripts/cold-agent.sh` must keep validating its verb (`run status cleanup logs`), argument count and task allowlist before any precondition or `claude` call: usage to stderr, empty stdout, exit 2. A task moved from `counter` to a stand task must still reach the agent session only as a fixed prompt naming the goal and no element id (per security-plan §Input Validation, row "CLI arguments (cold-agent.sh)").
- The swapped server is the `escher-session mcp` role. It is bounded by its own row: 65,536-byte request line, the `id` echoed in two forms only, every `tools/call` through `validate` / `validate_session`, a 1,048,576-byte result, no listener, port, socket or env read. The pipe must reach it only as a stdio child of the spawned client (per security-plan §Input Validation, row "MCP server (escher-driver `serve_mcp`, the `escher-session mcp` role)").
- The server's results carry ids, accessible names, control values (masked for password and file inputs), snapshot text and a screenshot `image` block to the client, and the client passes them on to the model provider. That carrier is ratified. Any new carrier of call or screen content is a new crossing for the founder: a count file holding arguments, or a channel for the agent to hand over its check. "What a client does with a result is the client's" covers the model provider only (per security-plan §Threat Model Summary, vector "local stdio — the driver's MCP server").
- The live run authenticates only through the operator's own Claude Code login, held by the `claude` CLI. Requirements: `apiKeySource` none; no env read by the pipe scripts (the census `os\.environ|getenv|ANTHROPIC_` over them reads 0); no CI secret, CI running only the shim contract tests. Committed live evidence must hold no key, auth header, credential-file name, token field or e-mail address (per security-plan §Secret Management, Storage → Development).
- The pipe logs run metadata only. `target/cold-agent/events.jsonl` and `verdict.json` carry counts and identities, with no content-named field and no transcript text, tool argument, tool result or reply. `target/cold-agent/transcript.jsonl` stays raw, never printed and gitignored under `target/`, and any committed copy has its host paths masked (per security-plan §Logging & Monitoring, the cold-agent pipe bullet).
- If the `tool_call` span line on the `mcp` role's stderr becomes the independent count, it must pass escher's sink unchanged. An `escher_*` target prints with content-named fields redacted, and a closed span is judged by the same rule as an event. The pipe must add no span field that holds call or screen content. Passing `RUST_LOG` to the child is a registered, non-secret env value, not a new one. Whether the sink prints that span at the default `warn` filter is research's question (per security-plan §Logging & Monitoring, "escher's own sink" bullet; per security-plan §Secret Management, Environment values read).
- If `covered`'s meaning or remedy is reworded, it stays a fixed non-empty `&'static str` holding no path and nothing a call supplied, and the cause set stays closed at nine (per security-plan §Error Handling, the escher-driver `Refusal` bullet).

## Patterns to follow
- The stub's call log records `seq`, `tool`, `outcome`, `cause` and never an argument value. Any replacement independent count follows that shape: closed words and counts only (per security-plan §Input Validation, row "Cold-agent MCP stub (cold_agent_stub.py)").
- `scripts/agent-run.sh`'s split: metadata events with no content-named field beside a raw, unscrubbed log that is never printed by `logs` and stays gitignored under `target/` (per security-plan §Logging & Monitoring, the agent-run bullet).
- The 2026-10-10 live trial's form: stdlib Python, one `claude -p` with the pipe's flags and `--allowedTools mcp__escher`, the same login and no new secret source (per security-plan §Secret Management, Storage → Development).
- A refusal holds nothing of the call: no `String` field in `Cause`, `Fault` or `Refusal` (per security-plan §Error Handling).

## Anti-patterns to avoid
- Never fire the live run in CI, and never give CI a credential or API key. The contract tests stay shim-only (per security-plan §Secret Management, Storage → Development).
- Never write transcript text, a tool argument, a tool result or the agent's reply into `events.jsonl`, `verdict.json` or a committed file without host-path masking (per security-plan §Logging & Monitoring, the cold-agent pipe bullet).
- Never let anything the operator or the agent supplied reach the session prompt beyond the allowlisted task's fixed text (per security-plan §Input Validation, row "CLI arguments (cold-agent.sh)").

## Contract bindings
- security ↔ tests: `.github/scripts/test_cold_agent.py` pins the CLI's exit-2 validation and the hygiene of events and verdict under a `claude` shim, with no live model and no credential in CI. The stub's row retires, or is replaced, with the stub (per security-plan §Input Validation; per security-plan §Secret Management).
- security ↔ obs: the `tool_call` span as an independent count binds obs-plan §3 (the sink's record classes) to security-plan §Logging & Monitoring's scrub rule. Its fields stay `tool` and `ending`, both closed words.
- security ↔ arch: the pipe's footprint adds no listener, port, env var or crate. The run directory, `target/cold-agent/` and the session's stdio stay arch §Occupied Resources' registration (per security-plan §API Security, its opening paragraph).
- The wrap amends §Input Validation's two cold-agent rows, §Logging & Monitoring's cold-agent bullet and §Secret Management's Development line to the measured pipe.

## Acceptance criteria contributions
- `scripts/cold-agent.sh` with a bad verb, a wrong argument count or a task outside the new stand allowlist prints usage to stderr, leaves stdout empty and exits 2 before any `claude` call. A shim test asserts this (per security-plan §Input Validation, row "CLI arguments (cold-agent.sh)").
- The live run's `events.jsonl` and `verdict.json` hold 0 scrub-set keys and no transcript text, tool argument or tool result. The committed transcript copy, if any, has every host path masked (per security-plan §Logging & Monitoring).
- The committed live evidence passes these censuses: 0 for key, auth header, credential-file name, token field and e-mail address. The session reports `apiKeySource: "none"`. The env-read census `os\.environ|getenv|ANTHROPIC_` over the pipe's scripts reads 0 (per security-plan §Secret Management).
- If a refusal text changes, the cause set is still nine, each text a fixed `&'static str` with no path, and a recorded reason is attached (per security-plan §Error Handling).
