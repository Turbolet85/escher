@answer n=1 entity=standard-contracts#17.1 v=no
why: the headless stand boot and Harness::from_vdom path are unchanged and the stand file is not reached
@answer n=2 entity=standard-contracts#18.1 v=no
why: the stand's bundled font and its font context are unchanged and the stand file is not reached
@answer n=3 entity=standard-contracts#19.1 v=no
why: boot_cells builds through the unchanged harness path and the stand file is not reached
@answer n=4 entity=standard-contracts#19.2 v=no
why: the load page boot and its keyed elements are unchanged and the stand file is not reached
@answer n=5 entity=standard-contracts#19.3 v=no
why: LoadNet is a stand-side fetch stand-in that the session does not touch and the stand file is not reached
@answer n=6 entity=standard-contracts#20.1 v=no
why: the telemetry bootstrap and its line shape are unchanged, the replay re-fires the writer's own trace calls, and no new log site is added
@answer n=7 entity=standard-contracts#21.1 v=no
why: the driver session contract and its files are unchanged and none of its files is reached
@answer n=8 entity=standard-contracts#22.1 v=no
why: the recording reads the call and its answer exactly as before and the escher-driver files are not reached
@answer n=9 entity=standard-contracts#22.2 v=no
why: the replay runs each step through the unchanged Session::run and the escher-driver files are not reached
@answer n=10 entity=standard-contracts#22.3 v=no
why: the visual lint reads the snapshot exactly as before and the escher-driver files are not reached
@answer n=11 entity=standard-contracts#22.4 v=no
why: the load wait settles through the unchanged Harness::settle with the same busy classes and the escher-driver files are not reached
@answer n=12 entity=standard-contracts#23.1 v=no
why: the select-option overlap reading is a lint and snapshot reading the session leaves alone and the cited test files are not reached
@answer n=13 entity=standard-contracts#23.2 v=no
why: the load page witness is a stand test whose file and behaviour the session does not move
@answer n=14 entity=standard-contracts#24.1 v=no
why: the JavaScript crate is not touched and none of its files is reached
@answer n=15 entity=standard-contracts#25.1 v=no
why: document_may_load in the JavaScript crate is not touched and none of its files is reached
@answer n=16 entity=standard-contracts#26.1 v=no
why: the CSS property accessors in the JavaScript crate are not touched and none of its files is reached
@answer n=17 entity=standard-contracts#26.2 v=no
why: URLSearchParams in the JavaScript crate is not touched and none of its files is reached
@answer n=18 entity=standard-contracts#27.1 v=no
why: the dioxus-native launch and its configs keep calling the bridge as before and dioxus-native's own files are not reached
@answer n=19 entity=standard-contracts#28.1 v=yes mode=add-to
why: the bridge now renders through the crate-private MutationBuffer (F1) and DioxusDocument holds the buffered_mutations field (F2), both to be carried beside its writer and event path
@answer n=20 entity=standard-contracts#28.2 v=no
why: cells.rs is not reached and the synchronous scroll_to calls and onmounted handles stand, since the session keeps the calls and the row does not name the hazard as their reason
@answer n=21 entity=standard-contracts#29.1 v=yes mode=stale
why: its witness citation dioxus_runtime_probe.rs:386-428 sits in a reached file whose two should_panic pins and helper changed, so the line range may have moved while the listener claim holds
@answer n=22 entity=standard-contracts#29.2 v=yes mode=contradict
why: the new mutation buffer replays create_event_listener and remove_event_listener into the MutationWriter, a workspace caller, so the claim that these have no workspace caller is no longer true
@answer n=23 entity=standard-contracts#30.1 v=no
why: is_masked_input lives in the unreached snapshot.rs and the crate root gains only a private mod line with no re-export moved
@answer n=24 entity=standard-contracts#30.2 v=yes mode=stale
why: dioxus_document.rs gains lines above 407 at 7, 90-91, 155, 166-168, 171, 302-304 and 307, so the citation dioxus_document.rs:407-412 has moved while the NativeFormData claim holds
@answer n=25 entity=standard-contracts#30.3 v=no
why: the snapshot carries no expanded key and NodeState holds no list field, and snapshot.rs is not reached
