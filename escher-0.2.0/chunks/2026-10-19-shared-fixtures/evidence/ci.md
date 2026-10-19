# Operator pass — 2026-10-19-shared-fixtures

Fired 2026-10-19 in plan order, on the operator's instruction. Lines copied verbatim from each call's output, headers
left out.

## Entry 21 — hygiene (before the pre-CI commit)
```
hygiene: clean — read 41 (runs 39 · evidence 2 · inputs 0) · trails 14 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`0fbdda02242a07bf583c0ef82998c098fc162394` — `chore(2026-10-19-shared-fixtures): operator pre-CI commit, for the run
this chunk's verdict reads` (the whole tree staged, as at the previous chunk's pre-CI commit `62403b643`).

## Entry 22 — fast leg, clean-tree guard, push
```
 22 probe       green · exit 0 · 99.87s · 108587 B → 22.log · history moved: refs/remotes/origin/build/escher-0.2.0 5174f92a→0fbdda02 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The history move is the intended push. The fast leg's workspace tally, read from its log: lines 199 · passed 1045 ·
failed 0 · ignored 12 (unmoved).

## Entry 23 — CI conclusion
```
0fbdda02242a verdict: green · checks 17/17 · wall 1017 s · runs CI#39336988025 completed/success
runs: CI#39336988025 push completed/success
```
Checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy ·
Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated ·
Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features].

## Entry 24 — per-OS tallies (`<id>` = 39336988025)
```
 24 probe       recorded · exit 0 · 6.14s · 266 B → 24.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 24 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 23
```
Output (job · result lines · passed · failed · ignored), against the previous chunk's CI#39324662409:

| job | CI#39336988025 | CI#39324662409 | moved |
|---|---|---|---|
| Test [default features] | 199 · 1045 · 0 · 12 | 199 · 1045 · 0 · 12 | none |
| Test (macos) | 180 · 1042 · 0 · 12 | 180 · 1042 · 0 · 12 | none |
| Coverage report | 180 · 1042 · 0 · 12 | 180 · 1042 · 0 · 12 | none |
| Test (windows) | 180 · 1000 · 0 · 9 | 180 · 1000 · 0 · 9 | none |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | none |
| Stand checks repeated | 156 · 624 · 0 · 21 | 156 · 624 · 0 · 21 | none |

Predicted unmoved, measured unmoved: moving the checks onto the shared module added and removed no test on any OS. The
stand-repeat leg ran its three rounds with 0 failed. The windows job compiled and ran the moved checks off unix.
