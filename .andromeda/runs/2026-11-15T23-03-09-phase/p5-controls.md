# P5 readings — 2026-11-15-session-binary-smoke

Tree: HEAD 4263cec1f77b96da941db4e829cded1c91252633, untouched (no chunk edit yet). Logs under
`$TMPDIR/andromeda-gate/2026-11-15-session-binary-smoke/phase-2026-11-15T23-03-09/` ($TMPDIR = /tmp).

## Baselines (gate.py run --entry n, this run dir's trail)
- entry 1 — red · exit 0 · `Ran 14 tests` · `OK` · the six new case names absent (the cases this chunk writes). Line
  form read in the log: `test_status_before_boot_exits_3 (test_agent_run.AgentRunTest.test_status_before_boot_exits_3) ... ok`.
- entry 2 — green · exit 0 · `Ran 112 tests` · `OK` (10.9 s).
- entry 3 — green · exit 0 · `{"event": "boot", "ts": "2026-11-15T23:33:31Z", "outcome": "ready", "cargo_exit": 0, "head": "4263cec1f77b96da941db4e829cded1c91252633"}` (72.5 s).
- entry 4 — red · exit 2 · the usage text `run selections: stand · all · <blitz-tests file name>`: `session` is no selection
  yet (the subject this chunk builds).
- entry 5 — red · exit 0 · `"run": {"selection": null, "state": "none", "passed": 0, "failed": 0, "ignored": 0}`
  (a recency selector, red on the untouched tree as required).

## Entry 4's literals re-spelt from the run's own output (check 4 (9))
`bash scripts/agent-run.sh run text_extent` on the same tree, exit 0, printed:
- `{"event": "run.start", "ts": "2026-11-15T23:33:45Z", "selection": "text_extent", "files": ["text_extent"]}`
- `{"event": "test", "file": "text_extent", "test": "right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box", "outcome": "ok"}`
- `{"event": "run.end", "ts": "2026-11-15T23:33:46Z", "selection": "text_extent", "passed": 6, "failed": 0, "ignored": 0, "cargo_exit": 0, "outcome": "passed"}`
Entry 4's three atoms are these forms with `session` · `cli_flow` · `each_flow_completes_under_plain_sh_and_leaves_nothing`
· `2` substituted; the test name and the count from `cargo test -p seven_guis --locked --test cli_flow 2>&1` read at
P3 (two `test … ... ok` lines, `Running tests/cli_flow.rs (target/debug/deps/cli_flow-50cfa2445ed124aa)`).

## CI re-read (Setup 5a's sha)
`ci.py conclusion --sha 4263cec1f77b…`: `verdict: green · checks 17/17 · wall 1798 s · CI#43771902054 completed/success`.

## Check 11 (b)
`doc:2026-10-10-driver-cli/report` and its 13 evidence files searched for `kill|pkill|signal|orphan|leak|left running|
stray|harm|survivor|reap` (report) and `pkill|killall|kill -|left running|orphan|stray host` (evidence): no recorded harm
to the host; the report's one hit names `start` spawning the binary with streams closed.
