@answer n=1 entity=standard-contracts#1.1 v=yes
why: the DioxusDocument stable-id accessibility_tree override and build_accessibility_tree's TreeUpdate are named by the scope, and the graph is keyed on that node
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock is not read or set by the chunk's registry, graph read or stand
@answer n=3 entity=standard-contracts#2.2 v=yes
why: the snapshot's control value for a select is read through select_value, and the masking rule passes every control value through the mask test
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list readers and close_select_list writer are not called by the chunk's new code
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and reconcile_select_list are document-internal reads the chunk does not call
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal arm's clear are select-list lifecycle the chunk does not touch
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of is called by the snapshot builder the chunk extends, and it decides the select value the graph carries for an open list
@answer n=8 entity=standard-contracts#2.7 v=no
why: a mutator write of multiple is not written or read by the chunk, which adds no mutation
@answer n=9 entity=standard-contracts#2.8 v=no
why: resource-load error handling is out of scope, since the graph makes no fetch and the load page is read as built
@answer n=10 entity=standard-contracts#3.1 v=yes
why: build_accessibility_tree's TreeUpdate, its author-id keyed nodes and the adapter crossing are the surface the graph keys on
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded value is neither a key nor a read of the graph or the snapshot
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus field is not read by the chunk, and the built-node set it relies on is not changed
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkable toggled write is a tree state the graph does not read
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height reader is the driver's style read, which the chunk does not touch
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs only for document writes, and the chunk adds no mutation
@answer n=16 entity=standard-contracts#4.1 v=yes
why: click's bubbling flag and the dispatch path to the vdom decide whether an owner's handler serves sub-targets and whether the stand's click reaches a handler
@answer n=17 entity=standard-contracts#4.2 v=no
why: a select list's default actions are not read by the reaction-to-click reading, which reads handler presence through dynamic attributes
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label's click focusing a bound select is a default action the reaction reading does not use
@answer n=19 entity=standard-contracts#4.4 v=no
why: change on a checkable control is not a click handler and the reaction reading does not read it
@answer n=20 entity=standard-contracts#5.1 v=yes
why: the NodeTree get and contains_key rule and the stale-id panic bind the graph read's walk over DOM nodes, which must go through get and never index
@answer n=21 entity=standard-contracts#6.1 v=yes
why: the readers of get_client_bounding_rect include the snapshot's bounds and the bounds the agent-visible node carries, which the scope names
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is read only by the driver's in_view, crop and lint, none of which the chunk touches
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is read only by Session::lint, which the chunk does not change
@answer n=24 entity=standard-contracts#6.4 v=no
why: the graph read does no hit test, so the hit walk over dropped nodes is not reached
@answer n=25 entity=standard-contracts#6.5 v=no
why: a non-invertible transform takes no hit, and the chunk performs no hit test
