# tests extract

## Relevance
relevant — the chunk is test infrastructure: blitz-tests' shared modules, two file-private held `NetProvider`s and the agent-run contract.

## Constraints
- A blitz-tests shared module holds no test and is no test target of its own; a check reads it by declaring `mod common;` (or `mod session_common;`) after its `use` block, and items one check alone reads (each file's `SECRET`, `stand_element_ids`' own `boot`, `component`, `ceiling`) stay per-file — a new fixture statement, in an existing module or a third one, takes the same shape, and the plan's reader lists (21 `common` readers, 25 `session_common` readers) move with it (per test-plan §3 Crate-local test helpers).
- Every `.rs` file directly under `tests/blitz-tests/tests/` is its own auto-discovered target and `tests/all.rs` stays the one unbuilt `[[test]]` (`test = false`), so a shared module lives as `tests/{name}/mod.rs`, never as a top-level `.rs`, and the target tally (138 `.rs` files, 137 targets) must hold (per test-plan §2 Directory pattern).
- The held network stand-in replaces `stand_settle`'s `ManualNetProvider` (a head stylesheet then its `@import` answered on `.test` URLs, no real fetch) with no change to what that check asserts (per test-plan §8 Hand-written fakes and stubs, Network row).
- The same stand-in replaces `font_face_sniffed_source`'s `HeldNetProvider` with its readings intact: offline, the `data:` source requested exactly once, a `wOF2` body registered, an unsniffable body registering nothing, the file installing no subscriber and printing nothing, both layout modes (per test-plan §1 blitz-dom covered behaviours).
- The harness wraps a supplied `net_provider` whose `is_noop()` is false in the load counter `settle` reads, while the stand's `options(incremental)` carries `net_provider: None` — so the shared stand-in's `is_noop` answer decides whether `stand_settle` still reads `NotSettled { busy: Loads }`; whether each copy overrides `is_noop` today is research's question (per test-plan §7 Builders and options).
- `run {selection}` accepts only `stand` (one `--test` per `stand_*.rs`), `all`, or a `^[a-z0-9_]+$` name with its file present, anything else exit 2 — `font_face_sniffed_source` carries no `stand_` prefix, so `run stand` does not take it; any new selection is a contract change pinned by `.github/scripts/test_agent_run.py` (per test-plan §3 Agent-run contract).
- A check that records is `cfg(unix)`-gated item by item, so a shared fixture-hold helper `stand_session_recording` reads (unix only) beside checks the windows job runs must compile clean on both (per test-plan §3 `session-checks`).

## Patterns to follow
- `session_common::hold(task, incremental)` — a `Session` booted through the stand, the timer's carrying `stand::timer_step` — is the shape a shared fixture hold mirrors, booted with `seven_guis::stand::options(incremental)` (per test-plan §3 Crate-local test helpers; per test-plan §7 Builders and options).
- Earlier moves into `common` were stated as "moved in from {file}" with the reader list re-counted (`tab_order` from `stand_accessibility_ids`, `focused` from `stand_snapshot_state`) — each moved fixture or helper records its origin and its readers the same way (per test-plan §3 Crate-local test helpers).
- blitz-test-harness's settle unit tests already hold a `Held` `NetProvider` that keeps each handler until the test takes it — the held-request shape to state once in blitz-tests (it is crate-private, not reachable from blitz-tests) (per test-plan §8 Hand-written fakes and stubs, settle unit-tests row).
- Headless-stand fixtures assert unconditionally under the bundled DejaVu Sans with system fonts off — no skip path in a shared fixture boot (per test-plan §2 Font-dependent tests).

## Anti-patterns to avoid
- An empty run is never a pass: a selection that matches no file exits 1, so a fixture selection naming files that moved or were renamed must not read green on zero tests (per test-plan §3 Agent-run contract).
- A green `cargo test -p blitz-tests` (what `agent-run.sh run` executes) is not the workspace leg's green — the two builds resolve different engine features (`tracing`, `writing-mode`), so a moved check is read under `ci-leg.sh fast` too (per test-plan §9 Engine features by runner).
- No retry, `--ignored` or env var to make a moved check pass the repeat leg (per test-plan §9 Legs).

## Contract bindings
- tests §3 Agent-run contract ↔ obs-plan §3: agent-run's events are harness metadata, not an escher-telemetry sink — a new selection changes `run.start`'s `files` and is pinned by `.github/scripts/test_agent_run.py`.
- tests §2 Directory pattern ↔ CI scripts: the blitz-tests target pin `.github/scripts/test_blitz_tests_targets.py` (`BlitzTestsTargetsTest`) holds `tests/all.rs` unbuilt and the auto-discovered target set.
- tests ↔ security-plan §Input Validation: a moved password fixture and its checks keep no id, name or typed text in a test name or assertion message.
- tests ↔ arch §Occupied Resources / security-plan environment values: the held stand-in opens no listener and reads no env var — a test-only one would need registration before it lands; whether the code opens nothing is P3's question.

## Acceptance criteria contributions
- `bash scripts/agent-run.sh boot` then `run stand` ends `passed` with the same `ok` stand-event count as before the chunk, and `run font_face_sniffed_source` ends `passed` with its 2 tests (per test-plan §3 Agent-run contract).
- `tests/blitz-tests/tests/` still holds 138 `.rs` files and 137 test targets with `tests/all.rs` unbuilt, every new shared module a `{dir}/mod.rs` that is no target (per test-plan §2 Directory pattern).
- `bash .github/scripts/ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green` over the 44 `stand_*.rs` files (per test-plan §10 Flakiness budget).
- `bash .github/scripts/ci-leg.sh fast` is green, its workspace test tally equal to before the chunk (no test lost) (per test-plan §9 Local pre-push gate).
