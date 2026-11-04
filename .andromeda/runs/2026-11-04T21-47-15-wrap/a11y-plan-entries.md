@amendment marker=2026-11-04-dioxus-runtime-probe
title: the «reacts to a click» reading measured by link 2
section: §7 Screen Reader Support (body: the actionable-key rule; screen-reader-support.md: the actionable-key rule, a new row for the link-2 reading)
change:
  - That an element reacts to a click is a measured reading, its snapshot field owed to the route entry «The UI graph», in the body's line and the detail's actionable-key rule (was owned by a `.andromeda/residuals.md` line, target: next; now measured by link 2, its field owed to «The UI graph»).
  - A new detail row states link 2's reading, in both layout modes: on Cells the 677 elements carrying the listener mark and stating no role are the 676 cells reacting to `click` and `cells-grid` reacting to `keydown`; on Counter `counter-increment` carries `click` and `counter-value` none; CRUD's `crud-list` carries `keydown`; no snapshot field is added (`NODE_FIELDS` thirteen).
why: the report's Counts (the «reacts to a click» reading measured as link 2's answer, no snapshot field added) and its a11y Outcome, met; the field's place with the UI graph is the founder's answer to the probe's third fork, by dialog in the overseer session, relayed by the overseer and then by the operator at the wrap's resume (2026-11-04).
kept: the founder's C3 ruling (2026-10-14) deferring the field, and the stand's count as the reading's starting point, stand as written.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the listener mark's over-report and the actionable-key check
section: §7 Screen Reader Support (body: the actionable-key rule; screen-reader-support.md: the actionable-key rule's listener-mark clause, a new row for the bridge's listener attributes)
change:
  - The actionable-key check's listener mark `data-dioxus-id` also holds an element whose listener a re-render removed, its mark still standing, so the check over-includes (conservative), while a `mounted` listener writes none — read, not run; the body's line states the over-inclusion.
  - A new detail row: the bridge's listener attributes over-report — a listener removed on a re-render leaves its event attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none; the vdom walk's dynamic attributes (`VNode.dynamic_attrs`, `VNode::mounted_dynamic_attribute`, public dioxus-core 0.7.10 API) read each element's current listeners exactly, `mounted` included.
why: the report's Counts (the listener reading's limit, measured; its consequence on the actionable-key check read, not run) and its measured fact on the vdom walk's dynamic attributes; the plan's prediction that the attribute stays after the listener's removal measured true.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the stand's graph re-read on pulldown-cmark 0.13.6
section: §1 A11y Scope Summary (a11y-scope-summary.md: the platform-adapter entry)
change:
  - A re-read appended: `Cargo.lock`'s move of `pulldown-cmark` to 0.13.6, no crate added or removed and no `Cargo.toml` changed, Dioxus at the `"0.7.3"` requirement — no platform adapter joins the stand's graph, so no stand binary builds a platform adapter still.
  - The adapter on the stand binary and its windowed AT witness are owned by a `residual` row in the store that reads `re-carried:0.2.0` (was a `.andromeda/residuals.md` line).
why: the report's Dependencies (pulldown-cmark 0.13.5 to 0.13.6 in `Cargo.lock` alone, fixing RUSTSEC-2026-0433, Dioxus unchanged), the update the operator's own directive in this session (2026-11-04); its Schema / config: the residuals live as `residual` rows in the store since the bases door and no `.andromeda/residuals.md` stands, so a fact rewritten for this chunk names its owner so.
kept: the plan's other `.andromeda/residuals.md` sites stand — a rename of that path alone was rejected at Validate, the family routed whole to a `CARRY:` at P5.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the probe target outside the a11y leg
section: §9 CI Integration (ci-integration.md: a new item after the legs)
change:
  - `dioxus_runtime_probe` (10 tests, in both layout modes), which measures link 2's «reacts to a click» reading (§7), is none of the a11y leg's fourteen files and carries no `stand_` prefix, so it runs in neither the `a11y` nor the `stand-repeat` leg; the workspace `test` leg, `run all` and `run dioxus_runtime_probe` reach it; the a11y leg and its job are unchanged.
why: the report's Crates / modules (one blitz-tests target added, auto-discovered, no `stand_` prefix, reached by the workspace `test` leg, `run all` and `run dioxus_runtime_probe`) and its Harness / gate surface (no script, CI step or leg list moved).
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
@amendment marker=2026-11-04-dioxus-runtime-probe
title: the MountedData hazard re-measured, its fix routed
section: §5 Keyboard Navigation (keyboard-navigation.md: the MountedData record)
change:
  - The hazard's mechanism, re-measured at HEAD: `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, dioxus-core 0.7.10 first polls the task there through `handle_task_wakeup`, and its `set_focus`, borrowing through `doc_mut`, panics `RefCell already borrowed`; the same handler calling `set_focus` with no task moves focus and does not panic (was the panic and the task's first poll inside `render_immediate` alone).
  - Measured headlessly in both layout modes by `dioxus_runtime_probe`'s two `should_panic` pins, their precondition asserted first (the handle held, the key target focused by Tab); the windowed path read and not run.
  - Cells still calls its handles synchronously, each acting at the call; the hazard's fix is routed to the route entry «MountedData buffered render» (new).
why: the report's Counts (the MountedData record re-measured at HEAD, both pins, the synchronous contrast acting); the fix is not made here but routed to a chunk of its own on the founder's answer to the MountedData fork, by dialog in the overseer session, relayed by the operator (2026-11-04), the route entry inserted by the P5 card's first row, approved by the operator's own word in this session.
ref: .andromeda/runs/2026-11-04T21-47-15-wrap/
