@answer n=1 entity=screen-reader-support#1.1 v=no
why: text-run labelling is the builder's name rule, not touched by the chunk (not_needed names), witnessed only by non-stand tests
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label name computation is the builder's, not touched (not_needed names), witnessed by accessibility_names and accessibility_label
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label binding runs over the whole built tree at tree build, the builder is not touched and the replay keeps recorded order
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: the snapshot's select value, options and selected state are read through the re-asserted stand_* checks, and the replay writes their option attributes
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the select's expanded reading and list opening are not touched (not_needed keys and list opening), witnessed by select_popup tests outside the re-asserted set
@answer n=6 entity=screen-reader-support#1.6 v=no
why: a stated limit on the open list's highlighted row in the capabilities doc, which the chunk does not change
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is an unmeasured select-popup record the chunk does not touch
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: multiple-select row bounds are read through stand_act_select, a re-asserted stand_* check, in both layout modes
@answer n=9 entity=screen-reader-support#1.9 v=no
why: multiple_select_values at dioxus_document.rs:31-45 is another unit of an edited file, not a render site or the ported test, and no re-asserted check reads it
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: the buffer carries element ids through the replay, which must leave each element's author_id as the direct write gave it, checked by stand_accessibility_ids
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot model reads role, name, focus and states from accessibility_tree and its diff, which the chunk re-asserts unchanged (task item 6)
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: the snapshot's thirteen-field set with no expanded key is the model the chunk re-asserts as unchanged
@answer n=13 entity=screen-reader-support#3.3 v=yes
why: option labels reach the buffer as text, which must stay in process with no log site (task item 15), and stand_act_popup names the snapshot name read
@answer n=14 entity=screen-reader-support#3.4 v=yes
why: the snapshot's checked reads the native checkbox state from the element, which a replayed set-attribute must set through the same MutationWriter path (task item 8)
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and unkeyed_actionable returning nothing on every stand task are what author keys must keep through the replay (task item 7)
@answer n=16 entity=screen-reader-support#4.2 v=yes
why: link 2 of dioxus_runtime_probe is one of the probe's other tests the chunk re-asserts, and its listener marks come through the replayed writes
@answer n=17 entity=screen-reader-support#4.3 v=yes
why: listener attributes and data-dioxus-id marks are written through MutationWriter paths the replay must route through, and the probe test at 387-429 re-asserts them
@answer n=18 entity=screen-reader-support#4.4 v=no
why: cells-grid's aria_label and absent role are not what the unkeyed check reads, its key comes from its id, and no chunk rule runs over it
@answer n=19 entity=screen-reader-support#4.5 v=yes
why: unkeyed_actionable returning nothing on the load page is a stand reading the re-asserted check must keep (task item 7), named by stand_load_page
@answer n=20 entity=screen-reader-support#5.1 v=yes
why: the tree's focus is the list and never the option, and the actionable check still returns nothing on the stand, both re-asserted readings
@answer n=21 entity=screen-reader-support#6.1 v=no
why: element tag names in the tree are the builder's output, which the chunk does not change and no re-asserted check names
@answer n=22 entity=screen-reader-support#6.2 v=yes
why: a set_focus during render before the buffer is applied must not leave the tree's focus on an unbuilt node, which the built-node check guarantees (task item 10)
@answer n=23 entity=screen-reader-support#6.3 v=no
why: a test-file comment on the role-mapping goal, stating no rule the chunk acts on
@answer n=24 entity=screen-reader-support#7.1 v=yes
why: InitialTreeRequested reaching the handler with no synchronous initial tree is the path the tree refresh must reach (task items 4 and 14)
@answer n=25 entity=screen-reader-support#8.1 v=no
why: adapter creation from the raw window handle is not moved (not_needed item 7), and the refresh depends only on the routing in entity 7.1
