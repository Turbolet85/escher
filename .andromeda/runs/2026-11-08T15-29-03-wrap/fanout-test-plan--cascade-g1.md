@proposal n=1 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 1. Test Scope Summary
change: After the `graph_needles()` clause add: and, since 2026-11-08-what-can-i-do-here, `actions_needles()`, the CRUD screen's listed author-key ids holding a `-` as needles, called by the same two, each making one `actions` call and reading 0 id, name and actions needles at `trace` under both builds.
sidecar: test-scope-summary apps entry: the log checks' needle helpers gain `actions_needles` beside `graph_needles` and `option_needles` (2026-11-08-what-can-i-do-here).
rationale: The report's Symbols / APIs names seven_guis common's `actions_needles()` (examples/seven_guis/tests/common/mod.rs:454-476), called by `host_log` and `mcp_log`, and its Counts list moves the needles pattern with `test-scope-summary.md:14` changed; Outcome (obs) reads 0 id, name and actions needles under both builds.
@proposal n=2 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: `tests/cli_commands.rs` (17 tests, unix only; 16 until 2026-11-08-what-can-i-do-here: ...; and since that chunk an `actions` answering one line of the actionable nodes against a started host, an argument to `actions` refused with status 1).
sidecar: test-scope-summary apps entry: cli_commands 16 -> 17 tests per OS, the `actions` command test added (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list moves `cli_commands` tests per OS 16 -> 17 (`test-scope-summary.md:15` `fct-k5y3m6cu` «16 tests» changed); the new test is `actions_answers_one_line_of_the_actionable_nodes` (cli_commands.rs:495-516) and Outcome (security) reads the argument refused with status 1.
@proposal n=3 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-k5y3m6cu basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: `tests/mcp_server.rs` (8 tests; 7 until 2026-11-08-what-can-i-do-here: ... `tools/list` offering the verbs in order, fourteen, `graph` after `lint` and `actions` after `graph` ...; and since that chunk an `actions` tool call answered with its line of the actionable nodes, an argument to it reading `isError`).
sidecar: test-scope-summary apps entry: mcp_server 7 -> 8 tests, tools/list fourteen with `actions` after `graph` (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list moves `mcp_server` tests 7 -> 8 (`test-scope-summary.md:16` «7 tests» changed) and VERBS 13 -> 14, «thirteen verbs / tools» -> fourteen; Outcome (tests) counts reads `tools/list` fourteen in table order and Outcome (security) the argument `isError` over `mcp_server`.
@proposal n=4 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: escher-driver: 102 unit tests in seventeen files, 66 inline and 36 in four `#[path]` test files (100 in sixteen, 64 inline, until 2026-11-08-what-can-i-do-here; ...), and in the per-file list, since that chunk, actions.rs 2 (every call named for a target verb validates; an id past its bound reads malformed).
sidecar: test-scope-summary escher-driver entry: unit tests 100 -> 102, actions.rs a seventeenth file with 2 inline tests (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list moves escher-driver unit tests 100 -> 102 («actions.rs 2, a seventeenth file in the per-file list», `test-scope-summary.md:161` changed); Gates read `cargo test -p escher-driver --locked` green, 102 passed; actions.rs holds `mod tests` at 138-158.
@proposal n=5 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: schema_tests.rs 7 (...; the verbs are named in order, each at its level - ten instance-level (`screenshot` the seventh, after `scroll`, `lint` the eighth, after `screenshot`, `graph` the ninth, after `lint`, and, since 2026-11-08-what-can-i-do-here, `actions` the tenth, after `graph`), then `start` · `status` · `stop`, then `help` at the tool level, fourteen; ...).
sidecar: test-scope-summary escher-driver entry: schema_tests names ten instance verbs and fourteen in all, `actions` the tenth after `graph` (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list moves instance verbs 9 -> 10 and VERBS 13 -> 14 and renames `the_thirteen_verbs_are_named_in_order_each_at_its_level` -> `the_fourteen_…`; Schema / config places `actions` the tenth instance verb, after `graph`.
@proposal n=6 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: mcp_tests.rs 15 (...; the listing is the fourteen verbs of the table, `graph` after `lint` and `actions` after `graph`, `snapshot`'s tool taking an optional integer `part` and naming its two counts; ...).
sidecar: test-scope-summary escher-driver entry: mcp_tests' listing is the fourteen verbs, `actions` after `graph` (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list renames `mcp_tests`' `the_listing_is_the_thirteen_verbs_of_the_table` -> `the_fourteen_…` and moves VERBS 13 -> 14; mcp.rs gains one line (mcp.rs:58) and mcp_tests.rs four (399 · 408 · 417-418), the test count unchanged.
@proposal n=7 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: describe.rs 2 (the document holds `help`'s seven fields in order; every text in it is its tables'; its count row reads `[14, 14, 12, 3, 8, 4, 15]`, `[13, 14, 12, 3, 8, 4, 15]` until 2026-11-08-what-can-i-do-here).
sidecar: test-scope-summary escher-driver entry: describe's count row [13, ...] -> [14, 14, 12, 3, 8, 4, 15] (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list moves describe's count row `[13, 14, 12, 3, 8, 4, 15]` -> `[14, 14, 12, 3, 8, 4, 15]` (describe pattern, 2 rows changed); Outcome (tests) counts reads describe `[14, 14, 12, 3, 8, 4, 15]`.
@proposal n=8 detector=D-tests-coverage severity=warning file=test-scope-summary anchor=fct-nks3g3e7 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-nnb75cma
section: ## 1. Test Scope Summary
change: After «the executor's verbs and its refusals are covered by the sixteen `stand_act_*` integration files (...)» add: and, since 2026-11-08-what-can-i-do-here, `Session::actions` and the shared per-verb target check `check_target` by `stand_actions` (8 tests over the six served screens, both layout modes), not by a unit test.
sidecar: test-scope-summary escher-driver entry: `Session::actions` and `check_target` are covered at integration by `stand_actions` (2026-11-08-what-can-i-do-here).
rationale: The report's Coverage of new surfaces reads `Session::actions` / `check_target` tests unit/integ ✓ through `stand_actions`; Outcome (matrix#v030-06) lists its tests and Gates read `--test stand_actions` green, 8 passed; the entry's list of what covers the executor names no `actions` read.
@proposal n=9 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 1. Test Scope Summary
change: dioxus-native-dom: 104 unit tests in eleven files (100 until 2026-11-08-what-can-i-do-here, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md) - ...
sidecar: test-scope-summary dioxus-native-dom entry: unit tests 100 -> 104 (2026-11-08-what-can-i-do-here).
rationale: The report's Counts list reads «dioxus-native-dom `actionable` unit tests 4 -> 8, the crate 100 -> 104» [unit-counts]; the sweep's unit-counts dispositions name no row at this entry, so its count was left at 100.
@proposal n=10 detector=D-cascade severity=warning file=test-scope-summary anchor=fct-i5a6vv3t basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-yftgk4an
section: ## 1. Test Scope Summary
change: eight in actionable.rs - four on the actionable-key check `DioxusDocument::unkeyed_actionable` (...), and, since 2026-11-08-what-can-i-do-here, four on the shared actionable rule read by it and by `DioxusDocument::actionable_nodes`, compiled only under the crate's default `accessibility` feature.
sidecar: test-scope-summary dioxus-native-dom entry: actionable.rs unit tests 4 -> 8, on the shared rule and `actionable_nodes` (2026-11-08-what-can-i-do-here).
rationale: The report's Symbols / APIs adds `actionable_nodes` and the crate-private `Readings` as the one rule both readers read; Counts reads `actionable` unit tests 4 -> 8, and Gates read `cargo test -p dioxus-native-dom --locked actionable` green, 8 passed.
@proposal n=11 detector=D-cascade severity=warning file=body anchor=fct-jydhmppz basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-nnb75cma
section: ## 1. Test Scope Summary
change: dioxus-native-dom: unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id and its reserved virtual form, the actionable-key check and the actionable nodes reader over their one rule, the bridge and its dispatch record, ... (the rest unchanged).
sidecar: test-scope-summary body: dioxus-native-dom's broad line names the actionable nodes reader beside the actionable-key check (2026-11-08-what-can-i-do-here).
rationale: The report's Coverage of new surfaces reads `DioxusDocument::actionable_nodes` and the shared rule tests unit ✓ (eight `actionable::tests`); the body's broad line names only the actionable-key check.
@remark
F1-F2 F4-F6 (integration-test-strategy), F3 (test-data-fixtures), F7-F9 (unit-test-strategy), F10-F11 (e2e-test-strategy) are no test-scope-summary entity's; §3 body facts outside this group also move: fct-prxcxiqp (session_common readers 33->34, actions()/actioned()), fct-osorhgow and fct-6bs5asmb (fixtures readers 31->32, stand_* 23->24), fct-og7dxjok (ACTIONABLE_AT_BOOT), fct-glytb3w3/fct-6bs5asmb (run stand 253·0·7 over 53), fct-63656bwh/fct-szqqqnrt (thirteen verbs -> fourteen); D-tests-obs-harness: no harness change (report fct-tayahs5z), no drift.
