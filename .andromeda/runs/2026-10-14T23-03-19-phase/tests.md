# tests extract

## Relevance
relevant — the chunk is test-plan §10's own entry (a coverage floor and a flakiness budget enforced on fork CI). Its freight touches the `coverage`, `audit`, `clippy` and `ci-scripts` legs, the stand and process checks, and two blitz-dom unit tests. test-plan §10's gate line reads NOT YET MEASURED, so the plan sets no gate value. Only §10's first coverage reading is citable.

## Constraints
- test-plan §9 Coverage defines the `coverage` leg as `cargo llvm-cov --workspace --locked --lcov --output-path target/coverage/lcov.info`, then `cargo llvm-cov report --workspace --locked`, with no threshold, on cargo-llvm-cov 0.9.1 with `llvm-tools-preview`. A floor attaches to that leg. Whether the leg or `ci.yml` already passes any `--fail-under-*` flag is research's question. The leg's instrumented run excludes doctests, so a floor reads non-doctest coverage only (per test-plan §9 Local baseline).
- test-plan §10 records the only coverage reading: 53.23 % on the host, 53.25 % in CI, workspace-wide, taken 2026-10-05, before escher-driver existed. It is no per-crate baseline for the driver crates. The floor has to come from a fresh per-crate reading at HEAD.
- test-plan §9 Legs requires every linux job to run `bash .github/scripts/ci-leg.sh {leg}`, every cargo leg `--locked`, so every leg reproduces on the host. A new repeat leg, or a change to the coverage leg, goes through the script and stays host-reproducible.
  - Its log goes to `target/ci-logs/{leg}.log`, uploaded only on failure. The coverage job alone also uploads `coverage-report` on success (per test-plan §9 Failure logs).
- test-plan §4 CI workflows and leg script: `test_ci_workflows.py` pins:
  - the fast four with no `needs`, and every other job needing all four;
  - the cache-less `coverage` job and its `coverage-report` upload;
  - SHA-pinned `uses:` and a workflow-level `permissions: contents: read`;
  - each leg's presence in `ci-leg.sh` (`LegScriptTest`'s rows for `audit`, `a11y` and `coverage`).

  A leg changed or a job added must keep these invariants and extend them.
- test-plan §3 Agent-run contract `run`:
  - `run stand` is one `--test` per `tests/blitz-tests/tests/stand_*.rs`, run with `cargo test -p blitz-tests --locked`.
  - A run passes only when cargo exits 0, no test fails and at least one test line parsed.
  - Per test-plan §3 Agent-run contract State, it is one driver at a time. The session checks hold fixed state directories (`target/tmp/ss-life`, `target/tmp/ss-idle`, `target/tmp/ss-quiet`), so repeats of the stand checks cannot run concurrently in one tree.
- test-plan §2 Process-lifecycle checks:
  - Host-holding checks bound every wait and kill and reap through a guard.
  - The `#[ignore]` host children serve until their 1,800 s `IDLE_EXPIRY`, so a bare `--ignored` run of blitz-tests waits on a host for that long.
  - Socket-holding checks are `cfg(unix)`-gated, while `mcp_server` and `mcp_log` run on windows (also per §3 → Session lifecycle `session-checks`).

  Per-OS tallies therefore differ by design, which bears on CARRY 8 and on any budget read per OS.
- test-plan §9 Engine features by runner: the workspace build and the per-package `-p blitz-tests` build compile blitz-dom differently. The workspace build has 16 features (`tracing` and `writing-mode` on), the per-package build 10. A green per-package run is not the workspace leg's green, and a sink capture is counted by target, never in total. A coverage reading or a repeat measurement states which build it ran under.

## Patterns to follow
- `LegScriptTest` runs `ci-leg.sh` against a `cargo` shim on PATH and asserts the leg's cargo dispatch and its log. A floor flag or a repeat leg is pinned the same way (per test-plan §4 CI workflows and leg script).
- A gate is shown live by a control: break the guarded thing, see the named test go red, restore the file and compare its sha256. 2026-10-09-audit-corrections did this four times (per test-plan §4 CI workflows and leg script). Here the control is a floor set above the reading turning the leg red, or a deliberate flake breaking the budget. It is a hand control, never a `cargo mutants` run (scope inputs#I1).
- Repeat outcomes come from the agent-run events, never from raw cargo output: one `test {file, test, outcome}` event per libtest line, and `run.end` with counts (per test-plan §3 Agent-run contract Events).
- Counts are recorded as result lines · passed · failed · ignored at a named `ci-leg.sh fast` run, with the delta attributed per file. The last reading is 185 · 951 · 0 · 12 (per test-plan §9 Local baseline).
- Per-OS presence is read from the macOS and windows job logs file by file, with the iOS and android jobs noted as read by conclusion only. This is the witness form for CARRY 8's attribution (per test-plan §3 → Session lifecycle `session-proof`).

## Anti-patterns to avoid
- Counting a test that passes without asserting as coverage. That covers a test that passes on a conditional-only assertion (test-plan §4 What unit tests cover, Conditional assertions) and one that skips with `eprintln!` when no usable font is found (test-plan §2 Font-dependent tests; §8 Real dependencies kept). test-plan §1 blitz-dom covered behaviours lists "hover cursor" as covered. Whether `hover_state_tests` actually asserts at HEAD is research's question (CARRY 3).
- Counting an empty run, or one that ran zero tests, as a clean repeat (per test-plan §3 Agent-run contract `run`). Passing `--ignored` to a repeat runner over blitz-tests is also out, since it blocks on a host's expiry (per test-plan §2 Process-lifecycle checks).
- Treating a per-package green, or a total line count of a sink capture, as the workspace leg's reading (per test-plan §9 Engine features by runner).

## Contract bindings
- tests ↔ obs: CARRY 4 asks for a real reading of the command span's `busy` field. Its witness is a capture of the sink's span lines, counted by span and target (test-plan §2 Process-lifecycle checks; §3 Stand log format). That ties to obs-plan's command-span and log-format contract.
- tests ↔ security:
  - The `audit` leg (cargo-deny advisories, pinned by `LegScriptTest`, test-plan §4) carries CARRY 1's reach gap.
  - CARRY 6's licence gate is undecided.
  - Both bind to security-plan §Dependency Security.
  - CARRY 9's `@font-face` hint-less check binds to security-plan §Input Validation.
- tests ↔ a11y: the `a11y` leg runs six files, three of them stand checks: `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` (test-plan §9 Legs). A stand flakiness budget covers them in two legs, which binds to a11y-plan's CI gate.
- tests ↔ arch: a repeat leg or job adds no listener, port, env var or workspace crate. Any of those would be an arch §Occupied Resources registration (scope Boundaries).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh coverage` exits 0 at HEAD and non-zero, with its log in `target/ci-logs/coverage.log`, when a named driver crate reads under its floor. Each floor sits at or below that crate's line coverage measured at HEAD under the workspace build (per test-plan §9 Coverage).
- `bash .github/scripts/ci-leg.sh ci-scripts` passes, with `LegScriptTest` and `CiWorkflowTest` extended to pin the floor's dispatch and any repeat leg or job. Each pin is shown red by a control and then restored (per test-plan §4 CI workflows and leg script).
- The flakiness budget's measurement is N sequential repeats of `bash scripts/agent-run.sh run stand`, plus any seven_guis process checks it covers. Each repeat is non-empty, and failures are tallied per test from `events.jsonl` against the budget. The CI enforcement reads the same definition of a flake (per test-plan §3 Agent-run contract `run`).
- `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` are green, with the workspace count re-stated as result lines · passed · failed · ignored against 185 · 951 · 0 · 12 and every delta attributed (per test-plan §9 Local baseline).
