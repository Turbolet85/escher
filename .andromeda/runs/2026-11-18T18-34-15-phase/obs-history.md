@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — the cold-agent pipe log, scrub position and artifact: created the pipe's python3-`json` events (`run.start`, `run.end` appended to `target/cold-agent/events.jsonl`; `status`, `cleanup` printed only), placed the pipe outside escher's sink with no content-named key and no transcript content in events or verdict, recorded `transcript.jsonl` raw by design, never printed, gitignored, the one committed copy host-path-masked, and the state row local only, uploaded by no CI leg, removed by `cleanup` alone. Origin of decision/harness-log-not-telemetry-sink, decision/cold-agent-log-no-user-content, decision/cold-agent-transcript-raw, decision/committed-transcript-copies-masked and decision/cold-agent-cleanup-scoped: whatever this chunk commits of the live run must stay masked (`gate.py hygiene` P1 already refuses an unmasked host path).

@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run harness log recorded as harness metadata, not a telemetry sink, and its state local only; the precedent the cold-agent pipe log follows (decision/harness-log-not-telemetry-sink, decision/local-state-never-uploaded).

@history_item n=3 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the pipe moved to `run stand` over `escher-session mcp` with `RUST_LOG=info` on that live child only (its span lines to `target/cold-agent/server.log`, counted by span name only), replay children stderr to `/dev/null`; verdict fields gained `server_tool_calls`, `refused_calls`, `error_calls`, `tasks_met`, the check fields and `counts_agree`; reasons `no-check` · `check-failed` · `check-cannot-fail`; `server.log` and `check.json` joined the never-printed files; the transcript's carriage of the stand's screen was ratified by the founder with his one-run, $3 word. Basis of decision/cold-agent-sink-on-live-child-only, decision/replay-children-uninstrumented, contract/cold-agent-verdict/counts-agree and decision/cold-agent-state-never-printed: the pipe is a reader of the ratified `tool_call` line, not a new emitter.

@history_item n=4 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe's verdict fields, its live child's recording and the six apps' carriage) — the live child became `exec env RUST_LOG=info … escher-session mcp --record target/cold-agent/record`, `record/` owner-only (`mkdir -m 700`), never printed or copied, removed by `cleanup`; verdict gained `wrong_call_bar_pct` (5), `lint_calls` · `screenshot_calls` · `advance_calls`, `part_reads`, `recorded_sessions` · `recording_kept`; `wrong_calls` decides against the bar; reasons `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar` ordered after `task-unmet` and before the check's; §8 the transcript's carriage of six apps (Cells, the flight booker's options, a lint's findings, a shot) confirmed by the founder; no live kept check committed. The 0.2.0-era measured run this chunk's live run supersedes; the carriage list is the one the seventh app and F4/F5 now extend.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence and the host-sink part read measured) — the served stand's task table (six slugs then, seven since the order page) is the program's fixed text in the usage line and the MCP `start` enum; the served stand's boots read silent; a paged part read through the host's own sink measured at 0 needles. Background for the seven apps the run is given and the part-read requirement.

@history_item n=6 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the escher-driver emitter's reach (the `command` and `tool_call` spans, stderr only, no line at `warn`, the scrub's limits) and its pin reads none of these claims, so a change there is made by hand (decision/capability-statement-updated-by-hand). This chunk rewrites the capability page from the verdict: any span or scrub statement it touches must be kept true by hand.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; the closed span's line) — the `command` span (target `escher_driver`, INFO, eight fields recorded only where they apply, never an argument, `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`) and the closed-span line shape `{time} {LEVEL} {target} service.name=… service.version=… span=…`, fields in order recorded, then `message`, `time.busy`, `time.idle`, written only on close and judged pair by pair by the scrub. The line shape the pipe's `span_counts` and `refused_by_cause` parse in `server.log` (contract/escher-log-line/span-line, contract/span-line/value-quoting, decision/closed-span-lines).

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed span's line ratified by the founder as a second record class (decision/closed-span-lines, decision/closed-span-lines-judged-alike); the diff's three sizes may reach a log while its content never does.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span ratified: one per MCP `tools/call`, entered before params are read, held until the reply is written, two closed fields `tool` and `ending` (`accepted` · `refused` · `error`), `command` its child; the `mcp` role writes protocol lines only on stdout, the sink stderr only. The span lines every counted verdict field (`*_calls`, `wrong_calls`, `counts_agree`) is read from.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined the `tool_call` `tool` set, a `help` call ending `accepted` with no `command` child (contract/tool-call-span/help, contract/tool-call-span/command-child); a help call counts as a `tool_call` line with no command line beside it.

@history_item n=11 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — eight cause names (`time-unavailable` retired), an `advance` accepted on every session recording the acting fields with no cause, an `advance` tool call ending `accepted` with one `command` child; PROVISIONAL then. The eight words `refused_by_cause` counts, and decision/advance-accepted-every-session behind `advance_calls`.

@history_item n=12 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — the one-virtual-clock marks ratified by the founder: the eight cause names and the accepted `advance` stand on his word.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing; a call after which the recording ended is answered `recording-failed`, its `tool_call` ending `error`; the `mcp` role's `--record <dir>` checked before serving, a directory open to group or others refused with status 3; the Recording directory row (`0700`, one `0600` check per session, call content at rest, local only) ratified by the founder. Basis of decision/recording-dir-private, decision/recording-bounds and the `recording_kept` / `recorded_sessions` reading; a recording failure on the live run reads as an `error` ending and moves `wrong_calls`.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer at rest (a shot's without `png`), the committed checks limited to the two Counter checks, replay steps writing their own `command` spans; behind decision/committed-checks-hold-no-private-data and decision/cold-agent-kept-checks-stay-local: no kept check of the live run may be committed.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a shot records `verb` alone (`off-screen` for an empty crop), never its image, size or dimensions; a shot's image leaves only in its answer (decision/shot-image-leaves-only-in-answer, contract/command-span/screenshot); the live run's one screenshot crosses to the provider in the transcript only.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — `lint` became an instance verb recording `verb` alone, its findings never logged or fielded, ratified by the founder «Без поля (Рекомендую)» (contract/command-span/lint, contract/command-answer/linted); `lint_calls` counts these lines.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read's line is `span="command" verb="snapshot"` with no part number, count, size or text (contract/command-span/snapshot-part, decision/part-answer-never-logged); `part_reads` counts paged reads without any of those.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — `covered`'s added meaning and the click/type/press help are the program's fixed text, no span field added; the scope's CARRY (cry-32fid2ry) traces to this chunk: `covered` under an open list may move `wrong_calls`, now read per cause in `refused_by_cause`. The companion entries record an open list's labels read at 0 needles through the host and the role, behind `list_opens`.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set; the cold-agent pipe's verbs take graph; the graph verb's command and tool_call lines) — the graph answer carries ids, component names, registered state and handler event names, closed flag words and booleans, never a value (a boundary widening, then PROVISIONAL); `graph`'s line is `verb="graph"` holding no node or name; the pipe's `VERBS` took `graph` uncounted then. F4 now ratifies `graph` answers to the model provider on the live run; the uncounted state was reversed at 2026-11-18-cold-agent-bar.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer's carried set) — the graph answer's carried set moved from PROVISIONAL to ratified by the founder's own word (contract/graph-answer/content).

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set; VERBS fourteen, actions uncounted; actions in the spans) — the actions answer carries ids, owner ids, verb and cause words and booleans only, ratified; its `command` line holds nothing of its answer; the read changes nothing (decision/actions-read-changes-nothing); the pipe's `VERBS` took `actions` uncounted then, reversed at 2026-11-18-cold-agent-bar.

@history_item n=22 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — decision/layout-read-opens-no-span added, and `stand_layout`'s own TRACE recorder named as an exception in decision/headless-stand-no-sink and decision/checks-silent.

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen with `layout` (contract/command-span/verb, contract/tool-call-span/tool), contract/command-span/layout and contract/command-span/settled name it; decision/sink-host-read-at-trace gained the layout-answer key needles; contract/layout-answer/content and /logging added, ratified via the delegate confirm later ratified by the founder.

@history_item n=24 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — contract/layout-answer/logging gained the crossing to the model provider in the transcript on a live cold-agent run (operator-resolved «Добавить сейчас»); decision/graph-and-actions-not-counted-verbs replaced by decision/graph-actions-layout-not-counted-verbs (since reversed); and the founder ratified the delegate confirms of the error-field and layout cards. The run's provider carriage of `layout` answers rests here.

@history_item n=25 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (6. Log Coverage: 7 rows) — `graph`, `actions`, `layout` became counted verbs (contract/cold-agent-counted-verbs/*); capability-unused gained the list opening; decision/cold-agent-server-log-read-by-span-name now reads the `tool_call` span's `tool`/`ending` and the `command` span's cause word; decision/cold-agent-verdict-holds-no-content admits objects of closed words to counts; decision/graph-actions-layout-not-counted-verbs removed (reversed). The pipe rules the live run is judged by; the scope plans no change to them.

@history_item n=26 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (6. Log Coverage: 6 rows) — verdict fields `graph_calls`, `actions_calls`, `layout_calls`, `list_opens`, `refused_by_cause` added and decision/verdict-fields-closed widened to objects of closed words to counts; the report must state `refused_by_cause` as read (CARRY cry-32fid2ry).

@history_item n=27 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (4. Span / Trace Coverage: 1 row) — contract/cold-agent-verdict/capability-unused shortened to «a counted verb, a part read or a list opening went unused», stated without the inventory.

@history_item n=28 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (3. Observability Harness Contract: 1 row) — decision/cold-agent-server-log-counted-only shortened to «reads server.log only to count its span lines, never printing or copying it»; the run's evidence keeps `server.log` uncopied.

@history_item n=29 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (4. Span / Trace Coverage and 6. Log Coverage: capability-unused) — both capability-unused rows now also fire when any of panic, error and log in `caused_by_kind` reads 0; the route CARRY this chunk concretizes into v030-15's acceptance (each kind at least 1).

@history_item n=30 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (6. Log Coverage: caused_by_kind) — contract/cold-agent-verdict/caused-by-kind added: counts of accepted acting answers whose `caused` list holds each kind, read from the acting answers, a non-JSON answer skipped.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Observability Harness Contract: checks-silent; 6. Log Coverage: exit-codes/2) — the order page adds one fieldless info log site under its own target, named as an exception in decision/checks-silent; the usage line names the served task names without a count (seven now). The seventh app the run is given and the one log source its `log` kind reads.

@history_item n=32 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — the scoped capture keeps each record's text and the step's panics in process (decision/driver-emits-no-event, decision/headless-stand-no-sink), a caused item's message, chain and kept fields hold what their author wrote (decision/answers-echo-nothing-typed except), decision/scrub-at-sink-not-call-sites replaced by decision/scrub-by-field-name, `password` joined the content-named set, the capture admits engine targets at warn and above (decision/capture-framework-targets-warn-up). The `caused` content the order page's authored error text crosses to the provider on (F5).

@history_item n=33 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a caused item's `error` field crosses as written, cut at the text bound, the capture judging by the answer's own hidden set (contract/telemetry-scrub-set/error, decision/scrub-by-field-name, decision/answers-echo-nothing-typed); later ratified by the founder (2026-11-17-a-component-in-isolation). The live run carries the order page's error text to the provider in the transcript under this.

@history_item n=34 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — the driver sets escher-telemetry's scoped capture around an acting step and installs no sink; RUST_LOG's EnvFilter filters the sink layer alone, the capture layer unfiltered (contract/telemetry-sink/rust-log, contract/escher-sink-env/rust-log); decision/capture-framework-targets-warn-up added. The live child's `RUST_LOG=info` governs `server.log` only, never what `caused` counts.

@history_item n=35 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (7. Error Capture & Reporting) — decision/exploration-catches-panics added: a panic in an explored call is caught and reported as Panicked with no payload, the hook still running; exploration is one of the places the reach statement says the run does not reach.

@history_item n=36 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — escher-telemetry's sink: `RUST_LOG` EnvFilter defaulting to `warn`, one non-ANSI text layer on stderr only, the `LogTracer` bridge, ungated `tracing` in the escher crate, the allowlist scrub. Origin of contract/telemetry-sink/*, contract/escher-log-line/text-format and decision/escher-crates-always-trace.

@history_item n=37 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-session` became the second sink-installing binary; trap recorded: a zero read from a host that installs no sink says nothing about one that does (decision/sink-installed-first-in-main, decision/scrub-sink-reach).

@history_item n=38 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records dropped whole at every level whatever `RUST_LOG` names; rule for later chunks: a target is admitted only together with a scrub rule for it (decision/sink-target-allowlist). The live child's `server.log` at `info` holds only admitted, scrubbed lines.

@history_item n=39 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — what a command prints is its answer under the ratified crossing, never a log field; standing rule that no later surface adds a span or log field holding what a call supplied or a screen reads (decision/fixed-diagnostics, decision/escher-session-fixed-diagnostics).

@history_item n=40 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — no OTel export, crate, egress or credential path in escher (absence/telemetry-backend); the live run adds no export.

@history_item n=41 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence · overview) on the founder's approval; every later entry names its change by row handle, the form the picked rows and any amendment this chunk proposes take.
