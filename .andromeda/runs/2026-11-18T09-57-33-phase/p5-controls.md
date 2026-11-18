# P5 controls — 2026-11-18-served-error-page

Clone: `git clone --shared` of the project at HEAD `e4c8e3047037f25503ef7ac4670fbfea905ff4f4`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/ce669042-e58f-4c17-b5af-9d69d69e92fe/scratchpad/p5clone`;
each entry fired through `gate.py run --chunk 2026-11-18-served-error-page --docs-root . --entry {n} --project-root {clone}`.

## Entry 8 — the four-term secret search over README.md and docs/capabilities.md
- unmodified clone: `8 probe green · exit 1 · 0 B` — as on the tree.
- input: a line `planted /home/someone line` appended to the clone's README.md (uncommitted): `8 probe red · exit 1 ✗
  (exit 0) · 41 B` — the known positive reads red.

## Entry 9 — gate.py guard, base e4c8e304, the twenty touchpoints allowed
- unmodified clone: `9 probe green · exit 0` — `guard: clean`, as on the tree.
- input it must pass: the same README.md edit: `9 probe green · exit 0` — `guard: clean`.
- input it must fail: README.md restored, a line appended to `scripts/cold-agent.ps1`: `9 probe red · last line
  'guard: clean' ✗ (last: 'guard: outside 1 files')`.
