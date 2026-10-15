# The coverage floor — step 4

## The leg, with its floor

`bash .github/scripts/ci-leg.sh coverage` — gate entry 8 of implement's first block pass, 2026-10-15: exit 0,
127.4 s warm (`target/llvm-cov-target` reused, no clean). `target/coverage/lcov.info` written 2026-10-15 05:30:17Z,
after the pass began. The leg's last four lines:

```
packages/escher-driver 4784/4865 = 98.34 % (floor 97 %)
packages/dioxus-native-dom 2472/2934 = 84.25 % (floor 83 %)
packages/blitz-test-harness 916/999 = 91.69 % (floor 90 %)
coverage floor: 3 of 3 crates at or above their floors
```

The report's TOTAL row: 46 686 lines, 15 377 missed, **67.06 %** (regions 66.73 %, functions 67.93 %).

## The control — a known-fail input, on the same lcov

| run | last line | exit |
|---|---|---|
| `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=99 packages/dioxus-native-dom=83 packages/blitz-test-harness=90` | `coverage floor: 1 of 3 crates under their floors` (its first line names `packages/escher-driver 4784/4865 = 98.34 % (floor 99 %)`) | 1 |
| the same with the committed floors (`=97`, `=83`, `=90`) | `coverage floor: 3 of 3 crates at or above their floors` | 0 |

## Every crate the leg reads (lines · hit · %, summed from `lcov.info` per crate directory)

| crate directory | lines | hit | % |
|---|---|---|---|
| `apps/browser` | 1911 | 749 | 39.19 |
| `apps/bump` | 59 | 0 | 0.00 |
| `apps/readme` | 314 | 0 | 0.00 |
| `packages/accesskit_xplat` | 73 | 0 | 0.00 |
| `packages/blitz` | 70 | 0 | 0.00 |
| `packages/blitz-dom` | 18998 | 14842 | 78.12 |
| `packages/blitz-html` | 248 | 192 | 77.42 |
| `packages/blitz-net` | 237 | 0 | 0.00 |
| `packages/blitz-paint` | 4351 | 1865 | 42.86 |
| `packages/blitz-shell` | 1451 | 0 | 0.00 |
| `packages/blitz-test-harness` | 999 | 916 | **91.69** (floor 90) |
| `packages/blitz-traits` | 523 | 304 | 58.13 |
| `packages/blitz-vibey-script` | 5202 | 3032 | 58.29 |
| `packages/debug_timer` | 5 | 5 | 100.00 |
| `packages/dioxus-native` | 462 | 0 | 0.00 |
| `packages/dioxus-native-dom` | 2934 | 2472 | **84.25** (floor 83) |
| `packages/escher-driver` | 4865 | 4784 | **98.34** (floor 97) |
| `packages/escher-telemetry` | 426 | 411 | 96.48 |
| `packages/stylo_taffy` | 1309 | 999 | 76.32 |
| `wpt` | 2249 | 738 | 32.81 |
| **total** | 46686 | 31309 | 67.06 |

## Against HEAD and CI#38649787488

At HEAD (`eb5ba90c`) the three crates read 4750/4837 = 98.20 %, 2472/2934 = 84.25 % and 916/999 = 91.69 %, on the
host and in CI#38649787488's `Coverage report` job alike, to the line (research.md, Readings); each floor is one
point under its HEAD reading. This tree moves only escher-driver: +28 lines, +34 hit (the answer-bound unit test
in `client.rs` reaches `exchange`'s timed-out read), 98.20 → 98.34 %. dioxus-native-dom and blitz-test-harness read
as at HEAD.
