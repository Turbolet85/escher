proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Document core (blitz-dom)"
    change: "Add `clock` to the `DocumentConfig` fields list: `clock: Option<Arc<dyn Clock>>`, `None` in `Default` and then a fresh `WallClock` (wall time, as before); the document reads it for the overlay scrollbar fade, the double-click window, pan and smooth-scroll starts and the fling and smooth frames, and a sub-document's config carries its parent's clock. Register blitz-dom's new crate-root `Clock` (`Send + Sync`, `now() -> Duration`, monotonic from its own origin), `WallClock` (a `web_time::Instant` origin; `new`, `Default`) and `VirtualClock` (`Clone`, `Default`, every clone reading one `Arc<AtomicU64>` of nanoseconds; `new()` reads zero, `advance(Duration)` adds with saturation) — PROVISIONAL, the delegate operator's answer for the founder's epoch batch."
    sidecar: "architecture §Standard Contracts Document core: `DocumentConfig.clock` (wall time when `None`) and blitz-dom's `Clock` · `WallClock` · `VirtualClock` registered, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "The report's Symbols / APIs lists the new public clock types re-exported from blitz-dom's root and the new config key `DocumentConfig.clock`; Schema / config names it a new config key; the contract's field list (viewport … subdocument_depth) does not carry it — an unregistered API. Addendum inputs#I4 (a) marks the one clock PROVISIONAL."
    basis: "packages/blitz-dom/src/config.rs:71-73; packages/blitz-dom/src/clock.rs:14-82; packages/blitz-dom/src/lib.rs:96; .andromeda/architecture.md (Document core bullet, `DocumentConfig` fields)"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Test harness"
    change: "State that the harness's time is a `VirtualClock` its documents share: `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`, so `from_html_with`, `from_vdom` and through them `from_html` and `from_component` build documents that read it; `wrap(doc)` gives the harness a fresh `VirtualClock` while the wrapped document keeps its own config's clock; `time()` returns the clock's reading in seconds and `pump` resolves at it; `tick(dt)` advances it (a negative or non-finite `dt` moves nothing); and the new `advance_time(&mut self, by: Duration)` advances by exactly `by`, then pumps, and does not settle; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` still never move the clock."
    sidecar: "architecture §Standard Contracts Test harness: the harness's time is the documents' shared `VirtualClock`; `Harness::advance_time` registered (2026-10-18-one-virtual-clock)."
    rationale: "The report's Symbols / APIs (blitz-test-harness `Harness`) and Harness / gate surface list the new `Harness::advance_time(Duration)` and `time()` / `tick` reading and moving the shared `VirtualClock`; the contract lists the harness's API without them — an unregistered API (Counts item 6, `harness-time`, architecture.md:157 changed)."
    basis: "packages/blitz-test-harness/src/harness.rs:49-51; packages/blitz-test-harness/src/harness.rs:81-82; packages/blitz-test-harness/src/harness.rs:127-133; packages/blitz-test-harness/src/harness.rs:167-172; .andromeda/architecture.md:157"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Test harness"
    change: "Restate the sentence '`pump`, `tick`, `dispatch` and every input helper are unchanged: none settles' so it no longer calls `tick` unchanged: `tick` now advances the shared `VirtualClock` its documents read, `advance_time` joins it, and none of `pump`, `tick`, `advance_time`, `dispatch` or the input helpers settles."
    sidecar: "architecture §Standard Contracts Test harness: '`tick` … unchanged' retired — `tick` moves the documents' clock (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 6 names this clause as changed (architecture.md:157, '`pump`, `tick` … unchanged'); the Harness bullets of Symbols / APIs say `tick(dt)` advances the clock the documents read."
    basis: ".andromeda/architecture.md:157 ('`pump`, `tick`, `dispatch` and every input helper are unchanged')"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Restate `with_time` as `with_time(self, step: impl FnMut(&mut Harness<DioxusDocument>, u32) + 'static) -> Session` with no return: the session moves the harness's clock on every `advance`, and the step, handed the held harness and the milliseconds asked, moves the app's own time — PROVISIONAL, the delegate operator's answer for the founder's epoch batch."
    sidecar: "architecture §Standard Contracts Driver session: `with_time`'s step returns nothing; the session moves the clock, the step the app's own time, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Symbols / APIs (escher-driver, the time step): `TimeStep` and `with_time` take a step with no return, and the doc says the session moves the clock on every `advance`; Counts item 3 (`with-time`) names the architecture.md:160 `with_time` signature as changed; inputs#I4 (a) marks `advance` on every session PROVISIONAL."
    basis: "packages/escher-driver/src/session.rs:21-23; packages/escher-driver/src/session.rs:116-125; .andromeda/architecture.md:160 ('-> u32 + 'static) -> Session … it returns the milliseconds the app moved')"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace '`advance` asks for the app's time to move by `ms` milliseconds; the step that moves it is the session's caller's to supply, through `Session::with_time`, and a session that carries none refuses `advance` as `time-unavailable`' with: `advance` moves the session's clock forward by `ms` milliseconds on every session — the engine's animations, the overlay scrollbar fade and, through the step a caller hands over with `Session::with_time` where there is one, the app's own time together — then settles; two `click`s on one point with no `advance` of 500 ms between them read as a double click (the `click` help says so). PROVISIONAL."
    sidecar: "architecture §Standard Contracts Driver session: `advance` accepted on every session, moving the one clock; `time-unavailable` gone, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Symbols / APIs (escher-driver `advance`: accepted on every session; help texts for `advance` and `click`) and Counts items 1, 3 and 8 (`with-time`, `cli-advance`, `time-unavail` at architecture.md:160)."
    basis: "packages/escher-driver/src/execute.rs:178; packages/escher-driver/src/execute.rs:181-184; packages/escher-driver/src/schema.rs:192-194; packages/escher-driver/src/schema.rs:250-252; .andromeda/architecture.md:160"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Replace '`advance` on a session that carries no time step is refused `time-unavailable` with nothing run; otherwise it hands `ms` to the step, and `advanced_ms` is the lesser of the step's return and `ms`' with: `advance` advances the harness's clock by `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed, and `advanced_ms` is exactly `ms`, the clock's move — keeping that on the stand's Timer an `advance` of 60000 reports 60000 while the elapsed display stops at its 15 s duration; a step's diff can be non-empty on any animated page. PROVISIONAL."
    sidecar: "architecture §Standard Contracts Driver session: `advanced_ms` reads exactly `ms`; no `advance` is refused for want of a step, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 2 (`advanced-ms`: architecture.md:160 'the lesser of the step's return and `ms`' changed) and item 3; Symbols / APIs: `advance` reports `advanced_ms = ms`; Decisions item 8: `advance` now moves CSS animations."
    basis: "packages/escher-driver/src/execute.rs:181-184; packages/escher-driver/src/execute.rs:190; .andromeda/architecture.md:160 ('`advanced_ms` is the lesser of the step's return and `ms`')"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "Turn the clause 'a call `validate` refuses, and an `advance` refused `time-unavailable`, record nothing (the operator, 2026-10-07 …)' into history: a call `validate` refuses records nothing, and until 2026-10-18-one-virtual-clock neither did an `advance` refused `time-unavailable`, a refusal no longer returned."
    sidecar: "architecture §Standard Contracts Driver session: the 2026-10-07 record-of-ids clause on a `time-unavailable` refusal made history (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 1: 'architecture.md:160 … the 2026-10-07 ruling that an `advance` refused `time-unavailable` records nothing. It becomes history.'"
    basis: ".andromeda/architecture.md:160 ('an `advance` refused `time-unavailable`, record nothing')"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Headless stand (seven_guis, native only)"
    change: "Restate `timer_step(ticks: TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32)` with no return: handed the milliseconds asked, it delivers the whole ticks they hold through `TimerTicks::deliver`, carrying the sub-tick remainder across calls (250 then 250 deliver 2 then 3 ticks); the session, not the step, moves the harness's clock — retiring 'returns the milliseconds those ticks stand for — the remainder below one tick is dropped per call … the harness's animation clock stays where it was'."
    sidecar: "architecture §Standard Contracts Headless stand: `timer_step` returns nothing and carries its remainder; the session moves the clock (2026-10-18-one-virtual-clock)."
    rationale: "Report Symbols / APIs (seven_guis `stand::timer_step`) and Counts items 3, 4 and 6 (`with-time`, `timer-step`, `harness-time`: architecture.md:158 changed)."
    basis: "examples/seven_guis/src/stand.rs:89-100; .andromeda/architecture.md:158"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → CLIs"
    change: "After 'the other three tasks' carrying no time step', add that those three still accept `advance`, which moves their session's clock alone."
    sidecar: "architecture §Standard Contracts CLIs: the three tasks with no time step still take `advance` (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 3 (`with-time`): architecture.md:165 changed — 'the other three tasks carrying no time step now still `advance`'; Symbols / APIs: `advance` accepted on every session."
    basis: ".andromeda/architecture.md:165 ('the other three tasks' carrying no time step')"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`Cause` is closed at eight, `CAUSES: [Cause; 8]` — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable` — `time-unavailable` removed at 2026-10-18-one-virtual-clock (PROVISIONAL), `fonts-unavailable` now the eighth; `describe()` lists eight."
    sidecar: "architecture §Standard Contracts Driver session: refusal causes nine → eight, `time-unavailable` retired, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Symbols / APIs ('`Cause::TimeUnavailable` is removed. `CAUSES: [Cause; 8]`'), Schema / config (the refusal set closes at eight) and Counts item 1 (`causes-nine`, `time-unavail`: architecture.md:160 changed)."
    basis: "packages/escher-driver/src/refusal.rs:35; packages/escher-driver/src/describe.rs:145; .andromeda/architecture.md:160 ('`Cause` is closed at nine')"
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "'All nine are returned by code: `validate` returns `unknown-verb` and `malformed` only, and the executor returns the other seven' → all eight are returned by code, the executor returning the other six — `not-found`, `stale`, `disabled`, `covered`, `off-screen` and `fonts-unavailable`."
    sidecar: "architecture §Standard Contracts Driver session: all eight causes returned by code, six by the executor (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 1 (`causes-nine` at architecture.md:160) and Outcome (security): 'The refusal set is eight fixed causes. Every one is returned by code'."
    basis: ".andromeda/architecture.md:160 ('All nine are returned by code')"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → Driver session (escher-driver)"
    change: "`screenshot`'s help now states one measured limit (an element shot is cropped to the part of its bounds that can be seen); drop 'in a build that paints overlay scrollbars a thumb painted within 0.7 s of a scroll fades by wall time' — the fade reads the session's clock, so two shots of one state read the same pixels until an `advance` moves it (PROVISIONAL)."
    sidecar: "architecture §Standard Contracts Driver session: the screenshot help's wall-time fade limit retired, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Symbols / APIs (help texts: '`screenshot` drops its sentence …') and Counts item 5 (`wall-fade`, `help-text`: architecture.md:160 changed); Outcome (tests): `stand_virtual_clock` S2 == S1 across an 800 ms sleep."
    basis: "packages/escher-driver/src/schema.rs:279; .andromeda/architecture.md:160 ('a thumb painted within 0.7 s of a scroll fades by wall time')"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Scrolling and input]"
    change: "Qualify 'repeated clicks within 500 ms and 2 px' (and the smooth-scroll and fling timings beside it) as read on the document's clock — `DocumentConfig.clock`, wall time when unset, a harness's `VirtualClock` headlessly — with the values unchanged; no `Instant::now()` or `SystemTime::now()` is left in blitz-dom outside `clock.rs`, the epoch `unwrap`s gone."
    sidecar: "architecture §Established Decisions [Scrolling and input]: the double-click window, smooth scroll and fling read the document's clock, values unchanged (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 8 (`dblclick`: architecture.md:86 changed — the clock qualifier) and Symbols / APIs (the five engine time reads go through the document's clock; the census prints 0)."
    basis: "packages/blitz-dom/src/events/pointer.rs:392; packages/blitz-dom/src/events/pointer.rs:403; packages/blitz-dom/src/scrolling.rs:514; packages/blitz-dom/src/scrolling.rs:865; .andromeda/architecture.md:86"
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Scrolling and input] (detail: dec-scrolling-and-input)"
    change: "Beside the seven direct engine changes, record the document clock (`DocumentConfig.clock`, an API addition with wall time the default behaviour, PROVISIONAL), flagged upstreamable with no upstream pull request, and extend the merge-surface CARRY on «Upstream sync ahead of visual lint and real-app readiness» to own its files too: `packages/blitz-dom/src/{config,clock,document,resolve,iframe,scrolling}.rs` and `events/pointer.rs`."
    sidecar: "architecture §Established Decisions [Scrolling and input] detail: the clock's merge surface joins the upstream-sync CARRY, PROVISIONAL (2026-10-18-one-virtual-clock)."
    rationale: "Report Counts item 19 (`carry-owner`, `owned-by`: architecture.md:90 changed, the merge surface by file), the P5 card row 1 accepted by the operator (inputs#I3) and confirmed by addendum inputs#I4 (c); Expected amendments item 1 names the pattern «An engine behaviour change for every document» (an API addition with default behaviour unchanged), PROVISIONAL."
    basis: ".andromeda/architecture.md:90 ('their merge surface, by file, is owned by a CARRY on «Upstream sync ahead of visual lint and real-app readiness»')"
note: "No new socket, port, env var, IPC endpoint or workspace crate (report: Crates / modules adds only blitz-dom's `clock` module; Dependencies none — web-time already a blitz-dom dependency), so §Occupied Resources needs no entry. architecture.md:121/:161 (blitz-vibey-script's own clock), :157 'reads no clock but the harness's own', :285, :290, :291 and the 60000-reports-60000 Timer reading stay true per the report's no-change rows. The seven D-arch-resources dependents attach to three distinct primaries: the with_time/advance claim (dependents on :160 ×3, :158, :165), the harness clock (:157 '`tick` unchanged'), and the nine causes (:160 'All nine')."
