@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the formatting and lint legs it states are the S4 convention (cargo fmt, clippy -D warnings, rustdoc errors, ci-leg.sh)
@answer n=2 entity=conventions#2.1 v=no
why: the chunk touches no Cargo manifest and adds no crate; the bridge's existing WriteMutations use already consumes dioxus-core
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the //! and /// doc rules it states are the S3 convention, which names the new mutation_buffer.rs
@answer n=4 entity=conventions#4.1 v=yes
why: mutation_writer.rs carries a WARNING: DO NOT REORDER block (lines 203-207) and the replay feeds MutationWriter in buffered order, so the replay must make its calls in the recorded order
@answer n=5 entity=conventions#5.1 v=no standing=S5
why: the pub(crate) and pub(super) rule it states is the S5 convention, which names the new buffer type
@answer n=6 entity=conventions#6.1 v=yes
why: task item 11 names the per-call-site tracing gate with a cfg(not(feature = "tracing")) fallback, and any trace the buffer module keeps follows it
@answer n=7 entity=conventions#7.1 v=no standing=S6
why: the unsafe-block and borrow-workaround rule it states is the S6 convention
@answer n=8 entity=conventions#8.1 v=no standing=S1
why: the unit-test placement rule it states is the S1 convention, which the ported test in dioxus_document.rs mod tests follows
@answer n=9 entity=conventions#9.1 v=no standing=S1
why: the escher-driver #[path] test-module placements are the S1 convention's #[path] option, and the chunk touches no escher-driver file
@answer n=10 entity=conventions#9.2 v=no
why: the home of shared fixture helpers is a convention every new test owes equally, and the chunk adds no fixture page or held stand-in read by more than one check
@answer n=11 entity=conventions#9.3 v=no
why: the row-walk helper messages of the requirement sweep do not reach the chunk's tests, which walk no rows
@answer n=12 entity=conventions#10.1 v=no
why: the example and binary conventions do not reach the chunk, which adds no example, app or binary
@answer n=13 entity=conventions#11.1 v=no
why: the HTML fixture conventions do not reach the chunk, which adds no HTML fixture and whose task marks fixtures under examples/assets not needed
