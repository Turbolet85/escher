@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — created the cold-agent pipe log (python3-`json` events `run.start`/`run.end` appended to `target/cold-agent/events.jsonl`, `status`/`cleanup` printed only, `verdict.json`) and set it outside escher's sink with no content-named key and no transcript content in events or verdict; transcript raw by design, gitignored, the committed copy host-path-masked; §9 state row local only. Any new verdict field this chunk adds (`{verb}_calls`, cause counts) stays inside this rule (rows `decision/cold-agent-log-no-user-content`, `decision/cold-agent-events-carry-no-user-content`, `contract/cold-agent-events/*`, `decision/committed-transcript-copies-masked`).

@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added the agent-run harness log (`scripts/agent-run.sh`, `target/agent-run/events.jsonl`, raw output in `run.log` never printed); the sibling harness-log precedent the cold-agent pipe copies (rows `decision/agent-run-raw-output-never-printed`, `decision/agent-run-raw-log-unscrubbed`).

@history_item n=3 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set escher's sink: stderr only, non-ANSI one-line text format, `RUST_LOG` filter defaulting to `warn`, allowlist scrub, escher-telemetry tracing ungated. Why the live server child needs `RUST_LOG=info` for any span line to reach `server.log` (rows `contract/telemetry-sink/stream`, `contract/telemetry-sink/rust-log`, `contract/escher-log-line/text-format`, `decision/escher-crates-always-trace`).

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became a second sink installer and its stderr was measured by level. Trap recorded: a zero read from a host that installs no sink says nothing about one that does — counting from `server.log` must read the sink-installing live child.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `escher_` targets print every field but the content-named set, other targets dropped whole; rule for later chunks: a target is admitted only together with a scrub rule (row `decision/escher-target-content-named-set`). The `command` span's `cause` and `tool_call`'s fields print under it, which is what the pipe would count.

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the refusal cause names (`Cause::name`) became the domain of the span's `cause` field; rule: field a cause by `Cause::name`, never a `Command`/`Call`/`ArgValue` with `Debug` nor an argument by its schema name. Bears on the planned verdict counts of refusals by cause: closed words only (rows `contract/refusal-cause/*`, `decision/command-span-no-call-content`).

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — all eight cause names are returned by code (validate and executor), so every `cause` value the chunk might count by reaches a `command` line (rows `contract/refusal-cause/*`).

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built) — the `command` span: target `escher_driver`, INFO, entered before `validate`, eight fields (`verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`) recorded only where they apply; escher-driver takes `tracing` ungated. The `cause` field the chunk wants to read from `server.log` is this span's (rows `contract/command-span/span`, `contract/command-span/cause`, `contract/escher-driver-command-span/*`, `decision/escher-crates-tracing-ungated`, `decision/one-span-per-command`).

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the closed span's line, PROVISIONAL) — defined the closed-span line shape (`span=…`, own fields in recorded order, Debug-quoted strings, then `message`, `time.busy`, `time.idle`) and that span pairs pass the same scrub; the textual form the pipe's `span_counts` greps, e.g. ` span="command" ` and ` cause="covered" ` (rows `contract/escher-log-line/span-line`, `contract/span-line/*`, `decision/closed-span-lines`, `decision/closed-span-lines-judged-alike`).

@history_item n=10 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line as a second record class (rows `decision/closed-span-lines`, `contract/escher-log-line/span-on-close-only`); the pipe reads a ratified carrier, not a new one, so counting `command` lines by cause needs no new ruling as long as no field is added.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the session host writes the command span; what a command prints is its answer under the ratified crossing, never a log field. Standing rule: no span or log field added that holds what a call supplied or a screen reads (rows `decision/answers-echo-nothing-typed`, `contract/mcp-role-stdout/json-rpc`).

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span (`tool`, `ending` ∈ accepted/refused/error), one per `tools/call`, parenting `command` by nesting; ratified by the founder as a new log field. The pipe's counted lines are these; a `command` child sits beside each `tool_call` line in `server.log`, so cause counts must not double-count tool calls (rows `contract/tool-call-span/*`, `contract/escher-driver-tool-call-span/*`, `decision/tool-call-span-two-closed-fields`, `decision/driver-span-fields-print`).

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined the two spans' closed sets: a `help` tool call leaves a `tool_call` with no `command` child, and a `help` handed to `Session::run` is refused `unknown-verb`; a line-pairing trap for cause counting (row `contract/command-span/verb`).

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` in the spans' closed sets; an accepted shot records `verb` alone, an empty crop refused `off-screen`, a shot's image never on a line (rows `contract/command-span/screenshot`, `contract/refusal-cause/off-screen`, `contract/refusal-cause/fonts-unavailable`).

@history_item n=15 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — moved the pipe onto escher's own MCP server: the per-run `mcp.json` spawns `escher-session mcp` with `RUST_LOG=info` on that child only, its span lines landing in `target/cold-agent/server.log`, counted by span name only (` span="tool_call" `, ` ending="refused" ` / ` ending="error" `), never printed; verdict fields became counts/booleans/closed words, `counts_agree` added; transcript carrying the stand's screens ratified («Один запуск, до $3»). The base the chunk widens; the pipe a reader, not an emitter (rows `decision/cold-agent-sink-on-live-child-only`, `decision/cold-agent-server-log-counted-only`, `decision/cold-agent-server-log-read-by-span-name`, `contract/cold-agent-verdict/counts-agree`, `decision/cold-agent-transcript-raw`).

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — cause set cut to eight (`time-unavailable` retired); an `advance` is accepted on every session and records the acting fields, no cause; an `advance` tool call parents one `command` child (rows `decision/advance-accepted-every-session`, `contract/refusal-cause/*`).

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the eight cause names and the accepted `advance`; the cause domain a by-cause count reads is ratified, not provisional.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing; the Recording directory is `0700`, call content at rest, local only, ratified at-rest widening; an MCP call after a failed recording ends `error` — an `error` ending the pipe counts in `wrong_calls` (rows `decision/recording-dir-private`, `contract/kept-check/*`).

@history_item n=19 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — `lint` became a verb inside the `command` span, no new field (founder «Без поля»); an accepted `lint` records `verb` alone; precedent for a new verb crossing without a span field (row `contract/command-span/lint`).

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read's line is `span="command" verb="snapshot"` and holds no part number; why `part_reads` cannot be read from `server.log` and comes from the transcript (rows `contract/command-span/snapshot-part`, `decision/part-answer-never-logged`).

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits by hand (its pin reads none); F2's claim edits there touch text this plan owns.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the verdict fields, the live child's recording, the six apps' carriage) — 0.2.0's bar entry: live child `RUST_LOG=info … mcp --record target/cold-agent/record` (`mkdir -m 700`), `server.log` read only to count `tool_call` lines, refused/error endings and accepted lines per counted verb (`lint`·`screenshot`·`advance`); verdict gained `wrong_call_bar_pct` (5), `{verb}_calls`, `part_reads`, recording counts; reasons `capability-unused`·`recording-not-kept`·`wrong-calls-over-bar` ordered after `task-unmet`; the six apps' carriage ratified. The direct precedent this chunk extends — new `{verb}_calls`, cause counts and the named bar must fit this closed-field shape and reason order (rows `contract/cold-agent-verdict/*`, `contract/cold-agent-counted-verbs/*`, `contract/cold-agent-verdict-reasons/*`, `decision/cold-agent-record-owner-only`, `decision/verdict-fields-closed`).

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — the host serves six slugs (Cells, load among them) and the paged part read through the host's sink measured at 0 needles; the stand surface the cold agent drives.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — `covered`'s added clause and the select list are fixed `'static` text, no span field, the `command` span keeping eight fields; the CARRY that `covered` refusals can move `wrong_calls` reads these (row `contract/refusal-cause/covered`).

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a page's labels as pixels, ratified) — a shot of an open list carries option and group labels as pixels, ratified widening; a live run opening `select#flight-type` carries that to the provider (rows `decision/open-list-shot-carries-labels`, `decision/optgroup-label-crossing`, `decision/select-list-label-never-logged`, `contract/snapshot-diff/open-list-row-bounds`).

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe's verbs take graph) — `VERBS` thirteen with `graph`, fake server answering `{"nodes":[],"states":[]}`; `graph` uncounted, no verdict field, `capability-unused` unchanged — the CARRY this chunk decides (row `decision/graph-actions-layout-not-counted-verbs`).

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's command and tool_call lines) — a graph line is `span="command" verb="graph"`, an argument refused `malformed`; a `graph` tool call parents one `command` recording `verb` alone; no field added — so accepted `graph` lines are countable from `server.log` like the counted verbs (rows `contract/command-span/graph`, `contract/tool-call-span/tool`).

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the harness contract takes the graph verb; the graph answer's carried set) — `Outcome::Graphed` never logged or fielded; the answer carries ids, component/state/handler names, closed flag words, booleans — a ratified widening that named no model-provider crossing, the F3 crossing question the scope raises (rows `decision/graph-answer-never-logged`, `contract/graph-answer/*`, `contract/command-answer/graphed`).

@history_item n=29 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's VERBS fourteen, actions uncounted) — `actions` joined after `graph`, fake server answering it, uncounted; the second CARRY this chunk decides.

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions joins the spans; the actions read opens no span; the actions answer's carried set) — an actions line is `span="command" verb="actions"`, `tool_call` parents one `command`, no field added; `Session::actions` changes nothing; the answer's ids, owners, verb and cause words a ratified widening, never logged, not yet named as crossing to the provider (rows `contract/command-span/actions`, `decision/actions-read-changes-nothing`, `contract/actions-answer/*`).

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the command span's reach gains the coverage role) — handler coverage is the `escher-session coverage` role, no MCP tool, adding no span; the basis for naming it out of a cold agent's reach.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows (handles), the form every picked row and any amendment this chunk proposes takes.

@history_item n=33 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer's carried set) — the graph answer's carriage ratified by the founder's own word; the crossing to the model provider on a live run still unnamed (F3).

@history_item n=34 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `decision/layout-read-opens-no-span` added and `stand_layout`'s own recording subscriber named an exception to `decision/headless-stand-no-sink`.

@history_item n=35 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — acting steps run under escher-telemetry's scoped capture (prints nothing); `RUST_LOG` filters the sink layer alone (rows `decision/headless-stand-no-sink`, `contract/telemetry-sink/rust-log`, `contract/escher-sink-env/rust-log`); `caused` items come from this capture, never from `server.log`.

@history_item n=36 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `caused` answers carry record text, `decision/answers-echo-nothing-typed` gaining its exception; `decision/scrub-by-field-name` replaced the sink-only scrub; `password` joined the content-named set; `error`/`url` scrub rows rewritten. A live run asking for `caused` carries this text to the provider (rows `decision/answers-echo-nothing-typed`, `decision/scrub-by-field-name`, `decision/escher-target-content-named-set`, `contract/telemetry-scrub-set/*`).

@history_item n=37 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's `error` field crosses as written, cut at the text bound (operator «Открыть error в caused», ratified 2026-11-17); the current form of rows `contract/telemetry-scrub-set/error`, `decision/scrub-by-field-name`, `decision/answers-echo-nothing-typed`.

@history_item n=38 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `layout` became the fifteenth word of `command`'s `verb` and `tool_call`'s `tool`; `contract/command-span/layout` added; `settled`'s verb-alone list gained layout; layout read runs inside the call's span; `decision/sink-host-read-at-trace` gained layout-answer key needles; `contract/layout-answer/content`/`logging` added. Fixes what a `layout_calls` count would read.

@history_item n=39 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — `decision/graph-and-actions-not-counted-verbs` replaced by `decision/graph-actions-layout-not-counted-verbs` (layout uncounted, no `layout_calls`, `capability-unused` unchanged), and `contract/layout-answer/logging` names the transcript's crossing to the model provider on a live run (operator «Добавить сейчас»); delegate confirms ratified. Precedent: counting any of the three changes that decision row, and graph/actions have no such provider-crossing row yet.
