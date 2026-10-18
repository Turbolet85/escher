# Operator pass — 2026-10-18-upstream-sync-determinism-and-traces

Run on 2026-10-18, entry by entry in plan order, by the session at the operator's direction ("Run the operator pass
… each entry by hand in plan order", the operator, 2026-10-18).

## Entry 27 — `gate.py hygiene`, fired as written (06:28Z, before the pre-CI commit)
```
gate v1.20 · da334230
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 88 (runs 74 · evidence 6 · inputs 8) · trails 14 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Green: `hygiene: clean`.

## The pre-CI commit
- `git add -A && git commit -F -`: exit 0, giving **`275950dc0985f258c4fd26fc98faa80fb29df31d`**, 96 files changed, on
  `build/escher-0.2.0` over `fb842671`.
- Subject, exactly: `chore(2026-10-18-upstream-sync-determinism-and-traces): operator pre-CI commit, for the run this chunk's verdict reads`.
- The body carries `(cherry picked from commit fb8d6c15e9ce062b3b41e71cd48988f0093506cc)`.
- After the commit, `git status --short` lists 0 lines.

## Entry 28 — `gate.py run --plan … --operator 28` (the operator-pass form, no run dir; log dir `$TMPDIR/andromeda-gate/2026-10-18-upstream-sync-determinism-and-traces/run-20261018T062939Z`)
```
operator entry 28 · history tripwire: git
 28 probe       green · exit 0 · 131.1s · 103387 B → 28.log · history moved: refs/remotes/origin/build/escher-0.2.0 fb842671→275950dc · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
not run — excluded · 29: 1-27,29-30
entries 30 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 29
```
- Green. The history move is the intended push. Its log reads `fb8426711..275950dc0  build/escher-0.2.0 -> build/escher-0.2.0`.
- The fast leg inside it: `test result:` lines summed are `lines 192 passed 991 failed 0 ignored 12`, equal to
  /implement's (`evidence/counts.md`). The CI scripts read `Ran 100 tests`, `OK`.

## Entry 29 — `ci.py conclusion --sha HEAD --wait 1800`, fired as written
```
ci v1.3 · 5f288001
repo Turbolet85/escher (the push remote `origin`) · polled 28× over 837 s
275950dc0985 verdict: green · checks 17/17 · wall 842 s · runs CI#39104364530 completed/success
runs: CI#39104364530 push completed/success
```
Green: `verdict: green`, every check settled. The check list is in `evidence/ci.md`.

## Entry 30 — `gate.py run --plan … --operator 30 --id 39104364530` (log dir `$TMPDIR/andromeda-gate/2026-10-18-upstream-sync-determinism-and-traces/run-20261018T064611Z`)
```
operator entry 30 · history tripwire: none of 7 forms matched — not a clearance
 30 probe       recorded · exit 0 · 4.94s · 262 B → 30.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
not run — excluded · 29: 1-29
entries 30 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 29
```
The output, from its log (report-only, `expect = []`):
```
Coverage report · 173 · 988 · 0 · 12
Accessibility (a11y) tests · 6 · 43 · 0 · 0
Test (macos) · 173 · 988 · 0 · 12
Test (windows) · 173 · 964 · 0 · 9
Test [default features] · 192 · 991 · 0 · 12
Stand checks repeated · 144 · 582 · 0 · 21
```
The comparison against CI#39100630123 and the attribution by name are in `evidence/ci.md`.

## Result
Entries 27, 28 and 29 are green and entry 30 is recorded. No entry was red.
