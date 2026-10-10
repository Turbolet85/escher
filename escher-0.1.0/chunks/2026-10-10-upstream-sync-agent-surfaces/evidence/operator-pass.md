# Operator pass — 2026-10-10-upstream-sync-agent-surfaces

The plan's four `leg = 'operator'` entries (35 · 36 · 37 · 38), each driven by hand in plan order, on
the word given in the implement session, 2026-10-10 (`inputs#I2`, verbatim): "Run the operator pass
for 2026-10-10-upstream-sync-agent-surfaces now, each entry by hand in plan order: entry 35 hygiene;
then the pre-CI commit, which IS the merge commit (MERGE_HEAD is set: subject and body as in the plan
implementation notes; scrolling.rs, the new standing check and the evidence go into it; no stash,
reset, merge --abort or checkout before it); entry 36 the fast leg with the clean-tree guard and the
push; entry 37 ci.py conclusion --sha HEAD --wait 3600; entry 38 the job-log count of our named
targets into evidence/ci.md. Push nothing on top until the CI run settles. Record it in
evidence/operator-pass.md. Stop and tell me if any entry is red. Do not start the wrap." Every
command is spelled as the gate tool prints it: the repository root as `.`, the home directory as `~`.

## Entry 35 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-10T01:33:03Z, bare, from the repository root, on the uncommitted merge the
  pre-CI commit then staged (`MERGE_HEAD` = `7832c177ff272128154bac58afe56c2b9164b417`)
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.13 · 3718c868
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 46 (runs 36 · evidence 6 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-10T01:33:14Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 47 (runs 36 · evidence 7 · inputs 4) · trails 15 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.

## The pre-CI commit, between entries 35 and 36 — the merge commit

- **commit:** `9462a7e47923ee512ef71a8b0eb86613315aa69d` on `build/escher-0.1.0`, made at
  2026-10-10T01:33:28Z with `MERGE_HEAD` set, so it is the merge: two parents, first
  `09f479b8b2e61d997a84f115ec8c8802c47769fb` (the chunk start), second
  `7832c177ff272128154bac58afe56c2b9164b417` (the pin). Subject
  `chore(2026-10-10-upstream-sync-agent-surfaces): operator pre-CI commit, for the run this chunk's verdict reads`;
  body `Merge upstream/main 7832c177ff272128154bac58afe56c2b9164b417 (61 commits)`.
- **carried:** 128 files against the first parent (`git diff --name-only HEAD^1 HEAD`), `git add -A`
  over the tree entry 35 read plus this file's entry-35 record: the merge's own files, the four
  resolved conflicts, `packages/blitz-dom/src/scrolling.rs`,
  `.github/scripts/test_blitz_tests_targets.py`, the chunk folder with its seven evidence files, and
  the two run dirs.
- No `stash`, `reset`, `merge --abort` or `checkout` ran between the staged merge and this commit.
  Git recorded the four conflict resolutions (`Recorded resolution for …`, four lines). The tree
  read clean after it (`git status --short`: 0 rows).

## Entry 36 — the local pre-push gate, the clean-tree guard, the push

- **run:** `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- **fired:** 2026-10-10T01:33:34Z, ended 2026-10-10T01:35:05Z, from the repository root, on the
  committed tree, in the background. Fired once. Nothing was written to the tree between the commit
  and its end.
- **exit:** 0 — every link of the chain held: the fast leg (fmt, clippy, workspace tests, CI
  scripts), no unstaged change, no staged change, the push
- **atoms:** the entry states none; the default `exit 0` — held
- **verdict:** green
- **the fast leg's workspace count,** read from `target/ci-logs/test.log` written by this firing
  (mtime 2026-10-10T01:34:57Z): 158 result lines, 719 passed, 0 failed, 10 ignored; the CI scripts
  leg ran 78 tests, `OK` (`target/ci-logs/ci-scripts.log`, mtime 2026-10-10T01:35:03Z) — the
  figures the implement gate block read on the uncommitted merge.
- **the push, as git printed it:**

```
To https://github.com/Turbolet85/escher.git
   09f479b8..9462a7e4  build/escher-0.1.0 -> build/escher-0.1.0
```

- **read back:** `git ls-remote origin build/escher-0.1.0` answers
  `9462a7e47923ee512ef71a8b0eb86613315aa69d`. The push was not refused.

## Entry 37 — the CI run on the pushed merge commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 3600`
- **fired:** 2026-10-10T01:35:16Z, ended 2026-10-10T01:53:51Z, from the repository root, in the
  background (its bound is past the foreground ceiling), `HEAD` =
  `9462a7e47923ee512ef71a8b0eb86613315aa69d` (the work tree's one uncommitted change was this
  file). Fired once. Nothing was pushed on top of the merge commit while it ran, or since.
- **exit:** 0
- **atoms:** `exit 0` — held · `contains verdict: green` — held
- **verdict:** green
- **output, whole:**

```
ci v1.2 · 881cd498
repo Turbolet85/escher (the push remote `origin`) · polled 37× over 1115 s
9462a7e47923 verdict: green · checks 16/16 · wall 1100 s · runs CI#38013740580 completed/success
runs: CI#38013740580 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

The run the acceptance names is **CI#38013740580** on `9462a7e4…`, green 16/16 at its first attempt
(`gh api repos/Turbolet85/escher/actions/runs/38013740580`, read 2026-10-10T01:54Z: attempt 1,
`completed` / `success`, created 2026-10-10T01:35:08Z, updated 2026-10-10T01:53:32Z, 16 jobs, each
`success`). A cold run: 1100 s against 641 s on the chunk start commit.

## Entry 38 — our three named targets in the run's job logs (report-only)

- **run:** `gh run view 38013740580 -R Turbolet85/escher --log | grep -c -E 'Running tests/(stand_act_keys|stand_session_lifecycle|scroll_into_view_nested)\.rs'`
  — the entry's `<id>` substituted with the run id entry 37 printed after `CI#`
- **fired:** 2026-10-10T01:54:01Z, from the repository root. Fired once.
- **exit:** 1 — the pipe's last command, `grep -c` with no match; `gh` itself exited 0
- **atoms:** none (`expect = []`) — recorded, not counted green and not a red
- **output, whole:**

```
0
```

**The 0 is not an absence: the entry's pattern cannot match this log.** The log was saved outside
the tree (3,215,667 bytes, 23,599 lines) and read again. GNU grep with the entry's own pattern reads
the same `0`; the name `stand_act_keys` alone reads 4 lines. CI's cargo colours the word and the log
spells the colour codes as text between it and the path — `Running^[[0m tests/stand_act_keys.rs` —
on all 559 `Running` lines, and the Windows job prints `tests\stand_act_keys.rs`. The entry's
baseline was not taken on a CI log, and the entry is the plan's; the plan is not edited here.

- **the count, read a second time beside the entry** (the saved log, colour codes dropped, either
  separator): **12** — each of the three targets once in each of four jobs, `Test [default features]`,
  `Coverage report`, `Test (macos)` and `Test (windows)`, 0 failed in every one; 0 in the other
  twelve jobs. GNU grep with the colour code written into the pattern reads the same 12. The
  per-job table, with each target's result line, is in `ci.md`.
- **also read from that log:** the linux test job's tally is the local one (158 result lines, 719
  passed, 0 failed, 10 ignored); `Test CI scripts` ran 78 tests, `OK`; upstream's `all` target ran
  in no job (0 `Running` lines for `tests/all.rs`, 0 occurrences of `all_test_files_are_included`,
  the Coverage report job included); 0 lines of the install script's failure line.

## Summary of the pass

| entry | firing | fired (UTC) | exit | verdict |
|---|---|---|---|---|
| 35 hygiene | 1 (and a second read with this file in the tree) | 2026-10-10T01:33:03Z | 0 | green — `hygiene: clean` |
| the pre-CI commit | — | 2026-10-10T01:33:28Z | 0 | the merge commit `9462a7e4`, parents `09f479b8` and `7832c177` |
| 36 fast · clean tree · push | 1 | 2026-10-10T01:33:34Z | 0 | green — pushed `09f479b8..9462a7e4` |
| 37 CI conclusion | 1 | 2026-10-10T01:35:16Z | 0 | green — `verdict: green · checks 16/16` (CI#38013740580, attempt 1) |
| 38 named-target count | 1 | 2026-10-10T01:54:01Z | 1 | recorded — `0` as written, a pattern that cannot match the log; 12 on the second read (3 targets × 4 jobs) |

No entry read red. No re-run, no fix push, nothing pushed on top of the merge commit. One finding:
entry 38's pattern is blind to a CI log (colour codes after `Running`, a backslash path on Windows).
After the pass the work tree holds two uncommitted changes, this file's later sections and `ci.md`,
for the wrap's commit. The wrap was not started, on the word that opened this pass.
