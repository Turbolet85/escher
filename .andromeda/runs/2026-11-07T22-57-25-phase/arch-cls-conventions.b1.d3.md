@answer n=1 entity=conventions#1.1 v=no standing=S3
why: the fmt, clippy and rustdoc legs and the expect/unwrap justification are the convention S3 names, which every change owes alike
@answer n=2 entity=conventions#2.1 v=yes
why: the scope's n=18 names the manifest rule for a dependency the graph reader takes, dioxus-core, declared once in workspace dependencies, taken as workspace = true, with in-repo crates at default-features = false
@answer n=3 entity=conventions#3.1 v=yes
why: the scope's n=19 names rustdoc on every public item of the registry and graph reader, and the new modules must open with a //! doc
@answer n=4 entity=conventions#4.1 v=no
why: the chunk adds no TODO or FIXME marker and edits no ordering-sensitive block, and the DO NOT REORDER block in mutation_writer.rs is one it only runs through and leaves unchanged, a re-check every change owes
@answer n=5 entity=conventions#5.1 v=yes
why: the scope's n=19 names crate internals staying pub(crate) or pub(super) and the with_ and without_ builder names for the registry and graph reader
@answer n=6 entity=conventions#6.1 v=yes
why: the scope's n=16 names the accessibility feature gating the graph reader and the snapshot field and the forwarding of features down the crates, while the tracing clause is S4's and does not bind
@answer n=7 entity=conventions#7.1 v=yes
why: the graph read borrows the bridge document through DioxusDocument::element_id and the registry walks the runtime, so the bridge rule to render into the mutation buffer with the RefCell unborrowed and replay through one borrow binds the new code
@answer n=8 entity=conventions#8.1 v=yes
why: the scope's n=13 has stand_non_lean_tasks pin the starting reading in both layout modes, so the both-layout-modes and assert-the-fixture-condition-first rules bind, and the rest of the entry is the S1 test-placement convention
@answer n=9 entity=conventions#9.1 v=yes
why: the scope's n=14 recorded step reaches the escher-driver recording module, whose unix-gated recording tests and Windows-only off_unix_tests placement is the OS-only check rule over where that OS's code is verified
@answer n=10 entity=conventions#9.2 v=no standing=S1
why: the shared blitz-tests fixtures module is the stand-check helper convention S1 names, and the chunk adds no fixture page that more than one check reads
@answer n=11 entity=conventions#9.3 v=yes
why: the scope's n=15 pins the MCP driver-command count that the escher-driver MCP listing test asserts row by row, so its track_caller helpers whose messages name only the row and index bind the edit
@answer n=12 entity=conventions#10.1 v=yes
why: the scope's n=17 names reactive state through derive Store and the signal and store hooks on the stand, which the registry-wrapped signals must keep
@answer n=13 entity=conventions#11.1 v=no
why: the scope's not_needed n=8 says the chunk adds no HTML fixture page and no licensed asset under examples/assets, so no rule over that tree is reached
