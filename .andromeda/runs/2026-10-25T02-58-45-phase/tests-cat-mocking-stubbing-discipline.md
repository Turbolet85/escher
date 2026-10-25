# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) The statement's account of `advance` and the one virtual clock must match the stated time model: the session's clock moves by exactly `ms` in one settled step, the Timer's step delivers whole 100 ms ticks, and `advanced_ms` reports the clock's move, not what the app's counter gained.
  > the driver moves that time by `advance`, which moves the session's clock — the harness's `VirtualClock`, the clock every harness-built document reads — by exactly `ms` inside one settled step, then hands `ms` to the step: `seven_guis::stand::timer_step(ticks)` is the step the Timer's session carries through `Session::with_time` — it returns nothing and delivers the whole 100 ms ticks (`TIMER_TICK_MS`) the asks hold, carrying the remainder below one tick across calls (250 then 250 → 2 then 3 ticks); `advanced_ms` is exactly `ms`, the clock's move (ratified by the founder, 2026-10-18), not what the Timer's counter gained — 60000 is reported while the elapsed display stops at its 15 s duration
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) The statement must not claim that settle advances time or waits on an app timer: settle moves no timer and waits on no tick that has not been delivered.
  > settle neither advances the timer nor waits on a tick not yet delivered; an undelivered source holds elapsed at 0.0 s
- (.andromeda/test-plan.md:352) The body's broad line on time that the statement's `advance` claim must stay consistent with.
  > the driver's `advance` moves the session's clock by exactly `ms` inside one settled step, and `advanced_ms` is exactly `ms`, the clock's move, not what the Timer's counter gained
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:15) If the statement claims the load wait, it must state that reach as proven against network stand-ins, a held one and a self-answering one, not against a real fetch.
  > `fetch` answers each request from its own thread after a 200 ms sleep, so a load is in flight when the step settles and is answered inside `LOAD_WAIT`; the never-answered arm uses the shared `HeldNet`
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:14) The busy-class reading `Loads` is proven through the held network stand-in with no real fetch, and the statement must not describe it as measured on a live network.
  > `stand_settle` reads `NotSettled { busy: Loads }` through it, answering a head stylesheet and then its `@import` on `.test` URLs with no real fetch
- (.andromeda/test-plan.md:358) The statement's proven-on claims (the stand's screenshot, contrast and lint readings) must state that they were measured under the pinned bundled font with system fonts off.
  > The headless-stand checks keep a real but pinned font — the bundled DejaVu Sans woff2, decoded through seven_guis' native `dioxus-native` `woff` feature, with system fonts off and no skip

## Bindings
- **Time stand-in (`TimerTicks`, `advance`) ↔ the body §3 Harness construction**: the `advance` semantics the statement gives must agree with the detail row and with the body's statement of `stand::timer_step` and the session clock (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:26) ↔ (.andromeda/test-plan.md:130)
- **Network stand-ins (`HeldNet`, `LateNet`) ↔ the body §5 Wall time in the session checks**: the load wait is measured through `LateNet`'s 200 ms sleep, the one wall-time exception besides `stand_virtual_clock` (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:15) ↔ (.andromeda/test-plan.md:259)
- **Pinned real font ↔ the body §9 Fonts and §2 Font-dependent tests**: the stand's readings hold on any runner because they use the bundled font alone, and a document with no `sans-serif` family is refused `fonts-unavailable` (.andromeda/test-plan.md:358) ↔ (.andromeda/test-plan.md:387) ↔ (.andromeda/test-plan.md:94)
