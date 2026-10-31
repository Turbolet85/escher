# CI — 2026-10-31-engine-debts

The operator pass, on the operator's word in session, 2026-10-31: "The pause holds the next chunk, not this one: this
chunk runs through its push, its CI and its wrap on the founder's plan, so entry 35 pushes." and "Stop and tell me if any
entry is red."

**State: green at the fix commit `40ac6ddac`, CI#41192433140, 17 of 17.** The first firing stopped red at entry 36, on a
runner out of disk; the operator's word put a free-disk step in the job; the second firing reads green, and entry 37 is
recorded. The second firing's readings are below the first's.

## The first firing, on `42eed99a6`: the entries, as they printed

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

**Entry 37** was not fired on this run.

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

## The second firing, on `40ac6ddac`

**The fix commit:** `40ac6ddac6410d929daf949123e28fd3f0a46b58`, subject `fix(2026-10-31-engine-debts): the free-disk step
for stand-repeat`, above the pre-CI commit, carrying `ci.yml`, the plan's entry 1, `scope-record.md`, this file and the
handoff. Entry 1 read green with the shorter guard before it, `scope: clean — changed 47 · listed 43 · recorded 4
(companion 3 · mechanical 0 · in-intent 0 · widening 1)`.

**Entry 35**, through `gate.py run --operator 35`:

```
 35 probe       green · exit 0 · 109.08s · 121233 B → 35.log · history moved: refs/remotes/origin/build/escher-0.3.0 42eed99a→40ac6dda · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 37 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 36
```

**Entry 36**, `ci.py conclusion --sha HEAD --wait 1800`:

```
40ac6ddac641 verdict: green · checks 17/17 · wall 1271 s · runs CI#41192433140 completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

**Lines the acceptance names**, read from the job logs: `stand-repeat: 3 of 3 rounds green` and `coverage floor: 3 of 3
crates at or above their floors`.

| Crate | Coverage | Floor |
|---|---|---|
| escher-driver | 4950/5049 = 98.04 % | 97 % |
| dioxus-native-dom | 2786/3235 = 86.12 % | 83 % |
| blitz-test-harness | 956/1008 = 94.84 % | 90 % |

**Entry 37**, `gate.py run --operator 37 --id 41192433140`: `37 probe recorded · exit 0 · 5.36s · 269 B → 37.log · history:
unmoved`. Its output, against CI#40487650912 (`.andromeda/test-plan-detail/ci-integration.md:50`):

| Job | Base (lines · passed · failed · ignored) | Now | Move |
|---|---|---|---|
| Test [default features] | 209 · 1125 · 0 · 12 | 214 · 1181 · 0 · 12 | +5 · +56 |
| Test (macos) | 190 · 1122 · 0 · 12 | 195 · 1178 · 0 · 12 | +5 · +56 |
| Coverage report | 190 · 1122 · 0 · 12 | 195 · 1178 · 0 · 12 | +5 · +56 |
| Test (windows) | 190 · 1076 · 0 · 9 | 195 · 1132 · 0 · 9 | +5 · +56 |
| Accessibility (a11y) tests | 14 · 85 · 0 · 0 | 14 · 112 · 0 · 0 | 0 · +27 |
| Stand checks repeated | 204 · 864 · 0 · 21 | 219 · 1008 · 0 · 21 | +15 · +144 |

Each move, by name:

- **The four test jobs, +5 · +56:** the five new targets, and the +56 of `counts.md`'s table: 7 unit tests (blitz-dom +6,
  blitz-shell +1), blitz-vibey-script `dom` +1, the five new targets 17, and 31 in the edited checks. All four jobs move
  alike, as the chunk adds no `cfg` site.
- **The a11y job, +27:** `select_interaction` +11, `stand_keyboard` +6, `stand_act_select` +5, `inert_subtree` +2,
  `stand_contrast` +2, `accessibility_states` +1. Fourteen files, unmoved.
- **The stand repeat, +15 · +144, three rounds:** +5 targets a round. +48 passed a round: 17 in `stand_*` files
  (`stand_keyboard` 6, `stand_act_select` 5, `stand_contrast` 2, `stand_act_transformed` 1, `stand_cells_served` 1,
  `stand_lint` 1, `stand_load_page` 1), 17 in the five new targets, and 14 in named targets already on the list
  (`select_interaction` 11, `inert_subtree` 2, `accessibility_states` 1).

## What the green run does and does not show about the disk

The step ran, `completed · success`, its first witness in this workflow. On this run's runner, from its own `df` lines:

| Reading of `/dev/root` | Size | Used | Free |
|---|---:|---:|---:|
| before the step | 145G | 59G | 86G |
| after the step | 145G | 37G | 109G |

It saved Android 11 GiB, .NET 5.7 GiB, Haskell 3.6 GiB and Docker images 2.5 GiB. No disk warning stands in this run's log.

**The green does not prove the step was the cure.** This runner had 86 GB free before the step, far past what the job's
binaries need by the dev host's figures, and the failed run left `Free space left: 5 MB`. That run has no `df` line, so its
runner's size and free space are unknown. The earlier inference that this chunk's five targets tipped the job over is
weakened by these readings: a runner of another size, or runner variance, explains the red at least as well. The step
stands on the operator's word, and it does what it says. Whether it was needed is unmeasured.

## The runner images: the red job alone stood on an older one

Read on the operator's word at the wrap's resume, 2026-10-31, from each job's own log (the `Runner Image` and `Hosted
Compute Agent` groups of its setup step), every job of both runs:

| Run · job | Image | Image version | Agent version |
|---|---|---:|---:|
| CI#41190771843 · **Stand checks repeated (failure)** | `ubuntu-24.04` | **20261018.96.1** | **20261015.610** |
| CI#41190771843 · the 12 other `ubuntu-24.04` jobs (success) | `ubuntu-24.04` | 20261025.353.1 | 20261021.618 |
| CI#41192433140 · Stand checks repeated (success) | `ubuntu-24.04` | 20261025.353.1 | 20261021.618 |
| CI#41192433140 · the 12 other `ubuntu-24.04` jobs (success) | `ubuntu-24.04` | 20261025.353.1 | 20261021.618 |
| Both runs · Test (windows) | `windows-2025-vs2026` | 20261021.278.1 | 20261021.618 |
| Both runs · Test (macos) | `macos-26-arm64` | 20261026.0455.2 | 20261021.618 |
| CI#41190771843 · Test (ios) | `macos-26-arm64` | 20261021.0445.8 | 20261021.618 |
| CI#41192433140 · Test (ios) | `macos-26-arm64` | 20261026.0455.2 | 20261021.618 |

So among the `ubuntu-24.04` jobs the red job alone ran on image `20261018.96.1`; the others read `20261025.353.1`. This
adds to the runner-variance reading above and does not settle it: the older image's free disk was not read, the failed run
having no `df` line, and the one job that failed is also the one job building this job's targets three rounds over.
