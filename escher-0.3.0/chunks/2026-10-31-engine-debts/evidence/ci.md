# CI — 2026-10-31-engine-debts

The operator pass, on the operator's word in session, 2026-10-31: "The pause holds the next chunk, not this one: this
chunk runs through its push, its CI and its wrap on the founder's plan, so entry 35 pushes." and "Stop and tell me if any
entry is red."

**State: stopped at entry 36, red.** Entry 37 was not fired. CI#41190771843 was still open at the stop.

## The entries, as they printed

**Entry 34**, fired as written:

```
hygiene: clean — read 606 (runs 587 · evidence 5 · inputs 14) · trails 14 not read · copies 12 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**The pre-CI commit:** `42eed99a64a9997e4d6113e09be2981652aad316`, subject `chore(2026-10-31-engine-debts): operator pre-CI commit, for the run this chunk's verdict reads`. The tree was clean after it.

**Entry 35**, through `gate.py run --operator 35`:

```
 35 probe       green · exit 0 · 181.81s · 121312 B → 35.log · history moved: refs/remotes/origin/build/escher-0.3.0 a6c6a37c→42eed99a · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 37 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 36
```

`git ls-remote` read `42eed99a` on the branch. The push carried the three records-door commits with it.

**Entry 36**, `ci.py conclusion --sha HEAD --wait 1800` on the operator's word (the plan's entry writes `--wait 2400`):

```
42eed99a64a9 verdict: red · checks 17/17 · first-fail +543 s Stand checks repeated · runs CI#41190771843 in_progress/-
failed 1: Stand checks repeated (failure)
running 5: oldest Test (windows) 182 s
run open: CI#41190771843 in_progress
```

**Entry 37** was not fired.

## The job states at the stop

| Job | State |
|---|---|
| Rustfmt · Clippy · Test [default features] · Test CI scripts · Build counter example · Dependency audit · Build [default features] · MSRV Build [Rust 1.91] · Documentation · Accessibility (a11y) tests · Test (ios) | success |
| **Stand checks repeated** | **failure** |
| Coverage report · Test (windows) · Test (macos) · Build wasm examples · Test (android) | in progress, unread |

## The failure

Read from the job's log (job 124508189681).

- **No test ran.** The log reaches `stand-repeat: round 1 of 3`, and the build fails in that round.
- **The linker died**, on three test binaries: `text_range_no_change_event`, `visible_region_transformed` and
  `stand_act_states`. Each reads `linking with cc failed`, `collect2: fatal error: ld terminated with signal 7 [Bus
  error], core dumped`.
- **The runner's disk was full:** `You are running out of disk space … Free space left: 5 MB`. That line stands after the
  failure, in the job's upload step.

`ld`'s bus error on a full disk is the expected form of this failure, and the warning names the disk. That is the direct
cause as read. The log holds no `df` reading from before the link.

## What is and is not known about why

| Reading | Value |
|---|---|
| The same job on the last green run, CI#40507800481 at `a6c6a37c7` | success, 3 rounds |
| Its restored cache, that run and this one | 957 MB in each |
| Targets this chunk added to this job (step 18) | 5 |
| One such test binary on the dev host, in debug | about 200 MB (`visible_region_transformed` 194 MB, `stand_act_states` 222 MB) |
| A free-disk step in this job | none. `.github/workflows/ci.yml:386` has one, on the `matrix_test` job alone |

**Inferred, not measured:** this chunk's five targets, near 1 GB by the dev host's figure, took a job already near its
limit over it. Three things stand against reading that as proven: the runner's own figures were not read, the two runs'
free space before the link is unknown, and `Test [default features]` built these targets among more and passed, with no
free-disk step either. Runner variance is not ruled out.

## The owner, and the word

The operator, in session, 2026-10-31: "Operator's word: the free-disk step. The other 16 jobs read green. Give stand-repeat
ci.yml:386's action at its pinned sha, before rust-cache, with android: true, large-packages: false, swap-storage: false.
Entry 1 loses .github/workflows on this word: record that in the plan and evidence/ci.md, and scope records ci.yml."

- **The step:** `jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be` in the `stand-repeat` job, after the
  toolchain and before `Swatinem/rust-cache`, `with` `android: true`, `large-packages: false`, `swap-storage: false`.
- **Entry 1** loses `.github/workflows` from its `run`, and its `note` says so. Its `baseline` key stands as measured then.
- **`scope-record.md`** holds `.github/workflows/ci.yml` as a `widening`, carrying the word.
- CI#41190771843 ended `failure`: `Stand checks repeated` the one failed job of 17, the other 16 `success`.

## A correction to the first report

The first report said the same action "already runs on the `matrix_test` job at `ci.yml:386`". **That was wrong.** The step
stands there, but its `if:` names `ubuntu-24.04` and `ubuntu-24.04-arm`, and the one Linux row, android, runs on
`ubuntu-latest`, so the step does not fire: on CI#41190771843's `Test (android)` it read `completed · skipped`. So this
workflow held no measured run of the action before this fix, and the new run is its first witness here.
