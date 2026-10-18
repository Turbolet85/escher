proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Session ↔ held instance
    change: >-
      The refused-call clause reads "any of the eight causes: `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `off-screen`, `covered` and, since 2026-10-12-headless-screenshot, `fonts-unavailable`", and adds that since 2026-10-18-one-virtual-clock `time-unavailable` is retired and `advance` is accepted on every session (PROVISIONAL, the delegate operator's answers for the founder's epoch batch).
    sidecar: >-
      test-plan §5 Session ↔ held instance: the refusal set closes at eight causes, `time-unavailable` retired, `advance` accepted on every session (PROVISIONAL).
    rationale: >-
      Report Changes, Schema / config: "The driver's refusal set closes at eight causes … `describe()` lists eight"; Symbols / APIs: "`Cause::TimeUnavailable` is removed. `CAUSES: [Cause; 8]`" and `advance` "is accepted on every session"; Counts item 1 lists test-plan.md:181 under `causes-nine` and `time-unavail`; Addendum inputs#I4 (a) marks the eight causes and `advance` on every session PROVISIONAL.
    basis: packages/escher-driver/src/refusal.rs:208
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → escher-driver
    change: >-
      refusal.rs 3 reads "the causes are named in order — eight since 2026-10-18-one-virtual-clock (`the_eight_causes_are_named_in_order`), `time-unavailable` retired, `fonts-unavailable` last (nine from 2026-10-12-headless-screenshot until then)"; the crate's count of 74 stays.
    sidecar: >-
      test-plan §1 escher-driver: refusal.rs pins eight causes, `the_eight_causes_are_named_in_order`.
    rationale: >-
      Report Symbols / APIs, unit-test pins: "`refusal.rs`: `the_eight_causes_are_named_in_order` (208)"; Counts item 16: escher-driver unit tests stay 74. This §1 site ("nine since 2026-10-12-headless-screenshot, `fonts-unavailable` last") restates the nine-cause claim and is not among the report's `causes-nine` rows.
    basis: packages/escher-driver/src/refusal.rs:208
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests (stand_act_spans)
    change: >-
      `stand_act_spans`' screenshot clause reads "a refused shot's line, `cause=fonts-unavailable`, the eighth cause since 2026-10-18-one-virtual-clock retired `time-unavailable` (the ninth until then)", and adds that since that chunk a CRUD `advance` is accepted, its `command` line read with only the schema fields (the row `Expect::Acts`), each of the eight causes read back from its own refused call.
    sidecar: >-
      test-plan §1 stand_act_spans: fonts-unavailable the eighth cause; the CRUD `advance` row now an accepted line.
    rationale: >-
      Report Counts item 1 lists test-plan.md:26 @28374 ("the ninth cause") as changed; Outcome (security): "`stand_act_spans` reads each of the eight back from its own refused call"; Outcome (obs): "An accepted `advance` leaves one `command` line with only the schema fields (`stand_act_spans`, the CRUD `advance` row now `Expect::Acts`)".
    basis: tests/blitz-tests/tests/stand_act_spans.rs:461-462
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests (stand_mcp_tools)
    change: >-
      `stand_mcp_tools`' cause clause reads "every cause a tool call reaches is served with its remedy — seven through MCP, all but `unknown-verb`, since 2026-10-18-one-virtual-clock retired `time-unavailable` (eight from 2026-10-12-headless-screenshot until then, seven before)", and adds that since that chunk an accepted `advance` tool call reads `tool_call` `ending` = `accepted` with one child `command` whose `verb` is `advance` and which records no `cause`.
    sidecar: >-
      test-plan §1 stand_mcp_tools: seven causes served over MCP; the accepted `advance`'s `tool_call` span read.
    rationale: >-
      Report Outcome (security): "`stand_mcp_tools` serves seven through MCP, all but `unknown-verb`"; Deviations item 10: "`stand_mcp_tools` now reads the accepted `advance`'s `tool_call` span: `ending` = `accepted`, and one child `command` whose `verb` is `advance` and which records no `cause`". The site's "eight since 2026-10-12-headless-screenshot" restates the retired count and is not among the report's `causes-nine` rows (it reads "eight").
    basis: tests/blitz-tests/tests/stand_mcp_tools.rs:481-521
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover (escher-driver)
    change: >-
      refusal.rs pins the closed cause set — eight causes in a stated order (`fonts-unavailable` the eighth since 2026-10-18-one-virtual-clock retired `time-unavailable`; the ninth from 2026-10-12-headless-screenshot until then), each name, meaning and remedy a fixed text against a stated table (PROVISIONAL).
    sidecar: >-
      test-plan §4 escher-driver: refusal.rs pins eight causes.
    rationale: >-
      Report Counts item 1 lists test-plan.md:143 under `causes-nine`; Symbols / APIs: "`CAUSES: [Cause; 8]` keeps the old order less that one".
    basis: packages/escher-driver/src/refusal.rs:208
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Command line (`escher-session`)
    change: >-
      The cause list reads "every cause the binary provokes (`unknown-verb`, `malformed`, `disabled`, `not-found`, `off-screen`, `stale`) reads `remedy` equal to its cause's", `time-unavailable` retired since 2026-10-18-one-virtual-clock (PROVISIONAL), and `host_timer`'s hosted `advance` reports exactly what was asked.
    sidecar: >-
      test-plan §6 Command line: `time-unavailable` leaves the causes `cli_commands` provokes.
    rationale: >-
      Report Counts item 1 lists test-plan.md:200 under `time-unavail` and item 3 under `cli-advance`; Files: `examples/seven_guis/tests/cli_commands.rs` modified; Outcome (tests): "`stand_act_timer`, `host_timer` and `cli_commands` pass, with `advance` moving the clock by exactly what was asked".
    basis: examples/seven_guis/tests/cli_commands.rs:401
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §8 Mocking & Stubbing Discipline → Time (seven_guis timer)
    change: >-
      The driver clause reads: every `advance` of `ms` moves the session's clock — the harness's `VirtualClock`, the clock every harness-built document reads — by exactly `ms` inside one settled step, then hands `ms` to the step where one was handed; `seven_guis::stand::timer_step(ticks)` is the Timer's step through `Session::with_time`, returns nothing and delivers whole 100 ms ticks (`TIMER_TICK_MS`) carrying the sub-tick remainder across calls (250 then 250 → 2 then 3 ticks); `advanced_ms` is exactly `ms`, the clock's move, not what the Timer's counter gained — 60000 is reported while the elapsed display stops at its 15 s duration (PROVISIONAL).
    sidecar: >-
      test-plan §8 Time: `advance` moves the one clock by exactly `ms`; `timer_step` carries its remainder and returns nothing; `advanced_ms` = `ms`.
    rationale: >-
      Report Symbols / APIs: `advance` "calls `harness.advance_time(Duration::from_millis(ms))`, then the session's time step where one was handed, and reports `advanced_ms = ms`"; `timer_step` "carries the sub-tick remainder across calls, so 250 then 250 deliver 2 then 3 ticks, and returns nothing"; Counts items 2, 3, 4 and 6 list test-plan.md:284 as changed ("never more than asked (50 → 0", "drops the remainder", "leaves the harness's animation clock where it was").
    basis: packages/escher-driver/src/execute.rs:181-184
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests (stand_act_timer)
    change: >-
      `stand_act_timer` reads: Reset then `advance` 300 returns settled with `advanced_ms` 300 and `Elapsed: 0.3s` in the returned diff, every `advance` reporting exactly the `ms` asked and the harness clock moving by exactly that; since 2026-10-18-one-virtual-clock an `advance` on a Counter, FlightBooker or Crud session with no time step moves the clock and settles (`an_advance_on_a_session_with_no_time_step_moves_the_clock_and_settles`), and on the animated fixture `an_advance_reports_exactly_what_was_asked_whatever_the_step_does` — the step handed exactly `ms`, `advanced_ms` = `ms`, the clock moved by exactly `ms`, nothing added or removed, `fx-press` changed exactly when the step pressed it (PROVISIONAL).
    sidecar: >-
      test-plan §1 stand_act_timer: no-step sessions accept `advance`; `advanced_ms` exact; the two renamed tests.
    rationale: >-
      Report Counts item 1 (`time-unavail`), 2 (`advanced-ms`: "50 reports 0, 250 reports 200"), 3 (`with-time` @20952) and 6 (`harness-time` @20874 "the harness clock reading the same before and after every call") list test-plan.md:26 as changed; Counts item 4 names the two stand_act_timer renames; Deviations items 4 and 6 state what the renamed tests assert and that the no-step test runs Counter, FlightBooker and Crud.
    basis: tests/blitz-tests/tests/stand_act_timer.rs:192-206
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → apps (seven_guis library unit tests and host_timer)
    change: >-
      The library's stand test reads `stand::tests::the_timer_step_carries_milliseconds_onto_whole_ticks` (six cumulative rows and two carry rows, renamed from `the_timer_step_maps_milliseconds_onto_whole_ticks` at 2026-10-18-one-virtual-clock) beside a new `the_timer_step_takes_the_largest_advance` (examples/seven_guis/src/stand.rs:161-173); `cargo test -p seven_guis --locked --lib` reads 3 passed (2 until then); `host_timer`'s hosted `advance` reports `advanced_ms` equal to the `ms` asked (250 reads 250, was 200), its elapsed text the whole ticks delivered.
    sidecar: >-
      test-plan §1 seven_guis: the timer-step test renamed to the carry rows, a third library test, `host_timer`'s `advanced_ms` exact.
    rationale: >-
      Report Counts item 4: `renamed-tests` "test-plan.md:10 names `stand::tests::the_timer_step_maps_milliseconds_onto_whole_ticks`, now `the_timer_step_carries_milliseconds_onto_whole_ticks` plus `the_timer_step_takes_the_largest_advance`"; `sg-units`: "`cargo test -p seven_guis --locked --lib`: 3 passed" (the same "reads 2 passed" claim stands at test-plan.md:10, not among `sg-units`' rows); Counts item 2: "test-plan.md:10 (250 → `advanced_ms` 200, now 250)"; Deviations item 5.
    basis: examples/seven_guis/src/stand.rs:161-173
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract → blitz-test-harness (`Harness`) → Construction
    change: >-
      The Timer clause reads `stand::timer_step(TimerTicks)` with `TIMER_TICK_MS` (100), the time step a `Session` takes through `with_time` — returning nothing and carrying its sub-tick remainder across calls, the session moving its clock on every `advance` before handing the step `ms` (§8 Time).
    sidecar: >-
      test-plan §3 Construction: `timer_step` returns nothing and carries its remainder; the session moves the clock on every `advance`.
    rationale: >-
      Report Counts items 3 and 4 list test-plan.md:87 under `with-time` and `timer-step` as changed; Symbols / APIs: "`TimeStep = Box<dyn FnMut(&mut Harness<DioxusDocument>, u32)>`, with no return" and `with_time`'s doc "say the session moves the clock on every `advance`, and the step moves the app's own time".
    basis: packages/escher-driver/src/session.rs:116-125
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle (session-start)
    change: >-
      `with_time(step)` hands the session its caller's time step, the step moving the app's own time; every session accepts `advance`, which moves the session's clock by exactly `ms` and then runs the step where one was handed, `advanced_ms` reading `ms` — `time-unavailable` retired at 2026-10-18-one-virtual-clock; a no-font shot is refused `fonts-unavailable`, the eighth cause (PROVISIONAL).
    sidecar: >-
      test-plan session-lifecycle session-start: `advance` on every session, `time-unavailable` retired, `fonts-unavailable` the eighth cause.
    rationale: >-
      Report Counts items 1 and 3 list session-lifecycle.md:5 under `causes-nine`, `time-unavail`, `with-time` and `cli-advance` as changed; Symbols / APIs: `advance` "is accepted on every session"; `with_time` "and its doc say the session moves the clock on every `advance`".
    basis: packages/escher-driver/src/session.rs:116-125
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 Test Harness Contract → blitz-test-harness (`Harness`) → Core
    change: >-
      Core reads: `time` returns the harness's `VirtualClock` reading in seconds and `pump` resolves at it; `tick(dt)` advances that clock, a negative or non-finite `dt` moving nothing; since 2026-10-18-one-virtual-clock `advance_time(&mut self, Duration)` advances it by exactly the duration, then pumps (packages/blitz-test-harness/src/harness.rs:167-172); `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (PROVISIONAL).
    sidecar: >-
      test-plan §3 Core: the harness's time is a `VirtualClock`; `advance_time` new; `tick` moves nothing on a negative or non-finite step.
    rationale: >-
      Report Symbols / APIs, blitz-test-harness `Harness`: "holds a `VirtualClock` in place of `time: f64`", "`time()` returns the clock's reading in seconds; `pump` resolves at it", "`tick(dt)` advances the clock. A negative or non-finite `dt` moves nothing", "New: `pub fn advance_time(&mut self, by: Duration)` advances by exactly `by`, then pumps (167-172)"; Counts item 6 lists test-plan.md:88 ("the Harness API list gains `advance_time`").
    basis: packages/blitz-test-harness/src/harness.rs:167-172
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Builders and options
    change: >-
      Add: since 2026-10-18-one-virtual-clock `HarnessOptions::into_config(self, &VirtualClock)` puts a clone of the harness's clock in `DocumentConfig.clock`, so documents built by `from_html_with` and `from_vdom` (and through them `from_html` and `from_component`) read the harness's time, while `wrap(doc)` gives the harness a fresh `VirtualClock` and the wrapped document keeps the clock its own config gave it (packages/blitz-test-harness/src/harness.rs:49-51; packages/blitz-test-harness/src/harness.rs:127-133).
    sidecar: >-
      test-plan §7 Builders: harness documents carry the harness's `VirtualClock` in `DocumentConfig.clock`; a wrapped document keeps its own.
    rationale: >-
      Report Symbols / APIs: "`HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock` (49-51, 71)" and "`wrap(doc)` gives the harness a fresh `VirtualClock`, while the wrapped document keeps the clock its own config gave it (127-133)"; this section states what `HarnessOptions` carries into `DocumentConfig` (`font_ctx`, `incremental`) and what `wrap` carries, and names no clock.
    basis: packages/blitz-test-harness/src/harness.rs:49-51
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → Engine features by runner
    change: >-
      The fade clause reads: since 2026-10-18-one-virtual-clock the overlay scrollbar fade reads the document's clock — a harness document's the harness's `VirtualClock` — so a shot repeats until `advance` moves the fade, read by `stand_virtual_clock` across an 800 ms sleep; the `screenshot` help's wall-time sentence is dropped and the limit retired, the residual's move done (PROVISIONAL).
    sidecar: >-
      test-plan §9 Engine features by runner: the fade's wall-clock limit retired; the fade reads the harness clock.
    rationale: >-
      Report Counts item 5: `wall-fade` and `help-text` list test-plan.md:324 ("no check shoots inside that window") as changed, and item 6 lists test-plan.md:324 @4553 under `harness-time`; Symbols / APIs: "`screenshot` drops its sentence 'in a build that paints overlay scrollbars, a scrollbar thumb painted within 0.7 s of a scroll fades by wall time'"; Outcome (tests): `stand_virtual_clock` "S2 == S1 across the 800 ms sleep".
    basis: tests/blitz-tests/tests/stand_virtual_clock.rs:50-96
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Session ↔ held instance
    change: >-
      The clause "with no sleep and no wall-clock read in the checks" adds: bar `stand_virtual_clock`'s one recorded 800 ms sleep, since 2026-10-18-one-virtual-clock, taken on purpose so a shot after a scroll is read to repeat across wall time until `advance` moves the fade.
    sidecar: >-
      test-plan §5: the one recorded sleep in a session check, `stand_virtual_clock`'s 800 ms.
    rationale: >-
      Report Counts item 10: "One recorded 800 ms sleep in a stand check (`stand_virtual_clock`)"; Spec claims disproved: "The new `stand_virtual_clock` sleeps 800 ms on purpose, as the plan's §Constraints records"; Expected amendments item 5 names "§5 (the one recorded sleep)". The site's general "no sleep … in the checks" is read across the session checks, which `stand_virtual_clock` (a `Session::run` check) now joins.
    basis: tests/blitz-tests/tests/stand_virtual_clock.rs:50-96
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → blitz-dom (clock)
    change: >-
      New bullet: since 2026-10-18-one-virtual-clock `clock.rs` (`Clock`, `WallClock`, `VirtualClock`) holds 3 inline unit tests — a virtual clock reads the sum of its advances on every clone, a virtual clock saturates rather than wrapping, a wall clock never reads less on a later read (packages/blitz-dom/src/clock.rs:84-114); `DocumentConfig.clock` and the five engine time reads are read by blitz-tests' `virtual_clock_engine` (PROVISIONAL).
    sidecar: >-
      test-plan §1: blitz-dom's new `clock` module and its three unit tests.
    rationale: >-
      Report Symbols / APIs: "Three inline unit tests at 84-114"; Counts item 17: "blitz-dom library unit tests +3 (`clock.rs`). No pattern was built: the masters state blitz-dom unit tests per module (test-plan §1), and `clock.rs` is a new module"; Crates / modules: "blitz-dom gains a module, `clock`"; Coverage of new surfaces: `DocumentConfig.clock` tests unit (`clock.rs` 3) + integ (`virtual_clock_engine` 4).
    basis: packages/blitz-dom/src/clock.rs:84-114
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: >-
      New bullet blitz-dom (clock): `VirtualClock` pinned to read the sum of its advances on every clone and to saturate rather than wrap; `WallClock` never reading less on a later read (packages/blitz-dom/src/clock.rs:84-114).
    sidecar: >-
      test-plan §4: what blitz-dom's clock unit tests pin.
    rationale: >-
      Report Symbols / APIs, the clock types: `advance(&self, Duration)` "adds with saturation", "Every clone reads the same value", "Three inline unit tests at 84-114"; New text rows 88-97, 99-105 and 107-113 name the three tests. §4 states per-module unit coverage and names no clock module.
    basis: packages/blitz-dom/src/clock.rs:84-114
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → tests/blitz-tests
    change: >-
      Add the two checks of 2026-10-18-one-virtual-clock: `virtual_clock_engine` 4, an engine check on a parsed page with no `stand_` prefix, in both layout modes — the scrollbar fade moves only with the clock (1.0 at 0 and 500 ms, part faded at 600, 0 and not animating at 700), two clicks are a double click by the clock alone (a 499 ms boundary row; case b through `Harness::wrap`), a smooth scroll stands still until the clock moves and ends at 300 ms (a 299 ms midway reading), and a document built with no clock fades by the wall clock (tests/blitz-tests/tests/virtual_clock_engine.rs:77-234); and `stand_virtual_clock` 2, a session check through `Session::run`, both layout modes — a shot after a scroll repeats across an 800 ms sleep until `advance` moves the fade (S2 == S1, S3 ≠ S1, S4 ≠ S3, S5 == S4, S1 ≠ S4, `harness().time()` 1.7 s), and two clicks with no `advance` between them are a double click, differing from two single clicks and repeating across three fresh sessions per mode (tests/blitz-tests/tests/stand_virtual_clock.rs:50-133) (PROVISIONAL).
    sidecar: >-
      test-plan §1 tests/blitz-tests: `virtual_clock_engine` 4 and `stand_virtual_clock` 2.
    rationale: >-
      Report Counts item 18: "Two new check names, `virtual_clock_engine` and `stand_virtual_clock` … No master names them yet"; Files, New; Outcome (tests, arch, design, a11y) states each test's readings; Deviations items 7 and 8 (the 499 and 299 ms rows, `Harness::wrap`, three fresh sessions); Counts item 12 (+4 and +2).
    basis: tests/blitz-tests/tests/stand_virtual_clock.rs:50-96
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy → Test directory + naming conventions (Directory pattern)
    change: >-
      The blitz-tests count reads 134 `.rs` files, 133 test targets and the unbuilt `all.rs`, the two newest — `virtual_clock_engine.rs` and `stand_virtual_clock.rs`, added at 2026-10-18-one-virtual-clock — each its own auto-discovered target with no manifest line (132 and 131 until then).
    sidecar: >-
      test-plan §2 Directory pattern: blitz-tests 132 · 131 → 134 · 133.
    rationale: >-
      Report Counts item 11: "It was 132 `.rs` files and 131 test targets; it is now 134 and 133 (two new files, `ls tests/blitz-tests/tests/*.rs`, implement run). Pattern `bt-targets`: … 1 changed (test-plan.md:66)".
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → Per-OS test tallies
    change: >-
      Add the re-read at 2026-10-18-one-virtual-clock on CI#39111579925, the pre-CI commit `87f12079` (green 17/17): linux 194 · 1001 · 0 · 12; macos and coverage 175 · 998 · 0 · 12; windows 175 · 974 · 0 · 9; the a11y job 6 · 43 · 0 · 0; the stand repeat 147 · 588 · 0 · 21; ios and android build only — +2 result lines (`virtual_clock_engine`, `stand_virtual_clock`) and +10 passed on the four test jobs (blitz-dom +3, `virtual_clock_engine` +4, `stand_virtual_clock` +2, seven_guis +1), +3 · +6 on the stand repeat, a11y unmoved — as measured at .andromeda/runs/2026-10-18T08-45-11-wrap/ci-tallies.md.
    sidecar: >-
      test-plan §9 Per-OS tallies: moved from CI#39104364530 to CI#39111579925.
    rationale: >-
      Report Addendum inputs#I4 (b): "test-plan §9 Per-OS moves from CI#39104364530 to CI#39111579925 beside items 12 and 13", read at reconcile and recorded in ci-tallies.md; Counts items 12 (+10 breakdown), 13 (stand-repeat 144 · 582 → 147 · 588) and 14 (a11y unchanged at 6 · 43 · 0 · 0).
    basis: .andromeda/runs/2026-10-18T08-45-11-wrap/ci-tallies.md
  - detector: D-tests-coverage
    severity: warning
    section: §9 CI Integration → Local baseline (`cargo test --workspace`)
    change: >-
      Add the re-count at 2026-10-18-one-virtual-clock: 194 result lines, 1001 passed · 0 failed · 12 ignored, from 192 · 991 — +2 result lines (`virtual_clock_engine`, `stand_virtual_clock`) and +10 passed (blitz-dom +3 in `clock.rs`, `virtual_clock_engine` +4, `stand_virtual_clock` +2, seven_guis +1), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` runs.
    sidecar: >-
      test-plan §9 Local baseline: the workspace tally 192 · 991 → 194 · 1001.
    rationale: >-
      Report Counts item 12: "The local workspace tally (`ci-leg.sh fast`) was 192 · 991 · 0 · 12; it is now 194 · 1001 · 0 · 12 … The +10 is: blitz-dom +3 (`clock.rs`), virtual_clock_engine +4, stand_virtual_clock +2, seven_guis +1". The report places the pattern at test-plan.md:325, but the local `fast` record ("re-counted at 2026-10-18-upstream-sync-determinism-and-traces: 192 result lines, 991 passed") is the §9 Local baseline's workspace sub-bullet; line 325 holds the CI per-OS reading (proposed above).
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Session lifecycle (session-proof)
    change: >-
      session-proof reads seven_guis' library unit tests 3 (the stand's time step, its carry rows and, since 2026-10-18-one-virtual-clock, `the_timer_step_takes_the_largest_advance`, beside the Timer's tick stream; 2 until then), and adds since that chunk `stand_virtual_clock` 2 — `Session::run` in both layout modes: a shot after a scroll repeats until `advance` moves the fade, and two clicks with no `advance` between them a double click.
    sidecar: >-
      test-plan session-lifecycle session-proof: seven_guis library tests 3; `stand_virtual_clock` 2 joins the proof.
    rationale: >-
      Report Counts item 4 `sg-units`: "session-lifecycle.md:11 says 'seven_guis' library unit tests 2'; it is now 3"; Counts item 18: `stand_virtual_clock` named by no master; Outcome (tests, a11y) states its two tests, run through a held session.
    basis: examples/seven_guis/src/stand.rs:161-173
note: >-
  Not proposed, for the orchestrator — (1) §9 Legs (test-plan.md:315, "— 41 files —") and §10 Flakiness budget (:344, "the 41 `stand_*.rs` files") count the stand files that `stand-repeat` runs; `stand_virtual_clock.rs` joins them by its prefix (report Harness / gate surface), but the report states no new count and 41 may predate `stand_act_transformed.rs`, so measure before moving it; §3 agent-run Proof's `run stand` re-count (:116) is likewise unmoved by the report. (2) D-tests-coverage, possible gap: the report's Coverage of new surfaces and Outcome name no check of the sub-document's carried clock (`iframe.rs` 75) or of the pan and fling reads under the virtual clock (`pointer.rs` 266 and 713; the fling arm at `scrolling.rs` 865); only the fade, the double click and the smooth scroll are read on the clock. Owning that is a route call (CARRY or WATCH), not a test-plan edit. D-tests-framework and D-tests-obs-harness: no drift (standard libtest, no agent-run, status or log-format change).
