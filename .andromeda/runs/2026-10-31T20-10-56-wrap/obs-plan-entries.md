## 2026-10-31-select-popup — the line citations the select list's added lines moved
**Section:** §1 Obs Scope Summary, §2 Telemetry Strategy, §3 Observability Harness Contract, §7 Error Capture & Reporting, §8 PII Scrubbing & Compliance (obs-plan.md); §9 CI Integration (ci-integration.md); §6 Log Coverage (log-coverage.md); §5 Metric Coverage (metric-coverage.md); §8 (pii-scrubbing-compliance.md); §4 Span / Trace Coverage (span-trace-coverage.md); §2 (telemetry-strategy.md)
**Change:** Line citations re-pointed at the moved lines of `ci-leg.sh`; seven_guis `host_log.rs`, `mcp_log.rs` and `tests/common/mod.rs`; blitz-dom `document.rs`, `events/pointer.rs` and `resolve.rs`; blitz-paint `render.rs`; escher-driver `execute.rs` and `lint.rs`. Beyond the text this chunk's other entries here name, only the citations moved; the claims stand as they read.
**Why:** The chunk added lines to each of those files, 340 to `document.rs` and 131 to `pointer.rs` among them, so the lines the rows cite moved.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list read through the host, the role and a hosted shot
**Section:** §9 CI Integration (ci-integration.md: the session state directory row); §6 Log Coverage (log-coverage.md: examples, the observed-absent rows)
**Change:**
- The session state directory row's seven_guis names gain `hl-popup`, after `hl-cells`.
- New examples row: `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one, an answer and no log line.
- New observed-absent row, an option's label in a sink-installing host's log: `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reads the host at `trace`: 0 option needles, its `command` span lines counted.
- New observed-absent row, an option's label in the `mcp` role's log: `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` reads the role at `trace`: 0 option needles; it records nothing, so no `ml-popup` directory exists.
- New row: the seven_guis shared module holds `option_needles`, a select's option labels as needles for the popup log tests.
**Why:** An open list shows page text, and the chunk measured where that text goes: `host_log` and `mcp_log` read 0 needles at `trace`, and the hosted shot equals the in-process one. `host_log`'s test carries `#[cfg(unix)]` as every host-driving test there does; recording in `mcp_log`'s would have needed a `cfg!(unix)` site, so it records nothing.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list adds no log site or span
**Section:** §6 Log Coverage (log-coverage.md: escher-driver, blitz-dom (document / resolve / mutator / net), blitz-dom (node)); §4 Span / Trace Coverage (span-trace-coverage.md)
**Change:**
- New escher-driver row: `click`'s, `type`'s and `press`'s help and `covered`'s meaning are the program's own `'static` text, holding no call text and adding no log site or span field, the `command` span keeping its eight fields (`stand_act_spans`).
- The silent readers' row: was `closed_select_of` and `is_multiple_select` from dioxus-native-dom; now also `is_select`, read by `execute.rs`'s `locate` alone, instrumentation n/a and adding no log site or print.
- New blitz-dom (node) row: blitz-dom `node::select_list` and blitz-paint `render::select_list` hold no tracing or log token, the census over the two finding none, and the select list's added `packages/*.rs` lines hold none, the added-lines census reading 0.
- New §4 row: `node::select_list`, `render::select_list`, the `BaseDocument` readers `open_select` · `select_list_bounds` · `select_list_view` · `select_row_layout` and writer `close_select_list`, `is_select` and the driver's `focus_select` open no span; escher-driver keeps its 2 `info_span!` sites and `command` its eight fields; `host_log` and `mcp_log` drive an open list at `trace`, reading 0 needles, `host_log` counting the host's closed `command` span lines.
**Why:** The chunk added no log field or span and required none: the censuses read green and the driver's span counts did not move. The answer card, main's read, puts `covered`'s clause and the restated help outside the widening as the program's own fixed text, under the rule of 2026-10-12.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the Focussed node line's count, not settled
**Section:** §6 Log Coverage (log-coverage.md: blitz-dom (document / resolve / mutator / net), the focussed-node row)
**Change:** The row goes on, in its own line and with no new row: was once per focus move under the workspace build, with no count through the sink; now also a count through the sink on that build reads 0 lines with no base reading, so neither moved nor unmoved.
**Why:** The chunk counted the line through the sink on the workspace build and has no base reading to set it against, so the count settles nothing either way.
**Kept:** The row's once-per-focus-move claim stands; a reading with no base does not retire it.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list's checks are silent
**Section:** §6 Log Coverage (log-coverage.md: the observed-absent rows); §3 Observability Harness Contract (obs-plan.md: the no-sink row, the silence rows)
**Change:**
- New observed-absent row: a print, an env read, a subscriber or a listener in `select_popup`, `select_popup_painted` and `stand_act_popup` (a `session_common` reader); the silence census over those files exits 1, reading 0.
- The no-sink row gains the sixteenth driver-action check, `stand_act_popup`, and the engine's `select_popup` and `select_popup_painted`; its clause on the fourteen driver-action checks is unchanged, the fifteenth and sixteenth each named after it.
- The shared module's readers: was twenty-nine checks; now thirty-two, its list gaining `stand_cells_served` and `stand_load_page`; was the fifteen `stand_act_*`; now the sixteen.
- New §3 row beside the select checks' row: the three checks, in both layout modes, install no subscriber, read no env var and print nothing, their readings going into assertions only. The list they pin (`node::select_list`, the `BaseDocument` readers, `render::select_list`, `stroke_text_as`) adds no log site, span, field, log field or env var: the added-lines census reads 0 and the census over the two modules finds none at the pre-CI tree, `text.rs`'s fix adding no such token; escher-driver keeps its 2 `info_span!` sites and `stand_act_spans` its eight fields. `host_log` and `mcp_log` each read a list opened and chosen at `trace`, 0 needles, `host_log`'s test `#[cfg(unix)]` and `mcp_log`'s recording nothing. `SelectListRow.label` is page text in a read type, never logged.
**Why:** The chunk adds three checks and reads silence over them. `stand_act_*` files went from 15 to 16 and checks declaring `mod session_common;` from 31 to 32, so the row's twenty-nine was already behind the base's 31.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a page's labels as pixels and the size read, the widening ratified
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the shot's continuation line, the `size` row, the `optgroup` row)
**Change:**
- New continuation line: a shot of an open list carries a page's option labels and group `label`s as pixels, a boundary widening ratified, the shot's own fields unmoved.
- New row: `size` on a `multiple` select is an attribute read, parsed into a whole number capped at 65534, carrying no text out and moving no redaction shape, an engine change for every document and a boundary widening, ratified.
- The `optgroup` row: was its non-empty `label` not painted; now also painted, as a group's header in an open list, leaving as pixels in a shot and on no log line or field, a boundary widening ratified. Its `Role::Group` name inside the ratified name crossing and its added-lines census reading 0 stand.
**Why:** The chunk paints an open list on every document, remote pages included, and reads `size` there, so a validated surface admits new input and new pixels. Ratified by the founder, by dialog, 2026-10-31, each word relayed verbatim by the operator: his forks F1 to F10 in the overseer session, F7 painting a group's label as a header and F8 reading `size` on a `multiple` select, then his confirmation of this wrap's answer card at P2 Validate (Q1), which lists these three rows inside the widening. The widening adds no carrier, verb, key name, result field, cause, log field or span.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open row's bounds and SelectListRow.label
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: two new rows)
**Change:**
- New row: an open list's row crosses the snapshot and the diff by its own `bounds`, `differs` naming `bounds` within its closed set, adding no field, key name, node key or carrier.
- New row: `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name`.
**Why:** The list crosses the ratified carriers as a row's own `bounds` and as pixels; the chunk adds no log site, and a label's only exits are those two. The answer card, main's read, puts a row's `bounds` outside the widening: a closed set inside the ratified crossing, under the field rule of 2026-10-10.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
