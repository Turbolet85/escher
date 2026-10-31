# Apply — obs-plan

## Verdicts
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:92#1
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:77#1
- fanout-obs-plan.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:70#1
- fanout-obs-plan--cascade-g2.md 1 · applied · .andromeda/obs-plan-detail/log-coverage.md:42+#1
- fanout-obs-plan--cascade-g2.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:72+#1
- fanout-obs-plan--cascade-g2.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:72+#2
- fanout-obs-plan--cascade-g3.md 1 · applied · .andromeda/obs-plan-detail/log-coverage.md:97+#1
- fanout-obs-plan--cascade-g3.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:126+#1
- fanout-obs-plan--cascade-g4.md 1 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13+#1
- fanout-obs-plan--cascade-g4.md 2 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13+#2
- fanout-obs-plan--cascade-g4.md 3 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13+#3

## Edits
- line: .andromeda/obs-plan.md:92
  old: «its 24 readers (as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md)»
  new: «its 28 readers (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md)»
- line: .andromeda/obs-plan.md:77
  old: «which drives the four lean stand tasks and its own fixtures in process»
  new: «which drives the four lean stand tasks, the load page and Cells, and its own fixtures in process»
- line: .andromeda/obs-plan-detail/log-coverage.md:70
  old: «info: focussed node (packages/blitz-dom/src/document.rs:1901-1902)»
  new: «info: focussed node, once per focus move under the workspace build, where engine `tracing` is compiled in, holding a node number and no id or name; the focus fixup adds no log site, and neither `clear_focus` nor the flush prints (packages/blitz-dom/src/document.rs:1901-1902; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/sink.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:42
  anchor: « a recording directory open to its group or others ends `start` with status 3 and nothing spawned and `mcp --record` with status 3, its fixed message on stderr and stdout empty, the two recording session errors ending status 3 like the rest; the two-argument form `<task> <state-dir>` is gone (it reads as an unknown verb, refused, status 1), every diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed (examples/seven_guis/src/session_host.rs:31; packages/escher-driver/src/cli.rs:278-293; packages/escher-driver/src/cli.rs:341-368; packages/escher-driver/src/cli.rs:370-383)»
  add: «  - Cells' `cells-grid` Tab stop and keys and its `.cell.current` outline are instrumentation n/a (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:72
  anchor: «  - `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:19-155)»
  add: «  - no site added: the transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:72
  anchor: «  - `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:19-155)»
  add: «  - no site added: the readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site or print (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:97
  anchor: «  - `warn!` is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)»
  add: «  - `packages/blitz-shell/src/event.rs` holds the crate's unit test, an inline `#[cfg(test)]` module, held out of the added-lines census under `packages/`, so that census's 0 does not speak for it (packages/blitz-shell/src/event.rs:128-172; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
- after: .andromeda/obs-plan-detail/log-coverage.md:126
  anchor: «- observed absent — a print or log site in blitz-vibey-script `tests/dom.rs`'s `style_property_accessors`, `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls` · searched: the fixed-sha census over the added `packages/*.rs` lines of the upstream take read at `ebacd68bd`, its one hit in the file a test-only env read — `temp_dir_with_files` reads `TMPDIR` through `std::env::temp_dir()`, test code only and not secret, the second test-only `TMPDIR` site beside escher-driver's client unit test (packages/blitz-vibey-script/tests/dom.rs:903; as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
  add: «- observed absent — a print, an env read, a subscriber or a listener in the five checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` over those files, exit 1, reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md)»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
  anchor: « The sink allowlist re-checked after the upstream take read at `ebacd68bd` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged and dioxus-native-dom taking one test alone (`input_focused_on_mount_enables_ime`); `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the fixed-sha census over the taken commits' `packages/*.rs` lines reads its two named hits, blitz-dom holds 0 wall-clock reads outside `clock.rs`, and the take adds no log site, span, field, log field or env read in source — the uncaught-JS-error `tracing::error!` stands whole in `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = "tracing")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields and the message `[redacted]`; `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
  add: «- A `multiple` select's chosen set leaves only as form entries — on `input` and `change` one `(name, FormValue::Text(value))` per chosen option (`multiple_select_values`) — as the snapshot's `value`, which reads `select_value`, the first chosen option in tree order, and as the options' tree nodes, never as a log line or field: the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/dioxus-native-dom/src/dioxus_document.rs:31-45; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
  anchor: « The sink allowlist re-checked after the upstream take read at `ebacd68bd` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged and dioxus-native-dom taking one test alone (`input_focused_on_mount_enables_ime`); `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the fixed-sha census over the taken commits' `packages/*.rs` lines reads its two named hits, blitz-dom holds 0 wall-clock reads outside `clock.rs`, and the take adds no log site, span, field, log field or env read in source — the uncaught-JS-error `tracing::error!` stands whole in `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = "tracing")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields and the message `[redacted]`; `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
  add: «- An `optgroup`'s non-empty `label` is its `Role::Group` name in the accessibility tree, inside the ratified name crossing, and it is not painted; the added lines hold no log site, print or env read, the added-lines census reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13
  anchor: « The sink allowlist re-checked after the upstream take read at `ebacd68bd` holds: no scrub set, redaction shape or violation schema moves, `packages/escher-telemetry/` and `packages/escher-driver/` unchanged and dioxus-native-dom taking one test alone (`input_focused_on_mount_enables_ime`); `host_log` and `mcp_log` (the host's and the `mcp` role's stderr at `trace`, read for id, name and typed-sentinel needles) pass in the per-package and the workspace build; the fixed-sha census over the taken commits' `packages/*.rs` lines reads its two named hits, blitz-dom holds 0 wall-clock reads outside `clock.rs`, and the take adds no log site, span, field, log field or env read in source — the uncaught-JS-error `tracing::error!` stands whole in `report_js_error` (`runtime/events.rs`), still `#[cfg(feature = "tracing")]` under the engine target `blitz_vibey_script`, the sink printing its safe fields and the message `[redacted]`; `pseudo_element_flow_classification` reads 0 env · 0 print · 0 subscriber (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/sink.md)»
  add: «- The engine's `change` on a checkbox or radio, sent after `input` with the same value, a checkbox always and a radio only where it was not checked before the click, carries the value class `input` already carries; a page's script hears one `change`, from the engine, the script runtime synthesising none; the added lines hold no log site, print or env read, the added-lines census reading 0 (packages/blitz-dom/src/events/pointer.rs:793-799; packages/blitz-dom/src/events/pointer.rs:811; packages/blitz-dom/src/events/pointer.rs:827-836; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md).»

## Patterns
[[pattern]]
id = "obs-01"
fixed = "24 readers"

[[pattern]]
id = "obs-02"
fixed = "drives the four lean stand tasks and its own fixtures"
