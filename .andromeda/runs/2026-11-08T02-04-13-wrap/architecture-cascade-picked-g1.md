@account n=1 entity=design-philosophy#1.1 v=not_touched
why: dioxus-native-dom stays the headless Dioxus renderer and the report adds modules to it without changing what the claim says of any crate
@account n=2 entity=design-philosophy#2.1 v=not_touched
why: the report states the bridge's new dispatch does nothing without a registry and leaves `poll`, so the integration through `handle_ui_event` and `poll` still holds
@account n=3 entity=stack-and-technologies#1.22 v=not_touched
why: cold-agent.sh and test_cold_agent.py only gain `graph` in their `VERBS`, no package or tool is added (Dependencies none), so the CI stack it names still holds
@account n=4 entity=established-decisions#11.10 v=not_touched
why: the select list's limits are unchanged, `clickable` is a separate reading and the highlight is still absent, and only the capabilities.md lines moved, which is a citation matter
@account n=5 entity=established-decisions#14.3 v=not_touched
why: the IME-on-mount behaviour and its test still hold, since the report's dioxus_document.rs hunks touch only dispatch and the test-only `dispatch_to_vdom`
@account n=6 entity=established-decisions#15.1 v=proposed
why: the new snapshot reading `NodeState.clickable` reads `ElementData::is_disabled`, so it joins the readers of the one `disabled` reading
@account n=7 entity=established-decisions#24.1 v=proposed
why: the bridge's `handle_event` now sets a registry's dispatch around `runtime.handle_event` on every Dioxus document, doing nothing where no state is registered, which is a new Dioxus-integration decision
@account n=8 entity=established-decisions#25.1 v=not_touched
why: the report states the buffered render, `initial_build`, `poll` and `flush_queued_mounted_events` are untouched in dioxus_document.rs, so the claim holds on the changed file
@account n=9 entity=established-decisions#25.3 v=not_touched
why: the dioxus-core polling order is unchanged, and the related measured fact F9 is placed in cross-cutting-patterns, not here
@account n=10 entity=established-decisions#26.2 v=proposed
why: hook-local state registered through `use_graph_state` now has readers and writes reachable through public dioxus-core API with no fork-level change, which contradicts "not reachable"
@account n=11 entity=established-decisions#30.1 v=proposed
why: the `graph` verb's names and the `clickable` node key cross the ratified carriers, a boundary widening ratified by the founder (inputs#I4, inputs#I9)
@account n=12 entity=established-decisions#30.2 v=not_touched
why: the row states the select list's own crossing, and `clickable` is a node key of every node whose crossing is placed under the 30.1 addition, not the select list's
@account n=13 entity=established-decisions#31.1 v=proposed
why: a kept check now also holds a `graph` answer's names at rest, because the recording routes every verb through `VERBS` and `Outcome::to_json`
@account n=14 entity=conventions#2.1 v=not_touched
why: no manifest changed (the report's Dependencies gate is green), so the manifest conventions hold
@account n=15 entity=conventions#6.1 v=proposed
why: dioxus-native-dom gains the `graph` module gated by `accessibility` and the ungated `signal_registry`
@account n=16 entity=conventions#7.1 v=not_touched
why: the report states the buffered render is untouched, so the borrow workaround it describes holds
@account n=17 entity=conventions#8.1 v=proposed
why: the new Session-holding check `stand_graph` shares through `session_common` and is missing from the list of checks reading it
@account n=18 entity=conventions#9.1 v=not_touched
why: cli_tests.rs and recording_tests.rs gain cases but stay `#[path]` children with unchanged test paths
@account n=19 entity=conventions#9.3 v=not_touched
why: the MCP listing test and the requirement sweep keep their row walk and `#[track_caller]` helpers, and only one count line and one test name moved
@account n=20 entity=conventions#10.1 v=proposed
why: F10's `const { &[...] }` declaration form and the seven_guis tasks' state registered through `use_graph_signal` are new app conventions
