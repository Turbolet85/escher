# P5 controls — 2026-10-20-select-by-id

Each guard entry fired as the entry (`gate.py run --plan … --entry {n} --project-root {clone}`, no `--run-dir`) over a
scratch clone `git clone --shared` of the project at HEAD 2f0997354972, at `$TMPDIR/{scratchpad}/clone`, re-made per
entry; the input of each control lived in the clone alone. Script: `$TMPDIR/{scratchpad}/controls.sh`. Each line as
the tool printed it.

- entry 7 · unmodified · green · exit 1, last line 0 (as on the real tree)
- entry 7 · must-fail: `Instant::now` planted in blitz-dom `document.rs` · red · exit 0
- entry 8 · unmodified · red · exit 2 (select.rs absent, as on the real tree)
- entry 8 · must-pass: a silent `select.rs` minted · green · exit 1, last line 0
- entry 8 · must-fail: an `unwrap()` added to it · red · exit 0
- entry 9 · unmodified · red · exit 2 (both checks absent, as on the real tree)
- entry 9 · must-pass: both checks minted silent · green · exit 1, last line 0
- entry 9 · must-fail: a `println!` in `select_interaction.rs` · red · exit 0
- entry 10 · unmodified · green · last line 2
- entry 10 · must-fail: a third `info_span!` in escher-driver `lib.rs` · red · last line 3
- entry 11 · unmodified · green · last line 0
- entry 11 · must-pass: blitz-dom `document.rs` edited · green · last line 0
- entry 11 · must-fail: `tests/blitz-tests/tests/fixtures/mod.rs` edited · red · last line 1
- entry 12 · unmodified · green · exit 1, last line 0
- entry 12 · must-pass: blitz-dom `document.rs` edited · green · exit 1
- entry 12 · must-fail: blitz-dom `lib.rs` edited · red · exit 0
- entry 13 · unmodified · green · exit 1, last line 0
- entry 13 · must-pass: `packages/blitz-dom/src/node/select.rs` minted · green · exit 1
- entry 13 · must-fail: `packages/blitz-dom/src/extra.rs` minted · red · exit 0

Entries 4 and 5 name targets this chunk adds; their atom `0 failed` is libtest's own summary form, read in entry 6's
output on the real tree (`test result: ok. 10 passed; 0 failed; …`) — the must-pass reading of the literal.
