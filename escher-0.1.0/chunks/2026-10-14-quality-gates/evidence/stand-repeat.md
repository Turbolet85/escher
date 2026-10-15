# The `stand-repeat` leg and its hand control — step 8

The leg: three rounds, each `cargo test -p blitz-tests --locked` with one `--test` per
`tests/blitz-tests/tests/stand_*.rs` (40 files, the set `scripts/agent-run.sh run stand` takes) then `cargo test
-p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test host_timer
--test mcp_server --test mcp_log`; the first red command ends the leg with its status, nothing is retried.

| reading | exit | what the log reads |
|---|---|---|
| 1. first firing — gate entry 11, implement's first block pass, 2026-10-15 | 0 · 252.0 s | `stand-repeat: round 1 of 3` … `round 3 of 3`, last line `stand-repeat: 3 of 3 rounds green`; 141 result lines, every one `ok`, 0 failed |
| 2. hand control — one `#[test] fn hand_control_reads_red() { assert!(false, …) }` appended to `tests/blitz-tests/tests/stand_counter.rs` | 101 | `stand-repeat: round 1 of 3`, `test hand_control_reads_red ... FAILED`, `test result: FAILED. 1 passed; 1 failed`, `error: test failed, to rerun pass -p blitz-tests --test stand_counter`; no `round 2` line, no seven_guis command run |
| 3. green again, after the restore — gate entry 11 (`--only 11`) | 0 · 230.4 s | last line `stand-repeat: 3 of 3 rounds green` |

The restore (testing rule, Session Addition 2026-10-09): the file's bytes copied aside before the edit and written
back after the red run; sha256 before the edit `1ba268131ba5ed3a74da810b7c9e02372fd7f4221de0e1c96c7dce4960cfeca8`,
during it `64507011c850743a8f2f1da8bed1cc12b39664591011cc66570edacc67ac8ce9`, after the restore
`1ba268131ba5ed3a74da810b7c9e02372fd7f4221de0e1c96c7dce4960cfeca8` — equal to the saved copy's, checked before
reading 3 was taken; `git diff --stat` on the file empty.
