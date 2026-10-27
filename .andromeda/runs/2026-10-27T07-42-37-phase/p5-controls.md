# P5 controls — 2026-10-27-upstream-sync-engine-debts

Fired as the entries themselves, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py run --plan {plan} --entry {n} --project-root $TMPDIR/{scratchpad}/p5-clone`, no `--run-dir`, over a `git clone --shared` of the project (validation.md check 4 (9)), 2026-10-27T09:45Z–09:47Z. The cargo entries reused `CARGO_TARGET_DIR=target/tmp/trial-target`. Each input below was left uncommitted where the entry reads the work tree, and reverted (`git checkout --`) before the next; the clone read 0 changed paths after the last.

## Unmodified — clone HEAD `de85792f5` (the base), each matching the real tree's baseline
- 1 red · exit 1 · 73039 B (the eleven commits' lines against an empty left side)
- 2 red · exit 0 · last line 23 (the 23 files absent)
- 3 red · last line `+0 -494` (`runtime/events.rs` absent)
- 4 red · exit 2 (`runtime/events.rs` absent)
- 5 green · exit 0 · 6 green · exit 1, last line 0 · 8 green · exit 1, last line 0 · 9 green · exit 0, last line 2
- 18 red · last line 147

## Must-pass — clone HEAD `02b24fccf` (the trial take, shape A, research.md §The trial takes)
- 1 green · exit 0 · no output · 2 green · exit 1, last line 0 · 3 green · last line `+2 -2` · 4 green · last line 1
- 5 green · exit 0 · 6 green · exit 1, last line 0 · 8 green · exit 1, last line 0 · 9 green · last line 2
- 10 green · `test result: ok. 3 passed; 0 failed` · 11 green · `test result: ok. 30 passed; 0 failed` · 12 green · `test result: ok. 82 passed; 0 failed`
- 18 green · last line 148

## Must-fail — clone HEAD `02b24fccf`, one input each
- 1 red · exit 1 · input: our range re-sanitize call in `packages/blitz-dom/src/mutator.rs` replaced by a comment
- 2 red · exit 0, last line 1 · input: a line appended to `packages/blitz-vibey-script/src/runtime/css.rs`
- 4 red · exit 1, last line 0 · input: upstream's comment restored in `packages/blitz-vibey-script/src/runtime/events.rs`
- 5 red · exit 1 · input: a line appended to `.github/scripts/ci-leg.sh`
- 6 red · exit 0, last line 1 · input: a git dependency line appended to `Cargo.toml`
- 8 red · exit 0, last line 1 · input: an `Instant::now()` planted in `packages/blitz-dom/src/util.rs`
- 9 red · last line 1 · input: one `--ignore-filename-regex 'keyboard_compat\.rs$'` dropped from `.github/scripts/ci-leg.sh`
