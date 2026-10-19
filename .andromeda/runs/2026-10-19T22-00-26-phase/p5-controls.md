# P5 controls — 2026-10-19-shared-fixtures

Every control was fired as the entry: `gate.py run --plan … --entry {n} --project-root {clone}`, with no `--run-dir`.
The clone is `git clone --shared` of the project at HEAD `5174f92a7335b4f469cbaf94763086378016b04b`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/704607c1-6d7b-46d8-a252-21eb3dea421d/scratchpad/clone`.
Entry 2's control is typed: it differs from the entry's `run` by the record's path alone.

## Clone, unmodified (must read as the real tree read) — 2026-10-19T22:18:10Z–22:18:14Z
- 4 red · last line differs (the five current stand-in files) — as the real tree
- 5 red · last line differs (the nine current page files) — as the real tree
- 6 red · exit 0 (9 helpers) — as the real tree
- 7 red · last 27 — as the real tree
- 8 red · exit 1 (no reader) — as the real tree
- 9 red · exit 2 (fixtures/mod.rs absent) — as the real tree
- 10 red · exit 1 (fixtures/mod.rs absent) — as the real tree
- 11 green · 12 green · 13 green — as the real tree

## Clone, after-state simulated — 2026-10-19T22:18:19Z–22:18:22Z
Input: `scratchpad/simulate_after.py` over the clone. It writes the line-level shape plan.md predicts: the three escher
`NetProvider for` lines, the seven shared page definitions and the nine helper definitions removed; the 20 fixture
`Harness::from_vdom` lines removed; `mod fixtures;` added to the 19 readers; and a minimal `fixtures/mod.rs` holding
one `Harness::from_vdom`, one `impl NetProvider for HeldNet` and no print or env site.
- 4 green · 5 green · 6 green (exit 1, last 0) · 7 green (last 7) · 8 green (last 19) · 9 green (exit 1, last 0) ·
  10 green (last 4) · 11 green · 12 green (exit 1, last 0) · 13 green (exit 1, last 0) — the must-pass side

## Clone, must-fail inputs planted on the after-state — 2026-10-19T22:18:30Z–22:18:32Z
Input: `fn leak() { println!("x"); let _ = env!("HOME"); }` appended to the clone's `fixtures/mod.rs`, a newline appended to
the clone's `Cargo.toml`, and an untracked `tests/blitz-tests/tests/extra.rs` minted.
- 9 red (exit 0) · 10 red (last 5) · 11 red (an extra env! read) · 12 red (last 1, Cargo.toml) · 13 red (last 1,
  extra.rs)

## Entry 2, typed on the real tree — 2026-10-19T22:22:59Z
`cargo test -p blitz-tests --locked -- --list 2>/dev/null` written to `scratchpad/list-record.txt` (874 lines); the same
command `| cmp -` against it: exit 0 (must pass). Against a copy with line 5 altered: exit 1 (must fail).
