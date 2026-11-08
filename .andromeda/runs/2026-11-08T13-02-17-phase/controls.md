# P5 controls — 2026-11-08-what-can-i-do-here

Entry 10 (the silence census over `packages/escher-driver/src/actions.rs`, `packages/dioxus-native-dom/src/actionable.rs`
and `tests/blitz-tests/tests/stand_actions.rs`) reads green on the base vacuously, two of its three files absent. Its
two sides were fired as the entry, through `gate.py run --chunk 2026-11-08-what-can-i-do-here --docs-root . --entry 10
--project-root $TMPDIR/{scratchpad}/ctl-census`, over a `git clone --shared` of the project at
`7e2e8be6d77c7a2e6b4ed5cea1ed53056dcab03c`:

- the clone unmodified: `10 probe green · exit 1` — as on the tree;
- a silent `actions.rs` (a doc comment and an empty `fn`) and a silent `stand_actions.rs` (a doc comment and an empty
  `#[test]`): `10 probe green · exit 1` — the must-pass side;
- `actions.rs` holding `println!("x")`: `10 probe red · exit 1 ✗ (exit 0)` — the must-fail side.

Entry 9 (the no-engine, no-manifest, committed-checks guard), fired the same way over the same clone:

- the clone unmodified: `9 probe green · exit 0` — as on the tree;
- a line `# control` appended to `Cargo.toml`: `9 probe red · exit 0 ✗ (exit 1)` — the must-fail side.

The control inputs lived in the clone alone; nothing was written to the tree.
