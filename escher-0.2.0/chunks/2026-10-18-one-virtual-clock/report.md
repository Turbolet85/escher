# Report — 2026-10-18-one-virtual-clock

**Chunk:** One virtual clock — every time a headless run reads, the overlay scrollbar fade included, taken from one clock only the driver moves; two shots of one state read the same pixels
**Date:** 2026-10-18
**Commits:** since `last_wrap` 2026-10-18T07:09:10Z (`git log --since`):
- `87f120794 chore(2026-10-18-one-virtual-clock): operator pre-CI commit, for the run this chunk's verdict reads` — this chunk's one commit.
- `f45bb570a` — the previous wrap's own commit, the basis.

## Changes (structured — detectors read this)

- **Files** (basis: `gate.py scope` — changed 28 · listed 25 · recorded 3; `cites.py added`, `new-text.md`):
  - **New (3):**
    - `packages/blitz-dom/src/clock.rs`
    - `tests/blitz-tests/tests/virtual_clock_engine.rs`
    - `tests/blitz-tests/tests/stand_virtual_clock.rs`
  - **Modified (25):**
    - blitz-dom: `lib.rs`, `config.rs`, `document.rs`, `resolve.rs`, `iframe.rs`, `events/pointer.rs`, `scrolling.rs`.
    - blitz-test-harness: `harness.rs`.
    - escher-driver: `execute.rs`, `session.rs`, `refusal.rs`, `schema.rs`, `schema_tests.rs`, `lib.rs`, `json.rs`, `describe.rs`.
    - seven_guis: `src/stand.rs`, `tests/host_timer.rs`, `tests/cli_commands.rs`.
    - blitz-tests: `stand_act_timer.rs`, `stand_act_spans.rs`, `stand_mcp_tools.rs`, `stand_screenshot_repeat.rs`, `stand_non_lean_tasks.rs`, `session_common/mod.rs`.
  - Bookkeeping that rode the pre-CI commit: this chunk's folder (`scope-record.md`), the implement run dir, `.andromeda/friction-log.ndjson`, and the matrix (`v020-06`).

- **Symbols / APIs:**
  - **blitz-dom: the clock types** (`packages/blitz-dom/src/clock.rs`, new):
    - `pub trait Clock: Send + Sync { fn now(&self) -> Duration }` at 14-19. It is monotonic from its own origin.
    - `pub struct WallClock` at 21-25. It holds a `web_time::Instant` origin; `new()` and `Default` at 27-40; its `Clock` impl at 42-46.
    - `pub struct VirtualClock` at 48-55. It is `Clone` and `Default`, and holds an `Arc<AtomicU64>` of nanoseconds.
      - `new()` reads zero.
      - `advance(&self, Duration)` at 63-75 adds with saturation, through a `compare_exchange_weak` loop.
      - Every clone reads the same value. Its `Clock` impl is at 78-82.
    - Three inline unit tests at 84-114.
    - Re-exported from `lib.rs` (`pub use clock::{Clock, VirtualClock, WallClock}`, the added line 96; `mod clock;` at 42).
  - **blitz-dom: `DocumentConfig.clock: Option<Arc<dyn Clock>>`** (`config.rs` added 71-73; `None` in `Default` at 92). It is documented as wall time when `None`, read for the fade, double-click detection, smooth scrolling and fling.
    - `BaseDocument` holds `pub(crate) clock: Arc<dyn Clock>` (`document.rs` 264-265): the config's clock, else a new `WallClock` (503).
    - A sub-document's config carries its parent's clock (`iframe.rs` 75).
    - Two new `pub(crate)` helpers on `BaseDocument`: `elapsed_since(Duration) -> Duration` (1897-1900, saturating) and `clock_millis() -> u64` (in 1894-1904).
    - Every other caller of `DocumentConfig` builds it with `..Default::default()` (a fixed-string read of each `DocumentConfig {` literal over `packages apps examples tests wpt`). Their behaviour is unchanged: wall time.
  - **blitz-dom: the five engine time reads** now go through the document's clock:
    1. **The overlay scrollbar fade.**
       - `scrollbar_activity: HashMap<NodeId, Duration>` (328-330) is stamped with the clock's reading in `show_scrollbars` (1883-1884).
       - It is read by `scrollbar_opacity` (1874-1875) and `scrollbars_animating`, and the finished-fade prune is in `resolve.rs` (71, 73).
    2. **The double-click window.** `last_mousedown_time: Option<Duration>` (317-318), compared in `handle_mousedown` (`pointer.rs` 392 and 403) against the unchanged 500 ms and 2 px.
    3. **Pan, and the fling's generation** (`pointer.rs` 266 and 713).
    4. **The smooth-scroll start** (`scrolling.rs` 514).
    5. **The fling and smooth frames.** `resolve_scroll_animation` reads the clock once, before its `match`, at line 865.
    - `FADE_DELAY`, `FADE_DURATION`, `opacity_at`, the 300 ms smooth duration and the fling constants are unchanged. A `git diff -U0` of `scrolling.rs` and `pointer.rs`, filtered for those tokens, matched no line.
    - No `Instant::now()` or `SystemTime::now()` is left in `packages/blitz-dom/src` outside `clock.rs` (the census gate printed `0`). The five `duration_since(UNIX_EPOCH).unwrap()` panic paths are gone.
  - **blitz-test-harness `Harness`** (`harness.rs`):
    - The harness holds a `VirtualClock` in place of `time: f64` (81-82).
    - `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock` (49-51, 71). So `from_html_with` and `from_vdom` (95-96, 113-114), and through them `from_html` and `from_component`, build documents that read the harness's clock.
    - `wrap(doc)` gives the harness a fresh `VirtualClock`, while the wrapped document keeps the clock its own config gave it (127-133).
    - `time()` returns the clock's reading in seconds; `pump` resolves at it.
    - `tick(dt)` advances the clock. A negative or non-finite `dt` moves nothing (`Duration::try_from_secs_f64`).
    - New: `pub fn advance_time(&mut self, by: Duration)` advances by exactly `by`, then pumps (167-172).
    - `screenshot`, `screenshot_rect`, `text_contrast` and `settle` still never move the clock.
    - Remaining callers: `tick` has none in the tree, and `wrap`'s are settle's two unit tests.
  - **escher-driver.**
    - **`advance`** (`execute.rs` 178, 181-184, 190): inside one settled step it calls `harness.advance_time(Duration::from_millis(ms))`, then the session's time step where one was handed, and reports `advanced_ms = ms`. It is accepted on every session.
    - **The time step:** `TimeStep = Box<dyn FnMut(&mut Harness<DioxusDocument>, u32)>`, with no return (`session.rs` 21-23). `with_time(step: impl FnMut(&mut Harness<DioxusDocument>, u32) + 'static)` and its doc say the session moves the clock on every `advance`, and the step moves the app's own time (116-125).
    - **`Cause::TimeUnavailable` is removed.** `CAUSES: [Cause; 8]` keeps the old order less that one.
    - **Help texts** (`schema.rs` 161, 192-194, 250-252, 257, 279). The new texts:
      - `click`: "clicks the element an id names, then settles; a click on the same point within 500 ms of the session's time after the last click there, `type`'s own click included, is a double click, and an `advance` of at least 500 ms separates two single clicks".
      - `advance`: "moves the session's clock forward by `ms` milliseconds, then settles; on every session it moves the engine's animations, the overlay scrollbar fade and the app's own time together, and `advanced_ms` is that move".
      - The `ms` argument: "how far to move the session's clock, in milliseconds".
      - The `advanced_ms` field: "the time the session's clock moved, in milliseconds: exactly `ms`".
      - `screenshot` drops its sentence "in a build that paints overlay scrollbars, a scrollbar thumb painted within 0.7 s of a scroll fades by wall time".
    - **Crate docs** (`lib.rs` 30-31, 51-55, 78-80): the eight causes, `advance` on the one clock, and the screenshot no longer wall-time dependent.
    - **The unit-test pins:**
      - `schema_tests.rs`: the `amended` table goes from 4 rows to 7 (82-99, 102).
      - `refusal.rs`: `the_eight_causes_are_named_in_order` (208).
      - `json.rs` 657: 13 refusals.
      - `describe.rs` 145 and 149: the causes row reads 8.
  - **seven_guis `stand::timer_step(ticks) -> impl FnMut(&mut Harness<DioxusDocument>, u32)`** (`stand.rs` 89-100):
    - It carries the sub-tick remainder across calls, so 250 then 250 deliver 2 then 3 ticks, and returns nothing.
    - Its unit test is renamed `the_timer_step_carries_milliseconds_onto_whole_ticks` (six cumulative rows, two carry rows). A new test, `the_timer_step_takes_the_largest_advance`, is at 161-173.
    - Remaining callers: `session_host.rs` and `session_common::hold` still hand the Timer alone a step.

- **Crates / modules:** blitz-dom gains a module, `clock`. No crate is added or removed.

- **Dependencies:** none added or bumped. `web-time` was already a blitz-dom dependency (`packages/blitz-dom/Cargo.toml:108`). `Cargo.lock` is unchanged.

- **Schema / config:**
  - New config key `DocumentConfig.clock`.
  - The driver's refusal set closes at eight causes: `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`. `describe()` lists eight.
  - The `command` and `tool_call` span fields are unchanged; no time, size or image is fielded.
  - The wire stays `v2`, and no MCP or wire carrier changed.

- **Spec-master edits:** none.

- **Counts / qualifiers moved.** Each fact names its pattern id. Pattern ids are in `report-patterns.toml`, and the sweep's trail is `cascade-2026-10-18-one-virtual-clock-pre-pass.json`, both in `.andromeda/runs/2026-10-18T08-45-11-wrap/`. A row's disposition is changed (stale) or no-change (why).
  1. **Refusal causes: 9 → 8.** `time-unavailable` is retired. Patterns `causes-nine`, `time-unavail`.
     - `causes-nine`: standing 9 rows · 8 changed:
       - architecture.md:160 @17052;
       - security-plan.md:81 @3963 and @4047 ("all nine … two by `validate` and seven by the executor" — now eight, six by the executor);
       - security-plan.md:403;
       - test-plan.md:26 @28374 ("the ninth cause");
       - test-plan.md:143;
       - test-plan.md:181;
       - obs-plan.md:113;
       - registries/contracts/test-plan/session-lifecycle.md:5.
     - `causes-nine` no-change:
       - architecture.md:137 (a CRUD "ninth row");
       - test-plan.md:26 @19985 (the ninth control row) and @31919 (a ninth list row);
       - architecture.md:160 @19425 (a dated history clause: "the nine causes and their names unchanged").
     - `time-unavail`: standing 8 rows · 8 changed:
       - architecture.md:160;
       - security-plan.md:81 and :357;
       - test-plan.md:26, :181 and :200;
       - obs-plan.md:113;
       - session-lifecycle.md:5.
     - Of these, architecture.md:160 @32414 is the 2026-10-07 ruling that "an `advance` refused `time-unavailable`" records nothing. It becomes history.
  2. **`advanced_ms` reads exactly `ms`.** Pattern `advanced-ms`: standing 7 · 4 changed · 3 no-change.
     - Changed:
       - architecture.md:160 @35435 ("the lesser of the step's return and `ms`");
       - test-plan.md:10 (250 → `advanced_ms` 200, now 250);
       - test-plan.md:26 (50 reports 0, 250 reports 200);
       - test-plan.md:284 ("never more than asked (50 → 0").
     - No-change, because the field still exists and is never fielded: security-plan.md:81 @8131, test-plan.md:101, obs-plan.md:113.
  3. **`advance` moves the session clock on every session, and the step returns nothing.** Patterns `with-time` and `cli-advance`.
     - `with-time`: standing 14 · 9 changed · 5 no-change.
       - Changed:
         - architecture.md:158;
         - architecture.md:160 (@1395 the `with_time` signature, @14885, @35335);
         - architecture.md:165 (the "other three tasks carrying no time step" now still `advance`);
         - security-plan.md:81;
         - test-plan.md:10;
         - test-plan.md:26 @20952;
         - test-plan.md:87;
         - test-plan.md:284;
         - session-lifecycle.md:5.
       - No-change: architecture.md:285, test-plan.md:101, obs-plan.md:70, session-lifecycle.md:9 and :11. Each names the step or its holder, and those stay true.
     - `cli-advance`: standing 14 · 9 changed · 5 no-change.
       - Changed:
         - architecture.md:160 (@1593, @14743, @14937, @32350/@32395, @35296);
         - security-plan.md:81 (@4375; @4488 "clamped to the `ms` asked" — now equal);
         - security-plan.md:217;
         - design-system.md:291 ("the driver's `advance` moves an app's own time … and leaves that clock where it was");
         - test-plan.md:10;
         - test-plan.md:26;
         - test-plan.md:200;
         - test-plan.md:284;
         - session-lifecycle.md:5.
       - No-change (verb lists, bounds, or "time moved by `advance`"):
         - architecture.md:165;
         - architecture.md:290;
         - security-plan.md:123;
         - test-plan.md:181;
         - session-lifecycle.md:9.
       - The no-change offsets on architecture.md:160: @8894, @12595, @33048, and @35591 (60000 reports 60000 while the display stops at 15 s, still true).
  4. **`timer_step` carries its remainder** (250 + 250 → 0.5 s). The "remainder dropped, harness clock untouched" reading is retired. Patterns `timer-step`, `renamed-tests` and `sg-units`.
     - `timer-step`: standing 7 · 4 changed (architecture.md:158; test-plan.md:10; test-plan.md:87; test-plan.md:284) · 3 no-change (architecture.md:165; test-plan.md:101; session-lifecycle.md:9 — each names the step).
     - `renamed-tests`: standing 1 · 1 changed. test-plan.md:10 names `stand::tests::the_timer_step_maps_milliseconds_onto_whole_ticks`, now `the_timer_step_carries_milliseconds_onto_whole_ticks` plus `the_timer_step_takes_the_largest_advance`.
     - `sg-units`: standing 1 · 1 changed. session-lifecycle.md:11 says "seven_guis' library unit tests 2"; it is now 3 (`cargo test -p seven_guis --locked --lib`: 3 passed, implement run).
     - The stand_act_timer renames (`a_session_with_no_time_step_refuses_advance_and_changes_nothing` → `an_advance_on_a_session_with_no_time_step_moves_the_clock_and_settles`; `an_advance_reports_no_more_than_it_was_asked` → `an_advance_reports_exactly_what_was_asked_whatever_the_step_does`) have 0 master rows under `renamed-tests`.
  5. **The fade's wall-clock limit is retired**, along with the `screenshot` help sentence. Patterns `wall-fade` and `help-text`.
     - `wall-fade`: standing 4 · 3 changed · 1 no-change.
       - Changed: architecture.md:160 @9433/@9460; security-plan.md:217 ("fades by the engine's wall clock, so such a shot can differ"); test-plan.md:324 ("no check shoots inside that window").
       - No-change: test-plan.md:181 ("no wall-clock read in the checks", true).
     - `help-text`: standing 2 · 2 changed, the same two sites: architecture.md:160 @9451 and test-plan.md:324.
  6. **The harness's time is the documents' `VirtualClock`**, with `advance_time` new. Pattern `harness-time`: standing 17 · 8 changed · 9 no-change.
     - Changed:
       - architecture.md:157 (@9170 "`pump`, `tick` … unchanged");
       - architecture.md:158 ("the harness's animation clock stays where it was");
       - security-plan.md:217 @1238 ("moving the fade onto the harness clock … owned by" a residual);
       - design-system.md:291 ("a controlled clock advanced by tick … leaves that clock where it was");
       - test-plan.md:26 @20874 ("the harness clock reading the same before and after every call");
       - test-plan.md:88 (the Harness API list gains `advance_time`);
       - test-plan.md:284;
       - test-plan.md:324 @4553.
     - No-change:
       - architecture.md:113 and test-plan.md:327 (the WPT harness timeout);
       - architecture.md:165, architecture.md:291 and test-plan.md:101 (the tick handle);
       - test-plan.md:47 (the pump/tick loop);
       - test-plan.md:89 ("resolves at the harness time", true);
       - session-lifecycle.md:9 and :11.
  7. **`DocumentConfig.clock` and the clock types.** Pattern `doc-clock`: standing 2 · 0 changed · 2 no-change (architecture.md:121 and test-plan.md:98 cite `blitz-vibey-script/src/clock.rs`, another clock). This fact has no master site yet.
  8. **The double-click window reads the document's clock**, so two driver clicks with no `advance` between them are a double click. Patterns `dblclick` and `driver-click`.
     - `dblclick`: standing 7 · 1 changed · 6 no-change.
       - Changed: architecture.md:86 ("repeated clicks within 500 ms and 2 px" — the clock qualifier).
       - No-change:
         - architecture.md:113 and obs-plan.md:258 (WPT 500 ms);
         - architecture.md:156, architecture.md:290 and test-plan.md:18 (the password double click, true);
         - design-system.md:286 (`FADE_DELAY` = 500 ms, value unchanged).
     - `driver-click`: standing 6 · 0 changed · 6 no-change. These are single-click readings: architecture.md:137, :160 and :163; layout-templates.md:45; test-plan.md:26; a11y-plan.md:161.
  9. **The scrolling and pointer epoch `unwrap` panic paths are retired.** Pattern `epoch-unwrap`: standing 1 · 1 changed (security-plan.md:374, `scrolling.rs:515-518` and `pointer.rs:717-720`).
  10. **One recorded 800 ms sleep in a stand check** (`stand_virtual_clock`). Pattern `no-sleep`: standing 5 · 0 changed · 5 no-change (architecture.md:290; test-plan.md:10, :26, :181 and :200 — each names a check that still sleeps not). The general no-sleep rule's master site is not reached by this pattern's wording (it is in the leaf `.claude/rules/testing.md:30`).
  11. **blitz-tests target count.** It was 132 `.rs` files and 131 test targets; it is now 134 and 133 (two new files, `ls tests/blitz-tests/tests/*.rs`, implement run). Pattern `bt-targets`: standing 1 · 1 changed (test-plan.md:66).
  12. **The local workspace tally (`ci-leg.sh fast`)** was 192 · 991 · 0 · 12; it is now 194 · 1001 · 0 · 12. This is result lines · passed · failed · ignored, summed by a scratch `test result:` reader over the `fast` logs of this chunk's gate runs (entries 9, 9.2 and 16, all three the same). Pattern `local-tally`: standing 1 · 1 changed (test-plan.md:325).
      - The +10 is: blitz-dom +3 (`clock.rs`), virtual_clock_engine +4, stand_virtual_clock +2, seven_guis +1.
      - The per-OS CI tallies of CI#39111579925 are not read at P1.
  13. **The stand-repeat tally** was 144 · 582 · 0 · 21; it is now 147 · 588 · 0 · 21 (entry 11.2's log, the same reader; +3 result lines and +6 tests, `stand_virtual_clock` × 3 rounds). Pattern `repeat-tally`: standing 1 (×2) · 1 changed (test-plan.md:325).
  14. **The a11y job tally is unchanged at 6 · 43 · 0 · 0** (entry 12's log). Not a moved fact.
  15. **Settle's `animating` now reads the virtual fade.** After a driver `scroll` it reads true until 700 ms of clock pass. Pattern `settle-anim`: standing 5 · 1 changed · 4 no-change.
      - Changed: design-system.md:290 (the fade's membership in the animating set reads the clock).
      - No-change: architecture.md:157 ("reads no clock but the harness's own", true); design-system.md:291 (the shell's clock); test-plan.md:26; test-plan.md:89.
  16. **escher-driver unit tests stay 74.** Pattern `sg-units`'s neighbour row (session-lifecycle.md:11) also states 74 — unchanged. Not a moved fact.
  17. **blitz-dom library unit tests +3** (`clock.rs`). No pattern was built: the masters state blitz-dom unit tests per module (test-plan §1), and `clock.rs` is a new module. The detectors' sweeps are the reach control.
  18. **Two new check names, `virtual_clock_engine` and `stand_virtual_clock`.** Pattern `new-checks`: its control did not fire over the seven masters at 87f12079 (the first sweep's refusal line). No master names them yet.
  19. **The merge-surface CARRY gains this chunk's seven engine files.** Patterns `carry-owner` and `owned-by`.
      - `carry-owner`: standing 1 · 1 changed (architecture.md:90, the merge surface by file).
      - `owned-by`: standing 2 · 1 changed (architecture.md:90) · 1 no-change (security-plan.md:123, an id record); base 2 no-change (playbook.md:58 and :66).
  - Sweep reach: 22 patterns · 17 moved facts (the report's own count; items 14 and 16 are not moved facts), 15 with a pattern (items 17 and 18 without one) · owner pattern control unfired · by line 18 lines · 60 offsets.
    - Basis: `cascade.py sweep --pre-pass`, `total (22 patterns) · 198 rows over 22 files`. The first call was refused: `control: title, new-checks never fired over the seven masters' pre-pass text (87f12079)`.

- **Dev-tool versions:** none. The host toolchain is not re-read; one reading was met: `AtomicU64::fetch_update` is deprecated there ("renamed to `try_update`", rustc diagnostic at implement P1), and `try_update` is past the MSRV of 1.91. This is not a version change.

- **Harness / gate surface:**
  - A new `Harness::advance_time(Duration)`. `Harness::time()` and `tick` read and move the shared `VirtualClock`.
  - The driver's `advance` is accepted on every session.
  - No agent-run or CI-leg change; `stand_virtual_clock` joins `stand-repeat` by its `stand_` prefix.

- **Cross-project / external claims:**
  - **CI:** CI#39111579925 (push) on `87f1207940e1`, the pre-CI commit, reads `verdict: green · checks 17/17 · wall 924 s · runs CI#39111579925 completed/success`. The read is `ci.py conclusion --sha HEAD --wait 1800`, the operator entry recorded in `evidence/operator-pass.md`.
  - **The inputs**, from `inputs.py verify` (`inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3 · uncited 1 · unparsed 0`):
    - `I1 · message: the operator, 2026-10-18T07:18Z, the /andromeda-phase invocation · copy · n/a` — cited.
    - `I2 · message: the operator, as the founder delegate, 2026-10-18T07:36Z · copy · n/a` — cited. These are the founder forks F1 B · F2 A · F2b A · F3 A, answered by the delegate operator, PROVISIONAL.
    - `I3 · message: the operator, 2026-10-18T07:55Z, the P5 word · copy · n/a` — `UNCITED` before this report, cited here as inputs#I3.
      - Its `why` names the P5 review card `.andromeda/runs/2026-10-18T07-18-53-phase/p5-card-1.md`, read whole. The word: "yes. My planlint: 0 hits. CI#39107039862 on f45bb570 read green 17/17 (my own read, 07:31Z). Beyond-the-plan row 1 accepted." Authority: the operator's own word, 2026-10-18.
      - The card's `## Beyond the plan` rows, verbatim:
        - `| 1 | CARRY: the merge surface of this chunk's engine change (`packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs`, `events/pointer.rs`), flagged upstreamable, no upstream pull request | the existing entry «Upstream sync ahead of visual lint and real-app readiness» |`
      - For P5 (route-resolve): row 1 is the placement of the CARRY, by the operator's own word (inputs#I3). The amendment's owner is architecture.md:90's CARRY, Counts item 19.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  - **Plan step 8:** "`describe.rs` and `json.rs` iterate `CAUSES` and need no change of their own" (plan.md:55). Measured false: their unit tests pin the old count (`json.rs`: 14 refusals; `describe.rs`: a causes row of 9), and `cargo test -p escher-driver --locked` read 2 failed at implement P1. They were edited as recorded companions. This is a plan claim, not a master's: `no master site — a plan line`.
  - **The plan's Implementation notes:** "No harness-built check sleeps … nothing waits on wall time through a harness document". This stays true. The new `stand_virtual_clock` sleeps 800 ms on purpose, as the plan's §Constraints records. Not disproved.

- **Expected amendments (from plan).** Pattern counts are standing rows from the site sweep above; registry hits are counted with the master that owns them.
  1. **architecture §Standard Contracts.** `DocumentConfig.clock` (wall by default), the harness's time as the documents' `VirtualClock`, `advance` moving it on every session with `advanced_ms` its move, eight causes, `timer_step` carrying its remainder, and the merge surface on a CARRY on «Upstream sync ahead of visual lint and real-app readiness».
     - Carried: Symbols / APIs; Counts items 1–7 and 19.
     - Searches (architecture rows): `doc-clock` 1 (no-change) · `harness-time` 5 · `with-time` 4 · `cli-advance` 3 · `time-unavail` 1 · `causes-nine` 2 · `timer-step` 2 · `carry-owner` 1.
     - Pattern: «An engine behaviour change for every document» (an API addition with default behaviour unchanged), answered by the delegate operator, PROVISIONAL under «Provisional discharge».
  2. **security-plan.**
     - §Input Validation (Driver command schema row): eight causes, all returned by code; `advance` accepted on every session; the clock the harness's value.
     - §API Security (Settle and loads in flight): the fade limit retired.
     - §Error Handling (Panic paths): the `scrolling.rs:515-518` and `pointer.rs:717-720` epoch `unwrap`s retired.
     - Delegate operator, PROVISIONAL (F2, F2b). Carried: Counts items 1, 3, 5 and 9.
     - Searches (security-plan rows): `causes-nine` 2 · `time-unavail` 2 · `cli-advance` 3 · `wall-fade` 1 · `epoch-unwrap` 1.
  3. **design-system §Motion (Animation runtime).** `advance` moves the harness's clock, and the fade, smooth scroll and fling read it headlessly. The `document.rs` and `scrolling.rs` citations are re-pointed by measurement.
     - Carried: Counts items 3, 6 and 15.
     - Searches (design-system rows): `harness-time` 1 (:291) · `cli-advance` 1 (:291) · `settle-anim` 2 (:290, :291) · `dblclick` 1 (:286, no-change; its `document.rs:1856-1873` citation is the citation sweep's).
  4. **layout-templates §Surface: cli.** `advance` accepted on every task, `time-unavailable` gone.
     - Searches (layout-templates rows): `cli-advance` 0 · `time-unavail` 0 · `with-time` 0 · `driver-click` 1 (layout-templates.md:45, the CRUD list's `scroll` — no-change). layout-templates.md:73, the cli surface, was read; it names the refusal statuses without listing the causes or `advance`.
     - Not carried: no layout-templates site states the moved fact. No master is named owner; the detector's own sweep of layout-templates is the reach control.
  5. **test-plan.** §3 Core (`time()`, `tick`, `advance_time` on the `VirtualClock`); §3 session-start (`advance` on every session, no `time-unavailable`); §5 (the one recorded sleep); §8 Time row (the remainder carried); §2 and §1 (the two new checks); the counts and tallies.
     - Carried: Counts items 1–6 and 10–13.
     - Searches (test-plan rows, with session-lifecycle.md's under test-plan): `harness-time` 8 + 2 · `with-time` 5 + 3 · `cli-advance` 5 + 2 · `time-unavail` 3 + 1 · `causes-nine` 3 + 1 · `advanced-ms` 4 · `timer-step` 4 + 1 · `renamed-tests` 1 · `sg-units` 0 + 1 · `bt-targets` 1 · `local-tally` 1 · `repeat-tally` 1 · `no-sleep` 4 (§5's general rule not reached).
  6. **obs-plan.** §4: eight causes; `stand_act_spans`' `advance` line accepted. §3: the two new checks in the silent census.
     - Carried: Counts items 1 and 18, and Outcome (`stand_act_spans`).
     - Searches (obs-plan rows): `causes-nine` 1 · `time-unavail` 1 · `advanced-ms` 1 (no-change) · `with-time` 1 (no-change). The §3 silent census has no hit (`new-checks` control unfired).
  7. **a11y-plan §5.** Two driver clicks on one point read as a double click by session time; single-click focus unchanged.
     - Carried: Counts item 8.
     - Searches (a11y-plan rows): `driver-click` 1 (a11y-plan.md:161, the single-click focus reading — no-change) · `dblclick` 0. The double-click reading has no current site; a11y-plan §5 is named by the plan, and a11y-plan.md:161 is its one hit.

- **Coverage of new surfaces:**
  - `DocumentConfig.clock` (embedder API) → validation n/a (an `Arc<dyn Clock>` of the embedder's own) · instrumentation n/a (no log site, by design) · PII n/a · tests unit (`clock.rs` 3) + integ (`virtual_clock_engine` 4, both layout modes) · a11y n/a · tokens n/a.
  - `Harness::advance_time` → validation n/a (a `Duration`) · instrumentation n/a · PII n/a · tests integ (`stand_act_timer`, `stand_virtual_clock`, `host_timer`) · a11y n/a · tokens n/a.
  - Driver `advance` accepted on every session → validation ✓ (`validate`: `ms` 1..=60000, `advance --ms 0` and `60001` still `malformed`, unchanged) · instrumentation ✓ (the `command` span: `verb`, `settled`, `busy`, `passes` and diff counts; `stand_act_spans` reads an accepted `advance` line; `stand_mcp_tools` reads its `tool_call` ending `accepted` with one child `command`) · PII n/a (no time or clock fielded) · tests integ · a11y n/a · tokens n/a.
  - The double click on the one clock (`click`) → validation ✓ (unchanged) · instrumentation n/a · PII n/a · tests integ (`virtual_clock_engine` case b, `stand_virtual_clock` double click) · a11y ✓ (the a11y leg green, 6 · 43; single-click focus unchanged) · tokens n/a.

## Deviations from intent

**From implement's report** (`.andromeda/runs/2026-10-18T07-55-10-implement/outcome-2026-10-18-one-virtual-clock.md`):
1. **`VirtualClock`'s mechanism.** It uses `Arc<AtomicU64>` nanoseconds with a `compare_exchange_weak` loop and saturating add. `fetch_update` is deprecated on the host toolchain, and `try_update` is past the MSRV. A unit test pins the saturation. This is within plan step 1's "an `Arc` over an atomic or a mutex".
2. **`resolve_scroll_animation`** reads the clock once, before its `match`: the fling and smooth arms borrow `scroll_animation` mutably. Both arms read the same value per frame, and two `pub(crate)` helpers were added.
3. **`Harness::tick`** with a negative or non-finite `dt` now moves nothing, because the clock is monotonic. `tick` has no caller.
4. **`stand_act_timer`'s animated-fixture test**, renamed `an_advance_reports_exactly_what_was_asked_whatever_the_step_does`. The plan's text was "an advance reports exactly what was asked, whatever the step does". The old "settled with an empty diff" cannot hold, because `advance` now moves the fixture's infinite CSS animation. It now asserts:
   - the step is handed exactly `ms`;
   - `advanced_ms` is `ms`;
   - the clock moved by exactly `ms`;
   - nothing was added or removed;
   - `fx-press` reads changed exactly when the step pressed it.
5. **The `timer_step` unit test** was split into the carry rows (their sums computed in the test) and a 60 000 ms test of its own.
6. **`stand_act_timer`'s no-step test** runs Counter, FlightBooker and Crud, where the plan named the counter: a superset.
7. **`virtual_clock_engine`** adds a 499 ms boundary row to case b and a 299 ms midway reading to case c. Case b drives its document through `Harness::wrap`.
8. **`stand_virtual_clock`'s double-click case** types `Grace Hopper`, shoots `crud-name` alone, and runs three fresh sessions per mode.
9. **Help texts beyond the plan's three.** The `advanced_ms` field help and the `ms` argument help are restated, and the `amended` pin table holds 7 rows.

**After implement, on the operator's word (2026-10-18):**
10. `stand_mcp_tools` now reads the accepted `advance`'s `tool_call` span: `ending` = `accepted`, and one child `command` whose `verb` is `advance` and which records no `cause`. This was plan step 13's F2 A row, "its `tool_call` `ending` `accepted`", which implement had not asserted.
11. `session_common/mod.rs:94`'s doc line was restated (below).

**The scope record** (`gate.py scope`: `scope: clean — changed 28 · listed 25 · recorded 3 (companion 3 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 57`):
- companion — `packages/escher-driver/src/json.rs` · serves `packages/escher-driver/src/refusal.rs` · self.
- companion — `packages/escher-driver/src/describe.rs` · serves `packages/escher-driver/src/refusal.rs` · self.
- companion — `tests/blitz-tests/tests/session_common/mod.rs` · serves `packages/escher-driver/src/execute.rs` · `word: "session_common/mod.rs:94: restate the doc line to what advance does now (it moves the session clock by N, then hands N to the app's time step); record the file in scope-record.md as companion, the operator's word." — the operator, 2026-10-18`.

## Decisions & corrections
1. **The founder forks, answered by the delegate.** F1 B · F2 A · F2b A · F3 A were answered by the operator as the founder's delegate, PROVISIONAL (inputs#I2). For the founder's epoch-boundary batch, beside the engine-debts and cursor-fix batches.
2. **The P5 card's row 1 accepted** in the operator's own word (inputs#I3): the merge-surface CARRY on «Upstream sync ahead of visual lint and real-app readiness». The operator also read CI#39107039862 on `f45bb570` green 17/17 at 07:31Z.
3. **The operator's direction after implement (2026-10-18, this window).**
   - Close the two loose ends: the `stand_mcp_tools` `tool_call` ending, and the `session_common` doc line recorded as a companion under the operator's word.
   - Re-fire the affected entries and the `fast` leg.
   - Run the operator pass in plan order: entry 15 as written; the pre-CI commit with exactly the plan's subject; entry 16 through `gate.py run --operator 16`, backgrounded; entry 17 as written.
   - Record it in `evidence/operator-pass.md`. Stop on any red; start no wrap.
   - All done. Evidence: `evidence/operator-pass.md`.
4. **The operator's word at this wrap's invocation:** "run P1 only and stop". This is the two-window form; P2–P7 run in a later window that resumes this run dir (Setup 2a).
5. **A guard refusal, and the recurrence of a standing learning.** At implement P1 the Bash guard refused a bare `cd` into `~/.claude/skills/andromeda-implement/references`: "Blocked: cd (…) moves the session cwd for every later call". This is host-linux «Paths & the working directory», already recorded as recurring in the handoff's deferred learnings. It was re-issued once as an absolute-path `grep`.
6. **A guard refusal.** After implement, the Bash guard refused a whole call because it ended with an empty `cat >> escher-0.2.0/chunks/…/scope-record.md <<'EOF'` append: "Blocked: a cat/tee heredoc with a file target - a new document goes through the Write tool, an append through the Edit tool anchored on the file's last lines". The python edits in the same call did not run either. They were re-issued without the append, and the record line was written with the Edit tool. The hazard: a guard refusal blocks the whole compound call, its other edits included.
7. **A sweep hazard: a crate's inline unit tests pin a table's count.** The testing.md 2026-10-10 rule names `tests/` and `examples/*/tests/` as the readers of a changed table. `escher-driver/src/json.rs`'s and `describe.rs`'s inline `#[cfg(test)]` modules pinned `CAUSES`' count (14 refusals; a causes row of 9) and read red at the first `cargo test -p escher-driver`. Evidence: implement's evolve record `2026-10-18T08:04:09Z-b` (`input.research-files-wrong`); the scope record's two companion lines.
8. **A measurement: `advance` now moves CSS animations, so a step's diff can be non-empty on any animated page.** The old `stand_act_timer` "empty diff after an advance" assertion read the clock-independence of `advance`, which F2 A removed.
9. **A measurement: the host toolchain deprecates `AtomicU64::fetch_update`** ("renamed to `try_update` for consistency", a rustc `deprecated` warning at implement P1), and `try_update` is not in MSRV 1.91. A hand CAS loop is the form that builds on both.
10. **A measurement: under `clippy -D warnings`, `cargo test -p escher-driver` builds blitz-paint with its known `variant Intrinsic is never constructed` warning.** This is the existing 2026-10-10 learning, re-met unchanged (implement P1, three per-package builds); not this chunk's.
11. **A sweep hazard, at this wrap's P1.**
    - The first `cascade.py sweep --pre-pass` was refused with exit 3: `title, new-checks never fired over the seven masters' pre-pass text (87f12079)`. The chunk's title and the two new check names are not yet in any master, as expected.
    - It was re-fired with `--pattern` naming the rest. `owner pattern: control unfired` is the title pattern's reading.
    - The `cli-advance` pattern's `` `advance` `` form reads one case variant (`case-variants +1`).

## Outcome

**Acceptance criteria**, re-asserted against the diff:
- **(tests) `verification-matrix.json#v020-06`: MET.** `stand_virtual_clock` passes in both modes: S2 == S1 across the 800 ms sleep; S3 ≠ S1; S4 ≠ S3; S5 == S4; S1 ≠ S4; and `harness().time()` == 1.7 s. Evidence: entry 5, green on the first firing and on the re-fire (5.2); `stand-repeat` 3 of 3 twice. The matrix has `v020-06` `implemented`, its ref `tests/blitz-tests/tests/stand_virtual_clock.rs::a_shot_after_a_scroll_repeats_until_advance_moves_the_fade`.
- **(arch) The census prints 0: MET.** Entry 8, `last line 0`. The diff adds the clock's reads in `clock.rs` alone.
- **(arch) A document built with no clock behaves as today: MET.** `virtual_clock_engine`'s `a_document_built_with_no_clock_fades_by_the_wall_clock` passes, and `fragment_navigation`, `scroll_into_view_nested`, `scrollbars` and `scrollbar_drag` pass in entry 6.
- **(design) MET.**
  - `virtual_clock_engine` passes in both modes: fade 1.0 at 0 and 500 ms, part faded at 600 ms, 0 and not animating at 700 ms; the smooth scroll stands until the clock moves and ends at 300 ms.
  - `FADE_DELAY`, `FADE_DURATION` and the smooth and fling constants are unchanged in the diff.
- **(a11y) MET.**
  - `stand_virtual_clock`'s `two_clicks_with_no_advance_between_them_are_a_double_click` passes: the double click differs from two single clicks and repeats across fresh sessions.
  - `ci-leg.sh a11y` is green, 6 · 43 · 0 · 0, with no a11y file edited.
- **(security) MET.**
  - `advance --ms 0` and `--ms 60001` are still refused `malformed`: validation is unchanged, and the `cli_commands` and schema tests are green.
  - The refusal set is eight fixed causes. Every one is returned by code: `stand_act_spans` reads each of the eight back from its own refused call (its doc and assertion say eight), and `stand_mcp_tools` serves seven through MCP, all but `unknown-verb`.
  - `describe()` lists eight (`describe.rs` pin 8).
  - Every help text is a fixed `'static` string.
- **(obs) MET.** An accepted `advance` leaves one `command` line with only the schema fields (`stand_act_spans`, the CRUD `advance` row now `Expect::Acts`). `stand_mcp_tools`, `host_log` and `mcp_log` pass (entries 6 and 7, and the `fast` leg).
- **(tests) MET.**
  - `stand_act_timer`, `host_timer` and `cli_commands` pass, with `advance` moving the clock by exactly what was asked.
  - `stand-repeat`: `stand-repeat: 3 of 3 rounds green` (entries 11 and 11.2).
  - Coverage: `coverage floor: 3 of 3 crates at or above their floors` (entry 13).
- **(tests) MET.** `fast` and `doc` pass, and fork CI reads `verdict: green` on the pushed `87f1207940e1`, run CI#39111579925 (17/17).

**The gates**, by `run`, at implement P2 (implement trail `gate-2026-10-18-one-virtual-clock.json`):

| `run` | verdict |
|---|---|
| `cargo test -p blitz-dom --locked --lib` | green · exit 0 |
| `cargo test -p escher-driver --locked` | green · exit 0 |
| `cargo test -p blitz-test-harness --locked` | green · exit 0 |
| `cargo test -p blitz-tests --locked --test virtual_clock_engine` | green · exit 0 (baseline red: the target was absent) |
| `cargo test -p blitz-tests --locked --test stand_virtual_clock` | green · exit 0 (baseline red: the target was absent); re-fired green |
| `cargo test -p blitz-tests --locked --test stand_act_timer --test stand_act_spans --test stand_mcp_tools …` (11 targets) | green · exit 0; re-fired green after the loose ends |
| `cargo test -p seven_guis --locked` | green · exit 0 |
| `awk '/Instant::now\(\)|SystemTime::now\(\)/ …'` (the census) | green · `exit 0` and `last line 0` held |
| `bash .github/scripts/ci-leg.sh fast` | green · exit 0; re-fired green |
| `bash .github/scripts/ci-leg.sh doc` | green · exit 0 |
| `bash .github/scripts/ci-leg.sh stand-repeat` | green · `contains stand-repeat: 3 of 3 rounds green`; re-fired green |
| `bash .github/scripts/ci-leg.sh a11y` | green · exit 0 |
| `bash .github/scripts/ci-leg.sh coverage` | green · `contains coverage floor: 3 of 3 crates at or above their floors` |
| `bash .github/scripts/ci-leg.sh audit` | green · exit 0 |

Summaries: `entries 17 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 3`; the re-fire `entries 17 · green 4 · red 0 · …`.

The three `leg = 'operator'` entries were fired by the operator pass and are recorded in `evidence/operator-pass.md`:
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` → `hygiene: clean — read 47 …`.
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.2.0` → green · exit 0 · `history moved: refs/remotes/origin/build/escher-0.2.0 f45bb570→87f12079`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` → `verdict: green · checks 17/17 … CI#39111579925`.

No `defer` and no deferral; every changed-surface gate ran. Smoke: `skipped — no boot-path change`. No binary or entry point was touched; the `escher-session` binary ran as a process in `cargo test -p seven_guis --locked`.

**Watches:** none folded.

**Outcome basis:**
- The operator pass ran. Setup 4's commit list from `f45bb570a` is the one `87f120794` (the pre-CI commit), and the final HEAD's CI run is CI#39111579925 (green, 17/17), recorded in `evidence/operator-pass.md`.
- Implement's report: `.andromeda/runs/2026-10-18T07-55-10-implement/outcome-2026-10-18-one-virtual-clock.md`, the only `-implement` dir for this marker, rewritten after the operator pass. This window holds that implement run whole.
- The operator directive between implement and this report: Decisions & corrections item 3.
- Post-implement artifacts: `evidence/operator-pass.md`, the re-fired gate entries (trail as above), and the pre-CI commit `87f12079`.
- The two edits after implement are in the pre-CI commit, and the scope record carries them.

**Process hygiene:** implement P4's census: `census: none running`, for every process the gate entries started (session hosts, MCP children, cargo). Re-measured here with `gate.py census --comm escher-session --comm seven_guis_native --comm cargo --args escher-session` against this wrap's run dir: `census: none running`.
## Addendum — resumed 2026-10-18
- inputs#I4 · message: the operator, 2026-10-18 08:53Z, at the wrap resume · copy · n/a — the resume directive: (a) the founder is away; the four delegate answers (F1 B · F2 A · F2b A · F3 A) stand PROVISIONAL and join the epoch batch for his word, marked PROVISIONAL where the masters state them, with no card waited on; (b) test-plan §9 takes the per-OS tallies from CI#39111579925, read at reconcile, beside the local `fast` and `stand-repeat` readings; (c) P5 places the merge-surface CARRY on «Upstream sync ahead of visual lint and real-app readiness»; then through to the commit, no next skill.
  - bears on **Expected amendments (from plan)**: adds to it — every master site that states a delegate answer's fact (entries 1, 2 and 3: `DocumentConfig.clock` and the one clock, `advance` on every session, eight causes, the fade's limit retired) marks it PROVISIONAL, for the founder's epoch batch beside the engine-debts and cursor-fix batches; no escalation card is raised for it at this wrap.
  - bears on **Counts / qualifiers moved**: adds to it — item 12's «The per-OS CI tallies of CI#39111579925 are not read at P1» is read at reconcile and recorded in `.andromeda/runs/2026-10-18T08-45-11-wrap/ci-tallies.md`; test-plan §9 Per-OS moves from CI#39104364530 to CI#39111579925 beside items 12 and 13.
  - bears on **Cross-project / external claims**: adds to it — the inputs#I3 card row 1's placement is confirmed by the directive (c) for P5.
- inputs.py verify, re-fired: `inputs: 4 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 4 · uncited 1 · unparsed 0`; moved since the report: a new entry I4, uncited before this addendum (I3 cited since the report: uncited 1 → I4 alone).
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: f45bb570 (the parent of the oldest pre-CI commit 87f12079) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### examples/seven_guis/src/stand.rs — added 47 line(s) in 9 range(s)
added: 89-95 · 97-100 · 129-131 · 133-138 · 140 · 142-146 · 148-152 · 154-155 · 160-173
  - 161-173 @162 «fn the_timer_step_takes_the_largest_advance() {»
### examples/seven_guis/tests/cli_commands.rs — added 5 line(s) in 3 range(s)
added: 7-8 · 401 · 403-404
### examples/seven_guis/tests/host_timer.rs — added 14 line(s) in 6 range(s)
added: 2-5 · 70 · 76 · 86 · 88 · 90-95
### packages/blitz-dom/src/clock.rs — new file · 114 line(s)
- 14-19 @15 «pub trait Clock: Send + Sync {»
- 21-25 @23 «pub struct WallClock {»
- 27-34 «impl WallClock {»
  - 28-33 @29 «pub fn new() -> Self {»
- 36-40 «impl Default for WallClock {»
  - 37-39 «fn default() -> Self {»
- 42-46 «impl Clock for WallClock {»
  - 43-45 «fn now(&self) -> Duration {»
- 48-55 @53 «pub struct VirtualClock {»
- 57-76 «impl VirtualClock {»
  - 58-61 @59 «pub fn new() -> Self {»
  - 63-75 @64 «pub fn advance(&self, by: Duration) {»
- 78-82 «impl Clock for VirtualClock {»
  - 79-81 «fn now(&self) -> Duration {»
- 84-114 @85 «mod tests {»
  - 88-97 @89 «fn a_virtual_clock_reads_the_sum_of_its_advances_on_every_clone() {»
  - 99-105 @100 «fn a_virtual_clock_saturates_rather_than_wrapping() {»
  - 107-113 @108 «fn a_wall_clock_never_reads_less_on_a_later_read() {»
### packages/blitz-dom/src/config.rs — added 5 line(s) in 3 range(s)
added: 1 · 71-73 · 92
### packages/blitz-dom/src/document.rs — added 26 line(s) in 10 range(s)
added: 18 · 20 · 70 · 264-265 · 317-318 · 328-330 · 503 · 1874-1875 · 1883-1884 · 1894-1904
  - 1897-1900 @1898 «pub(crate) fn elapsed_since(&self, stamp: Duration) -> Duration {»
### packages/blitz-dom/src/events/pointer.rs — added 5 line(s) in 5 range(s)
added: 4 · 266 · 392 · 403 · 713
### packages/blitz-dom/src/iframe.rs — added 1 line(s) in 1 range(s)
added: 75
### packages/blitz-dom/src/lib.rs — added 2 line(s) in 2 range(s)
added: 42 · 96
### packages/blitz-dom/src/resolve.rs — added 2 line(s) in 2 range(s)
added: 71 · 73
### packages/blitz-dom/src/scrolling.rs — added 2 line(s) in 2 range(s)
added: 514 · 865
### packages/blitz-test-harness/src/harness.rs — added 32 line(s) in 16 range(s)
added: 2 · 4-6 · 49-51 · 71 · 81-82 · 95-96 · 99 · 113-114 · 118 · 127-129 · 133 · 152 · 154 · 160-161 · 164-165
       167-172
- 4-6 «use blitz_dom::{»
### packages/escher-driver/src/describe.rs — added 2 line(s) in 2 range(s)
added: 145 · 149
### packages/escher-driver/src/execute.rs — added 12 line(s) in 6 range(s)
added: 3-4 · 106 · 121-123 · 178 · 181-184 · 190
### packages/escher-driver/src/json.rs — added 1 line(s) in 1 range(s)
added: 657
### packages/escher-driver/src/lib.rs — added 10 line(s) in 3 range(s)
added: 30-31 · 51-55 · 78-80
### packages/escher-driver/src/refusal.rs — added 5 line(s) in 5 range(s)
added: 35 · 208 · 219 · 280 · 315
### packages/escher-driver/src/schema.rs — added 9 line(s) in 5 range(s)
added: 161 · 192-194 · 250-252 · 257 · 279
  - 192-194 «help: "clicks the element an id names, then settles; a click on the same point within 500 ms \»
  - 250-252 «help: "moves the session's clock forward by `ms` milliseconds, then settles; on every session \»
### packages/escher-driver/src/schema_tests.rs — added 19 line(s) in 2 range(s)
added: 82-99 · 102
### packages/escher-driver/src/session.rs — added 10 line(s) in 4 range(s)
added: 21-23 · 116 · 118-122 · 125
### tests/blitz-tests/tests/session_common/mod.rs — added 2 line(s) in 1 range(s)
added: 94-95
### tests/blitz-tests/tests/stand_act_spans.rs — added 5 line(s) in 4 range(s)
added: 4 · 429 · 461-462 · 654
### tests/blitz-tests/tests/stand_act_timer.rs — added 74 line(s) in 22 range(s)
added: 3-9 · 14-15 · 19 · 41-44 · 46-47 · 51-52 · 57 · 69 · 80 · 82-83 · 90 · 101 · 108 · 116-118 · 120-133 · 137
       161-164 · 166 · 168-169 · 177-182 · 188-189 · 192-206
### tests/blitz-tests/tests/stand_mcp_tools.rs — added 50 line(s) in 6 range(s)
added: 8-9 · 14-15 · 435 · 461-463 · 481-521 · 593
### tests/blitz-tests/tests/stand_non_lean_tasks.rs — added 3 line(s) in 1 range(s)
added: 82-84
### tests/blitz-tests/tests/stand_screenshot_repeat.rs — added 2 line(s) in 1 range(s)
added: 5-6
### tests/blitz-tests/tests/stand_virtual_clock.rs — new file · 133 line(s)
- 32-39 @34 «fn acts(session: &mut Session, call: &Call, at: &str) {»
  - 35-37 «let Some(acted) = act(session, call) else {»
- 41-48 @43 «fn png(session: &mut Session, id: &str, at: &str) -> Vec<u8> {»
  - 44-46 «let Some(shot) = shot(session, &screenshot_of(id)) else {»
- 50-96 @51 «fn a_shot_after_a_scroll_repeats_until_advance_moves_the_fade() {»
  - 52-95 «for incremental in [false, true] {»
- 98-115 @101 «fn clicked_twice(incremental: bool, between: Option<i64>, at: &str) -> Vec<u8> {»
  - 103-107 «acts(»
  - 110-112 «if let Some(ms) = between {»
- 117-133 @118 «fn two_clicks_with_no_advance_between_them_are_a_double_click() {»
  - 119-132 «for incremental in [false, true] {»
### tests/blitz-tests/tests/virtual_clock_engine.rs — new file · 234 line(s)
- 21-27 «const PAGE: &str = r#"<html><body style="margin:0">»
  - 22-24 «<div id="scroller" style="width:200px; height:100px; overflow-y:auto;">»
  - 25-26 «<input id="field" value="alpha beta"»
- 29-45 @30 «fn config(incremental: bool, clock: Option<&VirtualClock>) -> DocumentConfig {»
  - 32-44 «DocumentConfig {»
- 47-51 «fn document(incremental: bool, clock: Option<&VirtualClock>) -> HtmlDocument {»
- 53-57 «fn scroller(doc: &HtmlDocument) -> NodeId {»
  - 54-56 «doc.query_selector("#scroller")»
- 59-64 «fn offset_y(doc: &HtmlDocument, node: NodeId) -> f64 {»
  - 60-63 «doc.get_node(node)»
- 66-75 @68 «enum Thumb {»
- 77-119 @78 «fn the_scrollbar_fade_moves_only_with_the_clock() {»
  - 79-85 @80 «let rows = [»
  - 87-118 «for incremental in [false, true] {»
- 121-132 @122 «fn selection(harness: &Harness<HtmlDocument>) -> (usize, usize) {»
  - 125-129 «let data = doc»
- 134-143 @135 «fn field_text(harness: &Harness<HtmlDocument>) -> String {»
  - 138-142 «doc.get_node(field)»
- 145-180 @146 «fn two_clicks_are_a_double_click_by_the_clock_alone() {»
  - 150-179 «for incremental in [false, true] {»
- 182-216 @183 «fn a_smooth_scroll_stands_still_until_the_clock_moves() {»
  - 184-215 «for incremental in [false, true] {»
- 218-234 @219 «fn a_document_built_with_no_clock_fades_by_the_wall_clock() {»
  - 220-233 «for incremental in [false, true] {»
