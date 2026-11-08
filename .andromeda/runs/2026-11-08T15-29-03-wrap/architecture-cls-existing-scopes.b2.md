@answer n=1 entity=existing-scopes#1.26 v=yes mode=stale
why: the session adds the private actions module (seventeen becomes eighteen), Command::Actions and Outcome::Actions, and the ActionNode and VerbReading re-exports this scope's lib.rs and execute holdings describe
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is not reached by the session
@answer n=3 entity=existing-scopes#1.28 v=no
why: apps/browser/persistence is not reached by the session
@answer n=4 entity=existing-scopes#1.29 v=no
why: apps/readme is not reached by the session
@answer n=5 entity=existing-scopes#1.30 v=no
why: wpt/runner is not reached by the session
@answer n=6 entity=existing-scopes#1.31 v=yes mode=stale
why: its cited lines sit in reached files (stand_act_spans, stand_act_scroll, stand_mcp_tools, stand_requirement_sweep, session_common and common), and the session adds stand_actions as a session_common reader, moving that count from 33 to 34
@answer n=7 entity=existing-scopes#1.32 v=yes mode=stale
why: its cited lines in the reached seven_guis common/mod.rs and host_log.rs can move with the new actions_needles beside graph_needles, and the reached cli_commands, mcp_server and mcp_log tests move their counts
@answer n=8 entity=existing-scopes#1.33 v=no
why: the example crates counter, todomvc, transparent, wasm_hello and wgpu_texture are not reached by the session
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root examples are not reached by the session
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures are not reached by the session
@answer n=11 entity=existing-scopes#2.1 v=no
why: stand_cells_served and stand_load_page are not reached, and both still read session_common/mod.rs with their counts unchanged
@answer n=12 entity=existing-scopes#3.1 v=no
why: the cli_tests.rs and recording_tests.rs #[path] children and their cli.rs and recording.rs attachments are unchanged, the session only adding cases inside them
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: the session moves the session_common declarers from 33 to 34, the fixtures declarers from 31 to 32 and their stand files from 23 to 24
@answer n=14 entity=existing-scopes#3.3 v=yes mode=stale
why: stand_requirement_sweep.rs is reached and its cited line 1097 for inside_stand_repeat can move with the witness-row changes
@answer n=15 entity=existing-scopes#3.4 v=no
why: the blitz-vibey-script runtime modules are not reached by the session
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification is not reached by the session
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets are not reached and no engine file changed
@answer n=18 entity=existing-scopes#4.2 v=no
why: dioxus_runtime_probe is not reached by the session and its claims do not read the actionable rule
@answer n=19 entity=existing-scopes#4.3 v=no
why: dioxus_document.rs is not reached by the session and its test set does not move
