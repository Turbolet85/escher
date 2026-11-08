@answer n=1 entity=quality-gates-coverage-targets#1.0 v=no
why: the dated first line-coverage readings at earlier chunks are historical and the session brings no workspace reading.
@answer n=2 entity=quality-gates-coverage-targets#2.1 v=yes mode=stale
why: graph.rs and signal_registry.rs join dioxus-native-dom inside its unchanged 83 % floor, so the read-now per-crate readings predate the session's new code.
@answer n=3 entity=quality-gates-coverage-targets#3.1 v=no
why: mutation_buffer.rs is not a file the change reaches and the mutation buffer is byte-equal to upstream.
@answer n=4 entity=quality-gates-coverage-targets#4.1 v=yes mode=stale
why: the stand_ glob now takes stand_graph, so the stand-repeat leg runs 52 stand files, not the 51 the entity counts.
@answer n=5 entity=quality-gates-coverage-targets#5.1 v=no
why: the scoped mutation runs cover timer.rs, mcp/framing.rs and mutation_writer.rs, none of which the session reaches.
@answer n=6 entity=quality-gates-coverage-targets#6.1 v=yes mode=stale
why: mcp_tests.rs is reached and its named test reads the_listing_is_the_twelve_verbs_of_the_table while the MCP listing now holds thirteen tools.
