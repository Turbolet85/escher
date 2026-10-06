
## 2026-10-06-headless-stand — stand checks, HarnessOptions fields, bundled font, timer seam, workspace re-count
**Section:** §1 Coverage scope (tests/blitz-tests) · §2 Font-dependent tests · §3 blitz-test-harness Construction · Core · Pump semantics · §7 Seed strategies (Font payload) · Builders and options · §8 Hand-written fakes (Events, Time) · Real dependencies kept · §9 Fonts · Local baseline
**Change:**
- §1: tests/blitz-tests covers the seven_guis headless stand (`stand_boot` · `stand_counter` · `stand_flight_booker` · `stand_timer` · `stand_crud`) and the Dioxus falsy-`disabled` clearing (`dioxus_falsy_disabled`).
- §2 / §8 / §9 Fonts: was "font-dependent tests skip with `eprintln!`" and "rely on `system-fonts`"; the skip and the dependency now cover the pre-existing tests only — stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, in the package-alone build too.
- §3 Construction: the stand boot `seven_guis::stand::{boot, boot_timer}` with `stand::options(incremental)` is the stand checks' drive surface.
- §7: HarnessOptions was six fields; now eight (`font_ctx`, `incremental`); `stand::options` builds the pinned literal; the Font payload row adds `seven_guis::DEJAVU_SANS`.
- §8 Time: `TimerTicks` is the stand's time stand-in (one 0.1 s step per delivered tick, no `Delay`).
- §9 Local baseline: `cargo test --workspace` re-counted 430 passed · 0 failed · 4 ignored, 120 result lines (was 416 · 0 · 4, 114); the blitz-tests note adds 6 files / +14 tests, not re-measured per crate.
- 6 `harness.rs` citations re-pointed by the two new HarnessOptions fields' line shift.
**Why:** the headless stand chunk wrote 13 stand checks and 1 regression test and gave the harness a font and layout-mode option.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/
