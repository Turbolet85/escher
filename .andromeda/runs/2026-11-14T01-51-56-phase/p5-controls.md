# P5 controls — 2026-11-14-seeded-randomness

Each control fired AS THE ENTRY through `gate.py run --chunk 2026-11-14-seeded-randomness --docs-root . --entry {n}
--project-root {clone}`, the clone a `git clone --shared` of the project at HEAD
`4f68f0d523aed2d6c3ef7e34ddccf2ca05c0506c` in the session scratchpad,
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/f23fbc41-dab7-4c6b-8c97-a5156f5731fa/scratchpad/ctl-clone`.

| entry | clone input | reading |
|---|---|---|
| 5 | unmodified (seed.rs absent) | red · exit 0 ✗ (exit 1) — as on the tree |
| 6 | unmodified (witness absent) | red · exit 0 ✗ (exit 1) — as on the tree |
| 7 | unmodified | green · exit 0 — as on the tree |
| 5 | must-pass: a clean `seed.rs` stub, a `println!` inside its `#[cfg(test)]` module only | green · last line 0 |
| 6 | must-pass: a clean witness stub printing one digest line with `println!` | green · last line 0 |
| 5 | fail: the stub with `println!("leak")` in its production body | red · last line '0' ✗ (last: '1') |
| 6 | fail: the stub with `std::thread::sleep` | red · last line '0' ✗ (last: '1') |
| 7 | fail: a line appended to `packages/escher-driver/src/lib.rs` | red · exit 0 ✗ (exit 1) |
