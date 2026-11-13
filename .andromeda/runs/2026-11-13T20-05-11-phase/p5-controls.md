# P5 controls — 2026-11-13-cognitive-load-corrective

Each fired as the entry through `gate.py run --chunk 2026-11-13-cognitive-load-corrective --docs-root . --entry {n}
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/a64e5d38-40c2-4490-b1d6-5095a09ac54c/scratchpad/ctl`,
the clone a `git clone --shared` of the project at HEAD `5a4d029f31a79f542f6d5e114eb96db6ecfd4128`.

## Unmodified (the clone reads as the tree)
- entry 1 (complexity): `red · last line '3' ✗ (last: '6') · exit 0` — as on the tree
- entry 2 (census): `green · exit 0` — as on the tree
- entry 3 (mutator/document guard): `green · exit 0` — as on the tree

## Control inputs
- entry 1, must-pass: the bodies of `handle_click`, `execute` and `graph` replaced by `todo!()` in the clone
  (`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/a64e5d38-40c2-4490-b1d6-5095a09ac54c/scratchpad/stub_bodies.py`)
  → `green · exit 0`, listing touch_action_pan_axes 17, handle_pointermove 77, handle_pointerdown 39, last line 3
- entry 2, must-fail: `fn _control() { println!("x"); }` appended to `packages/escher-driver/src/execute.rs` in the clone
  → `red · last line '0' ✗ (last: '1') · exit 0`
- entry 3, must-fail: a comment line appended to `packages/blitz-dom/src/mutator.rs` in the clone
  → `red · exit 0 ✗ (exit 1) · exit 1`

Each input reverted in the clone after its read (`git checkout -- .`).
