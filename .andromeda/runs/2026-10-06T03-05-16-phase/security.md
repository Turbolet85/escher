# security extract

## Relevance
partial — the chunk adds no trust boundary, auth, served surface or secret. It touches the plan in four places: env reads (the five-command scripts), log emission (the JSON-line log), a possible new dependency (a test-runner or output-parsing crate), and the `disabled` mechanism the PREREQ doc fix restates.

## Constraints
- Per security-plan §Secret Management (Environment values read), the env values source may read are a closed list: `CARGO_MANIFEST_DIR`, `HOME`, `PAINT_TREE_BENCH_HTML`, `WPT_DIR`, `RUST_LOG`, none of them secret-bearing. Any new env read in Rust source (for example a `HARNESS_*` variable read by a test or harness crate) needs a security-plan amendment, recorded at the wrap and flagged in the plan. Env reads confined to `scripts/agent-run.{sh,ps1}` fall outside the plan's "source" wording; P4 states that lean explicitly. No env read, in either place, may carry a credential.
- Per security-plan §API Security, no served API surface exists (no `TcpListener`/`bind`/`listen`). The in-process stand needs none, so boot/status must not open any socket, port, IPC endpoint or status endpoint. Adding one is a security-plan amendment before any code, never a part of this chunk.
- Per security-plan §Logging & Monitoring (Log format and backends), escher's sink scrubs by allowlist. These fields are redacted at any target: `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload`. The chained std panic hook still prints the raw panic message to stderr, and test diagnostics go out through `println!`/`eprintln!`. Two cases follow:
  - If the JSON-line log routes through escher-telemetry, it inherits that scrub and must not widen the allowlist.
  - If it is harness-level, it must not copy captured test stdout/stderr or panic payloads into JSON fields. It carries harness metadata only: verb, test name, outcome, counts, timing.
- Per security-plan §Bootstrap phases (`logging-redaction-wire`), redaction counts as done for escher's own sink only, and the upstream `fmt::init()` and `env_logger` outputs stay unscrubbed. A `logs` verb that collects those streams (for example a WPT or upstream-app run) must not present them as scrubbed.
- Per security-plan §Dependency Security (Audit tool · Pinning · Supply chain integrity), these apply to any crate or tool this chunk adds (a libtest-output parser, a JSON crate, a runner):
  - it is checked by cargo-deny (`bash .github/scripts/ci-leg.sh audit`), with per-ID ignores that carry a written reason;
  - `Cargo.lock` is committed and the build passes `--locked`;
  - a git dep is pinned by `rev`;
  - a CI-installed tool is version-pinned.
  The audit's reach stops at cargo-deny's resolved graph, so a new optional-chain crate also gets a review by hand.
- Per security-plan §Input Validation (`disabled` row), the plan records two separate mechanisms: `disabled` is parsed as a bool for focusability, while presence alone sets `DISABLED`, `:disabled` and the click and pointer-selection exclusions. The PREREQ's corrected `//!` wording must match that row. Whether blitz-dom at HEAD still matches the row's coordinates is research's question.

## Patterns to follow
- Per security-plan §Error Handling (External responses), CLI tools write an stderr message and exit non-zero (status 1). The five verbs follow the same shape: a machine-readable result on the output channel, the diagnostic on stderr, a non-zero exit on failure.
- Per security-plan §Logging & Monitoring, escher binaries log to stderr only, through `escher_telemetry::init`, and never to stdout. The scripts' stdout (status lines, `logs` JSON lines) is the harness's output channel and stays distinct from any binary's log sink.
- Per security-plan §Error Handling (Graceful degradation), new input paths (parsing libtest output, reading status/PID files) handle failure as a typed `Result` or a reported not-ready state, never a panic or a silent pass.

## Anti-patterns to avoid
- No new user-content log field in the JSON-line schema. Naming a field `path`, `error`, `text` or `value` collides with the content-named scrub set (per security-plan §Logging & Monitoring). Use non-colliding names such as `test`, `file`, `outcome` or `reason`, or confirm the harness-level channel bypasses the scrub on purpose and carries no content.
- No listener or daemon-style status endpoint, even though the setup template's skeleton assumes a daemon (per security-plan §API Security).
- No unregistered env read in Rust source (per security-plan §Secret Management).

## Contract bindings
- security §Logging & Monitoring ↔ obs-plan §3/§6: the JSON log schema and where the log file lives. The scrub set limits which field names the schema can use, and obs owns the schema itself.
- security §Secret Management ↔ arch §Occupied Resources: each new env var or temp/state path is registered in arch §Occupied Resources. A new env read in Rust source also needs the security-plan amendment, both at the wrap.
- security §Dependency Security ↔ tests (CI audit leg): a dependency added for per-test result parsing must pass the `audit` leg that ci.yml runs on every push.
- security §Input Validation (`disabled` row) ↔ a11y-plan §5: the PREREQ doc text must agree with both.

## Acceptance criteria contributions
- A diff grep for `env::var|env!\(|std::env` over changed Rust files finds no read outside the §Secret Management list, or the wrap carries a flagged security-plan amendment for each new one (per security-plan §Secret Management).
- The emitted JSON-line log (one boot → run → logs round trip on the stand checks) contains no unredacted content-named field (`url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error`, `panic.payload`) and no captured test stdout or panic payload (per security-plan §Logging & Monitoring).
- If a dependency was added: `bash .github/scripts/ci-leg.sh audit` passes, `Cargo.lock` is committed and builds pass `--locked` (per security-plan §Dependency Security).
- `scripts/agent-run.{sh,ps1}` and any harness code open no socket, port or listener: a grep for `TcpListener|bind\(|listen\(|nc |socat` over the new files is empty (per security-plan §API Security).
