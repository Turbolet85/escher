# tests extract

## Relevance
partial — a README-only rewrite adds no test surface; tests binds through the local gates a README must not break, and through any built-vs-planned claim or "try it" command the README makes about the test harness.

## Constraints
- test-plan §9 (Local pre-push gate) names `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) as the gate a change passes before push; a README-only change still runs it whole.
- test-plan §9 (Legs) defines the `doc` leg as `cargo doc --workspace --no-deps` (`-D warnings`, per §9 Local baseline). Whether any workspace crate pulls the root `README.md` into rustdoc (a `#![doc = include_str!(…README.md)]`, a Cargo `readme =`) is research's question; if one does, the rewrite becomes rustdoc input and the `doc` leg is its check.
- test-plan §4 (CI workflows and leg script) describes `test_ci_workflows.py` as pinning workflow and leg-script invariants. Whether that file or any other `ci-scripts` contract test reads the root README is research's question. The answer decides whether the 64-test `ci-scripts` count can move.
- test-plan §9 (Local baseline) records the latest workspace count as 122 result lines, 444 passed · 0 failed · 5 ignored, measured at 2026-10-06-id-persistence. A change to `README.md` alone is expected to leave that count unchanged, and any drift is a finding to explain.
- test-plan §3 (Agent-run contract) defines the invocation as `bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}`, with an in-process library boot and no daemon, PID file, status endpoint, port or socket. Any "try it" line the README carries (a P4 lean per scope) must use this verb set and selection grammar (`stand` · `all` · `{name}`), and must not imply a server.
- test-plan §3 (Cold-agent run pipe) records a cold-agent pipe that runs a fresh `claude` session against a stdlib stub counter (`list`/`read`/`press`), proven by one live run that is never part of CI. A README "built vs planned" line about the cold-agent test must keep apart the pipe's reachability, which §3 records, and the cold-agent gate over escher's own driver, which scope lists as planned.

## Patterns to follow
- test-plan §1 (Coverage scope, tests/blitz-tests) and §3 (Harness construction) define the stand's built test surface: each lean task booted headless through `seven_guis::stand` at a pinned viewport with the bundled font, offline, plus the stable-id and id-persistence checks. These are the built facts a README "what works today" line may cite.
- test-plan §3 (Agent-run contract, Proof) states every count as "as measured at {report}". Any number the README states (stand events, test counts) should either carry its source the same way or be left out. Leaving it out fits "short and plain".
- test-plan §9 (Local baseline) re-counts the workspace at each chunk's `ci-leg.sh fast` run. This chunk's report reads the count again and compares it with 444 · 0 · 5.

## Anti-patterns to avoid
- Presenting a planned capability as a test-backed feature. test-plan §1 and §3 record only the stand checks, the agent-run contract and the stub-backed cold-agent pipe as built. Snapshot, diff, settle, act-by-id, CLI, MCP and screenshot have no test entry in the plan, so the README must not claim them as verified (per test-plan §1 Coverage scope).
- Describing the harness as a service (a status endpoint, a port, a daemon) in any run instructions. test-plan §3 (Agent-run contract, Invocation) rules all of these out.

## Contract bindings
- tests §3 agent-run contract ↔ obs-plan §3: the events are harness metadata, not an escher-telemetry sink. This applies only if the README describes `agent-run.sh` output, and then it must not call those events telemetry.
- tests §9 local gate ↔ the project's done-rule (CLAUDE.md "Work is not done until `ci-leg.sh fast` … and `ci-leg.sh doc` pass"): both legs gate this chunk even though it changes no code.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` exits 0, and the workspace test count stays 444 passed · 0 failed · 5 ignored over 122 result lines, unchanged by the README-only change (per test-plan §9 Local pre-push gate / Local baseline).
- `bash .github/scripts/ci-leg.sh doc` exits 0. This covers a rustdoc include of the root README if research finds one (per test-plan §9 Legs).
- If the README carries a "try it" with `bash scripts/agent-run.sh boot` then `run stand`, that sequence exits 0 with 20 `ok` stand events and a `run.end` outcome of `passed`, followed by `cleanup` (per test-plan §3 Agent-run contract).
- Every capability the README calls built maps to a test surface test-plan §1/§3 records. Every capability with no such entry is worded as planned (per test-plan §1 Coverage scope).
