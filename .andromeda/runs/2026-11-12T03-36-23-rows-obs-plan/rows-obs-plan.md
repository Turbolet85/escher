@row_add n=1 row=decision/obs-tier area=1-obs-scope-summary rec=fct-ctlmka2e@1
name: Observability tier
rule: The project's observability tier is 0.
@row_none n=1 rec=fct-kxxqlvml@1 type=S
@row_none n=2 rec=fct-boih5dla@1 type=5
@row_none n=3 rec=fct-bj6twg7o@1 type=5
@row_none n=4 rec=fct-hzn2d43d@1 type=5
@row_none n=5 rec=fct-q4xi73qh@1 type=5
@row_none n=6 rec=fct-w6etu55s@1 type=4
relied: The escher telemetry crate's formatter scrubs engine and escher targets, drops every other target, and prints one stderr line per event and one per closed span, with a chaining panic hook.
@row_none n=7 rec=fct-25zbg7yi@1 type=5
@row_none n=8 rec=fct-juzpej5s@1 type=5
@row_none n=9 rec=fct-peocrtvg@1 type=5
@row_none n=10 rec=fct-lur73a47@1 type=5
@row_none n=11 rec=fct-gxlylff7@1 type=5
@row_none n=12 rec=fct-jojxddul@1 type=5
@row_none n=13 rec=fct-2jfjxpkc@1 type=5
@row_none n=14 rec=fct-rvanvmh2@1 type=5
@row_none n=15 rec=fct-2kl45u3w@1 type=5
@row_none n=16 rec=fct-rjxz3u75@1 type=S
@row_none n=17 rec=fct-2y6lzycr@1 type=S
@row_add n=2 row=decision/telemetry-via-tracing area=2-telemetry-strategy rec=fct-fmm6lpcc@1,fct-oz3cjgm7@1,fct-fkrum74w@1
name: Telemetry through the tracing crate
rule: Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.
@row_add n=3 row=decision/escher-crates-always-trace area=2-telemetry-strategy rec=fct-fmm6lpcc@1
name: Escher crates take tracing unconditionally
rule: The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
@row_none n=18 rec=fct-zlpgv5wu@1 type=0
what: a note that the engine crate's root comment lists the tracing feature under a TODO to document features
@row_none n=19 rec=fct-bnk56wze@1 type=5
@row_none n=20 rec=fct-k5tfpym6@1 type=5
@row_none n=21 rec=fct-da2pqgdx@1 type=S
@row_none n=22 rec=fct-3ggncwzf@1 type=6
@row_none n=23 rec=fct-6bscfeh5@1 type=6
@row_none n=24 rec=fct-byb3pmvg@1 type=6
@row_add n=4 row=decision/engine-tracing-by-invocation area=2-telemetry-strategy rec=fct-upbrcclf@1
name: Engine tracing set by the build's invocation
rule: A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
@row_none n=25 rec=fct-3trbrghu@1 type=6
@row_none n=26 rec=fct-6oskq4op@1 type=6
@row_none n=27 rec=fct-xn6gxfr3@1 type=6
@row_none n=28 rec=fct-tsxuw7xn@1 type=6
@row_none n=29 rec=fct-zdyzbc6s@1 type=6
@row_none n=30 rec=fct-6rhi6wdi@1 type=S
@row_none n=31 rec=fct-xh47hod6@1 type=S
@row_none n=32 rec=fct-w7hmy2un@1 type=S
@row_none n=33 rec=fct-ocdewrnq@1 type=S
@row_add n=5 row=decision/sink-installed-first-in-main area=3-observability-harness-contract rec=fct-cg2a6gz6@1,fct-d7dwo6bi@2
name: Telemetry sink installed first in each native main
rule: Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing.
@row_add n=6 row=contract/telemetry-sink/stream area=3-observability-harness-contract rec=fct-cg2a6gz6@1,fct-d7dwo6bi@2
boundary: escher's telemetry sink
item: output stream
promise: The sink writes uncoloured lines to stderr only, never to stdout.
@row_add n=7 row=contract/telemetry-sink/rust-log area=3-observability-harness-contract rec=fct-cg2a6gz6@1
boundary: escher's telemetry sink
item: RUST_LOG
promise: The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset.
@row_add n=8 row=contract/telemetry-sink/log-records area=3-observability-harness-contract rec=fct-cg2a6gz6@1
boundary: escher's telemetry sink
item: log records
promise: Records of the log crate are bridged into the tracing sink.
@row_add n=9 row=decision/driver-library-installs-no-subscriber area=3-observability-harness-contract rec=fct-d7dwo6bi@2,fct-icom2cq7@5
name: Driver library installs no subscriber
rule: The driver library, its command line, MCP server and coverage reader included, installs no subscriber and reads no environment variable; only a binary's own main installs the sink.
@row_add n=10 row=contract/escher-session-stdout/command-answer area=3-observability-harness-contract rec=fct-d7dwo6bi@2
boundary: escher-session stdout
item: command answer
promise: Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
@row_add n=11 row=contract/mcp-role-stdout/json-rpc area=3-observability-harness-contract rec=fct-d7dwo6bi@2
boundary: escher-session mcp role stdout
item: JSON-RPC lines
promise: In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.
@row_add n=12 row=contract/mcp-role-exit-status/0 area=3-observability-harness-contract rec=fct-d7dwo6bi@2
boundary: escher-session mcp role exit status
item: 0
promise: The server ended without error.
@row_add n=13 row=contract/mcp-role-exit-status/3 area=3-observability-harness-contract rec=fct-d7dwo6bi@2
boundary: escher-session mcp role exit status
item: 3
promise: The server ended on an error, or a record directory was refused before serving; the fixed message is on stderr and stdout is empty.
@row_add n=14 row=contract/coverage-role-stdout/coverage-line area=3-observability-harness-contract rec=fct-d7dwo6bi@2
boundary: escher-session coverage role stdout
item: coverage line
promise: The coverage line, holding ids, owner ids, event names and two counts, is written on stdout as an answer and never a log line; a malformed refusal holds nothing of the path.
@row_add n=15 row=decision/headless-stand-no-sink area=3-observability-harness-contract rec=fct-icom2cq7@5
name: Headless stand runs with no sink
rule: A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink.
except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
@row_add n=16 row=decision/checks-silent area=3-observability-harness-contract rec=fct-icom2cq7@5,fct-nsbvo3cq@1,fct-6bgpiy63@1,fct-abwv7xhd@1,fct-kcfcn3em@1,fct-77ml32lr@1,fct-6pnxbanc@1,fct-mpqa22sf@1,fct-2pnajfbv@1,fct-63agsccw@1,fct-gzhrqf5l@1,fct-qu4fsnfx@1
name: Silent checks and shared modules
rule: Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read.
guard: silence census
except: A re-executed test child may print its ids or shots to stdout, which the parent reads into its assertions only.
@row_add n=17 row=decision/debug-types-never-logged area=3-observability-harness-contract rec=fct-icom2cq7@5,fct-mpqa22sf@1,fct-gzhrqf5l@1
name: Answer-bearing types never logged
rule: Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
@row_add n=18 row=decision/answers-echo-nothing-typed area=3-observability-harness-contract rec=fct-icom2cq7@5
name: Answers echo nothing the caller typed
rule: No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask.
@row_add n=19 row=decision/one-span-per-command area=3-observability-harness-contract rec=fct-icom2cq7@5
name: One span per driver command
rule: Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
@row_add n=20 row=decision/session-id-record-unprinted area=3-observability-harness-contract rec=fct-icom2cq7@5
name: Session id record kept private
rule: The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
@row_add n=21 row=contract/command-answer/shot area=3-observability-harness-contract rec=fct-icom2cq7@5
boundary: driver command answer
item: shot
promise: A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
@row_add n=22 row=contract/command-answer/linted area=3-observability-harness-contract rec=fct-icom2cq7@5
boundary: driver command answer
item: findings
promise: A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
@row_add n=23 row=contract/command-answer/graphed area=3-observability-harness-contract rec=fct-icom2cq7@5
boundary: driver command answer
item: nodes and states
promise: A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
@row_add n=24 row=decision/log-check-failures-name-kind-count area=3-observability-harness-contract rec=fct-icom2cq7@5,fct-77ml32lr@1
name: Log check failures name a kind and a count
rule: A failing log check's message names a kind and a count, never a needle or anything the host wrote.
@row_add n=25 row=decision/sink-host-read-at-trace area=3-observability-harness-contract rec=fct-icom2cq7@5,fct-abwv7xhd@1,fct-gzhrqf5l@1
name: Sink-installing host read at trace
rule: A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write.
guard: host_log, mcp_log
@row_add n=26 row=decision/empty-stderr-check-unsets-rust-log area=3-observability-harness-contract rec=fct-icom2cq7@5
name: Empty-stderr checks unset RUST_LOG
rule: A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
@row_none n=34 rec=fct-4knuudbx@1 type=S
@row_add n=27 row=contract/kept-check/arguments area=3-observability-harness-contract rec=fct-abwv7xhd@1
boundary: kept check file
item: arguments
promise: Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
@row_add n=28 row=contract/kept-check/accepted area=3-observability-harness-contract rec=fct-abwv7xhd@1
boundary: kept check file
item: accepted
promise: Each answered call records whether it was accepted.
@row_add n=29 row=contract/kept-check/answer-digest area=3-observability-harness-contract rec=fct-abwv7xhd@1
boundary: kept check file
item: answer digest
promise: Each answered call records its answer's FNV-1a 64 digest.
@row_add n=30 row=contract/kept-check/answer area=3-observability-harness-contract rec=fct-abwv7xhd@1
boundary: kept check file
item: answer
promise: Each answered call records the answer itself, a screenshot's without the PNG.
@row_add n=31 row=decision/part-answer-never-logged area=3-observability-harness-contract rec=fct-n7fxcwkd@1
name: A part's answer is never logged
rule: A part's answer ({"text":…,"part":n,"parts":m}) is a command's answer, written over MCP as its JSON line, and is never logged or fielded.
@row_none n=35 rec=fct-m4exvb46@1 type=5
@row_none n=36 rec=fct-dm2tjxc6@1 type=5
@row_none n=37 rec=fct-ngkmttm3@1 type=S
@row_none n=38 rec=fct-mxtpiyn3@1 type=4
@row_none n=39 rec=fct-evqaxitl@1 type=4
@row_none n=40 rec=fct-s3r6gxgv@1 type=4
@row_none n=41 rec=fct-gc4pzt6f@1 type=7
@row_none n=42 rec=fct-hapzv436@1 type=7
@row_none n=43 rec=fct-qi776ftm@1 type=7
@row_add n=32 row=contract/script-diagnostics-drain/take-js-errors area=3-observability-harness-contract rec=fct-vhstio6o@1,fct-gvsn33bd@1
boundary: Embedder drain of script diagnostics
item: take_js_errors
promise: Embedders drain JS errors with take_js_errors.
@row_add n=33 row=contract/script-diagnostics-drain/take-messages area=3-observability-harness-contract rec=fct-vhstio6o@1,fct-gvsn33bd@1
boundary: Embedder drain of script diagnostics
item: take_messages
promise: Embedders drain JS messages with take_messages.
@row_add n=34 row=contract/script-diagnostics-drain/error-retention area=3-observability-harness-contract rec=fct-c6zljylw@1,fct-gvsn33bd@1
boundary: Embedder drain of script diagnostics
item: error retention between drains
promise: At most 256 JS errors are retained between drains.
@row_add n=35 row=contract/escher-log-line/service-name area=3-observability-harness-contract rec=fct-c6csl7s2@1
boundary: escher's sink log line
item: service.name
promise: Every line carries service.name, the OTel resource key, read from the binary's own CARGO_PKG_NAME; the name is the package's, so two binaries of one package are not told apart.
@row_add n=36 row=contract/escher-log-line/service-version area=3-observability-harness-contract rec=fct-c6csl7s2@1
boundary: escher's sink log line
item: service.version
promise: Every line carries service.version, the OTel resource key, read from the binary's own CARGO_PKG_VERSION.
@row_add n=37 row=contract/escher-log-line/text-format area=3-observability-harness-contract rec=fct-c6csl7s2@1
boundary: escher's sink log line
item: line format
promise: Each record is written in a one-line text format, not JSON.
@row_add n=38 row=decision/harness-json-via-python-json area=3-observability-harness-contract rec=fct-duatlkfz@1,fct-ah5jnzn5@1
name: Harness JSON lines encoded by python's json module
rule: The agent-run and cold-agent harnesses write JSON lines encoded by python3's stdlib json module embedded in the script, never assembled in bash.
@row_add n=39 row=decision/harness-log-not-telemetry-sink area=3-observability-harness-contract rec=fct-duatlkfz@1,fct-ah5jnzn5@1
name: Harness logs are not telemetry sinks
rule: The agent-run and cold-agent harness logs are harness metadata, not telemetry sinks: the harness itself installs no tracing subscriber, OTel setup or third-party logger.
@row_add n=40 row=decision/agent-run-raw-output-never-printed area=3-observability-harness-contract rec=fct-duatlkfz@1,fct-d4fqxbu5@1
name: Agent-run raw output never printed
rule: The raw merged cargo and libtest output goes to target/agent-run/run.log, truncated at each run's start and never printed.
@row_add n=41 row=decision/cold-agent-sink-on-live-child-only area=3-observability-harness-contract rec=fct-ah5jnzn5@1,fct-h5qg7fkm@1
name: Cold-agent live server child logs at info
rule: The cold-agent pipe sets RUST_LOG=info on the live escher-session mcp server child only, so that child installs escher's own sink and its stderr goes to target/cold-agent/server.log.
@row_add n=42 row=decision/cold-agent-server-log-counted-only area=3-observability-harness-contract rec=fct-ah5jnzn5@1,fct-h5qg7fkm@1,fct-2ugqvc5q@1
name: Cold-agent server log read only to count
rule: The cold-agent pipe reads server.log only to count tool_call span lines, their refused and error endings and the accepted lines of each counted verb (lint, screenshot, advance), never printing or copying it.
@row_add n=43 row=decision/replay-children-uninstrumented area=3-observability-harness-contract rec=fct-ah5jnzn5@1,fct-h5qg7fkm@1
name: Replay children uninstrumented
rule: A judged check is replayed on further escher-session mcp children spawned directly, without --record, with stderr to /dev/null.
why: uninstrumented by design
@row_add n=44 row=decision/cold-agent-state-never-printed area=3-observability-harness-contract rec=fct-ah5jnzn5@1,fct-2ugqvc5q@1
name: Cold-agent state never printed or copied
rule: The cold-agent state files (transcript.jsonl, client.log, server.log, check.json and the record directory) are never printed or copied.
@row_add n=45 row=decision/cold-agent-record-owner-only area=3-observability-harness-contract rec=fct-ah5jnzn5@1,fct-h5qg7fkm@1
name: Cold-agent record directory owner-only
rule: The live session runs mcp --record into target/cold-agent/record/, made owner-only (mkdir -m 700).
@row_none n=44 rec=fct-u6wnwk74@2 type=S
@row_add n=46 row=contract/escher-driver-command-span/span area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: command
promise: Every call handed to Session::run opens one INFO span named command at target escher_driver.
@row_add n=47 row=contract/escher-driver-command-span/verb area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: verb
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=48 row=contract/escher-driver-command-span/settled area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: settled
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=49 row=contract/escher-driver-command-span/busy area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: busy
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=50 row=contract/escher-driver-command-span/passes area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: passes
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=51 row=contract/escher-driver-command-span/added area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: added
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=52 row=contract/escher-driver-command-span/removed area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: removed
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=53 row=contract/escher-driver-command-span/changed area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: changed
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=54 row=contract/escher-driver-command-span/cause area=4-span-trace-coverage rec=fct-yqpbkmod@1
boundary: escher-driver command span
item: cause
promise: A field of the command span, a closed word, bool or count, recorded only where it applies.
@row_add n=55 row=decision/driver-spans-carry-no-content area=4-span-trace-coverage rec=fct-yqpbkmod@1,fct-pzih6tk6@1
name: Driver spans carry no content
rule: escher-driver's spans field only closed words, bools and counts, never the screen text, any argument, a shot byte, a lint finding, a part number, or any Command, Call, ArgValue, Outcome, Refusal or Fault.
@row_add n=56 row=decision/driver-emits-no-event area=4-span-trace-coverage rec=fct-yqpbkmod@1
name: Driver emits no event and installs no subscriber
rule: escher-driver emits no log event and installs no subscriber; its telemetry is its spans alone.
@row_add n=57 row=contract/escher-driver-tool-call-span/span area=4-span-trace-coverage rec=fct-wa4xq2qh@1
boundary: escher-driver tool_call span
item: tool_call
promise: One tool_call span is opened per MCP tools/call request, before its params are read, and held until its reply is written; it has two fields and no other.
@row_add n=58 row=contract/escher-driver-tool-call-span/tool area=4-span-trace-coverage rec=fct-wa4xq2qh@1
boundary: escher-driver tool_call span
item: tool
promise: The verb table's word for the called tool, a closed word.
@row_add n=59 row=contract/escher-driver-tool-call-span/ending area=4-span-trace-coverage rec=fct-wa4xq2qh@1
boundary: escher-driver tool_call span
item: ending
promise: A closed word, one of accepted, refused or error.
@row_add n=60 row=contract/escher-driver-tool-call-span/command-child area=4-span-trace-coverage rec=fct-wa4xq2qh@1
boundary: escher-driver tool_call span
item: command child
promise: An instance verb run through Session::run makes its command span the tool_call's child; the session verbs and help, answered by the server, leave a tool_call with no command child.
@row_none n=45 rec=fct-dfgrfrux@1 type=7
@row_none n=46 rec=fct-oi7dm2qv@1 type=S
@row_none n=47 rec=fct-c3cp6kvf@1 type=S
@row_add n=61 row=overview/5-metric-coverage area=5-metric-coverage rec=fct-asz2jsep@1
text: In-process timings and counters are printed, shown in the browser's in-app FPS overlay or held in process: the frame and phase timings behind the log-times features, blitz-dom's resolve phases, debug_timer, blitz-paint's layer counters, the bench, screenshot and multi_doc examples and the WPT runner's figures; the WPT scores are published to Pages on upstream DioxusLabs/blitz only.
@row_none n=48 rec=fct-edixds7b@1 type=S
@row_add n=62 row=absence/metrics-export area=5-metric-coverage rec=fct-3goqnphf@1
capability: Metrics exporters and metric emission (counters, gauges, histograms)
@row_none n=49 rec=fct-q2l22unm@1 type=S
@row_none n=50 rec=fct-dvpabb7b@1 type=S
@row_add n=63 row=contract/escher-log-line/stream area=6-log-coverage rec=fct-uw6udpfj@1
boundary: escher's sink log line
item: output stream
promise: One line per printed event and one per closed span, on stderr, with no ANSI.
@row_add n=64 row=contract/escher-log-line/event-layout area=6-log-coverage rec=fct-uw6udpfj@1
boundary: escher's sink log line
item: event line layout
promise: An event's line is {RFC 3339 UTC time} {LEVEL} {target} service.name={name} service.version={version} {field}={value}…
@row_add n=65 row=contract/escher-log-line/escaping area=6-log-coverage rec=fct-uw6udpfj@1
boundary: escher's sink log line
item: value escaping
promise: Newlines and carriage returns in values are escaped as \n and \r, and string values are Debug-quoted.
@row_add n=66 row=contract/escher-log-line/bridged-log-record area=6-log-coverage rec=fct-uw6udpfj@1
boundary: escher's sink log line
item: bridged log record
promise: A bridged log record is printed under its log.target, that field itself omitted, and is judged against the allowlist by its log.target.
@row_add n=67 row=contract/escher-log-line/scrub area=6-log-coverage rec=fct-uw6udpfj@1,fct-dsb5vef5@1
boundary: escher's sink log line
item: field scrub
promise: Every field of an event line and every pair of a span line passes the sink's scrub.
@row_add n=68 row=contract/escher-log-line/span-line area=6-log-coverage rec=fct-dsb5vef5@1
boundary: escher's sink log line
item: closed span line
promise: A closed span's line is {time} {LEVEL} {target} service.name=… service.version=… span={the span's name}, then the span's own fields in the order recorded, then message, time.busy and time.idle.
@row_add n=69 row=contract/escher-log-line/span-on-close-only area=6-log-coverage rec=fct-dsb5vef5@1
boundary: escher's sink log line
item: span line timing
promise: A span line is written only when a span whose target the allowlist admits closes, never when it is created, recorded to, entered or exited; an event's line is unchanged inside a span.
@row_add n=70 row=contract/escher-sink-env/rust-log area=6-log-coverage rec=fct-lb45e7zq@1
boundary: escher's sink configuration
item: RUST_LOG
promise: Per-module levels come from RUST_LOG through EnvFilter, defaulting to warn when unset or unparsable.
@row_add n=71 row=contract/agent-run-events/events-jsonl area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: events.jsonl
promise: One JSON object per line on stdout, the same lines appended to target/agent-run/events.jsonl, recreated by boot and removed by cleanup; logs prints it verbatim.
@row_add n=72 row=contract/agent-run-events/boot area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: boot
promise: boot {ts, outcome, cargo_exit, head}, printed and appended.
@row_add n=73 row=contract/agent-run-events/run-start area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: run.start
promise: run.start {ts, selection, files}, printed and appended.
@row_add n=74 row=contract/agent-run-events/test area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: test
promise: test {file, test, outcome}, one per libtest result line, printed and appended.
@row_add n=75 row=contract/agent-run-events/run-end area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: run.end
promise: run.end {ts, selection, passed, failed, ignored, cargo_exit, outcome}, printed and appended.
@row_add n=76 row=contract/agent-run-events/status area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: status
promise: status {booted, boot_ts, head, run}, printed only.
@row_add n=77 row=contract/agent-run-events/cleanup area=6-log-coverage rec=fct-2ttx4skb@1,fct-duatlkfz@1
boundary: agent-run harness events
item: cleanup
promise: cleanup {outcome}, printed only.
@row_add n=78 row=decision/agent-run-no-captured-output area=6-log-coverage rec=fct-d4fqxbu5@1
name: Agent-run events hold no captured output
rule: Nothing between a failures: line and the next test result: line is read, so captured stdout and panic text never reach an agent-run event.
@row_none n=51 rec=fct-yj4a4bua@1 type=S
@row_add n=79 row=contract/cold-agent-events/events-jsonl area=6-log-coverage rec=fct-rjzgnnph@1,fct-ah5jnzn5@1
boundary: cold-agent pipe events
item: events.jsonl
promise: One JSON object per line on stdout, the same lines appended to target/cold-agent/events.jsonl, recreated by run and removed by cleanup; logs prints it.
@row_add n=80 row=contract/cold-agent-events/run-start area=6-log-coverage rec=fct-rjzgnnph@1,fct-ah5jnzn5@1
boundary: cold-agent pipe events
item: run.start
promise: run.start {ts, task}, printed and appended.
@row_add n=81 row=contract/cold-agent-events/run-end area=6-log-coverage rec=fct-rjzgnnph@1,fct-ah5jnzn5@1
boundary: cold-agent pipe events
item: run.end
promise: run.end carrying every verdict field, printed and appended.
@row_add n=82 row=contract/cold-agent-events/status area=6-log-coverage rec=fct-rjzgnnph@1,fct-ah5jnzn5@1
boundary: cold-agent pipe events
item: status
promise: status {the verdict}, or status {outcome: "none"} with exit 3, printed only.
@row_add n=83 row=contract/cold-agent-events/cleanup area=6-log-coverage rec=fct-rjzgnnph@1,fct-ah5jnzn5@1
boundary: cold-agent pipe events
item: cleanup
promise: cleanup {outcome: "done"}, printed only.
@row_add n=84 row=decision/verdict-fields-closed area=6-log-coverage rec=fct-ritxkqi6@1
name: Cold-agent verdict fields closed
rule: Each cold-agent verdict field is a count, a boolean or a closed word, never transcript text, a tool argument, a tool answer, a reply or the check.
@row_add n=85 row=contract/cold-agent-verdict/part-reads area=6-log-coverage rec=fct-ritxkqi6@1
boundary: cold-agent verdict
item: part_reads
promise: Counts the paged snapshot reads.
@row_add n=86 row=contract/cold-agent-verdict/recorded-sessions area=6-log-coverage rec=fct-ritxkqi6@1
boundary: cold-agent verdict
item: recorded_sessions
promise: Reports the live child's kept checks.
@row_add n=87 row=contract/cold-agent-verdict/recording-kept area=6-log-coverage rec=fct-ritxkqi6@1
boundary: cold-agent verdict
item: recording_kept
promise: Reports whether the live child's checks were kept.
@row_add n=88 row=contract/cold-agent-verdict/reasons area=6-log-coverage rec=fct-ritxkqi6@1
boundary: cold-agent verdict
item: reasons
promise: The reasons add capability-unused, recording-not-kept and wrong-calls-over-bar after task-unmet and before the check's.
@row_none n=52 rec=fct-4jwzhjyp@1 type=S
@row_none n=53 rec=fct-qnbwu5oa@1 type=5
@row_add n=89 row=decision/escher-session-fixed-diagnostics area=6-log-coverage rec=fct-75rfqs4p@1
name: escher-session diagnostics are fixed strings
rule: Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
@row_add n=90 row=contract/escher-session-mcp/stdout area=6-log-coverage rec=fct-75rfqs4p@1
boundary: escher-session mcp role
item: stdout
promise: The mcp role writes protocol lines only on stdout.
@row_add n=91 row=contract/escher-telemetry-events/telemetry-installed area=6-log-coverage rec=fct-6pasqyd4@1
boundary: escher-telemetry events
item: telemetry installed
promise: info at target escher_telemetry, message telemetry installed, once per successful init, with no argv, path or URL.
@row_add n=92 row=contract/escher-telemetry-events/panic area=6-log-coverage rec=fct-6pasqyd4@1,fct-rcgwsxfj@1
boundary: escher-telemetry events
item: panic
promise: ERROR at target escher_telemetry::panic, message panic, fields panic.file, panic.line, panic.column and panic.payload (redacted), once per panic.
@row_add n=93 row=contract/escher-driver-span-lines/default-level area=6-log-coverage rec=fct-pzih6tk6@1
boundary: escher-driver span lines
item: default warn level
promise: The driver's command and tool_call span lines print only in a process that installs escher's sink at info or below; at the default warn there is no line.
@row_none n=54 rec=fct-2ebvsswy@1 type=5
@row_none n=55 rec=fct-7w2h2v4h@1 type=S
@row_none n=56 rec=fct-mtwafre2@1 type=7
@row_none n=57 rec=fct-ca3bdqo4@1 type=S
@row_none n=58 rec=fct-jbwdgc47@1 type=S
@row_add n=94 row=decision/wasm-console-panic-hook area=7-error-capture-reporting rec=fct-qrv3mgeb@1,fct-c6q7st3h@1
name: console_error_panic_hook for WASM builds
rule: WASM builds install console_error_panic_hook, a workspace dependency, as their panic hook.
@row_none n=59 rec=fct-upwb3ama@1 type=4
@row_add n=95 row=contract/escher-telemetry-panic-hook/previous-hook area=7-error-capture-reporting rec=fct-rcgwsxfj@1
boundary: escher-telemetry panic hook
item: previous hook
promise: The hook takes the previous hook, logs its ERROR event, then runs the previous hook, so std's default still prints the raw message to stderr and the exit code is unchanged.
@row_add n=96 row=contract/wpt-report/message area=7-error-capture-reporting rec=fct-3fabmvrk@1
boundary: WPT report
item: message
promise: A crashed WPT test's panic message is carried into the report's message field.
@row_none n=60 rec=fct-6j4pbnn5@1 type=4
@row_none n=61 rec=fct-adjhpj3n@1 type=S
@row_none n=62 rec=fct-5uzwnhfa@1 type=4
@row_none n=63 rec=fct-ip3ek2mr@1 type=4
relied: Resource load errors in blitz-dom end in a warn event with an `error` field and are not propagated to callers.
@row_none n=64 rec=fct-ngexhhlb@1 type=4
@row_none n=65 rec=fct-mpa2fp3p@1 type=4
@row_none n=66 rec=fct-loynp5wv@1 type=4
@row_none n=67 rec=fct-6jm2c55w@1 type=4
relied: A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it.
@row_none n=68 rec=fct-x2ssk5sc@1 type=4
@row_none n=69 rec=fct-effx6n7y@1 type=4
@row_add n=97 row=contract/wpt-report/subtest-message area=7-error-capture-reporting rec=fct-7lo7w5qm@1
boundary: The WPT report
item: subtest message
promise: A subtest's error strings are joined with newlines into the report's subtest message.
@row_none n=70 rec=fct-qfc5o7x3@1 type=S
@row_add n=98 row=contract/window-error-event/message area=7-error-capture-reporting rec=fct-bmvmmkgl@1
boundary: The window error event
item: message
promise: The window error event carries the error's message.
@row_add n=99 row=contract/window-error-event/filename area=7-error-capture-reporting rec=fct-bmvmmkgl@1
boundary: The window error event
item: filename
promise: The window error event carries filename as the empty string.
@row_add n=100 row=contract/window-error-event/lineno area=7-error-capture-reporting rec=fct-bmvmmkgl@1
boundary: The window error event
item: lineno
promise: The window error event carries lineno 0.
@row_add n=101 row=contract/window-error-event/colno area=7-error-capture-reporting rec=fct-bmvmmkgl@1
boundary: The window error event
item: colno
promise: The window error event carries colno 0.
@row_add n=102 row=contract/window-error-event/error area=7-error-capture-reporting rec=fct-bmvmmkgl@1
boundary: The window error event
item: error
promise: The window error event carries the thrown error.
@row_add n=103 row=contract/window-onerror/arguments area=7-error-capture-reporting rec=fct-qeqmva2y@1
boundary: window.onerror
item: arguments
promise: window.onerror is called with message, source, lineno, colno and error, in that order.
@row_add n=104 row=contract/script-error-surface/error-handler-exception area=7-error-capture-reporting rec=fct-goyzwrjv@1
boundary: The script error surface
item: exception thrown by an error handler
promise: An exception thrown by an error handler is recorded but fires no further error event.
@row_add n=105 row=contract/script-error-source-labels/timer-callback area=7-error-capture-reporting rec=fct-x4l57aps@1
boundary: Script error source labels in reports
item: timer callback
promise: An error raised in a timer callback is labelled "timer callback" in reports.
@row_add n=106 row=contract/script-error-source-labels/event-listener area=7-error-capture-reporting rec=fct-x4l57aps@1
boundary: Script error source labels in reports
item: event listener
promise: An error raised in an event listener is labelled "event listener" in reports.
@row_add n=107 row=contract/script-error-source-labels/error-event-listener area=7-error-capture-reporting rec=fct-x4l57aps@1
boundary: Script error source labels in reports
item: error event listener
promise: An error raised in an error event listener is labelled "error event listener" in reports.
@row_add n=108 row=contract/script-error-source-labels/timer-microtasks area=7-error-capture-reporting rec=fct-x4l57aps@1
boundary: Script error source labels in reports
item: timer microtasks
promise: An error raised in timer microtasks is labelled "timer microtasks" in reports.
@row_add n=109 row=contract/script-error-source-labels/event-microtasks area=7-error-capture-reporting rec=fct-x4l57aps@1
boundary: Script error source labels in reports
item: event microtasks
promise: An error raised in event microtasks is labelled "event microtasks" in reports.
@row_add n=110 row=decision/no-file-loads-from-non-file-documents area=7-error-capture-reporting rec=fct-7fes5odc@1
name: No file: loads from non-file: documents
rule: A file: load asked by a non-file: document is refused as a typed forbidden fetch error: a script fetch rejects with a TypeError, a module import fails, and a script element's source is skipped and recorded as a script error.
@row_none n=71 rec=fct-5nzgw7ac@1 type=S
@row_none n=72 rec=fct-udl2j2ey@1 type=S
@row_none n=73 rec=fct-wshf7nrf@1 type=S
@row_none n=74 rec=fct-eg6utkqt@1 type=4
@row_none n=75 rec=fct-76cmzljs@1 type=4
@row_none n=76 rec=fct-rde3377b@1 type=4
@row_none n=77 rec=fct-euyiczpe@1 type=4
@row_none n=78 rec=fct-ejbd26xj@1 type=4
@row_none n=79 rec=fct-gmdeubr7@1 type=4
@row_none n=80 rec=fct-tqdmt7tv@1 type=4
@row_none n=81 rec=fct-5ypsm7gy@1 type=4
@row_none n=82 rec=fct-dxerzkok@1 type=4
@row_none n=83 rec=fct-lrj2spd7@1 type=4
relied: A log record from a target outside the sink's allowlist never reaches stderr.
@row_none n=84 rec=fct-x5fp5xws@1 type=S
@row_add n=111 row=decision/scrub-at-sink-not-call-sites area=8-pii-scrubbing-compliance rec=fct-cbcilbst@1
name: Scrubbing at the sink, not at call sites
rule: Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.
@row_add n=112 row=decision/content-span-fields-never-stored area=8-pii-scrubbing-compliance rec=fct-t7poegaj@1
name: Content-named span fields never stored
rule: The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
@row_add n=113 row=decision/scrub-sink-reach area=8-pii-scrubbing-compliance rec=fct-bqrgb4jq@1
name: Reach of the scrubbing sink
rule: The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
@row_add n=114 row=decision/agent-run-log-no-user-content area=8-pii-scrubbing-compliance rec=fct-azbkcd52@1
name: Agent-run harness log carries no user content
rule: The agent-run harness log sits outside the sink and carries no user content: none of its event keys is in the content-named set and no captured test output enters it.
@row_add n=115 row=decision/agent-run-raw-log-unscrubbed area=8-pii-scrubbing-compliance rec=fct-azbkcd52@1
name: Agent-run raw log left unscrubbed
rule: The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/.
why: Unscrubbed by design, like the CI logs.
@row_add n=116 row=decision/cold-agent-log-no-user-content area=8-pii-scrubbing-compliance rec=fct-xl75p7b6@1
name: Cold-agent pipe log carries no user content
rule: The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
@row_add n=117 row=decision/cold-agent-transcript-raw area=8-pii-scrubbing-compliance rec=fct-xl75p7b6@1
name: Cold-agent transcript kept raw
rule: The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/.
why: Carried to the model provider by design.
@row_add n=118 row=decision/cold-agent-kept-checks-at-rest area=8-pii-scrubbing-compliance rec=fct-xl75p7b6@1
name: Cold-agent kept checks held at rest only
rule: The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
@row_none n=85 rec=fct-hsniiont@1 type=8
@row_none n=86 rec=fct-lm2odqex@1 type=7
@row_none n=87 rec=fct-pm7tzhrc@1 type=S
@row_add n=119 row=decision/ci-platform-github-workflows area=9-ci-integration rec=fct-bod5ftj2@1
name: CI on GitHub workflows
rule: CI runs as GitHub workflows under .github/workflows/.
@row_none n=88 rec=fct-5duk2scj@1 type=S
@row_add n=120 row=decision/wpt-publishing-upstream-only area=9-ci-integration rec=fct-5q6m2wwa@1,fct-qltcdnl2@1,fct-bupbhqb3@1
name: WPT results published upstream only
rule: The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork.
@row_add n=121 row=decision/fork-ci-two-artifacts area=9-ci-integration rec=fct-5q6m2wwa@1
name: Fork CI uploads two artifacts
rule: The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
@row_add n=122 row=decision/local-state-never-uploaded area=9-ci-integration rec=fct-4rkn3jky@1,fct-zrlbr4lw@1,fct-3pafqnmo@1,fct-kvfmhgfh@1
name: Local state stays local
rule: The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.
@row_add n=123 row=decision/committed-kept-checks-clean area=9-ci-integration rec=fct-4rkn3jky@1
name: Committed kept checks hold no secrets
rule: The kept checks committed in git hold no typed text, host path or secret.
@row_none n=89 rec=fct-ii27cbwv@1 type=6
@row_add n=124 row=contract/apt-install-diagnostic/attempt area=9-ci-integration rec=fct-6l3zphxt@1
boundary: The package-install diagnostic line
item: attempt
promise: One stderr line per failed attempt names the attempt number, and nothing of a user's.
@row_add n=125 row=contract/apt-install-diagnostic/phase area=9-ci-integration rec=fct-6l3zphxt@1
boundary: The package-install diagnostic line
item: phase
promise: The line names the phase as one of update, install or repair.
@row_add n=126 row=contract/apt-install-diagnostic/exit-status area=9-ci-integration rec=fct-6l3zphxt@1
boundary: The package-install diagnostic line
item: exit status
promise: The line names the failed attempt's exit status.
@row_none n=90 rec=fct-bhta7sqq@1 type=S
@row_none n=91 rec=fct-puswulp4@1 type=S
@row_none n=92 rec=fct-mwf7q5yz@1 type=S
@row_add n=127 row=decision/ci-log-artifact-on-failure area=ci-integration rec=fct-b7ujkv3r@1
name: Per-leg CI log uploaded on failure
rule: Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
@row_add n=128 row=decision/coverage-artifact-on-success area=ci-integration rec=fct-2srac6vk@1
name: Coverage report uploaded on success
rule: The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days.
@row_add n=129 row=decision/agent-run-cleanup-scoped area=ci-integration rec=fct-zrlbr4lw@1
name: Agent-run cleanup touches only its own state
rule: The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/.
@row_add n=130 row=decision/session-state-dir-private area=ci-integration rec=fct-3pafqnmo@1
name: Session state directory private and empty of content
rule: A session's state directory is created 0700 by the host and holds the socket file alone while a session is up and nothing after stop or idle expiry; no log, event or screen content is written into it.
@row_add n=131 row=contract/session-lifecycle/idle-expiry area=ci-integration rec=fct-3pafqnmo@1
boundary: The session lifecycle
item: idle expiry
promise: The host's idle expiry ends a session exactly as stop does.
@row_add n=132 row=decision/recording-dir-private area=ci-integration rec=fct-g4yyfogl@3
name: Recording directory private
rule: A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.
@row_add n=133 row=contract/record-refusals/record-dir-not-private area=ci-integration rec=fct-g4yyfogl@3
boundary: Session recording refusals
item: record-dir-not-private
promise: A recording directory open to its group or others is refused with record-dir-not-private.
@row_add n=134 row=contract/recording-file/check-file area=ci-integration rec=fct-g4yyfogl@3
boundary: The recording file
item: check-{n}.json
promise: Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.
@row_add n=135 row=decision/recording-bounds area=ci-integration rec=fct-g4yyfogl@3
name: Recording bounds
rule: A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording.
@row_add n=136 row=contract/recording-file/typed-text area=ci-integration rec=fct-g4yyfogl@3
boundary: The recording file
item: typed text
promise: Typed text is held unmasked, except a type to a masked or unresolved control, held as masked.
@row_add n=137 row=contract/recording-file/answer area=ci-integration rec=fct-g4yyfogl@3
boundary: The recording file
item: answer
promise: Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
@row_add n=138 row=decision/cold-agent-cleanup-scoped area=ci-integration rec=fct-kvfmhgfh@1
name: Cold-agent cleanup touches only its own state
rule: The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits.
@row_add n=139 row=decision/cold-agent-kept-checks-stay-local area=ci-integration rec=fct-pkacqyww@1,fct-yac65j6e@1
name: Cold-agent kept checks stay local
rule: The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.
@row_add n=140 row=decision/committed-checks-hold-no-private-data area=ci-integration rec=fct-7467zgun@1
name: Committed checks hold no private data
rule: A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step.
why: The repository is public.
@row_none n=93 rec=fct-7au5a6cr@1 type=4
relied: A failed, empty or non-CSS stylesheet is answered as an error, so it leaves the pending critical resources and stops blocking render.
@row_none n=94 rec=fct-omr4vkvp@1 type=4
relied: A net handler that does not override the error method drops the error, the trait's default doing nothing.
@row_add n=141 row=contract/telemetry-scrub-set/url area=error-capture-reporting rec=fct-kkhzj346@1
boundary: escher-telemetry's scrub set
item: url
promise: A `url` field is in the scrub set, so the sink prints its value as `[redacted]`.
@row_add n=142 row=contract/telemetry-scrub-set/error area=error-capture-reporting rec=fct-kkhzj346@1
boundary: escher-telemetry's scrub set
item: error
promise: An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line.
@row_add n=143 row=decision/closed-span-lines area=log-coverage rec=fct-uz2p5hye@1
name: Closed spans as a second record class
rule: The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited.
why: Ratified as a boundary widening of the log record classes.
@row_add n=144 row=contract/span-line/field-order area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: field order
promise: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`.
@row_add n=145 row=contract/span-line/level-and-target area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: level and target
promise: The line's level and target are the span's own.
@row_add n=146 row=contract/span-line/time-busy area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: time.busy
promise: The busy timing is tracing-subscriber's own; no escher code reads a clock for it.
@row_add n=147 row=contract/span-line/time-idle area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: time.idle
promise: The idle timing is tracing-subscriber's own; no escher code reads a clock for it.
@row_add n=148 row=contract/span-line/span area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: span
promise: `span` is a reserved field name holding the span's name.
@row_add n=149 row=contract/span-line/scrub area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: scrub
promise: Every pair of the line, `span` included, passes the scrub under the span's target.
@row_add n=150 row=contract/span-line/value-quoting area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: value quoting
promise: A string value is Debug-quoted (`span="command"`, `message="close"`); a number or a bool is written bare.
@row_add n=151 row=contract/span-line/target-outside-allowlist area=log-coverage rec=fct-uz2p5hye@1
boundary: The closed span's line
item: target outside the allowlist
promise: A span from a target outside the allowlist writes no byte at any point.
@row_add n=152 row=contract/event-line/inside-a-span area=log-coverage rec=fct-uz2p5hye@1
boundary: The event line
item: event inside a span
promise: An event emitted inside a span keeps its line unchanged, gaining no span name and no span field.
@row_add n=153 row=decision/cold-agent-verdict-holds-no-content area=log-coverage rec=fct-iekb43j4@1
name: Cold-agent verdict holds no content
rule: Every field of the cold-agent verdict is a count, a boolean or a closed word, holding no transcript text, tool argument, tool answer, reply or check.
@row_add n=154 row=contract/cold-agent-verdict/reasons-order area=log-coverage rec=fct-iekb43j4@1
boundary: The cold-agent verdict
item: reasons order
promise: Reasons appear in the order `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, `no-check`, `check-failed`, `check-cannot-fail`.
@row_add n=155 row=contract/cold-agent-verdict-reasons/capability-unused area=log-coverage rec=fct-iekb43j4@1
boundary: The cold-agent verdict's reasons
item: capability-unused
promise: Given when a counted verb or a part read went unused.
@row_add n=156 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar area=log-coverage rec=fct-iekb43j4@1
boundary: The cold-agent verdict's reasons
item: wrong-calls-over-bar
promise: Given when `wrong_calls × 100 > 5 × tool_calls`.
@row_add n=157 row=decision/graph-and-actions-not-counted-verbs area=log-coverage rec=fct-zygqqe4l@2
name: Graph and actions not counted for a cold agent
rule: The `graph` and `actions` verbs are not counted verbs and are not required of a cold agent, so neither adds a verdict field or a `capability-unused` reason.
@row_add n=158 row=contract/cold-agent-counted-verbs/lint area=log-coverage rec=fct-nrv3hivl@1
boundary: The cold-agent pipe's counted verbs
item: lint
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
@row_add n=159 row=contract/cold-agent-counted-verbs/screenshot area=log-coverage rec=fct-nrv3hivl@1
boundary: The cold-agent pipe's counted verbs
item: screenshot
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
@row_add n=160 row=contract/cold-agent-counted-verbs/advance area=log-coverage rec=fct-nrv3hivl@1
boundary: The cold-agent pipe's counted verbs
item: advance
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.
@row_add n=161 row=decision/cold-agent-server-log-read-by-span-name area=log-coverage rec=fct-nrv3hivl@1
name: Cold-agent server log read by span name only
rule: The cold-agent pipe reads the server log by span name only and never prints or copies it.
@row_add n=162 row=decision/decoded-screen-text-stays-in-the-match area=log-coverage rec=fct-w6eyzz4k@1
name: Decoded screen text stays in the match
rule: The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.
@row_none n=95 rec=fct-ljzfwjuo@1 type=5
@row_add n=163 row=contract/escher-session-exit-codes/0 area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session exit codes
item: 0
promise: Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.
@row_add n=164 row=contract/escher-session-exit-codes/1 area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session exit codes
item: 1
promise: Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.
@row_add n=165 row=contract/escher-session-exit-codes/2 area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session exit codes
item: 2
promise: Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.
@row_add n=166 row=contract/escher-session-exit-codes/3 area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.
@row_add n=167 row=contract/escher-session-error-object/remedy area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session's error object
item: remedy
promise: The error object carries a fixed `remedy` beside `kind` and `message`.
@row_add n=168 row=contract/escher-session-commands/help area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session commands
item: help
promise: `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1.
@row_add n=169 row=contract/escher-session-commands/mcp area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session commands
item: mcp
promise: `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2.
@row_add n=170 row=contract/escher-session-commands/open-recording-directory area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session commands
item: recording directory open to group or others
promise: `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.
@row_add n=171 row=contract/escher-session-stderr/telemetry-not-installed area=log-coverage rec=fct-3ogdy56e@1
boundary: escher-session's stderr
item: telemetry not installed
promise: The binary's one direct stderr print is `telemetry not installed: …`.
@row_add n=172 row=decision/fixed-diagnostics area=log-coverage rec=fct-3ogdy56e@1
name: Fixed diagnostics
rule: Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
@row_add n=173 row=contract/escher-session-coverage/command-shape area=log-coverage rec=fct-7go4n2qu@1
boundary: escher-session's coverage role
item: command shape
promise: `escher-session coverage <app> <check>...` takes an app the binary boots and one or more paths, no `--session` or `--record` among them; any other shape is usage, status 2.
@row_add n=174 row=contract/escher-session-coverage/accepted area=log-coverage rec=fct-7go4n2qu@1
boundary: escher-session's coverage role
item: accepted
promise: One uncoloured JSON coverage line on stdout, status 0.
@row_add n=175 row=contract/escher-session-coverage/boot-failure area=log-coverage rec=fct-7go4n2qu@1
boundary: escher-session's coverage role
item: boot failure
promise: A boot that fails ends with the session error, status 3.
@row_add n=176 row=contract/escher-session-coverage/malformed area=log-coverage rec=fct-7go4n2qu@1
boundary: escher-session's coverage role
item: malformed
promise: A file that cannot be read, is over 1,048,576 bytes, is no check or is a check of another app is refused `malformed`, status 1, holding nothing of the path or the file.
@row_add n=177 row=contract/command-span/line area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: line
promise: One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.
@row_add n=178 row=contract/command-span/screenshot area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: screenshot
promise: A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.
@row_add n=179 row=contract/command-span/lint area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: lint
promise: A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count.
@row_add n=180 row=contract/command-span/snapshot-part area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: snapshot part read
promise: A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.
@row_add n=181 row=contract/command-span/graph area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: graph
promise: A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name.
@row_add n=182 row=contract/command-span/actions area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: actions
promise: An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.
@row_add n=183 row=contract/host-wire/v3 area=log-coverage rec=fct-gwvelgmp@4
boundary: The escher-session host wire
item: v3
promise: Wire version `v3` carries `hello`, `stop` and `call`.
@row_add n=184 row=contract/host-wire/recording-failed area=log-coverage rec=fct-gwvelgmp@4
boundary: The escher-session host wire
item: recording-failed
promise: A call after which the host's recording ended is answered by the one fixed reply `recording-failed`.
@row_add n=185 row=decision/coverage-line-never-logged area=log-coverage rec=fct-gwvelgmp@4,fct-3v7kxekw@1,fct-7qggry7n@1
name: Coverage line never logged
rule: The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line.
guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
@row_add n=186 row=contract/tool-call-span/line area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's tool_call span
item: line
promise: One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.
@row_add n=187 row=contract/tool-call-span/help area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's tool_call span
item: help
promise: A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.
@row_add n=188 row=contract/command-span/field-count area=log-coverage rec=fct-gwvelgmp@4
boundary: The driver's command span
item: field count
promise: The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.
@row_add n=189 row=decision/capability-statement-updated-by-hand area=log-coverage rec=fct-sich66ga@1
name: Capability statement updated by hand
rule: A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand.
why: The statement's pin reads none of its emitter claims.
@row_none n=96 rec=fct-7dblu2wz@1 type=5
@row_none n=97 rec=fct-yxcanyya@1 type=5
@row_none n=98 rec=fct-zq7so4f4@1 type=5
@row_none n=99 rec=fct-bh4ucznh@1 type=5
@row_none n=100 rec=fct-tfiyidhf@1 type=5
@row_none n=101 rec=fct-sbnh4ne3@1 type=5
@row_none n=102 rec=fct-iay2mfrj@1 type=5
@row_none n=103 rec=fct-4za533k3@1 type=5
@row_none n=104 rec=fct-gxkjmvep@1 type=5
@row_none n=105 rec=fct-x7v6upkc@1 type=4
relied: The mutation buffer's replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.
@row_add n=190 row=contract/graph-answer/shape area=log-coverage rec=fct-5ydei676@1
boundary: The graph answer
item: shape
promise: `{"nodes":[…],"states":[…]}`.
@row_add n=191 row=decision/graph-answer-never-logged area=log-coverage rec=fct-5ydei676@1
name: Graph answer never logged
rule: The graph answer is written as a command answer alone, never logged or fielded.
@row_none n=106 rec=fct-7tp6qj2l@1 type=4
relied: The actions read acts on nothing and changes nothing in the document.
@row_none n=107 rec=fct-v53ms3q6@1 type=4
relied: A page with no registered state reaches no signal registry and dispatches as before.
@row_none n=108 rec=fct-5rpr77we@1 type=4
relied: The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone.
@row_none n=109 rec=fct-xzizr6ea@1 type=5
@row_none n=110 rec=fct-c3qjmbbv@1 type=5
@row_none n=111 rec=fct-ozeelxve@1 type=6
@row_none n=112 rec=fct-4julxjdl@1 type=7
@row_none n=113 rec=fct-acdiytyf@1 type=7
@row_none n=114 rec=fct-zxoieuap@1 type=7
@row_none n=115 rec=fct-x7cwmno3@1 type=7
@row_none n=116 rec=fct-h6shwcuf@1 type=7
@row_none n=117 rec=fct-k3vr6esa@1 type=7
@row_none n=118 rec=fct-4hkolobh@3 type=5
@row_none n=119 rec=fct-onrhzzvo@1 type=7
@row_none n=120 rec=fct-wdv62het@1 type=7
@row_none n=121 rec=fct-vhgizwef@1 type=7
@row_none n=122 rec=fct-jok55ozp@1 type=7
@row_none n=123 rec=fct-2yjtpnwu@1 type=7
@row_none n=124 rec=fct-pynegysb@1 type=7
@row_none n=125 rec=fct-pae5nu2v@1 type=7
@row_none n=126 rec=fct-ajxo3ln2@1 type=7
@row_none n=127 rec=fct-nn47hteb@1 type=7
@row_none n=128 rec=fct-353c26b2@1 type=7
@row_none n=129 rec=fct-5sph2bsu@1 type=5
@row_none n=130 rec=fct-aam23khb@1 type=5
@row_none n=131 rec=fct-sebz7gwy@1 type=5
@row_none n=132 rec=fct-wz6bnufw@1 type=5
@row_none n=133 rec=fct-o5ykgbiv@1 type=5
@row_none n=134 rec=fct-bijrvqkd@1 type=5
@row_none n=135 rec=fct-bcmc3bxz@1 type=5
@row_none n=136 rec=fct-qqnmxie7@1 type=5
@row_none n=137 rec=fct-wy742at6@1 type=5
@row_none n=138 rec=fct-am5wh42y@1 type=5
@row_none n=139 rec=fct-3yyhhgqp@1 type=5
@row_none n=140 rec=fct-r4xf77v5@1 type=5
@row_none n=141 rec=fct-q5nddp2s@1 type=5
@row_add n=192 row=decision/wpt-scores-upstream-only area=metric-coverage rec=fct-wq22aub3@1
name: WPT scores published from upstream only
rule: WPT scores are computed and published to Pages only on the upstream DioxusLabs/blitz repository, the wpt job being repository-guarded.
@row_add n=193 row=absence/metrics-exporters area=metric-coverage rec=fct-lyhr4y7p@1,fct-z7ne4oph@1
capability: Metrics exporters or telemetry backends (OpenTelemetry, Prometheus, Sentry)
@row_add n=194 row=absence/metric-emission area=metric-coverage rec=fct-lzmpn22q@1,fct-ta2pcs4x@1,fct-uvsw5eti@1,fct-wwppfpry@1,fct-ujfm2d3p@1
capability: Metric emission: counters, gauges or histograms
@row_add n=195 row=decision/session-logs-carry-no-page-content area=pii-scrubbing-compliance rec=fct-hlttx6z4@5
name: Session logs carry no page content
rule: The session host's, the mcp role's and every client's stderr carries no element id, accessible name, typed text, recording path or recorded content at any log level, under the package-alone and the workspace build.
guard: host_log, mcp_log
@row_add n=196 row=decision/shot-image-leaves-only-in-answer area=pii-scrubbing-compliance rec=fct-hlttx6z4@5
name: Shot image leaves only in its answer
rule: A shot's image is screen content that leaves only in the command's answer, never on a log line.
guard: host_log, mcp_log
@row_add n=197 row=decision/open-list-shot-carries-labels area=pii-scrubbing-compliance rec=fct-hlttx6z4@5
name: Open list shot carries labels as pixels
rule: A shot of an open list carries a page's option labels and group labels as pixels, the shot's own fields unmoved.
why: a boundary widening, ratified
@row_add n=198 row=contract/graph-answer/content area=pii-scrubbing-compliance rec=fct-lhkfbl2j@1
boundary: the UI graph answer (graph)
item: content
promise: Carries stable ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a state's value; a boundary widening, ratified.
@row_add n=199 row=contract/actions-answer/shape area=pii-scrubbing-compliance rec=fct-gvvwqvxr@1
boundary: the actions answer (actions)
item: shape
promise: Written as {"nodes":[{"id":…,"owner":…,"keyed":…,"verbs":[{"verb":…,"cause":…}]}]}.
@row_add n=200 row=contract/actions-answer/owner area=pii-scrubbing-compliance rec=fct-gvvwqvxr@1
boundary: the actions answer (actions)
item: owner
promise: Left out when the node has no owner.
@row_add n=201 row=contract/actions-answer/cause area=pii-scrubbing-compliance rec=fct-gvvwqvxr@1
boundary: the actions answer (actions)
item: cause
promise: A closed cause word, left out when absent.
@row_add n=202 row=contract/actions-answer/content area=pii-scrubbing-compliance rec=fct-gvvwqvxr@1
boundary: the actions answer (actions)
item: content
promise: Carries ids, owner ids, the table's verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a boundary widening, ratified.
@row_add n=203 row=contract/actions-answer/logging area=pii-scrubbing-compliance rec=fct-gvvwqvxr@1
boundary: the actions answer (actions)
item: logging
promise: The answer is kept whole in a recorded check and is never logged or fielded.
@row_add n=204 row=contract/coverage-line/content area=pii-scrubbing-compliance rec=fct-bmyck4th@1
boundary: the coverage line (escher-session coverage)
item: content
promise: Carries stable ids, owner ids, handler event names and two counts only, never a value or typed text; a boundary widening, ratified.
@row_add n=205 row=contract/coverage-line/logging area=pii-scrubbing-compliance rec=fct-bmyck4th@1
boundary: the coverage line (escher-session coverage)
item: logging
promise: The line is a command answer, never logged or fielded.
@row_add n=206 row=contract/multiple-select-form-entries/chosen-option area=pii-scrubbing-compliance rec=fct-v4kxqyim@1
boundary: multiple select form entries
item: chosen option
promise: On input and change, one (name, FormValue::Text(value)) entry per chosen option.
@row_add n=207 row=contract/snapshot/multiple-select-value area=pii-scrubbing-compliance rec=fct-v4kxqyim@1
boundary: the snapshot
item: value of a multiple select
promise: The first chosen option in tree order.
@row_add n=208 row=decision/select-chosen-set-never-logged area=pii-scrubbing-compliance rec=fct-v4kxqyim@1
name: Select chosen set never logged
rule: A multiple select's chosen set leaves only as form entries, the snapshot's value and the options' tree nodes, never as a log line or field.
@row_add n=209 row=decision/select-size-attribute-read area=pii-scrubbing-compliance rec=fct-hjqxxir3@1
name: Select size attribute read
rule: The size attribute on a multiple select is read as a whole number capped at 65534, carrying no text out and moving no redaction shape, for every document.
why: an engine change for every document and a boundary widening, ratified
@row_add n=210 row=decision/optgroup-label-crossing area=pii-scrubbing-compliance rec=fct-oir2d6z3@1
name: Optgroup label as group name and painted header
rule: An optgroup's non-empty label is its group name in the accessibility tree and is painted as a group header in an open list, leaving as pixels in a shot and on no log line or field.
why: inside the ratified name crossing; a boundary widening, ratified
@row_add n=211 row=contract/snapshot-diff/open-list-row-bounds area=pii-scrubbing-compliance rec=fct-pkczt72m@1
boundary: the snapshot and the diff
item: open list row bounds
promise: An open list's row crosses by its own bounds, differs naming bounds within its closed set, adding no field, key name, node key or carrier.
@row_add n=212 row=contract/snapshot-node/clickable area=pii-scrubbing-compliance rec=fct-5gqyxd3k@1
boundary: the snapshot node
item: clickable
promise: A closed bool, true where an element's current listeners hold click and it is no disabled form control, carrying no page text; written as a node's "clickable" key.
@row_add n=213 row=contract/snapshot-text/clickable area=pii-scrubbing-compliance rec=fct-5gqyxd3k@1
boundary: the snapshot text
item: clickable
promise: The token clickable stands after focused.
@row_add n=214 row=decision/select-list-label-never-logged area=pii-scrubbing-compliance rec=fct-stuzvy46@1
name: Select list row label never logged
rule: A select list row's label holds page text in a public read type, is never logged, and leaves only as a painted pixel and the snapshot's existing name.
@row_add n=215 row=contract/page-events/checkbox-change area=pii-scrubbing-compliance rec=fct-jkqmrdqe@1
boundary: page script events
item: change on a checkbox
promise: The engine always sends change after input with the same value, carrying the value class input carries.
@row_add n=216 row=contract/page-events/radio-change area=pii-scrubbing-compliance rec=fct-jkqmrdqe@1
boundary: page script events
item: change on a radio
promise: The engine sends change after input with the same value only where the radio was not checked before the click.
@row_add n=217 row=contract/page-events/single-change area=pii-scrubbing-compliance rec=fct-jkqmrdqe@1
boundary: page script events
item: change count
promise: A page's script hears one change, from the engine, the script runtime synthesising none.
@row_none n=142 rec=fct-ovjvwkua@1 type=4
relied: Page scripts never see the native search-params parser, which is deleted from globalThis before any page script runs.
@row_add n=218 row=decision/runtime-probe-leaks-no-read-value area=pii-scrubbing-compliance rec=fct-yvbbn3zt@1
name: Runtime probe leaks no read value
rule: The dioxus runtime probe lets no read value leave it: its failure messages name a layout mode, a task or page and a link or index only, and its reading quotes no captured stderr, panic payload, signal value or node id.
@row_add n=219 row=decision/mutation-buffer-memory-only area=pii-scrubbing-compliance rec=fct-myhl4hwq@1
name: Mutation buffer contents stay in memory
rule: The buffered render's mutation buffer holds page content in process memory only from a render to its replay, its Debug printing its length alone, and nothing logs, fields or prints its contents.
@row_add n=220 row=decision/state-registry-holds-no-values area=pii-scrubbing-compliance rec=fct-n6hpgwji@1
name: State registry holds no values
rule: The state registry holds each registered state's name, declarations, write records and dispatch chain, never a state's value; its Debug prints counts alone and it holds no log site, print or env read.
@row_add n=221 row=decision/engine-target-safe-fields area=pii-scrubbing-compliance rec=fct-q64qhjfz@1
name: Engine targets print safe fields only
rule: Under an engine target (blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer, js_console) the sink prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line, every other field as {name}=[redacted].
@row_add n=222 row=decision/escher-target-content-named-set area=pii-scrubbing-compliance rec=fct-q64qhjfz@1
name: Escher targets redact the content-named set
rule: Under an escher_ target the sink prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which are redacted; escher alone, without the underscore, is not admitted.
@row_add n=223 row=decision/closed-span-lines-judged-alike area=pii-scrubbing-compliance rec=fct-q64qhjfz@1
name: Closed span lines judged by the same rule
rule: A closed span's line is judged pair by pair by the target rule, and a span from a target outside the allowlist writes no byte at creation, record, enter, exit or close.
@row_add n=224 row=decision/content-named-span-values-not-stored area=pii-scrubbing-compliance rec=fct-q64qhjfz@1
name: Content-named span values never stored
rule: The value of a content-named span field is never stored, its marker written at print time.
@row_add n=225 row=decision/driver-span-fields-print area=pii-scrubbing-compliance rec=fct-q64qhjfz@1
name: Driver span fields tool and ending print
rule: The tool_call span's fields tool and ending are not in the content-named set and print.
why: both are closed words; the closed-span line was ratified
@row_none n=143 rec=fct-ukvug6pr@1 type=4
relied: A failed load's error text reaches only the Resource load failed line, where the sink redacts url, error and the message.
@row_add n=226 row=decision/cold-agent-events-carry-no-user-content area=pii-scrubbing-compliance rec=fct-tnipivhp@1
name: Cold-agent events carry no user content
rule: No cold-agent pipe log event or verdict key is in the content-named set, and no transcript text, tool argument, tool answer, reply or check enters an event or the verdict.
@row_add n=227 row=decision/cold-agent-artifacts-never-printed area=pii-scrubbing-compliance rec=fct-tnipivhp@1
name: Cold-agent server log and check never printed
rule: The cold-agent server log is counted by span name and never printed or copied, and the check file holding the agent's check is never printed.
@row_add n=228 row=decision/cold-agent-transcript-unscrubbed area=pii-scrubbing-compliance rec=fct-tnipivhp@1
name: Cold-agent transcript unscrubbed
rule: The cold-agent transcript holds raw model output and tool I/O, carried to the model provider, never printed or logged and gitignored under target.
why: unscrubbed by design, the provider crossing ratified
@row_add n=229 row=decision/committed-transcript-copies-masked area=pii-scrubbing-compliance rec=fct-tnipivhp@1
name: Committed transcript copies masked
rule: Each committed copy of a cold-agent transcript masks the session dir and the home prefix.
@row_none n=144 rec=fct-vi77hg33@1 type=7
@row_none n=145 rec=fct-fp6mjt7k@1 type=7
@row_none n=146 rec=fct-melmijj3@1 type=7
@row_none n=147 rec=fct-bh6lx2ix@1 type=7
@row_none n=148 rec=fct-7ojvewix@1 type=7
@row_none n=149 rec=fct-do76ezht@1 type=7
@row_none n=150 rec=fct-sx3exauv@1 type=7
@row_none n=151 rec=fct-mje4xjif@1 type=7
@row_none n=152 rec=fct-gqydbxkn@1 type=7
@row_none n=153 rec=fct-sicgzqzx@1 type=7
@row_none n=154 rec=fct-hwmommmi@1 type=7
@row_none n=155 rec=fct-sgavnz6y@1 type=7
@row_none n=156 rec=fct-uvyy5siu@1 type=7
@row_none n=157 rec=fct-hb6qlpie@1 type=7
@row_none n=158 rec=fct-uhhja7ws@1 type=7
@row_none n=159 rec=fct-jfxkjttv@1 type=7
@row_none n=160 rec=fct-xc6f5zsc@1 type=7
@row_none n=161 rec=fct-gag5zsei@1 type=7
@row_none n=162 rec=fct-j47hr7dn@1 type=7
@row_none n=163 rec=fct-n4m37wsf@1 type=7
@row_add n=230 row=contract/command-span/span area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: span
promise: One span per call handed to the session's run, target escher_driver, name command, level INFO, entered before validation and held until the run returns, so a refusal is inside it.
@row_add n=231 row=contract/command-span/settled area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: settled
promise: Recorded from an acting verb's outcome only; snapshot, screenshot, lint, graph, actions and a part read record verb alone, and a refusal records verb and cause alone.
@row_add n=232 row=contract/command-span/passes area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: passes
promise: Settle's pass count, recorded from an acting verb's outcome when settle answered quiet; on a step that waited on a load, the settle that went quiet.
@row_add n=233 row=contract/command-span/added area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: added
promise: The length of the returned diff's added list, recorded from an acting verb's outcome.
@row_add n=234 row=contract/command-span/removed area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: removed
promise: The length of the returned diff's removed list, recorded from an acting verb's outcome.
@row_add n=235 row=contract/command-span/changed area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: The driver's command span
item: changed
promise: The length of the returned diff's changed list, recorded from an acting verb's outcome.
@row_add n=236 row=contract/refusal-cause/unknown-verb area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: unknown-verb
promise: A closed refusal class word recorded in the command span's cause field, returned by validation.
@row_add n=237 row=contract/refusal-cause/malformed area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: malformed
promise: A closed refusal class word recorded in the command span's cause field, returned by validation.
@row_add n=238 row=contract/refusal-cause/not-found area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: not-found
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=239 row=contract/refusal-cause/stale area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: stale
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=240 row=contract/refusal-cause/disabled area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: disabled
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=241 row=contract/refusal-cause/covered area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: covered
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor.
@row_add n=242 row=contract/refusal-cause/off-screen area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: off-screen
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor; an element screenshot that crops to no pixel is refused off-screen.
@row_add n=243 row=contract/refusal-cause/fonts-unavailable area=span-trace-coverage rec=fct-zuu6oxcq@3
boundary: Refusal cause names
item: fonts-unavailable
promise: A closed refusal class word recorded in the command span's cause field, returned by the executor before anything is painted when the held app has no font it can paint text with.
@row_add n=244 row=decision/command-span-no-call-content area=span-trace-coverage rec=fct-zuu6oxcq@3
name: No call content in the command span
rule: The command span never fields an argument, an id, the screen text, an accessible name, an image or its size, lint findings, graph content, the session label, or any command, call, outcome or refusal value.
why: An argument fielded by its schema name, or a command or outcome fielded with Debug, would print an id, typed text, an accessible name or a control's value.
guard: stand_act_spans
@row_add n=245 row=decision/advance-accepted-every-session area=span-trace-coverage rec=fct-zuu6oxcq@3,fct-mp6exhtv@3
name: advance accepted on every session
rule: The advance verb is accepted on every session and records the acting fields like every acting verb, with no cause.
why: Ratified.
@row_none n=164 rec=fct-jyfscbc7@2 type=4
relied: A hosted part read runs through the session's run alone, so its spans are command spans recording verb alone.
@row_none n=165 rec=fct-gf4hwplk@1 type=7
@row_none n=166 rec=fct-xswdmupu@1 type=7
@row_none n=167 rec=fct-lgkmjtef@1 type=7
@row_add n=246 row=decision/tool-call-span-two-closed-fields area=span-trace-coverage rec=fct-mp6exhtv@3
name: tool_call span with tool and ending
rule: Each MCP tools/call request writes one tool_call span carrying exactly two fields, tool and ending, each a closed word.
why: Ratified as the span's shape.
guard: stand_mcp_tools
@row_add n=247 row=contract/tool-call-span/span area=span-trace-coverage rec=fct-mp6exhtv@3
boundary: The MCP tool_call span
item: span
promise: One span, target escher_driver, name tool_call, level INFO, opened once per MCP tools/call request before its params are read and held until its reply is written.
@row_add n=248 row=contract/tool-call-ending/accepted area=span-trace-coverage rec=fct-mp6exhtv@3
boundary: The tool_call span's ending
item: accepted
promise: The tool call was accepted and answered.
@row_add n=249 row=contract/tool-call-ending/refused area=span-trace-coverage rec=fct-mp6exhtv@3
boundary: The tool_call span's ending
item: refused
promise: The tool call was refused, its reply one text block with isError true.
@row_add n=250 row=contract/tool-call-ending/error area=span-trace-coverage rec=fct-mp6exhtv@3
boundary: The tool_call span's ending
item: error
promise: The tool call ended in a session error or a protocol error.
@row_add n=251 row=contract/tool-call-span/command-child area=span-trace-coverage rec=fct-mp6exhtv@3
boundary: The MCP tool_call span
item: command child
promise: An instance verb run through the session's run makes its command span the tool_call's child; the session verbs start, status, stop and help, answered by the server, leave a tool_call with no command child.
@row_add n=252 row=contract/cold-agent-verdict/capability-unused area=span-trace-coverage rec=fct-bcvmu5pc@1
boundary: The cold-agent verdict
item: capability-unused
promise: The reason given when lint_calls, screenshot_calls or advance_calls reads 0.
@row_add n=253 row=overview/telemetry-strategy area=telemetry-strategy rec=fct-weudw6wt@2
text: Telemetry is the tracing crate's event macros and, in escher-driver, two hand-built spans, command and tool_call. In the engine and upstream crates tracing is compiled in only with the tracing cargo feature. escher-telemetry takes tracing with no feature and always compiles its startup and panic events into the two native binaries, seven_guis_native and escher-session. escher-driver takes tracing ungated, so its spans are written in escher-session through its host, mcp and coverage roles, and by no code path in the windowed seven_guis_native.
@row_add n=254 row=decision/tracing-for-telemetry area=telemetry-strategy rec=fct-weudw6wt@2
name: tracing as the telemetry crate
rule: Telemetry is written with the tracing crate's event macros and span macros.
@row_add n=255 row=decision/engine-tracing-feature-gated area=telemetry-strategy rec=fct-weudw6wt@2
name: Engine tracing behind a cargo feature
rule: In the engine and upstream crates, tracing is compiled in only with the tracing cargo feature.
@row_add n=256 row=decision/escher-crates-tracing-ungated area=telemetry-strategy rec=fct-weudw6wt@2
name: escher crates take tracing ungated
rule: escher-telemetry and escher-driver take tracing with no feature gate, so the startup and panic events and the driver's spans are always compiled into the binaries that use them.
@row_none n=168 rec=fct-46rgllvo@1 type=4
relied: A workspace-wide build turns the engine's tracing feature on, so a test child that installs escher's sink at info also receives engine events.
@row_none n=169 rec=fct-atc72w5q@1 type=7
@row_none n=170 rec=fct-tasmmgp5@1 type=7
@row_none n=171 rec=fct-svifzymf@1 type=7
@row_add n=257 row=contract/sink-resource-keys/service-name area=bootstrap-phases-derive-for-route-setup-project rec=fct-k2hfwj7x@3
boundary: The sink's resource keys
item: service.name
promise: The sink names the service under the OTel resource key service.name.
@row_add n=258 row=contract/sink-resource-keys/service-version area=bootstrap-phases-derive-for-route-setup-project rec=fct-k2hfwj7x@3
boundary: The sink's resource keys
item: service.version
promise: The sink gives the service version under the OTel resource key service.version.
@row_none n=172 rec=fct-h7si4phe@1 type=S
@row_add n=259 row=absence/telemetry-backend area=2-telemetry-strategy rec=fct-nvda2d25@1,fct-jaobq4tl@1,fct-qollkyhe@1,fct-k2hfwj7x@3
capability: A telemetry backend or OTel SDK (Sentry, OpenTelemetry, Prometheus or a metrics exporter)
@row_add n=260 row=decision/sink-target-allowlist area=6-log-coverage rec=fct-uw6udpfj@1,fct-lb45e7zq@1,fct-cbcilbst@1,fct-bqrgb4jq@1,fct-q64qhjfz@1,fct-zofw4lw2@1,fct-hgfacxn5@1
name: Sink target allowlist
rule: escher's sink applies an allowlist of targets in its formatter: an engine target prints only the safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole, not an empty line, at every level and whatever RUST_LOG names.
why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting.
guard: telemetry_drop
except: The upstream apps' fmt::init() stdout subscribers and the WPT runner's env_logger stay unscrubbed.
@row_add n=261 row=contract/agent-run-test-event/file area=6-log-coverage rec=fct-d4fqxbu5@1,fct-q6vts7q4@1
boundary: The agent-run harness test event
item: file
promise: The target binary's stem with its hash stripped, or doc:{crate}.
@row_add n=262 row=contract/cold-agent-verdict/counts-agree area=6-log-coverage rec=fct-ritxkqi6@1,fct-iekb43j4@1
boundary: The cold-agent verdict
item: counts_agree
promise: Ties the server's span lines to the transcript: the tool_call lines equal the escher tool uses, and its refused and error lines equal the escher results marked is_error.
@row_add n=263 row=contract/cold-agent-verdict/wrong-calls area=6-log-coverage rec=fct-ritxkqi6@1,fct-iekb43j4@1,fct-bcvmu5pc@1
boundary: The cold-agent verdict
item: wrong_calls
promise: The sum of refused, error and foreign tool uses and permission denials, the refused and error counts read from the live server child's server.log, deciding the verdict against wrong_call_bar_pct.
@row_add n=264 row=contract/cold-agent-verdict/lint-calls area=6-log-coverage rec=fct-ritxkqi6@1,fct-bcvmu5pc@1
boundary: The cold-agent verdict
item: lint_calls
promise: The count of accepted tool_call span lines of the lint verb in the live server child's server.log.
@row_add n=265 row=contract/cold-agent-verdict/screenshot-calls area=6-log-coverage rec=fct-ritxkqi6@1,fct-bcvmu5pc@1
boundary: The cold-agent verdict
item: screenshot_calls
promise: The count of accepted tool_call span lines of the screenshot verb in the live server child's server.log.
@row_add n=266 row=contract/cold-agent-verdict/advance-calls area=6-log-coverage rec=fct-ritxkqi6@1,fct-bcvmu5pc@1
boundary: The cold-agent verdict
item: advance_calls
promise: The count of accepted tool_call span lines of the advance verb in the live server child's server.log.
@row_add n=267 row=contract/cold-agent-verdict/wrong-call-bar-pct area=6-log-coverage rec=fct-ritxkqi6@1,fct-iekb43j4@1
boundary: The cold-agent verdict
item: wrong_call_bar_pct
promise: The bar on wrong calls, 5; the reason wrong-calls-over-bar is added when wrong_calls × 100 > 5 × tool_calls.
@row_add n=268 row=absence/error-reporting-service area=7-error-capture-reporting rec=fct-zwruciiy@1,fct-lb23aler@1,fct-hqxj2olk@1,fct-hkioledl@1,fct-dih52bkv@1,fct-kic3ctd2@1,fct-dtqujcdx@1
capability: An error-reporting client or error-capture service (such as Sentry)
@row_add n=269 row=contract/command-span/verb area=log-coverage rec=fct-gwvelgmp@4,fct-zuu6oxcq@3
boundary: The driver's command span
item: verb
promise: The verb table's own word for a known verb, run or refused, one of its fourteen words; never the caller's verb text; unrecorded for a verb the table lacks; the three session-level words and the tool-level help appear only beside the cause unknown-verb.
@row_add n=270 row=contract/command-span/cause area=span-trace-coverage rec=fct-zuu6oxcq@3,fct-gwvelgmp@4
boundary: The driver's command span
item: cause
promise: Recorded at one site for every refusal the run returns: one of eight fixed cause names, class words holding nothing a call supplied.
@row_add n=271 row=contract/command-span/busy area=span-trace-coverage rec=fct-zuu6oxcq@3,fct-gwvelgmp@4
boundary: The driver's command span
item: busy
promise: One of three busy-class words, the schema's word for the busy class, recorded from an acting verb's outcome when the outcome carries one.
@row_add n=272 row=contract/tool-call-span/tool area=span-trace-coverage rec=fct-mp6exhtv@3,fct-gwvelgmp@4
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the tool, one of its fourteen; unrecorded for an unknown tool.
@row_add n=273 row=contract/tool-call-span/ending area=span-trace-coverage rec=fct-mp6exhtv@3,fct-gwvelgmp@4
boundary: The MCP tool_call span
item: ending
promise: One of three closed words saying how the call ended: `accepted` · `refused` · `error` (a session error or a protocol error).
@row_add n=274 row=contract/coverage-line/shape area=pii-scrubbing-compliance rec=fct-bmyck4th@1,fct-3v7kxekw@1
boundary: The handler coverage line (escher-session coverage)
item: shape
promise: Written by the crate's hand writer as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, a command line answer never logged or fielded.
@row_add n=275 row=contract/coverage-line/owner area=pii-scrubbing-compliance rec=fct-bmyck4th@1,fct-3v7kxekw@1
boundary: The handler coverage line (escher-session coverage)
item: owner
promise: `owner` is left out when absent.
@row_add n=276 row=decision/no-otel-export area=bootstrap-phases-derive-for-route-setup-project rec=fct-k2hfwj7x@3
name: No OTel export
rule: The product ships no OTel export: no OTel crate, no egress and no OTEL_EXPORTER_OTLP_HEADERS credential path.
why: Ratified; the export transport and the credential path are left undecided.
