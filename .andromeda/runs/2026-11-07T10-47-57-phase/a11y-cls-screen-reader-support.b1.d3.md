@answer n=1 entity=screen-reader-support#1.1 v=no
why: TextRun derivation is the tree builder's own rule (blitz-dom accessibility.rs:269-273), neither edited nor named by the scope; faithful text replay is not this rule.
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label naming is the builder's name derivation (accessibility.rs:158-171), untouched by the chunk; the snapshot's reading of names is task item n=6, carried by 3.1.
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label binding is the builder's association rule (accessibility.rs:36-103; document.rs:695-733), a built-node relation the replay leaves as it is, with no unit of the chunk touching it.
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: an option's selected state is a bridge boolean attribute the replay routes through MutationWriter (task item n=8), and the flight booker's select value and selected reading are re-asserted by stand_act_select.
@answer n=5 entity=screen-reader-support#1.5 v=no
why: a single select's expanded reading from closed engine popup state is untouched by the chunk, and no set_focus path in the scope reaches an open select list.
@answer n=6 entity=screen-reader-support#1.6 v=no
why: the open list's highlighted row is a stated popup limit that the chunk neither edits nor reads.
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is unmeasured and the windowed list is not run; no popup, adapter or stand change is in scope.
@answer n=8 entity=screen-reader-support#1.8 v=no
why: multiple-select row bounds are client-rect layout geometry in the snapshot; the chunk changes no layout or client-rect path, and its get_client_rect check reads an input, not a select row.
@answer n=9 entity=screen-reader-support#1.9 v=no
why: multiple_select_values at dioxus_document.rs:31-45 is an unedited, unnamed unit of an edited file, a re-check every change owes.
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: the stable element id as author_id is the DioxusDocument accessibility_tree override (dioxus_document.rs:314-330) that task items n=4 and n=5 name, and the buffer must carry the ids unchanged.
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot's role, name, focus and states read from accessibility_tree, and its diff, are task item n=6, which the buffered render must leave unchanged.
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: the thirteen NODE_FIELDS and the absent expanded key are the snapshot model shape that task item n=6 keeps unchanged and the re-asserted snapshot checks pin.
@answer n=13 entity=screen-reader-support#3.3 v=no
why: an open list's option and group names reach the snapshot only through a popup row and a cropped shot, paths the chunk does not touch, and no focus, id or refresh rule of the scope is stated here.
@answer n=14 entity=screen-reader-support#3.4 v=yes
why: the native checked state is set through the boolean checked attribute the replay routes (task item n=8), and the snapshot checked and tree toggled readings are task item n=6.
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and unkeyed_actionable returning nothing on every stand task are task items n=7, and author keys must survive the replay unchanged.
@answer n=16 entity=screen-reader-support#4.2 v=yes
why: link 2's reacts-to-click readings are among the probe's other tests the chunk re-asserts, and the listener marks they read are written through the MutationWriter path the buffer replays.
@answer n=17 entity=screen-reader-support#4.3 v=yes
why: the listener attributes are written by MutationWriter (mutation_writer.rs:331-360), which the replay routes, and the probe's re-asserted tests at 387-429 read them.
@answer n=18 entity=screen-reader-support#4.4 v=no
why: cells-grid's aria-label and absent role are Cells markup and a residual; the chunk edits no Cells code and no key rule.
@answer n=19 entity=screen-reader-support#4.5 v=yes
why: unkeyed_actionable must return nothing on the load page served beside the lean four (task item n=7, every stand task), and stand_load_page is re-asserted.
@answer n=20 entity=screen-reader-support#5.1 v=yes
why: its clause that the actionable-key check still returns nothing is task item n=7, and the listbox, option and aria-invalid readings it states are tree states the snapshot reads (task item n=6).
@answer n=21 entity=screen-reader-support#6.1 v=no
why: the HTML tag name on element nodes is a downstream read of the element by the tree builder (accessibility.rs:268), untouched by the chunk, and not a value the new code must pass.
@answer n=22 entity=screen-reader-support#6.2 v=yes
why: the tree's focus falling back to the Window where the focused node is unbuilt is what task item n=10 requires the chunk to hold for a set_focus issued during render before the buffer is applied.
@answer n=23 entity=screen-reader-support#6.3 v=no
why: the role-mapping test's stated goal lives in accessibility_roles.rs, which the chunk does not touch.
@answer n=24 entity=screen-reader-support#7.1 v=yes
why: accesskit_xplat's routing of InitialTreeRequested to one handler with no synchronous initial tree is the path task items n=4 and n=14 read the DioxusDocument refresh through.
@answer n=25 entity=screen-reader-support#8.1 v=no
why: adapter creation from the window handle and event forwarding are untouched (task not_needed item n=7), and the windowed refresh is read, not run.
