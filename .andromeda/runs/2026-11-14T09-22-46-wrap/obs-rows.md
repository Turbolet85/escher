# obs-plan · the rows 2026-11-14-bundled-fonts touches — picked 4 of 274 · rule any · pin 861

## 3-observability-harness-contract · ## 3. Observability Harness Contract
> NOT YET MEASURED — product mode, snapshot integration, trace context propagation and heartbeat ticks, and a JSON schema or log-file location for escher's own sink: the reading recorded none of them
decision/agent-run-raw-output-never-printed · Agent-run raw output never printed — The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed. · r1
decision/answers-echo-nothing-typed · Answers echo nothing the caller typed — No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask. · r1
decision/checks-silent · Silent checks and shared modules — Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read. · except: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads. · r2 · add-to · F1
decision/cold-agent-record-owner-only · Cold-agent record directory owner-only — The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700). · r1
decision/cold-agent-server-log-counted-only · Cold-agent server log read only to count — The cold-agent pipe reads server.log only to count tool_call span lines, their refused and error endings and the accepted lines of each counted verb (lint, screenshot, advance), never printing or copying it. · r1
decision/cold-agent-sink-on-live-child-only · Cold-agent live server child logs at info — The cold-agent pipe sets RUST_LOG=info on the live escher-session mcp server child only, so that child installs escher's own sink and its stderr goes to target/cold-agent/server.log. · r1
decision/cold-agent-state-never-printed · Cold-agent state never printed or copied — The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied. · r1
decision/debug-types-never-logged · Answer-bearing types never logged — Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer. · r1
decision/driver-library-installs-no-subscriber · Driver library installs no subscriber — The driver library, its command line, MCP server and coverage reader included, installs no subscriber and reads no environment variable; only a binary's own main installs the sink. · r1 · add-to · F1
decision/empty-stderr-check-unsets-rust-log · Empty-stderr checks unset RUST_LOG — A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment. · r1
decision/harness-json-via-python-json · Harness JSON lines encoded by python's json module — The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash. · r1
decision/harness-log-not-telemetry-sink · Harness logs are not telemetry sinks — The agent-run and cold-agent harness logs are harness metadata, not telemetry sinks: the harness itself installs no tracing subscriber, OTel setup or third-party logger. · r1
decision/headless-stand-no-sink · Headless stand runs with no sink — A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink. · except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture. · r1
decision/log-check-failures-name-kind-count · Log check failures name a kind and a count — A failing log check's message names a kind and a count, never a needle or anything the host wrote. · r1
decision/one-span-per-command · One span per driver command — Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below. · r1
decision/part-answer-never-logged · A part's answer is never logged — A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded. · r1
decision/replay-children-uninstrumented · Replay children uninstrumented — A judged check is replayed on further escher-session mcp children spawned directly, without --record, with stderr to /dev/null. · why: uninstrumented by design · r1
decision/session-id-record-unprinted · Session id record kept private — The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded. · r1
decision/sink-host-read-at-trace · Sink-installing host read at trace — A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write. · guard: host_log, mcp_log · r1
decision/sink-installed-first-in-main · Telemetry sink installed first in each native main — Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing. · r1
contract/command-answer/graphed · driver command answer · nodes and states — A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value. · r1
contract/command-answer/linted · driver command answer · findings — A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded. · r1
contract/command-answer/shot · driver command answer · shot — A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded. · r1
contract/coverage-role-stdout/coverage-line · escher-session coverage role stdout · coverage line — The coverage line, holding ids, owner ids, event names and two counts, is written on stdout as an answer and never a log line; a malformed refusal holds nothing of the path. · r1
contract/escher-log-line/service-name · escher's sink log line · service.name — Every line carries service.name, the OTel resource key, read from the binary's own CARGO_PKG_NAME; the name is the package's, so two binaries of one package are not told apart. · r1
contract/escher-log-line/service-version · escher's sink log line · service.version — Every line carries service.version, the OTel resource key, read from the binary's own CARGO_PKG_VERSION. · r1
contract/escher-log-line/text-format · escher's sink log line · line format — Each record is written in a one-line text format, not JSON. · r1
contract/escher-session-stdout/command-answer · escher-session stdout · command answer — Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet. · r1
contract/kept-check/accepted · kept check file · accepted — Each answered call records whether it was accepted. · r1
contract/kept-check/answer · kept check file · answer — Each answered call records the answer itself, a screenshot's without the PNG. · r1
contract/kept-check/answer-digest · kept check file · answer digest — Each answered call records its answer's FNV-1a 64 digest. · r1 · add-to · F1
contract/kept-check/arguments · kept check file · arguments — Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print. · r1
contract/mcp-role-exit-status/0 · escher-session mcp role exit status · 0 — The server ended without error. · r1
contract/mcp-role-exit-status/3 · escher-session mcp role exit status · 3 — The server ended on an error, or a record directory was refused before serving; the fixed message is on stderr and stdout is empty. · r1
contract/mcp-role-stdout/json-rpc · escher-session mcp role stdout · JSON-RPC lines — In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout. · r1
contract/script-diagnostics-drain/error-retention · Embedder drain of script diagnostics · error retention between drains — At most 256 JS errors are retained between drains. · r1
contract/script-diagnostics-drain/take-js-errors · Embedder drain of script diagnostics · take_js_errors — Embedders drain JS errors with take_js_errors. · r1
contract/script-diagnostics-drain/take-messages · Embedder drain of script diagnostics · take_messages — Embedders drain JS messages with take_messages. · r1
contract/telemetry-sink/log-records · escher's telemetry sink · log records — Records of the log crate are bridged into the tracing sink. · r1
contract/telemetry-sink/rust-log · escher's telemetry sink · RUST_LOG — The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset. · r1
contract/telemetry-sink/stream · escher's telemetry sink · output stream — The sink writes uncoloured lines to stderr only, never to stdout. · r1

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/no-otel-export · No OTel export — The product ships no OTel export: no OTel crate, no egress and no OTEL_EXPORTER_OTLP_HEADERS credential path. · why: Ratified; the export transport and the credential path are left undecided. · r1
contract/sink-resource-keys/service-name · The sink's resource keys · service.name — The sink names the service under the OTel resource key service.name. · r1
contract/sink-resource-keys/service-version · The sink's resource keys · service.version — The sink gives the service version under the OTel resource key service.version. · r1

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/driver-emits-no-event · Driver emits no event and installs no subscriber — escher-driver emits no log event and installs no subscriber; its telemetry is its spans alone. · r1 · add-to · F1
decision/driver-spans-carry-no-content · Driver spans carry no content — escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault. · r1
contract/escher-driver-command-span/added · escher-driver command span · added — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/busy · escher-driver command span · busy — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/cause · escher-driver command span · cause — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/changed · escher-driver command span · changed — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/passes · escher-driver command span · passes — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/removed · escher-driver command span · removed — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/settled · escher-driver command span · settled — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-command-span/span · escher-driver command span · command — Every call handed to Session::run opens one INFO span named command at target escher_driver. · r1
contract/escher-driver-command-span/verb · escher-driver command span · verb — A field of the command span, a closed word, bool or count, recorded only where it applies. · r1
contract/escher-driver-tool-call-span/command-child · escher-driver tool_call span · command child — An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child. · r1
contract/escher-driver-tool-call-span/ending · escher-driver tool_call span · ending — A closed word, one of accepted, refused or error. · r1
contract/escher-driver-tool-call-span/span · escher-driver tool_call span · tool_call — One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other. · r1
contract/escher-driver-tool-call-span/tool · escher-driver tool_call span · tool — The verb table's word for the called tool, a closed word. · r1

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/advance-accepted-every-session · advance accepted on every session — The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause. · why: Ratified. · r1
decision/command-span-no-call-content · No call content in the command span — The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value. · why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value. · guard: stand_act_spans · r1
decision/hosted-call-through-session-run · Hosted call runs through the session's run alone — A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone. · why: stand_act_spans pins a part read's command line carrying its verb alone, and the session host answers each call through Session::run with no span of its own · r1
decision/tool-call-span-two-closed-fields · tool_call span with tool and ending — Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word. · why: Ratified as the span's shape. · guard: stand_mcp_tools · r1
contract/cold-agent-verdict/capability-unused · The cold-agent verdict · capability-unused — The reason given when lint_calls, screenshot_calls or advance_calls reads 0. · r1
contract/command-span/added · The driver's command span · added — The length of the returned diff's added list, recorded from an acting verb's outcome. · r1
contract/command-span/busy · The driver's command span · busy — One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one. · r1
contract/command-span/cause · The driver's command span · cause — Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied. · r1
contract/command-span/changed · The driver's command span · changed — The length of the returned diff's changed list, recorded from an acting verb's outcome. · r1
contract/command-span/passes · The driver's command span · passes — Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet. · r1
contract/command-span/removed · The driver's command span · removed — The length of the returned diff's removed list, recorded from an acting verb's outcome. · r1
contract/command-span/settled · The driver's command span · settled — Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions and a part read record verb alone, and a refusal records verb and cause alone. · r1
contract/command-span/span · The driver's command span · span — One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it. · r1
contract/refusal-cause/covered · Refusal cause names · covered — A closed refusal class word recorded in the command span's cause field, returned by the executor. · r1
contract/refusal-cause/disabled · Refusal cause names · disabled — A closed refusal class word recorded in the command span's cause field, returned by the executor. · r1
contract/refusal-cause/fonts-unavailable · Refusal cause names · fonts-unavailable — A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with. · r1
contract/refusal-cause/malformed · Refusal cause names · malformed — A closed refusal class word recorded in the command span's cause field, returned by validation. · r1
contract/refusal-cause/not-found · Refusal cause names · not-found — A closed refusal class word recorded in the command span's cause field, returned by the executor. · r1
contract/refusal-cause/off-screen · Refusal cause names · off-screen — A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen. · r1
contract/refusal-cause/stale · Refusal cause names · stale — A closed refusal class word recorded in the command span's cause field, returned by the executor. · r1
contract/refusal-cause/unknown-verb · Refusal cause names · unknown-verb — A closed refusal class word recorded in the command span's cause field, returned by validation. · r1
contract/tool-call-ending/accepted · The tool_call span's ending · accepted — The tool call was accepted and answered. · r1
contract/tool-call-ending/error · The tool_call span's ending · error — The tool call ended in a session error or a protocol error. · r1
contract/tool-call-ending/refused · The tool_call span's ending · refused — The tool call was refused, its reply one text block with isError true. · r1
contract/tool-call-span/command-child · The MCP tool_call span · command child — An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child. · r1
contract/tool-call-span/ending · The MCP tool_call span · ending — One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error). · r1
contract/tool-call-span/span · The MCP tool_call span · span — One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written. · r1
contract/tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool. · r1
