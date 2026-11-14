@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand `seven_guis::stand` and its in-process checks install no subscriber, read no env var and print nothing; only the windowed binary installs the sink. The served tasks' boot in `examples/seven_guis/src/stand.rs` that this chunk extends with named states stays sinkless (decision/headless-stand-no-sink).

@history_item n=2 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the one named print exception: a re-executed child's stdout read only into its parent's assertion, never a log. Precedent for decision/checks-silent's except clause if `stand_start_in_state` compares a by-hand screen across a re-run child.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the headless-stand silence census names the checks' shared modules, not only the `stand_*` files. A new or extended shared module, such as `tests/blitz-tests/tests/fixtures/mod.rs` or `session_common/mod.rs`, has to be read 0 by the census.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (a second sink installer) — `escher-session` installs the sink; a headless boot made in process has no sink, while one made by the binary runs under its sink. Trap: a zero read from a host that installs no sink says nothing about one that does. A state-started session through `start --state` boots inside the sink-installing host, so it has to be read there (decision/headless-stand-no-sink, decision/sink-host-read-at-trace).

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher targets print every field except the content-named ones, and all other targets are dropped. Standing rule: a target is admitted only together with a scrub rule. A state name is not in the content-named set, so it has to stay off every field. This is the basis for decision/sink-host-read-at-trace.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Session::act` and `Harness::settle` are silent. Standing rule: add no span, event or log line to the settle loop or the session step. This applies if `start` reaches a state by running steps from boot.

@history_item n=7 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the schema (`validate`, `VERBS`, `Refusal`/`Cause`) is silent, and `Command`/`Call`/`ArgValue` derive `Debug` but are never printed. Standing rule: never field an argument by its schema name and never field a `Command`, `Call` or `ArgValue` with `Debug`. Of the argument names only `text` is scrubbed, so a fielded `state` argument would print.

@history_item n=8 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor (`execute`, `Session::run`) is silent and `Outcome` joins the never-logged `Debug` types. The cause field takes `Cause::name` only. This bears on how a refused unknown state name is classed (`malformed` vs `not-found`) without any call content (decision/debug-types-never-logged).

@history_item n=9 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the session's record of ids is bounded, has no reader outside the crate, has no `Debug`, and is never printed, logged or fielded (decision/session-id-record-unprinted). A session started in a state seeds its first screen and must keep the record under the same rule.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built) — the `command` span has eight fields recorded only where they apply, and it never fields any argument, label or call value. `stand_act_spans`' re-run children are the sink exception. This is why the scope says the command span records no state name.

@history_item n=11 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed-span line was ratified as a second record class, and the `added`/`removed`/`changed` sizes stand on it. A new span field or record class is a boundary widening that needs the founder's word, which matters if the state ever tempts a field.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line writes answers, not logs) — `command_line` installs no subscriber and reads no env var, and each answer, refusal or error object is one line of JSON on stdout. A call refused on the command line opens no span, and the `start` spawn is observed absent. Standing rule: the next surface adds no span or log field that holds what a call supplied. A `--state` refusal on `start` falls under this.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (typed text measured in a host's log) — `host_log` reads the `escher-session` host at `trace` under both builds for id, name and typed-sentinel needles. This is the standing check that a state-started hosted session would extend (decision/sink-host-read-at-trace, decision/session-logs-carry-no-page-content).

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `tool_call` span (`tool`, `ending`) was ratified. Session verbs such as `start`, answered by the server, leave a `tool_call` with no `command` child, and the `mcp` role writes protocol lines only on stdout. A `state` argument on the MCP `start` tool must not reach either span.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help`/`describe()` is the program's own fixed text and the session-error object gained a fixed `remedy`. The usage line is fixed text built from `VERBS` and the task names. The scope touches `describe()` and the usage line for `--state` (contract/escher-session-commands/help, contract/escher-session-exit-codes/2).

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — an `advance` is accepted on every session and records the acting fields with no cause, and the cause set is eight names. Precedent for decision/advance-accepted-every-session on a session that starts in a state.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the eight cause names and the advance rows were ratified by the founder. They are the closed set a refused `--state` must reuse, not extend.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording (`recording_into`, the recorder, `Check::from_bytes`, the wire's `recording-failed`) opens no span and logs nothing. The wire is `v3`, and `start … --record`/`mcp --record` have their usage and status-3 endings. The recording directory is `0700` with one `0600` check per session, holding content at rest. The scope's fork on adding a state name to the kept check's closed key set and to `Check::from_bytes` changes this at-rest content and the `hello` wire (contract/host-wire/v3, decision/recording-dir-private).

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` writes each step's `command` span through `Session::run` and adds none of its own. The kept answer is held whole at rest. The committed Counter checks are public and written by `record_checks.sh`. A state in the kept check has to replay from that state, and committed checks must stay free of private data (decision/committed-checks-hold-no-private-data). This was a founder-ratified widening.

@history_item n=20 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — the seven_guis shared module holds the `host_log`/`mcp_log` needle helpers, and assertion messages name a kind and a count, never a needle (decision/log-check-failures-name-kind-count). Any new state-start log test reuses these helpers.

@history_item n=21 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/blitz-tests/tests/fixtures/mod.rs`, which holds boot, hold, pages and `HeldNet` and which this chunk builds on, is silent: no print, tracing, subscriber, env read, file, socket, listener or `blitz_net`. Its reader count is tracked in §3.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `fixtures` module's readers, 24 to 28) — the latest count of the fixtures module's readers. A new `stand_start_in_state` reader moves it.

@history_item n=23 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the driver's `session` reads a clock for the load wait alone, and that wait adds no span or field inside the `command` span. A state reached at `start` must not add a clock read or a span of its own.

@history_item n=24 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — `lint` runs inside its call's `command` span and records `verb` alone. Precedent for contract/command-span/lint on a state-started session.

@history_item n=25 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone and never a part number or count (contract/command-span/snapshot-part, decision/hosted-call-through-session-run).

@history_item n=26 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand_requirement_sweep` with its `WITNESSES` tables is silent, and `host_log`'s clients are read at `trace`. The scope requires the served tasks to name the states a stand check reaches by clicking, which bears on these witnessed stand checks.

@history_item n=27 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of those claims, so changes are made by hand (decision/capability-statement-updated-by-hand). The scope touches the statement where verb or argument counts move.

@history_item n=28 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the pipe spawns `escher-session mcp` with `RUST_LOG=info` on that child only, and its replay children write stderr to `/dev/null` (decision/replay-children-uninstrumented). A state on `start` reaches the pipe's MCP `start` calls.

@history_item n=29 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (verdict fields and the live child's recording) — the live child runs `mcp --record` into an owner-only `record/` that is never printed, copied or committed. The replay children run without `--record` (decision/cold-agent-kept-checks-at-rest, decision/cold-agent-kept-checks-stay-local). A state carried in a kept check stays inside this.

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the host's six slugs and the served stand's silence) — `main` hands `command_line` the six slugs (the private `Served` table) and the stand's boots, and the usage line `apps:` and the MCP `start` tool's `enum` follow as fixed text. `boot_cells`, `boot_load` and `LoadNet` are silent. This is the served-task table and boot that the chunk's state names extend.

@history_item n=31 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb and its never-logged answer) — `Outcome::Graphed` is written by `to_json` alone and never logged (decision/graph-answer-never-logged). `session_common` readers went to thirty-three.

@history_item n=32 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb's command and tool_call lines) — a graph's line is `verb="graph"` and holds no node, state, id or name (contract/command-span/graph). This applies on a state-started session too.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the state registry holds no state value) — `SignalRegistry` holds names, declarations, write records and the dispatch chain, never a value, and its `Debug` prints counts (decision/state-registry-holds-no-values). An app-supplied initial state must not put a value into it or into any log.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `clickable` is a closed bool carrying no page text (contract/snapshot-node/clickable, contract/snapshot-text/clickable). It is part of the first-screen snapshot text that the chunk's proof compares by digest or whole.

@history_item n=35 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions read opens no span and adds no site) — `Session::actions` changes nothing in the document and opens no span (decision/actions-read-changes-nothing).

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (actions joins the spans' verbs and tools) — `VERBS` has fourteen words and the `tool_call` `tool` is one of fourteen. An actions line holds no node or id (contract/command-span/actions). Adding an argument to `start` must not move these counts.

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role) — the `coverage` role replays kept checks through `Session::run`, and each step writes its own `command` span. The coverage line is a stdout answer that is never logged. This was ratified, with the obs edits PROVISIONAL (delegate overseer). `coverage.rs`'s replay boot is in scope for state-started checks (decision/coverage-line-never-logged, contract/coverage-role-stdout/coverage-line).

@history_item n=38 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's endings gain the coverage role) — a check of another app, or a file that is no check, is refused `malformed` and holds nothing of the path. Its usage clause is fixed text (contract/escher-session-coverage/*). A check that names a state the app lacks must be refused the same way.

@history_item n=39 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the command span's reach gains the coverage role) — the coverage role adds no span, and `host_log`'s coverage run reads 0 needles at `trace`. Its §2 and §6 edits are PROVISIONAL (delegate overseer) for the founder's batch.

@history_item n=40 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's carried fields) — the coverage line carries stable ids, owner ids, event names and two counts, a ratified widening with the row PROVISIONAL (contract/coverage-line/content). A state name on a coverage run must not enter it.

@history_item n=41 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the session_common module's readers) — the `session_common` readers are thirty-five. A new `stand_start_in_state` reader moves the count.

@history_item n=42 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master was turned into rows, each verified against the code, with fates by the founder's approval. Every picked row binding this chunk comes from this conversion, and amendments now name rows by handle.

@history_item n=43 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `~ decision/checks-silent`: the except clause now covers a re-run child printing "ids or digests" its parent reads. This chunk's state-versus-by-hand proof by answer digest may need the same exception. The seed's precedent rides a Dioxus root context, which is one of the scope's two ways to reach a state.
