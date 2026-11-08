@answer n=1 entity=existing-scopes#1.26 v=yes
why: the escher-driver unit's VERBS schema, Session::run and the action_point path, json, cli, mcp, recording and replay are the files the chunk edits
@answer n=2 entity=existing-scopes#1.27 v=no
why: the browser app modules sit outside the driver, the Dioxus bridge reads and the stand, which the chunk reaches
@answer n=3 entity=existing-scopes#1.28 v=no
why: the browser-persistence HistoryStore is not reached by the driver read
@answer n=4 entity=existing-scopes#1.29 v=no
why: the rdme markdown viewer is outside the chunk's scope
@answer n=5 entity=existing-scopes#1.30 v=no
why: the WPT runner is neither run nor changed by the chunk
@answer n=6 entity=existing-scopes#1.31 v=yes
why: the stand check inventory holds the session builders, stand_graph, the stand_session and stand_replay checks and stand_requirement_sweep, where the new read's checks sit and the shared builders it adds to
@answer n=7 entity=existing-scopes#1.32 v=yes
why: the escher-session binary's command line and its seven_guis tests (cli_commands, cli_flow, mcp_server, recording) must honour the new verb, usage and exit codes
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates are outside the driver, the bridge and the stand
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root example files are outside the chunk's reach
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixture pages under examples/assets are excluded, the chunk's fixtures being stand fixtures
@answer n=11 entity=existing-scopes#2.1 v=yes
why: the Cells screen's size and its six-part walk bound the new read's answer on the Cells stand task, which the read is proven on
@answer n=12 entity=existing-scopes#3.1 v=no standing=S1
why: placing unit tests of cli and recording in cfg(test) path children at the end of a source file is the convention S1 names
@answer n=13 entity=existing-scopes#3.2 v=yes
why: the chunk adds a minimal stand fixture and a session builder for the read, which the shared-module record in fixtures/mod.rs and session_common must hold
@answer n=14 entity=existing-scopes#3.3 v=no
why: the predicate reads only the 0.2.0 witness record, which the chunk does not add to
@answer n=15 entity=existing-scopes#3.4 v=no
why: the blitz-vibey-script runtime is not reached, the read touching no script
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo-element flow classification is layout internals the chunk does not change
@answer n=17 entity=existing-scopes#4.1 v=yes
why: visible_region_transformed and hit_non_invertible_transform state the visible-region reading and the hit stopping rule the scope names for the read's refusal path
@answer n=18 entity=existing-scopes#4.2 v=no
why: the probe's spawned-focus witness of the buffered render is not named by the scope and the read does not touch focus
@answer n=19 entity=existing-scopes#4.3 v=no
why: the buffered-render borrow witness governs calls made during a render, and the read runs between steps, not during one
