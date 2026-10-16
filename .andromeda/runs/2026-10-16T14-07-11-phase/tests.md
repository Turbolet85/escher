# tests extract

## Relevance
partial — part A (blitz-dom handler splits, `parse_float` steps) is a behaviour-preserving refactor that test-plan's existing checks must hold green; part B (README prose) has no test-plan surface.

## Constraints
- The pinning checks for the refactored code are named in test-plan §1 Coverage scope (tests/blitz-tests): `range_input_interaction` (8 tests, each in both layout modes, an engine check on parsed pages with no `stand_` prefix, so it is run by the workspace `test` leg and per package, not by `run stand`) and `stand_keyboard` (14 tests, each in both layout modes, picked up by `run stand`). Both must stay green with their tables, row counts and assertions unchanged; whether a split needs any test edit at all is research's question, and the plan gives no licence to change a stated row (per test-plan §1 Coverage scope).
- `parse_float`'s steps are already pinned by the inline table tests in `packages/blitz-dom/src/node/range.rs` (7 tests: the HTML floating-point parser at 20 rows, plus the bounds, seed, move, degenerate-attribute, decimal-step and re-seed tables); the split must leave that module and its rows passing unchanged (per test-plan §1 Coverage scope, blitz-dom (node)).
- The same check can read differently under the two runners: `cargo test --workspace --locked` (the `test` leg) resolves blitz-dom with `tracing` and `writing-mode` on, `cargo test -p blitz-tests --locked` leaves them off, and a green per-package run is not the workspace leg's green — the touched handlers must be read green under both builds (per test-plan §9 Engine features by runner).
- The clippy leg runs a second, upstream blitz-dom line after the workspace line — `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — so each new named function must lint clean (no dead code, no unused import) under that feature set as well as the workspace's (per test-plan §9 Legs).
- `stand_keyboard` also runs in the `a11y` leg (six files) and under the `stand-repeat` leg's zero-failure budget, three rounds with no retry over every `stand_*.rs` (per test-plan §9 Legs; per test-plan §10 Flakiness budget).
- Incremental and non-incremental layout must stay identical: the differential oracle compares both modes after each step, and the range and keyboard checks run each test `for incremental in [false, true]` (per test-plan §2 Differential oracle).
- No coverage floor binds blitz-dom — the `coverage` leg's floors are escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 % — so the chunk owes no coverage threshold, only the existing checks (per test-plan §10 Coverage floors).

## Patterns to follow
- Unit tests sit inline in `#[cfg(test)] mod tests` next to the code under test. If a step of `parse_float` or an extracted arm earns its own test, it goes in the same file's module as a stated table, the way `node/range.rs`'s tests are written (per test-plan §4 Conventions; per test-plan §1 Coverage scope, blitz-dom (node)).
- Engine behaviour checks go in `tests/blitz-tests/tests/`, one auto-discovered target per behaviour file with no manifest line, using inline HTML fixtures (per test-plan §2 Directory pattern; per test-plan §7 Seed strategies).
- Agent-driven runs go through `bash scripts/agent-run.sh boot`, then `run stand` (which picks up `stand_keyboard`) and `run range_input_interaction` (one file by name), with `status` · `logs` · `cleanup` (per test-plan §3 Agent-run contract).
- The local pre-push gate is `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red), with `ci-leg.sh doc` beside it (per test-plan §9 Local pre-push gate).

## Anti-patterns to avoid
- Do not take a green per-package run (`cargo test -p blitz-tests`, `agent-run.sh run`) as the workspace leg's green, and do not lint one crate with a bare `cargo clippy -p blitz-dom -- -D warnings` in place of the `clippy` leg (per test-plan §9 Engine features by runner; per test-plan §9 Legs).
- Do not change `range_input_interaction`'s, `stand_keyboard`'s or `node/range.rs`'s stated rows, event counts or expected values to make a split pass. A row that has to move means behaviour changed, and that is the founder's fork, not this chunk's (per test-plan §1 Coverage scope).

## Contract bindings
- tests ↔ a11y: `stand_keyboard` is one of the `a11y` leg's six files, so the keyboard-activation split runs through the a11y CI gate (per test-plan §9 Legs).
- tests ↔ arch: the `incremental_oracle` invariant (both layout modes identical) is asserted in the test surface by the differential oracle and the both-modes loops of the pinning checks (per test-plan §2 Differential oracle).
- tests ↔ obs: blitz-dom's `tracing` call sites compile in under the workspace build only. Whether any extracted arm carries a `#[cfg(feature = "tracing")]` site is research's question; if one does, it must compile under both runners (per test-plan §9 Engine features by runner).

## Acceptance criteria contributions
- `cargo test -p blitz-tests --locked --test range_input_interaction --test stand_keyboard` and `cargo test -p blitz-dom --locked` (the `node/range.rs` inline tables) pass with no test file edited, and the same tests pass in the workspace `test` leg (per test-plan §1 Coverage scope; per test-plan §9 Engine features by runner).
- `bash .github/scripts/ci-leg.sh fast` exits 0, including the clippy leg's second blitz-dom `--no-default-features … file-input …` line, and `bash .github/scripts/ci-leg.sh doc` exits 0 (per test-plan §9 Legs).
- `bash .github/scripts/ci-leg.sh a11y` exits 0, `stand_keyboard` among its six files (per test-plan §9 Legs).
- The `stand-repeat` leg (three rounds, zero failures, no retry) stays green on the chunk's CI run (per test-plan §10 Flakiness budget).
