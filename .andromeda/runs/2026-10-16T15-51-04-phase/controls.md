# P5 check 4 (9) controls — 2026-10-16-file-splits

Read by 2026-10-16T16:13:57Z. Every reading below is the entry's own `run`, fired through `gate.py run --entry {n}`.

## Scratch-clone controls (entries 10–19)
The scratch clone is `$TMPDIR/…/scratchpad/clone`, a `git clone --shared` of the project at
`4f3939914e525e284a32beb91bdc97972d40c904`, fired through `--project-root` with no `--run-dir`. It is reset between
cases. Each entry fired unmodified first and read what it read on the real tree.

| entry | input | reading |
|---|---|---|
| 10 | unmodified | red — last line 2 (the two inline modules) |
| 10 | must-pass: document.rs cut at line 3461, the two `#[path]` declarations appended | green — exit 1, last line 0 |
| 11 | unmodified | green — exit 0 |
| 11 | must-fail: document.rs line 100 edited | red — exit 1 |
| 11 | must-pass: document.rs cut at line 3461, the two `#[path]` declarations appended | green — exit 0 |
| 12 | unmodified | red — exit 1, `mcp.rs 1140` |
| 12 | must-pass: the eight files present, mcp.rs and schema.rs at their production lines (1–705, 1–622) | green — exit 0 |
| 13 | unmodified | green — exit 1, last line 0 |
| 13 | must-fail: tests/blitz-tests/tests/stand_keyboard.rs edited | red — exit 0 |
| 13 | must-pass: packages/escher-driver/src/mcp.rs edited | green — exit 1, last line 0 |
| 14 | unmodified | green — exit 1, last line 0 |
| 14 | must-fail: a new packages/escher-driver/src/mcp/server.rs | red — exit 0 |
| 14 | must-pass: a new packages/escher-driver/src/mcp/framing.rs | green — exit 1, last line 0 |
| 15 | unmodified | red — exit 2, the split files absent |
| 15 | must-pass: the six production files present and clean | green — exit 1, last line 0 (after the `env!("CARGO_PKG_VERSION")` exclusion; before it the same input read red on mcp.rs:339) |
| 15 | must-fail: the same with a `.unwrap()` line in mcp/result.rs | red — exit 0 |
| 16 | unmodified | red — exit 2, the split files absent |
| 16 | must-pass: the six production files present and clean | green — exit 1, last line 0 |
| 16 | must-fail: the same with a `serde_json::to_string` line in mcp/result.rs | red — exit 0 |
| 17 | unmodified | red — exit 2, the split files absent |
| 17 | must-pass: the three mcp submodules with a `pub(super) fn` in framing.rs | green — exit 1, last line 0 |
| 17 | must-fail: the three mcp submodules with a `pub fn` in framing.rs | red — exit 0 |
| 18 | unmodified | green — last line 2 |
| 18 | must-fail: the tool_call site given a third field | red — last line 1 |
| 19 | unmodified | green — last line 2 |
| 19 | must-fail: a third `info_span!` line in mcp.rs | red — last line 3 |

## Typed controls (entries 3, 4, 6, 7)
These are the entry's own command with only its record operand changed to a scratch file under
`$TMPDIR/…/scratchpad/typed/`. They ran in the real tree at `4f3939914e52` under `/usr/bin/bash -o pipefail -c`. The
records are absent until step 1, so a clone pair cannot hold them.

| entry | input | reading |
|---|---|---|
| 3 | must-pass: the `--list` output recorded from the same command (75 lines) | exit 0 |
| 3 | must-fail: that record with its first line edited | exit 1 |
| 4 | must-pass: the `--list` output recorded from the same command (99 lines) | exit 0 |
| 4 | must-fail: that record with its first line edited | exit 1 |
| 6 | must-pass: the `help` stdout recorded (17,576 B) | exit 0 |
| 6 | must-fail: that record with its first byte edited | exit 1 |
| 7 | must-pass: the `mcp` stdout over the seven requests step 1 names (6 reply lines; two runs byte-identical) | exit 0 |
| 7 | must-fail: that record with line 2's first byte edited | exit 1 |
