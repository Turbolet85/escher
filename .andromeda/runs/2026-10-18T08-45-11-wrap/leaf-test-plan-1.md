group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»

- leaf: .claude/docs/tests-summary.md:13   class: omission
  leaf-text: «construct (`from_html`, `from_component`, `from_vdom`; `HarnessOptions` also takes `font_ctx` and `incremental`), `pump`/`tick`, `settle`»
  source: .andromeda/test-plan.md:89 «`tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(Duration)` advances it by exactly the duration, each then pumping»
  replace-with: «construct (`from_html`, `from_component`, `from_vdom`; `HarnessOptions` also takes `font_ctx` and `incremental`), `pump`/`tick`/`advance_time` (the harness's time a `VirtualClock` its harness-built documents read — `tick(dt)` advancing it, a negative or non-finite `dt` moving nothing, `advance_time(Duration)` by exactly the duration, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never moving it; `wrap(doc)` keeping the clock the document's own config gave it; PROVISIONAL), `settle`»

- leaf: .claude/docs/tests-summary.md:14   class: stale
  leaf-text: «a driver session moves it by `advance`, through `stand::timer_step` handed to the session with `Session::with_time` — whole ticks of `TIMER_TICK_MS` (100), the remainder dropped per call, `advanced_ms` the time delivered and never more than asked, the harness clock untouched.»
  source: .andromeda/test-plan.md:287 «it returns nothing and delivers the whole 100 ms ticks (`TIMER_TICK_MS`) the asks hold, carrying the remainder below one tick across calls (250 then 250 → 2 then 3 ticks)»
  replace-with: «a driver session moves it by `advance`, which moves the session's clock — the harness's `VirtualClock` — by exactly `ms` inside one settled step, then hands `ms` to `stand::timer_step`, the step the Timer's session carries through `Session::with_time` — whole ticks of `TIMER_TICK_MS` (100), the remainder below one tick carried across calls (250 then 250 → 2 then 3 ticks), `advanced_ms` exactly `ms`, the clock's move (PROVISIONAL).»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the open dialog's controls each reading an author key)»
  source: .andromeda/test-plan.md:27 «`virtual_clock_engine` 4, an engine check on a parsed page, in both layout modes: the scrollbar fade moves only with the clock»
  replace-with: «the open dialog's controls each reading an author key), since 2026-10-18-one-virtual-clock `virtual_clock_engine` (4: an engine check on a parsed page, both layout modes — the scrollbar fade moving only with the clock, two clicks a double click by the clock alone, a smooth scroll standing still until the clock moves, a document built with no clock fading by the wall clock) and `stand_virtual_clock` (2, through `Session::run`, both layout modes: a shot after a scroll repeating across an 800 ms sleep until `advance` moves the fade, two clicks with no `advance` between them a double click; PROVISIONAL)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`refusal.rs` 3 (the nine causes, `fonts-unavailable` last since 2026-10-12-headless-screenshot,»
  source: .andromeda/test-plan.md:22 «refusal.rs 3 (the causes are named in order — eight (`the_eight_causes_are_named_in_order`, PROVISIONAL), `fonts-unavailable` last»
  replace-with: «`refusal.rs` 3 (the eight causes (`the_eight_causes_are_named_in_order`, PROVISIONAL), `fonts-unavailable` last,»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 41 `stand_*.rs` files»
  source: .andromeda/test-plan.md:318 «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 42 files —»
  replace-with: «three rounds of the 42 `stand_*.rs` files»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 991 · 0 · 12 over 192 result lines at 2026-10-18-upstream-sync-determinism-and-traces (from 187 · 971:»
  source: .andromeda/test-plan.md:334 «re-counted at 2026-10-18-one-virtual-clock: 194 result lines, 1001 passed · 0 failed · 12 ignored, from 192 · 991»
  replace-with: «`cargo test --workspace` 1001 · 0 · 12 over 194 result lines at 2026-10-18-one-virtual-clock (from 192 · 991: +2 result lines — `virtual_clock_engine`, `stand_virtual_clock` — and +10 passed — blitz-dom's lib +3 in `clock.rs`, `virtual_clock_engine` +4, `stand_virtual_clock` +2, seven_guis +1; timings not re-measured); 991 · 0 · 12 over 192 result lines at 2026-10-18-upstream-sync-determinism-and-traces (from 187 · 971:»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «the 41 `stand_*.rs` files and seven_guis' seven process checks run three times»
  source: .andromeda/test-plan.md:347 «run the 42 `stand_*.rs` files and seven_guis' seven process checks three times with no retry»
  replace-with: «the 42 `stand_*.rs` files and seven_guis' seven process checks run three times»

- leaf: .claude/rules/testing.md:15   class: stale
  leaf-text: «`stand_mcp_tools`, `stand_screenshot_repeat` and `stand_requirement_sweep` (22 readers;»
  source: .andromeda/test-plan.md:102 «`stand_requirement_sweep`, and `stand_virtual_clock`, none a `stand_act_*` file (23 readers;»
  replace-with: «`stand_mcp_tools`, `stand_screenshot_repeat`, `stand_requirement_sweep` and `stand_virtual_clock` (23 readers;»

- leaf: .claude/rules/testing.md:30   class: stale
  leaf-text: «no sleep (drive the timer through its `TimerTicks` handle)»
  source: .andromeda/test-plan.md:183 «with no sleep and no wall-clock read in the checks — bar `stand_virtual_clock`'s one recorded 800 ms sleep»
  replace-with: «no sleep (drive the timer through its `TimerTicks` handle; the one exception `stand_virtual_clock`'s recorded 800 ms sleep, taken so a shot after a scroll is read to repeat across wall time until `advance` moves the fade)»

- leaf: .claude/rules/testing.md:31   class: omission
  leaf-text: «- A harness check that needs a font or a layout mode sets `HarnessOptions.font_ctx` / `.incremental`.»
  source: .andromeda/test-plan.md:258 «so documents built by `from_html_with` and `from_vdom` — and through them `from_html` and `from_component` — read the harness's time»
  replace-with: «- A harness check that needs a font or a layout mode sets `HarnessOptions.font_ctx` / `.incremental`. A harness-built document reads the harness's `VirtualClock`: move its time with `tick(dt)` or `advance_time(Duration)` (`screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it), while `wrap(doc)` gives the harness a fresh clock and the wrapped document keeps the one its own config gave it (PROVISIONAL).»
