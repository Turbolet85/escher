# Operator pass — 2026-10-13-upstream-sync-polish-and-ship

Run 2026-10-13 on the operator's direction ("Run the operator pass, steps 1–6, then stop before the wrap"). Each entry was fired by the agent in this session. Lines are copied verbatim without the header.

## Entry 47 — hygiene, before the pre-CI commit
First firing, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, exit 0, **refused**:

```
P1 .andromeda/runs/2026-10-13T14-26-44-phase/controls.txt:25 ×2 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 101 (runs 91 · evidence 5 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

- **The cause:** the phase run's control 8 recorded its command with the absolute path of a session scratchpad under the OS temp dir (`trial-Cargo.lock`).
- **The fix:** that prefix was replaced in place by `$TMPDIR/{phase-session-scratchpad}/`, the form the gate tool prints for its own temp dir. Nothing else in the file moved.
  - `respell` was not used, because it rewrites only in-root paths.
  - The file's sha256 before the mask was `2103cde3…`.

Second firing, exit 0:

```
hygiene: clean — read 101 (runs 91 · evidence 5 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
- `a6c24d24ef661ecc7fadaade79cfb56d703c5e96` — `chore(2026-10-13-upstream-sync-polish-and-ship): operator pre-CI commit — merge upstream/main 434e06615, for the run this chunk verdict reads`.
- **Parents:** `a703136878e47c0cf583448bbac02f06f8365be5` and `434e06615d5f47a3df92c0f94e916df088fdcf4d`. It is a true merge, with both parents kept.
- **Contents:** `git add -A` of the merged tree, the phase and implement run dirs, the chunk folder and the phase's route and handoff edits. The tree was clean after it.

## Entry 48 — fast, the clean-tree guard, the push
`gate.py run --plan …/plan.md --operator 48`:

```
operator entry 48 · history tripwire: git
 48 probe       green · exit 0 · 113.88s · 93720 B → 48.log · history moved: refs/remotes/origin/build/escher-0.1.0 a7031368→a6c24d24 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 50 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 49
```

The history move was intended: the push of the merge commit.

## Entry 49 — the CI conclusion
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 3600`, exit 0:

```
repo Turbolet85/escher (the push remote `origin`) · polled 34× over 1028 s
a6c24d24ef66 verdict: green · checks 16/16 · wall 1023 s · runs CI#38422391931 completed/success
runs: CI#38422391931 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

| run | sha | verdict | jobs | wall-clock |
|---|---|---|---|---|
| CI#38422391931 (push, created 2026-10-13T15:28:39Z, updated 15:45:45Z) | `a6c24d24` | **green**, completed/success | 16/16 | 1023 s |

The run was cold, as the plan expected (the lock change re-keys every cache). The previous runs took 923 s (`495067f8`) and 1100 s (the last sync merge, cold).

The run covers the MSRV build on Rust 1.91 with `taffy 0.15.0` (green), the wasm leg, coverage, and the windows, macos, ios and android legs.

## Entry 50 — our checks in the platform jobs
`gate.py run --plan …/plan.md --operator 50 --id 38422391931`:

```
operator entry 50 · history tripwire: none of 7 forms matched — not a clearance
 50 probe       recorded · exit 1 · 5.9s · 2 B → 50.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | grep -c -E '… (135 chars)
entries 50 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 49
```

Its log reads `0`. **This zero is the instrument's, not an absence.**
- `gh run view --log` writes the run's color codes as literal text: `Running^[[0m tests/stand_act_keys.rs`, with `^[` as two characters.
- The entry's pattern `Running tests/(…)\.rs` therefore cannot match any line of any job.
- `gh` itself exited 0 with 3,353,958 bytes and an empty stderr.

The intended reading, taken by hand over the same log with the literal escapes stripped first and the Windows separator `\` read as well:

| job | stand_act_keys | stand_session_lifecycle | button_alignment |
|---|---|---|---|
| Test [default features] (linux) | 1 | 1 | 1 |
| Test (macos) | 1 | 1 | 1 |
| Test (windows) | 1 (`tests\stand_act_keys.rs`) | 1 | 1 |
| Coverage report | 1 | 1 | 1 |

All three targets, two of ours and one of upstream's new ones, ran once in each of the linux, macOS and Windows test jobs and in coverage, for 12 in all.

`test result:` lines per job, against the last green run before the merge (CI#38357259286 on `495067f8`):

| job | before | after |
|---|---|---|
| Test [default features] | 174 | 179 |
| Test (macos) | 155 | 160 |
| Test (windows) | 155 | 160 |
| Coverage report | 155 | 160 |
| Accessibility (a11y) tests | — | 3 |

Each test job gains the five new targets. The linux job's tally is 893 passed · 0 failed · 12 ignored, equal to the local workspace leg.
