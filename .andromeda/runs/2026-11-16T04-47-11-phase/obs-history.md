@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — set up escher-telemetry: the stderr-only subscriber with its `RUST_LOG` filter (default `warn`) and the `LogTracer` bridge, the one-line text format, the panic hook that logs and then calls the previous hook (std still prints the raw message), and the allowlist scrub with `error` and `panic.payload` in the redacted set. This chunk captures a step's panic or error records without changing what the sink prints. Any capture layer or hook change must leave the hook chain, the bridge and the scrub sets as they are (rows `contract/escher-telemetry-panic-hook/previous-hook`, `contract/telemetry-scrub-set/error`, `contract/escher-telemetry-events/panic`).

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — added `escher-session` as a second binary that installs the sink, and measured its stderr by level. Trap it records: a zero read from a host that installs no sink says nothing about one that does. The new answer content must be checked for absence on a sink-installing host's log, not only in process.

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — every target outside the allowlist is dropped whole at every level, WARN and ERROR included. Rule for later chunks: a target is admitted only together with a scrub rule for it. This bears on capturing a third-party error or a handler's log from a non-`escher_` target, and the sink must keep dropping it (`decision/sink-target-allowlist`, `decision/escher-target-content-named-set`).

@history_item n=4 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — recorded the settle wait and `Session::act` as silent. Rule: add no span, event or log line to the settle loop or the session step. A per-step capture on the run path must keep that silence.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the span's `cause` field takes a `Cause::name`. Rule: never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. Any new refusal cause or result field for a panic or error ending must follow it (`decision/debug-types-never-logged`, `contract/command-span/cause`).

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` and the executor exist. `Outcome` joined the `Debug`-deriving types that nothing prints, logs or fields. A new `Outcome` field carrying a panic or log must stay unlogged and must not be fielded with `Debug`.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — all eight cause names are returned by code, and the session's record of ids (bounded, no reader outside the crate, no `Debug`) is printed, logged and fielded by nothing (`decision/session-id-record-unprinted`). Tying captured errors to node ids must not expose that record.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (both entries) — the `command` span as built: eight closed-word, bool or count fields, entered before `validate`, with a never-fielded list (any `Outcome`, `Refusal` or `Fault`). The workspace build turns engine `tracing` on, so a test child with the sink at `info` also receives engine events (`decision/engine-tracing-by-invocation`). `stand_act_spans`' re-run children install the sink over an in-memory capture. The closed-span line was added as a record class, PROVISIONAL at the time. This chunk's scrub-held boundary means the span gains no field.

@history_item n=9 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line and ruled that a diff's three sizes may reach a log while its content never does. This is one of the founder's log rulings of 2026-10-07/09 that the scope names as bounding what of a log may enter an answer (`decision/closed-span-lines`, `decision/content-span-fields-never-stored`).

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — what a command prints is its answer under the crossing the founder ratified on 2026-10-10, never a log line (`contract/escher-session-stdout/command-answer`). Standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. The second entry measured typed text in a host's log at `trace` under both builds. These are the 2026-10-10 widening rule and the host-log proof that the new answer field must keep.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the founder ratified the `tool_call` span with exactly two closed fields, `tool` and `ending` (`accepted`, `refused` or `error`), with `command` as its child. A panic or error returned in an answer must map onto the existing endings without adding a field (`decision/tool-call-span-two-closed-fields`, `contract/tool-call-ending/*`).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` and `describe()`, which write no record, and gave the session-error object a fixed `remedy`. The self-description and the MCP listing are answers, never log lines. This chunk extends both to describe the new field (`contract/escher-session-commands/help`, `contract/escher-session-error-object/remedy`).

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — precedent: screen content (a shot's image) leaves only in its answer, and `host_log` and `mcp_log` read 0 of it at `trace` under both builds. The shot added a cause name, closed, and no span field. That is the pattern for content that rides only in an answer (`decision/shot-image-leaves-only-in-answer`).

@history_item n=14 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `busy` was driven on a real line through a `stand_act_spans` fixture, giving six of the eight fields. A client's stderr and the windowed binary were read by build and level with 0 needles. This is a precedent for a fixture app that drives a case the stand tasks lack.

@history_item n=15 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe counts `tool_call` lines and their ` ending="refused" ` and ` ending="error" ` endings from `server.log`. If a caught panic changes how a call ends, the counted endings move.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set went back to eight names, and an accepted `advance` records `verb` and the acting fields with no cause. `advance` is one of this chunk's acting verbs, and its span shape is fixed (`decision/advance-accepted-every-session`).

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the eight cause names and `advance`'s accepted span and `tool_call` shape. A new refusal cause for a panic would touch this ratified closed set.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span. An MCP call after which the recording ended answers `isError` with its `tool_call` ending `error`. The recording directory is `0700` and holds call content at rest. This is the kept-check carrier the new field must ride (`decision/recording-dir-private`, `decision/recording-bounds`).

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder keeps each answer whole (a shot's without `png`) beside its digest. Each replayed step writes its own `command` span. This is the base for P4's choice on whether the new field joins the kept answer and its digest (`contract/kept-check/answer`, `contract/kept-check/answer-digest`, `contract/recording-file/answer`).

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — precedent: the founder's word «Без поля (Рекомендую)» brought new answer content (lint findings) onto every carrier with no new span field, never logged or fielded. This chunk's answer content follows the same rule.

@history_item n=21 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — on a step that waited on a load, `passes` is the settle that went quiet. The load wait opens no span of its own inside `command`. A per-step capture must cover the load wait too without adding a site (`contract/command-span/passes`).

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the live cold-agent child records into an owner-only `record/`, and `advance_calls` counts accepted `tool_call` lines (`decision/cold-agent-record-owner-only`, `contract/cold-agent-verdict/advance-calls`). New answer content lands at rest there.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — the uncaught-JS-error `tracing::error!` in `report_js_error` sits under the engine target `blitz_vibey_script`, behind the `tracing` feature, and the sink redacts its message. This is an existing error-level record that a step's capture may meet, and its text must not cross onto an answer unruled.

@history_item n=24 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — a failed load's error text stops at `ResourceLoadResponse` and the `Resource load failed` line, with `url` and `error` printing `[redacted]`, and no other carrier takes the text (`contract/telemetry-scrub-set/error`, `contract/telemetry-scrub-set/url`). Trap it records: a trait default can hide a fix, so sweep the implementers of any trait extended.

@history_item n=25 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — `dioxus_runtime_probe` reads a node's component and listeners and holds a `hazard` page whose panic site was confirmed by backtrace. Its reading quotes no panic payload or stderr, only coordinates. This is the existing handler-panic reading and its payload discipline, for attributing a panic to a handler.

@history_item n=26 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the mutation buffer's replay re-fires `MutationWriter`'s `trace!` calls, so a step's log stream holds engine trace records. A per-step capture would see them, and they are not handler logs.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the graph answer carries ids, component names, state and handler event names, closed flags and booleans, never a value. It was a boundary widening, now ratified (`contract/command-answer/graphed`, `contract/graph-answer/content`, `decision/graph-answer-never-logged`). The bridge dispatch through `SignalRegistry` was added, and a page with no registered state reaches no registry (`decision/no-state-page-skips-registry`). This is the graph identity this chunk ties a handler through.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` answers with ids, owner ids, verb words and closed cause words, a widening the founder ratified (pattern `rul-izni57qv`), and `actions_needles` reads 0 at `trace`. It is a precedent for owner-id attribution in an answer and for its needle proof (`contract/command-span/actions`).

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — dioxus-native-dom's `reached` record is written in `DioxusEventHandler::handle_event` and holds stable ids and event names only, no `NodeId`, with `Debug` printing its count alone (`decision/reached-record-ids-and-events-only`). The coverage role's endings and the `malformed` refusal were added (`contract/escher-session-coverage/*`). This is a candidate mechanism for attributing a step to its handler, and its silence must hold.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows. All the picked handles date from this conversion, with each row checked against the code.

@history_item n=31 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `decision/exploration-catches-panics` (a caught panic is reported as the closed ending `Panicked` with no payload, and the hook still runs) and `decision/exploration-adds-no-span`. It also widened the coverage role's malformed refusals. This chunk catches a handler panic on the run path itself, and `explore`'s payload-free ending must not widen.

@history_item n=32 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph answer's carried set and the coverage role's reach, endings and line. This chunk ties errors to handlers through that graph identity.

@history_item n=33 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `stand_layout` installs its own recording subscriber for a TRACE capture, now an exception in `decision/checks-silent` and `decision/headless-stand-no-sink`. It also added `decision/layout-read-opens-no-span`. It is the nearest precedent for a check-side capture subscriber, and a new one must be named the same way. The layout read must keep its no-span contract.
