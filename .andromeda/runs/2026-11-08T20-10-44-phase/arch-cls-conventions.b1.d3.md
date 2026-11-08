@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the cargo fmt, clippy and rustdoc -D warnings runs through ci-leg.sh and the expect_used and unwrap_used rule are the standing lint convention
@answer n=2 entity=conventions#2.1 v=no
why: the manifest rules sit over Cargo.toml files the chunk does not edit, and the chunk adds no dependency or feature
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the //! module docs, /// public item docs and constant-rationale rule are the standing documentation convention
@answer n=4 entity=conventions#4.1 v=yes
why: the canvas limit and the 0.4.0 virtual-node extension are known gaps the new code leaves open, which the TODO/FIXME marking rule binds
@answer n=5 entity=conventions#5.1 v=no standing=S5
why: the pub(crate) and pub(super) internals rule and the with_ and without_ consuming builders are the standing visibility and naming convention
@answer n=6 entity=conventions#6.1 v=yes
why: the new record must hold on apps with no registry, the graph module is gated by accessibility while signal_registry is ungated, and escher-driver's command span is built by hand ungated
@answer n=7 entity=conventions#7.1 v=yes
why: the bridge's event path in dioxus_document.rs, where the new handler record is written, must not borrow the document RefCell while the vdom runs under the MutationBuffer rule
@answer n=8 entity=conventions#8.1 v=yes
why: the owed replay test is a new stand session check holding a Session, which the session_common list of session checks must record
@answer n=9 entity=conventions#9.1 v=yes
why: recording_tests.rs and cli_tests.rs are files the chunk edits, and their moved-out test modules must keep their file placement and test paths
@answer n=10 entity=conventions#9.2 v=no standing=S2
why: the shared fixtures module under tests/ is the standing helper-sharing convention, and the chunk adds no fixture page or HeldNet use
@answer n=11 entity=conventions#9.3 v=yes
why: the new handler-coverage verb adds rows to stand_requirement_sweep.rs, which the chunk reaches whole, and its row walk must assert each row through the private track_caller helpers
@answer n=12 entity=conventions#10.1 v=no
why: the binary subsystem, Android entry, Store derive, BEM CSS, asset and example CSS rules sit over apps and examples the chunk does not edit
@answer n=13 entity=conventions#11.1 v=no
why: the seven_guis tasks' use_graph_signal registration and const DeclaredWrite writers are the stand's own code, which the chunk reads through without constructing writers
@answer n=14 entity=conventions#12.1 v=no
why: the HTML fixture rules over examples/assets do not reach the chunk, which adds no HTML fixture or parse path
