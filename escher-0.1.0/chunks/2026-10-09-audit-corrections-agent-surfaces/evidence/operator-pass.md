# Operator pass — 2026-10-09-audit-corrections-agent-surfaces

The plan's four `leg = 'operator'` entries (16 · 17 · 18 · 19), each driven by hand in plan order, on
the word given in the implement session, 2026-10-09 (`inputs#I3`, verbatim): "Run the operator pass
for 2026-10-09-audit-corrections-agent-surfaces now, each entry by hand in plan order: hygiene, the
commit "chore(2026-10-09-audit-corrections-agent-surfaces): operator pre-CI commit, for the run this
chunk verdict reads", the fast leg with the clean-tree guard and the push, then ci.py conclusion
--sha HEAD --wait 1800, then the install-step durations. Record it in evidence/operator-pass.md.
Stop and tell me if any entry is red. Do NOT start the wrap after it: a seam hold is in force until
my next line." Every command is spelled as the gate tool prints it: the repository root as `.`, the
home directory as `~`.

## Entry 16 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-09T20:13:00Z, bare, from the repository root, on the uncommitted tree the
  pre-CI commit then staged
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.13 · 3718c868
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 42 (runs 36 · evidence 1 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-09T20:13:11Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 43 (runs 36 · evidence 2 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.

## The pre-CI commit, between entries 16 and 17

- **commit:** `6c545ced9b14a1892bde5310e3746c9352c6700e` on `build/escher-0.1.0`, parent
  `6d62677517b172e01764ab92b826d03ff75a7a11` — `chore(2026-10-09-audit-corrections-agent-surfaces):
  operator pre-CI commit, for the run this chunk verdict reads`; 54 files, `git add -A` over the
  tree entry 16 read plus this file's entry-16 record. `.github/scripts/apt-install.sh` is committed
  `100755` (`git ls-files -s`). The tree read clean after it (`git status --short`: 0 rows).

## Entry 17 — the local pre-push gate, the clean-tree guard, the push

- **run:** `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- **fired:** 2026-10-09T20:13:23Z, ended 2026-10-09T20:14:47Z, from the repository root, on the
  committed tree
- **exit:** 0 — every link of the chain held: the fast leg (fmt, clippy, workspace tests, CI
  scripts), no unstaged change, no staged change, the push
- **atoms:** the entry states none; the default `exit 0` — held
- **verdict:** green
- **the fast leg's workspace count,** read from `target/ci-logs/test.log` written by this firing
  (mtime 2026-10-09T20:14:39Z): 154 result lines, 657 passed, 0 failed, 10 ignored; the CI scripts
  leg ran 70 tests, `OK` (`target/ci-logs/ci-scripts.log`, mtime 2026-10-09T20:14:45Z)
- **the push, as git printed it:**

```
To https://github.com/Turbolet85/escher.git
   6d626775..6c545ced  build/escher-0.1.0 -> build/escher-0.1.0
```

- **read back:** `git ls-remote origin build/escher-0.1.0` answers
  `6c545ced9b14a1892bde5310e3746c9352c6700e`. The push was not refused.

## Entry 18 — the CI run on the pushed sha

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- **fired:** 2026-10-09T20:14:51Z, ended 2026-10-09T20:26:45Z, from the repository root, in the
  background (its bound is past the foreground ceiling), `HEAD` =
  `6c545ced9b14a1892bde5310e3746c9352c6700e` (the work tree's one uncommitted change is this
  file). Fired once.
- **exit:** 0
- **atoms:** `exit 0` — held · `contains verdict: green` — held
- **verdict:** green
- **output, whole:**

```
ci v1.2 · 881cd498
repo Turbolet85/escher (the push remote `origin`) · polled 24× over 714 s
6c545ced9b14 verdict: green · checks 16/16 · wall 695 s · runs CI#37985678276 completed/success
runs: CI#37985678276 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

The run the acceptance names is **CI#37985678276** on `6c545ced…`, green 16/16 at its first attempt
(`gh api repos/Turbolet85/escher/actions/runs/37985678276`, read 2026-10-09T20:26:55Z: attempt 1,
`completed` / `success`, created 2026-10-09T20:14:49Z, updated 2026-10-09T20:26:27Z, 16 jobs).

## Entry 19 — the install steps of that run (report-only)

- **run:** `gh api "repos/Turbolet85/escher/actions/runs/37985678276/jobs?per_page=100" --jq '.jobs[] | .name as $j | .steps[] | select(.name | test("apt-install|Install apt deps")) | [$j, .name, .started_at, .completed_at] | @tsv'`
  — the entry's `<id>` substituted with the run id entry 18 printed after `CI#`
- **fired:** 2026-10-09T20:26:55Z, from the repository root. Fired once.
- **exit:** 0
- **atoms:** none (`expect = []`) — recorded, not counted green
- **output, whole** (tab-separated: job · step · started · completed):

```
Test CI scripts	Run python3 -c 'import yaml' || bash .github/scripts/apt-install.sh python3-yaml	2026-10-09T20:14:57Z	2026-10-09T20:14:57Z
Clippy	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:15:14Z	2026-10-09T20:15:26Z
Test [default features]	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:15:22Z	2026-10-09T20:15:32Z
MSRV Build [Rust 1.91]	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:54Z	2026-10-09T20:20:06Z
Build [default features]	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:42Z	2026-10-09T20:19:55Z
Build counter example	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:41Z	2026-10-09T20:19:53Z
Documentation	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:38Z	2026-10-09T20:19:54Z
Test (android)	Install apt deps	2026-10-09T20:20:36Z	2026-10-09T20:20:41Z
Test (ios)	Install apt deps	2026-10-09T20:20:37Z	2026-10-09T20:20:37Z
Test (windows)	Install apt deps	2026-10-09T20:22:36Z	2026-10-09T20:22:36Z
Coverage report	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:32Z	2026-10-09T20:19:43Z
Test (macos)	Install apt deps	2026-10-09T20:20:31Z	2026-10-09T20:20:31Z
Accessibility (a11y) tests	Run bash .github/scripts/apt-install.sh libfontconfig1-dev	2026-10-09T20:19:44Z	2026-10-09T20:20:00Z
```

### The durations, with each step's conclusion

A second read of the same endpoint beside the entry (same minute), adding `.conclusion` and the
difference of the two stamps, which the API gives to the whole second:

| job | step | conclusion | seconds |
|---|---|---|---|
| `Test CI scripts` | the guarded script call (`python3-yaml`) | success | 0 |
| `Clippy` | script call | success | 12 |
| `Test [default features]` | script call | success | 10 |
| `MSRV Build [Rust 1.91]` | script call | success | 12 |
| `Build [default features]` | script call | success | 13 |
| `Build counter example` | script call | success | 12 |
| `Documentation` | script call | success | 16 |
| `Coverage report` | script call | success | 11 |
| `Accessibility (a11y) tests` | script call | success | 16 |
| `Test (android)` | `Install apt deps` (the action, `timeout-minutes: 10`) | success | 5 |
| `Test (ios)` · `Test (windows)` · `Test (macos)` | `Install apt deps` | skipped (the step's `if`) | 0 |

- **The eight `libfontconfig1-dev` script calls took 10 to 16 s**, update and install together,
  against the script's bound of 120 s per call; the healthy readings the plan sized the bound on
  were 12 s and 31 s. The android action step took 5 s under its 10-minute timeout.
- **No attempt failed:** the run's log (`gh run view 37985678276 -R Turbolet85/escher --log`, read
  2026-10-09T20:27:16Z, 2,335,331 bytes, kept outside the tree) holds 0 lines of the script's
  failure line `apt-install: attempt`. Each of the eight jobs' logs holds apt's own output for the
  step; so does the android job's.
- **The script ran on real runners in eight jobs, not nine.** In `Test CI scripts` the step printed
  nothing after its command header and took 0 s: `python3 -c 'import yaml'` succeeded on the runner
  image, so the `||` never reached the script. The plan's entry-18 note and its last acceptance
  speak of "nine jobs"; nine steps call the script and eight executed it in this run. The script's
  own tests ran in that job: its `ci-scripts` leg printed `Ran 70 tests`, `OK`.
- **Not shown by this run**, as the plan states: the bound firing on a real stall, and the android
  step's timeout firing — no step came near either.

### The new check on the runners (read from the same log)

`test a_textarea_reads_back_typed_text ... ok` is printed in four jobs: `Test [default features]`,
`Test (windows)`, `Test (macos)` and `Coverage report`. Not read: the ios and android job logs
beyond their install step (their jobs read `success`), and what else each log holds.

## Summary of the pass

| entry | firing | fired (UTC) | exit | verdict |
|---|---|---|---|---|
| 16 hygiene | 1 (and a second read with this file in the tree) | 2026-10-09T20:13:00Z | 0 | green — `hygiene: clean` |
| 17 fast · clean tree · push | 1 | 2026-10-09T20:13:23Z | 0 | green — pushed `6d626775..6c545ced` |
| 18 CI conclusion | 1 | 2026-10-09T20:14:51Z | 0 | green — `verdict: green · checks 16/16` (CI#37985678276, attempt 1) |
| 19 install-step durations | 1 | 2026-10-09T20:26:55Z | 0 | recorded — 8 script calls 10–16 s, the guarded one not reached, android action 5 s |

No entry read red. No re-run, no fix push. The wrap was not started: a seam hold is in force on the
word that opened this pass.
