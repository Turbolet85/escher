# The flakiness budget's measurement — 20 local repeats (step 9)

Host: the dev host, 2026-10-15, the working tree of this chunk (HEAD `eb5ba90c` plus its edits). `bash
scripts/agent-run.sh boot` once (05:45:27Z, `ready`, cargo 0), then 20 rounds in sequence, each:

- `bash scripts/agent-run.sh run stand` — every `tests/blitz-tests/tests/stand_*.rs` (40 files) in one `cargo test
  -p blitz-tests --locked`; its figures are the round's `run.end` event read from `target/agent-run/events.jsonl`
  (passed · failed · ignored);
- `cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test
  host_timer --test mcp_server --test mcp_log`, stderr merged into stdout, its 7 `test result:` lines summed
  (passed · failed · ignored).

A round reads `passed` when both commands exit 0 and neither reports a failed test. The runner stops at the first
round that does not; it ran all 20. Names of tests only would be listed for a failure — there is none. No
captured output is recorded here.

| round | verdict | blitz-tests stand (passed · failed · ignored) | seven_guis process checks (passed · failed · ignored) | `run.end` | round wall (s) |
|---|---|---|---|---|---|
| 1 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:46:21Z · `passed` · cargo 0 | 62.5 |
| 2 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:47:53Z · `passed` · cargo 0 | 91.1 |
| 3 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:49:18Z · `passed` · cargo 0 | 85.5 |
| 4 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:50:51Z · `passed` · cargo 0 | 92.4 |
| 5 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:52:17Z · `passed` · cargo 0 | 86.1 |
| 6 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:53:43Z · `passed` · cargo 0 | 85.8 |
| 7 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:55:09Z · `passed` · cargo 0 | 86.7 |
| 8 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:56:29Z · `passed` · cargo 0 | 79.2 |
| 9 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:57:50Z · `passed` · cargo 0 | 81.0 |
| 10 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T05:59:24Z · `passed` · cargo 0 | 94.9 |
| 11 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:00:45Z · `passed` · cargo 0 | 79.8 |
| 12 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:02:04Z · `passed` · cargo 0 | 79.9 |
| 13 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:03:29Z · `passed` · cargo 0 | 85.0 |
| 14 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:05:00Z · `passed` · cargo 0 | 91.3 |
| 15 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:06:20Z · `passed` · cargo 0 | 79.1 |
| 16 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:07:41Z · `passed` · cargo 0 | 81.3 |
| 17 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:09:02Z · `passed` · cargo 0 | 81.7 |
| 18 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:10:25Z · `passed` · cargo 0 | 82.9 |
| 19 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:11:55Z · `passed` · cargo 0 | 90.1 |
| 20 | passed | 168 · 0 · 7 | 24 · 0 · 0 | 2026-10-15T06:13:21Z · `passed` · cargo 0 | 86.0 |

## Reading

20 of 20 rounds passed: 0 failures in 20 × (168 + 24) = 3 840 test runs, 0 flakes against a budget of zero. No
test failed in any round, so no failing test is named. The same 7 stand tests
read ignored in every round (`#[ignore]` by declaration, never by a failure). The budget CI enforces is the `stand-repeat` leg
(three rounds of the same two commands, the first red ending the job; `evidence/stand-repeat.md`).
