# CI — 2026-10-13-range-input-interaction

`/andromeda-implement`'s session fired the operator pass on the operator's direction (2026-10-13), in this order:
1. fire 16 as written;
2. commit;
3. fire 17 and 18;
4. write this record;
5. stop before the wrap.

## Entry 16 — hygiene
Fired as written — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, its own command
line, not through `gate.py run --operator`, so its head and control lines stand in the record. It was green on the
first firing: exit 0, `hygiene: clean`. No row needed a fix (`evidence/operator-16.txt`, whole).

## The pre-CI commit
`dde6b1705ac576d06118352e99cda282d9c4614f`
- message: `chore(2026-10-13-range-input-interaction): operator pre-CI commit, for the run this chunk verdict reads`;
- the whole tree (`git add -A`);
- parent `97ce760ddfb4db2996162551501b809e51b46367`.

## Entry 17 — fast, the clean-tree guard, push
```
operator entry 17 · history tripwire: git
 17 probe       green · exit 0 · 120.89s · 95668 B → 17.log · history moved: refs/remotes/origin/build/escher-0.1.0 97ce760d→dde6b170 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 18 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 17
```
The history move is the push the entry exists to make: `97ce760dd..dde6b1705  build/escher-0.1.0 -> build/escher-0.1.0`.
The `fast` leg's CI-scripts tail read `Ran 79 tests` · `OK`.

## Entry 18 — the CI read on the pushed sha
```
operator entry 18 · history tripwire: none of 7 forms matched — not a clearance
 18 probe       green · exit 0 · 811.49s · 563 B → 18.log · history: unmoved · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci… (95 chars)
entries 18 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 17
```
From its log:
```
repo Turbolet85/escher (the push remote `origin`) · polled 27× over 811 s
dde6b1705ac5 verdict: green · checks 16/16 · wall 797 s · runs CI#38466886504 completed/success
runs: CI#38466886504 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

This run is the witness of the legs not run locally: the MSRV build and the windows, macos, ios and android tests.
The macOS leg is included, so the new keyboard branch's key arms (`Key::Arrow*`, `Home`, `End`, with no `#[cfg]`
around them) compiled and passed there.

## Verdict
**green**
- sha: `dde6b1705ac5`
- run: CI#38466886504
- checks: 16/16
- wall: 797 s (the last run took 1178 s)
