@row_add n=1 row=decision/obs-tier area=1-obs-scope-summary rec=1
name: Observability tier
rule: The project's observability tier is 0.
@row_none n=1 rec=2 type=S
@row_none n=2 rec=3 type=5
@row_none n=3 rec=4 type=5
@row_none n=4 rec=5 type=5
@row_none n=5 rec=6 type=5
@row_none n=6 rec=7 type=4
relied: The escher telemetry crate's formatter scrubs engine and escher targets, drops every other target, and prints one stderr line per event and one per closed span, with a chaining panic hook.
@row_none n=7 rec=8 type=5
@row_none n=8 rec=9 type=5
@row_none n=9 rec=10 type=5
@row_none n=10 rec=11 type=5
@row_none n=11 rec=12 type=5
@row_none n=12 rec=13 type=5
@row_none n=13 rec=14 type=5
@row_none n=14 rec=15 type=5
@row_none n=15 rec=16 type=5
@row_none n=16 rec=17 type=S
@row_none n=17 rec=18 type=S
@row_add n=2 row=decision/telemetry-via-tracing area=2-telemetry-strategy rec=19,20,21
name: Telemetry through the tracing crate
rule: Telemetry is the tracing crate's event and span macros; the engine and upstream crates compile their log sites only with the tracing cargo feature, each call a no-op when the feature is off.
@row_add n=3 row=decision/escher-crates-always-trace area=2-telemetry-strategy rec=19
name: Escher crates take tracing unconditionally
rule: The escher telemetry and driver crates take tracing with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, while their engine features stay off in a package-alone build.
@row_none n=18 rec=22 type=0
what: a note that the engine crate's root comment lists the tracing feature under a TODO to document features
@row_none n=19 rec=23 type=5
@row_none n=20 rec=24 type=5
@row_none n=21 rec=25 type=S
@row_none n=22 rec=26 type=6
@row_none n=23 rec=27 type=6
@row_none n=24 rec=28 type=6
@row_add n=4 row=decision/engine-tracing-by-invocation area=2-telemetry-strategy rec=29
name: Engine tracing set by the build's invocation
rule: A build that resolves the whole workspace turns the engine's tracing feature on while a package-alone build of the test crate or seven_guis leaves it off, so under the workspace build a test child that installs escher's sink at info also receives engine events.
@row_none n=25 rec=30 type=6
@row_none n=26 rec=31 type=6
@row_none n=27 rec=32 type=6
@row_none n=28 rec=33 type=6
@row_none n=29 rec=34 type=6
@row_none n=30 rec=35 type=S
@row_add n=5 row=absence/telemetry-backend area=2-telemetry-strategy rec=36,39
capability: A telemetry backend (Sentry, OpenTelemetry, Prometheus or a metrics exporter)
@row_none n=31 rec=37 type=S
@row_none n=32 rec=38 type=S
@row_none n=33 rec=40 type=S
@row_add n=6 row=decision/sink-installed-first-in-main area=3-observability-harness-contract rec=41,42
name: Telemetry sink installed first in each native main
rule: Each native binary of the seven_guis package installs escher's telemetry sink in its native main before anything boots, reporting a failed install on stderr and continuing.
@row_add n=7 row=contract/telemetry-sink/stream area=3-observability-harness-contract rec=41,42
boundary: escher's telemetry sink
item: output stream
promise: The sink writes uncoloured lines to stderr only, never to stdout.
@row_add n=8 row=contract/telemetry-sink/rust-log area=3-observability-harness-contract rec=41
boundary: escher's telemetry sink
item: RUST_LOG
promise: The sink's filter is read from RUST_LOG, and is warn when RUST_LOG is unset.
@row_add n=9 row=contract/telemetry-sink/log-records area=3-observability-harness-contract rec=41
boundary: escher's telemetry sink
item: log records
promise: Records of the log crate are bridged into the tracing sink.
@row_add n=10 row=decision/driver-library-installs-no-subscriber area=3-observability-harness-contract rec=42,43
name: Driver library installs no subscriber
rule: The driver library, its command line, MCP server and coverage reader included, installs no subscriber and reads no environment variable; only a binary's own main installs the sink.
@row_add n=11 row=contract/escher-session-stdout/command-answer area=3-observability-harness-contract rec=42
boundary: escher-session stdout
item: command answer
promise: Each command's answer, refusal or session-error object is one uncoloured line of JSON on stdout, an answer and never a log line; the host role ends quiet.
@row_add n=12 row=contract/mcp-role-stdout/json-rpc area=3-observability-harness-contract rec=42
boundary: escher-session mcp role stdout
item: JSON-RPC lines
promise: In the mcp role stdout carries newline-delimited JSON-RPC 2.0 protocol lines only, never written by the sink, and the role's end writes nothing on stdout.
@row_add n=13 row=contract/mcp-role-exit-status/0 area=3-observability-harness-contract rec=42
boundary: escher-session mcp role exit status
item: 0
promise: The server ended without error.
@row_add n=14 row=contract/mcp-role-exit-status/3 area=3-observability-harness-contract rec=42
boundary: escher-session mcp role exit status
item: 3
promise: The server ended on an error, or a record directory was refused before serving; the fixed message is on stderr and stdout is empty.
@row_add n=15 row=contract/coverage-role-stdout/coverage-line area=3-observability-harness-contract rec=42
boundary: escher-session coverage role stdout
item: coverage line
promise: The coverage line, holding ids, owner ids, event names and two counts, is written on stdout as an answer and never a log line; a malformed refusal holds nothing of the path.
@row_add n=16 row=decision/headless-stand-no-sink area=3-observability-harness-contract rec=43
name: Headless stand runs with no sink
rule: A headless boot made in process installs no telemetry subscriber and reads no environment variable; a headless boot made by the escher-session binary runs under that binary's sink.
except: The MCP tools check installs its own in-process recording subscriber, and the command-span check's two re-run children install escher's sink over an in-memory capture.
@row_add n=17 row=decision/checks-silent area=3-observability-harness-contract rec=43,45,46,47,48,49,50,51,52,53,54,55
name: Silent checks and shared modules
rule: Checks, their shared modules and the stand-ins they use install no subscriber, read no environment variable and print nothing, their readings going into assertions only, and the code they pin adds no log site, span, field or env read.
guard: silence census
except: A re-executed test child may print its ids or shots to stdout, which the parent reads into its assertions only.
@row_add n=18 row=decision/debug-types-never-logged area=3-observability-harness-contract rec=43,51,54
name: Answer-bearing types never logged
rule: Types that derive Debug and hold ids, accessible names, control values or page text are never logged, fielded or printed with Debug; a command's outcome is written in one place, as the command's answer.
@row_add n=19 row=decision/answers-echo-nothing-typed area=3-observability-harness-contract rec=43
name: Answers echo nothing the caller typed
rule: No answer, refusal, usage line or session error writes back anything the caller typed, and a password's or a file input's value appears only as the fixed mask.
@row_add n=20 row=decision/one-span-per-command area=3-observability-harness-contract rec=43
name: One span per driver command
rule: Each driver command opens exactly one command span, which writes only where a process has installed a sink at info or below.
@row_add n=21 row=decision/session-id-record-unprinted area=3-observability-harness-contract rec=43
name: Session id record kept private
rule: The session's record of the ids its screens have read is bounded, has no reader outside the driver crate and no Debug, and is never printed, logged or fielded.
@row_add n=22 row=contract/command-answer/shot area=3-observability-harness-contract rec=43
boundary: driver command answer
item: shot
promise: A screenshot answer holds width, height and the PNG; over MCP it is an image block followed by the JSON line without the PNG, and is never logged or fielded.
@row_add n=23 row=contract/command-answer/linted area=3-observability-harness-contract rec=43
boundary: driver command answer
item: findings
promise: A lint answer is written as one JSON object with a findings array, kept whole in a recorded check and never logged or fielded.
@row_add n=24 row=contract/command-answer/graphed area=3-observability-harness-contract rec=43
boundary: driver command answer
item: nodes and states
promise: A graph answer is one JSON object of nodes and states holding ids, component names, registered state and handler event names, closed flag words and booleans, never a value.
@row_add n=25 row=decision/log-check-failures-name-kind-count area=3-observability-harness-contract rec=43,49
name: Log check failures name a kind and a count
rule: A failing log check's message names a kind and a count, never a needle or anything the host wrote.
@row_add n=26 row=decision/sink-host-read-at-trace area=3-observability-harness-contract rec=43,47,54
name: Sink-installing host read at trace
rule: A sink-installing host and its mcp role are read at trace under both builds, and must show no id, name or typed-sentinel needle in what they write.
guard: host_log, mcp_log
@row_add n=27 row=decision/empty-stderr-check-unsets-rust-log area=3-observability-harness-contract rec=43
name: Empty-stderr checks unset RUST_LOG
rule: A check that reads an accepted command's empty stderr removes RUST_LOG from the spawned child's environment.
@row_none n=34 rec=44 type=S
@row_add n=28 row=contract/kept-check/arguments area=3-observability-harness-contract rec=47
boundary: kept check file
item: arguments
promise: Each answered call's arguments are written into the kept check, a file at rest, never a log line, field or print.
@row_add n=29 row=contract/kept-check/accepted area=3-observability-harness-contract rec=47
boundary: kept check file
item: accepted
promise: Each answered call records whether it was accepted.
@row_add n=30 row=contract/kept-check/answer-digest area=3-observability-harness-contract rec=47
boundary: kept check file
item: answer digest
promise: Each answered call records its answer's FNV-1a 64 digest.
@row_add n=31 row=contract/kept-check/answer area=3-observability-harness-contract rec=47
boundary: kept check file
item: answer
promise: Each answered call records the answer itself, a screenshot's without the PNG.
