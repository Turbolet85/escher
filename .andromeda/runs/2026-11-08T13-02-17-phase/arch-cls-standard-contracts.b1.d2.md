@answer n=1 entity=standard-contracts#1.1 v=no
why: the Document trait, config, changed-set and net facts are not called by the new read, and DioxusDocument's accessibility override is not touched.
@answer n=2 entity=standard-contracts#2.1 v=yes
why: the scope's "moves no virtual clock time" names the VirtualClock the document reads, which the read must not advance.
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value and the snapshot's value field are not part of the new read's answer.
@answer n=4 entity=standard-contracts#2.3 v=yes
why: an open select list is answered first, through the public open_select and list readers the read's covered reading relies on.
@answer n=5 entity=standard-contracts#2.4 v=yes
why: live_select_list decides whether a list is open for select_list_hit, which the read's covered reading goes through.
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile ordering is a step the read does not run, since the read calls no resolve.
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of, is_select via locate and is_multiple_select are the select tests the task names the driver reads.
@answer n=8 entity=standard-contracts#2.7 v=no
why: mutator writes of a multiple select are not reached by a read that writes nothing.
@answer n=9 entity=standard-contracts#2.8 v=no
why: a failed stylesheet's release of pending critical resources is not used by a read that waits on no load.
@answer n=10 entity=standard-contracts#3.1 v=no
why: the selector, CSSOM, mutation and accessibility APIs it lists are not called by the new read.
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded write is not read by the new read.
@answer n=12 entity=standard-contracts#3.3 v=no
why: the accessibility tree's focus write is not reached by the driver read.
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkable state in the accessibility tree is not what the read answers.
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved width and height through resolved_style_value are not read by the new read.
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs on writes the read never makes.
@answer n=16 entity=standard-contracts#4.1 v=no
why: event dispatch and default actions are not reached, since the read dispatches nothing by the task's own rule.
@answer n=17 entity=standard-contracts#4.2 v=no
why: a select list's default actions run only on a dispatch the read does not make.
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label click's focus behaviour runs only on dispatch, which the read does not make.
@answer n=19 entity=standard-contracts#4.4 v=no
why: change dispatch on checkable controls runs only on dispatch, which the read does not make.
@answer n=20 entity=standard-contracts#5.1 v=yes
why: visible_region, the hit walk stopping at clipping boxes and scroll_into_view's no-scroll contract are the shared refusal path's readings the read must hold.
@answer n=21 entity=standard-contracts#6.1 v=yes
why: get_client_bounding_rect under transforms is the target's reading that the off-screen and in-view refusals depend on.
@answer n=22 entity=standard-contracts#6.2 v=yes
why: visible_region under transforms is the off-screen reading that execute.rs in_view shares with the read.
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is called by Session::lint alone, not by the new read.
@answer n=24 entity=standard-contracts#6.4 v=yes
why: hit_with_scrollbar answering the open list first and skipping dropped nodes is the covered reading the read shares with the executor.
@answer n=25 entity=standard-contracts#6.5 v=yes
why: a non-invertible transform taking no hit decides the covered reading through hit.
