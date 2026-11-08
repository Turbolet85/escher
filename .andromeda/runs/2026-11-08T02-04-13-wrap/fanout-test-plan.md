@proposal n=1 detector=D-tests-obs-harness severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: §3 Test Harness Contract
change: In fct-63656bwh (cold-agent server and judge) the driver's MCP surface is `escher-session mcp` (the thirteen verbs, `graph` among them since 2026-11-07-ui-graph), and `valid_step` admits only `tool` ∈ the thirteen verbs; `graph` is not required of a cold agent, so the `capability-unused` set is unchanged.
sidecar: §3 cold-agent pipe: the server's verbs and `valid_step`'s admitted tools read thirteen, `graph` joining `VERBS` in `scripts/cold-agent.sh` (2026-11-07-ui-graph).
rationale: The report's Harness / gate surface (fct-rsklenkh) states `scripts/cold-agent.sh` and `.github/scripts/test_cold_agent.py` `VERBS` gain `graph` (thirteen) with the `capability-unused` set unchanged; fct-63656bwh still states twelve verbs twice.
@proposal n=2 detector=D-tests-obs-harness severity=warning dependent_of=D-tests-obs-harness basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: §3 Test Harness Contract
change: In fct-szqqqnrt (cold-agent verdict) `isolated` = the init tools exactly the thirteen `mcp__escher__{verb}` tools (twelve until 2026-11-07-ui-graph), one MCP server `escher` connected, the rest unchanged.
sidecar: §3 cold-agent verdict: `isolated` reads the thirteen `mcp__escher__*` tools since 2026-11-07-ui-graph.
rationale: Same retired claim as n=1: the cold-agent `VERBS` gain `graph` (report fct-rsklenkh), so the isolation check's tool set is thirteen; fct-szqqqnrt states the twelve as standing (the dated live-run proof fct-u5f4aal7 stays historical).
@proposal n=3 detector=D-tests-obs-harness severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: §3 Test Harness Contract
change: fct-glytb3w3 (Proof, the newest reading) reads: `run stand`'s `run.end` passed 244 · failed 0 · ignored 7 over 52 files, +7 passed with `stand_graph`, a new file its `stand_` prefix picks up with no script change; `run all` not re-counted — as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md.
sidecar: §3 agent-run proof: newest `run stand` reading 237 · 0 · 7 over 51 files → 244 · 0 · 7 over 52 files (`stand_graph` new, 2026-11-07-ui-graph).
rationale: Report Counts / qualifiers moved (fct-ubggv3nh): `run-stand` 237 · 0 · 7 → 244 · 0 · 7 stale at test-plan's master row, `stand-files` 51 → 52 stale for the current count; Harness / gate surface (fct-rsklenkh) names the new `stand_graph` target taken by the `stand_*` glob; agent-run.sh is not in the Files list.
@proposal n=4 detector=D-tests-obs-harness severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: §3 Test Harness Contract
change: fct-xxpbx2ay (Stand log format) adds: since 2026-11-07-ui-graph `graph` is a word of `tool_call`'s `tool` and `command`'s `verb` closed sets, no span field added — a graph call's `command` line records `verb` alone, its `tool_call` the parent, and the host's and the `mcp` role's stderr at `trace` hold 0 id, component-name and state-name needles under both builds (`host_log`, `mcp_log`).
sidecar: §3 stand log format: `graph` joins the `tool_call` / `command` closed sets, `verb` alone, 0 graph needles at trace (2026-11-07-ui-graph), agreeing with obs-plan §4 / §6.
rationale: Report Coverage of new surfaces (fct-qxguy4c4): the `command` span records `verb` alone and its `tool_call` parents it; `host_log` and `mcp_log` read 0 id, name and graph needles at `trace`, both builds; Outcome (obs) fct-ctk4yyh6; Expected amendments (fct-qyqo7z6u) carry the same words into obs-plan §4 / §6, so §3 must say it too to stay consistent.
@proposal n=5 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qxguy4c4
section: §1 Test Scope Summary
change: fct-jydhmppz (dioxus-native-dom) also lists the UI graph reader (`graph::tests`), the signal registry (`signal_registry::tests`), the bridge's registry dispatch and the snapshot's `clickable` reading among its unit tests.
sidecar: §1 dioxus-native-dom: unit coverage gains the graph reader, the signal registry, the registry dispatch and `clickable` (2026-11-07-ui-graph).
rationale: Report Crates / modules (fct-z2vdklsi) adds the `graph` and `signal_registry` modules; Coverage of new surfaces (fct-qxguy4c4) names their unit tests (`signal_registry::tests`, `bridge_tests::a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is`); New text lists `graph.rs` tests 333-671 and `snapshot.rs` 1038-1059.
@proposal n=6 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: §3 Test Harness Contract
change: fct-i47d76zr (dioxus-native-dom helpers) adds: since 2026-11-07-ui-graph a test-only `DioxusDocument::dispatch_to_vdom` (`#[cfg(test)]`) dispatches a chain into the vdom for the bridge test `a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is` (packages/dioxus-native-dom/src/dioxus_document.rs:240-250; packages/dioxus-native-dom/src/dioxus_document_tests.rs:269-322).
sidecar: §3 crate-local helpers: dioxus-native-dom gains the test-only `dispatch_to_vdom` for the registry-dispatch bridge test (2026-11-07-ui-graph).
rationale: Report Symbols / APIs (fct-zklbw7ne): «A test-only `DioxusDocument::dispatch_to_vdom` was added (`#[cfg(test)]`)»; New text lists dioxus_document.rs 240-250 and dioxus_document_tests.rs 269-322.
@proposal n=7 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ej52fywd
section: §3 Test Harness Contract
change: fct-og7dxjok (blitz-tests stand checks) adds the table `GRAPH_DRIVES` with its `Drive` enum (each stand task's drives, seven rows, every conditional write driven once so no stand write reads `unconfirmed`) to the tables `tests/common/mod.rs` states (tests/blitz-tests/tests/common/mod.rs:233-323).
sidecar: §3 stand checks' shared module: gains `Drive` and `GRAPH_DRIVES` (2026-11-07-ui-graph).
rationale: Report Decisions (fct-ej52fywd): the stand's drive table `GRAPH_DRIVES` in `tests/common/mod.rs` drives each conditional write once; New text lists common/mod.rs 233-323 (`Drive` 234-241, `GRAPH_DRIVES: [(&str, &[Drive]); 7]` 243-323).
@proposal n=8 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zkn5zjmx
section: §3 Test Harness Contract
change: fct-prxcxiqp (blitz-tests session checks) adds to the driver's call builders and readers, since 2026-11-07-ui-graph, `graph()` and `graphed(session, call)` (a graph's `UiGraph`, `None` for a refused call or any other outcome) (tests/blitz-tests/tests/session_common/mod.rs:134-147).
sidecar: §3 session checks' shared module: gains the `graph()` builder and the `graphed` reader (2026-11-07-ui-graph).
rationale: Report Files (fct-zkn5zjmx) lists `session_common/mod.rs` changed; New text lists 134-137 `pub fn graph() -> Call` and 139-146 `pub fn graphed(session: &mut Session, call: &Call) -> Option<UiGraph>`.
@proposal n=9 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-e44heebj
section: §5 Integration Test Strategy
change: fct-gq2zcc44 (Session ↔ held instance) adds: since 2026-11-07-ui-graph `stand_graph` holds the `graph` verb equal to `DioxusDocument::graph` on every served screen in both layout modes — every graph node a snapshot node, no served id holding `///`, each task's declarations agreeing with its code once `GRAPH_DRIVES` ran, and a graph read changing nothing and holding no typed text.
sidecar: §5 Session ↔ held instance: the `graph` verb's integration check `stand_graph` (2026-11-07-ui-graph).
rationale: Report Outcome (fct-e44heebj, fct-dwh3o3se) and Gates (fct-5gqyzpdy, `stand_graph` 6 passed); New text lists stand_graph.rs tests 106-171, 173-211, 213-280, 282-339, 341-379, 381-394.
@proposal n=10 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: §3 → Session lifecycle
change: session-binary (fct-76fzwvj7) reads thirteen verbs — the nine instance-level ones, `graph` the ninth since 2026-11-07-ui-graph, after `lint`, no argument (one refused `malformed`), its accepted answer one line `{"nodes":[…],"states":[…]}`, status `0` — and the usage line gains `graph` after `lint`.
sidecar: §3 → Session lifecycle, session-binary: twelve verbs / eight instance-level → thirteen / nine, `graph` after `lint` (2026-11-07-ui-graph).
rationale: Report Schema / config (fct-yr4375o3): `VERBS` 12 → 13, instance verbs 8 → 9, the usage line gains `| graph |` after `lint`, `escher-session` answers `graph` on the command line; Coverage (fct-qxguy4c4): an argument is refused `malformed`; Counts moved (fct-ubggv3nh) marks session-lifecycle.md's `verbs` row stale.
@proposal n=11 detector=D-tests-coverage severity=warning dependent_of=D-tests-coverage basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-zklbw7ne
section: §3 → Session lifecycle
change: session-start (fct-3ylhu62k) reads: `snapshot`, `screenshot`, `lint` and, since 2026-11-07-ui-graph, `graph` run no step and wait on nothing, `graph` returning `Outcome::Graphed { graph }`; and `run` records each answered call of the nine instance verbs, a `lint` and a `graph` among them.
sidecar: §3 → Session lifecycle, session-start: the eight instance verbs → nine, `graph` running no step (2026-11-07-ui-graph).
rationale: Same retired count as n=10 (the eight instance verbs): report Symbols / APIs (fct-zklbw7ne) — `Outcome::Graphed { graph: UiGraph }` runs no step, the recording routes every verb through `VERBS`; Schema / config (fct-yr4375o3) — `escher-session` answers `graph` in a kept check; Counts moved (fct-ubggv3nh) `inst-verbs` stale.
@proposal n=12 detector=D-tests-coverage severity=warning basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-5gqyzpdy
section: §3 → Session lifecycle
change: session-proof (fct-egnkfdyv) adds, since 2026-11-07-ui-graph, `stand_graph` 6 (`Session::run` in both layout modes, the verb equal to `DioxusDocument::graph`), and names the graph checks `cli_commands::graph_answers_one_line_of_the_ui_graph`, `mcp_server::a_graph_tool_call_answers_its_line_of_the_ui_graph` and `stand_mcp_tools::a_graph_tool_call_answers_the_readers_graph_and_parents_a_command_recording_its_verb_alone`; escher-driver's unit count unmoved at 100.
sidecar: §3 → Session lifecycle, session-proof: `stand_graph` 6 and the three graph checks over the command line and MCP (2026-11-07-ui-graph).
rationale: Report Gates (fct-5gqyzpdy): `stand_graph` green (6 passed), `cargo test -p escher-driver` 100 passed; Outcome (fct-e44heebj) names the three graph checks; Counts moved (fct-ubggv3nh) `ed-units` no change.
@proposal n=13 detector=D-tests-coverage severity=escalate basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qyqo7z6u
section: §3 → Session lifecycle
change: session-wire (fct-2k7iskpv) adds: since 2026-11-07-ui-graph a `graph` call's answer carries the screen's UI graph out — ids, component names, registered state names, handler event names, closed flag words and booleans, never a value — ratified by the founder (P4 fork 1 «Новый глагол graph (Рекомендую)», relayed, inputs#I4).
sidecar: §3 → Session lifecycle, session-wire: the UI graph's names cross the socket (a ratified boundary widening, 2026-11-07-ui-graph).
rationale: Report Expected amendments (fct-qyqo7z6u) records the graph's names crossing the socket, the command line, MCP and kept checks as a boundary widening ratified at P4 (inputs#I4); Outcome (security, fct-dbzqvx2q) states the answer's contents; Addendum 1 (fct-zyezjtli) halts the widening on one Validate card with one confirm, hence escalate.
@remark
text: the click-reading Retired line (fct-vy7otued) has no occurrence in test-plan; the dated live-run proof fct-u5f4aal7 («the twelve mcp__escher__* tools») and fct-6bs5asmb («run stand 237 · 0 · 7 over 51 files, as measured at 2026-10-31-select-popup») are historical readings and were left unproposed; count rows in category files (test-scope-summary, unit-test-strategy, ci-integration, test-data-fixtures) are the cascade's.
