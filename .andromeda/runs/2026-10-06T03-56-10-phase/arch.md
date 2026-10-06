# arch extract

## Relevance
partial — the pipe is tooling beside the engine, not engine code, but arch governs where it lives, which resources it may occupy (listener, env var, outbound host, path, crate, process), and the agent-run contract grammar it extends or sits beside.

## Constraints
- The workspace has no network listener (per arch §Occupied Resources → Network ports and listeners; arch §Inherited Defaults → API style). The stub tool must therefore be reached over stdio or by process exec only: an MCP server over stdio, or a CLI the agent shells to. A TCP, HTTP or Unix-socket transport would be a new occupied resource that needs a registration first.
- arch §Occupied Resources → Environment variables lists the env reads that exist today: `RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML` and the CI script's vars. arch §Occupied Resources → Outbound hosts lists the hosts reached today, and no model-provider host is among them. A provider-key env var, a model or version pin read from env, or the provider's API host would each be a new entry. Each is a wrap amendment to that section, and a credential path is also the security plan's (see Contract bindings).
- arch §Occupied Resources → Filesystem gives the agent-run contract `target/agent-run/` (`status.json`, `events.jsonl`, `run.log`). `boot` recreates it, and `cleanup` removes it while touching nothing else under `target/`. The pipe's transcript, call log and verdict need their own registered location. Two placements are possible:
  - Under `target/agent-run/`, which extends that file set.
  - In a sibling `target/` directory. Then the existing `cleanup` must still leave it untouched, and the pipe's own cleanup must leave `target/agent-run/` untouched.
  The committed evidence copy goes under the chunk's `evidence/` directory, following the precedent of the evidence paths arch cites throughout §Established Decisions and §Infrastructure Patterns.
- arch §Standard Contracts → CI contracts defines the agent-run test contract, which is `bash scripts/agent-run.sh {boot | run … | status | cleanup | logs}`:
  - It runs from the repository root.
  - Exit codes are 0 success, 1 failed (an empty run included), 2 usage (checked before 3), and 3 precondition unmet.
  - Its JSON-line events are encoded by an embedded python3 `json`, and its state stays in `target/agent-run/` only.
  - `scripts/agent-run.ps1` is a pass-through with no logic.
  - `test_agent_run.py` pins it, and the `ci-scripts` leg runs that test.
  A new verb changes this contract and the test that pins it. A separate script that follows the same grammar leaves it unchanged.
- A new workspace crate for the stub is a §Occupied Resources → Names registration. arch §Conventions → Manifests requires dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`, with in-repo path entries `default-features = false`. Following arch §Inherited Defaults → Publishability, it would be `publish = false` like `escher-telemetry`. Its directory would join arch §Infrastructure Patterns → Directory structure.
- A new escher binary installs `escher_telemetry::init`, which writes to stderr only and never to stdout (per arch §Cross-cutting Patterns → Logging and timing; arch §Standard Contracts → Telemetry bootstrap). For a stdio MCP stub, stdout is the protocol channel, so no telemetry or `println!` output may reach it.
- If the stub task touches the stand, it boots through `seven_guis::stand` (`boot` / `boot_timer` with `options(incremental)`). That path is offline through `DummyNetProvider`, uses the pinned 800×600 Light viewport and the bundled font, installs no telemetry and reads no env var (per arch §Standard Contracts → Headless stand). Whether the stub touches the stand at all is P4's call.

## Patterns to follow
- Follow the agent-run grammar for the pipe's invocation surface: JSON lines on stdout, the 0/1/2/3 exit grammar with usage checked before precondition, state under `target/` only, and a no-logic `.ps1` pass-through (arch §Standard Contracts → CI contracts).
- Write contract tests the way the existing CI scripts do. They are Python `unittest` with one `TestCase` per function (arch §Conventions → Tests), pinned like `test_agent_run.py` and run by the existing `ci-scripts` leg of `.github/scripts/ci-leg.sh` (arch §Standard Contracts → CI contracts; arch §Infrastructure Patterns → CI/CD). They need no live model and add no new leg.
- Treat raw-artifact files like `run.log` and the CI leg logs: unscrubbed, never printed, local only, and uploaded by no CI job (arch §Occupied Resources → Filesystem). The transcript is the same kind of raw artifact, not a telemetry stream.
- Any new script or crate opens with a `//!` module doc, or the script equivalent, stating what it is (arch §Conventions → Documentation).

## Anti-patterns to avoid
- Serving the stub, or the client harness, over a port or socket. This breaks the "none" entry in arch §Occupied Resources → Network ports and listeners and the "no network listener" API style in arch §Inherited Defaults.
- Adding an env var, outbound host, `target/` path, workspace crate or long-lived process without its arch §Occupied Resources entry. The CLAUDE.md Critical Warnings call that a "silent add".
- Breaking the existing contract, for example by letting the pipe's cleanup remove or rewrite `target/agent-run/` contents it does not own, or by adding logic to `agent-run.ps1` (arch §Occupied Resources → Filesystem; arch §Standard Contracts → CI contracts).

## Contract bindings
- arch §Occupied Resources ↔ security:
  - A credential path (operator client login, env-held key or CI secret), the provider outbound host and any new env read are arch registrations that the security-plan amendment must come before.
  - The `github.repository` guard pattern in arch §Occupied Resources → CI infrastructure applies if a CI job reaches a secret.
- arch §Standard Contracts → CI contracts ↔ tests: test-plan §3 holds the full agent-run contract, and `test_agent_run.py` pins it. A new verb or a sibling script is amended in both places.
- arch §Infrastructure Patterns → CI/CD ↔ tests and security. Any CI job for the pipe must meet these rules, all asserted by `test_ci_workflows.py`:
  - It sits under the workflow-level `permissions: contents: read`.
  - It respects the fast/slow `needs` split.
  - Every `uses:` is pinned to a 40-hex commit SHA, and every `dtolnay/rust-toolchain` step names its `toolchain`.
  - It uploads only its own log.
  Whether such a job is in scope is a P4 fork.
- arch §Standard Contracts → Telemetry bootstrap ↔ obs: if the stub is a Rust binary, it uses the stderr-only sink, and the transcript is never routed into the escher telemetry stream.

## Acceptance criteria contributions
- The stub is reached only over stdio or by process exec. No `TcpListener`, `UdpSocket`, `bind(` or `listen(` call is introduced by the chunk's files (per arch §Occupied Resources → Network ports and listeners).
- Every resource the chunk introduces is in the wrap's arch §Occupied Resources amendments: each new `target/` path or committed evidence path under Filesystem, each env var under Environment variables, the provider host under Outbound hosts, each crate or binary under Names, and any spawned server process under Process-wide state and threads. None is left unregistered (per arch §Occupied Resources).
- `test_agent_run.py` stays green under `bash .github/scripts/ci-leg.sh ci-scripts`. If the pipe is a new `agent-run.sh` verb, the contract tests are extended to cover it. If it is a separate script, `agent-run.sh cleanup` still leaves the pipe's `target/` directory untouched, and the pipe's cleanup leaves `target/agent-run/` untouched (per arch §Standard Contracts → CI contracts; arch §Occupied Resources → Filesystem).
- Any new escher Rust binary calls `escher_telemetry::init(escher_telemetry::service_identity!())` at start and writes no diagnostics to stdout (per arch §Cross-cutting Patterns → Logging and timing).
