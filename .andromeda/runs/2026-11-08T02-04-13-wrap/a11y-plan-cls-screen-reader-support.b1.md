@answer n=1 entity=screen-reader-support#1.1 v=no
why: the TextRun rule lives in packages/blitz-dom/src/accessibility.rs, which the session did not reach.
@answer n=2 entity=screen-reader-support#1.2 v=no
why: the aria-label and aria-labelledby naming rule lives in blitz-dom accessibility.rs and unreached tests, which the session did not reach.
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label association rule lives in blitz-dom accessibility.rs and document.rs, neither reached by the session.
@answer n=4 entity=screen-reader-support#1.4 v=yes mode=stale
why: snapshot.rs is reached and NodeState.clickable inserted after focused moves the cited snapshot.rs lines 155-281, the select readings themselves unchanged.
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the select's expanded reading lives in blitz-dom accessibility.rs and select_popup.rs, neither reached, and the snapshot holds no expanded field.
@answer n=6 entity=screen-reader-support#1.6 v=yes mode=stale
why: docs/capabilities.md is reached and its not-built click line was removed, so the cited lines 230-232 can have moved while the open-list limit still holds.
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the assistive-technology measurement and harness-only assertions lie outside every reached file and the session changed neither.
@answer n=8 entity=screen-reader-support#1.8 v=no
why: the multiple-select bounds claim cites only stand_act_select.rs, which the session did not reach, and the change names no bounds reading.
@answer n=9 entity=screen-reader-support#1.9 v=yes mode=stale
why: dioxus_document.rs is reached and its event path changed, so the cited lines 33-47 for multiple_select_values can have moved, and the report does not show the values reading holds.
@answer n=10 entity=screen-reader-support#2.1 v=yes mode=add-to
why: F5 stand_graph asserts that no served id holds the reserved separator ///, the fifth stable-id form virtual_node_id adds to this author_id grammar, which every existing id still meets.
@answer n=11 entity=screen-reader-support#3.1 v=yes mode=stale
why: the model's seven fields read eight with clickable after focused (F1), and the snapshot, snapshot_text and snapshot_diff files it cites are reached so their cited lines moved.
@answer n=12 entity=screen-reader-support#3.2 v=yes mode=stale
why: NODE_FIELDS reads thirteen and becomes fourteen with clickable, while the no-expanded-field claim still holds.
@answer n=13 entity=screen-reader-support#3.3 v=no
why: the option-name and snapshot-name claim cites stand_act_popup.rs, which the session did not reach, and the report says names read as before.
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checked reading lives in blitz-dom accessibility.rs and accessibility_states.rs, neither reached by the session.
@answer n=15 entity=screen-reader-support#4.1 v=yes mode=contradict
why: the claim that no snapshot field says an element reacts to a click is no longer true, since NodeState.clickable now reads it and NODE_FIELDS reads fourteen.
@answer n=16 entity=screen-reader-support#4.2 v=yes mode=contradict
why: the clause that no snapshot field is added is contradicted by NodeState.clickable, and NODE_FIELDS moves from thirteen to fourteen.
@answer n=17 entity=screen-reader-support#4.3 v=yes mode=add-to
why: the listener-mark over-report note stands, and clickable reads the same current listeners so a removed listener drops out of it while the mark and its count of 677 stay as they are.
@answer n=18 entity=screen-reader-support#4.4 v=no
why: cells-grid's aria_label and absent role live in reached cells.rs, but the report says roles and names read as before and no served markup moved.
@answer n=19 entity=screen-reader-support#4.5 v=no
why: the load page's empty unkeyed_actionable reading holds, since the report says the check returns nothing on every stand task and the load-page files are unreached.
@answer n=20 entity=screen-reader-support#5.1 v=no
why: the CRUD and flight booker roles and states are reached but the report says roles, names and Tab orders read as before.
@answer n=21 entity=screen-reader-support#6.1 v=no
why: the tag-name rule lives in blitz-dom accessibility.rs, which the session did not reach.
@answer n=22 entity=screen-reader-support#6.2 v=no
why: the window root and focus rule lives in blitz-dom accessibility.rs, which the session did not reach.
@answer n=23 entity=screen-reader-support#6.3 v=no
why: the role-mapping test goal lives in accessibility_roles.rs, which the session did not reach.
@answer n=24 entity=screen-reader-support#7.1 v=no
why: accesskit_xplat event routing lies outside every reached file and the chunk builds no adapter.
@answer n=25 entity=screen-reader-support#8.1 v=no
why: blitz-shell adapter creation lies outside every reached file and the chunk builds no adapter.
