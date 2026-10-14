# P5 controls — 2026-10-14-stand-requirement-sweep

Each control fired as the entry itself: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan
escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/plan.md --entry {n} --project-root $TMPDIR/{scratchpad}/clone`,
over a `git clone --shared` of the project at HEAD 19756f2d60a0c770afe5b70ca51426019e33d02b (the clone's role: the
untouched tree, then each control's input left uncommitted in it). Script: `$TMPDIR/{scratchpad}/controls.sh`.

## Entry 7 — the silence census
- unmodified (the three new files absent): green · exit 1 · last line 0 — as on the real tree
- must-fail, a new file holding `eprintln!`: red · last line 1
- must-pass, the three new files present and silent: green · exit 1 · last line 0

## Entry 8 — the preservation guard (base 19756f2d)
- unmodified: green · exit 0 — as on the real tree
- must-fail, a line appended to `packages/blitz-dom/src/lib.rs`: red · exit 1
- must-pass, `packages/escher-driver/src/refusal.rs` and `examples/seven_guis/src/tasks/cells.rs` edited and a new
  test file added: green · exit 0
- must-fail, a line appended to the lean task `examples/seven_guis/src/tasks/crud.rs`: red · exit 1

## Entry 9 — the attribute-only probe over the task files (base 19756f2d)
- unmodified: green · exit 1 · last line 0 — as on the real tree
- must-pass, one `id: "temp-celsius",` line added in `temp_converter.rs`: green · exit 1 · last line 0
- must-fail, a `class: "x",` line added in `temp_converter.rs`: red · exit 0
- must-fail, an existing line rewritten in `temp_converter.rs`: red · exit 0
