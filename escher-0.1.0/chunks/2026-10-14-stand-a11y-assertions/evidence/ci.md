# Operator pass — hygiene, the push and the CI read

Entries 12 and 13 were fired in the window before a context clear. Entry 12's printed output was not kept, and it
rests on the operator's word ("hygiene clean, fast green, aab4cda9 pushed"; the wrap's record `inputs#I6`). Entry 13's
log survived in the gate tool's log directory and is read below. The implement run's gate record
(`.andromeda/runs/2026-10-14T07-05-28-implement/gate-2026-10-14-stand-a11y-assertions.json`) lists both as
`not run — leg operator`, as it should.

## Entry 12 — hygiene, before the pre-CI commit
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, expect `['exit 0', 'contains hygiene: clean']`.

- Operator's word: `hygiene: clean`.
- Printed output: not kept, and not re-fired here. A re-fire now would run after the pre-CI commit, so it would not
  be the entry the plan orders. No log of it stands in the gate tool's log directory.
- The operator's ruling at the wrap (inputs#I6): the entry rests on the pre-pause context, the wrap's own P7 hygiene
  read is the check on it, and no re-run is needed.

## Entry 13 — local pre-push gate, clean-tree guard, push
`bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0`,
after the pre-CI commit `aab4cda979ba79aadbe2c9f16e1b848321c2620a`
(`chore(2026-10-14-stand-a11y-assertions): operator pre-CI commit, for the run this chunk verdict reads`,
committed 2026-10-14 09:19:00 +0200, 70 files).

The entry's own log, `$TMPDIR/andromeda-gate/2026-10-14-stand-a11y-assertions/run-20261014T071904Z/13.log`
(2053 lines, 97115 B, last written 09:20:16 +0200), was found and read at the wrap's P1:
- 182 `test result:` lines, all `ok`: 937 passed · 0 failed · 12 ignored.
- The CI scripts: `Ran 79 tests in 5.561s` · `OK`.
- Its last lines are the push: `To https://github.com/Turbolet85/escher.git` ·
  `ccfcda487..aab4cda97  build/escher-0.1.0 -> build/escher-0.1.0`.

Also measured:
- The leg logs in `target/ci-logs/` are dated after that commit: `fmt.log` 09:19:04 (empty), `clippy.log` 09:19:05,
  `test.log` 09:20:08, `ci-scripts.log` 09:20:13.
- `test.log`: 182 result lines, 937 passed · 0 failed · 12 ignored. This matches the implement outcome's figure.
- `ci-scripts.log`: `Ran 79 tests` · `OK`.
- `git rev-parse origin/build/escher-0.1.0` = `aab4cda979ba79aadbe2c9f16e1b848321c2620a`, the same as local HEAD,
  so the push landed.

## Entry 14 — the run on the pushed sha
Fired as written (`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`),
exit 0. Expect `['exit 0', 'contains verdict: green']`: held.

```
ci v1.3 · c2400620
repo Turbolet85/escher (the push remote `origin`) · polled 1× over 2 s
aab4cda979ba verdict: green · checks 16/16 · wall 796 s · runs CI#38527662148 completed/success
runs: CI#38527662148 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

- sha `aab4cda979ba79aadbe2c9f16e1b848321c2620a` · run CI#38527662148 · verdict **green**, 16/16 checks, wall 796 s.
  The run had already completed, so the tool polled once.
- `Accessibility (a11y) tests` is green on the hosted runner. That is the widened leg, which now also runs
  `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` (the entry's note).
- `Test (windows)`, `Test (macos)`, `Test (ios)`, `Test (android)` and `MSRV Build [Rust 1.91]` are green.
