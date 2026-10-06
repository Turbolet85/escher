# Codebase Research — 2026-10-06-cold-agent-run-pipe

## Scope
- **Depth:** moderate · **Reads:** 9 (agent-run.sh, agent-run.ps1 by size, test_agent_run.py 1-120, verification-harness.md, testing.md, code-graph-cookbook.md, playbook.md 30-60, `claude --help` sections, matrix v010-15) · **Globs/Greps:** 6 · **Live probes:** 1 (a second was denied by the operator's permission prompt and was not retried)
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full: the 5-command contract and its one Session Addition (exit 124/137 is read by the gate tool as its own bound firing). `.claude/rules/testing.md`, read in full: its one Session Addition says never to prefix a unittest helper with `test_`. Both are applied: the pipe's contract tests follow the testing rule, and no gate entry here expects exit 124.
- **Platform issues consulted:** none — no runner-only bullet. Setup 5a's CI read was in progress, not red.
- **External inputs:** none. Every fact this chunk turns on lives in this repository or in the host's `claude` binary, which research reads as a platform behaviour (its `--help` and one probe run).

## Files inspected
- `scripts/agent-run.sh` (full, 228 lines) is the grammar precedent:
  - `VERBS` array plus `usage()` writing to stderr with exit 2 (`:8-17`).
  - An embedded python3 program in a `read -r -d '' … <<'PY'` heredoc doing all JSON encoding (`:26-147`), called through `py()` (`:149-151`).
  - State lives in `STATE_DIR=target/agent-run` (`:9`). `boot` does `rm -rf` and recreates it (`:168-170`), and `cleanup` runs `rm -rf "$STATE_DIR"` and touches nothing else (`:217`).
  - "An empty run is never a pass": `passed = cargo_exit == 0 and counts["failed"] == 0 and len(tests) > 0` (`:131`).
- `scripts/agent-run.ps1` — 4 lines, a pass-through (`wc -l`).
- `.github/scripts/test_agent_run.py` (1-120 of 270) is the shim pattern:
  - `AgentRunTest.setUp` builds a temp root with `bin/` on PATH (`:89-96`).
  - `shim()` writes a `cargo` script that logs its argv and prints canned output (`:101-109`).
  - `agent_run()` runs the real script with the shim first on PATH (`:117-120`).
  - `SCRUB_KEYS` (`:15`) and `PANIC_TEXT` (`:16`) back the census and leak assertions.
- `.claude/rules/verification-harness.md` (full) — the stated contract, mirroring arch §Standard Contracts → CI contracts.
- `.github/scripts/test_ci_workflows.py:151` — `self.assertNotIn("secrets.", self.text)` pins that ci.yml references no secret (grep `secrets\.`).
- `.gitignore:1,22` — `/target` and `target/`, so any path under `target/` is gitignored (`grep -n '^/\?target' .gitignore`).
- `.andromeda/playbook.md:35-38` — pattern "Boundary widening — a chunk WIDENS what crosses an already-hardened boundary (… a subprocess/IPC boundary gains a new crossing)", `verdict: escalate`, "always a human's call".
- `escher-0.1.0/verification-matrix.json` v010-15, read by `matrix.py show --id v010-15`: method `dynamic-external`, acceptance "A fresh agent with no prior context, given only the tool, completes a stand task and writes a check that passes; the run records its count of wrong calls.", `chunk: None`.
- `claude --help` (Claude Code 2.1.288 on PATH at `~/.local/share/mise/installs/claude/latest/claude`):
  - `--bare`: "Anthropic auth is strictly ANTHROPIC_API_KEY or apiKeyHelper via --settings (OAuth and keychain are never read)".
  - `--safe-mode`: disables CLAUDE.md, skills, plugins, hooks, MCP servers and more; "Auth … work[s] normally".
  - `--tools ""`: "disable all tools".
  - `--strict-mcp-config`: "Only use MCP servers from --mcp-config".
  - `--setting-sources`: "user, project, local".
  - `--disable-slash-commands`: "Disable all skills".
  - `--no-session-persistence`, `--max-budget-usd` (print only), `--output-format stream-json` (needs `--verbose`), `--model`.
  - `--exclude-dynamic-system-prompt-sections`: MOVES the cwd, env and memory paths into the first user message; it does not remove them.
- Host auth (`claude auth status`, email redacted in the read): `"authMethod": "claude.ai"`, `"subscriptionType": "max"`, and `ANTHROPIC_API_KEY` unset (`[ -n … ]` test, value never read).

## Live probe (the isolation equality)
The pipe's load-bearing equality is: given the isolation flags, a session's context contains only the given tool and the task, and nothing from this repository or the operator's instructions.

Probe 1 command: `claude -p … --output-format stream-json --verbose --tools "" --strict-mcp-config --mcp-config '{"mcpServers":{}}' --setting-sources "" --disable-slash-commands --no-session-persistence --max-budget-usd 0.25`, run from the session scratchpad (cwd `…/-home-turbolet-dev-projects-escher-…/scratchpad/probe1`). It exited 0 with 7 stream-json lines and an empty stderr.

The `system/init` event:
- `tools = []`, `mcp_servers = []`, `slash_commands = []`, `skills = []`.
- `plugins` = the three `builtin` plugins only (`cc-plugin-agents-md`, `cc-plugin-telemetry`, `cc-plugin-plugin-authoring`).
- `agents` = the five built-in agent types (no Agent tool is present to use them).
- `model = "claude-opus-5-5"`, `apiKeySource = "none"`, `permissionMode = "default"`.
- `memory_paths.auto` = a fresh per-cwd path.

The `result` event: `subtype success`, `num_turns 1`, `total_cost_usd 0.018955`, `duration_ms 2025`.

The model's answer: tools `NONE` · "Global Preferences" (the operator's `~/.claude/CLAUDE.md` heading) `NO` · mentions "escher" `YES`.

Reading:
- **Verified:** tools, MCP, skills, slash commands and the user-level CLAUDE.md are all absent under these flags.
- **Falsified as stated:** "sees nothing of this repository". The cwd path itself names the project (`-projects-escher-`) and reaches the system prompt (the `--help` text for `--exclude-dynamic-system-prompt-sections` names cwd and memory paths as prompt sections). A session cwd whose path does not name the project is therefore part of the isolation, not a nicety.
- **Not measured:** a probe from a neutral `mktemp -d /tmp/…` cwd was denied at the permission prompt, so whether the `YES` came from the cwd and nothing else is unmeasured. The implemented pipe proves it in its recorded run (a context check in the verdict, see Open questions).

## Graph impact
- **`boot`** (`seven_guis::stand`, `callee_file LIKE '%stand.rs'`) has 11 callers, all `#[test]` fns in `tests/blitz-tests/tests/stand_{boot,counter,crud,flight_booker}.rs` (trace `tree-query-2026-10-06-cold-agent-run-pipe.json`, rows 11). A stub task that does not touch the stand leaves this surface untouched. The pipe's modify set is shell and python, with no Rust symbol on the rust plane in the recommended design. A Rust stub crate would be a new leaf crate with zero inbound edges.

## Patterns detected
- **Verb script with embedded python for JSON** (`scripts/agent-run.sh:26-151`): bash owns argv, exits and process control, and python3 `json` owns every emitted byte. The pipe's events and verdict take the same split.
- **Shim on PATH** (`test_agent_run.py:101-120`): the external process (`cargo` there, `claude` here) is replaced by a script that logs its argv and replays canned output. A `claude` shim replaying a recorded stream-json transcript proves transcript capture, wrong-call counting and verdict paths with no live model, as test-plan §4 requires.
- **Positive-evidence pass** (`agent-run.sh:131`): pass needs exit 0, zero failures and at least one parsed result. The verdict mirrors this: pass needs a `result` event with `subtype success`, a satisfied task check, and at least one tool call in the transcript.
- **Disjoint state areas** (`agent-run.sh:9,168,217`): `cleanup` removes exactly its own directory. A sibling `target/cold-agent/` keeps `agent-run.sh cleanup` and the pipe's cleanup disjoint.
- **stream-json as the transcript**: one JSON object per line (`system/init`, `assistant` with `tool_use` blocks, `user` with `tool_result` blocks, `result`), with tool inventory and model in `init` and cost and turns in `result`. That is everything the verdict needs, measured in probe 1's 7 lines.

## Conventions to follow
- **Exit grammar 0/1/2/3, usage before precondition** (`agent-run.sh:12-17,161`; test-plan §3).
- **No content-named key in events**: `SCRUB_KEYS` (`test_agent_run.py:15`) is the census set. The verdict's transcript pointer takes a non-scrub name (for example `transcript_file`), never `path`.
- **unittest helpers without the `test_` prefix** (`.claude/rules/testing.md` Session Additions).
- **The `.ps1` is a pass-through with no logic** (`scripts/agent-run.ps1`, 4 lines).

## New files to create
- `scripts/cold-agent.sh` — the pipe: verbs, isolation flags, transcript capture, wrong-call count, verdict (embedded python3 for JSON)
- `scripts/cold-agent.ps1` — Windows pass-through to cold-agent.sh
- `scripts/cold_agent_stub.py` — the stub tool: a stdlib-only MCP stdio server with a minimal verb set, checked arguments, cause-naming refusals and a call log
- `.github/scripts/test_cold_agent.py` — contract tests driving cold-agent.sh against a `claude` shim that replays canned stream-json, with no live model
- `escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/evidence/` — the one recorded live green run: verdict, transcript, stub call log

## Files to modify
- none

## Open questions
- Agent client and credential: the operator's own Claude Code login (`claude -p`, OAuth, `apiKeySource none`, so escher reads no secret) or a direct model-API loop with an `ANTHROPIC_API_KEY` env var (`--bare` also needs one) → blocks: plan-decision. Either way it is a new outbound crossing (playbook "Boundary widening", escalate).
- Stub form: an MCP stdio server (`--mcp-config` plus `--strict-mcp-config`, `--tools ""`; a new local IPC crossing) or a CLI the agent reaches through a Bash allowlist (`--tools Bash` plus `--allowedTools`, which gives a general shell that is narrowed but not "only the tool") → blocks: plan-decision.
- Where the live run happens: operator host only, recorded as evidence, or a fork CI job, which needs the fork's first secret and breaks `test_ci_workflows.py:151` → blocks: plan-decision.
