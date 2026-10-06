# Codebase Research — 2026-10-06-stand-test-contract

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition applied: a boot smoke bounded by `timeout N` exits 124 and `gate.py` reads 124/137 as its own bound, so no gate entry may rely on a `timeout` exit. `.claude/rules/testing.md` was also read in full; it has 0 Session Additions.
- **Platform issues consulted:** none. The chunk has no runner-only bullet, and no entry reads CI outside the operator leg.
- **External inputs:** none. Every fact this chunk turns on lives in this repository. The setup skill's template and re-run rule are a tool's behaviour, recorded under Patterns detected.

## Files inspected
- `packages/blitz-dom/src/node/element.rs` (440-455, 620-640) — `flush_is_focussable` at :628. It reads `disabled` with `attr_parsed(...)` and `unwrap_or(false)` (:629), a parsed bool. The element-state block at :446-451 inserts `ElementState::DISABLED` on `has_attr("disabled")` (:449), which is presence.
- `packages/blitz-dom/src/events/pointer.rs` (grep `disabled`) — :330 and :635 test `el.attr("disabled").is_some()`, and :457 maps `has_attr("disabled")` to `ClickTarget::Disabled`. These are presence checks.
- `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` (full) — `//!` lines 1-6. The single test asserts only the `disabled` attribute's absence or presence and `#target:disabled` matching, across two toggles. **It asserts no focusability** (no `focused`, no Tab press).
- `.github/scripts/ci-leg.sh` (full, 64 lines) — the shape for a verb-dispatch runner (details under Patterns).
- `.github/scripts/test_ci_workflows.py` (204-264) — `LegScriptTest`: a `cargo` shim on PATH in a temp dir runs the leg script and asserts the exit, the log contents and exit 2 on an unknown leg.
- `/home/turbolet/.claude/skills/andromeda-setup-project/references/scripts-templates/agent-run-bash.md` (full) and `andromeda-setup-project/SKILL.md` (214-234) — the generated harness is a daemon skeleton (boot starts a process and polls a status endpoint). Phase 4's re-run rule is **only-if-missing**: an existing `scripts/agent-run.*` is preserved, never overwritten, and a differing fresh render is backed up and noted.
- `examples/seven_guis/src/stand.rs` (grep) — 0 hits for `escher_telemetry|env::var|println` (re-derived: `grep -c` over the file). The stand installs no subscriber.
- `.github/workflows/ci.yml` (296-345) — `matrix_test` runs `cargo test --all --tests` on `windows-latest` and `macos-latest` directly, not through `ci-leg.sh`.
- `.andromeda/architecture.md` §Occupied Resources (Filesystem :147, Environment variables :151) — registered harness writes are `target/ci-logs/` and `target/coverage/`. Registered env reads are `RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML` and the CI script's `GITHUB_*`/`PR_NUMBER`/`RUN_URL`.
- `.gitignore` — `/target` (:1) and `target/` (:22). Anything under `target/` is ignored.
- `tests/blitz-tests/Cargo.toml` — `seven_guis = { workspace = true }` (:22). The stand checks reach the stand through it.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **No query run — no Rust symbol is created, changed or called anew.** The only Rust edit is a `//!` doc comment. The new surface is bash, PowerShell and Python, and the graph indexes only the `rust` plane. derived-without-graph for those files.
- **`seven_guis::stand::{boot, boot_timer, options}`** — unchanged and not called by the new scripts. The scripts drive cargo, which runs the existing `stand_*.rs` checks.

## Patterns detected
- **Verb-dispatch runner** (`.github/scripts/ci-leg.sh:8-64`) has these parts:
  - a `LEGS=(…)` array;
  - `usage()` prints the list to stderr and exits 2 on a missing or unknown verb;
  - one `case` per verb, with `set -euo pipefail`;
  - merged output `2>&1 | tee target/ci-logs/{leg}.log`, truncated by the redirect at each start;
  - the exit is the wrapped command's status. `pipefail` makes the `tee` pipe report it. Measured in `LegScriptTest.test_failing_leg_propagates_status_and_writes_its_log`.
- **libtest pretty output is the per-test channel on stable.** It was recorded at `evidence/p3-stand-run.log` (`cargo test -p blitz-tests --locked --test stand_boot --test stand_counter --test stand_crud --test stand_flight_booker --test stand_timer`, exit 0, 4 s warm). The line shapes are:
  - per binary: `     Running tests/{file}.rs (target/debug/deps/{file}-{hash})`;
  - then `running {N} tests` or `running 1 test`;
  - one line per test: `test {name} ... ok`;
  - then `test result: ok. {p} passed; {f} failed; {i} ignored; {m} measured; {x} filtered out; finished in {t}s`.
  The JSON format is not available: `cargo test -p blitz-tests --locked --test stand_counter -- --format json` exits 101 with `error: The "json" format is only accepted on the nightly compiler with -Z unstable-options` (measured this phase). The failing-line shape `test {name} ... FAILED` and its `failures:` block (captured stdout and panic message) are libtest's documented format. They are not recorded here because no test failed, so the plan's parser contract must be proved against a shim that emits them.
- **Shell-entrypoint tests through a cargo shim** (`.github/scripts/test_ci_workflows.py:204-260`). This proves exit and log contracts with no real cargo run. It runs in the `ci-scripts` leg, which discovers `.github/scripts/test_*.py` (`ci-leg.sh:22`).
- **Harness writes under `target/`.** `target/ci-logs/` and `target/coverage/` are the registered precedent (arch §Occupied Resources → Filesystem). `target/` is gitignored (`.gitignore:1,22`).

## Conventions to follow
- **`--locked` on every cargo call** (`ci-leg.sh:19-39`; architecture §Infrastructure Patterns → CI/CD).
- **One-file selection recipe**: `cargo test -p blitz-tests --test {name}` (`.claude/rules/testing.md` §Running tests). Several `--test` flags in one call are accepted (`ci-leg.sh:34`, the a11y leg).
- **Integration-test `//!` doc names the behaviour and the guarded bug** (`.claude/rules/testing.md` §File and naming conventions). The PREREQ rewrite keeps that form.
- **Python tests in `.github/scripts`**: one `TestCase` per function under test, stdlib `unittest`, a temp dir per case (`test_ci_workflows.py:204-260`; testing.md §Framework).

## New files to create
- `scripts/agent-run.sh` — the five verbs (boot, run, status, cleanup, logs), the JSON-line output, the state area under `target/`, and an `ensure_fresh_artifacts` hook (setup's U10 detector reads it).
- `scripts/agent-run.ps1` — the PowerShell counterpart. Its form is a P4 fork; every branch except dropping it creates this file.
- `.github/scripts/test_agent_run.py` — the shim-driven contract tests for `scripts/agent-run.sh`, discovered by the `ci-scripts` leg.

## Files to modify
- `tests/blitz-tests/tests/dioxus_falsy_disabled.rs` — the `//!` doc only (PREREQ).

## Open questions
- The PowerShell script cannot be run on this host. `pwsh` is absent: `command -v pwsh` printed nothing, while `jq`, `python3`, `timeout` and `flock` are present. Windows is tested in CI only through `matrix_test`'s direct cargo call. → blocks: plan-decision (the ps1's form, and how its contract is checked).
