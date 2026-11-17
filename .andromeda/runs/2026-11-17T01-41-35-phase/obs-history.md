@history_item n=1 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (4. Span / Trace Coverage) — added `+ decision/layout-read-opens-no-span`: the in-process layout read opens no span, emits no event and writes no escher_ line at TRACE (guard: stand_layout's the_read_writes_no_escher_line_at_trace). This chunk runs that read through `Session::run`, so it now sits inside a `command` span. The row must be rewritten to say the read itself still opens no span of its own while the `layout` verb's call writes one command line. The TRACE guard must stay green either way.

@history_item n=2 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (3. Observability Harness Contract) — changed `~ decision/checks-silent` and `~ decision/headless-stand-no-sink` so that stand_layout installs its own in-process recording subscriber for its TRACE capture. That is the named exception this chunk's layout checks rest on. Any new layout check that installs a recorder or sink has to be named in these exceptions as well.

@history_item n=3 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb inside the command span, no field) — the closest precedent. An in-process read (`Session::lint`, «no verb, never handed to `Session::run`») became an instance verb on every carrier. It was then reached inside its call's `command` span, opening none of its own. An accepted call records `verb` alone and its findings and their count are never fielded. The `tool_call` `tool` set and the `command` verb table each grew by one word. `host_log` and `mcp_log` each drive one call with 0 needles, and `stand_act_spans` reads its lines. No span field was added, and that was the founder's F2 «Без поля». `layout` follows the same shape: no count, id or reading of the answer is fielded.

@history_item n=4 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the visual lint recorded silent and spanless) — the in-process read's silence census and its observed-absent entry were later rewritten when the read became a verb (n=3). `Finding` and `Reason` derive Debug but are never logged or fielded. This is the same treatment the layout explanation's types need under `decision/debug-types-never-logged`.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions joins the command span's verbs and the tool_call span's tools) — the last verb-count move. The `command` line's verb table went from thirteen to fourteen words and from nine to ten instance-level words. `tool_call`'s `tool` became one of fourteen. An accepted `actions` call records `verb` alone and parents exactly one `command` child (`stand_mcp_tools`, `stand_act_spans` one call per mode). An argument is refused `malformed`. This chunk moves the same rows to fifteen and eleven: `contract/command-span/verb`, `contract/tool-call-span/tool`, `contract/escher-driver-tool-call-span/tool` and `contract/command-span/actions`.

@history_item n=6 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set, at rest and in the PII boundary) — created `contract/actions-answer/*`, a ratified boundary widening that carries ids, owner ids, the table's verb words, closed cause words and booleans only. The Recording directory row's at-rest list also gained the actions answer. This chunk gives every listed node the verb word `layout`, which changes the content of `contract/actions-answer/content` and `shape` but stays inside the table's verb words. The recording row's at-rest list needs the layout answer's ids, closed words and numbers added, as graph and actions were.

@history_item n=7 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site; the actions verb read at 0 needles; the cold-agent pipe's VERBS fourteen) — set four precedents for a fifteenth verb. First, `Session::actions` is reached inside the call's `command` span and opens none of its own. Second, `actions_needles` exists in the seven_guis shared module, and `host_log` and `mcp_log` each make one call reading 0 needles at `trace` under both builds. Third, `stand_act_spans`' calls per mode went from 36 to 38. Fourth, the cold-agent pipe's `VERBS` (`scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`) went from thirteen to fourteen words, uncounted, with no verdict field or `capability-unused` reason. Expect the same moves for `layout`, including fourteen to fifteen in the pipe's VERBS.

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's command and tool_call lines; the harness contract takes the graph verb; the cold-agent pipe's verbs take graph) — the verb table went from 12 to 13 words and the instance verbs from 8 to 9. A graph line records `verb` alone and holds no node or id. `graph_needles` exists for the `host_log` and `mcp_log` calls. `Outcome::Graphed` joins the never-logged answers, written only by `to_json` and kept whole in a recorded check. The fake cold-agent server answers the new verb. This is the template for `Outcome`'s layout writing under `decision/graph-answer-never-logged`-style rows.

@history_item n=9 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set; the snapshot's clickable reading) — added the at-rest set to the Recording directory row, and a §8 carried-set row for a ratified widening. The `contract/snapshot-node/clickable` and `contract/snapshot-text/clickable` rows say a closed bool carries no page text. A new answer's carried set on the carriers is a boundary widening needing the founder's word. Here that word is the ruled «Глагол `layout --id`».

@history_item n=10 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer's carried set) — a carried set confirmed only by a delegate stays PROVISIONAL until the founder's own word. With the founder away tonight, any layout carried-set row confirmed by a delegate is written PROVISIONAL.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` joined `tool_call`'s `tool` and `command`'s `verb` closed sets with no field added. A `help` call leaves a `tool_call` with no `command` child, and `describe()` writes no record. This chunk's `help` description of `layout` is fixed program text that adds no log site, per `contract/tool-call-span/help` and `contract/command-span/field-count`.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a new instance verb joined both spans' closed sets. Its accepted call records `verb` alone, a refused one `verb` and `cause`, and the verb's payload is never fielded. The `command` line's word counts moved, and `stand_act_spans` and `host_log`/`mcp_log` read 0 needles. This is the pattern for a verb that adds no cause.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set `decision/hosted-call-through-session-run`: a part read records `verb` alone and never its numbers. `stand_act_spans` went from 34 to 35 calls per mode. The layout answer's numbers likewise must never be fielded (`contract/command-span/settled`: non-acting verbs record verb alone).

@history_item n=14 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — standing rule: the span's cause field takes a `Cause` name. Never field a `Command`, `Call` or `ArgValue` with Debug, nor an argument by its schema name. An `id` fielded by name would print an id. This applies to `layout --id` (`decision/command-span-no-call-content`).

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joined the Debug-deriving types that nothing prints, logs or fields. Fielded with Debug, an `Outcome` would print an id, name or value. The layout `Outcome` variant falls under the same rule (`decision/debug-types-never-logged`).

@history_item n=16 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the session's record of ids (bounded, no reader outside the crate, no Debug) is printed, logged and fielded by nothing (`decision/session-id-record-unprinted`). The refusal-cause domain is returned by code. The layout verb's `stale` / `not-found` refusals reuse these causes and must not expose the record.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set became eight names, ratified at 2026-10-18-engine-debts. The scope adds no cause, so the eight cause names in `contract/command-span/cause` and `contract/command-span/line` stand unchanged.

@history_item n=18 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the driver's command span as built; the closed span's line) — created the `command` span: eight fields, entered before `validate`, and never fielding any argument, screen text or `Outcome`. It also introduced the closed-span line class. The span keeps its eight fields; adding one is a boundary widening that waits for the founder.

@history_item n=19 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line, ruling that a diff's three sizes may reach a log while its content never does. No other size of an answer, such as a layout reading's numbers, is ratified for a log.

@history_item n=20 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — created the ratified `tool_call` span with two closed fields, `tool` and `ending`, and `command` as its child by nesting. `tool` is the verb table's word. The new MCP tool `layout` extends the closed set only, with no field added (`decision/tool-call-span-two-closed-fields`, `contract/tool-call-span/*`).

@history_item n=21 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session host writes the command span; the command line writes answers, not logs) — what a command prints is its answer under the ratified crossing, never a log line (`contract/escher-session-stdout/command-answer`). Standing rule: no new surface adds a span or log field holding what a call supplied or a screen reads. `layout --id {id} --session {dir}` on the command line falls under it.

@history_item n=22 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span. The `command` span keeps its eight fields whether a recording is held or not. Created the Recording directory row and the wire `v3` (`contract/host-wire/v3`). The layout step is recorded through the same recorder and the wire stays unchanged; the scope says `call` gains a verb inside it.

@history_item n=23 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each replayed step writes its own `command` span through `Session::run`, and the replay adds none. The answer is kept whole at rest as `answer` (`contract/kept-check/answer`, `contract/recording-file/answer`). Committed checks are only the Counter scripted sessions (`decision/committed-kept-checks-clean`). `Check::replay` of a layout step follows the same rules, and no committed check should gain one holding private data.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the command span's reach gains the coverage role; escher-session's coverage role) — the coverage role replays kept checks through `Session::run`, each step writing its own `command` line, and the coverage line is never logged (`decision/coverage-line-never-logged`). It was ratified at 2026-11-15T05-21-42-wrap. A kept check holding a `layout` step will be replayed by this role. Its malformed refusals (`contract/escher-session-coverage/malformed`) stay the closed list.

@history_item n=25 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of those claims. A change to the span set's closed words is made there by hand (`decision/capability-statement-updated-by-hand`). The verb list moving to fifteen must be edited there manually.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the pipe's live child runs `mcp --record` into an owner-only `record/` (`decision/cold-agent-record-owner-only`). The replay children run without `--record`. `server.log` is counted by span name, with the counted verbs `lint`, `screenshot` and `advance`. The scope keeps the cold-agent bar counting `layout` out, so `layout` is no counted verb and adds no verdict field.

@history_item n=27 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the replay children write stderr to `/dev/null`, uninstrumented by design (`decision/replay-children-uninstrumented`). The pipe reads the ratified `tool_call` line and emits nothing of its own, so a new tool word changes only what it counts.

@history_item n=28 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (3. Observability Harness Contract; 8. PII Scrubbing & Compliance) — the last rewrites of `decision/answers-echo-nothing-typed`'s exception and `decision/scrub-by-field-name`: the capture is judged by the answer's own hidden set and the error field is shown as written. The layout answer carries no caused item and no typed text, so the exception is not extended. The held security-plan row (cry-2dk6n44y) comes from this chunk's line of work.

@history_item n=29 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. / 4. / 8.) — `decision/driver-library-installs-no-subscriber` gained the chaining panic hook. `decision/driver-emits-no-event` now says the capture keeps records and panics in process. `decision/headless-stand-no-sink`'s exception gained the error-text founder check. `decision/escher-target-content-named-set` gained `password`. These are the current texts the layout verb must not contradict: it is a non-acting read, so no scoped capture is set around it.

@history_item n=30 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — `decision/driver-emits-no-event`, `decision/driver-library-installs-no-subscriber` and `decision/headless-stand-no-sink` were rewritten to name the scoped capture around an acting step. The capture is per acting step, so the layout verb, which runs no step, sets none.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open row's bounds and SelectListRow.label) — created `contract/snapshot-diff/open-list-row-bounds`: a row crosses by its own bounds, adding no field, key name, node key or carrier, inside a closed set. This is a precedent for geometry crossing without being a widening. The layout answer's bounds and numbers must likewise add no node key, as the scope requires.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (where a multiple select's set leaves) — created `contract/snapshot/multiple-select-value`: the first chosen option in tree order, leaving only as form entries and the snapshot's value, never a log line. The layout answer must carry no control value.

@history_item n=33 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — origin of `decision/escher-target-content-named-set` and the target allowlist (`decision/closed-span-lines-judged-alike`). Rule for later chunks: a target is admitted only together with a scrub rule for it. The layout verb adds no target.

@history_item n=34 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — trap: a zero read from a host that installs no sink says nothing about one that does. The layout verb's 0-needle reading must be taken from the sink-installing host and the mcp role at `trace` (`decision/sink-host-read-at-trace`, `decision/session-logs-carry-no-page-content`), not only in process.

@history_item n=35 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log; the session host by level under both builds) — established the `host_log` / `mcp_log` reading under the package-alone and workspace builds at `trace`, the standing guard for every new hosted verb. A `layout` call must be added there with its needles.
