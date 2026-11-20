# P5 controls — 2026-11-20-browser-s-surviving-mutants

Clone: `git clone --shared` of the project at HEAD `80c31730e59b9e9c344d24fd4d0b4bb643506ac2`, at `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/14d525e9-31bd-4d03-adf8-a1cd1c40ea08/scratchpad/ctl`. Each entry was fired as itself (`gate.py run --chunk 2026-11-20-browser-s-surviving-mutants --docs-root . --entry {n} --project-root {clone}`) by the script `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/14d525e9-31bd-4d03-adf8-a1cd1c40ea08/scratchpad/controls.sh`, the clone reset between inputs.

| entry | input | reading |
|---|---|---|
| 3 | unmodified clone | green · last line 0 (as on the tree) |
| 4 | unmodified clone | green · last line 0 (as on the tree) |
| 5 | unmodified clone | green · guard: clean (as on the tree) |
| 3 | must-pass: a `#[cfg(test)] mod tests` appended after line 122 of capture.rs | green · last line 0 |
| 4 | must-pass: the same | green · last line 0 |
| 5 | must-pass: the same | green · guard: clean |
| 3 | must-fail: capture.rs line 3 deleted | red · last line 1 |
| 4 | must-fail: a line inserted at capture.rs line 50 | red · last line 1 |
| 5 | must-fail: README.md changed beside capture.rs | red · last line `guard: outside 1 files` |

## Rev 2 — after the operator's review (inputs#I2), plan rev 4

Entries 3, 4 and 5 were widened to the out-of-line shape and re-controlled in the same clone (HEAD `80c31730e59b9e9c344d24fd4d0b4bb643506ac2`) by the same script, rewritten. "The two files" means the declaration `#[cfg(test)]` · `#[path = "capture_tests.rs"]` · `mod tests;` appended to capture.rs, and `capture_tests.rs` created.

| entry | input | reading |
|---|---|---|
| 3 | unmodified clone | red · exit 1 — `capture_tests.rs` absent (as on the tree: the subject) |
| 4 | unmodified clone | green · last line 0 (as on the tree) |
| 5 | unmodified clone | green · guard: clean (as on the tree) |
| 3 | must-pass: the two files | green · last line 0 |
| 4 | must-pass: the two files | green · last line 0 |
| 5 | must-pass: the two files | green · guard: clean |
| 3 | must-fail: the declaration, no `capture_tests.rs` | red · exit 1 |
| 3 | must-fail: the two files, capture.rs line 3 deleted | red · last line 1 |
| 4 | must-fail: the two files, a line inserted at capture.rs line 50 | red · last line 2 |
| 4 | must-fail: the two files, a test function appended to capture.rs after the declaration (the inline form) | red · last line 2 |
| 5 | must-fail: the two files, README.md changed | red · `guard: outside 1 files` |
| 5 | must-fail: the two files, `apps/browser/src/capture_extra.rs` added | red · `guard: outside 1 files` |
