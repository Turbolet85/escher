# Operator pass — 2026-10-07-driver-command-spans

The plan's three `leg = 'operator'` entries (26 · 27 · 28), each driven by hand in plan order, on
the operator's word given in the implement session, 2026-10-07: "Run the operator pass for
2026-10-07-driver-command-spans now, each entry by hand in plan order: hygiene, the commit
"chore(2026-10-07-driver-command-spans): operator pre-CI commit, for the run this chunk verdict
reads", the fast leg with the clean-tree guard and the push, then ci.py conclusion --sha HEAD
--wait 1800. Record it in evidence/operator-pass.md. The apt-get step hung in every run tonight:
five minutes after the push read each open job step through the API; a job in apt-get over three
minutes is cancelled and the unfinished jobs re-run, as often as needed. Stop and tell me on a real
red." Every command is spelled as the gate tool prints it: the repository root as `.`, the home
directory as `~`.

## Entry 26 — hygiene, before the pre-CI commit

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- **fired:** 2026-10-07T22:12:29Z, bare, from the repository root, on the uncommitted tree the
  pre-CI commit then staged
- **exit:** 0
- **atoms:** `exit 0` — held · `contains hygiene: clean` — held
- **verdict:** green
- **output, whole:**

```
gate v1.12 · 8357110d
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 37 (runs 32 · evidence 2 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record was written after that read, so the read above did not cover it. A second read, made
after this file was written and before the commit, is recorded under it.

- **second read:** 2026-10-07T22:12:40Z, the same command, with this file in the tree — exit 0,
  `hygiene: clean — read 38 (runs 32 · evidence 3 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
  The lines added to this file after it hold no path outside the repository.

## The pre-CI commit, between entries 26 and 27

- **commit:** `44ad388759ae95bf9586bddd102dd65482821c60` on `build/escher-0.1.0`, parent
  `b75ed30bbeb9a362283c1716eaf83440b3076276` — `chore(2026-10-07-driver-command-spans): operator
  pre-CI commit, for the run this chunk verdict reads`; 52 files, `git add -A` over the tree
  entry 26 read plus this file's entry-26 record. The tree read clean after it (`git status
  --short`: 0 rows).

## Entry 27 — the local pre-push gate, the clean-tree guard, the push

- **run:** `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- **fired:** 2026-10-07T22:13:03Z, ended 2026-10-07T22:14:20Z, from the repository root, on the
  committed tree
- **exit:** 0 — every link of the chain held: the fast leg (fmt, clippy, workspace tests, CI
  scripts), no unstaged change, no staged change, the push
- **atoms:** the entry states none; the default `exit 0` — held
- **verdict:** green
- **the fast leg's workspace count,** read from `target/ci-logs/test.log` written by this firing:
  154 result lines, 656 passed, 0 failed, 10 ignored; the CI scripts leg ran 64 tests, `OK`
- **the push, as git printed it:**

```
To https://github.com/Turbolet85/escher.git
   b75ed30b..44ad3887  build/escher-0.1.0 -> build/escher-0.1.0
```

- **read back:** `git ls-remote origin build/escher-0.1.0` answers
  `44ad388759ae95bf9586bddd102dd65482821c60`. The push was not refused.

## Entry 28 — the CI run on the pushed sha

- **run:** `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- **fired:** 2026-10-07T22:14:39Z, ended 2026-10-07T22:31:12Z, from the repository root, `HEAD` =
  `44ad388759ae95bf9586bddd102dd65482821c60` (the work tree's one uncommitted change is this
  file). Fired once.
- **exit:** 0
- **atoms:** `exit 0` — held · `contains verdict: green` — held
- **verdict:** green
- **output, whole:**

```
ci v1.0 · c042db81
repo Turbolet85/escher (the push remote `origin`) · polled 33× over 993 s
44ad388759ae verdict: green · checks 16/16 · wall 982 s · runs CI#37694873705 completed/success
runs: CI#37694873705 push completed/success
```

The run the acceptance names is **CI#37694873705** on `44ad3887…`, green 16/16 at its first
attempt: created 2026-10-07T22:14:22Z, concluded 22:30:49Z. All sixteen jobs read `success`
(`gh run view 37694873705 -R Turbolet85/escher`, read 22:31:35Z): `Rustfmt` · `Test CI scripts` ·
`Clippy` · `Test [default features]` (22:14:26Z → 22:19:48Z) · `Documentation` · `Build counter
example` · `Build wasm examples` · `Build [default features]` · `Accessibility (a11y) tests` ·
`MSRV Build [Rust 1.91]` (22:19:51Z → 22:21:38Z) · `Coverage report` · `Dependency audit` ·
`Test (ios)` · `Test (android)` · `Test (macos)` (22:19:56Z → 22:25:31Z) · `Test (windows)`
(22:19:51Z → 22:30:48Z).

### The new check on the runners (read from the job logs, 22:31Z)

`gh run view 37694873705 -R Turbolet85/escher --log --job {id}`, each log searched for the new
test target. In all three the target `stand_act_spans` was run and printed the same four lines —
both parents `ok`, both children `ignored` as the children they are:

| job | target as cargo ran it | `each_driver_command_leaves_one_line_holding_nothing_it_handled` | `a_driver_command_writes_no_line_at_the_default_level` |
|---|---|---|---|
| `Test (windows)` | `tests\stand_act_spans.rs`, `x86_64-pc-windows-msvc` | ok | ok |
| `Test (macos)` | `tests/stand_act_spans.rs`, `aarch64-apple-darwin` | ok | ok |
| `Test [default features]` | `tests/stand_act_spans.rs` | ok | ok |

So the re-run children pass on windows and macOS as well as on Linux. Not read: the ios and
android job logs (their jobs read `success`), and what else each log holds.

### The package-install watch, on the operator's word

The operator's rule for this pass: five minutes after the push read each open job's step through
the API; a job in `apt-get` over three minutes is cancelled and the unfinished jobs re-run, as
often as needed. It was run as a read-only watch beside entry 28 (`gh api
repos/Turbolet85/escher/actions/runs/{run}/jobs`, the latest attempt): the first read at
2026-10-07T22:19:09Z — 289 s after the push ended at 22:14:20Z — then one read about every 46 s
until the run concluded, 17 reads in all: the last with a job open at 22:30:43Z, the last of all
at 22:31:29Z, which read the run `completed` / `success` with 16 of 16 jobs `success`.

- **At the first read:** 4 jobs existed, 3 `success`; the one open job, `Test [default
  features]`, stood in its own step `bash .github/scripts/ci-leg.sh test`, 156 s in — its
  package-install step had already returned.
- **The twelve jobs created at 22:19:51Z:** read at 22:19:55Z (in checkout and toolchain steps) and
  at 22:20:41Z, when one job, `MSRV Build [Rust 1.91]`, stood in `sudo apt-get update && sudo
  apt-get install -y libfontconfig1-dev`, 16 s in; at the next read, 22:21:27Z, it was past it.
  Every other job carrying that step had passed it between two reads.
- **The longest any job was read standing in the package-install step: 16 s.** No job reached
  three minutes there, so **nothing was cancelled and nothing was re-run**; the run concluded on
  attempt 1.
- **The last open job** was `Test (windows)`: its `test` step ran until about 22:26Z and its
  cache-saving post step then stood about 4 minutes (read 26 s → 212 s, 22:26:51Z → 22:29:57Z)
  before the job concluded at 22:30:48Z. That is not the package-install step and it returned;
  it was left alone.

The hang of the earlier runs tonight did not recur on this run. Nothing here changes its
standing finding: the step still has no timeout and no retry.

## Summary of the pass

| entry | firing | fired (UTC) | exit | verdict |
|---|---|---|---|---|
| 26 hygiene | 1 | 22:12:29Z | 0 | green — `hygiene: clean` |
| 27 fast · clean tree · push | 1 | 22:13:03Z | 0 | green — pushed `b75ed30b..44ad3887` |
| 28 CI conclusion | 1 | 22:14:39Z | 0 | green — `verdict: green · checks 16/16` (CI#37694873705, attempt 1) |

No real red was read at any step. No cancel, no re-run, no fix push.
