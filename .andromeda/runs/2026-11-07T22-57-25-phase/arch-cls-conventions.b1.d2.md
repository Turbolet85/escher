@answer n=1 entity=conventions#1.1 v=no standing=S3
why: the fmt, clippy and rustdoc -D warnings legs through ci-leg.sh are the convention S3 names
@answer n=2 entity=conventions#2.1 v=yes
why: task item 18 needs the registry's dioxus-core dependency declared once in workspace.dependencies, used as workspace = true, with default-features = false on in-repo crates
@answer n=3 entity=conventions#3.1 v=no standing=S2
why: the //! module docs, /// public-item docs and constant rationale are the convention S2 names
@answer n=4 entity=conventions#4.1 v=no
why: the TODO/FIXME markers and the DO NOT REORDER block sit in mutation_writer.rs and other files the scope does not edit, and the chunk adds no known gap the list must record
@answer n=5 entity=conventions#5.1 v=yes
why: task item 19 needs crate internals pub(crate) or pub(super) and builder methods named with_ and without_ for the registry and graph reader
@answer n=6 entity=conventions#6.1 v=yes
why: task item 16 needs the accessibility feature to gate the graph reader and the snapshot field, with features forwarded down the crates
@answer n=7 entity=conventions#7.1 v=yes
why: the graph read walks the vdom and reads the registry while the document is borrowed, so it must render with the RefCell unborrowed and replay through one MutationWriter as the Dioxus bridge does
@answer n=8 entity=conventions#8.1 v=yes
why: the chunk adds session-holding stand checks, the graph read and its replay, which the session_common list must record, and possibly a #[path] child test module the eight-module count must record
@answer n=9 entity=conventions#9.1 v=yes
why: the chunk edits escher-driver's recording step, the escher-session command line and the schema and MCP tests, so the path-child homes of cli::tests and recording::tests must hold
@answer n=10 entity=conventions#9.2 v=no standing=S1
why: the fixtures module is the shared-helper home S1 names, and the scope names no fixture page or HeldNet that more than one check would read
@answer n=11 entity=conventions#9.3 v=yes
why: the MCP listing test is the pin that reads the driver tool count, which item 15 names, so its row walk and track_caller row helpers govern the new tool row
@answer n=12 entity=conventions#10.1 v=yes
why: task item 17 names the stand's reactive state through derive(Store) and signal and store hooks, and the registry-wrapped signals must keep those hooks' behaviour
@answer n=13 entity=conventions#11.1 v=no
why: task item 8 (not_needed) states the chunk adds no HTML fixture page under examples/assets
