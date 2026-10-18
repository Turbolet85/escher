group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:22   class: omission
  leaf-text: «and two read-only readers, `can_paint_text`»
  source: .andromeda/architecture.md:139 «The document's clock (PROVISIONAL): `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`; a document holds the config's clock, else a fresh `WallClock` — wall time, as before»
  replace-with: «the document's clock (PROVISIONAL: `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` in `Default`, a document holding the config's clock or else a fresh `WallClock` — wall time, as before — and reading it for every engine time read: the overlay scrollbar fade, the double-click window (500 ms and 2 px), pan and the fling's generation, the smooth-scroll start and the fling and smooth frames; a sub-document's config carries its parent's clock; the crate root re-exports `Clock`, `WallClock` and `VirtualClock`, and no `Instant::now()` or `SystemTime::now()` stands in blitz-dom outside `clock.rs`), and two read-only readers, `can_paint_text`»

- leaf: CLAUDE.md:35   class: omission
  leaf-text: «headless `Harness`: construction, pump, settle»
  source: .andromeda/architecture.md:169 «The harness's time (PROVISIONAL): the harness holds a `VirtualClock` its documents share — `HarnessOptions::into_config(self, &VirtualClock)` puts a clone in `DocumentConfig.clock`»
  replace-with: «headless `Harness`: construction, its time (PROVISIONAL: a `VirtualClock` its documents share — `HarnessOptions::into_config` puts a clone in `DocumentConfig.clock`, a `wrap`ped document keeping its own config's clock; `time()` reads it in seconds, `tick(dt)` and `advance_time(Duration)` advance it, each then pumping, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it), pump, settle»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «and nine refusal `Cause`s each with a fixed remedy (`fonts-unavailable` the ninth, returned by `screenshot` alone)»
  source: .andromeda/architecture.md:175 «`Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, PROVISIONAL) — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`»
  replace-with: «and eight refusal `Cause`s each with a fixed remedy (`fonts-unavailable` the eighth, returned by `screenshot` alone; PROVISIONAL)»

- leaf: CLAUDE.md:37   class: retired
  leaf-text: «`advance` moves time through the step a caller hands over with `Session::with_time` (`time-unavailable` without one)»
  source: .andromeda/architecture.md:175 «`advance` moves the session's clock forward by `ms` milliseconds on every session — the engine's animations, the overlay scrollbar fade and, through the step a caller hands over with»
  replace-with: «`advance` moves the session's clock — the harness's `VirtualClock` its documents read — by exactly `ms` on every session inside one settled step, then the app's own time through the step a caller hands over with `Session::with_time` where there is one, `advanced_ms` exactly `ms` and a step's diff possibly non-empty on any animated page; two `click`s on one point with no `advance` of at least 500 ms between them read as a double click (PROVISIONAL)»

- leaf: CLAUDE.md:75   class: omission
  leaf-text: «`packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` · range input»
  source: .andromeda/architecture.md:139 «every clone reading one `Arc<AtomicU64>` of nanoseconds; `new()` reads zero, `advance(Duration)` adds with saturation) (packages/blitz-dom/src/clock.rs:14-82; packages/blitz-dom/src/lib.rs:96)»
  replace-with: «`packages/blitz-dom/src/document.rs` · `packages/blitz-dom/src/config.rs` · the document's clock `packages/blitz-dom/src/clock.rs` (`Clock` · `WallClock` · `VirtualClock`, unit tests inline; checks `tests/blitz-tests/tests/{virtual_clock_engine,stand_virtual_clock}.rs`) · range input»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and nine refusal causes each with a fixed remedy, all nine returned today»
  source: .andromeda/architecture.md:175 «All eight are returned by code: `validate` returns `unknown-verb` and `malformed` only, and the executor returns the other six»
  replace-with: «and eight refusal causes each with a fixed remedy, all eight returned today (PROVISIONAL)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «`advance` moves the app's time through the step the session's caller supplies»
  source: .andromeda/architecture.md:175 «`advance` moves the session's clock forward by `ms` milliseconds on every session — the engine's animations, the overlay scrollbar fade and, through the step a caller hands over with»
  replace-with: «`advance` moves the session's clock — the harness's `VirtualClock` every harness-built document reads — by exactly `ms` on every session and the app's own time through the step the session's caller supplies where there is one (PROVISIONAL)»
