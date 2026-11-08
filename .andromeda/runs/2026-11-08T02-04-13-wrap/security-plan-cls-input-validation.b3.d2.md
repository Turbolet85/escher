@answer n=1 entity=input-validation#1.51 v=no
why: SVG width and height parsing lives in blitz-dom stylo.rs, which the change does not reach
@answer n=2 entity=input-validation#1.52 v=no
why: border and body margin parsing lives in blitz-dom stylo.rs and never reads the graph or the snapshot
@answer n=3 entity=input-validation#1.53 v=no
why: the markup numeric fallbacks live in blitz-dom layout files, which the change does not reach
@answer n=4 entity=input-validation#1.54 v=no
why: the rowspan clamp lives in blitz-dom table.rs, which the change does not reach
@answer n=5 entity=input-validation#1.55 v=no
why: the multiple select size rule lives in blitz-dom select_list.rs and default.css, which the change does not reach
@answer n=6 entity=input-validation#1.56 v=no
why: the ol start parse lives in blitz-dom construct.rs, which the change does not reach
@answer n=7 entity=input-validation#1.57 v=no
why: attr_parsed lives in blitz-dom element.rs, which the change does not reach
@answer n=8 entity=input-validation#1.58 v=yes mode=stale
why: its snapshot.rs:180-183 and :605-652 citations moved, since snapshot.rs gained lines at 77-80, 112-116 and 134-135 ahead of them, while the enabled presence reading holds
@answer n=9 entity=input-validation#1.59 v=no
why: the open attribute reading lives in blitz-dom element.rs and mutator.rs, which the change does not reach
@answer n=10 entity=input-validation#1.60 v=no
why: autofocus lives in blitz-dom mutator.rs, which the change does not reach
@answer n=11 entity=input-validation#1.61 v=no
why: dir matching lives in blitz-dom, which the change does not reach
@answer n=12 entity=input-validation#1.62 v=no
why: the canvas src parse lives in blitz-dom mutator.rs, which the change does not reach
@answer n=13 entity=input-validation#1.63 v=no
why: the custom widget attribute replay lives in blitz-dom element.rs and mutator.rs, which the change does not reach
@answer n=14 entity=input-validation#1.64 v=yes mode=add-to
why: element_id.rs gains the reserved fifth id form {owner}///{key} (F3), which this grammar row must carry beside the four tiers it lists
@answer n=15 entity=input-validation#1.65 v=yes mode=stale
why: its snapshot.rs:215-233 and snapshot_diff.rs:111 citations moved with the lines gained ahead of them
@answer n=16 entity=input-validation#1.66 v=no
why: descriptions are read in blitz-dom accessibility.rs, and the graph answer carries no description text, so the no-carrier claim still holds
@answer n=17 entity=input-validation#1.67 v=yes mode=stale
why: its snapshot.rs:181 citation moved with the lines gained at 77-80, 112-116 and 134-135 ahead of it, while the enabled reading holds
@answer n=18 entity=input-validation#1.68 v=no
why: select open and aria-expanded steering lives in blitz-dom, which the change does not reach
@answer n=19 entity=input-validation#1.69 v=yes mode=add-to
why: the snapshot's NodeState fields the tree feeds gain clickable (F6), a closed bool written after focused, which this states row must list
@answer n=20 entity=input-validation#1.70 v=no
why: the native checkbox checked-state write lives in blitz-dom accessibility.rs, which the change does not reach
@answer n=21 entity=input-validation#1.71 v=no
why: the inert rule lives in blitz-dom node.rs and accessibility.rs, and the change's dioxus_document.rs edits add the graph reader and dispatch wrap, not the inert skip in unkeyed_actionable
@answer n=22 entity=input-validation#1.72 v=no
why: the select list check set lives in blitz-dom document.rs, which the change does not reach
@answer n=23 entity=input-validation#1.73 v=no
why: the inert focus clear and the tree focus write live in blitz-dom accessibility.rs and mutator.rs, which the change does not reach
@answer n=24 entity=input-validation#1.74 v=yes mode=stale
why: its snapshot.rs:275-277 citation for NodeState.value moved with the lines gained ahead of it
@answer n=25 entity=input-validation#1.75 v=no
why: the keyboard activation default actions live in blitz-dom events, which the change does not reach, and the bridge dispatch wrap leaves them as they were
