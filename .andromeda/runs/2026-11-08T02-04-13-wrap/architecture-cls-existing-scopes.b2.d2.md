@answer n=1 entity=existing-scopes#1.26 v=yes mode=stale
why: the verb table gains graph, execute gains Outcome::Graphed, the node-key list gains clickable, and the citation lib.rs:131-168 sits in the reached lib.rs.
@answer n=2 entity=existing-scopes#1.27 v=no
why: apps/browser is not reached by the change and none of its modules is edited.
@answer n=3 entity=existing-scopes#1.28 v=no
why: apps/browser/persistence is not reached by the change and holds no edited file.
@answer n=4 entity=existing-scopes#1.29 v=no
why: rdme (apps/readme) is untouched by the chunk and none of its files is reached.
@answer n=5 entity=existing-scopes#1.30 v=no
why: the WPT runner is untouched by the chunk and none of its files is reached.
@answer n=6 entity=existing-scopes#1.31 v=yes mode=stale
why: tests/common/mod.rs, tests/session_common/mod.rs, stand_non_lean_tasks.rs and stand_mcp_tools.rs are reached, so the 21 mod common readers and the cited lines can move with the new stand_graph file.
@answer n=7 entity=existing-scopes#1.32 v=yes mode=stale
why: the seven_guis task files, tasks/mod.rs (cited at 22-28), cli_commands.rs and mcp_server.rs are reached, so the task-list citation and the test descriptions can move.
@answer n=8 entity=existing-scopes#1.33 v=no
why: the separate example crates under examples/ are not reached by the change.
@answer n=9 entity=existing-scopes#1.34 v=no
why: the root examples/*.rs files are not reached by the change.
@answer n=10 entity=existing-scopes#1.35 v=no
why: the HTML fixtures under examples/assets are not reached by the change.
@answer n=11 entity=existing-scopes#2.1 v=yes mode=stale
why: the Cells screen moves from 47,089 to 53,859 bytes and from 5 to 6 parts, per task item 14.
@answer n=12 entity=existing-scopes#3.1 v=no
why: cli.rs and recording.rs, which declare the #[path] test children, are unreached and recording.rs is unedited, so recording_tests.rs changes only its test bodies.
@answer n=13 entity=existing-scopes#3.2 v=yes mode=stale
why: tests/common/mod.rs and tests/session_common/mod.rs are reached, so their cited lines and the 21, 32 and 31 reader counts can move with stand_graph and GRAPH_DRIVES.
@answer n=14 entity=existing-scopes#3.3 v=no
why: stand_requirement_sweep.rs and the stand-repeat glob are unedited and the 0.2.0 witness predicate is unchanged.
@answer n=15 entity=existing-scopes#3.4 v=no
why: blitz-vibey-script is untouched and none of its runtime or dom files is reached.
@answer n=16 entity=existing-scopes#3.5 v=no
why: pseudo_element_flow_classification tests blitz-dom, which is untouched, and none of its files is reached.
@answer n=17 entity=existing-scopes#4.1 v=no
why: the engine-fact targets test blitz-dom behaviour, which is untouched, and none of their files is reached.
@answer n=18 entity=existing-scopes#4.2 v=no
why: dioxus_runtime_probe.rs is unreached, the buffered render is unchanged, and the task reports its 10 tests green.
@answer n=19 entity=existing-scopes#4.3 v=yes mode=stale
why: dioxus_document.rs is reached and the cited test lines 867-905 in it can move with DioxusDocument::graph and the event dispatch change.
