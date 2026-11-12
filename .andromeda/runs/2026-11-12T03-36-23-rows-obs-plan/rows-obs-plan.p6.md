@row_none n=1 rec=1 type=7
@row_none n=2 rec=2 type=7
@row_none n=3 rec=3 type=7
@row_none n=4 rec=4 type=7
@row_none n=5 rec=5 type=7
@row_none n=6 rec=6 type=7
@row_none n=7 rec=7 type=7
@row_none n=8 rec=8 type=7
@row_add n=1 row=contract/command-span/span area=span-trace-coverage rec=9
boundary: The driver's command span
item: span
promise: One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
@row_add n=2 row=contract/command-span/verb area=span-trace-coverage rec=9
boundary: The driver's command span
item: verb
promise: The verb table's own word for a known verb, run or refused; never the caller's verb text; unrecorded for a verb the table lacks.
@row_add n=3 row=contract/command-span/settled area=span-trace-coverage rec=9
boundary: The driver's command span
item: settled
promise: Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions and a part read record verb alone, and a refusal records verb and cause alone.
@row_add n=4 row=contract/command-span/busy area=span-trace-coverage rec=9
boundary: The driver's command span
item: busy
promise: The schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
@row_add n=5 row=contract/command-span/passes area=span-trace-coverage rec=9
boundary: The driver's command span
item: passes
promise: Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
@row_add n=6 row=contract/command-span/added area=span-trace-coverage rec=9
boundary: The driver's command span
item: added
promise: The length of the returned diff's added list, recorded from an acting verb's outcome.
@row_add n=7 row=contract/command-span/removed area=span-trace-coverage rec=9
boundary: The driver's command span
item: removed
promise: The length of the returned diff's removed list, recorded from an acting verb's outcome.
@row_add n=8 row=contract/command-span/changed area=span-trace-coverage rec=9
boundary: The driver's command span
item: changed
promise: The length of the returned diff's changed list, recorded from an acting verb's outcome.
@row_add n=9 row=contract/command-span/cause area=span-trace-coverage rec=9
boundary: The driver's command span
item: cause
promise: Recorded at one site for every refusal the run returns: one of eight fixed refusal class words, holding nothing a call supplied.
@row_add n=10 row=contract/refusal-cause/unknown-verb area=span-trace-coverage rec=9
boundary: Refusal cause names
item: unknown-verb
promise: A closed refusal class word recorded in the command span's cause field, returned by validation.
@row_add n=11 row=contract/refusal-cause/malformed area=span-trace-coverage rec=9
boundary: Refusal cause names
item: malformed
promise: A closed refusal class word recorded in the command span's cause field, returned by validation.
@row_add n=12 row=contract/refusal-cause/not-found area=span-trace-coverage rec=9
boundary: Refusal cause names
item: not-found
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=13 row=contract/refusal-cause/stale area=span-trace-coverage rec=9
boundary: Refusal cause names
item: stale
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=14 row=contract/refusal-cause/disabled area=span-trace-coverage rec=9
boundary: Refusal cause names
item: disabled
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=15 row=contract/refusal-cause/covered area=span-trace-coverage rec=9
boundary: Refusal cause names
item: covered
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=16 row=contract/refusal-cause/off-screen area=span-trace-coverage rec=9
boundary: Refusal cause names
item: off-screen
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.
@row_add n=17 row=contract/refusal-cause/fonts-unavailable area=span-trace-coverage rec=9
boundary: Refusal cause names
item: fonts-unavailable
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with.
@row_add n=18 row=decision/command-span-no-call-content area=span-trace-coverage rec=9
name: No call content in the command span
rule: The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value.
why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value.
guard: stand_act_spans
@row_add n=19 row=decision/advance-accepted-every-session area=span-trace-coverage rec=9,14
name: advance accepted on every session
rule: The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause.
why: Ratified.
@row_none n=9 rec=10 type=4
relied: A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone.
@row_none n=10 rec=11 type=7
@row_none n=11 rec=12 type=7
@row_none n=12 rec=13 type=7
@row_add n=20 row=decision/tool-call-span-two-closed-fields area=span-trace-coverage rec=14
name: tool_call span with tool and ending
rule: Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word.
why: Ratified as the span's shape.
guard: stand_mcp_tools
@row_add n=21 row=contract/tool-call-span/span area=span-trace-coverage rec=14
boundary: The MCP tool_call span
item: span
promise: One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
@row_add n=22 row=contract/tool-call-span/tool area=span-trace-coverage rec=14
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool.
@row_add n=23 row=contract/tool-call-span/ending area=span-trace-coverage rec=14
boundary: The MCP tool_call span
item: ending
promise: One of three closed words saying how the call ended: accepted, refused or error.
@row_add n=24 row=contract/tool-call-ending/accepted area=span-trace-coverage rec=14
boundary: The tool_call span's ending
item: accepted
promise: The tool call was accepted and answered.
@row_add n=25 row=contract/tool-call-ending/refused area=span-trace-coverage rec=14
boundary: The tool_call span's ending
item: refused
promise: The tool call was refused, its reply one text block with isError true.
@row_add n=26 row=contract/tool-call-ending/error area=span-trace-coverage rec=14
boundary: The tool_call span's ending
item: error
promise: The tool call ended in a session error or a protocol error.
@row_add n=27 row=contract/tool-call-span/command-child area=span-trace-coverage rec=14
boundary: The MCP tool_call span
item: command child
promise: An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child.
@row_add n=28 row=contract/cold-agent-verdict/lint-calls area=span-trace-coverage rec=15
boundary: The cold-agent verdict
item: lint_calls
promise: The count of accepted tool_call lines whose tool is lint in the live server child's server.log.
@row_add n=29 row=contract/cold-agent-verdict/screenshot-calls area=span-trace-coverage rec=15
boundary: The cold-agent verdict
item: screenshot_calls
promise: The count of accepted tool_call lines whose tool is screenshot in the live server child's server.log.
@row_add n=30 row=contract/cold-agent-verdict/advance-calls area=span-trace-coverage rec=15
boundary: The cold-agent verdict
item: advance_calls
promise: The count of accepted tool_call lines whose tool is advance in the live server child's server.log.
@row_add n=31 row=contract/cold-agent-verdict/wrong-calls area=span-trace-coverage rec=15
boundary: The cold-agent verdict
item: wrong_calls
promise: Fed by the counts of refused and error tool_call lines in the live server child's server.log.
@row_add n=32 row=contract/cold-agent-verdict/capability-unused area=span-trace-coverage rec=15
boundary: The cold-agent verdict
item: capability-unused
promise: The reason given when lint_calls, screenshot_calls or advance_calls reads 0.
@row_add n=33 row=overview/telemetry-strategy area=telemetry-strategy rec=16
text: Telemetry is the tracing crate's event macros and, in escher-driver, two hand-built spans, command and tool_call. In the engine and upstream crates tracing is compiled in only with the tracing cargo feature. escher-telemetry takes tracing with no feature and always compiles its startup and panic events into the two native binaries, seven_guis_native and escher-session. escher-driver takes tracing ungated, so its spans are written in escher-session through its host, mcp and coverage roles, and by no code path in the windowed seven_guis_native.
@row_add n=34 row=decision/tracing-for-telemetry area=telemetry-strategy rec=16
name: tracing as the telemetry crate
rule: Telemetry is written with the tracing crate's event macros and span macros.
@row_add n=35 row=decision/engine-tracing-feature-gated area=telemetry-strategy rec=16
name: Engine tracing behind a cargo feature
rule: In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
@row_add n=36 row=decision/escher-crates-tracing-ungated area=telemetry-strategy rec=16
name: escher crates take tracing ungated
rule: escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
@row_none n=13 rec=17 type=4
relied: A workspace-wide build turns the engine's tracing feature on, so a test child that installs escher's sink at info also receives engine events.
@row_add n=37 row=absence/telemetry-backend area=telemetry-strategy rec=18,22
capability: A telemetry backend or OTel SDK (Sentry, OpenTelemetry, Prometheus or a metrics exporter)
@row_none n=14 rec=19 type=7
@row_none n=15 rec=20 type=7
@row_none n=16 rec=21 type=7
@row_add n=38 row=decision/no-otel-export area=bootstrap-phases-derive-for-route-setup-project rec=22
name: No OTel export
rule: The product ships no OpenTelemetry export: no OTel crate, no egress and no OTLP header credential path.
why: Ratified; the export transport and the credential path are left undecided.
@row_add n=39 row=contract/sink-resource-keys/service-name area=bootstrap-phases-derive-for-route-setup-project rec=22
boundary: The sink's resource keys
item: service.name
promise: The sink names the service under the OTel resource key service.name.
@row_add n=40 row=contract/sink-resource-keys/service-version area=bootstrap-phases-derive-for-route-setup-project rec=22
boundary: The sink's resource keys
item: service.version
promise: The sink gives the service version under the OTel resource key service.version.
@row_add n=41 row=decision/sink-target-allowlist area=bootstrap-phases-derive-for-route-setup-project rec=23
name: Sink target allowlist
rule: escher's sink admits records by a target allowlist: engine-prefixed targets are scrubbed to their safe fields, content-named fields of escher targets are redacted, and records from any other target are dropped at every level.
why: Keeps stable ids and accessible names out of the logs, even at debug and trace.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
@row_none n=17 rec=24 type=S
