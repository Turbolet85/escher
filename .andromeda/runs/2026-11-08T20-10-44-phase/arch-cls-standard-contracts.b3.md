@answer n=1 entity=standard-contracts#17.1 v=yes
why: the stand's boots, LeanTask, pinned viewport and offline options are where the recorded checks run and the replay test boots its fresh session, scope item 13.
@answer n=2 entity=standard-contracts#18.1 v=yes
why: the pinned font of every stand boot is named in scope item 13 ("pinned viewport and font") and the replay's fresh session boots with it.
@answer n=3 entity=standard-contracts#19.1 v=yes
why: boot_cells mounts Cells, which scope item 13 names, and its keyed handlers are what the coverage list reads.
@answer n=4 entity=standard-contracts#19.2 v=yes
why: boot_load mounts the load page, which scope item 13 names, holding the keyed load-fetch and load-done handlers the coverage list reads.
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet is the load page's network stand-in with a 200 ms answer delay that neither the scope nor the chunk's code or tests act on.
@answer n=6 entity=standard-contracts#19.4 v=yes
why: use_graph_signal wraps use_graph_state and tasks::writes builds a DeclaredWrite, both named in scope item 2 and read by the no-registry handler record.
@answer n=7 entity=standard-contracts#20.1 v=yes
why: the escher_ target rule and the CONTENT_FIELDS scrub govern any span or log the new coverage code emits, scope item 18.
@answer n=8 entity=standard-contracts#21.1 v=yes
why: Session::start, act and run, the v3 wire, VERBS and the SessionError set are the driver path every coverage read and replay takes, scope items 5, 8 and 9.
@answer n=9 entity=standard-contracts#22.1 v=yes
why: recording_into, Check and the kept-check bounds govern the owed test's kept actions step, scope item 6 and the CARRY replay.
@answer n=10 entity=standard-contracts#22.2 v=yes
why: Check::replay and Change govern the owed replay test and its differing-actions answer naming the step and the differing key.
@answer n=11 entity=standard-contracts#22.3 v=yes
why: the VERBS count of fourteen and its table order are what scope item 9 says a new verb moves, though the lint readings themselves are untouched.
@answer n=12 entity=standard-contracts#22.4 v=yes
why: the load wait governs the acting steps a replay runs through Session::run, while the read verbs wait on nothing, as the coverage read must.
@answer n=13 entity=standard-contracts#23.1 v=no
why: the select-options overlap is a lint reading over stand_lint, which the chunk neither edits nor reads.
@answer n=14 entity=standard-contracts#23.2 v=no
why: the load-page witness is stand_load_page's click-waits-for-load behaviour, a test the chunk does not touch and no replay rule needs.
@answer n=15 entity=standard-contracts#23.3 v=yes
why: the graph verb's written keys (id, owner, component, handlers, event, writes) are what the coverage list is keyed on and reads, scope items 1 and 4.
@answer n=16 entity=standard-contracts#23.4 v=yes
why: the coverage answer is a new answer that must stay inside the 1,048,576-byte answer bound the Cells graph sits under.
@answer n=17 entity=standard-contracts#23.5 v=yes
why: stand_graph's rules on no reserved /// served id, changes-nothing reads and no typed text bind the coverage list's served ids and read, scope items 4 and 8.
@answer n=18 entity=standard-contracts#23.6 v=yes
why: the actions read's record of ids and its call_naming and validate loop are the actions verb the owed test reads and compares, scope item 7.
@answer n=19 entity=standard-contracts#23.7 v=yes
why: check_target's order fixes the per-verb causes the actions answer reports, which the owed test compares key by key.
@answer n=20 entity=standard-contracts#23.8 v=yes
why: the actions read's changes-nothing rule (no pump, tick, dispatch or focus call) is the one the coverage read must hold, and it names the actions verb.
@answer n=21 entity=standard-contracts#23.9 v=yes
why: the actions answer's acceptance-only limit and its 106,088-byte unpaged size under the bound constrain the replayed actions step and any new answer.
@answer n=22 entity=standard-contracts#24.1 v=no
why: JavaScript on Boa is not touched, not_needed item 4.
@answer n=23 entity=standard-contracts#25.1 v=no
why: document_may_load governs script fetches, which no path of the chunk touches, not_needed item 4.
@answer n=24 entity=standard-contracts#26.1 v=no
why: CSS accessors on the script side are not touched, not_needed item 4.
@answer n=25 entity=standard-contracts#26.2 v=no
why: URLSearchParams in the script bootstrap is not touched, not_needed item 4.
