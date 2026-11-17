# P5 check 4 (9) controls — 2026-11-17-answer-writer-by-family

Clone: `git clone --shared` of the project at HEAD `d875c3d22023bd57dcb3baf880b3d25c81b804f9`, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/b0420191-21b4-4abe-b45e-76a801bb0e48/scratchpad/ctl-clone`.
Each firing: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --chunk 2026-11-17-answer-writer-by-family --docs-root . --project-root {clone} --only {entries}`, no `--run-dir`.

## 1. Unmodified clone (entries 2–7) — reads what the tree read
- 2 red · exit 0 ✗ (exit 1) — `json_tests.rs` absent
- 3 red · exit 0 ✗ (exit 2) — absent
- 4 green · exit 1 · 0 B
- 5 red · exit 1 ✗ (exit 2) — the new files absent
- 6 green · exit 0 · last line 2
- 7 green · exit 0
(logs `$TMPDIR/andromeda-gate/2026-11-17-answer-writer-by-family/run-20261117T175918Z`)

## 2. Must-pass input (entries 2–7)
Input: `packages/escher-driver/src/json_tests.rs` built as step 5 builds it — `sed -n '842,1898p' json.rs | sed 's/^    //'`, the tests' body at column 0 — and a silent child `json/snapshot.rs` (`//!` line and one `pub(super) fn`), both staged.
- 2 green · exit 0 · last line 49
- 3 green · exit 0 · last line 9
- 4 green · exit 1 · 0 B
- 5 green · exit 1 · 0 B
- 6 green · last line 2
- 7 green · exit 0
(logs `…/run-20261117T175929Z`)

## 3. Must-fail input (entries 4–7)
Input, on top of 2: a staged child `json/lint.rs` holding `println!("x");`; a staged `tracing::info_span!("control")` appended to `packages/escher-driver/src/host.rs`; one line appended to `Cargo.lock`.
- 4 red · exit 1 ✗ (exit 0) · 42 B — the census finds the planted print
- 5 red · exit 1 ✗ (exit 0) · 85 B — the same over the working tree
- 6 red · last line '2' ✗ (last: '3') — the third span counted
- 7 red · exit 0 ✗ (exit 1) — the lock moved
(logs `…/run-20261117T175940Z`)

Entries 2 and 3 assert literals copied from `json.rs`'s raw text and read over the file the plan builds; the must-pass reading over that file reads them green, so the literals are the run's own reading.
