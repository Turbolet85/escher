@answer n=1 entity=conventions#1.1 v=no standing=S4
why: the fmt, clippy and rustdoc-as-errors ci-leg gates, which the task's standing convention S4 names
@answer n=2 entity=conventions#2.1 v=no
why: the manifest rule binds only where a dependency is added, and the chunk adds none; no manifest is reached and the Dioxus 0.7.3 pin stays
@answer n=3 entity=conventions#3.1 v=no standing=S3
why: the //! and /// doc convention for files and public items, which the task's standing convention S3 names and applies to the new mutation_buffer.rs
@answer n=4 entity=conventions#4.1 v=yes
why: the WARNING: DO NOT REORDER block at mutation_writer.rs:203-207 fixes the call order the buffered replay feeds into MutationWriter, so the replay must keep the recorded order; the TODO markers do not bind
@answer n=5 entity=conventions#5.1 v=no standing=S5
why: the pub(crate) and pub(super) visibility rule, which the task's standing convention S5 names
@answer n=6 entity=conventions#6.1 v=yes
why: task item 11 requires any trace the buffer module keeps to be gated per call site with a cfg(not(feature = "tracing")) fallback, and the dioxus-native-dom trace! macro rule applies
@answer n=7 entity=conventions#7.1 v=no standing=S6
why: the unsafe SAFETY-comment and take-out-and-restore borrow rules, which the task's standing convention S6 names; the chunk's borrow fix orders the calls and takes no value out
@answer n=8 entity=conventions#8.1 v=yes
why: the probe's module text is edited and its two pins are inverted, so the //! naming the behaviour, both layout modes and the precondition-first rule bind; the ported test goes into dioxus_document.rs mod tests
@answer n=9 entity=conventions#9.1 v=no standing=S1
why: the escher-driver test modules moved out as #[path] child files, a convention the task's standing convention S1 names; the chunk moves no test module and reaches no escher-driver file
@answer n=10 entity=conventions#9.2 v=no
why: the shared blitz-tests fixtures module binds only when the chunk adds a page more than one check reads, and the chunk adds none
@answer n=11 entity=conventions#9.3 v=no
why: the row-walk helper messages of stand_requirement_sweep and escher-driver's MCP listing test, which the chunk does not edit and whose re-run does not change those messages
@answer n=12 entity=conventions#10.1 v=no
why: the app and example binaries' entry and launch shape, which the chunk does not edit; the windowed launch is read, not run
@answer n=13 entity=conventions#11.1 v=no
why: HTML fixture forms under examples/assets, which the task names as not needed (item 11)
