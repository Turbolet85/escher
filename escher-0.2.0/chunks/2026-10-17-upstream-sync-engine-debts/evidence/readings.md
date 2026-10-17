# Readings — what the merged engine moved

Gate run: `.andromeda/runs/2026-10-17T14-34-19-implement/gate-2026-10-17-upstream-sync-engine-debts.json`.

## Coverage (entry 40, `bash .github/scripts/ci-leg.sh coverage`, `keyboard_compat.rs` excluded by filename)
| Crate | Before (2026-10-17T14:29Z) | After (this run) | Floor |
|---|---|---|---|
| escher-driver | 4014/4088 = 98.19 % | 4014/4088 = 98.19 % | 97 % |
| dioxus-native-dom | 2472/2934 = 84.25 % | 2472/2934 = 84.25 % | 83 % |
| blitz-test-harness | 916/999 = 91.69 % | 916/999 = 91.69 % | 90 % |

Last line: `coverage floor: 3 of 3 crates at or above their floors`. The lcov holds no `keyboard_compat` record
(entry 41: `grep -c -F keyboard_compat target/coverage/lcov.info` → 0, exit 1). dioxus-native-dom's ten `SF` files
in the lcov are the pre-merge set; upstream's `events.rs` edit (+10 −9) leaves its instrumented line count at 466
and its hit count unchanged, so the crate's figure is byte-equal to the baseline. The plan predicted "near 84.25 %".

## Stand geometry and sizes
No figure a standing check asserts moved: `run stand` (168 · 0 · 7) and `run all` passed, each check in both layout
modes, so —
- `crud-list` reads (24, 112.796875, 369 × 419.203125) (`stand_act_screenshot.rs:130` green);
- every lean screen's snapshot text stays under its ceiling (Counter 1024 · FlightBooker 1536 · Timer 1536 · Crud
  2304; `stand_snapshot_text::every_screen_fits_the_budget` green) and under `SNAPSHOT_TEXT_BUDGET` 10,000;
- `stand_screenshot_repeat` green: the whole-screen shot an 800 × 600 PNG, equal to the harness raster and
  byte-identical across processes;
- `stand_contrast` green in both layout modes (also in the a11y leg, 6 · 43 · 0);
- `stand_keyboard`, `range_input_interaction` and `stand_act_keys` green, `tab_order`, `ENABLED_TAB_ORDER` and the
  Enter/Space rows unchanged (no expected table was edited — step 6 is a spelling change).

`task-body` (0, 46, 800 × 554) is pinned by no check's literal (`evidence/before.md`); it was not re-measured here.
#1198 (baseline-source for atomic inlines) and the inline-layout edits moved no asserted stand number.

## Windowed boot (entry 43, the smoke)
`RUST_LOG=info timeout 10 target/debug/seven_guis_native` stayed up for the 10 s bound (`test $? -eq 124` → 0) and
logged `INFO escher_telemetry service.name=seven_guis service.version=0.1.0 message=telemetry installed` at
2026-10-17T14:50:20Z.
