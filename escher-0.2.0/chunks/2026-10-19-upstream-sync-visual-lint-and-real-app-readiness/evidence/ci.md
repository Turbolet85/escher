# Operator pass and CI — 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness

Run 2026-10-19 on the operator's word in this session: «Run the operator pass … each entry in plan order: 31 hygiene;
the commit …; 32 the push; 33 ci.py conclusion; 34 the per-OS tallies. Record it in evidence/ci.md. Stop and tell me if
any entry is red.» No entry read red.

**Verdict: green.** CI#39312031677 on the pre-CI commit `79603d1ef`, 17/17 checks.

## Entry 31 — hygiene, fired as written before the commit
```
hygiene: clean — read 51 (runs 40 · evidence 6 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`79603d1ef633cde4dfc841e3bb4da23a39b86115`, from `git add -A` of the whole tree: the staged take, the evidence, the
phase and implement run dirs, the phase's route and master edits, and the friction log. That is the precedent pre-CI
commit's shape (`d2f010a2a`), and entry 32's clean-tree guard needs it. Its message:

```
chore(2026-10-19-upstream-sync-visual-lint-and-real-app-readiness): operator pre-CI commit, for the run this chunk's verdict reads

(cherry picked from commit 38ca4e777513607c7dcb659748380e7a650b018f)
(cherry picked from commit 17de049b11095808e0d093be2a8f0520e4dcbc0e)

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
```

## Entry 32 — the push, through `gate.py run --plan … --operator 32`
```
operator entry 32 · history tripwire: git
 32 probe       green · exit 0 · 152.67s · 108769 B → 32.log · history moved: refs/remotes/origin/build/escher-0.2.0 67f17b9d→79603d1e · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 34 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 33
```
The fast gate was green on the committed tree, the tree was clean, and the push landed: `origin/build/escher-0.2.0`
moved from `67f17b9d` to `79603d1e`. That move is the one this entry intends.

## Entry 33 — `ci.py conclusion --sha HEAD --wait 1800`, fired as written
```
ci v1.3 · 2b582a9b
repo Turbolet85/escher (the push remote `origin`) · polled 34× over 1025 s
79603d1ef633 verdict: green · checks 17/17 · wall 1005 s · runs CI#39312031677 completed/success
runs: CI#39312031677 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```
- The wall time was 1005 s, close to the base's 998 s; the lock is unchanged, so the run was warm.
- The MSRV build on 1.91 is green.
- The "Coverage report" job's floors, read from its log: escher-driver 98.33 % (floor 97), dioxus-native-dom 84.74 %
  (83) and blitz-test-harness 93.95 % (90), giving `coverage floor: 3 of 3 crates at or above their floors`.
  `keyboard_compat.rs` is excluded by the leg's two flags, which entry 8 read at `2` (CARRY 2).

## Entry 34 — the per-OS tallies, `--operator 34 --id 39312031677`
```
operator entry 34 · history tripwire: none of 7 forms matched — not a clearance
 34 probe       recorded · exit 0 · 5.88s · 266 B → 34.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 34 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 33
```
Its log (lines · passed · failed · ignored), against the base's green run CI#39302945352 (`67f17b9d6`):

| job | CI#39302945352 (base) | CI#39312031677 (this take) | moved |
|---|---|---|---|
| Test [default features] (linux) | 197 · 1034 · 0 · 12 | 199 · 1045 · 0 · 12 | +2 · +11 |
| Test (macos) | 178 · 1031 · 0 · 12 | 180 · 1042 · 0 · 12 | +2 · +11 |
| Coverage report | 178 · 1031 · 0 · 12 | 180 · 1042 · 0 · 12 | +2 · +11 |
| Test (windows) | 178 · 989 · 0 · 9 | 180 · 1000 · 0 · 9 | +2 · +11 |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — |
| Stand checks repeated | 156 · 624 · 0 · 21 | 156 · 624 · 0 · 21 | — |

iOS and Android build only. No job reads a failed test.

### This take's moves, by name
Read per target from both runs' job logs (each `Running`/`Doc-tests` line's following result line, compared target
by target). On each of the four test jobs exactly two targets differ:
- `tests/focus_within.rs`: a new result line, 7 passed.
- `tests/focus_redraw.rs`: a new result line, 4 passed.

That is +2 result lines and +11 passed per job. Upstream's two files run as their own targets on every OS; neither is
unix-gated. The a11y job runs its six named targets and the stand repeat runs `stand_*` files plus seven_guis; neither
holds an upstream file, so both are unmoved.

### The 2026-10-19 chunks' moves since the last recorded reading
The last reading test-plan.md:330 records is CI#39111579925 (`87f12079`): linux 194 · 1001 · 0 · 12, macos and
coverage 175 · 998 · 0 · 12, windows 175 · 974 · 0 · 9, a11y 6 · 43 · 0 · 0, stand repeat 147 · 588 · 0 · 21. From it
to the base CI#39302945352, the move is linux, macos and coverage +3 · +33; windows +3 · +15; stand repeat +9 · +36;
a11y none.

Every moved target belongs to the two 2026-10-19 chunks. Between `87f12079` and `67f17b9d6`, the only commits touching
a `.rs` file are theirs; the one-virtual-clock flip `1c9bc58b5` and the engine-debts acceptance `93336e886` touch none.
The split between the two uses session-recording's green run CI#39202401256.

**2026-10-19-session-recording** (CI#39111579925 → CI#39202401256): +2 result lines on linux, macos and coverage
(+26 passed), +2 on windows (+7), +6 on the stand repeat (+24).
- New targets:
  - seven_guis `recording`: 3 passed. On windows it is 1, the off-unix refusal.
  - blitz-tests `stand_session_recording`: 3 passed. On windows its line reads 0, being unix-gated item by item.
- Extended targets:
  - escher-driver unit tests: +13 (+3 on windows).
  - `host_log` and `mcp_log`: +1 each, the recorded tests. Not run on windows.
  - `virtual_clock_engine`: +1.
  - blitz-dom unit tests: +1.
  - dioxus-native-dom unit tests: +1.
- Stand repeat: `recording` 9, `stand_session_recording` 9, and `host_log` and `mcp_log` +3 each, over 3 rounds.

**2026-10-19-recorded-check-replay** (CI#39202401256 → CI#39302945352): +1 result line on linux, macos and coverage
(+7 passed), +1 on windows (+8), +3 on the stand repeat (+12).
- New target: blitz-tests `stand_replay`, 3 passed on every OS.
- Extended targets:
  - escher-driver unit tests: +5 on every OS.
  - seven_guis `recording`: +1, `the_committed_checks_are_what_the_binary_records`, unix-gated. Windows unmoved.
- Stand repeat: `stand_replay` 9 and `recording` +3, over 3 rounds.

Sums:
- Linux, macos and coverage: +2 + 1 = +3 lines and +26 + 7 = +33 passed.
- Windows: +3 lines and +7 + 8 = +15 passed. Session-recording's +7 is escher-driver 72 → 75, `recording` 0 → 1,
  `virtual_clock_engine` +1, and blitz-dom and dioxus-native-dom +1 each. Recorded-check-replay's +8 is escher-driver
  75 → 80 and `stand_replay` 3.
- Stand repeat: +6 + 3 = +9 lines and +24 + 12 = +36 passed.

All match the measured job totals.

The per-target reads ran outside the gate tool, through a scratchpad script (`gh run view {id} --log`, colour codes
stripped, each `Running`/`Doc-tests` target's result lines summed per job). Their per-target sums reconcile with
entry 34's job totals on all three runs read.
