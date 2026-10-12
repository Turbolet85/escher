# P5 controls — 2026-10-12-self-description

Fired as the entries (`gate.py run --entry {n} --project-root {clone}`), over a scratch clone made with `git clone --shared` of the project at HEAD `bf973d33eee0d9776530fbb7bffebf50ab84a539`. The clone lives in the session scratchpad. Each input was an uncommitted edit in the clone, reverted after; the clone read 0 changed paths at the end.

| entry | clone | input | reading |
|---|---|---|---|
| 9 (lock and manifests unchanged) | unmodified | none | green · exit 0 |
| 9 | must-fail | a line appended to `Cargo.lock` | red · exit 1 |
| 10 (no print or panic outside tests) | unmodified | none (`describe.rs` absent) | green · exit 1, last line 0 — vacuous |
| 10 | must-fail | a `describe.rs` whose function calls `println!` | red · exit 0 (count 1) |
| 10 | must-pass | a `describe.rs` with no print, an `unwrap` inside its `#[cfg(test)]` module | green · exit 1, last line 0 |
| 12 (nothing outside the write set moves) | unmodified | none | green · exit 0 |
| 12 | must-fail | a line appended to `packages/escher-driver/src/execute.rs` | red · exit 1 |

One must-fail firing of entry 10 read red for a cause other than the count: with `describe.rs` absent and a `println!` placed in `mcp.rs`, `sed` failed (exit 2). It was re-fired with `describe.rs` present, and that firing is the one recorded above.
