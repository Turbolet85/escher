@history_item n=1 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — (4. Unit Test Strategy; 5. Integration Test Strategy) added `decision/no-change-read-control-needs-pending-work`, `contract/session-layout/changes-nothing`, `decision/layout-reason-words-each-read` and `decision/stated-tolerance-read-inclusively` for the in-process `Session::layout` read. This chunk carries that read onto every carrier and gives the unexplained element a closed word of its own. That new word must be read on a fixture in both layout modes. The verb's "changes nothing" control must give the read pending work, because a pump on a settled screen is invisible.

@history_item n=2 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (in-process part, then the gated part) `lint` went the same way this chunk goes. It was first an in-process `Session::lint` marked PROVISIONAL, then a verb on every carrier. The verb table, MCP listing and `describe` count row moved by one. `cli_commands`, `mcp_server`, `stand_mcp_tools`, `stand_lint` and `stand_act_spans` each gained a call. `host_log` and `mcp_log` each drove one call and read 0 needles. The cold-agent pipe's tools moved. A recording kept the new instance verb's calls. "Count histories" were dropped where a count moved. This is the closest precedent for a new read verb.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (graph verb: verb-table pins, session contract, CLI and MCP, stand_mcp_tools) `graph` became the thirteenth verb and ninth instance verb. In `schema_tests`: field kinds 10→12, `describe` row `[13, 14, 12, 3, 8, 4, 15]`, `cli_tests` line rows 64→101. In `mcp_tests`: the listing order. In the session contract: session-start (reads run no step), session-binary (usage line lists it), session-proof. Also `cli_commands` 15→16, `mcp_server` 6→7 and `stand_mcp_tools` 6→7. An argument to it is refused `malformed`. This is the template for the counts and pins `layout` moves.

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (a graph call logs its verb alone; the answer crosses the socket) `graph` joined the closed `command.verb` / `tool_call.tool` sets with no span field added. `host_log` and `mcp_log` each drive one call, with 0 id, name or graph needles at `trace` under both builds. `stand_act_spans` reads the new instance verb once per mode. The socket's crossing widened, listing what the answer carries, never a value or `NodeId`. `layout`'s command span (verb plus cause alone) and its no-value answer follow this pattern.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (verb table at fourteen; CLI and MCP; span and log checks; escher-driver 102) `actions` became the tenth instance verb. Changes: `VERBS` 14, field kinds 13, `describe` row `[14, 14, 12, 3, 8, 4, 15]`, `cli_tests` rows 102. The listing test was renamed `the_listing_is_the_fourteen_verbs_of_the_table`. `cli_commands` 17, `mcp_server` 8 and `stand_mcp_tools` 8 use the shared read-verb helper. `host_log` 9 / `mcp_log` 12 calls; `stand_act_spans` 38 calls per mode. `tools/list` puts `actions` after `graph`. This chunk moves every one of these to fifteen and lists `layout` after `actions`.

@history_item n=6 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (stand_actions; measured actionable counts and the Cells answer size; one actionable rule) added `decision/one-actionable-rule`, `decision/actions-answer-holds-no-value` and `contract/driver-verbs/actions`. `stand_actions` (8) checks that every listed cause equals what the call returns. `ACTIONABLE_AT_BOOT` reads cells 678. The Cells `actions` answer measured 106,088 B, asserted over 100,000 and within 1,048,576. If the actions answer lists the `layout` verb word for every node, it must still equal what the call returns. The Cells answer grows, so its bound assertion and the cross-OS actions digests move.

@history_item n=7 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the cold-agent pipe's tools at fourteen) `valid_step`, `isolated` and `test_cold_agent.py`'s fake were extended to the fourteen `mcp__escher__{verb}` tools. The picked rows `contract/cold-agent-verdict/isolated` and `contract/cold-agent-check/step` name fourteen. A fifteenth MCP tool breaks `isolated` unless the pipe's `VERBS` moves with it, even though the cold-agent bar itself is out of scope.

@history_item n=8 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (a kept actions step's replay, owed a test) the replay of a kept step of a new verb was recorded as untested ("holds by construction"). The test was then owed as a CARRY. This is a trap: the scope's "kept check … `Check::replay` replaying it" for `layout` needs a measured replay test, not one held by construction.

@history_item n=9 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the kept actions step's replay, measured) `stand_replay` gained a test that replays a recorded session with `actions` steps and stops on a flipped field, naming the keys. It covers both layout modes. This is the pattern for proving `layout`'s kept-check replay (`contract/check-replay/stop`). The same chunk added `cli_tests` rows up to 109 and the `coverage` role on the usage line (`contract/escher-session-command-line/usage-line`).

@history_item n=10 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — (2. Test Strategy) added `decision/cross-os-readings-by-answer-digest`. `stand_cross_os_readings` compares the snapshot, lint and actions answer digests of the six served tasks to one committed table, with no shot. The scope says these twelve digests are re-read on linux, windows and macOS: the actions digests move with the new verb word, and the snapshot and lint digests should not.

@history_item n=11 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — (the kept answer and the replay; test counts and the shot's same-host limit) defined the record fields (`answer` kept as an object) and `Check::from_bytes` strictness. It also defined replay stopping at the first differing step. It established that a check compared byte for byte across OSes holds no `screenshot` step (`decision/no-screenshot-in-cross-os-checks`, `decision/no-shot-in-cross-os-byte-checks`). A recorded `layout` step must keep its answer whole under these rules. Any cross-OS digest stays shot-free.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the session lifecycle contract gains the recording) introduced `run` recording each answered instance call (`contract/session-recording/record-fields`). It set `cfg(unix)` item-by-item gating for recording checks (`decision/recording-checks-unix-gated`) and wire `v3`. A `layout` recording check follows the same gating. The wire stays `v3`, since the scope says the wire is unchanged.

@history_item n=13 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — introduced `test_capability_statement.py` (`CapabilityStatementTest`). It holds `docs/capabilities.md`'s verb and cause code spans and its counts in words to the schema's tables, and README's "the {word} driver commands as tools". The guard is on `decision/capability-statement-pinned`, `decision/capabilities-doc-pinned-to-schema` and `decision/capability-statement-from-schema`. The fifteenth verb must be added to the statement and the README count by hand, or the ci-scripts leg goes red.

@history_item n=14 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — set up the schema's unit pins (verb table, lookup, `validate`, first-failure order). It recorded a trap: a count is stated in two word orders ("13 unit tests", "unit tests 13"), so a count-site sweep must search both. This matters for the "five verb counts move from 14 to 15" sweep.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** — introduced `describe()`, `help` answered before any `start`, and the same bytes on MCP and the command line (`contract/mcp-server/help`, `contract/mcp-help/parity`). It also added the `describe` count row pinned in unit tests. `help` describing `layout` from the tables moves that row and the bytes both carriers compare.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — introduced the `tool_call` span parenting `command` (`contract/stand-log/tool-call-span`, `decision/mcp-capture-counts-by-span`) and `mcp_log`. It recorded that MCP stdio-child checks are not unix-gated, but `mcp_server`'s CLI comparison is (`decision/socket-session-checks-unix-gated`). Standing trap: `tracing`'s callsite interest is process-global, so a `with_default` check needs its own recorder per test.

@history_item n=17 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — added `stand_act_spans` and the closed span line. It found that the same check read green per package and red in the workspace leg, so a check reading a sink's capture counts lines by target, never in total (`decision/sink-capture-counted-by-target`). This applies when `stand_act_spans`, `host_log` and `mcp_log` gain a `layout` call.

@history_item n=18 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — added `host_log` on the real binary at `RUST_LOG=trace`, with needles read from the stand (`decision/host-stderr-no-screen-content`, `decision/no-ids-or-names-in-stand-logs`). Traps: `start` cannot be used on a piped host in a must-fail check, and a needle list read from the stand can outgrow a hand-kept one. A `layout` call driven through `host_log` and `mcp_log` keeps these needle rules.

@history_item n=19 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added `LOAD_WAIT` for acting verbs, while `snapshot` and `screenshot` wait on nothing (later `contract/driver-session/reading-verbs` with `lint` and `graph`). `layout` runs no step and no load wait, so it belongs with the reading verbs. Its session-start or reading-verbs statement should list it.

@history_item n=20 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — set the refusal causes at eight, in order, with `fonts-unavailable` last (`decision/eight-refusal-causes-in-order`). Seven are served over MCP (`decision/mcp-causes-served-with-remedy`, except `unknown-verb`). It also set the harness's virtual clock, which reads never move (`decision/harness-virtual-clock`). The scope says no new cause: `layout`'s `stale` / `not-found` / `malformed` refusals must leave the eight and their order unmoved. The clock must not move on a layout read.

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate coverage floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 % (`decision/per-crate-coverage-floors`). It also set the zero-failure flakiness budget on `stand-repeat`. The new executor arm, JSON writer, CLI reader and MCP tool in escher-driver must keep the 97 % floor (last read at 98.04 %).

@history_item n=22 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — (the dev-host complexity ceiling; the two table tests' private helpers) set a cognitive-complexity ceiling of 15 on escher's functions. It factored `mcp_tests.rs`'s listing test into private `#[track_caller]` helpers (`assert_described`, `assert_schema`, `assert_property`). The listing test is renamed per verb count (twelve → fourteen) and will become fifteen. Its recorded reading was taken under an earlier name.

@history_item n=23 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — (3. Test Harness Contract; 5. Integration Test Strategy) widened `decision/socket-carries-commands` with a step's caused and its error text. It added `decision/mcp-result-decoded-whole`: an MCP-vs-in-process comparison decodes the result's JSON string whole through the shared helper, after a Windows red. A `layout` MCP comparison must use that helper. The socket rule's list of what crosses out does not yet name a layout answer.

@history_item n=24 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — (9. CI Integration) added `decision/doc-leg-gates-public-doc-edits` after a public doc comment linked a private item and went red only in CI's Documentation job. The scope's gate block lists the `doc` leg because this chunk edits public doc comments.

@history_item n=25 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — (3. Test Harness Contract; 5. Integration Test Strategy) added `contract/session-recording/exploration-checks` and `decision/exhaustion-asserted-on-finite-page`. If exploration walks the `actions` answer's verb words, a `layout` word on every node adds steps to each explored path. Exhaustion and the kept exploration checks must be re-read.

@history_item n=26 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — (3. Test Harness Contract) restated `overview/session-lifecycle` and added `contract/agent-run-selection/session`, which runs `cli_flow` flows under `run session`. A new command-line verb reached through those flows, or through `cli_commands`, is run by that selection.
