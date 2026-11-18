# P5 controls — 2026-11-18-cold-agent-bar

Clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/d50dff12-b445-46a6-8e83-6b471c0da412/scratchpad/clone-ctl-1`
(`git clone --shared` of the project at HEAD `764b4ba63b4d8acfd1808572f75999085d531b1e`). Each entry was fired as itself:
`gate.py run --chunk 2026-11-18-cold-agent-bar --docs-root . --entry {n} --project-root {clone}`, no `--run-dir`.

| entry | clone input | reading |
|---|---|---|
| 4 (secret search over README.md, docs/capabilities.md) | unmodified | `green · exit 1 · 0 B` — matches the tree's baseline |
| 4 | README.md + a line holding `/home/<user>/x` | `red · exit 1 ✗ (exit 0) · 47 B` — the known positive fires |
| 5 (guard, base 764b4ba6) | unmodified | `green · exit 0` — `guard: clean` |
| 5 | README.md edited (an allowed touchpoint) | `green · exit 0` — must-pass, `guard: clean` |
| 5 | scripts/cold-agent.ps1 edited (outside the allow set) | `red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files')` — must-fail |

Each input was reverted with `git checkout` in the clone after its reading. Nothing was written to the tree.
