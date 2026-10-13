# P5 controls — 2026-10-13-stand-contrast-harness

Each entry fired as the entry, `gate.py run --plan escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/plan.md
--entry {n} --project-root {scratch clone}`, over a `git clone --shared` of the project at
198bf85cb674498549cd8f36721041680770e7ce in the session scratchpad. Every input lived in the clone, uncommitted.

| entry | clone unmodified | must-fail input | reading | must-pass input | reading |
|---|---|---|---|---|---|
| 5 lock `[[package]]` count | green · exit 1 | a `[[package]]` block appended to `Cargo.lock` | red · exit 0 | `"color",` added to blitz-test-harness's dependency list in `Cargo.lock` | green · exit 1 |
| 6 `contrast.rs` census outside tests | green · exit 1 (file absent) | `contrast.rs` whose `pub fn` calls `println!` | red · exit 0 | `contrast.rs` with `unwrap()` and `println!` only inside `#[cfg(test)] mod tests` | green · exit 1 |
| 7 `stand_contrast.rs` census | green · exit 1 (file absent) | `stand_contrast.rs` calling `eprintln!` | red · exit 0 | `stand_contrast.rs` with `mod common;` and one assertion | green · exit 1 |
| 8 harness manifest `tracing` | green · exit 1 | `tracing = { workspace = true }` appended to the harness manifest | red · exit 0 | `color = { workspace = true }` appended instead | green · exit 1 |
| 9 preservation guard | green · exit 0 | a comment line appended to `packages/blitz-paint/src/color.rs` | red · exit 1 | a comment line appended to `packages/blitz-test-harness/src/lib.rs` (the write set) | green · exit 0 |
