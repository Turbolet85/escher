@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — named `seven_guis_native` the sink installer and recorded that the headless stand `seven_guis::stand` and its in-process checks install no subscriber, read no env var and print nothing. This is the origin of `decision/headless-stand-no-sink`. An isolated component mounted in process through the stand's boot path inherits that rule: no sink, no env read and no print in the new mount or its check.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — scoped "a headless boot has no escher sink" to a boot made in process, with a boot made by `escher-session` running under that binary's sink. Its trap: a zero read from a host that installs no sink says nothing about one that does. If the isolated mount gets a carrier (a verb or `start` over the host or MCP), it has to be read through the sink-installing host as well, not only in process.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — extended the headless-stand silence census to the checks' shared module (`tests/blitz-tests/tests/common/mod.rs`). This is the reason `decision/checks-silent` covers "their shared modules". A new isolation check and any helper it adds to `common`, `session_common` or `fixtures` falls inside the census.

@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — made the one `println!` exception: a re-executed child prints ids to stdout and its parent reads them into assertions only. This is the precedent behind the re-run-child clause of `decision/checks-silent` if the isolation check compares ids across a fresh process.

@history_item n=5 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — widened the `decision/checks-silent` exception to "ids or digests" for a re-run child, which covers the seeded-digest child. The isolated mount boots with the run's default seed, so any seed or digest comparison done through a child stays inside this exception.

@history_item n=6 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — set the sink's three outcomes (engine targets scrubbed, `escher_` targets printed with content-named fields redacted, every other target dropped). Its rule for later chunks: a target is admitted only together with a scrub rule for it. Any new emitter on an isolation carrier is bound by this; the scope forbids a new log field or span content anyway.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — standing rule: the span's cause field takes a `Cause` name, and a `Command`, `Call` or `ArgValue` is never fielded with `Debug`, nor an argument by its schema name. If props cross as an argument of a verb or `start`, the props value must never be fielded or logged (`decision/debug-types-never-logged`, `decision/command-span-no-call-content`).

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — added `Outcome` to the `Debug`-deriving types that nothing prints, logs or fields, and kept the executor silent. A props-validation refusal or an isolated-mount outcome falls under the same rule: written in one place as the command's answer.

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — recorded the session's record of ids as bounded, with no reader outside the crate, no `Debug`, and never printed, logged or fielded (`decision/session-id-record-unprinted`). An isolated mount whose ids are anchored at the component must keep that record private too.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — built the `command` span (target `escher_driver`, INFO, entered before validation, eight fields recorded only where they apply) and the list of what is never fielded. A props refusal over a carrier lands in this span as `verb` and `cause` alone. The span keeps its eight fields, as the scope's "no new log field or span content" requires.

@history_item n=11 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line (a second record class). That ratified line is the only log class an isolation verb may reach. Any new record class would be a widening needing his word.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the command line writes answers on stdout, never as log lines (`contract/escher-session-stdout/command-answer`). Its standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. This directly binds given props crossing a carrier.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span (two closed fields, `tool` and `ending`), ratified by the founder as a boundary widening (a new log field). An isolation MCP tool would add a word to `tool`'s closed set and no field. The founder's-word path for widenings is the precedent for the scope's fork.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined both spans' closed sets with no field added. The self-description and remedies are an answer, never a log line. This is a precedent for how a new verb or tool word enters the verb table and `contract/command-span/verb` / `contract/tool-call-span/tool`.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `fonts-unavailable` cause, returned before anything is painted. The isolated mount boots with the stand's bundled font, and `contract/refusal-cause/fonts-unavailable` applies if the mount is reached by a verb.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — a new instance verb on the founder's own word, F2 «Без поля» (no new span field). This is the precedent for a verb added by the founder's fork answer, recording `verb` alone and adding no field.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — set the cause names to eight (`time-unavailable` retired) and recorded the delegate-answered facts as PROVISIONAL for the founder's batch. This is the cause domain behind `contract/command-span/cause`. A props refusal must reuse a fixed cause (likely `malformed`, per `contract/refusal-cause/malformed`), never add one silently.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — precedent for discharging delegate/PROVISIONAL marks on the founder's own word, the clause text unchanged. This chunk's wrap lifts the delegate marks on the 24 widening operations of the last two wraps the same way.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing, and the recording directory is 0700 with call content at rest (`decision/recording-dir-private`), ratified by the founder. An isolation carrier recorded in a kept check, with its props among the arguments, would rest there and nowhere else (`contract/kept-check/arguments`).

@history_item n=20 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder keeps the answer whole (`contract/kept-check/answer`, `answer-digest`, `accepted`). Each replayed step writes its own `command` span. A kept isolated-mount step would be replayed and spanned the same way.

@history_item n=21 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — `host_log` and `mcp_log`'s shared module states needle helpers whose assertion messages name a kind and a count, never a needle (`decision/log-check-failures-name-kind-count`). If the isolation carrier gets a host or MCP log reading, its needles follow this form.

@history_item n=22 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — the agent-run harness log is JSON lines written by python3's `json` and kept local only. This is the origin of `decision/harness-json-via-python-json` and `decision/local-state-never-uploaded`, which bind any edit to the harness scripts.

@history_item n=23 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added the cold-agent pipe log, its python-json events, its scrub position (outside the sink, no content-named key) and the raw transcript (gitignored, never printed). These are the rows (`decision/cold-agent-log-no-user-content`, `decision/cold-agent-transcript-raw`) that bind the `scripts/cold-agent.sh` edit adding `layout`.

@history_item n=24 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — moved the pipe onto `escher-session mcp` with `RUST_LOG=info` on the live child only (`decision/cold-agent-sink-on-live-child-only`). `server.log` is counted by span name only. Replay children write stderr to `/dev/null`. The transcript's carriage of screens is a founder-ratified widening. Adding `layout` to the pipe makes a layout answer reachable by the model provider, the price the founder accepted.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe's verdict fields and live child's recording) — the counted verbs `lint`, `screenshot` and `advance`, the verdict fields, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, and the owner-only `record/` directory. These are the contracts the `layout` pipe edit must leave unchanged, since `layout` is to be an uncounted verb like `graph` and `actions`. The live run stays «Cold-agent bar»'s.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs, the served stand's silence) — `main` hands the stand's six slugs and boots (`boot` / `boot_timer` / `boot_cells` / `boot_load` / `options`) to `command_line`, and the `start` tool's `enum` follows the program's fixed text. The served stand's boots are recorded silent. An isolated mount that adds a boot or extends `start` touches exactly this surface and its silence census.

@history_item n=27 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits by hand, and its pin reads none of them. A new verb or tool word for the isolated mount (or `layout` joining the pipe) has to be reconciled there by hand.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — a verb's help and meaning are the program's own `'static` text, outside the widening under the rule of 2026-10-12, and add no log site or span field (`contract/command-span/field-count`). A new carrier's help text falls under the same reading.

@history_item n=29 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — the probe read each element's rendering component and signals through public dioxus-core API, kept every read value inside the test, and recorded its silence. This is the precedent for reading a component across the component boundary in a test, and for the isolation check's failure messages naming no component-to-node tie or value.

@history_item n=30 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `DioxusDocument::initial_build` renders through the silent, process-only `MutationBuffer`. The boundary-marks fallback test carried here concerns a document built around an already-built vdom, so the isolated mount's build path runs through this buffer and the test must stay silent.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the harness contract takes the graph verb; the graph answer's carried set) — `Outcome::Graphed` is never logged (`decision/graph-answer-never-logged`), and the graph answer carries ids, component names, state and handler names, closed flags and booleans (`contract/graph-answer/content`, `contract/command-answer/graphed`). The isolated mount is keyed by the graph's component names, so this is the boundary the scope crosses. The carried set was PROVISIONAL (delegate) until ratified.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe's verbs take graph; the graph verb's command and tool_call lines; the graph modules add no log site; the clickable reading) — `graph` joined both `VERBS` lists uncounted, with the fake server answering it. The `command` and `tool_call` words grew by one, and `host_log` / `mcp_log` were read at 0 needles. dioxus-native-dom's `graph` and `signal_registry` were recorded silent, and `clickable` is a closed bool. This is the exact precedent for `layout` joining the pipe and for any graph-side change the isolated mount needs.

@history_item n=33 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the cold-agent pipe's VERBS fourteen, actions uncounted; actions joins the spans; the actions answer's carried set) — `actions` joined `scripts/cold-agent.sh` and `.github/scripts/test_cold_agent.py` uncounted, adding no verdict field or `capability-unused` reason (`decision/graph-and-actions-not-counted-verbs`). Its answer's carried set was ratified as a widening. This is the template for the `layout` pipe edit: fourteen to fifteen words, the isolation check matching the MCP server's tools again.

@history_item n=34 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage` role and its edits were confirmed by the overseer as delegate (PROVISIONAL), and `session_common` readers moved 34 to 35. This is a precedent for delegate-marked widening records and for counting a new `session_common` reader when the isolation check is added. The coverage line is never logged (`decision/coverage-line-never-logged`).

@history_item n=35 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — graph carried set; coverage role, endings, command span reach, coverage line) — each PROVISIONAL (delegate) mark was lifted by a sidecar entry "was PROVISIONAL … now ratified, text stands as written" on the founder's own word. This is the form this chunk's wrap should follow when it lifts the delegate marks on the 24 widening operations of «Error field in the answer» and «Layout on the carriers».

@history_item n=36 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — widened the `decision/checks-silent` and `decision/headless-stand-no-sink` exceptions for `stand_layout`'s own recorder, and added `decision/layout-read-opens-no-span`. If the isolation check captures at TRACE or installs a recorder, it needs a named exception the same way, or it must stay silent.

@history_item n=37 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — the driver sets escher-telemetry's scoped capture around an acting step. `decision/headless-stand-no-sink`, `decision/driver-library-installs-no-subscriber` and `decision/driver-emits-no-event` were reworded to name the capture as the one subscriber, and it prints nothing. An isolated mount's acting steps run under the same capture.

@history_item n=38 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — added the error-text founder check's sink-in-process exception (`decision/headless-stand-no-sink`, `decision/checks-silent`) and the chaining panic hook to `decision/driver-library-installs-no-subscriber`. It gave `decision/answers-echo-nothing-typed` its caused-item exception. This is the chunk whose Deviations 7 dropped the `boundary.rs` fallback test the scope now carries (cry-c4emakc4).

@history_item n=39 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — opened the `error` field in a caused item (`contract/telemetry-scrub-set/error`, `decision/scrub-by-field-name`, `decision/answers-echo-nothing-typed`) on the operator-relayed founder word. These are among the 9 widening operations whose delegate marks this chunk's wrap lifts. The security-plan `no-attribute-text` exception this chunk lands restates the same crossing.

@history_item n=40 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — moved the verb table and tools to fifteen (`contract/command-span/verb`, `contract/tool-call-span/tool`), added `layout` to `contract/command-span/settled`'s verb-alone list and `contract/command-span/layout`, and added `decision/layout-read-opens-no-span` inside the command span. It extended `decision/sink-host-read-at-trace`'s needles with the layout-answer keys and added `contract/layout-answer/content` and `logging`. These are the 15 widening operations whose delegate marks this chunk's wrap lifts. `layout` joining the cold-agent pipe rests on these rows.

@history_item n=41 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — established escher-telemetry's `telemetry installed` event, the scrub set (with `error` redacted) and the chaining panic hook. These are the base of `contract/escher-telemetry-events/telemetry-installed` and `contract/telemetry-scrub-set/error`, which the isolated mount must leave unmoved, since the scope adds no log field.
