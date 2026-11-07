@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the fmt, clippy, rustdoc and ci-leg gate convention, which the task lists as S4
@answer n=2 entity=conventions#2.1 v=no
why: the chunk edits no manifest and adds no dependency, since no Cargo.toml is among its reached files and Dioxus stays pinned
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the module and item doc-comment convention, which the task lists as S3
@answer n=4 entity=conventions#4.1 v=no
why: the TODO marks and the DO NOT REORDER block at mutation_writer.rs lines 203-207 sit outside DioxusState, a unit the chunk does not edit, and the replay feeds the writer in recorded order without reordering its statements
@answer n=5 entity=conventions#5.1 v=no standing=S5
why: the pub(crate) and pub(super) visibility convention, which the task lists as S5
@answer n=6 entity=conventions#6.1 v=yes
why: task item n=11 binds any trace the buffer module keeps to the per-call-site cfg(feature = "tracing") gate with its not-feature fallback, and the crate-local trace macro sits in the same crate
@answer n=7 entity=conventions#7.1 v=no standing=S6
why: the unsafe-block and borrow-workaround convention, which the task lists as S6
@answer n=8 entity=conventions#8.1 v=no standing=S1
why: the unit-test location convention (a mod tests or a #[path] child file), which the task lists as S1 and which also names the S2 integration-test rules
@answer n=9 entity=conventions#9.1 v=no standing=S1
why: it names escher-driver's #[path] child test files, the S1 convention, and the chunk adds no test file of its own
@answer n=10 entity=conventions#9.2 v=no
why: the shared fixtures module is a test-helper home each new test owes equally, and the chunk adds no fixture page or held double shared across checks
@answer n=11 entity=conventions#9.3 v=no
why: the track_caller row-helper style belongs to escher-driver's MCP listing test and the requirement-sweep check, which the chunk neither edits nor re-runs
@answer n=12 entity=conventions#10.1 v=no
why: app and example declaration shapes, and the chunk changes no app or example, only the engine render path that the windowed launch reaches and is read, not run
@answer n=13 entity=conventions#11.1 v=no
why: HTML fixture conventions under examples/assets, which the task lists as not needed and the chunk does not touch
