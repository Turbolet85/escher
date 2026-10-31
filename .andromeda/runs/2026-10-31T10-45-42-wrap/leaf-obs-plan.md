group: obs-plan
read: .claude/docs/obs-summary.md · 59 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
read: .claude/rules/observability.md · 41 · «- 2026-10-06: With the workspace's tracing 0.1.44, `tracing:»
- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «driving the four lean stand tasks and its own fixtures in process in both layout modes»
  source: .andromeda/obs-plan.md:77 «which drives the four lean stand tasks, the load page and Cells, and its own fixtures in process in both layout modes»
  replace-with: «driving the four lean stand tasks, the load page and Cells, and its own fixtures in process in both layout modes»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «no driver schema, span field or log line moving (2026-10-25-cold-agent-bar)»
  source: .andromeda/obs-plan-detail/log-coverage.md:131 «observed absent — a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event`»
  replace-with: «no driver schema, span field or log line moving (2026-10-25-cold-agent-bar); the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` hold no print, env read, subscriber or listener, the silence census reading 0 (2026-10-31-engine-debts)»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «and the driver's reworded fixed texts add no log site, span, field, log field or env read»
  source: .andromeda/obs-plan-detail/log-coverage.md:74 «the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines»
  replace-with: «and the driver's reworded fixed texts add no log site, span, field, log field or env read, and the transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines (2026-10-31-engine-debts)»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites»
  source: .andromeda/obs-plan-detail/log-coverage.md:75 «`BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site or print»
  replace-with: «adds no log site, span, field, log field or env read and reads no wall clock, escher-driver keeping its 2 `info_span!` sites, and the readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site or print (2026-10-31-engine-debts)»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «the `Focussed node` INFO line standing after the new gate»
  source: .andromeda/obs-plan-detail/log-coverage.md:71 «info: focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name; the focus fixup adds no log site, and neither»
  replace-with: «the `Focussed node` INFO line standing after the new gate — once per focus move under the workspace build, holding a node number and no id or name, the focus fixup adding no log site and neither `clear_focus` nor the flush printing (2026-10-31-engine-debts)»
- leaf: .claude/rules/observability.md:36   class: omission
  leaf-text: «one engine line, `blitz_dom::document` at INFO, printed twice from `info` up under the workspace build»
  source: .andromeda/obs-plan-detail/log-coverage.md:71 «info: focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name»
  replace-with: «one engine line, `blitz_dom::document` at INFO — the focussed-node line, once per focus move, holding a node number and no id or name — printed twice from `info` up under the workspace build»
- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «A shot's image (since 2026-10-12-headless-screenshot) is screen content»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14 «A `multiple` select's chosen set leaves only as form entries — on `input` and `change` one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`) — as the snapshot's `value`»
  replace-with: «A `multiple` select's chosen set leaves only as form entries — on `input` and `change` one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`) — as the snapshot's `value`, the first chosen option in tree order, and as the options' tree nodes, never as a log line or field, the added-lines census reading 0 (2026-10-31-engine-debts). A shot's image (since 2026-10-12-headless-screenshot) is screen content»
- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «A lint's line is `span="command" verb="lint"`»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15 «An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted»
  replace-with: «An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (2026-10-31-engine-debts). A lint's line is `span="command" verb="lint"`»
- leaf: .claude/docs/obs-summary.md:43   class: omission
  leaf-text: «a file, never a log line, field or print.»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:16 «a page's script hears one `change`, from the engine, the script runtime synthesising none; the added lines hold no log site, print or env read»
  replace-with: «a file, never a log line, field or print. The engine's `change` on a checkbox or radio, sent after `input` with the same value, a checkbox always and a radio only where it was not checked before the click, carries the value class `input` already carries; a page's script hears one `change`, from the engine, the script runtime synthesising none; the added lines hold no log site, print or env read, the added-lines census reading 0 (2026-10-31-engine-debts).»
