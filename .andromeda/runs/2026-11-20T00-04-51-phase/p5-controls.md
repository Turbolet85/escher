# P5 controls — 2026-11-20-readme-and-records

Each line was fired as the entry through `gate.py run --chunk 2026-11-20-readme-and-records --docs-root . --entry {n}
--project-root $TMPDIR/{scratchpad}/ctl`, on a `git clone --shared` of the project at HEAD
`120fbc8b63959a1a90f91583790944232a82db01`.

## Unmodified clone (must read as the tree reads)
- entry 2 red (exit 1, 0) · entry 3 red (last 17) · entry 4 green · entry 5 green · entry 6 red (exit 0, stale present) ·
  entry 7 red (exit 1, 0) · entry 8 green · entry 9 green — each the reading the untouched tree gave.

## Must-pass input: the planned edits
The input was the 0.4.0 planned section (heading, lead, six epoch bullets) and the `Later:` line written into README.md,
and stack.md's five entries re-derived (scratch `ctl_edit.py pass`).
- entry 2 green · entry 3 green (23) · entry 4 green · entry 5 green · entry 6 green (exit 1) · entry 7 green (4) ·
  entry 9 green.
- entry 8 RED on the hand-edited stack.md: `render: .claude/docs/stack.md differs from the journal`, `check: 1
  failure(s)`. This is the guard's must-fail input.
- stack.md was restored, and the same bytes were landed with `rec.py harness land --root {clone} .claude/docs/stack.md
  --file {the edited copy}`: the dry run read `would land … rev 16`, and the land read `landed .claude/docs/stack.md rev
  16 · 0 new · 4 revised`. After it, entry 8 green and entry 7 green. Step 3's mechanism holds.

## Must-fail inputs for the guards
The input was the 0.3.0 heading changed to `done and verified`, the pinned phrase's `fifteen` dropped, and a line
appended to docs/capabilities.md (scratch `ctl_edit.py fail`).
- entry 4 red (last 2) · entry 5 red (exit 1) · entry 9 red (exit 1, the stat line) · entry 1 red (exit 1,
  test_the_readme_counts_the_mcp_tools).
