@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — registered the cold-agent pipe (`scripts/cold-agent.sh {run|status|cleanup|logs}`, exit 0/1/2/3, the `run.start`·`run.end`·`status`·`cleanup` events, one isolated `claude -p` session from a per-run `mktemp -d` dir, `verdict.json` passed only on positive evidence), its state area `target/cold-agent/` uploaded by no CI job, the live run's committed `evidence/live-*` copies, and the model provider reached through the operator's own login on the live run only, never CI. Origin of the rows contract/cold-agent-exit-codes/*, contract/cold-agent-events/*, contract/cold-agent-state/state-directory, decision/agent-state-areas-local and decision/model-provider-only-on-live-cold-agent-run that this chunk's one firing runs under; precedent for how a live run's evidence is committed.

@history_item n=2 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the cold-agent crossings (the `claude` client, the `claude -p` session, the provider through the operator's login, the spawned processes) went from PROVISIONAL to ratified by the founder. The provider crossing this chunk fires rests on that ratification; a new kind of content sent to the provider still needs his word, as F4/F5 gave here.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the driver's `command` span (target `escher_driver`, INFO) with `verb` and `cause` recorded on a refused call and nothing a call supplied fielded; the sink prints one line per closed span (PROVISIONAL here, ratified 2026-10-09). The verdict's `refused_by_cause` is counted from this span's `cause` field in `server.log` (contract/command-span/cause, contract/cold-agent-verdict/refused-by-cause).

@history_item n=4 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's closed-span line and a diff's three counts in a log. The pipe's `counts_agree` and per-verb counts read the closed `tool_call`/`command` span lines the `mcp` child prints at `RUST_LOG=info`; this is the ratified basis of that log reading.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP server and the `escher-session mcp` role over stdio with no listener (ratified crossing), its bounds (65,536-byte request line, 1,048,576-byte result), and the `tool_call` span holding only `tool` and `ending`, parent of `command`. The live run's server child is this role; the pipe's tool-call counts are read from `tool_call.tool` (rows decision/mcp-stdio-crossing, decision/tool-call-span-tool-and-ending, contract/tool-call-span/*, contract/mcp-stdio/*).

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the `ci-scripts` leg needs Python 3.11 or later (stdlib `tomllib`), no package added. The pipe's contract tests `.github/scripts/test_cold_agent.py` run in that leg, and the judge is stdlib python3 (decision/ci-scripts-python-floor, decision/cold-agent-judge-stdlib-python); a readiness check that runs them locally meets the same floor.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — the program's own fixed text (help, a session error's remedy, each tool's description with its result fields) rides the ratified carriers; four MCP revisions, default `2025-11-25`. What the cold agent learns of the tools comes from this text (decision/fixed-program-text-on-carriers, contract/mcp-role/tool-listing); a README or capability-page rewrite does not change what crosses.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` verb, `fonts-unavailable`, the PNG in band (base64 on socket/stdout, MCP image block, image result bounded by its whole written form), ratified. The run requires `screenshot_calls` ≥ 1 and the provider receives one shot (decision/screenshot-in-band, decision/screenshot-png-in-band, contract/mcp-server/tool-result).

@history_item n=9 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — (resources) the pipe swapped its stub for the workspace's own `escher-session mcp` server named `escher`, allowed as `mcp__escher`; the client spawns `sh` → `escher-session mcp` with `RUST_LOG=info` and stderr to `target/cold-agent/server.log` (never printed or copied), the judge spawns two replay children, no listener/port/socket added; `check.json` never printed; the provider row carries the stand's screen. (contract) task `stand`, exit 3 when `target/debug/escher-session` is absent (the pipe builds nothing), `--model claude-opus-5-5`, `--max-budget-usd 3`, isolation on the escher tools and one server, `counts_agree` against `tool_call` span lines, the agent's check replayed passing and failing when ablated, `test_cold_agent.py` under a `claude` shim and a fake `escher-session`. The ratified founder words «Один запуск, до $3» and «JSON-проверка с повтором» are the frame of F3's one run; the readiness checks must reach no provider, as the shim tests do.

@history_item n=10 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the README's `## Status` began stating recorded facts per version, among them the 0.1.0 cold-agent figure (43 tool calls, 0 wrong). Precedent for the README stating a cold-agent run as measured, which F2 asks this chunk to do for 0.3.0.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — a session whose launcher names `--record <dir>` keeps its answered calls as one owner-only check (`0700` directory refused `RecordDirNotPrivate`, `check-{n}.json` `0600`, never replaced), never logged; `RecordingFailed` over MCP; wire v3. The live server child records into `target/cold-agent/record/` and the verdict's `recording-not-kept` reads one check per accepted `start` (decision/recorded-session-checks, decision/record-dir-launcher-named-private, contract/recording/*, contract/mcp-server/recording-failed).

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer rests whole (masked, a shot without pixels), so a kept check holds screens' ids, names, values, snapshot text and diffs at rest; the two committed Counter checks are the one exception to owner-only. The run's kept checks under `target/cold-agent/record/` therefore hold screen content: never printed, copied or committed (decision/kept-checks-owner-only, decision/raw-run-output-never-printed); the replayed check the verdict reads goes through `Check::replay`.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` moves the session clock by exactly `ms` on every session, `advanced_ms` exactly `ms`, two clicks with no 500 ms `advance` a double click, and `Cause` closed at eight (`time-unavailable` retired). The run requires `advance_calls` ≥ 1 and `refused_by_cause` counts exactly these eight words in schema order (decision/refusal-causes-closed-at-eight, decision/advance-moves-session-clock); the double-click reading is a trap that can move `wrong_calls`.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (gated part) `lint` became the twelfth verb on every carrier and recorded, findings crossing as stable ids, closed defect words and numbers; the cold-agent session and its `isolated` reading took twelve tools. Origin of decision/lint-findings-on-carriers and contract/driver-verbs/lint; the run requires `lint_calls` ≥ 1.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` takes one optional `part` (1 to 1000), a screen over `SNAPSHOT_TEXT_BUDGET` answers in parts `{"text","part","parts"}`, a paged MCP result its JSON line. The verdict's `part_reads` needs a part read (Cells' screen is over the budget), the reach behind decision/large-screen-in-parts and contract/snapshot-verb/part.

@history_item n=16 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the five acting verbs wait up to `LOAD_WAIT` (5 s wall time) for a load in flight, returning `settled` false, `busy` `loads` past it. The load page's goal in the run (a click waiting out `LoadNet`'s 200 ms answer) rests on it (contract/load-wait/*, contract/driver-session/load-wait); a wall-time wait also counts toward the 600 s client timeout.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the 0.2.0 run's frame: the stand serves Cells and the load page (six slugs, `LoadNet` answering `http://stand.test/load.css` in process, never on the network); `passed` requires all apps met from accepted answers, `lint`/`screenshot`/`advance` used and a part read (`capability-unused`), one kept check per accepted `start` (`recording-not-kept`), `wrong_calls × 100 ≤ 5 × tool_calls` (`wrong-calls-over-bar`); the live child is `escher-session mcp --record target/cold-agent/record`; the provider row widened to the six apps' content; the measured baseline 6 tasks, 66 calls, 0 wrong, $0.8244, `claude` 2.1.308. Also a measured trap: right after a `scroll` a click in the overlay scrollbar band dispatches nothing until an `advance` past the fade, and the flight booker's options read `covered` while the list is closed. The 66/0 baseline is what F1 keeps the 5 % bar against, and the 0.2.0 run's report is where P3 reads `duration_ms` for the 600 s timeout question.

@history_item n=18 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` registered as escher's capability statement with five sections in order; README links it and states «the {word} driver commands as tools»; `CapabilityStatementTest` pins the verb and cause names and five counts to the schema's tables. The README and capability-page rewrite this chunk owes (F2) must keep the pinned phrases and counts green (decision/capability-statement, contract/capability-statement/sections, contract/root-readme/mcp-tool-count).

@history_item n=19 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the README front page restated: 0.2.0 done with three figures (a fresh agent with only the twelve MCP tools drove six apps in 66 calls, 0 wrong, under the 5 % bar), 0.3.0 planned; the default branch GitHub renders is `build/escher-0.3.0`; the pinned README phrase's citation moves with the text. Precedent and constraint for stating the 0.3.0 run's measured figures on the same page, which renders from this branch.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the select limits) an option of a closed single select, under a group too, takes no click and a driver `click` on one stays refused `covered` (decision/closed-select-options-covered). This is the `covered` clause whose effect on `wrong_calls` CARRY cry-32fid2ry asks the live run to read through `refused_by_cause`.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the select list painted by the engine; it crosses the carriers only as a row's own `bounds` in the snapshot and diff (`differs` naming bounds) and as pixels in a shot, a ratified boundary widening; `covered`'s meaning gains «an option of a closed select lies under it: click the select first». The verdict's `list_opens` counts accepted click/press answers whose changed list holds a ListBoxOption with `differs` exactly bounds, and cry-32fid2ry's `covered` reading originates here (decision/select-list-crossing, contract/refusal-causes/covered).

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (test-only binds) «no served network port»: no TCP or UDP port a binary serves, the session socket the one served listener. The live run adds none (decision/cold-agent-run-adds-no-listener, absence/served-network-port).

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the `graph` verb, the UI graph crossing socket, CLI and MCP and resting in a kept check (ids, component/state/event names, closed flags, booleans, never a value), unpaged with Cells' answer 293,869 bytes under the 1 MiB bound; the cold-agent pipe runs with thirteen tools, «the 2026-10-25 live run's twelve MCP tools stays as that run read them». Origin of decision/ui-graph-crossing and the graph-verb rows the run requires used; precedent that a prior run's figures stay as read when the tool count moves.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb and its crossing (ids, owner ids, verb words, closed causes, booleans, `keyed` PROVISIONAL), unpaged, Cells 106,088 bytes; fourteen verbs, the cold-agent pipe's escher tools fourteen; `actions` answers whether a call would be accepted, not what it would do. Origin of decision/actions-answer-crossing and the actions-verb rows the run requires used.

@history_item n=25 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — the founder ratified the actions answer's `keyed` field; decision/actions-keyed-reading and decision/keyed-wire-form went provisional → standing. The `actions` answers F4 lets cross to the provider carry no PROVISIONAL field.

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the `coverage <app> <check>...` role reading named kept checks in process, no listener, and `handler_coverage` with its stated limits. One of the things the F2 reach statement names as proven outside the run (handler coverage), reached by no MCP tool.

@history_item n=27 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — the coverage role's kept-check files on argv and the role's shape ratified by the founder's own word, replacing the delegate confirm. The reach statement can name handler coverage as a ratified, proven capability outside the run.

@history_item n=28 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — one run seed as a Dioxus root context and decision/no-seed-on-carriers: no verb, argument, flag, MCP field, kept-check key or span field carries a seed, so every driver session and every replay reads the default seed. The seed is one of the reach statement's out-of-run items, and the live run's sessions read the default seed.

@history_item n=29 marker=2026-11-14-remote-pages-local-files
**2026-11-14-remote-pages-local-files** — (Standard Contracts) decision/no-seed-on-carriers went provisional → standing on the founder's own answer. The reach statement can state the seed's absence on the carriers as ratified.

@history_item n=30 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — every stand boot loads the one bundled DejaVu Sans with system fonts off, guarded by `stand_bundled_font_families`, and the served tasks read the same answer digests on every CI OS. The bundled fonts are a reach-statement item proven outside the run; the live run's screens and shot are laid out in this typography (decision/stand-typography-dejavu-sans, decision/stand-pinned-environment).

@history_item n=31 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `Session::reach`, `start_in` and absence/fixture-state-on-carriers: a fixture state is reached in process only, never on the command line, MCP, socket or a kept check. A reach-statement item the live agent cannot reach; it starts every app from boot.

@history_item n=32 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — the in-process `explore` call and absence/exploration-on-carriers; the kept check's opening record gained `"format":1`. Exploration is a reach-statement item outside the run, and the run's kept checks open with the format record the strict reader requires.

@history_item n=33 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — absence/component-alone-on-carriers and the stand's `boot_alone`; also the founder ratified the delegate confirms of the error-field and layout-on-the-carriers answer cards. A component alone is a reach-statement item outside the run; the `caused` error field and the `layout` crossing the run exercises stand on his own word.

@history_item n=34 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — acting verbs gained `caused` (contract/acting-verb-results/caused), the outcome object writing it only when the step caused something, a second settle panic reading busy `render`, and the driver running each acting step under escher-telemetry's scoped capture with no global subscriber. `caused_by_kind` is counted from these `caused` lists, so this is the surface the run's panic/error/log requirement reads.

@history_item n=35 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — `caused` became one item per panic, error or record with its text (message cut at 1024 bytes, 16 kept, `caused_past` beyond), decision/no-echo-of-typed-input gained its except, decision/no-node-id-across-session added «a step's caused text», and the capture's panic hook became the driver library's stated except. The order page's authored messages that cross to the provider (F5, decision/cold-agent-run-carries-stand-screen) cross under these rules.

@history_item n=36 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — a `caused` item's `error` field crosses as written, scrubbed only by the answer's own hidden field names. Part of what the order page's error control sends to the provider on the live run.

@history_item n=37 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `layout` became the fifteenth verb (eleventh instance verb) on every carrier and kept checks, retiring decision/layout-read-in-process-only for decision/layout-answer-crossing; `actions` lists `layout` fifth with no cause; decision/no-node-id-across-session and decision/select-list-crossing widened to layout readings. The run requires `layout_calls` ≥ 1 and the agent's tools are now fifteen; the layout answer is among what the provider receives.

@history_item n=38 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — decision/capability-statement: the pin also holds the a11y leg's file count and stand-repeat's named-target count to the CI leg runner. A rewrite of `docs/capabilities.md` and the README for the run's result must keep those counts and the five schema counts as `.github/scripts/test_capability_statement.py` reads them.

@history_item n=39 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** — `capability-unused` widened to `graph`, `actions`, `layout` and a list opening; the verdict gained `graph_calls`, `actions_calls`, `layout_calls`, `list_opens` and `refused_by_cause`; decision/cold-agent-run-carries-stand-screen widened to graph and actions answers on the founder's F4. This is the bar the one live run is judged against (CARRY cry-6mautsk2) and the provider-crossing row F4 ratified; the wrap restates the security-plan row with F4's word.

@history_item n=40 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the seventh served task `order` (stand boot, `escher-session` invocation, usage, MCP start enum, cross-OS readings at seven); contract/cold-agent-verdict/passed requires all seven apps met; `capability-unused` also fires when any `caused_by_kind` kind reads 0; contract/cold-agent-verdict/caused-by-kind added; decision/cold-agent-run-carries-stand-screen widened to acting answers' caused text, the order page's authored messages among it (F5); decision/cold-agent-six-apps retired with no heir (the prompt's app list is the script's to hold). The immediate precedent this chunk rests on: CARRY cry-vrneu5x7 concretizes v030-15's acceptance on these seven apps and three kinds.
