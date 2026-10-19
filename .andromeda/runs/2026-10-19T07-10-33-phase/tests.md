# tests extract

## Relevance
relevant — the chunk adds replay checks as ordinary tests under agent-run, CI and `stand-repeat`, which is test-harness, CI-leg and quality-gate surface.

## Constraints
- `run stand` selects one `--test` per `tests/blitz-tests/tests/stand_*.rs` present, so a new replay target named `stand_*` joins with no script change. A run counts as `passed` only when cargo exits 0, no test fails and at least one test line parses. An empty run is never a pass (per test-plan §3 Agent-run contract, `run {selection}`).
- The `stand-repeat` leg runs three rounds. Each round runs one `--test` per `stand_*.rs` file, then `cargo test -p seven_guis --locked` over an explicit list of process checks (`recording` is among them). A new seven_guis replay target joins only if the list names it, and `test_stand_repeat_leg_runs_three_rounds` pins those words. The first red command ends the leg: no retry, no `--ignored`, no env var (per test-plan §9 Legs).
- The stand checks hold a zero-failure budget across the three `stand-repeat` rounds. A replay check must read the same way on every round, with no timing- or order-dependent verdict (per test-plan §10 Flakiness budget).
- A check that records is `cfg(unix)`-gated item by item. Beside it sits an off-unix check, run by the windows job alone, that a recording is refused `Unsupported`. A check that needs a session process must:
  - bound every wait;
  - hold the host or `mcp` child behind a guard that kills and reaps it;
  - send the child's streams to null unless it reads them.

  Per test-plan §3 `session-checks` (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`).
- A replayed step goes through `Session::run`, which validates first, so a refused call has run nothing. `Check::from_bytes` reads a kept check strictly and refuses anything else as `malformed`. Its closed key list is what the chunk's `answer` key must widen (per test-plan §3 `session-start`). Whether the reader already admits `answer` is research's question; the scope says it refuses it today.
- blitz-tests targets are discovered automatically. `test_blitz_tests_targets.py` asserts that `autotests` is not set to false and that the manifest holds exactly one `[[test]]` table, `all`. A new replay file must be a plain `tests/blitz-tests/tests/*.rs` target, not a new `[[test]]` table (per test-plan §4 blitz-tests targets).
- The headless-stand checks have no skip path. They assert unconditionally under the bundled DejaVu Sans with system fonts off, so a replay that boots the stand cannot skip on a missing font (per test-plan §2 Font-dependent tests).

## Patterns to follow
- Use the shared session module `tests/blitz-tests/tests/session_common/mod.rs`. It already has `hold(task, incremental)`, `replay(session, check)` (each kept step run again, giving its acceptance and digest), `kept(check)` and the unix-only `mode(path)`. A new `stand_*` replay check declares `mod session_common;` after its `use` block, as `stand_session_recording` does (per test-plan §3 blitz-tests (session checks)).
- Run every stand check in both layout modes. `stand_session_recording` replays on each of the four lean tasks with incremental layout off and on, and compares every step's acceptance and digest (per test-plan §1 Coverage scope, tests/blitz-tests).
- seven_guis process checks drive the real `escher-session` binary:
  - CLI recordings go through the shared runner in `examples/seven_guis/tests/common/mod.rs`, which bounds the run and drains both streams.
  - The `mcp` role is spoken to through the shared `Speaker`, with its 30 s `REPLY_BOUND`, kill and reap in `Drop`.
  - A host that `start` spawns is held behind a guard that stops it.

  Per test-plan §2 Process-lifecycle checks and Command-line checks and shell flows.
- The cold-agent judge shows the pattern for proving that a check can fail. It replays the check in full on a fresh `escher-session mcp` child, then replays it ablated on a second fresh child (`check_passed` / `check_can_fail`). That is a model for a replay that must read red when behaviour changes (per test-plan §3 Cold-agent run pipe, the server and the check's judge).
- `host_log` and `mcp_log` each run a recording host and a recording `mcp` role at `RUST_LOG=trace`. They read 0 of the recording needles on stderr, and the state directory still holds `session.sock` alone. That is the existing witness that a kept check is never logged (per test-plan §5 Session host ↔ what it writes).

## Anti-patterns to avoid
- Treating an empty or zero-test run as a pass, or a replay that never reads red as proof. An empty run is `failed`, exit 1 (per test-plan §3 Agent-run contract, `run {selection}`).
- Letting a child's own libtest lines reach the parent's output, where `agent-run.sh` would count them as tests. Also avoid a bare `--ignored` run that waits on a host for its idle expiry (per test-plan §2 Process-lifecycle checks).
- Adding a retry, `--ignored` or an env var to `stand-repeat` to make a replay pass (per test-plan §9 Legs).

## Contract bindings
- tests ↔ obs: the never-logged rule for a kept check is witnessed by `host_log` and `mcp_log`. A kept `answer` key adds screen readings at rest, and these checks must still read 0 needles on stderr (per test-plan §5 Session host ↔ what it writes).
- tests ↔ security: `stand_session_recording` asserts owner-only modes through `mode(path)` (`0600` check, `0700` directory). A check committed as test input is not owner-only in git. How a committed fixture squares with that is P4's question (per test-plan §3 blitz-tests (session checks)).
- tests ↔ CI: `stand-repeat`'s file list and the seven_guis process-check list in `.github/scripts/ci-leg.sh` are pinned by `test_stand_repeat_leg_runs_three_rounds` in `.github/scripts/test_ci_workflows.py` (per test-plan §9 Legs).
- tests ↔ arch: the session-lifecycle key file's `session-start` and `session-proof` rows restate the kept-check form and the per-file test counts. Both move at the wrap with the `answer` key and the new replay targets (per test-plan §3 `session-proof`).

## Acceptance criteria contributions
- `bash scripts/agent-run.sh run stand` picks up the new `stand_*` replay target by its prefix with no script change. Its `run.end` reads failed 0, and the passed count rises by the new tests (per test-plan §3 Agent-run contract, `run {selection}`).
- `bash .github/scripts/ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green`, with the replay targets included. Any new seven_guis replay target is named in the leg and in `test_stand_repeat_leg_runs_three_rounds` (per test-plan §9 Legs; test-plan §10 Flakiness budget).
- Two committed checks are replayed: one recorded through the command line, one through the `mcp` role. Each replays green on the unchanged stand in both layout modes, with every step's acceptance and digest equal. Each reads red against an app whose behaviour changed under it, with no hand edit of the check (per test-plan §3 `session-proof`).
- The coverage leg still ends `coverage floor: 3 of 3 crates at or above their floors`, with `packages/escher-driver` at 97 % or above after the replay code and the `answer` key land. The off-unix `Unsupported` checks stay green on the windows job (per test-plan §10 Coverage floors; test-plan §3 `session-checks`).
