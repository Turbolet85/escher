# arch extract

## Relevance
partial — a tests-only chunk: arch governs where the new tests sit, which shipped contracts they must leave unchanged, and what they may not add; it says nothing about mutation testing itself.

## Constraints
- New unit tests follow the plan's test placement: a `#[cfg(test)] mod tests` at the end of the source file, or an out-of-line `#[cfg(test)] #[path = …]` child declared at its parent. escher-driver's MCP tests already sit in `mcp_tests.rs` as `mcp::tests`. A new out-of-line module raises the plan's count of five such modules, which the wrap amends (per architecture §Conventions → Tests).
- `mcp`'s three private children (`framing`, `listing`, `result`) are widened to `pub(super)` and no further. A framing test must reach `read_request` from inside `mcp`'s module tree, with no wider visibility and no `cfg(test)` shim (per architecture §Existing Scopes → escher-driver).
- The MCP request-line bound is part of a ratified contract. That contract covers `MAX_MCP_REQUEST_BYTES` (private, 65,536), a longer line read to its terminator, dropped and answered `-32700`, and a notification running nothing. Tests may pin this behaviour but must not change the bound, its visibility or the protocol errors (per architecture §Standard Contracts → Driver session (escher-driver)).
- The Timer's time contract stays as written. `TimerTicks::deliver(n)` is read by `Timer` through `try_use_context`. With no context, it uses the 100 ms `futures_timer::Delay` loop. With a context, it takes one 0.1 s step per delivered tick, capped at the duration. `timer_step` delivers whole ticks and drops the remainder. A test that catches the `TimerTicks::next` mutants checks this contract and does not change it (per architecture §Standard Contracts → Headless stand (seven_guis, native only)).
- `mutation_writer.rs` is recorded as byte-identical to upstream's at `434e0661`. Adding a test module inside it, or changing it, breaks that identity. Whether the identity still holds at HEAD is research's question. A shipped change there is P4's fork (per architecture §Standard Contracts → Dioxus DOM bridge).
- Add no dependency, workspace crate or `[workspace.dependencies]` entry, for example a timeout crate for a watchdog. Dependencies are declared once at the workspace and taken as `{ workspace = true }` (per architecture §Conventions → Manifests).
- Add no env var and no listener. The environment-variable list is closed (`RUST_LOG`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, and `TMPDIR` in test code only). A test-only resource is still registered as test-only here before it lands (per architecture §Occupied Resources → Environment variables).

## Patterns to follow
- Bounded waits in tests. Every wait in a parent is bounded, and a guard kills and reaps any child a failing check still holds. `Speaker`, for example, reads replies under a 30 s bound and kills and reaps in `Drop`. This is the plan's model for a watchdog inside a test that turns a hang into a fast failure (per architecture §Occupied Resources → Process-wide state and threads).
- Test-only children in a file of their own. `bridge_tests` and `mcp::tests` reach their parent's private items with no constructor, `pub` or `cfg(test)` shim added. A `create_text_node` test can follow this pattern beside `dioxus_document.rs` and leave `mutation_writer.rs` untouched. Which existing test module reaches that site is research's question (per architecture §Conventions → Tests).
- Deterministic Timer driving. The stand's `boot_timer` returns a `TimerTicks` handle, and `timer_step` moves time by whole ticks with no sleep. A Timer test drives ticks and asserts the countdown, without waiting on real time (per architecture §Standard Contracts → Headless stand (seven_guis, native only)).
- Integration-test shape, if a blitz-tests target is added. One behaviour per file, opening with a `//!` doc. The test first asserts that the fixture produces the condition under test. Scenarios run `for incremental in [false, true]`, and stand-wide helpers come from `mod common;` or `mod session_common;` (per architecture §Conventions → Tests).

## Anti-patterns to avoid
- Widening an item's visibility (`pub`, `pub(crate)` beyond the `pub(super)` the mcp children carry) or adding a re-export so that a test can reach it. escher-driver's 45 re-exported names under `#![deny(missing_docs)]` are a contract (per architecture §Standard Contracts → Driver session (escher-driver)).
- Treating cargo-mutants as a CI tool. The CI-installed tools are cargo-deny and cargo-llvm-cov. Adding a mutation leg or a third installed tool is a §Stack amendment and is outside this chunk (per architecture §Stack and Technologies → CI/CD).
- Leaving a mutation run's output, such as cargo-mutants' default `mutants.out/` in the working directory, in the tracked tree. Written paths are enumerated (`target/ci-logs/`, `target/tmp/`, the run dirs). Where the audit's `mutate.py` wrote its output is research's question (per architecture §Occupied Resources → Filesystem).

## Contract bindings
- arch ↔ tests: a new blitz-tests target changes the target pin (`.github/scripts/test_blitz_tests_targets.py`) and the per-file counts. The arch §Existing Scopes rows for blitz-tests, dioxus-native-dom (`bridge_tests`, "six unit tests") and escher-driver carry test counts and module lists that the wrap's reconcile re-reads (per architecture §Existing Scopes).
- arch ↔ tests (CI gates): new tests run under the same legs. Those are fmt, clippy `-D warnings` (including the blitz-dom `--all-targets` line), rustdoc `-D warnings`, the per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %) and, for any stand check, the `stand-repeat` leg's three-round zero-failure budget (per architecture §Stack and Technologies → Code quality).
- arch ↔ security: the MCP framing bound and the ratified stdio crossing are pinned, not changed. A test that feeds `read_request` oversized or empty lines stays inside that crossing (per architecture §Standard Contracts → Driver session (escher-driver)).

## Acceptance criteria contributions
- The diff touches no shipped file under `packages/escher-driver/src/mcp/`, `packages/dioxus-native-dom/src/mutation_writer.rs` or `examples/seven_guis/src/tasks/timer.rs` other than an added `#[cfg(test)]` module, unless P4's fork rules otherwise. Recheck `mutation_writer.rs` against upstream after the chunk (per architecture §Standard Contracts → Dioxus DOM bridge).
- No new `pub` item, re-export or widened visibility. The mcp children stay `pub(super)`, and escher-driver's re-export count stays 45 (per architecture §Existing Scopes → escher-driver).
- `Cargo.toml` and `Cargo.lock` are unchanged: no dependency, crate or env read added (per architecture §Conventions → Manifests).
- `bash .github/scripts/ci-leg.sh fast` and `bash .github/scripts/ci-leg.sh doc` pass, and the coverage floors still hold (per architecture §Stack and Technologies → Code quality).
