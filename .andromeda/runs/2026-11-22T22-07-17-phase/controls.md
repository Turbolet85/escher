# P5 controls — 2026-11-22-upstream-sync-ahead-of-the-canvas (validation.md check 4 (9))

Scratch clone: `git clone --shared` of the project, detached at HEAD `31c8155fccd78e02714bed98eb03d7935f261f41`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/df92c0a5-ca68-456b-9edd-67897595ea4e/scratchpad/ctl`. Each
entry fired as itself: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk
2026-11-22-upstream-sync-ahead-of-the-canvas --docs-root . --project-root {the clone} --only {n}`.

## Unmodified (must read as on the tree)
- 1 red (exit 1) · 2 green (exit 1, no output) · 3 green · 4 green · 11 red (exit 1, last line 0) — equal to the tree's
  baseline readings.

## With the control input
- Input A — `git -c rerere.enabled=false merge --no-commit --no-ff 165d698c31d00b9614664dddfbb865ff030948b3` in the
  clone (exit 1: the two conflicts), and `deny.toml:21`'s reason given `; owned by the residual row res-y4gizreq that
  drops this ignore when a winit 0.31 beta chain release leaves ttf-parser 0.25.1` before `; re-checked`:
  - 1 green (exit 0) — the must-pass reading: MERGE_HEAD is the pinned commit.
  - 2 red (exit 0, 122 B: the markers in `.github/workflows/wpt.yml`) — the known positive; its pass side is HEAD's.
  - 3 red (exit 1: `AGENTS.md` stands) — the known positive; its pass side is HEAD's.
  - 11 green (exit 0, last line 1) — the must-pass reading of step 7's clause, the literal spelt as the plan spells it.
- Input B — one line `# control` appended to the clone's `Cargo.lock`:
  - 4 red (exit 1) — the known positive; its pass side is HEAD's.
