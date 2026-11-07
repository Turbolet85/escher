@answer n=1 entity=conventions#1.1 v=no
why: its cited sites are CI scripts, workflows and other crates' sources, none in a reached file, and the CI gates stay green on the change (CI#42371051720 17/17).
@answer n=2 entity=conventions#2.1 v=no
why: no manifest, lock or deny config moved, and the cited dioxus-native-dom lib.rs:1 sits above the one-line insertion at line 21.
@answer n=3 entity=conventions#3.1 v=no
why: its cited module docs are in files the session does not reach, and the report states no change to module doc comments.
@answer n=4 entity=conventions#4.1 v=no
why: its cited TODO/FIXME sites sit in unreached files (mutation_writer.rs and dioxus-native), and the buffer's limit is recorded as a residual row, not a code marker.
@answer n=5 entity=conventions#5.1 v=no
why: its cited sites are in unreached files, and the new buffer is pub(crate) with no pub use, so the crate-internal visibility rule still holds.
@answer n=6 entity=conventions#6.1 v=yes mode=stale
why: lib.rs gains one mod line at line 21, moving the cited trace! macro lines 57-80 and its 4-argument arm at 71-74 down by one; the cfg and tracing rule itself stands.
@answer n=7 entity=conventions#7.1 v=yes mode=add-to
why: the bridge now records mutations into the crate-private MutationBuffer (F1, F2) and replays them under one borrow, a new borrow-workaround shape beside taking a child list or layout out.
@answer n=8 entity=conventions#8.1 v=yes mode=stale
why: dioxus_document.rs gains 52 lines, all above its bridge_tests declaration, so the cited lines 888-890 move to 940-942; the eight-module convention itself holds.
@answer n=9 entity=conventions#9.1 v=no
why: its cited sites are escher-driver files the session does not reach, and no test in them changes.
@answer n=10 entity=conventions#9.2 v=no
why: its cited fixtures/mod.rs is an unreached blitz-tests file, and the fixture boot and HeldNet convention it states is unchanged.
@answer n=11 entity=conventions#9.3 v=no
why: its cited mcp_tests.rs and stand_requirement_sweep.rs are unreached, and the probe's new track_caller helper lives in a different file and changes neither.
@answer n=12 entity=conventions#10.1 v=no
why: its cited example and app sources are unreached, and no Dioxus example's launch or state pattern changes.
@answer n=13 entity=conventions#11.1 v=no
why: its cited HTML fixtures are unreached and no fixture file changes.
