group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «one of the nine names `escher_driver::Cause::name` returns, for a refused call — `fonts-unavailable` the ninth since 2026-10-12-headless-screenshot»
  source: .andromeda/obs-plan.md:114 «one of the eight names `Cause::name` returns (`unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`, PROVISIONAL)»
  replace-with: «one of the eight names `escher_driver::Cause::name` returns, for a refused call — `time-unavailable` no longer among them (PROVISIONAL)»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «record `verb` alone, and a shot»
  source: .andromeda/obs-plan.md:114 «an accepted `advance` — accepted on every session (PROVISIONAL) — records `verb` and the acting fields like every acting verb, read by `stand_act_spans`' CRUD `advance` row (`Expect::Acts`)»
  replace-with: «record `verb` alone, an accepted `advance` — accepted on every session (PROVISIONAL) — records `verb` and the acting fields like every acting verb (its MCP `tool_call` ending `accepted` and parenting that `command`), and a shot»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and the driver's reworded fixed texts add no log site, span, field, log field or env read.»
  source: .andromeda/obs-plan.md:75 «The clock checks are silent: `virtual_clock_engine` and `stand_virtual_clock` (a `session_common` reader) install no subscriber, read no env var and print nothing, their readings going into assertions only»
  replace-with: «and the driver's reworded fixed texts add no log site, span, field, log field or env read; the clock checks `virtual_clock_engine` and `stand_virtual_clock` (a `session_common` reader) likewise install no subscriber, read no env var and print nothing, their readings going into assertions only, and the document's clock and `Harness::advance_time` add no log site, span, field, log field or env read.»
