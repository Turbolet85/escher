# The operator pass — entries 16, 17 and 18

## Entry 16 — hygiene, fired by its own command line

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, from the repository root, before the
pre-CI commit — exit 0:

```
gate v1.17 · 383c6f7b
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 71 (runs 54 · evidence 11 · inputs 6) · trails 14 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Atoms: `exit 0` ✓ · `contains hygiene: clean` ✓.

## The pre-CI commit

`d44090ebf68688a1f4195560db75e8d8f0aa3899` — `chore(2026-10-14-quality-gates): operator pre-CI commit, for the run this
chunk verdict reads`, the whole tree staged (`git add -A`), on `build/escher-0.1.0`; the tree read clean after it.

## Entry 17 — the local pre-push gate, the clean-tree guard, the push

`bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
build/escher-0.1.0`, fired by its command line with the fast leg's output redirected to a scratch file (read whole
by count, not printed here) — exit 0. The fast leg: 186 result lines · 955 passed · 0 failed · 12 ignored, 0 `test
result: FAILED`, CI scripts `Ran 94 tests`, `OK`. The push: `eb5ba90c5..d44090ebf  build/escher-0.1.0 ->
build/escher-0.1.0`.

## Entry 18 — the CI read

`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` — exit 0:

```
ci v1.3 · c2400620
repo Turbolet85/escher (the push remote `origin`) · polled 42× over 1272 s
d44090ebf686 verdict: green · checks 17/17 · wall 1245 s · runs CI#38692629940 completed/success
runs: CI#38692629940 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

Atoms: `exit 0` ✓ · `contains verdict: green` ✓. **`Stand checks repeated` is among the 17 checks** (16 at
CI#38649787488; the new job is the seventeenth).

## What the changed and new jobs printed (job logs read through `gh run view -R Turbolet85/escher --job {id} --log`)

| job | wall | reading |
|---|---|---|
| Stand checks repeated (new) | 06:33:36Z → 06:42:45Z, 9 min 9 s | `stand-repeat: round 1 of 3` · `round 2 of 3` · `round 3 of 3` · `stand-repeat: 3 of 3 rounds green`; 141 result lines · 576 passed · 0 failed · 21 ignored = 3 × (168 + 24 passed, 7 ignored), equal to the host's rounds |
| Coverage report | 7 min 43 s | `packages/escher-driver 4784/4865 = 98.34 % (floor 97 %)` · `packages/dioxus-native-dom 2472/2934 = 84.25 % (floor 83 %)` · `packages/blitz-test-harness 916/999 = 91.69 % (floor 90 %)` · `coverage floor: 3 of 3 crates at or above their floors` — equal to the host to the line |
| Dependency audit | 15 s | `advisories ok, licenses ok`; one `warning[license-not-encountered]` |
| Clippy | 1 min 17 s | `Finished … in 7.08s` (the workspace line), `Finished … in 26.12s` (upstream's blitz-dom line) — both green |
| Test [default features] | 4 min 50 s | 186 result lines · 955 passed · 0 failed · 12 ignored — equal to the host's fast leg |

The run's wall, first job start to last job end: 1245 s (CI#38649787488: 910 s). The last job to end was `Test
(windows)` at 06:49:28Z; the new job ended at 06:42:45Z, inside that.
