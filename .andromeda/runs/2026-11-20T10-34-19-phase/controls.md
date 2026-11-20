# P5 controls — 2026-11-20-readme-s-0-3-0-closed

Clone: `git clone --shared` of the project at HEAD `84c8b9e23` into `$TMPDIR/{scratchpad}/ctl-readme`, each entry fired
as itself: `gate.py run --chunk 2026-11-20-readme-s-0-3-0-closed --docs-root . --entry {n} --project-root {the clone}`.

## Unmodified clone (must read as the tree read)
- 2 red (exit 1, count 0) · 3 red (last line 2) · 4 red (exit 0, count 1) · 5 green (exit 1) · 7 green (guard clean,
  no change yet) — the same words as the tree's baseline runs.

## Input: step 1's edit (README.md line 87 → `### 0.3.0 — done and verified`; numstat `1 1 README.md`) — must pass
- 2 green · 3 green · 4 green · 5 green.
- 7 (plan rev 3, `--allow '.claude/session-handoff.md'`): red — `guard: outside 1 files` (`outside README.md`): the guard
  excuses only `--allow` matches and recorded companions, never the plan's own files. Re-spelt at plan rev 4 to
  `--allow 'README\.md|\.claude/session-handoff\.md'`; fired again on the same input: green, `guard: clean`.

## Input: step 1's edit + a line «The version-end audit's mutation testing was not finished.» after README.md:89 + a line appended to docs/capabilities.md — must fail
- 5 red (exit 0, count 1): the forbidden sentence in the 0.3.0 section is caught.
- 7 red: `guard: outside 1 files` (`docs/capabilities.md`).

## Entry 5's earlier spelling (plan rev 2, `mutation|audit`)
- red on the untouched tree, by «the functions the 0.2.0 audit found» in the 0.3.0 Epoch 1 bullet — an existing
  property, not the subject; re-spelt at rev 3 to `mutation|version-end audit`, then green on the tree.
