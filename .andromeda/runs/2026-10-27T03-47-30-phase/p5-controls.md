# P5 controls — 2026-10-27-readme-and-records

Fired by `.andromeda/runs/2026-10-27T03-47-30-phase/p5-controls.sh` (read by 2026-10-27T05:08Z): each entry through
`gate.py run --plan escher-0.3.0/chunks/2026-10-27-readme-and-records/plan.md --entry {n} --project-root {clone}`, no
run dir, over a `git clone --shared` scratch clone at `316c6d25b59a` in the script's own `$(mktemp -d)`, the chunk
folder copied in, the clone reset between controls. Each control's input lived in the clone only.

## Unmodified (must read what the real tree read)
- 1 green · 2 red (last 1) · 3 red (exit 0) · 4 green · 5 red (exit 0) · 6 red (exit 1) · 7 green · 8 green · 10 red
  (exit 1, records.md absent) · 11 green (guard: clean). All equal the real tree's baselines.
- 9 green in the clone and red on the real tree (2): a clone carries git's template `.git/info/exclude`, not the
  local one. The must-fail control below supplies the real tree's input.

## Controls
- 1 must-fail, README's pinned phrase removed: red (exit 1).
- 2 must-pass, the three status headings present: green (last line 3).
- 3 must-pass, the 0.2.0 planned heading replaced: green (exit 1, 0).
- 4 must-fail, a lowercase «escher» in prose: red (exit 0).
- 5 must-pass, the three stale lines removed: green (exit 1, 0).
- 6 must-pass, the 2026-10-25 run sentence present: green (last line 1).
- 7 must-fail, a `/home/` path in the statement: red (exit 0).
- 8 must-fail, `.gitignore:39` removed: red (exit 1).
- 9 must-fail, an incubator line in the exclude: red (exit 0).
- 10 must-pass, `evidence/records.md` naming the three names: green (names: 3).
- 11 must-pass, README edited: green (guard: clean). Must-fail, a tracked source file edited
  (`packages/blitz-dom/src/lib.rs`): red (guard: outside 1 files). Must-fail, an untracked file minted under
  `packages/`: red (guard: outside 1 files).
- 14 (gated, deferred, not fired by the tool): its `run` typed in the tool's shell form, `/usr/bin/bash -o pipefail
  -c`, read 0 at exit 1 on the default branch's README. Its literals are the headings entries 2's must-pass reads, so
  the README this chunk writes carries them.
