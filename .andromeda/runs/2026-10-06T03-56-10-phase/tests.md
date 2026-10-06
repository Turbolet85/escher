# tests extract

## Relevance
relevant — the pipe is Foundation test infrastructure that extends or sits beside the 5-command agent-run contract, and it needs its own contract tests that need no live model (test tier `0` per test-plan §1 Test Scope Summary).

## Constraints
- The invocation surface keeps the agent-run grammar. test-plan §3 Agent-run contract → Exit grammar requires `0` success · `1` ran and failed · `2` usage · `3` precondition unmet on every verb, with a usage error checked before not-booted. §3 Events requires JSON lines on stdout, encoded by python3's `json` module. Whether the pipe is a new `agent-run.sh` verb or a sibling script, it follows this grammar.
- State lives only under `target/`. test-plan §3 State requires `target/agent-run/{status.json, events.jsonl, run.log}` and nothing else, gitignored. §3 `cleanup` requires idempotent removal that touches nothing else under `target/`. Transcript and verdict paths need the same discipline, and any new path is an arch §Occupied Resources registration.
- test-plan §3 Agent-run contract → Invocation states that the script reads no environment variable of its own and holds no daemon, port or socket. A credential env var or an MCP stdio stub would break that clause, so it must be amended, not added silently.
- test-plan §3 `run` requires that an empty run is never a pass: the outcome is `passed` only on positive evidence (exit 0, no failure, at least one parsed result). The verdict follows the same rule: a session that does not complete the stub task, or that yields no countable transcript, reads `fail` with exit 1.
- test-plan §3 Events requires that nothing between `failures:` and the next `test result:` is read, so captured stdout and panic text never reach an event, and that events are harness metadata, not an escher-telemetry sink. Transcript content (model output, tool I/O) therefore stays in its artifact file and never reaches the JSON-line events stream.
- test-plan §4 Agent-run contract and §1 Coverage scope (CI Python scripts) place the contract tests in `.github/scripts/test_*.py`, run by `python3 -m unittest discover -s .github/scripts` in the fast `ci-scripts` leg. test-plan §9 Local pre-push gate requires `bash .github/scripts/ci-leg.sh fast` to run them.
- If the pipe joins `agent-run.sh`, test-plan §3 Invocation requires `scripts/agent-run.ps1` to remain a pure pass-through that holds no contract logic. It is not run on the dev host (`pwsh` absent).

## Patterns to follow
- The `AgentRunTest` shim pattern (test-plan §4 Agent-run contract): run the script in a temp dir with a scripted shim on PATH that records every call, with fake input files. For this chunk, the agent client takes the place of `cargo`: a client shim emits a scripted or replayed session, so plumbing, wrong-call counting and verdict paths are proven with no live model.
- `LegScriptTest` (test-plan §4 CI workflows and leg script): drive the real script against a shim, then assert on the exit code, the written log and its truncation at start.
- Deterministic hand-written stand-ins (test-plan §8 Hand-written fakes and stubs: `ManualNetProvider`, `RecordingNetProvider`, `TimerTicks`, the `--dry-run` GitHub API stand-in). The stub tool and the scripted session are hand-written and deterministic, and they record their calls. No mocking library is used (§8 Observed absent).
- If the stub task touches the stand, boot it through `seven_guis::stand::boot` / `stand::options(incremental)`, the pinned drive surface (test-plan §3 blitz-test-harness `Harness` → Construction).
- Live proof is cited as measured evidence: test-plan §3 Proof records a live run in the chunk's `report.md` alongside the contract tests. The recorded green run is committed the same way.

## Anti-patterns to avoid
- Reading a run with no parsed outcome as a pass, or a verdict without positive evidence (test-plan §3 `run`: "an empty run is never a pass").
- Letting captured output, transcript content or a scrub-set key into the JSON-line events or `logs` output (test-plan §3 Events; §4 Agent-run contract: "none carrying a scrub-set key").
- Contract tests that call the real external process. The §4 precedent shims it on PATH. A live model run belongs only to the one recorded evidence run.

## Contract bindings
- tests ↔ obs: test-plan §3 Events binds the JSON-line events as harness metadata, not an escher-telemetry sink (obs-plan §3). The transcript and verdict are artifacts on the same side of that line.
- tests ↔ security: test-plan §4 CI workflows and leg script pins "no `secrets.`" in ci.yml through `test_ci_workflows.py`. A pipe that runs in fork CI with a provider secret would break that pin and needs a security-plan amendment first. test-plan §3 Invocation ("reads no environment variable of its own") binds to the security rules' env-read allowlist for any credential path.
- tests ↔ arch: test-plan §3 State (paths under `target/`) and §3 Invocation (no port, socket or daemon) bind to arch §Occupied Resources for transcript paths, any new script, and an MCP stdio endpoint.

## Acceptance criteria contributions
- The pipe has contract tests in `.github/scripts/test_*.py` that drive it against a client shim, with no live model. They cover the pass verdict, the fail verdict on a session that does not complete the task, the wrong-call count from a scripted transcript, and refusals with exit 2 (usage) and exit 3 (precondition). `bash .github/scripts/ci-leg.sh fast` passes, with the `ci-scripts` count above its 37 (per test-plan §4 Agent-run contract · §9 Local pre-push gate).
- A run where the agent fails the stub task, or yields no countable transcript, exits 1 with a `fail` verdict and is never green (per test-plan §3 `run`).
- No JSON-line event or `logs` output carries transcript content, captured output or a scrub-set key, and the existing 14 `test_agent_run.py` tests stay green if the pipe joins `agent-run.sh` (per test-plan §3 Events · §4 Agent-run contract).
- One live green run is committed as evidence under the chunk dir and cited "as measured at" in its report (per test-plan §3 Proof).
