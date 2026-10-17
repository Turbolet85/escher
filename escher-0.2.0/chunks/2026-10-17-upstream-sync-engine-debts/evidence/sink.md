# Sink — the allowlist re-checked against upstream's delta

- **Upstream's delta adds 0 log, print, env, listener or bind sites** (entry 19, a fixed-sha read of
  `git diff 434e06615 23b9b26a9 -- '*.rs'` for `tracing::|debug!|trace!(|info!|warn!|error!|println!|eprintln!|env::var|log::|TcpListener|UnixListener|UdpSocket|.bind(`
  over added lines): count 0, exit 1 — green.
- **The two new upstream test files** (`tests/blitz-tests/tests/accessibility_description.rs`,
  `accessibility_label.rs`): `command grep -c -E 'env::|std::env|println!|eprintln!|print!|dbg!|tracing|subscriber|log::'`
  reads 0 in each — 0 env reads, 0 prints, 0 subscribers.
- **The capture checks at `trace`, both builds:**
  - workspace build (`fast`'s test leg, `target/ci-logs/test.log`): `tests/host_log.rs` `ok. 1 passed; 0 failed`,
    `tests/mcp_log.rs` `ok. 1 passed; 0 failed`;
  - per-package build (entry 37, `cargo test -p seven_guis --locked`): `tests/host_log.rs` `ok. 1 passed; 0 failed`,
    `tests/mcp_log.rs` `ok. 1 passed; 0 failed`.
  Each asserts 0 id, name, typed-sentinel and shot-prefix needles in the host's or the `mcp` role's stderr.
- **No prefix, safe-field or scrub set moves:** `packages/escher-telemetry` is unchanged against HEAD `143b6013`
  (entry 11, the preservation guard, green). In `packages/escher-driver` only `src/execute.rs` moves — its key
  spellings and the macOS `ACTION_MODIFIER` — so `command` keeps its eight fields and `tool_call` its two.
