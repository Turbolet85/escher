# P5 controls — 2026-11-23-canvas-in-a-headless-shot (validation check 4 (9))

Scratch clone: `git clone --shared` of the project at HEAD `ac8e6d58afd29ba72b36ddd741bf6d0f5c42b173`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/79afddcf-aa59-4b5f-822e-b5e438ff0fbc/scratchpad/mustpass`.
Each entry fired through `gate.py run --chunk 2026-11-23-canvas-in-a-headless-shot --docs-root . --entry {n}
--project-root {the clone}`, no run dir.

## Unmodified clone (must read as the tree does)
- entry 5 — `red · exit 0 ✗ (exit 1) · exit 1 · 2 B` (count 0), as on the tree.
- entry 7 — `red · last line '1+0' ✗ (last: '') · exit 0 · 0 B`, as on the tree.

## Clone carrying the plan's change (must-pass)
Input: `packages/blitz-test-harness/Cargo.toml`'s `blitz-paint` line given `features = ["custom-widget"]`;
`tests/blitz-tests/Cargo.toml` gaining `peniko = { workspace = true }`; `Cargo.lock` written by
`cargo tree --offline -p blitz-tests --depth 0` (`git diff --stat`: Cargo.lock 1 +, the harness manifest 1 + 1 −,
blitz-tests' manifest 1 +). Left uncommitted in the clone; nothing of it in the tree.
- entry 5 — `green · exit 0 · 106 B` (last line 3).
- entry 7 — `green · exit 0 · 4 B` (last line `1+0`).
- entry 6 — `green · exit 1 · 2 B` (0 adapter rows; the `--locked` read accepts the clone's lock).
