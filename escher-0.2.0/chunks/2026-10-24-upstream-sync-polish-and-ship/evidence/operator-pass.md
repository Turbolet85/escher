# Operator pass — 2026-10-24-upstream-sync-polish-and-ship

Run by Claude Code on the operator's instruction (2026-10-24, this session): "Run the operator pass … now, each entry by hand in plan order … 38 ci.py conclusion --sha HEAD --wait 1800 … Stop and tell me if any entry is red." No entry read red.

## Entry 36 — hygiene (fired as written, 2026-10-24T11:55:53Z)
```
gate v1.22 · 8750b4ee
root . · case exact · planes rust · base HEAD (no --marker)
hygiene: clean — read 518 (runs 505 · evidence 6 · inputs 7) · trails 13 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Green: `hygiene: clean`.

## The pre-CI commit
`git add -A` then `git commit -F {message file}` → `34f6f051b81460d82837f9433dbe1faebaf49e56`, 551 files changed, working tree clean after. It carries the ten picks, the tokio lock update, the take-up documents, the two run dirs and the chunk folder, the same whole-tree shape as the earlier pre-CI commits (`191e66fdb`).
- Subject, exactly: `chore(2026-10-24-upstream-sync-polish-and-ship): operator pre-CI commit, for the run this chunk's verdict reads`.
- Body: one sentence naming the take on the founder's «Десять коммитов (Рекомендую)» (inputs#I4), then the ten `(cherry picked from commit {full sha})` trailers in step 2's order (`28f848d3b…` through `f9cbb2756…`), then the Co-Authored-By line.

## Entry 37 — the local gate, the clean-tree guard, the push (`gate.py run --operator 37`)
```
operator entry 37 · history tripwire: git
 37 probe       green · exit 0 · 185.74s · 117445 B → 37.log · history moved: refs/remotes/origin/build/escher-0.2.0 a356f908→34f6f051 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 39 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 38
```
Green. The history move is the intended push (log: `a356f9086..34f6f051b  build/escher-0.2.0 -> build/escher-0.2.0`). Inside `fast`: workspace tally `lines 206 passed 1099 failed 0 ignored 12`, CI scripts `Ran 100 tests` · `OK`.

## Entry 38 — the CI read (fired as written but for `--wait`)
```
ci v1.3 · c3fb7d94
repo Turbolet85/escher (the push remote `origin`) · polled 40× over 1207 s
34f6f051b814 verdict: green · checks 17/17 · wall 1210 s · runs CI#40092092468 completed/success
```
Green: `verdict: green`. `--wait 1800` on the operator's word, in place of the plan's 2400; the run settled inside both (1207 s).

## Entry 39 — the per-OS tallies (`gate.py run --operator 39 --id 40092092468`)
```
 39 probe       recorded · exit 0 · 5.13s · 266 B → 39.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
```
Recorded (report-only). Its output, copied from its log:
```
Accessibility (a11y) tests · 6 · 43 · 0 · 0
Coverage report · 187 · 1096 · 0 · 12
Test (macos) · 187 · 1096 · 0 · 12
Test (windows) · 187 · 1052 · 0 · 9
Test [default features] · 206 · 1099 · 0 · 12
Stand checks repeated · 168 · 684 · 0 · 21
```
Read against CI#39410865604 in `evidence/ci.md`: +1 line and +1 passed on the four test jobs, all `hit_test_dropped_nodes`, and a11y and the stand repeat unmoved.

## Left for the wrap
`evidence/ci.md` and this file are written after the pre-CI commit, so they are uncommitted and ride the wrap's commit.
