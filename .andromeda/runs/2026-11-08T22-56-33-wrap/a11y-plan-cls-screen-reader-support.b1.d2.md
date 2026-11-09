@answer n=1 entity=screen-reader-support#1.1 v=no
why: the text-run tree builder in blitz-dom accessibility.rs is not in the session's changed files
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label and aria-labelledby naming in blitz-dom and its tests is untouched by the session
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label-to-control association in blitz-dom is untouched by the session
@answer n=4 entity=screen-reader-support#1.4 v=no
why: the select option tree and the snapshot's select readings are not in the session's changed files
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the actions answer's covered, disabled and off-screen readings are unchanged by the session
@answer n=6 entity=screen-reader-support#1.6 v=no
why: the select's closed-bool expanded reading in the tree is unchanged by the session
@answer n=7 entity=screen-reader-support#1.7 v=yes mode=stale
why: the session's change reaches docs/capabilities.md whole, so the limit cited at lines 259-261 may have moved
@answer n=8 entity=screen-reader-support#1.8 v=no
why: the unmeasured assistive-technology reading of an open list is untouched by the session
@answer n=9 entity=screen-reader-support#1.9 v=no
why: multiple-select row bounds in stand_act_select are untouched by the session
@answer n=10 entity=screen-reader-support#1.10 v=yes mode=stale
why: the session's change reaches dioxus_document.rs whole, so the multiple_select_values citation at lines 34-48 may have moved
@answer n=11 entity=screen-reader-support#2.1 v=yes mode=stale
why: the session's change reaches dioxus_document.rs and the shared module common/mod.rs whole, so the cited lines 338-354 and 17-185 may have moved and the common declarers rose 23 to 24
@answer n=12 entity=screen-reader-support#2.2 v=no
why: stand_graph's assertions are untouched and a read-only reader still changes nothing
@answer n=13 entity=screen-reader-support#3.1 v=no
why: the snapshot model and its readers are unchanged and snapshot.rs with its tests is not in the session's changed files
@answer n=14 entity=screen-reader-support#3.2 v=no
why: clickable's source and its snapshot_text.rs reading are unchanged by the session
@answer n=15 entity=screen-reader-support#3.3 v=yes mode=add-to
why: the session adds handler_coverage as a second read-only reader of DioxusDocument::graph, F3
@answer n=16 entity=screen-reader-support#3.4 v=no
why: the actions verb and its answer are unchanged and the new reader is not an actions verb
@answer n=17 entity=screen-reader-support#3.5 v=no
why: the Cells actions answer count and size bound are unchanged by the session
@answer n=18 entity=screen-reader-support#3.6 v=no
why: NodeState and NODE_FIELDS are unchanged because the session adds no node key
@answer n=19 entity=screen-reader-support#3.7 v=no
why: the option-label leaving an open list as painted pixels is unchanged by the session
@answer n=20 entity=screen-reader-support#3.8 v=no
why: the native checkbox toggled reading in the blitz-dom tree builder is unchanged
@answer n=21 entity=screen-reader-support#4.1 v=yes mode=add-to
why: the session adds the coverage reader applying the same mounted exclusion over graph nodes, a new reader of the actionable rule beside unkeyed_actionable and actionable_nodes
@answer n=22 entity=screen-reader-support#4.2 v=no
why: actionable_nodes' listing and keyed reading are unchanged and the text does not name who reads the rule
@answer n=23 entity=screen-reader-support#4.3 v=no
why: the provisional keyed field on the actions answer is unchanged by the session
@answer n=24 entity=screen-reader-support#4.4 v=no
why: the fixture's unkeyed button reading and the inert or hidden exclusion are unchanged by the session
@answer n=25 entity=screen-reader-support#4.5 v=no
why: stand_non_lean_tasks' CLICKABLE table and the listener mark counts are unchanged by the session
