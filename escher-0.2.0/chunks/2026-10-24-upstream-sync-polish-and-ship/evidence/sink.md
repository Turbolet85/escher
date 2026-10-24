# The sink allowlist re-check (step 5)

Read on the take, 2026-10-24T11:51Z; the gate entries named are this run's (trail `.andromeda/runs/2026-10-24T11-39-00-implement/gate-2026-10-24-upstream-sync-polish-and-ship.json`).

## The take's added lines in the engine crates — the fixed-sha census
Entry 11 (`git show --format= {the ten shas} -- 'packages/*.rs' | grep -c -E '^\+[^+].*(tracing::|debug!|trace!\(|info!|warn!|error!|println!|eprintln!|dbg!|env::var|env!|std::env|log::|TcpListener|UnixListener|UdpSocket|\.bind\(|Instant::now|SystemTime::now)'`): **1**. The one hit is upstream's commented-out `// dbg!(&data.content);` in `packages/blitz-dom/src/layout/inline/build.rs`, moved there from `construct.rs` by `28f848d3b`. The take adds no log, print, env-read, listener, bind or wall-clock site to the engine crates.

## The wall-clock invariant
Entry 12: 0 files in `packages/blitz-dom/src` outside `clock.rs` hold `Instant::now()` or `SystemTime::now()`.

## The silence census of `tests/blitz-tests/tests/hit_test_dropped_nodes.rs`
0 env reads (`env::` / `env!`) · 0 prints (`println!` / `eprintln!` / `print!` / `dbg!`) · 0 subscribers (`tracing_subscriber` / `escher_telemetry` / `set_global_default` / `fmt::init`).

## The capture checks, both builds
- per-package build — entry 32, `cargo test -p seven_guis --locked`: green, 12 result lines · 36 passed · 0 failed · 0 ignored, with `Running tests/host_log.rs` and `Running tests/mcp_log.rs` among them;
- workspace build — entry 19, `bash .github/scripts/ci-leg.sh fast`: green, `host_log` and `mcp_log` inside the workspace tests.
- `packages/escher-telemetry/`, `packages/escher-driver/` and `packages/dioxus-native-dom/` are unchanged against the base (`git diff --cached --stat a356f9086 -- …` empty; entry 6, the preservation guard, green).

## For the wrap's registration — `examples/multi_doc.rs` (upstream's, from `fb297a38c`)
- three env variables through two `std::env::var` call sites: `MULTI_DOC_N` (line 55, parsed with a default fallback), `MULTI_DOC_SEQ` (line 36) and `MULTI_DOC_FLAT` (line 22), both through `env_flag(name)` (lines 17–18). Entry 13 reads 3 names. Non-secret, an upstream example's knobs;
- one `println!` (line 111): example output on stdout, no log line;
- `Instant::now()` twice (lines 61, 64), an example outside blitz-dom; `panic!("unknown mode {other}")` on an unknown mode (line 108);
- no `tracing`/`log` site, subscriber, listener or bind.
