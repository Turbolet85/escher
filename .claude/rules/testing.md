---
paths:
  - "tests/**"
  - "**/tests/**"
  - "wpt/runner/src/test_runners/**"
  - ".github/scripts/test_*.py"
---

# Testing Rules

Path-scoped rules for test files. Source: `.andromeda/test-plan.md` §2 §4 §5 §7 §8 (test tier 0) and arch §Conventions.

## Framework
- **Unit:** Rust built-in harness — `#[cfg(test)] mod tests` at the end of the source file; `#[tokio::test]` for async workers.
- **Integration:** `tests/blitz-tests/tests/*.rs`, one file per behaviour, depending only on dev-dependencies.
- **Headless E2E:** `blitz_test_harness::Harness` over `HtmlDocument` / `DioxusDocument`; WPT via `cargo run -rp wpt css svg` (needs `WPT_DIR`).
- **CI scripts:** `python3 -m unittest discover -s .github/scripts` (the `ci-scripts` leg; PyYAML for the workflow tests), one `TestCase` per function.
- No mocking library, property-test or snapshot crate is used — stand-ins are hand-written fakes (`RecordingNetProvider`, `ManualNetProvider`, `RecordingShell`, `NoopEventHandler`, probe widgets).

## File and naming conventions
- Each integration file opens with a `//!` doc naming the behaviour and any guarded bug; regression tests cite the upstream issue/PR URL and the real page that exposed it.
- Descriptive snake_case test names; `#[track_caller]` on assertion helpers.
- Assert first that the fixture produces the condition under test, then the behaviour.

## Determinism
- Build documents with `HtmlDocument::from_html` + a `DocumentConfig` carrying a fixed viewport (800×600 or 400×300, scale 1.0, `ColorScheme::Light`) and `HtmlProvider`, then `resolve(0.0)`.
- Run pipeline scenarios in both layout modes: `for incremental in [false, true]`.
- Drive pure-restyle paths through `:hover`, not attribute mutation — a mutation inserts full damage and masks under-damaging bugs.
- Randomized stress uses a seeded LCG, never an unseeded RNG.
- The pre-existing font-dependent tests skip with `eprintln!` when no usable font exists (`system-fonts` is on when testing the workspace); this skip is a known gap, not a pattern to copy. Stand checks boot through `seven_guis::stand::boot(task, stand::options(incremental))` — pinned 800×600 · scale 1 · Light, the bundled DejaVu Sans with system fonts off, offline — and assert unconditionally: no font skip, no sleep (drive the timer through its `TimerTicks` handle), no `dispatch_recorded`.
- A harness check that needs a font or a layout mode sets `HarnessOptions.font_ctx` / `.incremental`.

## Fixtures
- Inline HTML string constants or `format!`-built pages; image loads injected as `ImageData` with `Status::Ok`; test URLs use the `.test` TLD or `example.com`.
- Blitz-dom unit tests build DOM by hand through `DocumentMutator` — blitz-html would be a circular dev-dependency.

## Running tests
- **One file:** `cargo test -p blitz-tests --test {name}`
- **One crate:** `cargo test -p {crate}`
- **Agent-driven (stand checks · blitz-tests):** `bash scripts/agent-run.sh boot`, then `run stand` · `run all` · `run {name}` — JSON-line results, exit `0`/`1`/`2`/`3` (test-plan §3; `.claude/rules/verification-harness.md`)
- **Cold-agent pipe (one live model session — operator host only, never in CI):** `bash scripts/cold-agent.sh run counter` → `target/cold-agent/verdict.json`; its contract tests run in the `ci-scripts` leg under a `claude` shim (`.github/scripts/test_cold_agent.py`)
- **Whole workspace (CI leg):** `cargo test --workspace --locked` — or `bash .github/scripts/ci-leg.sh test`, exactly as CI runs it
- **Before a push:** `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → CI scripts)
- **Benchmarks (ignored):** `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture`

## Not yet measured
- Coverage is measured with no threshold (`bash .github/scripts/ci-leg.sh coverage`, cargo-llvm-cov); a coverage floor, quality gates and the flakiness budget (§10) are owned by the working route's "Quality gates" chunk.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run._
- 2026-10-06: In a `.github/scripts` unittest file, never give a helper method a `test_` prefix — `unittest` collects it as a test case; name helpers without it and check the run's `Ran N tests` against the cases written.
