@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels and the size read, the widening ratified) — added the `size`-on-a-`multiple`-select row (now decision/select-size-attribute-read: a whole number capped at 65534, no text out, no redaction shape moved, an engine change for every document, ratified), the open-list shot carrying option and group labels as pixels (decision/open-list-shot-carries-labels) and the optgroup label painted as a header (decision/optgroup-label-crossing), all on the founder's F1–F10 incl. F8. This chunk reverses F8's "single select's `size` stays unread" clause: the size-read row names `multiple` alone and must be widened to a sized single select, a boundary widening that needs ratification; a list box painting labels in-flow (not as a popup) also moves the "open list" wording of the pixel rows.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds and SelectListRow.label) — added contract/snapshot-diff/open-list-row-bounds (a row crosses by its own `bounds`, `differs` within its closed set, no new field/key/carrier) and decision/select-list-label-never-logged (`SelectListRow.label` page text in a public read type, leaves only as pixel and existing `name`). A list box's rows for a single select must cross the same way — no new field, key or carrier — and the label rule must still hold for the new rows; the row-bounds contract is worded for an "open list".

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — recorded `node::select_list`, `render::select_list`, the `BaseDocument` readers (`open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout`), `close_select_list`, `is_select`, `focus_select` as opening no span, the select-list added lines reading 0 on the added-lines census, escher-driver keeping 2 `info_span!` sites and `command` its eight fields. This chunk edits exactly these modules (`select_list.rs`, `select_list_allowed`, keyboard/pointer, blitz-paint) and must keep the census at 0 and the span counts unmoved.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list's checks are silent) — `select_popup`, `select_popup_painted`, `stand_act_popup` joined the silence census (no subscriber, env read or print, both layout modes) and `session_common` readers moved 29→32 (base already behind). The new checks beside `select_interaction`/`select_popup`/`stand_act_select`/`stand_act_popup` must be read silent the same way; precedent that reader counts drift and must be measured, not carried.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list read through the host, the role and a hosted shot) — added `host_log`'s / `mcp_log`'s open-list-at-`trace` tests (0 option needles, `hl-popup`, `#[cfg(unix)]`, mcp records nothing), `cli_commands`' hosted shot of an open list (`cc-popup`) and the shared `option_needles` helper. The guard pattern for decision/sink-host-read-at-trace and decision/session-logs-carry-no-page-content: a sized single select's list box (keys, row click) should be read through host and role with the same option needles; the popup tests assume a single select opens a popup, which a sized one no longer does.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the line citations the select list's added lines moved) — re-pointed citations into `document.rs`, `events/pointer.rs`, `resolve.rs`, blitz-paint `render.rs`, `host_log.rs`, `mcp_log.rs` after the select list's added lines. Trap: this chunk adds lines to the same files, so cited ranges in obs rows (and the `multiple_select_values` citation) may move again.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the Focussed node line's count, not settled) — the `focussed node` INFO line (once per focus move under the workspace build) counted 0 through the sink with no base reading, settling nothing. Keys moving a list-box choice and a row click touch focus; any re-read of this line needs a base reading to mean anything.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — added decision/select-chosen-set-never-logged, contract/snapshot/multiple-select-value (first chosen option in tree order), the optgroup `Role::Group` name, and contract/page-events/single-change (one `change`, from the engine, the script runtime synthesising none), on the founder's F6, F7, F10. A single select's list-box choice by keys or row click must keep the one-engine-`change` rule and leave its chosen value only through the same carriers; the multiple-select value row is the precedent for what a list box's snapshot `value` reads.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, no log site added, and the census's carve-out) — recorded the `multiple` select, `optgroup`, engine `change` and the readers `select_values`, `select_of`, `clear_select_typeahead`, `closed_select_of`, `is_multiple_select` as adding no log site; carve-out that `blitz-shell/src/event.rs`'s inline test sits outside the added-lines census. The single select's key model and `is_multiple_select`-style gates are what this chunk extends; mind the carve-out when reading the census.

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — the `focussed node` row states once per focus move under the workspace build, holding a node number and no id or name. The engine-side select interaction this chunk changes must not add a second engine line or field there.

@history_item n=11 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select checks in the silence census, and the PROVISIONAL discharge) — `select_interaction` and `stand_act_select` recorded silent; the select model adds no log site, span, field, log field or env read and reads no wall clock. These are the checks this chunk extends; the no-wall-clock clause binds any typeahead/key timing in the list box.

@history_item n=12 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** (the a11y leg's eight joining checks recorded silent) — `select_interaction` and `stand_act_select` are among the `a11y` leg's checks read silent. A list-box vs combo-box role change in `accessibility.rs` lands in checks this leg runs; their silence must stay 0.

@history_item n=13 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the multiple select's form-entry citation, re-pointed) — `multiple_select_values` cited at `dioxus_document.rs:32-46`. The chosen-set row this chunk leans on carries a line citation that moves with edits to that file.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading carries no page text) — added contract/snapshot-node/clickable and contract/snapshot-text/clickable (closed bool, token after `focused`). A list box's rows as real boxes may read `clickable` differently from popup rows; the closed-bool form must not move.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set, at rest and in the PII boundary) — added the actions-answer content/shape/owner/cause/logging contracts (ids, owner ids, verb words, closed causes, booleans; never a value; ratified widening). The scope lists actions readings of a sized single select; any new verb word or cause for a list box's rows would widen this ratified set.

@history_item n=16 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions joins the command span's verbs and the tool_call span's tools) — added contract/command-span/actions (verb alone, no node, id, owner, verb or cause word of its answer). An actions read over a list box must keep that line shape.

@history_item n=17 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — source of decision/actions-read-changes-nothing: `Session::actions` adds no `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call. A list box's select handling must not leak a select open/close or focus into the actions read.

@history_item n=18 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (4. Span / Trace Coverage: 1 row(s)) — `+ decision/layout-read-opens-no-span`, instrumentation absent by design so it is not read as drift. The layout placing words for a list box's rows are read through this span-less read.

@history_item n=19 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (4. Span / Trace Coverage: 1 row(s)) — `~ decision/layout-read-opens-no-span`: read in process it writes no `escher_` line at TRACE; reached by the `layout` verb it runs inside that call's `command` span. Guard `stand_layout` must stay green for a sized single select.

@history_item n=20 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (6. Log Coverage: 2 row(s)) — `+ contract/command-span/layout` (verb alone, `cause` when refused, nothing of its answer) and the verb table at fifteen words. No new verb or field may come from this chunk without the founder's ruling.

@history_item n=21 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Observability Harness Contract: 1 row(s)) — `~ decision/sink-host-read-at-trace`: needle set gains layout-answer keys. The host/role trace reads that would cover a sized single select hold id, name, typed-sentinel and layout-key needles.

@history_item n=22 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (8. PII Scrubbing & Compliance: 2 row(s)) — `+ contract/layout-answer/content` (ids, closed words, numbers; no `laid_out`/`placed`/`shifts` key at trace) and `+ contract/layout-answer/logging`. A list box's placing words must stay inside the closed word set.

@history_item n=23 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (8. PII Scrubbing & Compliance: 1 row(s)) — `~ contract/layout-answer/logging`: the answer also crosses to the model provider in a live cold-agent transcript. Any list-box layout reading travels there too.

@history_item n=24 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (the delegate confirms of two answer cards ratified) — ratified by the founder the layout-answer rows (and the error-in-caused rows). Precedent for how delegate confirms on a widening become ratified; this chunk's own widening (single select `size` read) needs the founder's word, not a delegate's.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set, ratified) — added contract/graph-answer/content (ids, component names, state/handler names, closed flags, booleans; never a value); its PROVISIONAL delegate confirm ratified by the 2026-11-15T05-21-42-wrap entry. A graph read over a page with a sized select is bound by it.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's command and tool_call lines) — added the graph `command` line (verb alone, no node, state, id or name) and `graph_needles` in host/mcp trace reads. Standing pattern for decision/graph-answer-never-logged.

@history_item n=27 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (3. Observability Harness Contract: 2 row(s)) — `~ decision/part-answer-never-logged` (snapshot part and graph part) and `~ contract/command-answer/graphed` (a part adds part number and count). Latest state of picked rows; the previous chunk's rows this chunk stands on.

@history_item n=28 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (6. Log Coverage: 2 row(s)) — `~ contract/graph-answer/shape` (part form, states in part 1 alone) and `~ contract/command-span/graph` (`part` outside 1–1000 refused `malformed`; no part number or count fielded). Current wording of two picked rows.

@history_item n=29 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (8. PII Scrubbing & Compliance: 1 row(s)) — `~ contract/graph-answer/content` gains part number and count. Current wording of the picked graph content row.

@history_item n=30 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (`screenshot` in the spans' closed sets, nine causes, no image in a log) — source of decision/shot-image-leaves-only-in-answer and contract/command-span/screenshot (no image byte, size or dimension), measured 0 image bytes on host and mcp stderr. A shot of an in-flow list box carries its labels as pixels under the same rule.

@history_item n=31 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — `lint` inside the `command` span, verb alone, findings never fielded (founder's F2 «Без поля»); source of contract/command-span/lint and contract/command-answer/linted. Lint readings over a list box's rows must add no field.

@history_item n=32 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (a part read's span and its silence) — source of contract/command-span/snapshot-part (verb alone, no part number, count, size or text). A long list box's rows can push a snapshot into parts; the part line shape is fixed.

@history_item n=33 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (the command schema is silent; the cause names are the owed span field's domain) — rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name; source of decision/command-span-no-call-content and decision/debug-types-never-logged. Any new select reading or argument path must not field Debug-derived types.

@history_item n=34 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — standing rule: no span or log field holding what a call supplied or a screen reads; a command's output is its answer, never a log line. Binds decision/driver-spans-carry-no-content for this chunk.

@history_item n=35 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — first measured 0 typed-sentinel, id and name needles in a sink-installing host's stderr at `trace`, both builds. The measurement pattern behind decision/session-logs-carry-no-page-content that a list box's key and click paths would be re-read under.

@history_item n=36 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (outside-target records dropped; both sink-installing binaries measured by level) — source of decision/sink-target-allowlist and decision/engine-target-safe-fields; rule for later chunks: a target is admitted only together with a scrub rule for it. Any engine log site touched in blitz-dom/blitz-paint stays under engine-target safe fields.

@history_item n=37 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** (escher-telemetry: stderr subscriber, identity, line format, chaining panic hook, allowlist scrub) — set the rule that engine and upstream crates compile tracing only with the `tracing` feature (decision/telemetry-via-tracing, decision/engine-tracing-feature-gated) and the engine safe-field list. Engine changes here (blitz-dom, blitz-paint) must not add an ungated log site.

@history_item n=38 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (8. PII Scrubbing & Compliance: 3 row(s)) — `+ decision/engine-sites-fixed-messages` (engine warn/error sites write fixed messages, values in named fields) with `− decision/scrub-at-sink-not-call-sites → decision/scrub-by-field-name`. Any warn/error site the engine select change adds must follow the fixed-message convention.

@history_item n=39 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (6. Log Coverage: 7 row(s)) — `capability-unused` gains "a list opening went unused"; graph, actions, layout became counted verbs. Trap: a sized single select opens no popup, so the cold-agent `list_opens` count (and its `capability-unused` reason) must not be read off a list box's key or click as a list opening, or the stand's select must stay unsized.

@history_item n=40 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (6. Log Coverage: 6 row(s)) — `+ contract/cold-agent-verdict/list-opens` with its decode rule. A list box with no popup changes what "a list opening" can mean; the decode rule is the contract to check against.
