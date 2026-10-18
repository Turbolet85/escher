group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/escher-driver.md:16   class: stale
  leaf-text: «a closure handed the held harness and the milliseconds an `advance` asks for, returning the milliseconds the app moved.»
  source: .andromeda/architecture.md:175 «it moves the app's own time and returns nothing, the session itself moving the harness's clock on every `advance` (PROVISIONAL)»
  replace-with: «a closure handed the held harness and the milliseconds an `advance` asks for, moving the app's own time and returning nothing, the session itself moving the harness's clock on every `advance` (PROVISIONAL).»

- leaf: .claude/docs/services/escher-driver.md:18   class: stale
  leaf-text: «`advance` hands its milliseconds to the session's time step — `time-unavailable` when the session carries none;»
  source: .andromeda/architecture.md:175 «`advance` moves the harness's clock by exactly `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed»
  replace-with: «`advance` moves the harness's clock by exactly its milliseconds (`Harness::advance_time`) inside one settled step on every session — the engine's animations, the overlay scrollbar fade and the app's own time together — then hands them to the session's time step where one was handed (PROVISIONAL);»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll;»
  source: .andromeda/architecture.md:175 «two `click`s on one point with no `advance` of at least 500 ms between them read as a double click, as `click`'s help says (PROVISIONAL)»
  replace-with: «Otherwise they click the centre of the element's snapshot `bounds` plus the viewport's scroll — two `click`s on one point with no `advance` of at least 500 ms between them read as a double click, as `click`'s help says (PROVISIONAL);»

- leaf: .claude/docs/services/escher-driver.md:19   class: stale
  leaf-text: «`advanced_ms` is the lesser of the step's return and the `ms` asked: the time delivered to the app, not what a counter inside it gained.»
  source: .andromeda/architecture.md:175 «`advanced_ms` is exactly `ms` — the clock's move, not what a counter inside the app gained, and a step's diff can be non-empty on any animated page (PROVISIONAL)»
  replace-with: «`advanced_ms` is exactly the `ms` asked: the clock's move, not what a counter inside the app gained, and a step's diff can be non-empty on any animated page (PROVISIONAL).»

- leaf: .claude/docs/services/escher-driver.md:24   class: stale
  leaf-text: «its help stating two measured limits — an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time)»
  source: .andromeda/architecture.md:175 «whose help states one measured limit — an element shot is cropped to the part of its bounds that can be seen — the overlay scrollbar fade reading the session's clock»
  replace-with: «its help stating one measured limit — an element shot is cropped to the part of its bounds that can be seen — the overlay scrollbar fade reading the session's clock, so two shots of one state with no `advance` between them read the same pixels (PROVISIONAL))»

- leaf: .claude/docs/services/escher-driver.md:26   class: retired
  leaf-text: «`CAUSES` is closed at nine: `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `time-unavailable` · `fonts-unavailable` (the ninth since 2026-10-12-headless-screenshot:»
  source: .andromeda/architecture.md:175 «`Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, PROVISIONAL)»
  replace-with: «`CAUSES` is closed at eight (`[Cause; 8]`, PROVISIONAL): `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable` (the eighth, since 2026-10-12-headless-screenshot:»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «All nine are returned today — `unknown-verb` and `malformed` by `validate`, the other seven by the executor,»
  source: .andromeda/architecture.md:175 «All eight are returned by code: `validate` returns `unknown-verb` and `malformed` only, and the executor returns the other six»
  replace-with: «All eight are returned today — `unknown-verb` and `malformed` by `validate`, the other six by the executor,»

- leaf: .claude/docs/services/escher-driver.md:35   class: stale
  leaf-text: «`screenshot`'s help, two: an element shot is cropped to the part of its bounds that can be seen, and in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time;»
  source: .andromeda/architecture.md:175 «whose help states one measured limit — an element shot is cropped to the part of its bounds that can be seen — the overlay scrollbar fade reading the session's clock»
  replace-with: «`screenshot`'s help, one: an element shot is cropped to the part of its bounds that can be seen — the overlay scrollbar fade reading the session's clock, so two shots of one state with no `advance` between them read the same pixels (PROVISIONAL);»

- leaf: .claude/docs/services/escher-driver.md:38   class: stale
  leaf-text: «time on a held instance moves only by `advance`, through the caller's step;»
  source: .andromeda/architecture.md:175 «`advance` moves the session's clock forward by `ms` milliseconds on every session»
  replace-with: «time on a held instance moves only by `advance`, which moves the session's clock on every session and, through the caller's step where one was handed, the app's own time (PROVISIONAL);»

- leaf: .claude/docs/services/escher-driver.md:61   class: retired
  leaf-text: «`stand_act_timer` (`advance`, the settled Timer flow, `time-unavailable`, the clamp)»
  source: .andromeda/architecture.md:305 «time moved by `advance`, a refused call changing nothing»
  replace-with: «`stand_act_timer` (time moved by `advance`, the settled Timer flow)»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «the other three carry none).»
  source: .andromeda/architecture.md:180 «the other three tasks' carrying no time step and still taking `advance`, which moves their session's clock alone (PROVISIONAL)»
  replace-with: «the other three carry none and still take `advance`, which moves their session's clock alone (PROVISIONAL)).»

- leaf: .claude/docs/services/seven_guis.md:19   class: stale
  leaf-text: «it delivers the whole ticks the milliseconds asked hold, returns the milliseconds those ticks stand for, drops the remainder per call and leaves the harness's animation clock alone.»
  source: .andromeda/architecture.md:173 «carrying the remainder below one tick across calls (250 then 250 deliver 2 then 3 ticks), and returns nothing — the session, not the step, moves the harness's clock (PROVISIONAL)»
  replace-with: «it delivers the whole ticks the milliseconds asked hold, carrying the remainder below one tick across calls (250 then 250 deliver 2 then 3 ticks), and returns nothing — the session, not the step, moves the harness's clock (PROVISIONAL).»
