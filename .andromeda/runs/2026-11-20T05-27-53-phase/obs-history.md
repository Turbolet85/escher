@history_item n=1 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the complexity factoring's private items recorded silent) added a log-coverage `observed absent` row: the private items that brought `lint.rs`'s findings walk, `mcp_tests.rs` and `stand_requirement_sweep.rs` under cognitive complexity 15 (`readings`, `placement`, `#[track_caller]` `assert_*` helpers, `inside_stand_repeat`) hold no print, log site, span, field, env read or subscriber, helper messages naming a row and an index only; silence census 0, escher-driver still 2 `info_span!` sites. Direct precedent: this chunk's extractions (`cli.rs` `read`, `mcp.rs` `run`, five test functions) owe the same reading and likely the same kind of row, private items only, no public symbol, verb, field or log site.

@history_item n=2 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — (the log checks' shared fragment and its silence census) a test-only move (escher-driver `cli_tests.rs`, `recording_tests.rs`, `recording_off_unix_tests.rs`, seven_guis `tests/common/mod.rs`) was recorded silent by a census over the new files, with assertion messages naming a kind and a count, never a needle. Precedent for splitting `json_tests.rs` into `json/{answer}` test children: each new file is read by the silence census and the move must keep the log checks' zeros (`host_log`, `mcp_log`).

@history_item n=3 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — recorded `command`, `refusal` and `schema` as silent private modules and that `Command`, `Call` and `ArgValue` derive `Debug` (printing ids and typed text) while nothing prints, logs or fields them; rule: field a cause by `Cause::name` only, never a `Command`/`Call`/`ArgValue` with `Debug` nor an argument by its schema name. Applies to splitting `command.rs` (826 lines) — no moved item may gain a log, `Debug` print or field (decision/debug-types-never-logged, decision/command-span-no-call-content).

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joined the `Debug`-deriving types nothing prints, logs or fields; rule kept for the span owner: never field a `Command`, `Call`, `ArgValue` or `Outcome` with `Debug`. Constraint on any extraction touching outcome writing in `json` / `command.rs`.

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the eight refusal cause names all returned by code, and the session's record of ids (no reader outside the crate, no `Debug`) printed, logged or fielded by nothing (decision/session-id-record-unprinted); also re-pointed a `command.rs` citation (`:161` → `:167`), showing that edits to `command.rs` move cited lines.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the command span as built) the `command` span: target `escher_driver`, INFO, entered before `validate`, eight fields recorded only where they apply, never any argument or `Command`/`Call`/`ArgValue`/`Outcome`/`Refusal`/`Fault`; `tracing` taken ungated in escher-driver (decision/telemetry-via-tracing scope). Moving `command.rs`/`execute`-adjacent code must keep exactly this span and its eight fields (contract/command-span/*, contract/command-span/field-count).

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the closed span's line, PROVISIONAL; a span's fields under the scrub) defined the closed-span line shape and that a closed span's pairs are judged by the same scrub rule; content-named span values never stored. Grounds decision/closed-span-lines-judged-alike and decision/content-named-span-values-not-stored, which a refactor must leave intact.

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified by the founder as built, with the ruling that a diff's three sizes (`added`/`removed`/`changed`) may reach a log while its content never does. The span line is a ratified record class: no refactor may alter its shape or fields.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the session host writes the command span; the command line writes answers, not logs) `escher_driver::command_line` (in `cli.rs`) installs no subscriber, reads no env var and writes each answer/refusal/error object as one JSON line on stdout, never a log line; a call refused on the command line opens no span; standing rule: no span or log field holding call or screen content. Constraint on extracting `cli.rs` `read` (CC 25): its refusals must still open no span and print only the fixed answers (contract/escher-session-stdout/command-answer, contract/escher-session-exit-codes/*).

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (typed text measured in a host's log) `host_log` reads typed sentinel, id and name needles at 0 at `trace` under both builds; the measurement the extraction of `cli.rs` / `mcp.rs` must keep green (decision/sink-host-read-at-trace).

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the ratified `tool_call` span: one per MCP `tools/call`, entered before params are read and held until the reply is written, fields `tool` and `ending` only, `command` its child by nesting; `serve_mcp` outside a tool call writes no record; `stand_mcp_tools` installs its own recorder. Key constraint for extracting `mcp.rs` `run` (CC 18): the span's open/close placement and its two fields must not move (contract/tool-call-span/*, decision/tool-call-span-two-closed-fields).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined both spans' closed sets: a server-answered `help` leaves a `tool_call` ending `accepted` with no `command` child; an argument to it refused `malformed`; the command line's `help` writes no record; session-error object gained the fixed `remedy`. Behaviour `mcp.rs` `run` and `cli.rs` `read` must preserve (contract/tool-call-span/help, contract/escher-session-commands/help, contract/escher-session-error-object/remedy).

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` in the spans' closed sets; an image leaves only in its answer (`to_json` and the MCP image block), never on a log line; `fonts-unavailable` and the `off-screen` empty crop. Grounds decision/shot-image-leaves-only-in-answer and contract/command-answer/shot, which moved answer-writing code (the `json` split) must keep.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set fixed at eight (`time-unavailable` retired), an accepted `advance` on every session recording the acting fields with no cause, its `tool_call` parenting one `command` child. Source of decision/advance-accepted-every-session and the refusal-cause contracts this chunk must leave unchanged.

@history_item n=15 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified) the one-virtual-clock marks (eight causes, `advance`) ratified by the founder (2026-10-18); these rows are founder-ratified, not open to drift by a refactor.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording opens no span; an MCP call after which the recording ended is answered `isError` with its `tool_call` ending `error`; `mcp --record` with a refused directory exits 3 before serving. Behaviour inside `mcp.rs`'s `run` the extraction must keep (contract/mcp-role-exit-status/3, contract/tool-call-ending/error, contract/escher-session-commands/open-recording-directory).

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (the gated part: the `lint` verb inside the command span, no field) put `lint` inside the `command` span, its findings and count never fielded, `Outcome::Linted` written by `to_json` alone. Grounds contract/command-span/lint and contract/command-answer/linted, which the `json`/`command.rs` splits must keep.

@history_item n=18 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone, a part's number and count never fielded; dioxus-native-dom's `Snapshot::text_parts` opens no span, emits no event and records no field. Directly bears on splitting `packages/dioxus-native-dom/src/snapshot.rs` (913): moved `text_parts` keeps no log site (contract/command-span/snapshot-part, decision/part-answer-never-logged).

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the snapshot's clickable reading carries no page text) `NodeState.clickable`, its text token after `focused` and a node's `"clickable"` key are a closed bool carrying no page text. Rows contract/snapshot-node/clickable and contract/snapshot-text/clickable live in `snapshot.rs`, which this chunk splits; the moved code must keep that output.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (an open row's bounds and SelectListRow.label) an open list's row crosses the snapshot and diff by its own `bounds`, `differs` within its closed set, adding no field or key; `SelectListRow.label` page text never logged. Rows contract/snapshot-diff/open-list-row-bounds and decision/select-list-label-never-logged rest on snapshot code this chunk moves.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) a multiple select's chosen set leaves only as form entries, the snapshot's `value` (first chosen option in tree order) and option nodes, never a log line. Grounds contract/snapshot/multiple-select-value and decision/select-chosen-set-never-logged, carried by the snapshot code being split.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list adds no log site or span) escher-driver's verbs' help and `covered`'s meaning are the program's own `'static` text, adding no log site or span field; `command` keeps its eight fields. Precedent that fixed help/meaning text living in `command.rs` adds no log site (contract/command-span/field-count).

@history_item n=23 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the Dioxus runtime probe's silence row) `dioxus_runtime_probe` recorded silent by the census `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`, its failure messages naming a layout mode, a task or page and a link or index only. Extracting its `visit` (CC 22) must keep the file silent and those messages content-free.

@history_item n=24 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the read values stay in the test) no read value (component, listener, signal value, node id) leaves the probe; grounds decision/runtime-probe-leaks-no-read-value, the rule any helper extracted from `visit` must respect in its failure messages.

@history_item n=25 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the probe's renamed spawned_acts pins, recorded silent) a `#[track_caller]` helper `spawned_focus_acts` added to `dioxus_runtime_probe` was recorded by a new observed-absent row after a census over the file. Precedent: a helper factored out of the probe gets the same census and may warrant a row.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb; dioxus-native-dom's graph modules add no log site; the graph verb's lines) `graph` runs inside the `command` span recording `verb` alone; `Outcome::Graphed` written by `to_json` alone, never logged; `stand_graph` and the graph modules recorded silent. Grounds contract/command-answer/graphed, contract/command-span/graph, decision/graph-answer-never-logged; `stand_graph`'s two over-ceiling tests are extracted here and must stay silent.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph answer's carried set) the graph answer carries ids, component names, state/handler names, closed flag words and booleans, never a value; a boundary widening (PROVISIONAL then) — see 2026-11-15T05-21-42-wrap. Grounds contract/graph-answer/content and /shape.

@history_item n=28 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified: the graph answer's carried set; the coverage role's sink reach, endings, command-span reach and coverage line) five PROVISIONAL entries ratified by the founder's own word. The graph and coverage contracts this chunk's moves must not alter are founder-ratified.

@history_item n=29 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions silent; actions joins the spans; the actions read opens no span; the actions answer's carried set) `stand_actions`, the `actions` module and `actionable_nodes` recorded silent; an `actions` line holds verb (and cause) alone; `Session::actions` changes nothing in the document; the actions answer shape and content ratified. Grounds contract/actions-answer/*, contract/command-span/actions, decision/actions-read-changes-nothing; `stand_actions`' over-ceiling test is extracted here.

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (cited line ranges re-pointed) the cited ranges in `json.rs`, `execute.rs`, `command.rs`, `stand_mcp_tools.rs`, `stand_act_spans.rs`, `session_common/mod.rs`, `host_log.rs`, `mcp_log.rs` were re-pointed after added lines. Trap: splitting `command.rs`/`json_tests.rs` and extracting in `cli.rs`/`mcp.rs` moves any cited lines still standing.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role's sink reach, endings, command-span reach, coverage line; citations re-pointed in `cli.rs` and `json.rs`) the `coverage` role on `command_line` (in `cli.rs`) with its shape, bounded read, `malformed` refusals holding nothing of the path, its endings and the hand-written coverage line, never logged. `cli.rs` `read` parses these shapes: its extraction must keep contract/escher-session-coverage/*, contract/coverage-role-stdout/coverage-line, contract/coverage-line/* and decision/coverage-line-never-logged.

@history_item n=32 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added decision/exploration-adds-no-span (exploration drives calls through `Session::run`, opening none of its own) and widened contract/escher-session-coverage/malformed to refuse a format other than 1. Both picked rows that extracted driver code must keep.

@history_item n=33 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen (contract/command-span/verb, contract/tool-call-span/tool), `layout` joined the verb-alone reads (contract/command-span/settled), added contract/command-span/layout and the layout-answer content/logging contracts, reworded decision/layout-read-opens-no-span (inside the layout call's span), and widened decision/sink-host-read-at-trace with layout-answer key needles. Current wording of many picked rows; layout answer code in `json`/`command.rs` moves under this chunk.

@history_item n=34 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — contract/layout-answer/logging gained the cold-agent transcript carrier; and delegate confirms (error in caused; `layout --id`) ratified by the founder. Current, ratified wording of picked rows.

@history_item n=35 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — decision/checks-silent and decision/headless-stand-no-sink excepts gained the layout stand check's recorder; + decision/layout-read-opens-no-span (no span by design, "so the absence is not read as drift"). History of the exceptions a refactor of the stand checks must not widen.

@history_item n=36 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — decision/headless-stand-no-sink, decision/driver-library-installs-no-subscriber and decision/driver-emits-no-event reworded around escher-telemetry's scoped capture around an acting step; contract/escher-sink-env/rust-log: the EnvFilter applies to the sink layer alone. Current wording of picked rows the driver refactor must keep true.

@history_item n=37 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — headless-stand-no-sink and checks-silent excepts gained the error-text founder check; answers-echo-nothing-typed gained its caused-item exception; driver-library-installs-no-subscriber gained the chaining panic hook; − scrub-at-sink-not-call-sites → + decision/scrub-by-field-name; escher-target-content-named-set gained `password`. Source of several picked rows' current text.

@history_item n=38 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — decision/scrub-by-field-name (capture by the answer's own hidden set) and decision/answers-echo-nothing-typed (an `error` field shown as written, cut at the text bound) reworded; ratified per 2026-11-17-a-component-in-isolation. Current text of two picked rows.

@history_item n=39 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — decision/checks-silent except gained "the order page logs"; contract/escher-session-exit-codes/2 usage line built from `VERBS` and the served task names. Latest wording of picked rows; the usage text is built in `cli.rs`, whose `read` this chunk extracts.

@history_item n=40 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — decision/checks-silent except widened so a re-run child prints ids or digests for its parent; part of the picked row's exception history.

@history_item n=41 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the headless-stand census was extended to the checks' shared module `tests/blitz-tests/tests/common/mod.rs` because checks read helpers from a module that is not itself a `stand_*` check. Precedent: helpers moved into shared modules (as the previous chunk did and test extractions here may) are inside decision/checks-silent ("their shared modules") and must read 0.

@history_item n=42 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the third shared module `fixtures/mod.rs` recorded silent; the stand line's "their shared modules" clause covers all three modules, so a new shared module needed no new wording. Precedent for where extracted test helpers may land.

@history_item n=43 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `stand_accessibility_ids` was first named in the silence census (no subscriber, env read, print or listener). Its `every_interactive_role_element_reads_a_name` test is extracted here and must stay silent.

@history_item n=44 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became a second sink installer; trap recorded: "a zero read from a host that installs no sink says nothing about one that does". Behind decision/sink-host-read-at-trace: the refactor's no-behaviour claim for `cli.rs`/`mcp.rs` is verified by `host_log`/`mcp_log`, not by in-process silence alone.

@history_item n=45 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the two prefix sets (engine scrubbed, `escher_` with content-named fields redacted, others dropped); rule for later chunks: a target is admitted only together with a scrub rule. Grounds decision/escher-target-content-named-set; this chunk must add no target or field.

@history_item n=46 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — tracing compiled only with the `tracing` feature is the engine and upstream crates' rule, escher-telemetry ungated; set the scrub sets. Grounds decision/telemetry-via-tracing and decision/engine-tracing-feature-gated (`snapshot.rs` lives in dioxus-native-dom, an engine-side crate: no ungated log site may be added in its split).

@history_item n=47 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (a master holds what a planner must respect, the code the rest), each verified against the code. Why this chunk is bound by rows rather than line citations; amendments since are row-handle edits (`+`/`~`/`−`), the form any amendment from this chunk would take.
