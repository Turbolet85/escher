@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher-telemetry: stderr subscriber, allowlist scrub — set the split the telemetry rows rest on: engine and upstream crates compile tracing only with the `tracing` feature, escher-telemetry ungated. Why it matters: the three blitz-dom bodies this chunk splits sit under the feature-gated rule (decision/engine-tracing-feature-gated, decision/telemetry-via-tracing); a moved helper keeps any engine site gated and adds none.

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped — fixed the three-outcome scrub (engine prefixes scrubbed, `escher_` printed with content-named fields redacted, every other target dropped) and the rule "a target is admitted only together with a scrub rule for it". Why it matters: decision/scrub-sink-reach; a split that moved a site under a new module path keeps its target (`blitz_dom…`, `escher_driver`) so the scrub still admits and judges it.

@history_item n=3 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the command schema is silent; the cause names are the span field's domain — the private modules `command`, `refusal` and `schema`; `Command`, `Call`, `ArgValue` derive `Debug` but nothing prints, logs or fields them. Standing rule: the span's cause field takes a `Cause` name; never field a `Command`, `Call` or `ArgValue` with `Debug` nor an argument by schema name. Why it matters: these are the types `packages/escher-driver/src/command.rs` holds; the split must keep them unfielded (decision/debug-types-never-logged, decision/command-span-no-call-content).

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor is silent; `Outcome` joins the Debug-deriving types nothing logs — rule kept for the span's owner: field a cause by `Cause::name`, never a `Command`, `Call`, `ArgValue` or `Outcome` with `Debug`. Why it matters: same constraint over whichever new module `Outcome` lands in after the split.

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — eight cause names returned; the record of ids among the silent values — the session's record of ids (bounded, no reader outside the crate, no `Debug`) is printed, logged and fielded by nothing; re-pointed `command.rs:161` → `:167`. Why it matters: decision/session-id-record-unprinted must stay true with crate-private visibility unchanged by the split; precedent that a move in `command.rs` re-points obs citations.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the driver's command span as built; `tracing` ungated in escher-driver; the closed span's line — the span (target `escher_driver`, name `command`, INFO, entered before `validate`, eight fields each recorded only where it applies, never-fielded list) and the closed-span line shape and scrub. Why it matters: the origin of every contract/command-span/* and contract/escher-driver-command-span/* row and of decision/escher-crates-tracing-ungated; the split must leave the span's site, entry point and field order (contract/span-line/field-order) exactly as they are.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified — the closed-span line became a founder-ratified record class; the diff's three sizes (`added`, `removed`, `changed`) may reach a log, its content never. Why it matters: contract/span-line/* and the size fields are ratified surface; a restructure may not reshape or reorder them.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session host writes the command span; the command line writes answers, not logs; by-invocation reading — an `Outcome` is written in one place, by `Outcome::to_json`; standing rule: no span or log field holding what a call supplied or a screen reads; the workspace-built host prints one engine INFO line. Why it matters: decision/debug-types-never-logged ("written in one place") and decision/engine-tracing-by-invocation; the split must keep a single writer of the answer.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — `host_log` reads 0 id, name and sentinel needles at every level under both builds. Why it matters: decision/sink-host-read-at-trace and decision/session-logs-carry-no-page-content; `host_log` and `mcp_log` are the existing checks that would catch a moved body leaking into a log.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span, ratified — one span per MCP `tools/call`, fields `tool` and `ending`, `command` its child by nesting; a founder-ratified boundary widening. Why it matters: origin of contract/tool-call-span/*, contract/escher-driver-tool-call-span/*, decision/tool-call-span-two-closed-fields and decision/driver-span-fields-print; `stand_mcp_tools` guards it and the parent-child nesting must survive the split.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` in the two spans' closed sets — `help` answered by the server leaves a `tool_call` with no `command` child; a `help` handed to `Session::run` records its table word beside `unknown-verb`. Why it matters: contract/tool-call-span/help, contract/command-span/verb; the verb table's words are closed sets the split must not change.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` in the spans' closed sets, no image in a log — a shot records `verb` alone (or `verb` and `cause`, an empty crop `off-screen`); the image, its byte size and dimensions never fielded. Why it matters: contract/command-span/screenshot, contract/command-answer/shot, contract/refusal-cause/off-screen and fonts-unavailable, decision/shot-image-leaves-only-in-answer.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — eight causes, an accepted `advance` — `time-unavailable` retired, the eight cause names fixed; `advance` accepted on every session recording the acting fields. Why it matters: decision/advance-accepted-every-session, contract/command-span/cause and the eight contract/refusal-cause/* rows; ratified at the Epoch 2 close (2026-10-18-engine-debts), so the cause set is founder-fixed.

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `busy` on a real line — `stand_act_spans` drives `busy="render"`, `settled=false`. Why it matters: contract/command-span/busy is pinned by `stand_act_spans`, the check that proves the span's acting fields unchanged after `command.rs` is split.

@history_item n=15 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the load wait reads a clock and opens no span — `passes` on a step that waited on a load is the settle that went quiet; the wait sits inside the `command` span. Why it matters: contract/command-span/passes as it now reads.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — `Outcome::Linted` written by `to_json` alone; an accepted `lint` records `verb` alone; no new span field, the founder's word. Why it matters: contract/command-span/lint, contract/command-answer/linted.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read's span and its silence — a part read records `verb` alone; part number and count never fielded. Why it matters: contract/command-span/snapshot-part, decision/part-answer-never-logged.

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence and the host-sink part read measured) — `host_log`'s paged part read through the host's own sink, its spans the `command` spans recording `verb` alone. Why it matters: decision/hosted-call-through-session-run — a hosted call runs through `Session::run` alone; the split must not add a path around it.

@history_item n=19 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the complexity factoring's private items recorded silent — the precedent this chunk repeats: escher-driver code (`lint.rs`, `mcp_tests.rs`, `stand_requirement_sweep`) factored under the cognitive ceiling of 15 with private items only, no public symbol, verb, field, carrier, env var or log site added; silence census 0 over the changed files, `info_span!` still 2; recorded as one observed-absent row in log-coverage, not in §3. Why it matters: the form of this chunk's obs record (`bars.py` comes from that chunk), and the trap that the census must cover every file the split creates.

@history_item n=20 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — the log checks' shared fragment and its silence census — a test-only move recorded silent over each new file, escher-driver keeping its 2 `info_span!` sites. Why it matters: precedent for a behaviour-preserving split: a moved body is re-censused file by file, the span count read unchanged.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (cited line ranges re-pointed after the chunk's added lines) — re-pointed citations into `command.rs`, `execute.rs`, `json.rs`, `stand_act_spans.rs`, `host_log.rs`, `mcp_log.rs` with no claim moved. Why it matters: splitting `command.rs` moves cited code into new files; obs citations into it must be re-pointed (and re-filed, not just re-numbered).

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed past the chunk's added lines) — re-pointed citations into blitz-dom `document.rs`, `mutator.rs`, `events/pointer.rs` and escher-driver `execute.rs`. Why it matters: moving escher's additions out of `accessibility.rs`, `events/keyboard.rs` and `stylo/presentational_hints.rs` likewise shifts cited lines; the same re-point is owed at wrap.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the chunk's surfaces in §6, no log site added, the census's carve-out; the five new checks read silent; `stand_keyboard`'s reach) — engine changes (inert focus fixup, `multiple` select, `optgroup`, checkable `change`) recorded as `no site added` by an added-lines census; trap: an inline `#[cfg(test)]` module held out of that census is not covered by its 0; `stand_keyboard` drives the lean tasks, load page and Cells. Why it matters: these are escher additions now inside the three upstream bodies; the added-lines census and the keyboard/accessibility checks are the evidence for "behaviour unchanged".

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a `multiple` select's set, an `optgroup`'s label and the engine's `change` leave) — founder forks F6, F7, F10: a multiple select's chosen set leaves only as form entries, snapshot value and tree nodes; `optgroup` label as `Role::Group` name; engine `change` after `input`, the script runtime synthesising none. Why it matters: contract/page-events/*, contract/multiple-select-form-entries/chosen-option and decision/optgroup-label-crossing are carried by escher code inside `build_accessibility_node` and the input/keyboard path; moving them must keep each exit exactly.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — the engine's `focussed node` INFO line, once per focus move under the workspace build, re-read after the focus fixup. Why it matters: the one engine line the workspace-built host prints; a keyboard-handler move that changed focus order would move its count (decision/engine-tracing-by-invocation).

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels and the size read, the widening ratified) — `optgroup` label painted as a group header; `size` read as a capped whole number; founder-ratified boundary widenings. Why it matters: decision/optgroup-label-crossing as it reads; the select's escher code in blitz-dom must keep these crossings unchanged.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — the driver's fixed help and meaning texts are `'static` program text adding no log site or span field, the `command` span keeping its eight fields. Why it matters: contract/command-span/field-count; moving those texts out of `command.rs` keeps them fixed text.

@history_item n=28 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — 8. PII Scrubbing & Compliance: 2 row(s) — `~ decision/select-chosen-set-never-logged` (a single list box's chosen row and labels leave only in snapshot, diff and actions answer) and `~ decision/select-size-attribute-read`. Why it matters: the most recent change to a picked row; its list-box code is among the fork's additions in blitz-dom (keyboard and accessibility paths) that this chunk may move.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the harness contract takes the graph verb; the graph answer's carried set; the graph verb's command and tool_call lines) — `Outcome::Graphed` written by `to_json` alone, never logged; a graph's line records `verb` alone; the carried set a ratified widening (ratified 2026-11-15). Why it matters: contract/command-answer/graphed, contract/command-span/graph, decision/graph-answer-never-logged.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's graph modules and bridge dispatch add no log site) — a page with no registered state reaches no registry and dispatches as before. Why it matters: decision/no-state-page-skips-registry.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set; actions joins the spans; the actions read opens no span) — `Outcome::Actions` written by `to_json` alone; `Session::actions`, `check_target` and `target_of` reached inside the call's `command` span, opening none; the read changes nothing in the document. Why it matters: contract/command-span/actions, contract/actions-answer/logging, decision/actions-read-changes-nothing.

@history_item n=32 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the command span's reach gains the coverage role; the coverage line's carried fields; the coverage module logs nothing) — each replayed step writes its own `command` span through `Session::run`; the coverage line a command-line answer, never logged; ratified 2026-11-15. Why it matters: decision/coverage-line-never-logged, contract/coverage-line/logging; guard `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace`.

@history_item n=33 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — 4. Span / Trace Coverage: 1 row(s) — `+ decision/exploration-adds-no-span`: exploration opens no span, each driven call leaving its own command span. Why it matters: a picked row; the split must keep exploration's calls routed through `Session::run`.

@history_item n=34 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — 3. Observability Harness Contract: 2 row(s); 4. Span / Trace Coverage: 1 row(s) — `~ decision/checks-silent` (layout stand check's recorder) and `+ decision/layout-read-opens-no-span` (guard `stand_layout`'s `the_read_writes_no_escher_line_at_trace`). Why it matters: picked rows; "no span by design" must not be read as drift.

@history_item n=35 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ decision/layout-read-opens-no-span` (inside the layout call's command span), `~ contract/command-span/verb` (fifteen words), `+ contract/command-span/layout`, `~ contract/tool-call-span/tool` (fifteen), `~ contract/command-span/settled` (layout verb alone), `~ decision/sink-host-read-at-trace` (layout-answer key needles), `+ contract/layout-answer/logging`. Why it matters: the current form of nine picked rows; the verb table's fifteen words are the closed set the split carries over.

@history_item n=36 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — 8. PII Scrubbing & Compliance: 1 row(s) — `~ contract/layout-answer/logging`: the answer also crosses to the model provider on a live cold-agent run. Why it matters: the picked row's current promise.

@history_item n=37 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — 3. Observability Harness Contract: 3 row(s); 4. Span / Trace Coverage: 1 row(s) — `~ decision/driver-library-installs-no-subscriber`, `~ decision/driver-emits-no-event`: the driver sets only escher-telemetry's scoped capture around an acting step. Why it matters: picked rows; the capture wrapping is part of the acting path in the driver that the split moves and must keep exactly once per step.

@history_item n=38 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — 3. Observability Harness Contract: 4 row(s); 4. Span / Trace Coverage: 1 row(s) — `~ decision/checks-silent` (error-text check installs the sink), `~ decision/answers-echo-nothing-typed`, `~ decision/driver-library-installs-no-subscriber` (chaining panic hook), `~ decision/driver-emits-no-event` (keeps records and panics in process). Why it matters: current text of four picked rows.

@history_item n=39 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — 8. PII Scrubbing & Compliance: 3 row(s) — `+ decision/engine-sites-fixed-messages`: the engine's interpolating warn/error sites rewritten to fixed messages with named fields, the field-name scrub relying on it. Why it matters: if any moved blitz-dom body holds an engine log site, it moves verbatim with its fixed message and named fields.

@history_item n=40 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — 3. Observability Harness Contract: 1 row(s) — `~ decision/answers-echo-nothing-typed` except: an `error` field shown as written, cut at the text bound; ratified 2026-11-17. Why it matters: the picked row's current exception.

@history_item n=41 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — 3. Observability Harness Contract: 1 row(s) — `~ decision/checks-silent` except gains "the order page logs". Why it matters: the picked row's current exceptions; the chunk's checks must stay inside them.

@history_item n=42 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — 6. Log Coverage: 6 row(s) and 7 row(s) — `+ contract/cold-agent-verdict/refused-by-cause`, `~ decision/cold-agent-server-log-read-by-span-name`: the pipe now reads the command span's `cause` word from `server.log`. Why it matters: the command span's line is read by an outside consumer; its field names and cause words must not move in the split.

@history_item n=43 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — 3. Observability Harness Contract: 2 row(s); 6. Log Coverage: 2 row(s) — `~ decision/part-answer-never-logged`, `~ contract/command-answer/graphed`, `~ contract/graph-answer/shape`, `~ contract/command-span/graph`: the graph part read. Why it matters: the change that added the 72 lines pushing `command.rs` to 848; the current form of four picked rows the split must preserve.
