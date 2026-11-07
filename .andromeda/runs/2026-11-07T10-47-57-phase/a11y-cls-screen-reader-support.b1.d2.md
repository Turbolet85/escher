@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the buffer carries text, and the re-asserted snapshot names of parents read the TextRun labels the builder gives them
@answer n=2 entity=screen-reader-support#1.2 v=yes
why: the buffer replays aria-label and aria-labelledby attribute values that the re-asserted snapshot name reads
@answer n=3 entity=screen-reader-support#1.3 v=yes
why: the label-to-element binding is a post-build pass over the document the replay produces, so the replayed for/id and order must match the direct write
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: the re-asserted stand_act_select and snapshot checks read the select's options, value and bounds, which the replayed render must still produce
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single select's expanded reading from its closed bool is select-popup engine state that the buffered render does not carry and no re-asserted check reads
@answer n=6 entity=screen-reader-support#1.6 v=no
why: the open list's highlighted row is a stated limit of the select popup, which the chunk does not change and no re-asserted check reads
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is unmeasured and the chunk does not touch the open-list path
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: a multiple select's row bounds are read in the re-asserted stand_act_select and snapshot checks in both layout modes
@answer n=9 entity=screen-reader-support#1.9 v=no
why: multiple_select_values is another unit of dioxus_document.rs that must only keep holding, and the buffered render does not touch event values
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: the scope names the stable element ids as AccessKit author_id, and the buffer must leave each element's id as the direct write gave it
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot model reads role, name, focus and states from accessibility_tree and its diff, both of which the chunk re-asserts unchanged
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: the re-asserted snapshot's field set is the value the buffered render must leave unchanged, with no expanded key
@answer n=13 entity=screen-reader-support#3.3 v=yes
why: the option and group names read by the snapshot pass through the replayed text, and the buffer must not log them (item 15)
@answer n=14 entity=screen-reader-support#3.4 v=yes
why: the snapshot's native checked reading and the tree's toggled reading depend on the checked state the replay must carry
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the scope names unkeyed_actionable returning nothing on the stand tasks, and author keys must survive the replay unchanged
@answer n=16 entity=screen-reader-support#4.2 v=yes
why: link 2 of dioxus_runtime_probe is one of the other eight probe tests the chunk re-asserts unchanged
@answer n=17 entity=screen-reader-support#4.3 v=yes
why: listener marks written through MutationWriter must survive the replay, and the probe test at 387-429 is re-asserted
@answer n=18 entity=screen-reader-support#4.4 v=yes
why: the actionable-key check's return on Cells and its reacting-element count are re-asserted, and cells-grid's aria-label must survive the replay
@answer n=19 entity=screen-reader-support#4.5 v=yes
why: the load page's actionable-key check and its stand_load_page and stand_keyboard rows are re-asserted in both layout modes
@answer n=20 entity=screen-reader-support#5.1 v=yes
why: the role and state attributes on the CRUD list, its rows and the date inputs must survive the replay, and the actionable-key check still returns nothing
@answer n=21 entity=screen-reader-support#6.1 v=yes
why: the buffer carries tag names, and the tree the snapshot reads reports each element's HTML tag
@answer n=22 entity=screen-reader-support#6.2 v=yes
why: the scope names the tree's focus falling back to the window for an unbuilt focused node, which a set_focus issued during render must not leave on an unbuilt node
@answer n=23 entity=screen-reader-support#6.3 v=no
why: the role-mapping test's goal comment is not a rule the chunk's code or tests must honour, and role derivation is not touched
@answer n=24 entity=screen-reader-support#7.1 v=yes
why: the scope names the InitialTreeRequested path that DioxusDocument's tree override must still reach, and the adapter routes that request to one handler without a synchronous tree
@answer n=25 entity=screen-reader-support#8.1 v=no
why: adapter creation from the window handle is excluded by the task's not-needed rows, and the windowed tree refresh is only read, not run
