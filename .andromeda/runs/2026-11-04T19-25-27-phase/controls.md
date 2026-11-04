# P5 check 4 (9) — controls, fired as the entries over a scratch clone

Clone: `git clone --shared` of the project at HEAD `fd2e99389e586b804e0c40df9dfb96cf03936830`, at
`$TMPDIR/{scratchpad}/clone`, each entry fired through `gate.py run --chunk 2026-11-04-dioxus-runtime-probe --docs-root .
--entry {n} --project-root {clone}`.

- entry 2 (silence census), unmodified clone: `red · exit 1 ✗ (exit 2)` — the file absent, as on the tree.
- entry 2, input a silent stand-in `tests/blitz-tests/tests/dioxus_runtime_probe.rs` (a `//!` doc and one test, no print): `green · exit 1` (must-pass).
- entry 2, the same stand-in with one `println!` line appended: `red · exit 1 ✗ (exit 0)` (must-fail).
- entry 3 (preservation guard), unmodified clone: `green · exit 0`, as on the tree.
- entry 3, input a one-line comment appended to the tracked `packages/dioxus-native-dom/src/lib.rs`: `red · exit 0 ✗ (exit 1)`.
- entry 3, input an untracked `examples/control-untracked.txt`: `red · exit 0 ✗ (exit 1)`.

Every control input was removed from the clone after its read; the clone's `git status --porcelain` read empty.
