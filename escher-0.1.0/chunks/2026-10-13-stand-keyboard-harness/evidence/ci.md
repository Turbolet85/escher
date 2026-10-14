# Operator pass — the push and the CI read

## Entry 16 — local pre-push gate, clean-tree guard, push
Fired through `gate.py run --plan escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/plan.md --operator 16`
(no run dir), after the pre-CI commit `71af33d06f0a782469c7589c5ff88245af780bbe`
(`chore(2026-10-13-stand-keyboard-harness): operator pre-CI commit, for the run this chunk verdict reads`).

```
operator entry 16 · history tripwire: git
 16 probe       green · exit 0 · 126.53s · 96733 B → 16.log · history moved: refs/remotes/origin/build/escher-0.1.0 e40d9437→71af33d0 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
```

The history move is the intended push: `origin/build/escher-0.1.0` e40d9437 → 71af33d0, local HEAD equal to it.

## Entry 17 — the run on the pushed sha
Fired as written (`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`),
exit 0. Expect `['exit 0', 'contains verdict: green']`: held.

```
ci v1.3 · c2400620
repo Turbolet85/escher (the push remote `origin`) · polled 26× over 775 s
71af33d06f0a verdict: green · checks 16/16 · wall 754 s · runs CI#38482525151 completed/success
runs: CI#38482525151 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

- sha `71af33d06f0a782469c7589c5ff88245af780bbe` · run CI#38482525151 · verdict **green**, 16/16 checks, wall 754 s.
- `Test (macos)` and `Test (windows)` green: the windows and macos witness of `stand_keyboard`'s
  `the_apple_delete_binding_deletes_on_every_os` (the acceptance criterion's CI half). `MSRV Build [Rust 1.91]`,
  `Test (ios)` and `Test (android)` green.
