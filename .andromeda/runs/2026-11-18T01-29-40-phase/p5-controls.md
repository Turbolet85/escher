# P5 controls — 2026-11-18-a11y-re-read

Clone: `git clone --shared` of the project, checked out at HEAD `712e09b3cba822f939837e3c064d8f2163a7cffb`, at `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/6010da18-c760-40b0-82bc-2f383a1dabc2/scratchpad/ctl-a11y`. Each entry was fired as itself through `gate.py run --chunk 2026-11-18-a11y-re-read --docs-root . --project-root {the clone} --entry {n}`.

## Entry 5 — the preservation guard
- Unmodified clone: `5 probe green · exit 0`. This matches the real tree's reading (`guard: clean`, allowed 0).
- Input it must pass — one line appended to `.github/scripts/ci-leg.sh`, a touchpoint: `5 probe green · exit 0`.
- Input it must fail on — one line also appended to `tests/blitz-tests/tests/stand_contrast.rs`, outside the touchpoints: `5 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files')`.

### Re-run after the review's fold (plan rev 4, the guard's allow list gaining `stand_keyboard.rs`, now entry 6)
- Real tree: `6 probe green · exit 0` (`guard: clean`, allowed 0).
- Input it must pass — one line appended to `tests/blitz-tests/tests/stand_keyboard.rs`, the new touchpoint: `6 probe green · exit 0`.
- Input it must fail on — that edit plus one line appended to `tests/blitz-tests/tests/stand_contrast.rs`: `6 probe red · last line 'guard: clean' ✗ (last: 'guard: outside 1 files')`.

## Entry 3 — the a11y leg's result-line count (entry 4 after the fold)
- Real tree (baseline): `3 probe red · last line '18' ✗ (last: '14')`, the log reading `115 passed 0 failed` then `14`. That is the count this chunk moves.
- Must-pass — `target/ci-logs/a11y.log` in the clone written with 18 `test result: ok. 3 passed; 0 failed; …` lines: `3 probe green · exit 0`.
- The clone's edits were reverted (`git checkout -- .`) between the guard and the count controls. The log sits under the clone's ignored `target/`.
