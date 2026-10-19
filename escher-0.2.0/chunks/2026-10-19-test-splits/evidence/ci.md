# Operator pass — 2026-10-19-test-splits

Fired in plan order on the operator's direction (this session, 2026-10-19): "Run the operator pass for
2026-10-19-test-splits now, each entry in plan order". Each entry's line and summary below are quoted verbatim, with the
call's header left out.

## Entry 23 — hygiene (before the pre-CI commit)
Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`:

```
hygiene: clean — read 46 (runs 40 · evidence 3 · inputs 3) · trails 14 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`62403b643d44b186781209916298ba4f92186a17` — `chore(2026-10-19-test-splits): operator pre-CI commit, for the run this
chunk's verdict reads` (the whole tree staged, as the previous chunk's pre-CI commit `79603d1ef` was; tree clean after).

## Entry 24 — fast leg, clean-tree guard, push
`gate.py run --plan … --operator 24`:

```
 24 probe       green · exit 0 · 89.44s · 108586 B → 24.log · history moved: refs/remotes/origin/build/escher-0.2.0 f98a4dce→62403b64 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The history move is the intended push.

## Entry 25 — CI conclusion
Fired as written, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`:

```
62403b643d44 verdict: green · checks 17/17 · wall 1026 s · runs CI#39324662409 completed/success
```

The coverage job's floor lines, read from `gh run view 39324662409 -R Turbolet85/escher --log`:

```
packages/escher-driver 4266/4350 = 98.07 % (floor 97 %)
packages/dioxus-native-dom 2565/3027 = 84.74 % (floor 83 %)
packages/blitz-test-harness 947/1008 = 93.95 % (floor 90 %)
coverage floor: 3 of 3 crates at or above their floors
```

The windows job ran `recording::off_unix_tests::off_unix_a_recording_is_refused_as_unsupported` from its new file (its
`… ok` line appears 2 times in the run's log).

## Entry 26 — per-OS tallies (report-only)
`gate.py run --plan … --operator 26 --id 39324662409` (this chunk's run):

```
 26 probe       recorded · exit 0 · 5.39s · 266 B → 26.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
```
```
Accessibility (a11y) tests · 6 · 43 · 0 · 0
Coverage report · 180 · 1042 · 0 · 12
Test (windows) · 180 · 1000 · 0 · 9
Test (macos) · 180 · 1042 · 0 · 12
Test [default features] · 199 · 1045 · 0 · 12
Stand checks repeated · 156 · 624 · 0 · 21
```

`… --operator 26 --id 39319403188` (the base run on `f98a4dce1`):

```
 26 probe       recorded · exit 0 · 6.08s · 266 B → 26.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
```
```
Coverage report · 180 · 1042 · 0 · 12
Accessibility (a11y) tests · 6 · 43 · 0 · 0
Test (macos) · 180 · 1042 · 0 · 12
Test (windows) · 180 · 1000 · 0 · 9
Test [default features] · 199 · 1045 · 0 · 12
Stand checks repeated · 156 · 624 · 0 · 21
```

Every per-job tally is unmoved, both against the base run CI#39319403188 and against CI#39312031677 (linux 199 · 1045 ·
0 · 12, windows 180 · 1000 · 0 · 9, stand repeat 156 · 624 · 0 · 21). No count moved, so there is nothing to explain
by name.

## A premise the run falsified: the coverage reading moved
The plan forecast that "the floor reading does not move", and research said "moved lines stay counted" (because
`coverage_floor.py` sums every `SF` record under the crate directory). The run reads otherwise for escher-driver:

| run | escher-driver LH/LF | % |
|---|---|---|
| base CI#39319403188 (`f98a4dce1`) | 5180/5268 | 98.33 |
| this CI#39324662409 (`62403b643`) | 4266/4350 | 98.07 |

The floor holds (97 %), so this is no red. The cause is measured in the coverage report's per-file rows.
- No `*_tests.rs` file appears in either run: neither the new `cli_tests.rs` and `recording_tests.rs`, nor
  2026-10-16-file-splits' `mcp_tests.rs` and `schema_tests.rs`.
- `cli.rs` reads 847 → 437 and `recording.rs` 1303 → 485, while `client.rs` and `mcp.rs` are unchanged.

So cargo-llvm-cov leaves a `*_tests.rs` sibling out of the lcov before `coverage_floor.py` reads it. The moved test
lines, nearly all covered, left the crate's sums (LF −918, LH −914). Nothing that was measured before went
uncovered. The two dioxus-native-dom and blitz-test-harness readings are identical. This belongs to the wrap: the
test-plan coverage text and research's premise.
