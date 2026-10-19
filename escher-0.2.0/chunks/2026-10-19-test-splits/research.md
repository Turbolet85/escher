# Codebase Research — 2026-10-19-test-splits

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8
- **Harness rules consulted:** `.claude/rules/testing.md` — 2 additions applied (2026-10-19: a moved `cfg(unix)` gate
  is unverified on linux until `cargo check --tests --locked --target x86_64-pc-windows-gnu -p {crate}` reads it;
  2026-10-10, extended 2026-10-18: search `tests/`, `examples/*/tests/` and the owning crate's `src/` before calling a
  check unaffected). No live leg: every gate is a `cargo` command or a CI leg script.
- **Platform issues consulted:** none — no runner-only bullet (CI#39319403188 on `f98a4dce1` was in progress at take-up)
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `packages/escher-driver/src/cli.rs` (440-500, 1028-1032) — production code ends at line 474 (`start`'s spawn); the
  inline `#[cfg(test)] mod tests {` opens at 476-477 and runs to the file's end at 1,032. Its `use` lines:
  `blitz_test_harness::Busy`, `super::*`, `crate::refusal::Fault`. 5 `#[test]` (`grep -c '#\[test\]'`).
- `packages/escher-driver/src/recording.rs` (440-500) — production code ends at 451 (`step_of`). It holds
  `#[cfg(all(test, not(unix)))] mod off_unix_tests` at 453-467 (1 `#[test]`, uses `super::*`, `Path`), then
  `#[cfg(all(test, unix))] mod tests` at 469-981 (9 `#[test]`; its `scratch` reads compile-time `CARGO_MANIFEST_DIR`
  into the workspace `target/tmp`). 10 `#[test]` in the file (`grep -c '#\[test\]'`).
- `packages/escher-driver/src/mcp.rs` (286-293) and `schema.rs` (481-484) — the precedent seam at the file's end:
  `#[cfg(test)]` / `#[path = "mcp_tests.rs"]` / `mod tests;`.
- `packages/escher-driver/src/mcp_tests.rs` (1-12) and `schema_tests.rs` (1-8) — a moved module's file opens with a
  `//!` doc that names what it drives and says it is a child module of its parent, then its `use` lines at top level,
  the body dedented one level.
- `examples/seven_guis/tests/host_log.rs` (full, 315 lines) and `examples/seven_guis/tests/mcp_log.rs` (full, 333
  lines) — the two log checks. Their shared fragments are listed under Patterns detected.
- `examples/seven_guis/tests/common/mod.rs` (1-13, imports) — `#![allow(dead_code)]` at line 8, so a helper that six
  of the eight targets leave unused does not warn under clippy `-D warnings`; its `//!` doc (1-6) lists what it holds;
  it already calls `escher_driver::stop` (357).
- `.config/nextest.toml` (full) — the `mutants` profile's one override names `tasks::timer::tests::…`,
  `mcp::tests::…` and `dioxus_document::bridge_tests::…`; no `cli::` or `recording::` test.
- `.github/scripts/coverage_floor.py` (40-60) — sums `LF`/`LH` per `SF` record whose repo-relative path lies under the
  crate directory (`relative.startswith(crate + "/")`), so moving test lines between files of
  `packages/escher-driver/src/` leaves the crate's sums equal. `ci-leg.sh:44-45` excludes only
  `keyboard_compat\.rs$` by filename.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T20-32-57-phase/tree-query-2026-10-19-test-splits.json`)
- **host_flow** — 2 refs, both in `examples/seven_guis/tests/host_log.rs` (lines 27, 33, editor lines) — a file-local
  helper; no caller outside the check.
- **mcp_flow** — 2 refs, both in `examples/seven_guis/tests/mcp_log.rs` (53, 62) — file-local.
- **recorded_needles** — 2 refs: `host_log.rs:306` and `mcp_log.rs:325`; the graph resolves both to the `host_log.rs`
  definition (two test crates' same-named fns, one SCIP symbol), so it reads as one helper already — the duplication
  this chunk removes. No cross-crate blast radius: every symbol the chunk moves is test-only and crate-local.

## Patterns detected
- **The `#[path]` test-child seam** (`packages/escher-driver/src/mcp.rs:291-293`): the module stays a child of its
  parent, so `super::*` and private items still resolve and the test paths (`mcp::tests::…`) are unchanged.
- **The six measured clone pairs** (jscpd 5.4.0, `--format rust`, default min-tokens, run over `host_log.rs`,
  `mcp_log.rs` and `common/mod.rs`: 6 clones · 95 duplicated lines, reproducing the audit's M1 row):
  1. `recorded_needles` — `host_log.rs:33-61` ↔ `mcp_log.rs:63-91` (29 L), identical body, `#[cfg(unix)]` in both;
  2. the needle read from the stand booted in process (boot, `element_ids` filtered to author keys holding `-`,
     `text_of`, the CRUD rows, the list's `aria-label`, `names`) — `host_log.rs:81-109` ↔ `mcp_log.rs:99-126` (29 L);
  3. the needle sanity asserts (≥ 15 ids, ≥ 3 rows) — `host_log.rs:109-115` ↔ `mcp_log.rs:126-132` (7 L);
  4. the stderr line count and the `service.name=seven_guis` stamp assert — `host_log.rs:255-266` ↔
     `mcp_log.rs:236-246` (12 L);
  5. and 6. the `found` counter and the needle-count read — `host_log.rs:278-293` ↔ `mcp_log.rs:299-315` (11 L, 7 L).
  What differs and stays per file: the key each check asserts present (`crud-surname` in `host_log`, `crud-name` in
  `mcp_log`), the sentinel text, the flow (socket host vs `Speaker`), the span counts, `mcp_log`'s `NOBODY` needle and
  `tool_call` field check, and the gates (`host_log` wholly `cfg(unix)`; `mcp_log`'s recording items alone).
- **Failure messages name a kind and a count** (`host_log.rs:247-248`, `mcp_log.rs:11-12`): no needle or captured
  line in any assertion message.

## Conventions to follow
- **A moved module's head**: a `//!` doc naming what the tests drive and that it is a child module of its parent
  (`packages/escher-driver/src/schema_tests.rs:1-4`).
- **The shared module's head lists its contents** (`examples/seven_guis/tests/common/mod.rs:1-6`): a new helper there
  is named in that doc.
- **Per-item `cfg(unix)` gates for recording** (`mcp_log.rs:56-59`, `:68`, `:323`): a recording helper moved into
  `common/mod.rs` keeps its `#[cfg(unix)]`, so `mcp_log` still compiles off unix.

## New files to create
- `packages/escher-driver/src/cli_tests.rs` — `cli::tests`, moved out whole
- `packages/escher-driver/src/recording_tests.rs` — `recording::tests`, moved out whole, still `cfg(all(test, unix))`
- `packages/escher-driver/src/recording_off_unix_tests.rs` — `recording::off_unix_tests`, moved out whole, still `cfg(all(test, not(unix)))`

## Files to modify
- `packages/escher-driver/src/cli.rs` — the inline module replaced by its `#[path]` declaration
- `packages/escher-driver/src/recording.rs` — the two inline modules replaced by their `#[path]` declarations
- `examples/seven_guis/tests/common/mod.rs` — the log checks' shared fragment stated once, and named in its `//!` doc
- `examples/seven_guis/tests/host_log.rs` — its copies replaced by calls into `common`
- `examples/seven_guis/tests/mcp_log.rs` — its copies replaced by calls into `common`

## Sweeps
- `cli\.rs|recording\.rs|_tests\.rs|host_log|mcp_log` over `.github/`, `scripts/`, `justfile`: 2 hits · 0 changed ·
  2 no-change (`test_coverage_floor.py:30` a synthetic lcov path `packages/driver/src/cli.rs`, not this crate;
  `ci-leg.sh:57` names the target `host_log`, whose name stays).
- `#\[test\]` count over `packages/escher-driver/src/*.rs` (`grep -c`): 95 attributes in 15 files, of which `cli.rs` 5
  and `recording.rs` 10 (9 unix + 1 off-unix) move; the unix-built count, the plan's 92, is re-read by
  `cargo test -p escher-driver --locked` at baseline.
- `cli::|recording::` over `.config/nextest.toml`: 0 hits — the mutation override names neither module.

## Open questions
- none
