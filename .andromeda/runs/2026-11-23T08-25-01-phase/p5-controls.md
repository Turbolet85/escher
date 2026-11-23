# P5 controls — 2026-11-23-canvas-placement-and-text

Entry 9 (the lock probe, `git diff --numstat d43bc0e7f… -- Cargo.lock | awk …`, `last line 1+0`): its literal is the
run's own printed form; must-pass reading over a scratch clone, `git clone --shared` of the project, detached at
`d43bc0e7f7f9c992760230e71eeba19c708935e8`, at `$TMPDIR/{scratchpad}/lockctl`, each firing
`gate.py run --chunk 2026-11-23-canvas-placement-and-text --docs-root . --entry 9 --project-root {the clone}`:

- clone, unmodified (the tree's reading reproduced): `9 probe red · last line '1+0' ✗ (last: '') · exit 0`
- clone carrying the change — `tests/blitz-tests/Cargo.toml` gains `parley = { workspace = true }` after the
  `peniko` line and the lock written by `cargo tree --offline -p blitz-tests --depth 0` (exit 0; numstat `1 0
  Cargo.lock`): `9 probe green · exit 0 · 4 B`

Entries 3–6 read red by absence (`error: no test target named …`): they show only that each entry can read red; their
control is implement's red-before-fix (steps 1, 3, 6) and step 8's `gate.py control`.
