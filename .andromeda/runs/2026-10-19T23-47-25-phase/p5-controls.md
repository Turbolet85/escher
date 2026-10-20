# P5 controls — 2026-10-19-visual-lint

Each control fired AS THE ENTRY through `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan
escher-0.2.0/chunks/2026-10-19-visual-lint/plan.md --project-root $TMPDIR/{scratchpad}/clone --entry {n}` (no run dir),
over a `git clone --shared` of the project at HEAD e97a7f41b10a, 2026-10-20T00:10Z. Script:
`$TMPDIR/{scratchpad}/controls.sh`. The gated entry 20 fired through `--void 20:control — …` in the clone only.

## A — the clone unmodified (must read as the real tree read)
- 5 green · exit 0 (last line 2) · 6 green · exit 1 (cat on absent files, grep 0) · 7 green · exit 1 · 8 green · exit 1
- 20 red · exit 2 (the word file absent)

## B — must-pass: the four new files minted clean, `packages/escher-driver/src/lib.rs` and `packages/blitz-dom/src/lib.rs` edited, a valid word file
- 5 green · exit 0 (last line 2) · 6 green · exit 1, last line 0 · 7 green · exit 1, last line 0 · 8 green · exit 1, last line 0
- 20 green · exit 0, last line 1 (`| F1 | 1 | «…» | 2026-10-20 |` and an F2 row)

## C — must-fail: `info_span!` and `println!` in lint.rs, `schema.rs` edited, `packages/escher-driver/src/extra.rs` minted, a word row with option 5
- 5 red · last line 3 · 6 red · exit 0 · 7 red · exit 0 · 8 red · exit 0
- 20 red · exit 1 (last line 0)
