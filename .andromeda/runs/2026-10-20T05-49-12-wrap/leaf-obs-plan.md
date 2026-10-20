group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»

- leaf: .claude/docs/obs-summary.md:49   class: retired
  leaf-text: «record no field (PROVISIONAL: no new field), escher-driver keeping its 2 `info_span!` sites»
  source: .andromeda/obs-plan.md:121 «which open no span, emit no event and record no field, escher-driver keeping its 2 `info_span!` sites»
  replace-with: «record no field, escher-driver keeping its 2 `info_span!` sites»

- leaf: .claude/docs/obs-summary.md:49   class: retired
  leaf-text: «the focus-move redraw (PROVISIONAL)»
  source: .andromeda/obs-plan.md:77 «the engine changes they pin, `:focus-within` matching, the not-in-document focus refusal and the focus-move redraw, add no log site, span, field or env read»
  replace-with: «the focus-move redraw»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «the fourteen `stand_act_*` checks — `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth —»
  source: .andromeda/obs-plan.md:70 «for the fifteenth driver-action check, `stand_act_select`, and the engine's select check `select_interaction`»
  replace-with: «the fifteen `stand_act_*` checks — `stand_act_screenshot` the thirteenth since 2026-10-12-headless-screenshot, `stand_act_transformed` the fourteenth, `stand_act_select` the fifteenth —»

- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «`host_log` and `mcp_log` passing under both builds. The driver»
  source: .andromeda/obs-plan.md:82 «- The select checks are silent: `select_interaction` (a `fixtures` reader) and `stand_act_select` (a `fixtures` and `session_common` reader), both layout modes, install no subscriber, read no env var»
  replace-with: «`host_log` and `mcp_log` passing under both builds; the select checks `select_interaction` (a `fixtures` reader) and `stand_act_select` (a `fixtures` and `session_common` reader), both layout modes, likewise install no subscriber, read no env var and print nothing, their readings going into assertions only, and the select model they pin (blitz-dom's crate-private `node::select`, the keyboard arm and its typeahead on the document's clock, the click arm, the mutator's select settle, `DomEventKind::Change`) adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites. The driver»
