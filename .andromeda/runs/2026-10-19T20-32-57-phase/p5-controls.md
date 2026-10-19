# P5 controls — 2026-10-19-test-splits

Scratch clone: `$TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/bcde2409-ab7b-4565-9533-c5f3e712a1f8/scratchpad/clone`,
`git clone --shared` of the project at HEAD `f98a4dce1fc1bcc57019947812802e63ccc8fe6c`. Each entry was fired through
`gate.py run --entry {n} --project-root {clone}` with no run dir.

## Unmodified clone (must read what the real tree read)
- 4 red, exit 0 · 5 red, exit 1 · 6 green · 7 red, 12225 B · 10 red · 13 red, last line 6 · 15 green, exit 1 ·
  17 green, exit 1 · 18 green, exit 1. These are the same readings as the real tree's baseline.

## Must-pass — the plan's steps 2-3 applied by `move.py`
`move.py` cuts `cli.rs` 478-1031, `recording.rs` 455-466 and 471-980, dedents one level and writes the three
`#[path]` declarations. rustfmt (edition 2024) was then run over `cli.rs` and `recording.rs`, which reformats the
children.
- 4 green, exit 1, last line 0 · 5 green, last line 3 · 6 green · 7 green, no output · 8 green, no output · 9 green,
  no output · 10 green (`cli.rs 478`, `recording.rs 459`) · 15 green, exit 1, last line 0 · 17 green, exit 1, last
  line 0 · 18 green, exit 1, last line 0.
- 13 green, last line 0, with `examples/seven_guis/tests/mcp_log.rs` replaced by a two-line stand-in. This is an input
  with no cross-file clone, and it shows the reader can print 0.

## Must-fail — on top of the moved clone, each input restored after its reading
- 7 red (exit 1, 24305 B) — one `"flight-booker"` literal in `cli_tests.rs` edited.
- 6 red (exit 1) — a trailing space added to `cli.rs` line 100.
- 15 red (exit 0, last line 1) — a `println!` function appended to `recording_tests.rs`.
- 17 red (exit 0, last line 1) — a blank line appended to `Cargo.toml`.
- 18 red (exit 0, last line 1) — `packages/escher-driver/src/extra.rs` minted.

## Typed controls for entries 2 and 12
These were run on the real tree. The command is the entry's own `run` with only the record operand changed, fired as
`/usr/bin/bash -o pipefail -c '…'`.
- 2: the list was written to `$TMPDIR/…/scratchpad/drv.txt`, then compared. Must-pass exit 0. With line 1 of the
  record edited, must-fail exit 1.
- 12: the list was written to `$TMPDIR/…/scratchpad/log.txt` (4 tests), then compared. Must-pass exit 0. With line
  1 edited, must-fail exit 1.
