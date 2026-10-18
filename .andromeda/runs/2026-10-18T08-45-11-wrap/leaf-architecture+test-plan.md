group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: omission
  leaf-text: «Constructors pump once; `wrap` does not. `pump` polls with no waker and resolves at harness time;»
  source: .andromeda/architecture.md:169 «The harness's time (PROVISIONAL): the harness holds a `VirtualClock` its documents share — `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`»
  replace-with: «Constructors pump once; `wrap` does not. The harness's time (PROVISIONAL, since 2026-10-18-one-virtual-clock) is a `VirtualClock` its documents share: `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`, so `from_html_with` and `from_vdom`, and through them `from_html` and `from_component`, build documents that read it, while `wrap(doc)` gives the harness a fresh `VirtualClock` and the wrapped document keeps the clock its own config gave it; `time()` reads that clock in seconds; `tick(dt)` advances it, a negative or non-finite `dt` moving nothing, and `advance_time(&mut self, by: Duration)` advances it by exactly `by`, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it. `pump` polls with no waker and resolves at harness time;»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «and by escher-driver, whose `screenshot` verb's frame is the harness's `screenshot` (since 2026-10-12-headless-screenshot).»
  source: .andromeda/architecture.md:175 «`advance` moves the harness's clock by exactly `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed»
  replace-with: «and by escher-driver, whose `screenshot` verb's frame is the harness's `screenshot` (since 2026-10-12-headless-screenshot) and whose `advance` moves the harness's clock by exactly `ms` through `Harness::advance_time` inside one settled step, then hands `ms` to the session's time step where one was handed (PROVISIONAL, since 2026-10-18-one-virtual-clock).»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: omission
  leaf-text: «(`packages/blitz-test-harness=90` through `coverage_floor.py`; read 91.69 % at that chunk).»
  source: .andromeda/test-plan.md:27 «two clicks are a double click by the clock alone (a 499 ms boundary row; its document driven through `Harness::wrap`)»
  replace-with: «(`packages/blitz-test-harness=90` through `coverage_floor.py`; read 91.69 % at that chunk); since 2026-10-18-one-virtual-clock the harness's time is read by `cargo test -p blitz-tests --test virtual_clock_engine` (4 tests, both layout modes: the scrollbar fade moving only with the clock, two clicks a double click by the clock alone — its document driven through `Harness::wrap` — a smooth scroll standing still until the clock moves, and a document built with no clock fading by the wall clock) and `cargo test -p blitz-tests --test stand_virtual_clock` (2 tests through `Session::run`, both layout modes, reading `harness().time()`) (PROVISIONAL).»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_requirement_sweep` · `cargo test -p seven_guis --locked --lib stand::` · `cargo test -p blitz-tests --locked --test password_painted_masked` — the driver's calls run on a held stand session (`Session::run`, both layout modes; since 2026-10-12-headless-screenshot the `screenshot` verb, and a shot of one state byte-identical in two child runs and the parent)»
  source: .andromeda/test-plan.md:27 «`stand_virtual_clock` 2, a session check through `Session::run`, in both layout modes: a shot after a scroll repeats across an 800 ms sleep until `advance` moves the fade»
  replace-with: «--test stand_requirement_sweep --test stand_virtual_clock` · `cargo test -p seven_guis --locked --lib stand::` · `cargo test -p blitz-tests --locked --test password_painted_masked` — the driver's calls run on a held stand session (`Session::run`, both layout modes; since 2026-10-12-headless-screenshot the `screenshot` verb, and a shot of one state byte-identical in two child runs and the parent; since 2026-10-18-one-virtual-clock `stand_virtual_clock`, PROVISIONAL: a shot after a scroll repeating across an 800 ms sleep until `advance` moves the fade, and two clicks with no `advance` between them a double click)»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «the measured `off-screen` limit, both layout modes»
  source: .andromeda/test-plan.md:27 «`virtual_clock_engine` 4, an engine check on a parsed page, in both layout modes: the scrollbar fade moves only with the clock»
  replace-with: «the measured `off-screen` limit, both layout modes; and `cargo test -p blitz-tests --locked --test virtual_clock_engine` — the document's clock (PROVISIONAL, since 2026-10-18-one-virtual-clock), an engine check on a parsed page, both layout modes: the scrollbar fade moving only with the clock, two clicks a double click by the clock alone, a smooth scroll standing still until the clock moves and ending at 300 ms, and a document built with no clock fading by the wall clock (not selected by `run stand`)»
