# P5 controls — 2026-10-24-a11y-re-read

Each control fired AS THE ENTRY: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan
{clone}/escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md --project-root {clone} --entry {n}`, no run dir. The clone
is `$TMPDIR/{scratchpad}/clone`, a
`git clone --shared` of the project at HEAD `db585b7116b5bd6b5d9baf15acac86937507cba3`, the chunk's scope, research and
plan copied in (untracked, outside every entry's pathspec). The inputs were written by
`$TMPDIR/{scratchpad}/ctl_edit.py`,
one mode per input. Read 2026-10-24T21:03:30Z–21:03:56Z.

## Unmodified (each reads what it read on the real tree)
- 3 red, last line the six words (the subject not yet built) · 4 red, last line empty · 8 green, exit 1 · 10 green, last
  line 0 · 11 green, `guard: clean`, changed 0.

## Must-pass — steps 1 and 2 applied as the plan writes them (mode `plan`)
- 3 green: the fourteen words in order.
- 4 green: `1 1`.
- 10 green: last line 0.
- 11 green: `guard: allowed 2 — .github/scripts/ci-leg.sh, .github/scripts/test_ci_workflows.py · … · changed 2`,
  `guard: clean`.
- the pin, `python3 -m unittest discover -s .github/scripts -p test_ci_workflows.py` in the clone: `Ran 33 tests`, `OK`.
- 2 green: `lines 14 passed 84 failed 0 ignored 0`, over a `target/ci-logs/a11y.log` concatenating the real tree's
  last a11y leg log (six result lines) and the P4 workspace run of the eight joining files (eight result lines).

## Must-fail — on top of the must-pass input
- mode `second-line` (a second line of ci-leg.sh edited) → 4 red, last line `2 2`.
- mode `unlocked` (`--locked` dropped from the a11y line) → 3 red, exit 1 (no line matches).
- mode `engine` (packages/escher-driver/src/lint.rs edited) → 10 red, last line `1`.
- modes `engine` and `stray` (an untracked tests/blitz-tests/tests/stray_extra.rs) with the edits above → 11 red,
  `guard: outside 3 files`.
- mode `println` (a `println!` planted in tests/blitz-tests/tests/inert_subtree.rs) → 8 red, exit 0 (count 1).
