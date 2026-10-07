# Operator pass — 2026-10-07-act-by-id

The plan's three `leg = 'operator'` entries (20, 21, 22 of `## Test Commands`) and the pre-CI
commit between the first two, each driven once by hand, in plan order, on the dev host (Linux).

**Who drove it:** the implementing agent, in the operator's stead and on the operator's word —
word: "Run the operator pass for 2026-10-07-act-by-id now, each entry by hand in plan order:
hygiene, the commit "chore(2026-10-07-act-by-id): operator pre-CI commit, for the run this chunk
verdict reads", the fast leg with the clean-tree guard and the push, then ci.py conclusion --sha
HEAD --wait 1800. Record it in evidence/operator-pass.md. Stop and tell me if any entry is red."
— the operator, 2026-10-07, given in the implement session after its green report.

Each `run` below is the plan's own text; the tools directory is spelled `~/…` as the gate tool
prints it. Every exit was read from the bare command, never through a pipe.

## Entry 20 — hygiene

- `run`: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`
- fired: 2026-10-07T14:02Z, before the commit, on the tree the commit then staged whole
- exit: `0` — expect `exit 0` **held**
- atom `contains hygiene: clean` — **held**; the summary line, verbatim:
  `hygiene: clean — read 42 (runs 38 · evidence 4 · inputs 0) · trails 13 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`
- its head: `root . · case exact · planes rust · base HEAD (no --marker)`; the control line fired
  on every synthetic positive (ten forms)
- verdict: **green**

This file was written after that read, so the read did not cover it; the wrap's own hygiene
read (with `--marker`) does.

## The pre-CI commit

- `git add -A`, then one commit: `f8eb42c891d6b7c98c77016d8cf561ebdc3513df`
- subject, as the operator gave it:
  `chore(2026-10-07-act-by-id): operator pre-CI commit, for the run this chunk verdict reads`
- parent: `3c58ce5cc087750f64497be92206790a04f1575a` (the commit the chunk starts from)
- carries 65 files (6079 insertions, 28 deletions): the chunk's 15 source and test files, its
  `plan.md` · `research.md` · `scope.md` and four evidence files, the matrix, the working route,
  the master route, the handoff and the friction log as the phase and implement steps left them,
  and three run dirs (wrap 1 file · phase 34 · implement 3)
- after it: `git status --short` printed nothing; the branch read 1 ahead of `origin/build/escher-0.1.0`

## Entry 21 — the local pre-push gate, the clean-tree guard, the push

- `run`: `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`
- fired once, as one chained command: 2026-10-07T14:02:42Z → 14:04:04Z (82 s; the entry's bound is 3600 s)
- exit: `0` — the entry lists no `expect`, so the default `exit 0` **held**; with `&&` between
  the four parts, exit 0 means each of them exited 0
- the fast leg, read from this run's logs under `target/ci-logs/` (each file's mtime inside the
  run's window): fmt — no output; clippy `--workspace --locked -- -D warnings` — finished, no
  warning; workspace tests — **149 result lines · 629 passed · 0 failed · 8 ignored**, every
  line `ok`; CI scripts — `Ran 64 tests` · `OK`
- the guard: both `git diff --quiet` forms exited 0 — nothing unstaged, nothing staged
- the push, verbatim: `3c58ce5c..f8eb42c8  build/escher-0.1.0 -> build/escher-0.1.0`
  (a fast-forward of the fork's build branch; no force)
- after it: `git ls-remote origin build/escher-0.1.0` read `f8eb42c891d6b7c98c77016d8cf561ebdc3513df`, equal to `HEAD`
- verdict: **green**

## Entry 22 — the CI verdict on the pushed sha

- `run`: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
- fired once: 2026-10-07T14:04:13Z → 14:20:17Z, `HEAD` = `f8eb42c891d6b7c98c77016d8cf561ebdc3513df`
  (964 s of the 1800 s wait; the entry's bound is 1980 s)
- exit: `0` — expect `exit 0` **held**
- atom `contains verdict: green` — **held**; the tool's lines, verbatim:
  - `ci v1.0 · bce1ea9a`
  - ``repo Turbolet85/escher (the push remote `origin`) · polled 32× over 964 s``
  - `f8eb42c891d6 verdict: green · checks 16/16 · wall 945 s · runs CI#37633611745 completed/success`
  - `runs: CI#37633611745 push completed/success`
- verdict: **green** — run **CI#37633611745** on the pushed sha

### What that run witnesses — read from the run, not from the colour

`gh run view 37633611745 -R Turbolet85/escher --json headSha,status,conclusion,jobs`: head sha
`f8eb42c891d6b7c98c77016d8cf561ebdc3513df`, `completed` / `success`, 16 jobs, each `success`:
Clippy · Test [default features] · Test CI scripts · Rustfmt · Build wasm examples · Build counter
example · Documentation · Accessibility (a11y) tests · Dependency audit · Coverage report ·
Build [default features] · Test (ios) · Test (macos) · MSRV Build [Rust 1.91] · Test (android) ·
Test (windows).

The job logs of the two desktop legs no local gate runs (`gh run view … --job {id} --log`):

| reading | Test (macos), job 112836079301 | Test (windows), job 112836079486 |
|---|---|---|
| target | `aarch64-apple-darwin` | `x86_64-pc-windows-msvc` |
| `Running` lines naming a `stand_act_*` file | 6 — diff, ids, keys, range, refused, timer | 6 — the same six |
| `test backspace_deletes_one_character_of_what_the_driver_typed` | `ok` | `ok` |
| `test tab_and_shift_tab_return_exactly_the_controls_whose_focus_moved` | `ok` | `ok` |
| `test result: ok.` lines | 130 | 130 |
| `test result: FAILED` lines | 0 | 0 |

So the macOS reading of the deleting key is measured: on `aarch64-apple-darwin`, where the
editor's Backspace arm is compiled out, the driver's `press backspace` left two of three typed
characters — through the `deleteBackward:` binding the macOS arm of `press` dispatches. The MSRV
build (Rust 1.91) and the windows leg are witnessed by the same run.

Not read from the logs: the ios and android legs' test lists (their jobs read `success`; whether
they run `blitz-tests` was not opened), and the count of tests per `stand_act_*` file on the two
desktop legs (the two key tests above were read by name; the rest by their files' `Running` lines
and the absence of any `FAILED` result line).

## Summary

| entry | what | exit | atoms | verdict |
|---|---|---|---|---|
| 20 | hygiene | 0 | `hygiene: clean` held | green |
| — | pre-CI commit `f8eb42c8` | 0 | — | made |
| 21 | fast leg · clean-tree guard · push | 0 | default `exit 0` held | green |
| 22 | CI conclusion on `f8eb42c8` | 0 | `verdict: green` held · CI#37633611745 · 16/16 | green |

No entry was red, and none was re-fired. Nothing was force-pushed. After the pass the tree
carries one uncommitted file, this record, which rides the wrap's commit.
