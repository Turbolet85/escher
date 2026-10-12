# P5 controls — 2026-10-12-headless-screenshot

Each control fired as its entry (`gate.py run --plan … --entry {n} --project-root {scratch clone}`, no run dir) over a
`git clone --shared` of the project at HEAD 938cd1b878ef01533d478fffed30139b3a3b3430, in the session scratchpad. The
clone's inputs were uncommitted edits; only the verdicts are recorded here.

| entry | clone input | line read |
|---|---|---|
| 10 | unmodified | `10 probe green · exit 1` (count 0) |
| 10 | must-fail: a `[[package]]` line appended to Cargo.lock | `10 probe red · exit 1 ✗ (exit 0)` (count 1) |
| 10 | must-pass: one dependency line added inside an existing package of Cargo.lock | `10 probe green · exit 1` (count 0) |
| 11 | unmodified (neither file exists) | `11 probe green · exit 1` — vacuous |
| 11 | must-fail: `image.rs` calling `println!`, a clean `screenshot.rs` | `11 probe red · exit 1 ✗ (exit 0)` |
| 11 | must-pass: a clean `image.rs` whose `#[cfg(test)]` module calls `unwrap()`, a clean `screenshot.rs` | `11 probe green · exit 1` (count 0) |
| 13 | unmodified | `13 probe green · exit 0` |
| 13 | must-fail: a line appended to `packages/escher-driver/src/session.rs` | `13 probe red · exit 0 ✗ (exit 1)` |
| 13 | must-pass: a line appended to `packages/escher-driver/src/execute.rs` (inside the write set) | `13 probe green · exit 0` |
