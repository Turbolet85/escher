# arch extract

## Relevance
partial — arch governs where the five-command surface lives, which resources it may occupy (paths, env vars, crates, listeners) and the in-process headless-stand contract it drives; the 5-command discipline and log schema themselves are test-plan §3 / obs-plan §3 territory.

## Constraints
- Every new env var, temp/state/PID/log path, workspace crate or binary the harness introduces is an arch registration: per architecture §Occupied Resources (→ Environment variables, → Filesystem, → Names), the registry lists today's env vars, the paths written under `target/` and the crate names; anything the chunk adds goes there at wrap. A template-style `HARNESS_*` env var counts as one, and so does a state dir outside `target/`.
- No listener, port, socket or IPC endpoint: per architecture §Occupied Resources → Network ports and listeners (none) and §Inherited Defaults → API style ("no network listener"), boot/status cannot be built on a daemon plus a polled endpoint. The stand is driven in-process.
- The stand is an in-process library boot: per architecture §Standard Contracts → Headless stand, `seven_guis::stand::boot` / `boot_timer` return a fresh `Harness<DioxusDocument>` per call (pinned viewport, bundled font, offline through `DummyNetProvider`). It installs no telemetry, reads no env var and names no `blitz_net`. The contract must keep all three of those properties. Whether "boot" therefore means building test binaries plus a temp area, rather than starting a process, is research's question.
- Telemetry stays on stderr and keeps the allowlist scrub: per architecture §Cross-cutting Patterns → Logging and timing and §Standard Contracts → Telemetry bootstrap, escher binaries write the one-line text format to stderr only and the scrub redacts `CONTENT_FIELDS` at every target. A JSON-line log added inside escher-telemetry would change that published line-shape contract, so it is a contract change and cannot go in silently.
- Cargo invocations the scripts make keep the workspace's build discipline: per architecture §Conventions → Formatting and lints and §Infrastructure Patterns → CI/CD, every cargo leg is `--locked` and the dev profile (`debug = "line-tables-only"`, per §Established Decisions → Build profiles) is shared by the dev host and CI.
- The scripts do not change what CI runs unless a decision says so: per architecture §Standard Contracts → CI contracts, the `ci-leg.sh` leg set and the ci.yml job shape are pinned by `test_ci_workflows.py`. A new leg exercising `agent-run` is a CI-contract change that test must also reflect.
- The PREREQ doc must match the plan's recorded mechanism: per architecture §Established Decisions → DOM semantics, `disabled` is read two ways. Focusability parses it as a boolean. The `DISABLED`/`ENABLED` state (and so `:disabled`) and the pointer click target key on presence. Per §Standard Contracts → Dioxus DOM bridge, the bridge clears a falsy `disabled`. Whether HEAD still matches those coordinates is research's question.

## Patterns to follow
- `.github/scripts/ci-leg.sh` as the shape for a verb-dispatch bash runner, per architecture §Standard Contracts → CI contracts: one positional verb, the merged output tee'd to a log under `target/` and truncated at the start, the exit taken from the wrapped command's status, and a list of verbs printed to stderr with exit 2 on an unknown or missing verb. The dev host and CI call it the same way.
- Generated state under `target/`: per architecture §Occupied Resources → Filesystem, the harness's writes so far are `target/ci-logs/` and `target/coverage/`. A harness temp/status/log area beside them, such as a subdirectory of `target/`, is the precedent. A path outside `target/` would be a new registration.
- Integration-test conventions for any check that proves the commands, per architecture §Conventions → Tests: one behaviour per file under `tests/blitz-tests/tests/`, a `//!` doc naming the behaviour, and an assertion first that the fixture produces the condition under test.
- The `//!` doc states only the guarded defect, per architecture §Conventions → Documentation and → Tests ("opens with a `//!` doc naming the behavior and any guarded bug"). This governs the PREREQ rewrite.

## Anti-patterns to avoid
- Turning boot/status into a daemon with a status endpoint, a socket or a PID-polled server. That contradicts architecture §Occupied Resources → Network ports and listeners and the in-process stand in §Standard Contracts → Headless stand.
- Adding a Rust-source env read (for example a `HARNESS_*` var read from the stand or a test) without registering it. That goes against architecture §Occupied Resources → Environment variables, and the security allowlist also binds it.
- Writing harness log lines to stdout from an escher binary, or adding a content-bearing field to an escher-telemetry event. Both go against architecture §Cross-cutting Patterns → Logging and timing.

## Contract bindings
- arch §Occupied Resources ↔ security-plan §Secrets/env-read allowlist: a new env var or state path is both an arch registration and possibly a security-plan amendment. Which one depends on whether the read sits in Rust source or only in the shell script (a P4 lean).
- arch §Standard Contracts → Telemetry bootstrap ↔ obs-plan §3 (line format, log location): a JSON-line format inside escher-telemetry changes the arch-recorded line shape. A harness-level JSON log leaves it alone.
- arch §Standard Contracts → Headless stand / Test harness ↔ test-plan §3 (the 5-command discipline): the scripts drive the stand through the published `stand` + `Harness` API and add no new public surface to either crate.
- arch §Standard Contracts → CI contracts ↔ `test_ci_workflows.py`: only engaged if a P4 decision adds a CI leg for the scripts.
- arch §Project Intent (the framework's own driver becomes the harness) ↔ the future driver CLI/MCP. The verb contract should not foreclose that later swap.

## Acceptance criteria contributions
- (arch) The five verbs open no listener, port or socket, and start no long-lived daemon. A grep of `scripts/agent-run.{sh,ps1}` and any new Rust finds no `TcpListener`/`bind(`/`listen(` (per architecture §Occupied Resources → Network ports and listeners).
- (arch) Every path the scripts create, including temp area, status/PID file and log, lies under `target/` or is listed as a registration for the wrap. Every env var they read or set is either already in the registry or listed for registration (per architecture §Occupied Resources → Filesystem / → Environment variables).
- (arch) `seven_guis::stand` and `blitz-test-harness` public signatures are unchanged by this chunk, and the stand still installs no subscriber and reads no env var (per architecture §Standard Contracts → Headless stand / → Test harness).
- (arch) The corrected `//!` doc of `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` claims only the `:disabled`/DISABLED-state/click-target defect and no focus-order exclusion, and the test body is byte-identical (per architecture §Established Decisions → DOM semantics and §Conventions → Tests).
