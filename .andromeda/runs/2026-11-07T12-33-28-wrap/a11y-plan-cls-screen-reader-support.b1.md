@answer n=1 entity=screen-reader-support#1.1 v=no
why: text-run labelling is in blitz-dom accessibility.rs, a file the buffered render does not reach
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label and aria-labelledby naming is in blitz-dom accessibility.rs and its tests, none of them reached
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label binding is in blitz-dom accessibility.rs and document.rs, neither reached by the session
@answer n=4 entity=screen-reader-support#1.4 v=no
why: select option nodes and select_value are in blitz-dom and the snapshot.rs file, not reached; the replay keeps the same mutations in the same order
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single-select expanded reading is in blitz-dom accessibility.rs, not reached
@answer n=6 entity=screen-reader-support#1.6 v=no
why: the open-list highlight limit is in docs/capabilities.md and the snapshot, both untouched by the session
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the unmeasured assistive-technology reading of an open list has no adapter or harness surface changed by the session
@answer n=8 entity=screen-reader-support#1.8 v=no
why: multiple-select row bounds come from blitz-dom layout and the stand_act_select test, neither reached
@answer n=9 entity=screen-reader-support#1.9 v=yes mode=stale
why: the buffered render rewrites dioxus_document.rs, which this entity cites at lines 32-46 for multiple_select_values, so the citation may have moved
@answer n=10 entity=screen-reader-support#2.1 v=yes mode=stale
why: the buffered render rewrites dioxus_document.rs, which this entity cites at lines 324-340 for the author_id, so the citation may have moved
@answer n=11 entity=screen-reader-support#3.1 v=no
why: the snapshot reading is in dioxus-native-dom snapshot.rs, not reached, and the session moves no snapshot field, focus reading or stand assertion
@answer n=12 entity=screen-reader-support#3.2 v=no
why: NodeState and NODE_FIELDS in snapshot.rs stay as they are, not reached
@answer n=13 entity=screen-reader-support#3.3 v=no
why: the open-list screenshot name is in the snapshot and the select popup test, neither reached
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checkbox toggled reading is in blitz-dom accessibility.rs, not reached
@answer n=15 entity=screen-reader-support#4.1 v=no
why: the actionable-key check is in actionable.rs and the stand tests, none reached, and the buffer replays the same listener mutations
@answer n=16 entity=screen-reader-support#4.2 v=yes mode=stale
why: dioxus_runtime_probe.rs is reached whole and this entity cites its link 2 lines 281-316 and 339-367, which may have moved while the link readings stay true
@answer n=17 entity=screen-reader-support#4.3 v=yes mode=stale
why: dioxus_runtime_probe.rs is reached whole and this entity cites its lines 386-428, which may have moved while the listener reading stays true
@answer n=18 entity=screen-reader-support#4.4 v=yes mode=stale
why: it names residual res-wvatzhpe as the open owner of cell semantics, which the wrap re-says to its row status, re-carried at 0.3.0 with target next
@answer n=19 entity=screen-reader-support#4.5 v=no
why: the load-page check is in the stand_load_page and stand_keyboard tests, none reached
@answer n=20 entity=screen-reader-support#5.1 v=no
why: the stand's attribute-only roles and the aria-activedescendant gap are in the stand tests and the crud and flight_booker examples, none reached
@answer n=21 entity=screen-reader-support#6.1 v=no
why: the element tag name is set in blitz-dom accessibility.rs, not reached
@answer n=22 entity=screen-reader-support#6.2 v=no
why: the tree root and focus fallback are in blitz-dom accessibility.rs, not reached
@answer n=23 entity=screen-reader-support#6.3 v=no
why: the role-mapping test goal is in accessibility_roles.rs, not reached
@answer n=24 entity=screen-reader-support#7.1 v=no
why: accesskit_xplat event routing is in packages/accesskit_xplat, not reached, and no adapter work this session
@answer n=25 entity=screen-reader-support#8.1 v=no
why: adapter creation is in blitz-shell accessibility.rs, not reached, and no adapter work this session
