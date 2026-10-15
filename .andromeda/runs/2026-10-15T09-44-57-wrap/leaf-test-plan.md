group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «`bash scripts/cold-agent.sh {run counter | status | cleanup | logs}` — same exit grammar; `run` starts one isolated `claude -p` session (a per-run `mktemp -d` cwd, only the stdlib stdio MCP stub's `list`/`read`/`press`)»
  source: .andromeda/test-plan.md:120 «the one task `stand` — the four lean apps in one client session (`TASKS=(stand)`; `counter` no task since 2026-10-15-cold-agent-test, so `run counter` is a usage error)»
  replace-with: «`bash scripts/cold-agent.sh {run stand | status | cleanup | logs}` — same exit grammar (`run counter` a usage error; `3` also when `target/debug/escher-session` is absent or not executable — the pipe builds nothing); `run stand` starts one isolated `claude -p` session (a per-run `mktemp -d` cwd holding only an `mcp.json` with one stdio server `escher`, the driver's own `escher-session mcp` with its eleven verbs, its stderr at `RUST_LOG=info` in `server.log`, counted by `tool_call` span lines and never printed — the stub `scripts/cold_agent_stub.py` deleted at 2026-10-15-cold-agent-test) asking for the four lean apps one at a time and a JSON check as the reply's last fenced json block»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «`isolated`, `counts_agree`, ≥ 1 tool call, the stub's own count at 3)»
  source: .andromeda/test-plan.md:124 «all four lean apps in `tasks_met` (an app is met from a `snapshot` answer while it is the app of the latest accepted `start` — an accepted `stop` clears it — never from the reply)»
  replace-with: «`isolated` — exactly the eleven `mcp__escher__*` tools, one server `escher` —, `counts_agree` — the server's `tool_call` span lines against the transcript's escher tool uses and errors —, ≥ 1 tool call, all four lean apps in `tasks_met`, read from `snapshot` answers and never the reply, and the agent's check replayed green on a fresh `escher-session mcp` child (`check_passed`) and failing with its acting steps left out (`check_can_fail`))»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «Proof: `test_cold_agent.py` under a `claude` shim + one recorded live run (passed, 5 calls, 0 wrong).»
  source: .andromeda/test-plan.md:127 «fired once: `passed`, `isolated` (the eleven `mcp__escher__*` tools, one server `escher`), `counts_agree`, `tasks_met` all four, `check_passed` and `check_can_fail` true, 43 tool calls»
  replace-with: «Proof: `test_cold_agent.py` (33) under a `claude` shim and a fake `target/debug/escher-session` + one recorded live run at 2026-10-15-cold-agent-test (passed, all four apps met, the check passed and able to fail, 43 calls, 0 wrong, 0.3197 USD; before it the retired stub's `counter` run, passed, 5 calls, 0 wrong).»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «94 CI-script tests in seven files — 79 in six until 2026-10-14-quality-gates»
  source: .andromeda/test-plan.md:8 «100 tests in seven test files (§4; 94 until 2026-10-15-cold-agent-test, whose test_cold_agent.py went 27 → 33 as `ColdAgentStubTest` retired with the stub;»
  replace-with: «100 CI-script tests in seven files — 94 until 2026-10-15-cold-agent-test, whose test_cold_agent.py went 27 → 33 as `ColdAgentStubTest` retired with the stub; 79 in six until 2026-10-14-quality-gates»

- leaf: .claude/docs/tests-summary.md:29   class: retired
  leaf-text: «`test_cold_agent.py` the cold-agent pipe and stub under a `claude` shim»
  source: .andromeda/test-plan.md:148 «runs `scripts/cold-agent.sh` in a temp root under a `claude` shim on PATH (`SHIM`, spawning the server exactly as `mcp.json` names it) and a fake `target/debug/escher-session`»
  replace-with: «`test_cold_agent.py` the cold-agent pipe under a `claude` shim and a fake `target/debug/escher-session`»

- leaf: .claude/rules/testing.md:41   class: stale
  leaf-text: «`bash scripts/cold-agent.sh run counter` → `target/cold-agent/verdict.json`; its contract tests run in the `ci-scripts` leg under a `claude` shim»
  source: .andromeda/test-plan.md:120 «the one task `stand` — the four lean apps in one client session (`TASKS=(stand)`; `counter` no task since 2026-10-15-cold-agent-test, so `run counter` is a usage error)»
  replace-with: «`bash scripts/cold-agent.sh run stand` (after `cargo build --locked -p seven_guis --bin escher-session` — the pipe builds nothing; `run counter` a usage error) → `target/cold-agent/verdict.json`; its contract tests run in the `ci-scripts` leg under a `claude` shim and a fake `target/debug/escher-session`»

- leaf: .claude/rules/verification-harness.md:5   class: retired
  leaf-text: «  - "scripts/cold_agent_stub.py"»
  source: .andromeda/test-plan.md:123 «(since 2026-10-15-cold-agent-test, which deleted the stub `scripts/cold_agent_stub.py`):** the agent talks to the driver's own MCP surface, `escher-session mcp` (the eleven verbs)»
  replace-with: «»

- leaf: .claude/rules/verification-harness.md:25   class: stale
  leaf-text: «`bash scripts/cold-agent.sh run counter`, then»
  source: .andromeda/test-plan.md:120 «the one task `stand` — the four lean apps in one client session (`TASKS=(stand)`; `counter` no task since 2026-10-15-cold-agent-test, so `run counter` is a usage error)»
  replace-with: «`bash scripts/cold-agent.sh run stand` (`run counter` a usage error), then»

- leaf: .claude/rules/verification-harness.md:25   class: omission
  leaf-text: «`3` precondition — `claude`/`python3` absent, no verdict, no events)»
  source: .andromeda/test-plan.md:121 «since 2026-10-15-cold-agent-test `target/debug/escher-session` absent or not executable, with one stderr line naming `cargo build --locked -p seven_guis --bin escher-session`»
  replace-with: «`3` precondition — `claude`/`python3` absent, `target/debug/escher-session` absent or not executable (the pipe builds nothing: `cargo build --locked -p seven_guis --bin escher-session` first), no verdict, no events)»

- leaf: .claude/rules/verification-harness.md:26   class: stale
  leaf-text: «with only the stdio stub's `list`/`read`/`press`»
  source: .andromeda/test-plan.md:122 «makes a per-run `mktemp -d` session directory outside the repository holding only an `mcp.json` with one stdio server `escher`»
  replace-with: «holding only an `mcp.json` naming one stdio server `escher`, the driver's own `escher-session mcp` (the eleven verbs; its stderr at `RUST_LOG=info` in `server.log`, counted by `tool_call` span lines, never printed or copied), with a fixed prompt asking for the four lean apps one at a time and a JSON check as the reply's last fenced json block (`--model claude-opus-5-5 --max-budget-usd 3`)»

- leaf: .claude/rules/verification-harness.md:26   class: stale
  leaf-text: «use a `claude` shim.»
  source: .andromeda/test-plan.md:148 «runs `scripts/cold-agent.sh` in a temp root under a `claude` shim on PATH (`SHIM`, spawning the server exactly as `mcp.json` names it) and a fake `target/debug/escher-session`»
  replace-with: «use a `claude` shim and a fake `target/debug/escher-session`.»

- leaf: .claude/rules/verification-harness.md:27   class: stale
  leaf-text: «`isolated`, `counts_agree`, ≥ 1 tool call, and the stub's own count at 3 (never the agent's reply)»
  source: .andromeda/test-plan.md:124 «all four lean apps in `tasks_met` (an app is met from a `snapshot` answer while it is the app of the latest accepted `start` — an accepted `stop` clears it — never from the reply)»
  replace-with: «`isolated` (exactly the eleven `mcp__escher__*` tools, one server `escher`), `counts_agree` (the server's `tool_call` span lines against the transcript's escher tool uses, its refused + error lines against the results marked `is_error`), ≥ 1 tool call, all four lean apps in `tasks_met` (read from a `snapshot` answer while the app is the latest accepted `start`'s, never the agent's reply), and the agent's check — at most 400 steps, never replayed when malformed — replayed green on a fresh `escher-session mcp` child (`check_passed`) and failing with its acting steps left out (`check_can_fail`)»

- leaf: .claude/rules/verification-harness.md:27   class: omission
  leaf-text: «no transcript text, argument or result.»
  source: .andromeda/test-plan.md:125 «no transcript text, tool argument, tool result, reply or check text reaches one»
  replace-with: «no transcript text, tool argument, tool result, reply or check text.»

- leaf: .claude/rules/verification-harness.md:28   class: omission
  leaf-text: «State lives in `target/cold-agent/` only (`cleanup` touches nothing else)»
  source: .andromeda/test-plan.md:126 «`target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}` only»
  replace-with: «State lives in `target/cold-agent/{verdict.json, events.jsonl, transcript.jsonl, client.log, server.log, check.json}` only (`cleanup` touches nothing else; `server.log` counted by span name and `check.json` written as parsed, neither ever printed or copied)»
