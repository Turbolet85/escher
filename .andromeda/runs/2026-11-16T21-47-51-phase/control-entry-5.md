# Entry 5's must-pass control — P5 check 4 (9)

Clone: `git clone --shared` of the project at HEAD 21a5bacbf26a8804573473c976a40dd06f099c5d, in the session scratchpad
(`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/8567c733-6939-4a99-9ac9-498ecb9a67fa/scratchpad/clone-e5`).
Fired as the entry: `gate.py run --chunk 2026-11-16-error-field-in-the-answer --docs-root . --entry 5 --project-root {the clone}`.

- unmodified (the clone as HEAD holds it) — `5 probe red · exit 1 ✗ (exit 0) · exit 0` — the help still lists `error`, as on the tree.
- input it must pass (the help's sentence with `error, ` removed: `request, error, panic.payload or password` → `request, panic.payload or password` at `packages/escher-driver/src/schema.rs:194`) — `5 probe green · exit 1` — `last line 0` held.
