# Sink allowlist re-read — 2026-10-27-upstream-sync-engine-debts

Read 2026-10-27, run `.andromeda/runs/2026-10-27T09-58-03-implement/` (gate logs under
`$TMPDIR/andromeda-gate/2026-10-27-upstream-sync-engine-debts/implement-2026-10-27T09-58-03/`).

## The fixed-sha census of the take's added engine lines (gate entry 7)

`git show --format= {the eleven} -- 'packages/*.rs'`, added lines matching a tracing, print, env, listener, bind or
wall-clock site: **2** (green, `last line 2`). By name:

1. `tracing::error!("Uncaught JS error in {what}: {error}");` — `packages/blitz-vibey-script/src/runtime/events.rs:21`,
   inside `report_js_error`, under `#[cfg(feature = "tracing")]` (`:20`). A moved line: `e6f4615ec` takes it whole out
   of `runtime.rs` (HEAD `:1105`). Target `blitz_vibey_script`, an engine target, so the sink prints its safe fields
   alone and the message `[redacted]`.
2. `std::env::temp_dir().join(format!("blitz-vibey-script-{name}-{}", std::process::id()));` —
   `packages/blitz-vibey-script/tests/dom.rs:863`, in `temp_dir_with_files` (`cee6b8731`), test code only: `TMPDIR`
   read through `std::env::temp_dir()`, not secret — a second test-only site beside escher-driver's client unit test,
   for the wrap's registration (plan §Implementation notes).

No `println!`, `eprintln!`, `dbg!`, `std::env::var`, `env!`, `log::`, listener, `.bind(`, `Instant::now` or
`SystemTime::now` among the added lines.

## The clock invariant (gate entry 8)

`grep -rlE 'Instant::now\(\)|SystemTime::now\(\)' packages/blitz-dom/src | grep -vc 'clock\.rs$'` → 0 (exit 1).

## The silence census of `pseudo_element_flow_classification`

`tests/blitz-tests/tests/pseudo_element_flow_classification.rs` (78 lines): **0 env** (`env::|env!|std::env`) ·
**0 print** (`println!|eprintln!|print!|dbg!|tracing::|log::`) · **0 subscriber**
(`tracing_subscriber|set_global_default|escher_telemetry|fmt::init`).

## The capture checks, both builds

- Per-package build: `cargo test -p seven_guis --locked` (entry 28) green — 12 result lines, 41 passed, 0 failed —
  `Running tests/host_log.rs` and `Running tests/mcp_log.rs` among them.
- Workspace build: `bash .github/scripts/ci-leg.sh fast` (entry 15) green; `target/ci-logs/test.log` holds
  `Running tests/host_log.rs` and `Running tests/mcp_log.rs`, no `FAILED`.

## escher's own crates

`git diff --stat de85792f5 -- packages/escher-telemetry packages/escher-driver`: empty — the sink's prefix and scrub
sets and the driver's spans unchanged.
