@amendment marker=2026-11-08-what-can-i-do-here
title: dioxus-native-dom's actionable reading, one rule for two readers
section: §1 Test Scope Summary (body: the dioxus-native-dom line; test-scope-summary.md: the dioxus-native-dom entry) · §4 Unit Test Strategy (unit-test-strategy.md: new detail, the actionable reading)
change:
  - The body's dioxus-native-dom line names the actionable nodes reader beside the actionable-key check, over their one rule.
  - The crate's unit tests read 104 in eleven files, as measured at this chunk (was 100, as measured at 2026-11-07-ui-graph).
  - actionable.rs holds eight: the four on `unkeyed_actionable` and four on the one actionable rule read by it and by `DioxusDocument::actionable_nodes` — a listener removed on a re-render read by neither reader, a select's options and group listed with the select as owner, a positional id reading unkeyed and an author key keyed, an inert or hidden element not listed (was four).
  - New detail: `actionable::tests`' four new tests named, the removed-listener test also asserting that `data-dioxus-id` outlives the listener, the in-tree witness that a reading of the mark would still count it.
why: the report's Symbols / APIs (`actionable_nodes` and the crate-private `Readings`, the one rule both readers read, the listener leg moved from the mark to the exact current listeners), its Counts (the actionable unit tests 4 → 8, the crate 100 → 104) and its Deviations (the in-tree witness, chosen over a clone control).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: cited line ranges re-read where the chunk's code moved them
section: §3 Test Harness Contract (body) · §9 CI Integration (ci-integration.md) · §6 E2E Test Strategy (e2e-test-strategy.md) · §5 Integration Test Strategy (integration-test-strategy.md) · §7 Test Data & Fixtures (test-data-fixtures.md) · §1 Test Scope Summary (test-scope-summary.md) · §2 Test Strategy (test-strategy.md) · §4 Unit Test Strategy (unit-test-strategy.md) · Session lifecycle contract (contracts/session-lifecycle: session-start, session-state)
change:
  - The cited line ranges re-read to where this chunk's added lines moved them, in blitz-tests' `common`, `session_common`, `stand_mcp_tools` and `stand_act_spans`; seven_guis' `cli_commands`, `mcp_server`, `host_log`, `mcp_log` and shared test module; escher-driver's schema.rs, schema_tests.rs, command.rs, execute.rs, json.rs, cli_tests.rs, mcp_tests.rs and recording_tests.rs; dioxus-native-dom's actionable.rs; `test_cold_agent.py`; and `docs/capabilities.md` — no claim changed by the re-read alone.
why: the report's Files (31 changed against the base, each of these among them).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: blitz-tests' shared modules read by stand_actions, and its target count
section: §3 Test Harness Contract (body: the fixtures entry) · §2 Test Strategy (test-strategy.md: the directory pattern)
change:
  - `tests/common/mod.rs` read through `mod common;` by 23 `stand_*.rs` checks (was 22).
  - `tests/session_common/mod.rs` read by 34 checks, as measured at this chunk, `stand_actions.rs` the latest (was 33, as measured at 2026-11-07-ui-graph).
  - `tests/fixtures/mod.rs` read by 32 checks, 24 of them `stand_*` files, `stand_actions` added to both lists (was 31 and 23), as measured at this chunk in §2.
  - blitz-tests holds 160 `.rs` files, 159 test targets and the unbuilt `all.rs`, `stand_actions.rs` the latest auto-discovered target (was 159 and 158).
why: the report's Counts (session_common declarers 33 → 34, fixtures declarers 31 → 32 with its stand_* files 23 → 24, the stand_*.rs files 52 → 53) and Files (`stand_actions.rs` new, no manifest line); the `mod common;` count of 23 is main's measurement at this wrap's validate, which the report's Counts omit.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the cold-agent pipe's tools at fourteen
section: §3 Test Harness Contract (body: the server and the check's judge, the verdict) · §4 Unit Test Strategy (unit-test-strategy.md: the cold-agent pipe)
change:
  - The pipe's MCP surface speaks the fourteen verbs, and `valid_step` admits only a `tool` among the fourteen (was thirteen).
  - `isolated` reads the init tools exactly the fourteen `mcp__escher__{verb}` tools (was thirteen).
  - test_cold_agent.py's fake `escher-session` speaks the fourteen tools, `actions` after `graph` (was thirteen); the test count stays 37.
why: the report's Counts (the cold-agent pipe's `VERBS` joins `actions` after `graph`, uncounted; `VERBS` 13 → 14) and its Deviations (the fake's stale comment corrected to fourteen).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: stand_actions holds the actions read over the served screens
section: §5 Integration Test Strategy (body: Session ↔ held instance; integration-test-strategy.md: new details — what can be done here, a load in flight without the wait, the actions answer holds no value) · §7 Test Data & Fixtures (test-data-fixtures.md: the network answer row) · §1 Test Scope Summary (test-scope-summary.md: the driver-session checks, the escher-driver entry) · §2 Test Strategy (test-strategy.md: font-dependent tests)
change:
  - The body's Session ↔ held instance line adds the `actions` verb listing each served screen's actionable nodes with the table's verbs, every listed cause what the call returns, changing nothing (`stand_actions`).
  - New detail: `stand_actions` (8), over the six served screens in both layout modes — the listing and its counts; the lean tasks' causes equal to `Session::run`, 3 `disabled`; the stated Cells and load-page causes; the flight booker's options `covered` with owner `flight-type` until the list opens; `crud-person-14` `off-screen` until a `scroll`; the fixtures `covered`, unkeyed and `fonts-unavailable`; a read changing nothing and holding no typed text or mask; the Cells answer within the answer bound.
  - New detail: a fetch put in flight with `harness.click("#load-fetch")`, which waits on nothing, where a driver `click` waits out `LOAD_WAIT`; the load page row names `stand_actions` among its readers.
  - New detail: the actions answer holds no value — ids, owner ids, verb and cause words and booleans; a typed sentinel and `MASKED_VALUE` 0 times, no `NodeId`; an argument refused `malformed` in `stand_actions`, `cli_commands`, `mcp_server` and `stand_mcp_tools`.
  - §1 lists `stand_actions` 8, declaring `mod common;`, `mod fixtures;` and `mod session_common;`, picked up by `run stand` and the stand-repeat leg by its prefix; `Session::actions` and `check_target` covered by it, not by a unit test.
  - Font-dependent tests: the `actions` read names `fonts-unavailable` for `screenshot` with nothing painted, `check_target` reading `can_paint_text` after the crop and before painting.
why: the report's Outcome (matrix#v030-06, arch, security and a11y, each met through `stand_actions`), its Decisions (a fetch in flight without the driver's wait), its Deviations (`can_paint_text` read before painting; the `disabled` and `MASKED_VALUE` assertions) and its Coverage of new surfaces.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the legs with stand_actions, the baseline re-count and the coverage floors
section: §9 CI Integration (ci-integration.md: the legs, the local baseline) · §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the coverage floors, the flakiness budget)
change:
  - stand-repeat runs one `--test` per `stand_*.rs` file, 53 (was 52), `stand_actions` joining by its prefix.
  - The flakiness budget's leg runs the 53 `stand_*.rs` files (was 52), `stand_actions` entering through the existing glob, the CI run of the committed tree green 17/17.
  - The local baseline re-counted with `stand_actions`: 223 result lines, 0 failed, the passed and ignored totals not read, the moves from 219 not attributed line by line, `stand_actions` one new result line of 8 passed among them.
  - Coverage: with `actions.rs`, `check_target`, `actionable_nodes` and `Readings` in the tree, the `Coverage report` check passes under the same floors, holding escher-driver ≥ 97 % and dioxus-native-dom ≥ 83 %, the per-crate figures not read.
why: the report's Harness / gate surface (no CI step changed; the glob takes `stand_actions` in), its Counts (the stand_*.rs files 52 → 53), its Outcome basis (`ci-leg.sh fast` on the committed tree, 223 ok, 0 FAILED) and its Outcome legs (CI green 17/17, the Coverage report passing, the figures not read).
kept: the per-crate coverage figures last stated stay those measured at 2026-11-07-mounteddata-buffered-render; this chunk read none.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions verb through the command line and MCP
section: §6 E2E Test Strategy (e2e-test-strategy.md: command line, MCP; new details — command line, the actionable nodes; MCP, the actionable nodes) · §5 Integration Test Strategy (integration-test-strategy.md: the graph over MCP in process) · §1 Test Scope Summary (test-scope-summary.md: the apps entry, the driver-session checks) · §4 Unit Test Strategy (unit-test-strategy.md: the verb table's binary checks)
change:
  - `cli_commands` reads 17 tests per OS in §1, §4 and §6 (was 16): each of the ten instance-level verbs accepted (was nine), and `actions_answers_one_line_of_the_actionable_nodes` reading one JSON line, an argument refused status `1` (new detail).
  - `mcp_server` reads 8 (was 7), `tools/list` offering fourteen verbs, `graph` after `lint` and `actions` after `graph` (was thirteen), an `actions` tool call answering its line and an argument reading `isError` (new detail).
  - `stand_mcp_tools` reads 8 (was 7 in §1, 6 in §6): an `actions` tool call answering the reader's nodes and parenting one `command` span recording `verb` alone, in both layout modes; the `graph` test's body is the shared helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`, which the `actions` test runs too.
why: the report's Counts (cli_commands 16 → 17, mcp_server 7 → 8, stand_mcp_tools 7 → 8, instance verbs 9 → 10, `VERBS` 13 → 14), its Outcome (security: an argument refused on every carrier; obs: `tool_call` parenting one `command`) and its Deviations (the shared read-verb helper, one statement of the contract).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the measured actionable counts and the Cells actions answer's size
section: §7 Test Data & Fixtures (test-data-fixtures.md: new row, actionable counts at boot) · §5 Integration Test Strategy (integration-test-strategy.md: new detail, the Cells actions answer's size)
change:
  - New row: `ACTIONABLE_AT_BOOT` in the stand checks' shared module — counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2 — read by `stand_actions`.
  - New detail: the Cells `actions` answer measures 106,088 bytes for 678 nodes through the binary, 461 cells `off-screen` for `click`, `type` and `screenshot`, all 678 taking `scroll`; `the_whole_cells_answer_is_within_the_answer_bound` asserts over 100,000 bytes and within 1,048,576, in both layout modes.
why: the report's Decisions (the counts and the answer's size recorded as measured, not assumed, against the plan's Cells forecast of about 677) and its Counts (new, no prior site).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: a kept actions step's replay, owed a test
section: §5 Integration Test Strategy (integration-test-strategy.md: new detail, a kept actions step's replay)
change:
  - New detail: no test exercises the replay of a kept `actions` step; `Check::replay` runs every instance-verb step through `Session::run`, so it holds by construction, unmeasured; the replay test is owed, a CARRY on «Handler coverage».
why: the report's Outcome reads the criterion unmet as written, in part (the step kept whole and read back, its replay untested); the ruling that the replay test is owed rather than accepted by construction is inputs#I6, the operator's message relaying the overseer and the founder at this wrap's resume, 2026-11-08 (the report's Addendum 1).
kept: test-plan states no matrix status; that v030-06 reads verified only once the owed test passes stays the report's and the ledger's, left out of the detail.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions call in the span and log checks
section: §5 Integration Test Strategy (integration-test-strategy.md: the sink host's coverage) · §1 Test Scope Summary (test-scope-summary.md: the apps entry, the driver-session checks)
change:
  - `host_log` and `mcp_log` each drive one `actions` call: `host_log` nine calls and 9 command-span lines (was eight and 8), `mcp_log` twelve tool calls and 10 `command` lines (was eleven and 9), 0 needles at `trace`, the actions needles among them, as measured at this chunk.
  - seven_guis' shared test module gains `actions_needles()`, the CRUD screen's listed author-key ids holding a `-`, called by `host_log` and `mcp_log`.
  - `stand_act_spans` reads the tenth instance verb, `actions`, one call per mode, its `command` line recording `verb` alone, 38 calls per mode (was 36).
why: the report's Counts (host_log's hosted calls 8 → 9, mcp_log's tool calls 11 → 12, the needle helpers gaining `actions_needles`, stand_act_spans' driven calls → 38 and its instance-row pin 9 → 10) and its Outcome (obs: 0 id, name and actions needles at `trace` under both builds).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the verb table at fourteen, actions the tenth instance verb
section: §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: complexity ceiling) · §1 Test Scope Summary (test-scope-summary.md: the escher-driver entry) · §4 Unit Test Strategy (unit-test-strategy.md: the verb table, the list's help, the MCP listing test's helpers; new detail — the two `nodes` fields) · Session lifecycle contract (contracts/session-lifecycle: session-start, session-binary)
change:
  - schema_tests pins fourteen verbs, ten instance-level, `actions` the tenth after `graph` with no argument and one field `nodes` of `FieldKind::ActionNodes` (was thirteen and nine); the field kinds thirteen, `ActionNodes` (`action-nodes`) last (was twelve).
  - `VERBS` at 14 and `describe`'s count row `[14, 14, 12, 3, 8, 4, 15]` (was 13 and `[13, 14, 12, 3, 8, 4, 15]`).
  - cli_tests' line-reading rows 102, `actions`'s among them (was 101).
  - mcp_tests' listing test is `the_listing_is_the_fourteen_verbs_of_the_table`, asserting fourteen verbs, `actions` after `graph` (was `the_listing_is_the_twelve_verbs_of_the_table`, thirteen); the complexity ceiling names it so, its reading of 3 taken under the earlier name and not re-read.
  - New detail: schema_tests keys its field-kind table per verb (`graph.nodes`, `actions.nodes`) and pins the `actions` help among its 12 `amended` rows.
  - Session lifecycle: the command line holds fourteen verbs, ten instance-level (was thirteen and nine); a recording keeps each answered call of the ten instance verbs, an `actions` among them (was nine).
why: the report's Schema / config (`VERBS` 14, `FieldKind` thirteen, `NODE_FIELDS` and the causes unchanged), its Counts (instance verbs 9 → 10, describe's row, the two test names renamed to fourteen) and its Deviations (the per-verb field-kind table, the `amended` rows 11 → 12); cli_tests' 102 rows are main's measurement at this wrap's validate.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: escher-driver's unit tests at 102
section: §1 Test Scope Summary (test-scope-summary.md: the escher-driver entry) · §4 Unit Test Strategy (unit-test-strategy.md: new detail, the `actions` read)
change:
  - escher-driver reads 102 unit tests in seventeen files, 66 inline (was 100 in sixteen, 64 inline), actions.rs 2 joining the per-file list.
  - New detail: actions.rs' inline `mod tests` — every call `call_naming` names for a target verb validating, and an id past its bound reading `malformed`.
why: the report's Counts (escher-driver unit tests 100 → 102, actions.rs a seventeenth file) and its Gates (`cargo test -p escher-driver` green, 102 passed).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
