# CI — fork CI on the pushed merge commit

The operator pass's readings of entries 37 and 38 (`operator-pass.md` holds each entry's whole record).

## The run
| run | sha | attempt | verdict | checks | wall | created → updated (UTC) |
|---|---|---|---|---|---|---|
| CI#38013740580 | `9462a7e47923ee512ef71a8b0eb86613315aa69d` (the merge commit) | 1 | green — `completed` / `success` | 16/16 | 1100 s | 2026-10-10T01:35:08Z → 01:53:32Z |

- Read by `ci.py conclusion --sha HEAD --wait 3600`, fired 2026-10-10T01:35:16Z, ended 01:53:51Z, exit 0: `verdict: green · checks 16/16 · wall 1100 s`, polled 37× over 1115 s. The run was left to settle: nothing was pushed on top of the merge commit.
- A cold run, as the plan expected (the lock change re-keys every cache): 1100 s against 641 s for the run on the chunk start commit (CI#38010081458).
- All 16 jobs read `success` (`gh api …/runs/38013740580/jobs`): Rustfmt · Clippy · Test [default features] · Test CI scripts · Dependency audit · Documentation · Build wasm examples · Build counter example · Build [default features] · MSRV Build [Rust 1.91] · Accessibility (a11y) tests · Coverage report · Test (windows) · Test (macos) · Test (ios) · Test (android). The longest was Test (windows), 01:41:07Z → 01:53:31Z.
- Witnessed by this run and by no local leg: the MSRV build on the new dependency set, the wasm leg, the coverage job, and the four platform legs.

## Entry 38 — our three named targets across the run's jobs

**As written, the entry reads 0, and that 0 is not an absence.** `gh run view 38013740580 -R Turbolet85/escher --log | grep -c -E 'Running tests/(stand_act_keys|stand_session_lifecycle|scroll_into_view_nested)\.rs'` → `0`, exit 1 (fired 2026-10-10T01:54:01Z; the same `0` from GNU grep over a saved copy of the log, 3,215,667 bytes, 23,599 lines, kept outside the tree). The pattern cannot match this log, for two reasons read from the log's own lines:

- CI's cargo colours the word, and the log spells the colour codes as text between `Running` and the path: `Running^[[0m tests/stand_act_keys.rs (…)`. All 559 `Running` lines of the log carry one.
- The Windows job prints the path with a backslash: `tests\stand_act_keys.rs`.

**The count, read a second time beside the entry** — the same saved log, the colour codes dropped and either separator accepted (`Running tests[/\]{name}.rs`), per job:

| job | `stand_act_keys` | `stand_session_lifecycle` | `scroll_into_view_nested` | its result line for each, in that order |
|---|---|---|---|---|
| Test [default features] (linux) | 1 | 1 | 1 | 2 passed · 1 passed, 1 ignored · 7 passed |
| Coverage report (linux) | 1 | 1 | 1 | 2 passed · 1 passed, 1 ignored · 7 passed |
| Test (macos) | 1 | 1 | 1 | 2 passed · 1 passed, 1 ignored · 7 passed |
| Test (windows) | 1 | 1 | 1 | 2 passed · 0 passed, 0 ignored · 7 passed |
| the other 12 jobs | 0 | 0 | 0 | — |

Total 12: each of the three targets ran once in each of the four jobs that run the workspace's tests, 0 failed in every one. A cross-check with GNU grep and the colour code written into the pattern reads the same 12. `stand_session_lifecycle` reads 0 passed on Windows: its checks are `cfg(unix)`-gated, so the target is built there and holds no test.

## What else the same log shows
| job | `Running tests/*.rs` lines | result lines | passed | failed | ignored |
|---|---|---|---|---|---|
| Test [default features] | 107 | 158 | 719 | 0 | 10 |
| Coverage report | 107 | 139 | 716 | 0 | 10 |
| Test (macos) | 107 | 139 | 716 | 0 | 10 |
| Test (windows) | 107 | 139 | 712 | 0 | 8 |
| Accessibility (a11y) tests | 3 | 3 | 15 | 0 | 0 |

- The linux test job reads the local figures exactly: 158 result lines, 719 passed, 0 failed, 10 ignored. `Test CI scripts` reads `Ran 78 tests`, `OK`.
- **Upstream's `all` target ran in no job.** The log holds 0 `Running` lines for `tests/all.rs` in either separator and 0 occurrences of `all_test_files_are_included` — the Coverage report job included, which is the witness the plan named for the `coverage` leg (a built `all` target would have run upstream's guard, failed on our unlisted files and reddened the job).
- The package-install watch: 0 lines of the install script's failure line `apt-install: attempt` in the run's log; no job hung in its install step.
- Not read: why the three matrix-style jobs print 139 result lines against the linux job's 158, and Windows 712 passed and 8 ignored against 716 and 10 — the differences were not attributed in this pass. The ios and android jobs print no `Running` line and no result line; their jobs read `success`, and what their `test` step ran was not read.
