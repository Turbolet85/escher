# P5 controls — 2026-11-22-epoch-2-corrective (check 4 (9))

Each control fired AS THE ENTRY through `gate.py run --chunk 2026-11-22-epoch-2-corrective --docs-root . --entry {n}
--project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-escher/80cd3786-c861-47b2-b22d-5a6f89c1fcff/scratchpad/ctl`,
the clone a `git clone --shared` of the project at HEAD `40ec4749e74a0a3fb3b355f79aec0323ec86fbf6`. Inputs live in the
clone alone; the pass input is written by `$TMPDIR/…/scratchpad/ctl_pass.py`:
- `command.rs` cut at its test module, with the `#[path]` declaration;
- the module body de-indented into `command_tests.rs`, opened by one `//!` line;
- the three blitz-dom files set to upstream's text at `547d7514b`.

| entry | clone unmodified (must equal the tree) | must-pass input | must-fail input |
|---|---|---|---|
| 3 `tests_moved_whole.py` | red — `command_tests.rs absent` (as the tree) | green — `tests moved whole: PASS` | red — line 10 of `command_tests.rs` with one trailing space |
| 4 preservation guard | red — last line `''` (as the tree) | green — the five touchpoints, last line equal | red — the same plus `packages/escher-driver/src/lib.rs` (a comment appended) |
| 5 added-lines census | green — `0` on an empty diff (as the tree) | green — `0` over the pass input's non-empty diff | red — `1`, exit 0: `fn _control() { None::<u8>.unwrap(); }` appended to `events/keyboard.rs` |
| 13 `fn_scores.py` | red — 50 · 58 · 176 (as the tree) | green — `fn scores: PASS` on upstream's text (19 · 46 · 167, no fork-own function) | the tree itself: red, 50 · 58 · 176 |
| 14 `bars.py` | red — `ours over 800 outside the seven left for a later entry: 1` (command.rs 848), `bars: FAIL` | green — `bars: PASS` on the split | the tree itself: red, as unmodified |

Gate logs: `$TMPDIR/andromeda-gate/2026-11-22-epoch-2-corrective/run-20261122T1708{30,31,48,49,50,58,59}Z` and
`run-20261122T1709{11,34}Z`.
