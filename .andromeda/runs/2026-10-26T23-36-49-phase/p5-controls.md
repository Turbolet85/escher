# P5 controls — 2026-10-26-codebase-health

Validation check 4 (9): the plan's complexity instrument (entry 1) and its inline guards (entries 6–10) fired AS THE
ENTRY — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md --entry {n} --project-root {clone}`
— over a scratch clone (`git clone --shared` of the project at HEAD `19f9ff91ca95b28c168008c7ee1298c8b1d635ab`) at
`$TMPDIR/{scratchpad}/p5-clone`, first unmodified, then with each control's input. The clone was reset (`git checkout
-- .`, `git clean -fdq`) between controls and read clean at the end. The scripts are
`$TMPDIR/{scratchpad}/p5_controls.py` and `$TMPDIR/{scratchpad}/p5_controls_guard.py`. Entry 10's controls copy the
uncommitted chunk folder into the clone after each reset: the guard needs `--chunk-dir`, and the folder lies outside
its pathspec. Its first firing without the folder exited 2 (`--chunk-dir: not a directory`) on every input and is not
a reading.

| Entry | Input | Reading |
|---|---|---|
| 1 complexity | unmodified (the must-fail side) | red · `lacks 'lint.rs findings'` ✗ — the three rows and `over 15: 88`, as on the tree |
| 1 complexity | the three bodies emptied (must-pass) | green · exit 0 — the three rows gone, `over 15: 85`, the engine row 17.0 standing |
| 1 complexity | `findings` alone emptied (must-fail) | red · `lacks 'the_listing_is_the_twelve_verbs_of_the_table'` ✗ — two rows stand |
| 6 census | unmodified | green · exit 1, last line 0 |
| 6 census | `// println!("planted");` appended to `lint.rs` (must-fail) | red · exit 1 ✗ (exit 0) |
| 7 `env!` count | unmodified | green · last line 2 |
| 7 `env!` count | a third `env!` appended to `mcp_tests.rs` (must-fail) | red · last line '2' ✗ (last: '3') |
| 8 `info_span!` sites | unmodified | green · last line 2 |
| 8 `info_span!` sites | a third `info_span!` appended to `lint.rs` (must-fail) | red · last line '2' ✗ (last: '3') |
| 9 events | unmodified | green · exit 1, last line 0 |
| 9 events | `// tracing::info!("planted")` appended to `lint.rs` (must-fail) | red · exit 1 ✗ (exit 0) |
| 10 guard | unmodified | green · `guard: allowed 0 · … changed 0` · `guard: clean` |
| 10 guard | the three allowed files edited (must-pass) | green · `guard: allowed 3 — packages/escher-driver/src/lint.rs, packages/escher-driver/src/mcp_tests.rs, tests/blitz-tests/tests/stand_requirement_sweep.rs` · `guard: clean` |
| 10 guard | `packages/blitz-dom/src/document.rs` edited (must-fail) | red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files') |
| 10 guard | an untracked `packages/escher-driver/src/stray_extra.rs` minted (must-fail) | red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files') |
| 10 guard | `Cargo.lock` edited (must-fail) | red · `guard: outside 1 files` |
