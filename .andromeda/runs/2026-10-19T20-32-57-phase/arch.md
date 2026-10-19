# arch extract

## Relevance
partial — a test-placement refactor inside the escher-driver and seven_guis crates; arch governs where unit-test modules and shared integration-test helpers live, and which gates read file paths, not any surface or contract.

## Constraints
- The split follows the established out-of-file unit-test seam: a `#[cfg(test)]` child declared through `#[path = "…_tests.rs"]` at its parent, moved out whole with its test paths unchanged, so the child keeps reaching its parent's private items with no `pub`, constructor or `cfg(test)` shim added (per architecture §Conventions → Tests).
- `cli.rs`'s and `recording.rs`'s test modules keep their module names and paths (`cli::tests`, `recording::tests`, `recording::off_unix_tests`), as `mcp::tests` and `schema::tests` did at 2026-10-16-file-splits (per architecture §Conventions → Tests).
- The recording module is unix only, with `Unsupported` elsewhere; the off-unix test module's own `cfg` gate travels with it unchanged (per architecture §Existing Scopes → escher-driver row).
- seven_guis' shared helpers live in `examples/seven_guis/tests/common/mod.rs`, which is no test target. The crate keeps its eight integration-test targets, `host_log` and `mcp_log` among them, each still a target file of its own (per architecture §Existing Scopes → seven_guis row).
- A shared module in a test-directory subdirectory is no test target, so each test-target file still covers one behaviour. The log-check fragment moves into the existing module and adds no new target (per architecture §Conventions → Tests).
- The state directories the log checks make stay under `CARGO_TARGET_TMPDIR` (`target/tmp/`), with `host_log`'s `hl-trace` unchanged. No new env var, listener or path is added; `CARGO_TARGET_TMPDIR` and `CARGO_BIN_EXE_escher-session` remain the only test-time reads (per architecture §Occupied Resources → Filesystem).
- The coverage leg gates escher-driver at a 97 % line floor through `.github/scripts/coverage_floor.py`. The mutation tier runs on the dev host under the `mutants` profile of `.config/nextest.toml`, whose one priority override names the direct tests. Moving test bodies into new files must not move a floor or drop a named test. Whether the floor script or that override reads file paths or test paths that this split changes is research's question (per architecture §Stack and Technologies → Code quality row).

## Patterns to follow
- The 2026-10-16-file-splits precedent: `mcp.rs` → `mcp_tests.rs` and `schema.rs` → `schema_tests.rs` in `packages/escher-driver/src/`, each declared at the end of its parent (per architecture §Conventions → Tests; architecture §Existing Scopes → escher-driver row).
- `dioxus_document.rs` → `dioxus_document_tests.rs` (`bridge_tests`), declared at the parent, not in `lib.rs`, so it stays a child of the module under test (per architecture §Conventions → Tests; architecture §Existing Scopes → dioxus-native-dom row).
- Helpers shared by spawning checks are stated once in `tests/common/mod.rs` and read by `mod common;`. Examples: the kill-and-reap guard, the stream drain, the bounded client run, the started-session guard and `Speaker` (per architecture §Occupied Resources → Process-wide state and threads).
- Each integration-test file opens with a `//!` doc that names its one behaviour; module files open with `//!` docs (per architecture §Conventions → Tests; architecture §Conventions → Documentation).

## Anti-patterns to avoid
- Widening visibility (`pub`, `pub(crate)` beyond what exists, or a `cfg(test)` shim) so that a moved test can reach the parent's internals. The `#[path]` child seam exists to avoid this (per architecture §Conventions → Tests; architecture §Conventions → Visibility and naming).
- Making the shared log fragment a new test target, or moving it into a new top-level file under `examples/seven_guis/tests/`, which would change the crate's eight-target set (per architecture §Existing Scopes → seven_guis row).
- Calling `std::env::var` or adding a new env read inside the shared fragment (per architecture §Occupied Resources → Filesystem).

## Contract bindings
- arch ↔ tests: the `stand-repeat` leg runs `cargo test -p seven_guis --locked` over eight named targets, `host_log` and `mcp_log` among them. Their target names must stay unchanged (per architecture §Standard Contracts → CI contracts).
- arch ↔ obs: `host_log` and `mcp_log` are the measured witnesses of the sink's closed-span lines. The recorded figures are 3 `command` lines for 3 calls at `trace`, 5 `tool_call` and 3 `command` lines at `info`, and 0 id, name or typed-sentinel needles. The shared fragment must keep those assertions intact (per architecture §Occupied Resources → Process-wide state and threads).
- arch ↔ docs: arch line citations into `host_log.rs`, `mcp_log.rs`, `common/mod.rs`, `cli.rs` and `recording.rs` will move. Re-pointing them is the wrap's cascade (per architecture §Occupied Resources → Process-wide state and threads; architecture §Existing Scopes → escher-driver row).

## Acceptance criteria contributions
- `cli.rs` and `recording.rs` each declare their moved test modules as `#[cfg(test)] #[path = "{name}_tests.rs"]` children, with the module names `tests` and `off_unix_tests` unchanged. The sibling files sit under `packages/escher-driver/src/`, and no item's visibility is widened (per architecture §Conventions → Tests).
- `examples/seven_guis/tests/` still holds exactly the eight test-target files. The fragment shared by `host_log` and `mcp_log` lives only in `tests/common/mod.rs` (per architecture §Existing Scopes → seven_guis row).
- `bash .github/scripts/ci-leg.sh coverage` still reads escher-driver at or above its 97 % floor. The `mutants` profile's override still names tests that exist (per architecture §Stack and Technologies → Code quality row).
- No new env var, path or listener appears in the diff (per architecture §Occupied Resources → Filesystem).
