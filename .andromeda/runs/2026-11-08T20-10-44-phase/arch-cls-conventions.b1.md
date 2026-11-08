@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the cargo fmt, clippy -D warnings and rustdoc -D warnings gates through ci-leg.sh, which S4 names
@answer n=2 entity=conventions#2.1 v=no
why: manifest and workspace dependency rules; the chunk moves no dependency and reaches no Cargo.toml
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: //! module docs and /// public-item docs, which S3 names; the chunk adds no table or count they must record
@answer n=4 entity=conventions#4.1 v=no
why: TODO/FIXME markers and the DO NOT REORDER block sit in mutation_writer.rs and other files the chunk does not reach
@answer n=5 entity=conventions#5.1 v=no standing=S5
why: pub(crate) internals and with_/without_ builders, which S5 names; the chunk adds no builder the list must record
@answer n=6 entity=conventions#6.1 v=yes
why: graph is accessibility-gated while signal_registry is ungated, so the new exercised record must hold with no registry; escher-driver spans stay ungated and hand-built (item 18)
@answer n=7 entity=conventions#7.1 v=yes
why: the bridge's RefCell-unborrowed render and single-borrow replay in dioxus_document.rs, where the chunk's handle_event recording lands
@answer n=8 entity=conventions#8.1 v=yes
why: the owed replay test is a new Session-holding check, and the session-check list and the eight unit-test modules must record it
@answer n=9 entity=conventions#9.1 v=yes
why: recording.rs is reached, so its cfg(unix) recording_tests and off-unix recording_off_unix_tests must keep holding for the kept-check medium
@answer n=10 entity=conventions#9.2 v=no
why: the fixtures module and its HeldNet double serve blitz-tests fixture pages, which the chunk neither reads nor adds
@answer n=11 entity=conventions#9.3 v=yes
why: the chunk adds the coverage verb, a new row that the MCP listing test and stand_requirement_sweep row walk must assert through their private track_caller helpers
@answer n=12 entity=conventions#10.1 v=no
why: the chunk drives seven_guis but changes no example binary, entry point, component or CSS
@answer n=13 entity=conventions#11.1 v=yes
why: the DeclaredWrite and use_graph_state registry named in scope carries the const writers form that the seven_guis tasks use
@answer n=14 entity=conventions#12.1 v=no
why: HTML fixtures and the parse path are not_needed by the chunk; no fixture page is read or changed
