@proposal n=1 detector=D-cascade severity=warning file=test-strategy anchor=fct-2vwamopj basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 2. Test Strategy
change: after «read by `stand_act_screenshot` and `stand_mcp_tools` on an app booted with no usable font» add «and, since 2026-11-08-what-can-i-do-here, the `actions` read reports the same `fonts-unavailable` for `screenshot` with nothing painted — the shared `check_target` reading `can_paint_text` after the crop and before painting — read by `stand_actions`' `the_fixtures_read_covered_unkeyed_and_fonts_unavailable` in both layout modes»
sidecar: test-strategy#7.1: the actions read joins the fonts-unavailable readers (stand_actions), decided before painting
rationale: the report's Symbols / APIs states check_target's screenshot arm meets can_paint_text false as fonts-unavailable before painting, and its Deviations (fct-kwnslo3j) and Outcome name the stand_actions fixtures test
@proposal n=2 detector=D-cascade severity=warning file=test-strategy anchor=fct-7m7lxsmy basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 2. Test Strategy
change: `tests/session_common/mod.rs` read through `mod session_common;` by 34 checks as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md (`stand_actions.rs` the latest, then `stand_graph.rs`, …; 33 before `stand_actions.rs`); `tests/fixtures/mod.rs` read through `mod fixtures;` by 32 checks — 24 `stand_*.rs`, `stand_actions` added to the list — as measured at that report; the `.rs` count 160 files and 159 test targets, `stand_actions.rs` the latest, its own auto-discovered target with no manifest line (160 and 159 → the prior 159 and 158 before it)
sidecar: test-strategy#9.1: session_common readers 33 to 34, fixtures readers 31 to 32 (stand_* 23 to 24), blitz-tests files and targets one more, stand_actions.rs
rationale: the report's Counts moved names session_common declarers 33 to 34 and fixtures declarers 31 to 32 with stand_* 23 to 24, stand_actions the new reader, and stand_*.rs 52 to 53 with tests/blitz-tests/Cargo.toml not among the 31 changed files
@proposal n=3 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-klxgwgcy basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 4. Unit Test Strategy
change: schema_tests.rs pins the verb table — fourteen verbs in a stated order: the ten instance-level verbs, … `graph` after `lint` (no argument; fields `nodes` and `states`), since 2026-11-08-what-can-i-do-here `actions` after `graph` (no argument; one field `nodes` of `FieldKind::ActionNodes`), then `start`, `status`, `stop`, `help` …; the field kinds thirteen (… then `GraphNodes` and `GraphStates`, then `ActionNodes` (`action-nodes`)); through the binary by seven_guis' `cli_commands` (17) …; the closing measure «the verb table's fourteen verbs, thirteen field kinds and fourteen node keys at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: unit-test-strategy#4.2: VERBS 13 to 14 (actions the tenth instance verb), FieldKind 12 to 13 (ActionNodes), cli_commands 16 to 17
rationale: the report's Schema / config and Counts moved state VERBS 13 to 14, instance verbs 9 to 10, FieldKind 12 to 13 adding ActionNodes, and cli_commands tests per OS 16 to 17; NODE_FIELDS 14 unchanged
@proposal n=4 detector=D-new-detail severity=warning file=unit-test-strategy anchor=fct-klxgwgcy fact=F8 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 4. Unit Test Strategy
change: - **escher-driver (the `actions` read):** actions.rs holds two unit tests in its inline `mod tests` — every call `call_naming` names for a target verb validates (`every_call_named_for_a_target_verb_validates`), and an id past its bound reads `malformed` (`an_id_past_its_bound_reads_malformed`); escher-driver's unit tests 100 → 102, actions.rs the seventeenth file in the per-file list — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: unit-test-strategy: new detail beside #4.2, actions.rs' two unit tests (F8)
rationale: F8; the report's New text lists actions.rs 142-149 and 151-157 as the two tests, and Counts moved states escher-driver unit tests 100 to 102
@proposal n=5 detector=D-new-detail severity=warning file=unit-test-strategy anchor=fct-klxgwgcy fact=F9 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-mutkjl2d
section: ## 4. Unit Test Strategy
change: - **escher-driver (the two `nodes` fields):** schema_tests' field-kind table is keyed per verb (`graph.nodes`, `actions.nodes`), because the two verbs' `nodes` fields differ in kind, and the `actions` help is pinned word for word among the `amended` rows, 11 → 12 — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: unit-test-strategy: new detail beside #4.2, schema_tests' per-verb field-kind table and the actions help pin (F9)
rationale: F9; the report's Deviations (fct-mutkjl2d) and Decisions (fct-cm5gxn3o) state the per-verb keying and the amended rows 11 to 12
@proposal n=6 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-fghufsse basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 4. Unit Test Strategy
change: with `VERBS` at 14, causes at 8 and `describe`'s count row `[14, 14, 12, 3, 8, 4, 15]` — `VERBS` and the count row as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: unit-test-strategy#5.1: VERBS 13 to 14, describe count row 13 to 14 in its first place
rationale: the report's Counts moved: VERBS 13 to 14 and describe's row [13, 14, 12, 3, 8, 4, 15] to [14, 14, 12, 3, 8, 4, 15]; causes unchanged at 8
@proposal n=7 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-zot44d7e basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 4. Unit Test Strategy
change: mcp_tests.rs's `the_listing_is_the_fourteen_verbs_of_the_table` asserts the fourteen verbs of the table in `VERBS`' order, `graph` after `lint` and `actions` after `graph`, … — the fourteen verbs and the test's name as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: unit-test-strategy#6.1: the MCP listing test renamed to the_fourteen_verbs, asserting fourteen verbs with actions after graph
rationale: the report's Counts moved renames mcp_tests' the_listing_is_the_thirteen_verbs_of_the_table to the_fourteen_…, and Outcome reads tools/list fourteen in table order; the entity still names the older twelve
@proposal n=8 detector=D-cascade severity=warning file=unit-test-strategy anchor=fct-3ferhyj7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 4. Unit Test Strategy
change: a fake `target/debug/escher-session` (`FAKE`, which speaks the fourteen tools, `actions` joining after `graph` since 2026-11-08-what-can-i-do-here, answering `lint` with `{"findings":[]}` and `graph` with `{"nodes":[],"states":[]}`, …), the test count unchanged at 37
sidecar: unit-test-strategy#10.1: the cold-agent FAKE speaks fourteen tools, actions among them
rationale: the report's Counts moved states the cold-agent pipe's VERBS joins actions after graph, uncounted, and Deviations (fct-kjwgstc4) corrects the fake's comment to fourteen; no test is added to test_cold_agent.py
@proposal n=9 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-t7rg232i fact=F4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-cfi7tfzq
section: ## 5. Integration Test Strategy
change: - **Session ↔ held instance, a load in flight without the wait:** `stand_actions` puts a fetch in flight with `harness.click` on `load-fetch`, which waits on nothing, where a driver `click` would wait out `LOAD_WAIT` — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: integration-test-strategy: new detail beside #5.1, a fetch put in flight through the harness without LOAD_WAIT (F4)
rationale: F4; the report's Decisions (fct-cfi7tfzq) states the harness click waits on nothing while a driver click waits out LOAD_WAIT
@proposal n=10 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F1 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ccyc2gwa
section: ## 5. Integration Test Strategy
change: - **Session ↔ held instance, what can be done here:** `stand_actions` (8) holds, over the six served screens in both layout modes, the listed actionable nodes, their verbs and counts; every listed cause on the four lean tasks equal to what `Session::run` returns, with 3 `disabled` controls; the stated Cells and load-page causes; the flight booker's options reading `covered` with owner `flight-type` until the list opens; `crud-person-14` reading `off-screen` until a `scroll`; the fixtures reading `covered`, unkeyed and `fonts-unavailable`; a read changing nothing and holding no typed text or mask; and the whole Cells answer within the answer bound — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: integration-test-strategy: new detail beside #6.1, stand_actions' eight tests (F1)
rationale: F1; the report's Outcome (fct-ccyc2gwa, fct-ai45gnz6, fct-7utc4dyd, fct-64gpihqw) names the tests, and Gates reads stand_actions 8 passed
@proposal n=11 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F2 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-d57gqkg5
section: ## 5. Integration Test Strategy
change: - **The Cells actions answer's size:** the Cells `actions` answer measures 106,088 bytes for 678 nodes through the binary (`start cells` · `actions` · `stop`); 461 cells read `off-screen` for `click`, `type` and `screenshot`, and all 678 take `scroll`; `stand_actions` asserts the answer exceeds 100,000 bytes and stays within 1,048,576, in both layout modes — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: integration-test-strategy: new detail beside #6.1, the Cells actions answer measured and bounded (F2)
rationale: F2; the report's Counts moved (New, no prior site) and Decisions (fct-d57gqkg5) state the measure, and Outcome fct-64gpihqw the bound test in both modes
@proposal n=12 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F6 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-zylybc3q
section: ## 5. Integration Test Strategy
change: - **A kept actions step's replay, owed:** no test exercises the replay of a kept `actions` step — `Check::replay` runs it through `Session::run` by construction, unmeasured; the replay test is owed as a CARRY on «Handler coverage», and v030-06 reads verified only once that test passes — as stated at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md, Addendum 1
sidecar: integration-test-strategy: new detail beside #6.1, the owed replay test of a kept actions step (F6)
rationale: F6; the report's Outcome (fct-5opbm4ta) reads the criterion UNMET in part, and Addendum 1 (fct-zylybc3q, inputs#I6) makes the replay test an owed CARRY with v030-06 not verified until it passes
@proposal n=13 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-eja3jr5q fact=F5 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-a7grs7bs
section: ## 5. Integration Test Strategy
change: - **The actions answer holds no value:** `stand_actions` types a sentinel into `fixtures::password_labelled` and reads it 0 times in the `actions` answer, `MASKED_VALUE` also 0 times and no `NodeId` field written; the answer carries only ids, owner ids, the table's verb words, cause words and booleans — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: integration-test-strategy: new detail beside #8.1, the actions answer holds no value (F5)
rationale: F5; the report's Outcome (security, fct-a7grs7bs) and Schema / config (fct-wuaab5w2) state the 0 occurrences and the answer's closed content
@proposal n=14 detector=D-cascade severity=warning file=integration-test-strategy anchor=fct-5t64ong4 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-njnsyi3r
section: ## 5. Integration Test Strategy
change: add «; since 2026-11-08-what-can-i-do-here the test's body is the shared helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone`, the graph test keeping its name, and `an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone` runs the same assertions for an `actions` tool call — as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md»
sidecar: integration-test-strategy#7.1: the graph tool-call test's body became a shared read-verb helper the actions tool-call test also runs
rationale: the report's Deviations (fct-njnsyi3r) states the graph test body became the shared helper, the graph test keeping its name and the actions test running the same assertions
@proposal n=15 detector=D-cascade severity=warning file=integration-test-strategy anchor=fct-adkwyvky basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 5. Integration Test Strategy
change: each also drives one `lint`, one `snapshot` part read, one `graph` call and, since 2026-11-08-what-can-i-do-here, one `actions` call — `host_log` nine calls and 9 command-span lines, `mcp_log` twelve tool calls and 10 `command` lines — still reading 0 needles on stderr at `trace`, the graph's needles (`common::graph_needles`) and the actions needles (the CRUD screen's listed author-key ids, `common::actions_needles`) among them, as measured at escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md
sidecar: integration-test-strategy#9.2: host_log 8 to 9 calls, mcp_log 11 to 12 tool calls, one actions call each, actions_needles read beside graph_needles
rationale: the report's Counts moved states host_log's hosted calls 8 to 9 and mcp_log's tool calls 11 to 12, one actions call each, and the needle helpers gain actions_needles; Outcome (obs) reads 0 actions needles at trace under both builds
@proposal n=16 detector=D-cascade severity=warning file=body anchor=fct-gq2zcc44 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-ccyc2gwa
section: ## 5. Integration Test Strategy
change: after «… and changes nothing (`stand_graph`)» add «; the `actions` verb lists each served screen's actionable nodes with the table's verbs, every cause equal to what the call returns, and changes nothing (`stand_actions`)»
sidecar: test-plan body §5: the Session ↔ held instance broad line names the actions read beside the graph
rationale: the report's Outcome (fct-ccyc2gwa, fct-ai45gnz6) states stand_actions holds the listing, the causes equal to Session::run and a read that changes nothing, over the six served screens in both modes
@remark
F3 (ACTIONABLE_AT_BOOT) is test-data-fixtures', F7 (four dioxus-native-dom actionable unit tests) has no picked unit entity to carry it, F10/F11 are e2e-test-strategy's; proposals 3, 6, 7 overlap the verbs/instance/fieldkind/describe/cli-tests sweep rows (test-plan sites), apply once
