# P5 known-positive controls — 2026-10-13-stand-keyboard-harness

Each control fired AS THE ENTRY (`gate.py run --plan …/plan.md --entry {n} --project-root {clone}`, no run dir) over a
scratch clone, `git clone --shared` of the project at HEAD `e40d94371643eef83015c6a17f0ae750c8fbe4c0`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/b2bd5f17-9893-492a-9882-0db6ffd91a12/scratchpad/clone-p5`
(the clone driven by the script `…/scratchpad/controls.sh`, each firing's whole output read from
`…/scratchpad/controls-out/control-{k}.txt`). Each control's input was left uncommitted in the clone and restored after
its firing; the clone's `git status --porcelain` read empty at the end.

| firing | entry | input | reading |
|---|---|---|---|
| 1 | 7 · lock | unmodified | green · exit 1 (as on the real tree) |
| 2 | 8 · engine census | unmodified | green · exit 1 (as on the real tree) |
| 3 | 9 · silence | unmodified | red · exit 2, the file absent (as on the real tree) |
| 4 | 10 · write-set guard | unmodified | green · exit 0 (as on the real tree) |
| 5 | 7 | must-fail: a `[[package]]` block appended to `Cargo.lock` | red · exit 0 |
| 6 | 7 | must-pass: a non-package line added inside the lock | green · exit 1 |
| 7 | 8 | must-fail: a fn calling `println!` appended to `packages/blitz-dom/src/events/keyboard.rs` | red · exit 0 |
| 8 | 8 | must-pass: a comment and a plain fn appended to the same file | green · exit 1 |
| 9 | 9 | must-fail: `stand_keyboard.rs` calling `eprintln!` | red · exit 0 |
| 10 | 9 | must-pass: `stand_keyboard.rs` with `mod common;` and one assertion | green · exit 1 |
| 11 | 10 | must-fail: a line appended to `packages/blitz-dom/src/node/node.rs` (F4 deferred) | red · exit 1 |
| 12 | 10 | must-pass: a line appended to `packages/blitz-dom/src/events/keyboard.rs` (the write set) | green · exit 0 |
