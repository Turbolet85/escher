# Apply — security-plan · part 3 of 3

## Verdicts
- fanout-security-plan--cascade-g4.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:9#1, .andromeda/security-plan-detail/error-handling.md:10#1
- fanout-security-plan--cascade-g4.md 2 · applied · .andromeda/security-plan-detail/error-handling.md:9#2, .andromeda/security-plan-detail/error-handling.md:10#2
- fanout-security-plan--cascade-g4.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:38+#1
- fanout-security-plan--cascade-new-error-handling.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:38+#2
- fanout-security-plan--cascade-new-error-handling.md 2 · applied · .andromeda/security-plan-detail/error-handling.md:32+#1
- fanout-security-plan--cascade-g4.md 4 · applied · .andromeda/security-plan-detail/logging-monitoring.md:17+#1
- fanout-security-plan--cascade-g4.md 5 · applied · .andromeda/security-plan-detail/logging-monitoring.md:17+#2

## Edits
- line: .andromeda/security-plan-detail/error-handling.md:9
  old: «so it can read `covered` though nothing covers it (tests/blitz-tests/tests/stand_act_ids.rs:55) — and `off-screen`'s»
  new: «so it can read `covered` though nothing covers it (tests/blitz-tests/tests/stand_act_ids.rs:55), and an option of a closed select reads `covered`, its meaning stating `an option of a closed select lies under it: click the select first, then the option` (measured by `stand_act_popup::a_closed_option_stays_refused_covered_and_changes_nothing`; tests/blitz-tests/tests/stand_act_popup.rs:157-166) — and `off-screen`'s»
- line: .andromeda/security-plan-detail/error-handling.md:10
  old: «its no-area sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md,»
  new: «its no-area sentence at escher-0.2.0/chunks/2026-10-18-engine-debts/report.md, its closed-option sentence at escher-0.3.0/chunks/2026-10-31-select-popup/report.md,»
- line: .andromeda/security-plan-detail/error-handling.md:9
  old: «its match alone, a text opening on a space typing nothing (packages/escher-driver/src/schema.rs:239-243;»
  new: «its match alone, a text opening on a space typing nothing, `type` on any select focusing with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there, `click`'s help says a click on a select opens its list and `press`'s states the keys on a closed select — each still a fixed literal of the verb table holding nothing a call supplies, the verb count and `describe`'s count row unchanged (packages/escher-driver/src/schema.rs:222-226; packages/escher-driver/src/schema.rs:264-269; packages/escher-driver/src/schema.rs:239-243;»
- line: .andromeda/security-plan-detail/error-handling.md:10
  old: «the two help texts at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md)»
  new: «the two help texts at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md, the `click` and `press` help and `type` on a select at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/security-plan-detail/error-handling.md:38
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Key press after a hidden ancestor: a key press panics in `position_in_containing_block` (packages/blitz-dom/src/node/node.rs:1775), the stack through `handle_dom_event`'s `absolute_position`, when the focused control's anonymous block was dropped because an ancestor gained `hidden`; any control, reproduced on a `select` and a plain `button` with no list opened, and a remote page can reach it; that it predates the select list rests on `node.rs` and `events/mod.rs` unchanged against the chunk's base and on the stack, no controlled run at that base taken; `select_popup` skips the key press in that one row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; not fixed, ratified by the founder (2026-10-31), owner: a CARRY on «Upstream sync ahead of the UI graph», the fix the founder's fork at that entry's P4»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/error-handling.md:38
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Select list closing: a select removed, disabled, hidden or made inert with its list open closes the list with no panic (`a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`; tests/blitz-tests/tests/select_popup.rs:478-520), `clear_interaction_state_for_removed_node` clearing the field for a removed select (packages/blitz-dom/src/document.rs:1408-1410), and an option dropped with the list open is no row (`an_option_dropped_with_the_list_open_is_no_row`; tests/blitz-tests/tests/select_popup.rs:522-552); this holds of the close alone, a key pressed after an ancestor gained `hidden` being the panic of this section's `Key press after a hidden ancestor` row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/error-handling.md:32
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Select list readers: the list's public `BaseDocument` readers `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`, and its public writer `close_select_list`, go through `get_node`, a row with no styles reading `None`, and call no `layout_style()`, so they add no panic path (packages/blitz-dom/src/document.rs:881-889; packages/blitz-dom/src/document.rs:990-993; packages/blitz-dom/src/document.rs:995-1012; packages/blitz-dom/src/document.rs:1022-1052; packages/blitz-dom/src/document.rs:1054-1064) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/logging-monitoring.md:17
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · logging -->»
  add: «»
  add: «- A select list through the host and the role: `host_log` and `mcp_log` each hold a test that opens a select's list and chooses in it, through the host or the `mcp` role, with 0 option needles at `RUST_LOG=trace` (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; the needles at examples/seven_guis/tests/common/mod.rs:425-453); `host_log`'s runs in the state directory `hl-popup` (examples/seven_guis/tests/host_log.rs:296) under `#[cfg(unix)]`, as every host-driving test there does; `mcp_log`'s records nothing and no `ml-popup` directory exists — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/logging-monitoring.md:17
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · logging -->»
  add: «»
  add: «- The select list's silence: the list holds no log site, span or field: blitz-dom's `node::select_list` and blitz-paint's `render::select_list` hold no tracing token by gate, `packages/escher-driver/src` holds two `info_span!`, `stand_act_spans` reads eight span fields, and an option's or a group's label leaves only as a painted pixel and the snapshot's existing `name`, `SelectListRow.label` never logged — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  add: «»
  add: «<!-- /detail -->»

## Patterns
(none)
