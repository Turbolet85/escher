# Operator pass — 2026-10-19-session-recording

Fired by hand in plan order on the operator's direction (2026-10-19): «Run the operator pass … each entry by hand in
plan order … Stop and tell me if any entry is red; start no wrap.»

**Verdict: RED at entry 17.** CI#39199277925 failed `Test (windows)`. The pass stopped there and no wrap was started.

## Entry 15 — hygiene (fired as written, before the pre-CI commit)
```
gate v1.20 · 445f3409
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 113 (runs 108 · evidence 0 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
green — `contains hygiene: clean` holds.

## The pre-CI commit
`9da7b75dd chore(2026-10-19-session-recording): operator pre-CI commit, for the run this chunk's verdict reads`. Its
parent is `93336e886`. The tree was clean after the commit (`git status --short` read 0 rows).

## Entry 16 — `gate.py run --plan … --operator 16` (backgrounded)
```
operator entry 16 · history tripwire: git
 16 probe       green · exit 0 · 139.23s · 106844 B → 16.log · history moved: refs/remotes/origin/build/escher-0.2.0 93336e88→9da7b75d · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
not run — excluded · 16: 1-15,17
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```
green. The history move is the intended push of `9da7b75d` to `origin/build/escher-0.2.0`.

## Entry 17 — `ci.py conclusion --sha HEAD --wait 1800` (fired as written, backgrounded)
```
ci v1.3 · 2b582a9b
repo Turbolet85/escher (the push remote `origin`) · polled 31× over 929 s
9da7b75dd4ec verdict: red · checks 17/17 · first-fail +916 s Test (windows) · wall 916 s · runs CI#39199277925 complete…
runs: CI#39199277925 push completed/failure
failed 1: Test (windows) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/escher/actions/jobs/117889991186/logs
run failed: CI#39199277925 failure
```
**red** — `contains verdict: green` fails.

### The failing test, read from the job log (job 117889991186)
```
test a_recording_mcp_role_logs_nothing_of_its_checks_at_trace ... FAILED
thread 'a_recording_mcp_role_logs_nothing_of_its_checks_at_trace' (2524) panicked at examples\seven_guis\tests\mcp_log.rs:145:36:
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.33s
```
- **Cause, as read:** recording is unix-only by this chunk's design. `Session::recording_into` and
  `recording::prepare_dir` return `SessionError::Unsupported` off unix, so `escher-session mcp --record <dir>` ends
  with exit 3 before it answers.
- The new `mcp_log` test `a_recording_mcp_role_logs_nothing_of_its_checks_at_trace` is not `cfg(unix)`-gated. The
  file is gated only `not(target_arch = "wasm32")`.
- So on the Windows runner the test waits for a reply that never comes. It panics at `mcp_log.rs:145`, which is the
  snapshot reply's `string_at(...).expect`.
- This chunk's red, introduced by step 13's new test. The host's Linux gates cannot reproduce it, because there the
  recording works.
- **Not fixed: the operator's direction was to stop.** A fix would gate the recorded `mcp_log` test and its
  `recorded_needles` helper `cfg(unix)`, as `host_log` and `recording.rs` already are. It would land as a
  `fix(2026-10-19-session-recording): operator fix after CI run 39199277925, …` commit, re-pushed through entry 16 and
  re-read through entry 17.
- Cargo stops at the first failing target on that runner, so any seven_guis target after `mcp_log` went unrun on
  Windows in this run.
