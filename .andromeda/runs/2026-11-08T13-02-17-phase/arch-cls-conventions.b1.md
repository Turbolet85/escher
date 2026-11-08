@answer n=1 entity=conventions#1.1 v=no standing=S4
why: cargo fmt, clippy with -D warnings and rustdoc -D warnings through the ci-leg.sh legs, with expect_used and unwrap_used allowed only beside a justification, is the convention S4 names
@answer n=2 entity=conventions#2.1 v=no
why: workspace dependency declaration and package metadata; the chunk adds no crate or dependency and reaches no manifest
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the module and file doc comments, public item doc comments and constant rationale is the convention S3 names
@answer n=4 entity=conventions#4.1 v=no
why: TODO and FIXME markers, the DO NOT REORDER block and the section markers sit in files the chunk does not reach, and the chunk marks no known gap of its own
@answer n=5 entity=conventions#5.1 v=no standing=S6
why: crate-internal items pub(crate) or pub(super) is the convention S6 names; the rest (blitz-vibey-script re-exports, Dummy and __blitz_ names, non_exhaustive, stylo aliases) sits in files the chunk does not edit
@answer n=6 entity=conventions#6.1 v=yes
why: the graph module (DioxusDocument::graph and UiGraph) is gated by accessibility beside the snapshot model, and the new read calls through it, so the driver build must enable that feature; the ungated escher-driver tracing part is the convention S5 names
@answer n=7 entity=conventions#7.1 v=no
why: the bridge's MutationBuffer and RefCell discipline governs vdom rendering that the read never runs (it runs no step and dispatches nothing), and the other unsafe and borrowing rules sit in files the chunk does not reach
@answer n=8 entity=conventions#8.1 v=yes
why: the scope names the stand check in both layout modes over a minimal fixture, and this entity's own list of Session-holding checks (session_common) must record the new session check
@answer n=9 entity=conventions#9.1 v=yes
why: the chunk edits cli.rs and recording.rs, and the tests it adds to cli::tests and recording::tests, including the unix and off-unix recording tests, sit in the moved files this entity names
@answer n=10 entity=conventions#9.2 v=yes
why: the scope names a minimal stand fixture, and this entity decides its home (fixtures/mod.rs when more than one check reads it, else its check's file) and the shared fixture boot
@answer n=11 entity=conventions#9.3 v=yes
why: the chunk edits the MCP tool listing in mcp/listing.rs, adding a row that the escher-driver MCP listing test walks through its private track_caller row helpers
@answer n=12 entity=conventions#10.1 v=no
why: the chunk adds no binary, Android entry point, Store state, example or stand task; the stand is run, not edited, and the example declarations are not in scope
@answer n=13 entity=conventions#11.1 v=no
why: the seven_guis tasks' use_graph_signal and writers rule governs the stand's existing tasks, which the chunk only runs; the chunk adds no task and no signal
@answer n=14 entity=conventions#12.1 v=no
why: the chunk's fixtures are stand fixtures, not the examples/assets HTML pages this rule covers
