@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the fmt, clippy and rustdoc gate line is the convention S4 names, owed by every new line alike
@answer n=2 entity=conventions#2.1 v=no
why: the chunk adds no dependency or manifest line, and the workspace-dependency rule is a re-check every change owes
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the module, file and public-item doc comment rule is the convention S3 names
@answer n=4 entity=conventions#4.1 v=no
why: the TODO, FIXME and WARNING markers sit in files the chunk does not reach, and marking known gaps is a re-check every change owes
@answer n=5 entity=conventions#5.1 v=no standing=S6
why: the pub(crate) and pub(super) visibility and error-enum rule is the convention S6 names
@answer n=6 entity=conventions#6.1 v=yes
why: the read reads DioxusDocument::graph, whose graph module is gated by accessibility, so the new read's code in graph.rs must sit under that gate
@answer n=7 entity=conventions#7.1 v=no
why: the buffered-render borrow rule governs the mutation path the read never enters, and the chunk adds no unsafe block
@answer n=8 entity=conventions#8.1 v=yes
why: the chunk proves the read on the stand with a new session check, which the session_common list of Session-holding checks must record, and each new integration test file owes the file rules
@answer n=9 entity=conventions#9.1 v=no standing=S1
why: new cli and recording tests take the existing cli_tests.rs and recording_tests.rs homes, which the test-home convention S1 names
@answer n=10 entity=conventions#9.2 v=yes
why: the scope names a minimal stand fixture, and this entity decides whether it is stated in fixtures/mod.rs or kept in its one check's file
@answer n=11 entity=conventions#9.3 v=yes
why: the chunk adds a new MCP tool row, which the listing test's row walk must assert through the row-and-index-only helpers
@answer n=12 entity=conventions#10.1 v=no
why: the chunk adds no example binary or example crate, its fixture being a stand fixture under tests/blitz-tests, so the example rules are a re-check every change owes
@answer n=13 entity=conventions#11.1 v=no
why: the chunk adds no seven_guis task or signal, and the signal-registration rule is a re-check every change owes
@answer n=14 entity=conventions#12.1 v=no
why: the HTML fixture rule covers examples/assets pages, which the chunk does not add, since its fixtures are stand fixtures
