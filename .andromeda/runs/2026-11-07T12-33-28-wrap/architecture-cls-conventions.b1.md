@answer n=1 entity=conventions#1.1 v=no
why: the cited lint and gate files are not in the change, and the session adds no lint allowance or gate step.
@answer n=2 entity=conventions#2.1 v=yes mode=stale
why: the crate root dioxus-native-dom/src/lib.rs, a merge surface, gains one mod line at 21, and the docs.rs cfg citation at lib.rs:1 is re-checked against it.
@answer n=3 entity=conventions#3.1 v=no
why: the new mutation_buffer module follows the existing `//!` header rule and no cited doc file is reached.
@answer n=4 entity=conventions#4.1 v=no
why: the cited TODO/FIXME sites sit in files the change does not reach (mutation_writer.rs stays byte-identical), and the session records its limit as a residual row rather than a code marker.
@answer n=5 entity=conventions#5.1 v=no
why: the cited visibility sites are in blitz-dom and blitz-vibey-script, which the change does not reach, and the new pub(crate) items already follow the rule.
@answer n=6 entity=conventions#6.1 v=yes mode=stale
why: the crate root dioxus-native-dom/src/lib.rs gains one mod line at 21, so the cited trace! macro lines 57-80 and its 4-argument arm at 71-74 shift down by one.
@answer n=7 entity=conventions#7.1 v=yes mode=add-to
why: the bridge's borrow conflict is now avoided by recording mutations in MutationBuffer and replaying them under one borrow, a new borrow-workaround shape beside taking a child list or layout out.
@answer n=8 entity=conventions#8.1 v=yes mode=stale
why: dioxus_document.rs gains 52 lines in eight ranges, including 846-885, so the bridge_tests declaration cited at 888-890 moves, and the ported task_polled_during_render_can_access_document test lands in the DioxusDocument tests.
@answer n=9 entity=conventions#9.1 v=no
why: the cited escher-driver test modules and their cli.rs and recording.rs sites are not in the change.
@answer n=10 entity=conventions#9.2 v=no
why: the cited fixtures/mod.rs is not reached, and the probe's helper change adds no shared fixture page or reader.
@answer n=11 entity=conventions#9.3 v=no
why: the cited escher-driver mcp_tests.rs and blitz-tests stand_requirement_sweep.rs are not reached by the change.
@answer n=12 entity=conventions#10.1 v=no
why: the cited example files are not reached, and the launch path and example patterns the rule states stay as they are.
@answer n=13 entity=conventions#11.1 v=no
why: the cited HTML fixtures are not reached and the session adds no fixture page.
