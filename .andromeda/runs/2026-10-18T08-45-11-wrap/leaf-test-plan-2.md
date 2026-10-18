group: test-plan-2
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/verification-harness.md:36   class: stale
  leaf-text: «the remainder dropped, the harness clock untouched»
  source: .andromeda/test-plan.md:287 «carrying the remainder below one tick across calls (250 then 250 → 2 then 3 ticks); `advanced_ms` is exactly `ms`, the clock's move (PROVISIONAL)»
  replace-with: «the remainder below one tick carried across calls; `advance` first moves the session's clock — the harness's `VirtualClock` — by exactly `ms`, and `advanced_ms` reads exactly `ms`, PROVISIONAL»

- leaf: .claude/rules/verification-harness.md:31   class: omission
  leaf-text: «`pump` polls with no waker and resolves at harness time;»
  source: .andromeda/test-plan.md:89 «The harness's time is a `VirtualClock` its documents read: `time` reads it in seconds and `pump` resolves at it; `tick(dt)` advances it, a negative or non-finite `dt` moving nothing»
  replace-with: «`pump` polls with no waker and resolves at harness time — a `VirtualClock` its documents read, advanced by `tick(dt)` (a negative or non-finite `dt` moving nothing) and by exactly the duration by `advance_time(Duration)`, each then pumping, and never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle` (PROVISIONAL);»

- leaf: .claude/rules/verification-harness.md:35   class: omission
  leaf-text: «harness documents always use `HtmlProvider`.»
  source: .andromeda/test-plan.md:258 «`wrap(doc)` gives the harness a fresh `VirtualClock` and the wrapped document keeps the clock its own config gave it»
  replace-with: «harness documents always use `HtmlProvider`; `HarnessOptions::into_config(self, &VirtualClock)` puts a clone of the harness's clock in `DocumentConfig.clock`, so documents built by `from_html_with` and `from_vdom` (and through them `from_html` and `from_component`) read the harness's time, while `wrap(doc)` gives the harness a fresh `VirtualClock` and the wrapped document keeps the clock its own config gave it.»

- leaf: .claude/rules/verification-harness.md:32   class: stale
  leaf-text: «it reads no clock, advances no time»
  source: .andromeda/test-plan.md:90 «it reads no clock but the harness's, advances no time, waits on no load»
  replace-with: «it reads no clock but the harness's, advances no time»
