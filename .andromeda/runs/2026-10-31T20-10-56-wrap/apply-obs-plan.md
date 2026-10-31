# Apply — obs-plan

## Verdicts
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:95+#1
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:76#1, .andromeda/obs-plan.md:76#2, .andromeda/obs-plan.md:76#3, .andromeda/obs-plan.md:76#4
- fanout-obs-plan--cascade.md 1 · applied · .andromeda/obs-plan-detail/log-coverage.md:43+#1
- fanout-obs-plan--cascade.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:53+#1
- fanout-obs-plan--cascade.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:71#1
- fanout-obs-plan--cascade.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:75#1
- fanout-obs-plan--cascade.md 5 · applied · .andromeda/obs-plan-detail/log-coverage.md:89+#1
- fanout-obs-plan--cascade.md 6 · applied · .andromeda/obs-plan-detail/log-coverage.md:127+#1
- fanout-obs-plan--cascade.md 7 · applied · .andromeda/obs-plan-detail/log-coverage.md:127+#2
- fanout-obs-plan--cascade.md 8 · applied · .andromeda/obs-plan-detail/log-coverage.md:127+#3
- fanout-obs-plan--cascade.md 9 · applied · .andromeda/obs-plan-detail/log-coverage.md:131+#1
- fanout-obs-plan--cascade.md 10 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13+#1
- fanout-obs-plan--cascade.md 11 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14+#1
- fanout-obs-plan--cascade.md 12 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15#1
- fanout-obs-plan--cascade.md 13 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15+#1
- fanout-obs-plan--cascade.md 14 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15+#2
- fanout-obs-plan--cascade.md 15 · applied · .andromeda/obs-plan-detail/ci-integration.md:15#1
- cascade-compare-obs-plan.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:30+#1

## Edits
- after: .andromeda/obs-plan.md:95
  anchor: «- The select checks are silent: `select_interaction` (a `fixtures` reader) and `stand_act_select` (a `fixtures` and `session_common` reader), both layout modes, install no subscriber, read no env var and print nothing, their readings going into assertions only — the census probe reading 0 over the two files; the select model they pin (blitz-dom's crate-private `node::select`, the keyboard arm and its typeahead on the document's clock, the click arm, the mutator's select settle, `DomEventKind::Change`) adds no log site, span, field, log field or env read and reads no wall clock — the rules module holding 0 print, log, env or panic tokens, escher-driver keeping its 2 `info_span!` sites (as measured at escher-0.2.0/chunks/2026-10-20-select-by-id/report.md).»
  add: «- The select list's checks are silent: `select_popup`, `select_popup_painted` and `stand_act_popup`, both layout modes, install no subscriber, read no env var and print nothing, their readings going into assertions only — the silence census reading 0 over the three files; the list they pin (blitz-dom's `node::select_list` and the `BaseDocument` readers, blitz-paint's `render::select_list` and `stroke_text_as`) adds no log site, span, field, log field or env var — the added-lines census reading 0 and the census over the two modules finding none at the pre-CI tree, `text.rs`'s fix adding no such token, escher-driver keeping its 2 `info_span!` sites and `stand_act_spans` its eight fields; `host_log` and `mcp_log` each read a list opened and chosen at `trace`, 0 needles, `host_log`'s test `#[cfg(unix)]` and `mcp_log`'s recording nothing; `SelectListRow.label` is page text in a read type, never logged (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- line: .andromeda/obs-plan.md:76
  old: «and the engine's select check `select_interaction`,»
  new: «and the engine's select check `select_interaction`, for the sixteenth driver-action check, `stand_act_popup`, and the engine's `select_popup` and `select_popup_painted`,»
- line: .andromeda/obs-plan.md:76
  old: «for the module twenty-nine checks read»
  new: «for the module thirty-two checks read»
- line: .andromeda/obs-plan.md:76
  old: «the fifteen `stand_act_*`»
  new: «the sixteen `stand_act_*`»
- line: .andromeda/obs-plan.md:76
  old: «`stand_snapshot_parts`, `stand_load_wait`,»
  new: «`stand_snapshot_parts`, `stand_load_wait`, `stand_cells_served`, `stand_load_page`,»
- after: .andromeda/obs-plan-detail/log-coverage.md:43
  anchor: «  - Cells' `cells-grid` Tab stop and keys and its `.cell.current` outline are instrumentation n/a (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  add: «  - `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under the state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one, an answer and no log line (examples/seven_guis/tests/cli_commands.rs:419-464; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:53
  anchor: «  - since 2026-10-10-mcp-surface, one closed-span line per MCP tool call: INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` (the table's word, one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint`) and `ending` (`accepted` · `refused` · `error`), then `message="close"`, `time.busy`, `time.idle` — e.g. `span="tool_call" tool="start" ending="accepted" message="close"`; a `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it; no line at the default `warn`; the emitters are the `escher-session mcp` role and `serve_mcp` in process; over one CRUD flow at `info` the role's stderr reads 9 lines built alone and 10 built with the workspace — the tenth `INFO blitz_dom::document … message=[redacted]` — with 5 `tool_call` and 3 `command` lines in each (packages/escher-driver/src/mcp.rs:118-121; as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/report.md, read on the hand-run role, 2026-10-11); re-measured at 2026-10-14-stand-requirement-sweep, 9 built alone and 11 built with the workspace — the `blitz_dom::document` INFO line now printing twice — the 5 `tool_call` and 3 `command` lines unchanged, needles 0 (as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/evidence/log-counts.md)»
  add: «  - no site added: the driver's fixed texts, `click`'s, `type`'s and `press`'s help and `covered`'s meaning, are the program's own `'static` text, holding no call text and adding no log site or span field, the `command` span keeping its eight fields (`stand_act_spans`) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/obs-plan-detail/log-coverage.md:71
  old: «holding a node number and no id or name; the focus fixup»
  new: «holding a node number and no id or name; a count through the sink on that build reads 0 lines with no base reading, so neither moved nor unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md); the focus fixup»
- line: .andromeda/obs-plan-detail/log-coverage.md:75
  old: «and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site or print (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «and `dioxus_native_dom::closed_select_of`, `is_multiple_select` and `is_select`, the last read by `execute.rs`'s `locate` alone, are instrumentation n/a and add no log site or print (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, `is_select` at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:89
  anchor: «  - `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:1012-1026)»
  add: «  - no site added: the select list's two modules, blitz-dom `node::select_list` and blitz-paint `render::select_list`, hold no tracing or log token, the census over the two finding none, and the select list's added `packages/*.rs` lines hold none, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:127
  anchor: «- observed absent — a log site, span, field, env read or listener in the 0.2.0 requirement record's test additions (`stand_requirement_sweep`'s `WITNESSES_020` and its leg check reading `.github/scripts/ci-leg.sh` through `include_str!`, `stand_lint`'s `reading_page` and its seventh test, `text_extent`'s `extent` helper and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS`, seven_guis `host_log`'s third test) · searched: the silence census over those files, reading 0; `packages/` unchanged, the `RUST_LOG=trace` on `host_log`'s client commands the registered `RUST_LOG` read (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, its gate entry 8)»
  add: «- observed absent — an option's label in a sink-installing host's log · searched: `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` and the state directory `hl-popup`, reading the host at `trace`: 0 option needles, its `command` span lines counted (examples/seven_guis/tests/host_log.rs:282-393; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:127
  anchor: «- observed absent — a log site, span, field, env read or listener in the 0.2.0 requirement record's test additions (`stand_requirement_sweep`'s `WITNESSES_020` and its leg check reading `.github/scripts/ci-leg.sh` through `include_str!`, `stand_lint`'s `reading_page` and its seventh test, `text_extent`'s `extent` helper and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS`, seven_guis `host_log`'s third test) · searched: the silence census over those files, reading 0; `packages/` unchanged, the `RUST_LOG=trace` on `host_log`'s client commands the registered `RUST_LOG` read (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, its gate entry 8)»
  add: «- observed absent — an option's label in the `mcp` role's log · searched: `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, reading the role at `trace`: 0 option needles; it records nothing, so no `ml-popup` directory exists (examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:127
  anchor: «- observed absent — a log site, span, field, env read or listener in the 0.2.0 requirement record's test additions (`stand_requirement_sweep`'s `WITNESSES_020` and its leg check reading `.github/scripts/ci-leg.sh` through `include_str!`, `stand_lint`'s `reading_page` and its seventh test, `text_extent`'s `extent` helper and three limit tests, `stand_act_spans`' paged part read, the `fixtures` module's `many_items` and `ITEMS`, seven_guis `host_log`'s third test) · searched: the silence census over those files, reading 0; `packages/` unchanged, the `RUST_LOG=trace` on `host_log`'s client commands the registered `RUST_LOG` read (as measured at escher-0.2.0/chunks/2026-10-24-stand-requirement-sweep/report.md, its gate entry 8)»
  add: «- The seven_guis shared module holds `option_needles`, a select's option labels as needles for the popup log tests (examples/seven_guis/tests/common/mod.rs:425-453; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:131
  anchor: «- observed absent — a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those files, exit 1, reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md)»
  add: «- observed absent — a print, an env read, a subscriber or a listener in the popup checks `select_popup`, `select_popup_painted` and `stand_act_popup` (a `session_common` reader) · searched: the silence census over those files, exit 1, reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entry 2)»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
  anchor: « The sink allowlist re-checked after the upstream take read at `ebacd68bd` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged and dioxus-native-dom taking one test alone (`input_focused_on_mount_enables_ime`); `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the fixed-sha census over the taken commits' `packages/*.rs` lines reads its two named hits, blitz-dom holds 0 wall-clock reads outside `clock.rs`, and the take adds no log site, span, field, log field or env read in source — the uncaught-JS-error `tracing::error!` stands whole in `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = "tracing")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields and the message `[redacted]`; `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
  add: « A shot of an open list carries a page's option labels and group `label`s as pixels, a boundary widening ratified by the founder (2026-10-31), the shot's own fields unmoved (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14
  anchor: «- A `multiple` select's chosen set leaves only as form entries — on `input` and `change` one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`) — as the snapshot's `value`, which reads `select_value`, the first chosen option in tree order, and as the options' tree nodes, never as a log line or field: the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/dioxus-native-dom/src/dioxus_document.rs:31-45; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
  add: «- `size` on a `multiple` select is an attribute read, parsed into a whole number capped at 65534, carrying no text out and moving no redaction shape, an engine change for every document and a boundary widening, ratified by the founder (2026-10-31) (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15
  old: «and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «and it is also painted, as a group's header in an open list, leaving as pixels in a shot and on no log line or field, a boundary widening ratified by the founder (2026-10-31); the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, the header at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15
  anchor: «- An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
  add: «- An open list's row crosses the snapshot and the diff by its own `bounds`, `differs` naming `bounds` within its closed set, adding no field, key name, node key or carrier (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15
  anchor: «- An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
  add: «- `SelectListRow.label` holds page text in a public read type and is never logged; a label leaves only as a painted pixel and the snapshot's existing `name` (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»
- line: .andromeda/obs-plan-detail/ci-integration.md:15
  old: «`hl-client-rec`, `hl-cells`, `cc-*`»
  new: «`hl-client-rec`, `hl-cells`, `hl-popup`, `cc-*`»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  anchor: «- `packages/escher-driver` — a hosted part read: `escher-session serve cells` at `RUST_LOG=trace` answers `snapshot --part 2` through `Session::run` and nothing else, so its spans are the `command` spans above, a part read recording `verb` alone; seven_guis `host_log`'s fourth test counts the host's closed `command` span lines and reads 0 id and 0 name needles under both builds (§6 Log coverage, §8 PII); no span, field or log line added (examples/seven_guis/tests/host_log.rs:137-280; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  add: «- `packages/escher-driver` and the engine — the select list adds no span, field, event or log line: blitz-dom's `node::select_list`, blitz-paint's `render::select_list`, the `BaseDocument` readers `open_select` · `select_list_bounds` · `select_list_view` · `select_row_layout` and writer `close_select_list`, dioxus-native-dom's `is_select` and the driver's `focus_select` open no span; escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`), the `click` · `type` · `press` help and `covered`'s meaning being fixed `'static` text that reaches no field; seven_guis `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace` (under `#[cfg(unix)]`) and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` drive an open list at `trace`, reading 0 needles (§6 Log coverage, §8 PII), `host_log` counting the host's closed `command` span lines (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»

## Patterns
[[pattern]]
id = "obs-01"
fixed = 'for the module twenty-nine checks read'

[[pattern]]
id = "obs-02"
fixed = 'the fifteen `stand_act_*`'

[[pattern]]
id = "obs-03"
fixed = 'and it is not painted'
