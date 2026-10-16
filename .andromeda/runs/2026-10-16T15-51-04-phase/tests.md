# tests extract

## Relevance
relevant — the chunk moves unit-test modules (blitz-dom `document.rs`) and re-homes inline test modules with their code (escher-driver `mcp.rs`, `schema.rs`); the test suite is both the thing moved and the proof of no behaviour change.

## Constraints
- Every test that exists before the split runs after it, under the same names' coverage: the plan records 18 `#[test]` functions in `document.rs` (`zoom_tests` through `font_reading_tests`, from `packages/blitz-dom/src/document.rs:2897`), and 73 inline escher-driver unit tests in thirteen files, `mcp.rs` 11 and `schema.rs` 6 (per test-plan §1 Test Scope Summary). Whether the current tree still reads those counts is research's question; the split must leave the per-crate totals unchanged.
- An out-of-line unit-test module is a `cfg(test)` child of its parent declared through `#[path]` when it needs private items, as `dioxus_document::bridge_tests` in `dioxus_document_tests.rs` is (per test-plan §4 Unit Test Strategy, Test file location). Whether the `document.rs` modules reach `BaseDocument` internals, and so must stay child modules rather than move to a blitz-tests target, is research's question.
- A new blitz-tests file needs no manifest edit: the pin `test_blitz_tests_targets.py` requires `autotests` left on and exactly one `[[test]]` table, `all` with `test = false` (per test-plan §4 Unit Test Strategy, blitz-tests targets). A new target must not add a `[[test]]` table.
- The `hover_state_tests` font precondition asserts through `assert_text_has_size` with the two-argument `qual_name!(…, html)` fixtures, and no font-metric test of `document.rs` skips (per test-plan §8 Mocking & Stubbing Discipline, Real dependencies kept). A move keeps those fixtures as they are.
- The span checks read the `command` and `tool_call` lines and their closed field sets: `host_log`, `mcp_log`, and in process `stand_mcp_tools` and `stand_act_spans` (per test-plan §3 Test Harness Contract, Stand log format). Splitting `mcp.rs` must leave every `tool_call` span site and its fields as they read.
- The coverage leg ends with `coverage_floor.py`, and its floors are per crate directory: `packages/escher-driver` 97 % (per test-plan §10 Quality Gates & Coverage Targets, Coverage floors). Moving files inside the crate does not change the floor, but a split must not drop a test that holds escher-driver's lines.
- The stand checks run under a zero-failure budget, three rounds with no retry (per test-plan §10 Quality Gates & Coverage Targets, Flakiness budget).

## Patterns to follow
- Out-of-line test module: `#[cfg(test)] #[path = "{file}_tests.rs"] mod {name};` at the end of the parent, so private items stay reachable (per test-plan §2 Test Strategy, Directory pattern; test-plan §4 Unit Test Strategy, Test file location).
- Inline `#[cfg(test)] mod tests` next to the code under test for each new escher-driver submodule, so `mcp.rs`'s and `schema.rs`'s tests go with the code they pin (per test-plan §4 Unit Test Strategy, Test file location; the escher-driver rows in §4 What unit tests cover).
- Gate commands: `bash .github/scripts/ci-leg.sh fast` as the local pre-push gate (per test-plan §9 CI Integration, Local pre-push gate), then `ci-leg.sh doc`, `ci-leg.sh coverage` and `ci-leg.sh stand-repeat` (per test-plan §9 CI Integration, Legs). Use `bash scripts/agent-run.sh boot` then `run stand` for the agent-driven read (per test-plan §3 Test Harness Contract, Agent-run contract).
- The span checks run under both builds: `cargo test -p seven_guis --locked --test host_log --test mcp_log` and `cargo test --workspace --locked --test host_log --test mcp_log` (per test-plan §1 Test Scope Summary, the seven_guis targets).

## Anti-patterns to avoid
- Do not let a moved test reach private items by adding a constructor, a `pub` or a `cfg(test)` shim to the code under test. The bridge-tests move added none (per test-plan §3 Test Harness Contract, Crate-local test helpers, dioxus-native-dom).
- Do not read a `-p` build's green as the workspace build's: the two builds compile blitz-dom with and without its `tracing` call sites (per test-plan §9 CI Integration, Engine features by runner).

## Contract bindings
- tests ↔ obs: the stand log format's closed-span lines (`command` under `escher_driver` with its eight fields; `tool_call` with `tool` and `ending`) bind to obs-plan §6 Log format and §4. The split's "spans unchanged" is checked by `host_log`, `mcp_log`, `stand_mcp_tools` and `stand_act_spans` (per test-plan §3 Test Harness Contract, Stand log format).
- tests ↔ CI: a new blitz-tests target meets the manifest pin in `.github/scripts/test_blitz_tests_targets.py`, run in the `ci-scripts` leg (per test-plan §4 Unit Test Strategy, blitz-tests targets).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` and `ci-leg.sh doc` exit 0. `cargo test -p blitz-dom --locked --lib` runs the same number of `document` tests as before the move, 18 at the plan's reading, from their new location. `cargo test -p escher-driver --locked` runs the same unit-test total, 73 at the plan's reading (per test-plan §1 Test Scope Summary).
- `cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture` exits 0 with `2 passed` and no `skipping` line (per test-plan §8 Mocking & Stubbing Discipline, Real dependencies kept).
- `ci-leg.sh coverage` prints `coverage floor: 3 of 3 crates at or above their floors`, with escher-driver at or above 97 %. `ci-leg.sh stand-repeat` reads 0 failures over its three rounds (per test-plan §10 Quality Gates & Coverage Targets).
- `host_log` and `mcp_log` pass under both the `-p seven_guis` build and the `--workspace` build, and `stand_mcp_tools` and `stand_act_spans` pass in both layout modes. Together these read the span field sets unchanged (per test-plan §3 Test Harness Contract, Stand log format).
