# Operator pass — 2026-10-24-stand-requirement-sweep (2026-10-24)

The plan's four `leg = 'operator'` entries, fired by hand in plan order on the operator's instruction. Each entry's
line and the call's summary line are copied verbatim, without the header.

## Entry 20 — `gate.py hygiene`, before the pre-CI commit (fired as written)
```
hygiene: clean — read 477 (runs 471 · evidence 2 · inputs 4) · trails 14 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Green: `contains hygiene: clean`.

## The pre-CI commit
`git add -A`, staged as the previous chunk's pre-CI commit (`34f6f051b`) was, then committed:
`18b50a2a2 chore(2026-10-24-stand-requirement-sweep): operator pre-CI commit, for the run this chunk's verdict reads`.
The tree read 0 changed rows after it.

## Entry 21 — `fast`, the clean-tree guard, the push (`gate.py run … --operator 21`)
```
operator entry 21 · history tripwire: git
 21 probe       green · exit 0 · 103.23s · 114143 B → 21.log · history moved: refs/remotes/origin/build/escher-0.2.0 4b4804ba→18b50a2a · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
Green. The history move is the intended push of `build/escher-0.2.0` to `18b50a2a`. The `fast` leg's test log held 206
result lines, all `ok`.

## Entry 22 — `ci.py conclusion --sha HEAD --wait 1800` (fired as written)
```
18b50a2a20f3 verdict: green · checks 17/17 · wall 1127 s · runs CI#40114977936 completed/success
runs: CI#40114977936 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```
Green: `contains verdict: green`. Polled 38 times over 1146 s. `Stand checks repeated` and `Coverage report` are among the
checks.

## Entry 23 — the per-OS tallies (`gate.py run … --operator 23 --id 40114977936`)
```
operator entry 23 · history tripwire: none of 7 forms matched — not a clearance
 23 probe       recorded · exit 0 · 4.98s · 266 B → 23.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 23 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 22
```
The output it records, copied from its log:
```
Accessibility (a11y) tests · 6 · 43 · 0 · 0
Coverage report · 187 · 1102 · 0 · 12
Test (macos) · 187 · 1102 · 0 · 12
Test (windows) · 187 · 1057 · 0 · 9
Test [default features] · 206 · 1105 · 0 · 12
Stand checks repeated · 198 · 837 · 0 · 21
```
Read against CI#40105170961 in `evidence/ci.md`: every moved count is explained by name, and 0 failed in every job.

## Verdict
No entry was red. The wrap was not started: the operator holds it for a deploy.
