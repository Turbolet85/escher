@answer n=1 entity=conventions#1.1 v=no standing=S4
why: each new line passes cargo fmt, clippy with -D warnings and rustdoc -D warnings through the ci-leg.sh legs, with expect_used or unwrap_used allowed only beside a justification
@answer n=2 entity=conventions#2.1 v=no
why: the chunk adds no dependency and names no Cargo.toml edit, so the workspace dependency and manifest rules are not reached
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: each new module or file opens with a doc comment, public items carry doc comments, and constants state their rationale
@answer n=4 entity=conventions#4.1 v=no
why: the TODO and FIXME sites and the DO NOT REORDER block cited sit in files the chunk does not reach, and the chunk marks no known gap
@answer n=5 entity=conventions#5.1 v=no standing=S6
why: each new crate-internal item is pub(crate) or pub(super), and each new error type is an enum with Display and From per wrapped error
@answer n=6 entity=conventions#6.1 v=yes
why: the graph module (DioxusDocument::graph, UiGraph) in dioxus-native-dom is gated by accessibility, and the chunk's new read and GraphNode owner in graph.rs, reached whole, must sit under that gate
@answer n=7 entity=conventions#7.1 v=no
why: the chunk adds no unsafe block, and the render-into-MutationBuffer borrow rule lives in dioxus_document.rs, which the chunk does not reach, while its read takes no mutation
@answer n=8 entity=conventions#8.1 v=yes
why: the new verb and schema tests land in schema_tests.rs and mcp_tests.rs, and the new stand check that holds a Session joins the session-check list whose shared helpers sit in session_common
@answer n=9 entity=conventions#9.1 v=no standing=S1
why: each new test sits in a cfg(test) tests module at the end of its source file or a path child file, and the chunk adds no test module the path-child list must record
@answer n=10 entity=conventions#9.2 v=no standing=S2
why: each new stand fixture or shared page is stated once in tests/blitz-tests/tests/fixtures/mod.rs, a page with one reader staying in its check's file, and the minimal fixture has one reader
@answer n=11 entity=conventions#9.3 v=yes
why: the chunk adds a tool row to the MCP listing (mcp/listing.rs) that the listing test in mcp_tests.rs walks row by row, so the new row is asserted through private track_caller helpers naming the row and index only
@answer n=12 entity=conventions#10.1 v=no
why: the chunk touches no binary, example or stand source, and its fixtures are stand fixtures under tests rather than examples
@answer n=13 entity=conventions#11.1 v=no
why: the chunk adds no seven_guis task or signal and only uses the stand's use_graph_signal registrations
@answer n=14 entity=conventions#12.1 v=no
why: the HTML fixture forms under examples/assets are not reached, since the chunk's fixtures are stand fixtures
