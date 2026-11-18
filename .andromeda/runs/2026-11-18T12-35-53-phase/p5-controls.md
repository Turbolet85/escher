# P5 controls — 2026-11-18-served-error-page, revision 1

## Entry 10 — gate.py guard, base e4c8e304, the twenty touchpoints and the two dependency files allowed
In place (the tree as the stopped implement run left it, trail `gate-2026-11-18-served-error-page.json` of this run dir):
`10 probe green · exit 0 · 0.61s` — `guard: clean`, where the implement run read `guard: outside 1 files` (Cargo.lock).

Clone: `git clone --shared` of the project at HEAD `e4c8e3047037f25503ef7ac4670fbfea905ff4f4`, at
`/tmp/claude-1000/<session>/scratchpad/guardctl`; each run fired through `gate.py run --chunk
2026-11-18-served-error-page --docs-root . --entry 10 --project-root {clone}` (no `--run-dir`: a clone takes none).
- input it must pass: a line appended to the clone's `Cargo.lock`, `examples/seven_guis/Cargo.toml` and `README.md`:
  `10 probe green · exit 0` — `guard: clean`.
- stray file it must fail: the same, plus a line appended to `scripts/cold-agent.ps1`: `10 probe red · last line
  'guard: clean' ✗ (last: 'guard: outside 1 files')`.
- stray file it must fail (the new alternatives stay anchored): `scripts/cold-agent.ps1` restored, a line appended to
  `packages/blitz-dom/Cargo.toml`: `10 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files')`.
