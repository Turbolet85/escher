# P5 controls — 2026-11-20-codebase-health

The clone: `git clone --shared` of the project at HEAD `29b3a5fb74ae0cdd0cc4e3bf089afd1c72e7f925`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/4a7e6bab-271b-4f01-a303-3157cfcb5daa/scratchpad/ctl1`, the
chunk's `evidence/` copied into it (gitignored there as here). Each entry was fired through
`gate.py run --chunk 2026-11-20-codebase-health --docs-root . --entry {n} --project-root {the clone}`.

## Unmodified clone (must read as the tree)
- entry 5 `bars.py` — red · last `bars: FAIL` (the tree's baseline: red · `bars: FAIL`)
- entry 7 `scope_guard.py 29b3a5fb7…` — green · `scope guard: PASS` (tree: green)
- entry 8 the census — green · last line `11` (tree: green · `11`)

## Must-pass input — the subject trimmed
Input: `json_tests.rs`, `command.rs` and `snapshot.rs` cut to their first 100 lines; `cli.rs`, `mcp.rs`,
`dioxus_runtime_probe.rs`, `stand_graph.rs`, `stand_actions.rs` and `stand_accessibility_ids.rs` emptied (nine
allowed files modified, nothing else).
- entry 5 — green · `bars: PASS` (ours over 800 outside the seven: 0; ours over 15: 0)
- entry 7 — green · `scope guard: PASS` (only allowed files changed)

## Must-fail input — an upstream-authored edit, an outside file, a log token
Input (the clone reset first): one comment line appended to `packages/blitz-dom/src/document.rs` (upstream-authored),
a new `tests/blitz-tests/tests/zz_outside_control.rs` (outside the twelve), one `// println!("control");` line
appended to `packages/escher-driver/src/cli.rs`.
- entry 7 — red · `scope guard: FAIL`
- entry 8 — red · last line `12` (11 expected)

## Re-fired after the operator's review (`inputs#I3`): the allowed list takes `command/key.rs` for `command_tests.rs`
Entry numbers after the doc leg joined as entry 5: the scope guard is 8, the census 9. The clone was reset from the
must-fail input to the same HEAD between the reads.
- must-fail input (as above) — entry 8 red · `scope guard: FAIL`; entry 9 red · last line `12`
- must-pass input: a new `packages/escher-driver/src/command/key.rs` and one comment line in `command.rs` — entry 8
  green · `scope guard: PASS`

## Test-path guard (an input operand alone; the clone pair cannot hold a workspace build)
- entry 6 on the tree — green · `test paths: PASS` (1,631 listed, 1,631 in the base)
- the same `run` with its FILE operand a copy of the base minus its last three paths plus one path no tree holds,
  typed as the tool types an entry (`bash -o pipefail -c 'python3 …/test_paths.py check {that copy}'`) — exit 1 ·
  `gone 1 · new 3` · `test paths: FAIL`
