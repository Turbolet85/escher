@answer n=1 entity=standard-contracts#17.1 v=no
why: the headless stand boots through Harness::from_vdom and its timer step, which the session leaves unchanged, and no stand file is reached.
@answer n=2 entity=standard-contracts#18.1 v=no
why: the bundled DejaVu Sans typography lies in the stand module, which the session does not reach.
@answer n=3 entity=standard-contracts#19.1 v=no
why: boot_cells mounts Cells through the same unchanged boot path, and cells.rs is not a reached file.
@answer n=4 entity=standard-contracts#19.2 v=no
why: boot_load's page and its replaced net provider lie in the stand module, which the session does not reach.
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet is a stand-module network stand-in that the session does not touch.
@answer n=6 entity=standard-contracts#20.1 v=no
why: the session adds no log site, print or span and leaves the telemetry bootstrap unchanged.
@answer n=7 entity=standard-contracts#21.1 v=no
why: the escher-driver crate, its session, verbs and load wait are unchanged, and the buffer lives inside dioxus-native-dom.
@answer n=8 entity=standard-contracts#22.1 v=no
why: the recording lives in escher-driver, which the session leaves unchanged.
@answer n=9 entity=standard-contracts#22.2 v=no
why: the replay compares kept answers in escher-driver, which the session does not change.
@answer n=10 entity=standard-contracts#22.3 v=no
why: the visual lint reads the snapshot in escher-driver, which the session does not change.
@answer n=11 entity=standard-contracts#22.4 v=no
why: the load wait's settle loop is in escher-driver and the session leaves it and Harness::settle unchanged.
@answer n=12 entity=standard-contracts#23.1 v=no
why: the select-option overlap reading and its stand_lint pins lie in files the session does not reach.
@answer n=13 entity=standard-contracts#23.2 v=no
why: the load-page witness's click and LoadNet are unchanged, in files the session does not reach.
@answer n=14 entity=standard-contracts#24.1 v=no
why: blitz-vibey-script is a separate crate that the session does not reach.
@answer n=15 entity=standard-contracts#25.1 v=no
why: document_may_load in blitz-vibey-script is untouched by the session.
@answer n=16 entity=standard-contracts#26.1 v=no
why: the CSS property accessors live in blitz-vibey-script, which the session does not reach.
@answer n=17 entity=standard-contracts#26.2 v=no
why: URLSearchParams lives in blitz-vibey-script, which the session does not reach.
@answer n=18 entity=standard-contracts#27.1 v=no
why: the dioxus-native launch surface is unchanged, and the new buffer module is private with no re-export.
@answer n=19 entity=standard-contracts#28.1 v=yes mode=add-to
why: the bridge gains the crate-private MutationBuffer (F1) and the DioxusDocument buffered_mutations field (F2), and its dioxus_document.rs line citations have shifted by the 52 added lines.
@answer n=20 entity=standard-contracts#28.2 v=no
why: the Cells keyboard lives in examples/seven_guis, which the session does not reach, and its synchronous scroll_to calls are unchanged.
@answer n=21 entity=standard-contracts#29.1 v=yes mode=stale
why: the probe file is reached whole and its cited lines 386-428 may have moved, while the listener behaviour itself still holds.
@answer n=22 entity=standard-contracts#29.2 v=yes mode=contradict
why: the buffer records and replays dioxus-core's create_event_listener and remove_event_listener calls, so the replay is now a workspace caller of both.
@answer n=23 entity=standard-contracts#30.1 v=no
why: is_masked_input lives in snapshot.rs, which the session does not reach.
@answer n=24 entity=standard-contracts#30.2 v=yes mode=stale
why: its cited dioxus_document.rs:407-412 moved with the session's added lines above it (302-304 and 307).
@answer n=25 entity=standard-contracts#30.3 v=no
why: NodeState and NODE_FIELDS live in snapshot.rs, which the session does not reach.
