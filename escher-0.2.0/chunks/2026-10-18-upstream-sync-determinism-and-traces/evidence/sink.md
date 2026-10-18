# Sink allowlist re-check — 2026-10-18-upstream-sync-determinism-and-traces

Measured by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18.

## Upstream's delta
- Research's sweep over the whole five-commit Rust delta (`git diff 23b9b26a9 upstream/main -- '*.rs'`) found 0 log,
  print, env-read, subscriber or listener lines in 16 added lines (research.md §Sweeps).
- The taken commit, read at its fixed sha (entry 6, trail
  `gate-2026-10-18-upstream-sync-determinism-and-traces.json`):
  ```
    6 probe       green · exit 1 · 0.01s · 2 B → 6.log · git show fb8d6c15e9ce062b3b41e71cd48988f0093506cc -- '*.rs… (223 chars)
  ```
  Its log reads `0`. `fb8d6c15e` adds no `tracing::`/`log::`/print/`dbg!`/env read/listener/`bind(` site.

## The standing capture checks, both builds
- **Per-package build** (entry 23, `cargo test -p seven_guis --locked`): green, 11 result lines · 26 passed · 0 failed,
  no `FAILED`.
  - `Running tests/host_log.rs`: `1 passed; 0 failed`.
  - `Running tests/mcp_log.rs`: `1 passed; 0 failed`.
- **Workspace build** (inside entry 11, `ci-leg.sh fast`, `target/ci-logs/test.log`):
  - `Running tests/host_log.rs`: `1 passed; 0 failed`.
  - `Running tests/mcp_log.rs`: `1 passed; 0 failed`.
- Both checks read 0 id, name, typed-sentinel and shot-prefix needles at `trace`, as they assert.

## escher's sink
- `git diff --quiet fb842671 -- packages/escher-telemetry` exits 0, so `packages/escher-telemetry/` is unchanged.
  The preservation guard (entry 3) covers the same, with every path outside `document.rs`.
- The allowlist (`packages/escher-telemetry/src/format.rs`) admits the engine and `escher_*` target families. The take
  adds no target, so nothing new reaches it.
