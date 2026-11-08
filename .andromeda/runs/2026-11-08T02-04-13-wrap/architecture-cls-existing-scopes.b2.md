@answer n=1 entity=existing-scopes#1.26 v=yes mode=add-to
why: the session adds the graph verb as the ninth instance verb, with Command::Graph, Outcome::Graphed and the clickable node key, in escher-driver's schema, command, execute and json
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is not among the files the session reaches
@answer n=3 entity=existing-scopes#1.28 v=no
why: apps/browser/persistence is not among the files the session reaches
@answer n=4 entity=existing-scopes#1.29 v=no
why: apps/readme is untouched by the chunk and is not among the files the session reaches
@answer n=5 entity=existing-scopes#1.30 v=no
why: wpt/runner is untouched by the chunk and is not among the files the session reaches
@answer n=6 entity=existing-scopes#1.31 v=yes mode=stale
why: stand_graph joins the blitz-tests stand checks and GRAPH_DRIVES lands in the reached tests/common/mod.rs, so the stated count of 21 readers of common moves
@answer n=7 entity=existing-scopes#1.32 v=yes mode=add-to
why: every seven_guis task's signals now register through tasks::use_graph_signal in the reached tasks/mod.rs, and the reached cli_commands and mcp_server targets carry the new graph verb
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates counter, todomvc, transparent, wasm_hello and wgpu_texture are not among the files the session reaches
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root example files are not among the files the session reaches
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures under examples/assets are not among the files the session reaches
@answer n=11 entity=existing-scopes#2.1 v=yes mode=stale
why: the Cells screen the session measures moves from 47,089 bytes in 5 parts to 53,859 bytes in 6 parts, still 740 lines
@answer n=12 entity=existing-scopes#3.1 v=yes mode=stale
why: recording_tests.rs, one of the named #[path] children, is a file the session reaches whole, so the claim about it can have moved
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: tests/common/mod.rs and tests/session_common/mod.rs are reached, and the stand checks reading common move from 21 with stand_graph joining
@answer n=14 entity=existing-scopes#3.3 v=no
why: stand_requirement_sweep and the stand-repeat glob in ci-leg.sh are not among the files the session reaches, and the inside_stand_repeat prefixes are unchanged
@answer n=15 entity=existing-scopes#3.4 v=no
why: blitz-vibey-script is not among the files the session reaches
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification and the unbuilt all.rs are not among the files the session reaches
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets are not among the files the session reaches and blitz-dom is untouched
@answer n=18 entity=existing-scopes#4.2 v=no
why: dioxus_runtime_probe is not among the files the session reaches, the buffered render is unchanged, and the probe's 10 tests stay green with no registry present
@answer n=19 entity=existing-scopes#4.3 v=yes mode=stale
why: dioxus_document.rs is a file the session reaches whole, and the cited lines 867-905 can have moved with the handle_event change
