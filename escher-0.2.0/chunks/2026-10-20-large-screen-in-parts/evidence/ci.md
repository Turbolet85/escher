# CI evidence — 2026-10-20-large-screen-in-parts (operator pass)

Pushed sha `191e66fdbe7f7521fcc0d67561b44e13101fe8ee`, the pre-CI commit
`chore(2026-10-20-large-screen-in-parts): operator pre-CI commit, for the run this chunk's verdict reads`, on
`build/escher-0.2.0`. Every entry below was fired in plan order and none was red.

## Entry 18 — hygiene, fired as written (before the pre-CI commit)
```
hygiene: clean — read 50 (runs 45 · evidence 0 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Entry 19 — fast leg, clean-tree guard, push (`gate.py run --operator 19`)
```
operator entry 19 · history tripwire: git
 19 probe       green · exit 0 · 103.75s · 113463 B → 19.log · history moved: refs/remotes/origin/build/escher-0.2.0 c1a2e5b8→191e66fd · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```
The history move is the intended push: `c1a2e5b85..191e66fdb  build/escher-0.2.0 -> build/escher-0.2.0` (read from
its log).

## Entry 20 — ci.py conclusion, fired as written
```
191e66fdbe7f verdict: green · checks 17/17 · wall 1105 s · runs CI#39410865604 completed/success
runs: CI#39410865604 push completed/success
```
The coverage job (per-crate floors) and the stand-repeat job (three rounds, zero-failure budget) are among the 17 green
checks.

## Entry 21 — per-OS tallies (`gate.py run --operator 21 --id 39410865604`)
```
 21 probe       recorded · exit 0 · 6.21s · 266 B → 21.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 21 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 20
```

| job | CI#39387656399 (previous chunk) | CI#39410865604 (this chunk) | move |
|---|---|---|---|
| Test [default features] | 204 · 1087 · 0 · 12 | 205 · 1098 · 0 · 12 | +1 line · +11 |
| Test (macos) | 185 · 1084 · 0 · 12 | 186 · 1095 · 0 · 12 | +1 line · +11 |
| Coverage report | 185 · 1084 · 0 · 12 | 186 · 1095 · 0 · 12 | +1 line · +11 |
| Test (windows) | 185 · 1041 · 0 · 9 | 186 · 1051 · 0 · 9 | +1 line · +10 |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | unmoved |
| Stand checks repeated | 165 · 672 · 0 · 21 | 168 · 684 · 0 · 21 | +3 lines · +12 |

(lines · passed · failed · ignored)

How each move breaks down, by name:
- **+11 (default, macos, coverage):**
  - dioxus-native-dom lib +6, the `text_parts` units;
  - escher-driver +1, `mcp::tests::the_snapshot_tool_takes_an_optional_part_and_names_its_two_counts`;
  - the new target `stand_snapshot_parts` +3, which is also the +1 line;
  - seven_guis `cli_commands` +1, `snapshot_reads_a_part_by_number_and_a_part_outside_its_bound_is_refused`.
- **+10 (windows):** the same, less the `cli_commands` test. That file is `#![cfg(unix)]`, so it doesn't run on Windows.
- **+12 (stand checks repeated):** per-target diff of the two job logs:
  - `tests/stand_snapshot_parts.rs` 0 → 9 (3 tests × 3 rounds), which is also the +3 lines;
  - `tests/cli_commands.rs` 39 → 42 (1 new test × 3 rounds).
  - `stand_act_spans` gains no test: its added part read sits inside an existing test.
- **a11y:** unmoved, 6 · 6 · 4 · 8 · 5 · 14 = 43 locally as well.
