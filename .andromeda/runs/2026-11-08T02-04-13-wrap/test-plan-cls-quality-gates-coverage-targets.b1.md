@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: its readings are dated measurements at the 2026-10-05 and 2026-10-14 chunks, and the session moves no figure of those dates
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes mode=stale
why: the session adds graph.rs and signal_registry.rs to dioxus-native-dom and extends the reached escher-driver tests, so the current per-crate counts read at CI#42371051720 can move
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: mutation_buffer.rs is byte-unchanged and the 83 % floor still counts it with no exclusion, so its reading's claim holds on the changed tree
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes mode=stale
why: the stand-repeat leg's stand_ glob now takes stand_graph, so the 51 stand_*.rs files it runs read 52
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the mutation tier is unchanged and timer.rs, mcp/framing.rs, mutation_writer.rs and the nextest profile lie outside the session's change
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=yes mode=stale
why: mcp_tests.rs and stand_requirement_sweep.rs, both reached files, hold two of the named functions, and the count of 85 functions over the ceiling can move with the session's new functions
