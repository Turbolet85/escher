# tests extract

## Relevance
relevant: the chunk changes only test code. It moves escher-driver unit-test modules out of line and states the seven_guis log-check fragment once, so test layout, test counts, gating, coverage and the mutation tier are all this domain's.

## Constraints
- test-plan §2 Test Strategy → Directory pattern requires that an out-of-line unit-test module stays a `cfg(test)` module held in a file of its own through `#[path]`. The five existing ones, `mcp::tests` and `schema::tests` among them, are this chunk's precedent. Test paths stay unchanged, so `cli::tests::…`, `recording::tests::…` and `recording::off_unix_tests::…` must still resolve under the same names after the move.
- test-plan §4 Unit Test Strategy → Conventions (Test file location) says a module moved out at 2026-10-16-file-splits went "out whole with their test paths unchanged". Every test body moves byte for byte, with no renamed function and no added or dropped assertion.
- test-plan §1 Test Scope Summary → escher-driver states the crate's tally: 92 unit tests in fifteen files, cli.rs 5 and recording.rs 9, plus three off-unix tests that only the windows job runs. The tallies must read the same after the split. Only the inline-versus-`#[path]` split moves (72 inline and 20 in two `#[path]` files becomes 58 inline and 34 in four), along with the line-range citations `packages/escher-driver/src/cli.rs:476-1032` and `packages/escher-driver/src/recording.rs:469-981`. Wrap's cascade owns those statements, not this chunk's code.
- test-plan §3 `session-checks` (`.andromeda/registries/contracts/test-plan/session-lifecycle.md`) requires gating to stay as it is:
  - `host_log` is unix only.
  - `mcp_log` is not unix-gated, and the windows job runs it.
  - `mcp_log`'s recording items (`recorded_needles`, `the_check`) are `cfg(unix)`-gated item by item.
  - recording.rs's `off_unix_tests` keeps its own off-unix gate, and only the windows job runs it.

  A fragment moved into the shared module must keep each caller's gate, so `mcp_log` still compiles and runs off unix.
- test-plan §1 Test Scope Summary (apps line) and §2 Test Strategy → Directory pattern describe `examples/seven_guis/tests/common/mod.rs` as the one shared module of the seven_guis test crate. All eight targets read it through `mod common;`, and it is no target and holds no test. The new fragment therefore compiles into all eight targets. Whether the module already tolerates a helper that six of the eight leave unused under clippy `-D warnings` is research's question.
- test-plan §10 Quality Gates → Coverage floors sets escher-driver's floor at 97 %. `coverage_floor.py` reads it by summing `LF`/`LH` over the `SF` records under `packages/escher-driver`. Moving test bodies into sibling files under the same crate directory must not move that reading. Two things are research's question: whether llvm-cov attributes `cfg(test)` module lines to the source file's record, and so whether the per-crate sum is unchanged.
- test-plan §10 Quality Gates → Mutation tier states that the `mutants` profile of the root `.config/nextest.toml` carries a `priority = 100` override naming the direct tests. Unchanged test paths keep that override valid. Whether it names any `cli::` or `recording::` test, and whether any mutation file set reads `cli.rs` or `recording.rs` by path, is research's question.

## Patterns to follow
- The seam `#[cfg(test)] #[path = "{name}_tests.rs"] mod tests;` sits at the end of the parent source file, with the body in a sibling file under `packages/escher-driver/src/`. It is used today by `packages/escher-driver/src/mcp.rs:291-293` → `mcp_tests.rs` and `packages/escher-driver/src/schema.rs:482-484` → `schema_tests.rs` (per test-plan §4 Unit Test Strategy → Conventions).
- A shared test module is "the one statement" of what its checks share. It holds no test and no target, and each item is stated once. Items particular to one file stay in that file (per test-plan §3 Test Harness Contract → Crate-local test helpers, blitz-tests stand checks). The seven_guis shared module already holds the drain, the bounded run, the `Host`/`Started` guards and the `Speaker` on this pattern (per test-plan §1 Test Scope Summary, apps line).
- The log checks follow these rules (per test-plan §5 Integration Test Strategy → Session host ↔ what it writes):
  - needles are read from the stand booted in the test's own process;
  - both streams are drained by a thread each, starting from the spawn;
  - a failure prints a kind and a count only.

  The shared fragment must keep all of this, and it must still produce one `command` span line per call and one `tool_call` line per tool call.
- A check that captures a sink-installing child counts lines by span and target, never in total, so the same check stays green under both the `-p seven_guis` build and the workspace build (per test-plan §2 Test Strategy → Process-lifecycle checks; §9 CI Integration → Engine features by runner).

## Anti-patterns to avoid
- Do not count a sink capture's lines in total, and do not take a green per-package run as the workspace leg's green. A fragment shared by `host_log` and `mcp_log` must keep its per-target counting (per test-plan §9 CI Integration → Engine features by runner).
- Do not wait for a sink-installing child to exit before reading its pipes. The fragment must keep draining from the spawn on (per test-plan §2 Test Strategy → Process-lifecycle checks).
- Do not let an assertion message or a failure print carry a needle, id, name or typed text. Failures print a kind and a count only (per test-plan §5 Integration Test Strategy → Session host ↔ what it writes).

## Contract bindings
- tests ↔ obs: `host_log` and `mcp_log` are the standing witnesses of the stand log format and its zero-needle reading at `trace` under both builds (test-plan §3 Test Harness Contract → Stand log format; obs-plan §3, §8). Deduplicating them must leave every needle class they read. The classes are id, name, typed sentinel, a shot's first 64 base64 characters, and the recording needles.
- tests ↔ CI: the `stand-repeat` leg names seven_guis' `host_log`, `mcp_log` and the other six targets by name and runs them three rounds with no retry (test-plan §9 CI Integration → Legs; §10 Flakiness budget). The `coverage` leg chains `coverage_floor.py` (test-plan §9 → Coverage). Target names and the crate directory must not move.
- tests ↔ security: a test name or assertion message holds no id, name, value or call-supplied argument. The moved `cli` and `recording` test bodies carry that rule over unchanged (`.claude/rules/security.md` → Surfaces).

## Acceptance criteria contributions
- `cargo test -p escher-driver --locked` reads 92 passed both before and after the split. Its list of test names (`-- --list`) is identical, `cli::tests::…`, `recording::tests::…` and `recording::off_unix_tests::…` included. The three off-unix tests stay compiled for the windows job (per test-plan §1 Test Scope Summary → escher-driver; §4 Unit Test Strategy → Conventions).
- Under both `cargo test -p seven_guis --locked --test host_log --test mcp_log` and `cargo test --workspace --locked --test host_log --test mcp_log`, each file reads 2 passed with the same test names. The shared fragment sits once in `examples/seven_guis/tests/common/mod.rs`, and neither file keeps a copy (per test-plan §5 Integration Test Strategy → Session host ↔ what it writes).
- `bash .github/scripts/ci-leg.sh coverage` ends `coverage floor: 3 of 3 crates at or above their floors`, with `packages/escher-driver` at or above 97 % (per test-plan §10 Quality Gates → Coverage floors).
- `bash .github/scripts/ci-leg.sh fast`, `bash .github/scripts/ci-leg.sh doc` and `bash .github/scripts/ci-leg.sh stand-repeat` are each green, the last reading `stand-repeat: 3 of 3 rounds green` (per test-plan §9 CI Integration → Legs; §10 Quality Gates → Flakiness budget).
