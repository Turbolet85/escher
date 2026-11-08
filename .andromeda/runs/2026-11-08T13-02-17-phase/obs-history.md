@history_item n=1 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (the cause names are the owed span field's domain) — made the driver's schema (`validate`, `VERBS`, `Refusal` with `Cause`) silent and set the rule that the span's cause field takes a `Cause::name` only, never a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. This chunk edits `VERBS` / `VerbSpec` and returns per-verb causes, so it must keep using `Cause::name` words and keep the new outcome out of `Debug`.
@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the executor is silent) — `execute`, `Session::run` silent; `Outcome` joined the `Debug`-deriving types nothing prints, logs or fields, since an `Outcome` fielded with `Debug` would print an id, name or value. The new read's `Outcome` variant inherits this rule.
@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (eight cause names returned) — the refusal detection (the `disabled` / `covered` / `off-screen` path now behind `action_point`) was built under the crate's silence constraint and all eight cause names became returned by code. The read's per-verb causes come from this one path and its closed words.
@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built) — fixed the `command` span: INFO at `escher_driver`, eight fields recorded only where they apply, a never-fielded list (any argument, `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal`, `Fault`); `stand_act_spans`' re-run children are the census exception. The new read verb must enter this span with `verb` alone (or `verb` + `cause`) and add no field.
@history_item n=5 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (the closed span's line ratified) — the closed-span line, a boundary widening, ratified by the founder; any new field or record class is again a founder's ruling. The read adds none, so it stays inside this ratified line.
@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — a hosted `call` runs through `Session::run` alone, one closed `command` span per call, and a call refused on the command line or in the client opens no span; a command's answer is one JSON line on stdout, never a log line. Standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. This binds the read on the socket's `call` and the command line, if the founder picks those carriers.
@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log) — `host_log` reads a hosted `type` sentinel, ids and names at 0 at `trace` under both builds. This is the needle reading the new read's `host_log` call extends.
@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the `tool_call` span, ratified) — added the `tool_call` span (`tool`, `ending` only, ratified by the founder as a new log field), with `command` its child by nesting, plus `mcp_log` and `stand_mcp_tools`' in-process recorder. The new read's MCP tool call must open one `tool_call` holding only these two closed words.
@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` in the two spans' closed sets) — a verb joining the table joined both spans' closed sets with no field added, and `describe` was recorded silent. Self-description text is an answer on stdout, never a log line. This is the precedent for the new verb's `describe.rs` / help text.
@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (`screenshot` in the spans' closed sets) — the pattern for adding a verb: the `command` line's word count, instance count and `tool_call` count all move, never-fielded gains the answer's content and size, and an empty crop is refused `off-screen`. The read moves the same counts (thirteen → fourteen, nine → ten instance verbs) and must never field its node list or its size.
@history_item n=11 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the cold-agent pipe log on the escher server) — the pipe counts the live child's `tool_call` / `command` lines in `server.log` and is a reader, not an emitter. A new verb reaches this reader through its `tool_call` line only.
@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes) — `time-unavailable` was retired, leaving eight cause names (six returned by the executor). The read's per-verb refusal causes must be drawn from these eight.
@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, one-virtual-clock) — the founder ratified the eight cause names, `covered` and `off-screen` among them. The read's cause words rest on this ratified set, and adding a new cause word would need a new ruling.
@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (recording opens no span and logs nothing) — the recording module records every call with no span of its own. A call after the recording ended ends `tool_call` `error` with `tool` / `ending` alone. The new read recorded in a kept check goes through this path.
@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer at rest, the committed checks) — the answer is kept whole as `answer`, masked as on every carrier, and replay adds no span. The two committed Counter checks are read by `stand_replay` and the freshness check (ratified widening). Changing the verb table or the kept-check form must leave both replaying and free of typed text.
@history_item n=16 marker=2026-10-19-test-splits
**2026-10-19-test-splits** (the log checks' shared fragment) — seven_guis' `tests/common/mod.rs` holds `Needles`, `stand_needles` and `recorded_needles` for `host_log` / `mcp_log`, and its assertion messages carry a kind and a count only. The read's needle tests extend this module, and their messages must stay needle-free.
@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — the founder's F2 «no new span field» with F1 a new verb: `Session::lint` reached inside the call's `command` span, opening none of its own, recording `verb` alone, with its findings and count never fielded. This is the direct no-field precedent for a read verb.
@history_item n=18 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the PROVISIONAL discharge) — the lint's «no new field» was dropped from PROVISIONAL on the founder's own word, so the no-new-field rule for read verbs is founder-ratified.
@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (a part read's span and its silence) — a read records `verb` alone, its number, count and size never fielded. `host_log` / `mcp_log` drive one part read at `trace`, and `stand_act_spans` gained one call per mode. The read follows the same per-verb span and needle pattern.
@history_item n=20 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (the paged part and the recording clients read) — `stand_act_spans` went to 36 calls per mode, with ids and button text as needles, and `host_log` reads a recording session's clients at `trace`. This is a call count and needle set the new read's call extends.
@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the statement restates the driver's spans) — `docs/capabilities.md` restates the emitter's reach, and its pin reads none of it, so any span-set change is made by hand. The kept check's at-rest list extends with each new answer type, which is a trap for this chunk's `docs/capabilities.md` edit.
@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the cold-agent pipe's verdict fields) — counted verbs (`lint` · `screenshot` · `advance`) and their `capability-unused` reason, read from `server.log`'s accepted `tool_call` lines. A new verb is uncounted unless added here, and `tool_call` keeps two fields.
@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host-sink part read measured) — `host_log`'s fourth test runs `serve cells` at `trace` and counts the host's closed `command` span lines, with 0 id and 0 name needles. This is the hosted-read precedent the new read's host carriage follows.
@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list adds no log site or span) — `click` / `type` / `press` help and `covered`'s meaning are fixed `'static` text that reaches no field, kept outside the widening under the 2026-10-12 rule. `command` keeps its eight fields, and escher-driver keeps 2 `info_span!`. Any target, refusal or help text `VerbSpec` gains falls under this rule.
@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the harness contract takes the graph verb and its never-logged answer) — `graph` became the ninth instance verb under one `command` span. `Outcome::Graphed` is written by `to_json` alone, never logged or fielded, and `session_common` readers went to thirty-three. The new read follows as the next verb, and its `Outcome` gets the same never-logged clause and reader count.
@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set, ratified) — the `graph` answer's ids, names and closed words were added at rest to the Recording directory row and to the §8 boundary widening. The confirm that the proposals lie inside the founder's answers is PROVISIONAL (delegate the overseer). The new read's answer is a like widening (the founder's fork), and it rests on a still-PROVISIONAL graph mark.
@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe's verbs take graph) — the pipe's `VERBS` hold thirteen words, and the fake server answers `graph` empty. It is uncounted, adding no verdict field or `capability-unused` reason. A new verb follows the same pattern in `scripts/cold-agent.sh` and `.github/scripts/test_cold_agent.py`.
@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's command and tool_call lines) — the verb table went 12 → 13 and instance verbs 8 → 9. An accepted `graph` records `verb` alone and an argument is refused `malformed`. `host_log` / `mcp_log` each make one call at `trace` with `graph_needles` at 0, and `stand_mcp_tools` reads one `command` child. This is the direct precedent for every count and reading this chunk moves.
@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's graph modules and bridge dispatch add no log site) — `graph`, `signal_registry` and `stand_graph` are silent at a census reading of 0. The read reads these modules, so any reader it adds there joins this group's census.
@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading carries no page text) — `NodeState.clickable` is a closed bool (current listeners hold `click`, not a disabled control). This is one source of the read's actionable set, and it must stay text-free.
@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the state registry holds no state value) — `SignalRegistry` holds names, declarations and the dispatch chain, never a value, and its `Debug` prints counts alone. If the read draws handlers or owners from the graph's registry, its answer must keep this no-value line.
