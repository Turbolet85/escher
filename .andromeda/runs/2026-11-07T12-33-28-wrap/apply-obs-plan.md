@apply_row n=1 ref_file=fanout-obs-plan--cascade.md ref_n=4 v=refused
why: main's own — its target lies outside this writer's files
@apply_row n=2 ref_file=fanout-obs-plan--cascade.md ref_n=1 v=applied edit=E1
@apply_row n=3 ref_file=fanout-obs-plan--cascade.md ref_n=2 v=applied edit=E2
@apply_row n=4 ref_file=fanout-obs-plan--cascade.md ref_n=3 v=applied edit=E3
@fact after=fct-gxkjmvep as=n1
- **dioxus-native-dom (mutation buffer)**
  - no site added: dioxus-native-dom's crate-private module `mutation_buffer` (`MutationBuffer`, recording dioxus-core 0.7.10's 15 `WriteMutations` calls and replaying them into `MutationWriter`) and the lines of `DioxusDocument::initial_build` and `Document::poll` that render through it hold no log site, print, env read, listener or panic token, both silence censuses reading 0; the replay re-fires `MutationWriter`'s own `trace!` calls, so the log stream is the one the direct write gives (packages/dioxus-native-dom/src/mutation_buffer.rs:97-140; as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md)

@fact after=fct-pynegysb as=n2
- observed absent — a print, an env read, a subscriber or a listener in `dioxus_runtime_probe`'s two pins `mounted_data_in_a_task_a_key_handler_spawned_acts_non_incremental` and `mounted_data_in_a_task_a_key_handler_spawned_acts_incremental` and their `#[track_caller]` helper `spawned_focus_acts` · searched: the silence census over the file, exit 1, reading 0 (as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md, its gates)

@fact after=fct-yvbbn3zt as=n3
- The buffered render's `MutationBuffer` (dioxus-native-dom, crate-private) holds element ids, tag and attribute names, attribute values and text node contents in process memory only, from a render to its replay, which drains the entries and clears its text arena; its `Debug` prints `len` alone, and nothing logs, fields or prints its contents, the silence census over the module reading 0 (packages/dioxus-native-dom/src/mutation_buffer.rs:71-140; as measured at escher-0.3.0/chunks/2026-11-07-mounteddata-buffered-render/report.md)
