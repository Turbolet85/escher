# P5 controls — 2026-10-19-recorded-check-replay

Each control fired as the entry (`gate.py run --plan … --entry {n} --project-root {clone}`, no run dir) over a scratch
clone, `git clone --shared` of the project at HEAD `6fd1d3daf6a111c49d478967611b1014359eae81`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/6a276f86-0e02-46a0-b893-aa9cd8728857/scratchpad/clone`. The
inputs were written by `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/6a276f86-0e02-46a0-b893-aa9cd8728857/scratchpad/controls.py`
and left uncommitted in the clone. Each was applied after the previous one, in this order.

## Unmodified clone (the entry must read what it read on the real tree)
- 4 · red · exit 2 (`replay.rs` absent) — the same as the real tree.
- 5 · green · exit 0 — the same.
- 6 · red · exit 1, last line `0` — the same.

## Entry 4 (silence probe, `last line 0`)
- must-pass, input `silent` (`replay.rs` and `stand_replay.rs` holding plain Rust, no print): green · exit 0 · last line 0.
- must-fail, input `loud` (an `eprintln!` added to `replay.rs`): red · last line `1`.

## Entry 6 (`app.rs` moves by three lines, `last line 3`)
- must-pass, input `pub` (`/// The stand's chrome around one task.` added above `#[component]`, `fn TaskShell` → `pub fn TaskShell`): green · last line 3.
- must-fail, input `markup` (the above plus a class added to `div { id: "home", … }`): red · last line `5`.

## Entry 5 (the stand tasks, boot and host untouched)
- must-pass, inputs `silent`, `pub` and `markup` in the tree (none under the guarded paths): green · exit 0.
- must-fail, input `task` (a comment appended to `examples/seven_guis/src/tasks/counter.rs`): red · exit 1.
