# Scope — 2026-10-19-test-splits

**Working entry (escher-0.2.0/working-route.md:34, Epoch 4 — Visual lint and real-app readiness):**
Test splits — inline test modules out of escher-driver's `cli.rs` and `recording.rs`; `host_log`/`mcp_log`'s shared
fragment stated once; behaviour unchanged (audit 2026-10-19T11-16-25-code-audit M1, M2)

Freight on the entry: none (`route.py pins` lists no block on line 34).

## What it builds
- **cli.rs test module out.** `packages/escher-driver/src/cli.rs`'s inline `#[cfg(test)] mod tests` (line 476 to the
  file's end at 1,032, per the audit's M2) moves to a sibling file, the body unchanged.
- **recording.rs test modules out.** `packages/escher-driver/src/recording.rs`'s inline test modules
  (`off_unix_tests` from line 454 and `tests` from line 470 to the file's end at 981) move out to sibling files,
  bodies unchanged.
- **One shared log-check fragment.** The fragments that `examples/seven_guis/tests/host_log.rs` and
  `examples/seven_guis/tests/mcp_log.rs` hold twice (the audit's M1: 6 clones, 95 lines) are stated once, in the
  seven_guis test crate's shared `tests/common/mod.rs`, and both checks call it.
- **Behaviour unchanged.** No production code path, public item, wire, carrier, log field, verb or exit status
  changes. Every test that ran before runs after, under the same name, with the same assertions.

## Boundaries
- The seam (verified: `mcp.rs:291-293`, `schema.rs:482-484`) is the one 0.2.0's «File splits» chunk (2026-10-16-file-splits) used: `#[cfg(test)]
  #[path = "{name}_tests.rs"] mod tests;` in the source file, the module body in a sibling `{name}_tests.rs` under
  `packages/escher-driver/src/` (as `mcp_tests.rs` and `schema_tests.rs` stand today). `off_unix_tests` keeps its
  own `cfg` gate and module name.
- Each split source body ends up under 500 lines (the audit's M2 proposal), and both files leave the
  over-800 code-line set (verified: production code ends at `cli.rs:474` and `recording.rs:451`).
- Out of scope — the audit's other directions, which the working entry does not name: reuse of
  `command.rs`/`wire.rs`'s readers in the recording module's field parsing; `stand_replay`'s Counter variant
  vs `counter.rs`; per-verb helpers for `cli.rs::read`. The M1 clone pairs other than `host_log`↔`mcp_log` stay.
- A test-only change: no engine, driver or stand behaviour; no spec-source amendment is required beyond
  the documents that name where these tests live and count them (verified by the P2 extracts and histories:
  test-plan §1 · §2 · §4, architecture §Conventions → Tests, the testing rule's "five modules sit in files of their
  own", and line citations into the five files — wrap's cascade re-derives them).
- The CI coverage leg's per-crate floor and the mutation tier read nothing this split moves (verified:
  `coverage_floor.py` sums every `SF` record under the crate directory, so moved lines stay counted; the coverage
  leg excludes only `keyboard_compat\.rs$`; `.config/nextest.toml`'s `mutants` override names no `cli::` or
  `recording::` test, and the test paths are unchanged anyway).

## Surfaces touched
- `packages/escher-driver/src/cli.rs`, `packages/escher-driver/src/recording.rs` + new sibling test files.
- `examples/seven_guis/tests/host_log.rs`, `examples/seven_guis/tests/mcp_log.rs`,
  `examples/seven_guis/tests/common/mod.rs`.

## CI read at take-up (Setup 5a)
- `f98a4dce1` (the last wrap's flip, HEAD): CI#39319403188 — verdict not yet available (in progress, 17 checks,
  the oldest running Coverage report at 28 s). No red to fold.
