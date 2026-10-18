group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 58 · «- 2026-10-15: In a contract test whose expected total follow»

- leaf: .claude/docs/tests-summary.md:13   class: stale
  leaf-text: «`wrap(doc)` keeping the clock the document's own config gave it; PROVISIONAL)»
  source: .andromeda/test-plan.md:89 «`advance_time(Duration)` advances it by exactly the duration, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified by the founder, 2026-10-18)»
  replace-with: «`wrap(doc)` keeping the clock the document's own config gave it; ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/tests-summary.md:14   class: stale
  leaf-text: «`advanced_ms` exactly `ms`, the clock's move (PROVISIONAL).»
  source: .andromeda/test-plan.md:287 «(250 then 250 → 2 then 3 ticks); `advanced_ms` is exactly `ms`, the clock's move (ratified by the founder, 2026-10-18), not what the Timer's counter gained»
  replace-with: «`advanced_ms` exactly `ms`, the clock's move (ratified by the founder, 2026-10-18).»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «two clicks with no `advance` between them a double click; PROVISIONAL)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «a shot after a scroll repeats until `advance` moves the fade, and two clicks with no `advance` between them are a double click (ratified by the founder, 2026-10-18)»
  replace-with: «two clicks with no `advance` between them a double click; ratified by the founder, 2026-10-18)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «(`the_eight_causes_are_named_in_order`, PROVISIONAL)»
  source: .andromeda/test-plan.md:22 «refusal.rs 3 (the causes are named in order — eight (`the_eight_causes_are_named_in_order`, ratified by the founder, 2026-10-18), `fonts-unavailable` last;»
  replace-with: «(`the_eight_causes_are_named_in_order`, ratified by the founder, 2026-10-18)»

- leaf: .claude/rules/testing.md:31   class: stale
  leaf-text: «the wrapped document keeps the one its own config gave it (PROVISIONAL).»
  source: .andromeda/test-plan.md:89 «`advance_time(Duration)` advances it by exactly the duration, each then pumping; `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (ratified by the founder, 2026-10-18)»
  replace-with: «the wrapped document keeps the one its own config gave it (ratified by the founder, 2026-10-18).»
