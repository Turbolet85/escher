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
- **Unit:** Rust built-in harness — `#[cfg(test)] mod tests` at the end of the source file (one module sits in a file of its own: dioxus-native-dom's `bridge_tests`, a `#[path]` child of `dioxus_document`, so it reaches that module's private fields); `#[tokio::test]` for async workers.
- **Integration:** `tests/blitz-tests/tests/*.rs`, one file per behaviour, depending only on dev-dependencies; the stand checks read the tables and helpers they share from `tests/common/mod.rs` (`mod common;`) and the checks that hold a session — the session checks, `stand_settle` and the six driver-action checks `stand_act_*` — what they share from `tests/session_common/mod.rs` (`mod session_common;`: the held session, the driver's call builders `click` · `type_into` · `press` · `advance`, the outcome reader `act`); neither is a test target. A check that needs a session process starts and stops its own host (its test binary re-run on one `#[ignore]` child, or the package's binary through `CARGO_BIN_EXE_*`), bounds every wait, guards the host with kill-and-reap and is `cfg(unix)`-gated.
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
- 2026-10-06: Never drive a stand or harness check with a deleting or other platform-bound key through the harness's `press` — the editor's `Key::Backspace` arm is compiled out on macOS (the delete arrives there as an Apple standard key binding, which `press` does not send; `Harness::apple_keybinding` does, and the driver's `press backspace` adds it on macOS), so a bare press does nothing on the macOS CI leg while the Linux host reads green; prove a state change with clicks and typed characters — the one deleting-key check is `stand_act_keys`, through the driver — and read the `#[cfg]` lines above a `match` arm before trusting a grep hit of it on every platform. [corrected 2026-10-07: the harness synthesizes the binding since `apple_keybinding`, and the driver's backspace deletes on the macOS CI leg]
- 2026-10-07: In a hand-built blitz-dom unit test create elements with `qual_name!("div", html)` — the one-argument `qual_name!("div")` gives an element in no namespace, which the default stylesheet's type selectors do not match, so `body` and `div` lay out inline and a fixture that needs block layout (an anonymous block, a box with a height) builds none without failing; assert the layout condition the test rests on before the behaviour.
