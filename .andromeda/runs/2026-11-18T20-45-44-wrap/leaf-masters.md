@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=12 class=omission
leaf-text: the windowed `dioxus_native::launch` and the browser build their documents `Parallel`, serialised by the lock
source: dec-muo2nocn «decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel»
replace-with: the windowed `dioxus_native::launch`, the browser and the multi-document bench example build their documents `Parallel`, serialised by the lock
