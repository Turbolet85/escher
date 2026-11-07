@account n=1 entity=standard-contracts#27.1 v=not_in_scope
why: the launch API's entry points, configs and hooks are used unchanged; the windowed path only takes the buffered render through DioxusDocument, which no line here states
@account n=2 entity=standard-contracts#28.1 v=carried
why: the replayed buffer must keep the bridge's node drop, id mappings, stable ids, listener writes and snapshot/diff readings that stand_* and stale_node_mapping re-verify, with mutation_writer.rs byte-identical
@account n=3 entity=standard-contracts#28.2 v=carried
why: the Cells grid's synchronous MountedData::scroll_to over onmounted-held handles is a MountedData path the stand re-runs and the mounted events' order feeds
@account n=4 entity=standard-contracts#29.1 v=carried
why: create_event_listener / remove_event_listener behaviour is what the replay must reproduce, witnessed by a dioxus_runtime_probe test the chunk re-runs
@account n=5 entity=standard-contracts#29.2 v=carried
why: states which 0.7 WriteMutations calls dioxus-core makes for listeners, the calls the recorder must capture and the replay becomes the workspace caller of
@account n=6 entity=standard-contracts#30.1 v=not_in_scope
why: the masked-control test is a snapshot reader the render path never reaches; a re-check every change owes
@account n=7 entity=standard-contracts#30.2 v=not_in_scope
why: the select's form-data conversion is another unit of dioxus_document.rs that must only keep holding; a re-check every change owes
@account n=8 entity=standard-contracts#30.3 v=not_in_scope
why: the snapshot's node-key set is untouched by the render path; a re-check every change owes
@account n=9 entity=standard-contracts#30.4 v=not_in_scope
why: a multiple select's snapshot bounds are a snapshot reader the render path never reaches; a re-check every change owes
@account n=10 entity=standard-contracts#30.6 v=carried
why: flush_queued_mounted_events runs after the replay unchanged, its removed-element guard reading mappings the replay must have written
@account n=11 entity=standard-contracts#34.1 v=carried
why: the new mutation_buffer.rs counts against dioxus-native-dom's 83 % coverage floor, and stand-repeat's zero-failure budget runs the re-verified stand checks
