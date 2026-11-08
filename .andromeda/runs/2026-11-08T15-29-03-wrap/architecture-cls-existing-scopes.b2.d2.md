@answer n=1 entity=existing-scopes#1.26 v=yes mode=stale
why: the session adds the private module actions and its ActionNode and VerbReading re-exports, moving the seventeen private modules to eighteen and citing lib.rs which the session reaches
@answer n=2 entity=existing-scopes#1.27 v=no
why: the session reaches no browser file and the apps/browser module list is unchanged
@answer n=3 entity=existing-scopes#1.28 v=no
why: the session reaches no browser-persistence file and HistoryStore is unchanged
@answer n=4 entity=existing-scopes#1.29 v=no
why: the session reaches no rdme file and the markdown viewer is unchanged
@answer n=5 entity=existing-scopes#1.30 v=no
why: the session reaches no wpt runner file and its modules and kinds are unchanged
@answer n=6 entity=existing-scopes#1.31 v=yes mode=stale
why: the session reaches stand_act_spans, stand_act_scroll, stand_mcp_tools, stand_requirement_sweep, session_common and common, so the cited line ranges and the session-reader list in those reached files can have moved
@answer n=7 entity=existing-scopes#1.32 v=yes mode=add-to
why: the session adds actions_needles beside graph_needles and option_needles in examples/seven_guis/tests/common/mod.rs, which this entity names among the common module's readers
@answer n=8 entity=existing-scopes#1.33 v=no
why: the session reaches no separate example crate and the counter, todomvc, transparent, wasm_hello and wgpu_texture crates are unchanged
@answer n=9 entity=existing-scopes#1.34 v=no
why: the session reaches no root example file and the examples other than the seven_guis tests are not reached
@answer n=10 entity=existing-scopes#1.35 v=no
why: the session reaches no HTML fixture and fixtures are not reached
@answer n=11 entity=existing-scopes#2.1 v=no
why: stand_cells_served and stand_load_page are not reached and their claim of reading session_common still holds on the changed file
@answer n=12 entity=existing-scopes#3.1 v=no
why: cli_tests.rs and recording_tests.rs remain #[path] children and the module wiring of cli.rs and recording.rs is not changed
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: the new stand_actions file moves session_common declarers from 33 to 34, fixtures declarers from 31 to 32, and stand files from 23 to 24
@answer n=14 entity=existing-scopes#3.3 v=yes mode=stale
why: the session reaches stand_requirement_sweep.rs, where the cited line 1097 of the inside_stand_repeat predicate can have moved
@answer n=15 entity=existing-scopes#3.4 v=no
why: the session reaches no blitz-vibey-script file and its runtime and dom modules are unchanged
@answer n=16 entity=existing-scopes#3.5 v=no
why: the session reaches no pseudo_element_flow_classification file and tests/all.rs is not reached
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets sit on engine behaviour the session leaves unchanged, and none of their files is reached
@answer n=18 entity=existing-scopes#4.2 v=no
why: dioxus_runtime_probe.rs is not reached and its claims about Dioxus public API and the buffered render do not read the actionable rule
@answer n=19 entity=existing-scopes#4.3 v=no
why: dioxus_document.rs is not reached and its tests and their mutation control do not read the actionable rule or the listener leg
